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
            }
            let (assign25290_body9_e21828,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body9_e21828;
            let (assign25290_body10_e21836,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body10_e21836;
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
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
            let (assign25290_body21_e21940,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body21_e21940;
            s.b[1597] = (2.0 == 2.0);
            s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };
            let (assign25290_body23_e21958,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body23_e21958;
            s.b[1598] = (2.0 == 4.0);
            s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };
            let (assign25290_body25_e21979,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body25_e21979;
            s.b[1599] = (2.0 == 8.0);
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            let (assign25290_body27_e22003,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body27_e22003;
            let (assign25290_body28_e22013,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body28_e22013;
            let mut assign25290_body29_loop_guard: usize = 0;
            while {
                let assign25290_body29_cond_e22024: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body29_cond_e22024 != 0.0
            } {
                assign25290_body29_loop_guard += 1;
                assert!(assign25290_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25290_body29_body1_e22047,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
        let assign25290_body29_body1_e22045: f64 = (s.v[719] + 1.0);
        (assign25290_body29_body1_e22045,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25290_body29_body1_e22047;
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && (!s.b[1595])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
            }
            let (assign25290_body45_e22217,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body45_e22217;
            let (assign25290_body46_e22225,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body46_e22225;
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
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
            let (assign25290_body57_e22329,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body57_e22329;
            s.b[1603] = (2.0 == 2.0);
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            let (assign25290_body59_e22347,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body59_e22347;
            s.b[1604] = (2.0 == 4.0);
            s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };
            let (assign25290_body61_e22368,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body61_e22368;
            s.b[1605] = (2.0 == 8.0);
            s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };
            let (assign25290_body63_e22392,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body63_e22392;
            let (assign25290_body64_e22402,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body64_e22402;
            let mut assign25290_body65_loop_guard: usize = 0;
            while {
                let assign25290_body65_cond_e22413: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body65_cond_e22413 != 0.0
            } {
                assign25290_body65_loop_guard += 1;
                assert!(assign25290_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25290_body65_body1_e22436,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
        let assign25290_body65_body1_e22434: f64 = (s.v[719] + 1.0);
        (assign25290_body65_body1_e22434,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25290_body65_body1_e22436;
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && (!s.b[1601])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
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
                s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1523, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1475), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1525, 1523);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1606])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));
                s.store_mul_sqrt_ad_rhs(1475, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1475, 1.0);
                s.store_mul_add_ad_rhs(1523, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1525, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1607] = ((s.v[1520] > (s.v[1511] - s.v[1519])) && (s.v[1519] >= 0.0));
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1511, (-1.0), 1519, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1519);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign25290_body102_e22958,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body102_e22958;
            let (assign25290_body103_e22966,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body103_e22966;
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
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
            let (assign25290_body118_e23110,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body118_e23110;
            s.b[1610] = (4.0 == 2.0);
            s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };
            let (assign25290_body120_e23128,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body120_e23128;
            s.b[1611] = (4.0 == 4.0);
            s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };
            let (assign25290_body122_e23149,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body122_e23149;
            s.b[1612] = (4.0 == 8.0);
            s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };
            let (assign25290_body124_e23173,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25290_body124_e23173;
            let (assign25290_body125_e23183,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25290_body125_e23183;
            let mut assign25290_body126_loop_guard: usize = 0;
            while {
                let assign25290_body126_cond_e23194: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body126_cond_e23194 != 0.0
            } {
                assign25290_body126_loop_guard += 1;
                assert!(assign25290_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25290_body126_body1_e23217,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
        let assign25290_body126_body1_e23215: f64 = (s.v[719] + 1.0);
        (assign25290_body126_body1_e23215,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25290_body126_body1_e23217;
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && (!s.b[1608])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1519, 726);
                s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1511, 1.0, 1519, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1475), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1461)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);
                s.store_sub(1489, 1523, 185);
                s.store_add_scaled_inputs_products_indices(1490, 1525, 1.0, 1527, 1.0, 1529, 1533, 1.0, 1531, 1533, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
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
            let (assign25290_body152_e23544,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
        let assign25290_body152_e23542: f64 = (150.0 + 1.0);
        (assign25290_body152_e23542,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign25290_body152_e23544;
            let (assign25290_body153_e23552,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign25290_body153_e23552;
            let (assign25290_body154_e23558,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1461],)
    } else {
        (s.v[1469],)
    }
};
            s.v[1469] = assign25290_body154_e23558;
            let (assign25290_body155_e23564,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
            s.v[1470] = assign25290_body155_e23564;
            let (assign25290_body156_e23572,) = {
    if (s.b[1443] && s.b[1444]) {
        let assign25290_body156_e23570: f64 = (s.v[97] + 1.0);
        (assign25290_body156_e23570,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign25290_body156_e23572;
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_offset_sub(781, 1483, 1461, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign25380_e23659,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25380_e23659;

        let (assign25390_e23669,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25390_e23669;

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
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

        let (assign25500_e23791,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25500_e23791;

        s.b[1620] = (2.0 == 2.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        let (assign25520_e23811,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25520_e23811;

        s.b[1621] = (2.0 == 4.0);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        let (assign25540_e23834,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25540_e23834;

        s.b[1622] = (2.0 == 8.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        let (assign25560_e23860,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25560_e23860;

        let (assign25570_e23872,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25570_e23872;

        let mut assign25580_loop_guard: usize = 0;
        while {
            let assign25580_cond_e23885: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25580_cond_e23885 != 0.0
        } {
            assign25580_loop_guard += 1;
            assert!(assign25580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
                s.store_sqrt(726, 726);
            }
            let (assign25580_body1_e23912,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
        let assign25580_body1_e23910: f64 = (s.v[719] + 1.0);
        (assign25580_body1_e23910,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25580_body1_e23912;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1483, 1461, (-0.02), 780);
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
            s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
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
        }

        let (assign25820_e24190,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25820_e24190;

        let (assign25830_e24200,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25830_e24200;

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
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

        let (assign25940_e24322,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25940_e24322;

        s.b[1627] = (2.0 == 2.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        let (assign25960_e24342,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25960_e24342;

        s.b[1628] = (2.0 == 4.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        let (assign25980_e24365,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25980_e24365;

        s.b[1629] = (2.0 == 8.0);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        let (assign26000_e24391,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26000_e24391;

        let (assign26010_e24403,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26010_e24403;

        let mut assign26020_loop_guard: usize = 0;
        while {
            let assign26020_cond_e24416: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26020_cond_e24416 != 0.0
        } {
            assign26020_loop_guard += 1;
            assert!(assign26020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
                s.store_sqrt(726, 726);
            }
            let (assign26020_body1_e24443,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
        let assign26020_body1_e24441: f64 = (s.v[719] + 1.0);
        (assign26020_body1_e24441,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign26020_body1_e24443;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
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
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));
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
            s.store_add_scaled_product_right_ad(1477, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
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
        }

        let (assign26310_e24850,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26310_e24850;

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        let (assign26320_e24861,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26320_e24861;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
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

        let (assign26430_e24992,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26430_e24992;

        s.b[1634] = (2.0 == 2.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        let (assign26450_e25013,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26450_e25013;

        s.b[1635] = (2.0 == 4.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        let (assign26470_e25037,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26470_e25037;

        s.b[1636] = (2.0 == 8.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        let (assign26490_e25064,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26490_e25064;

        let (assign26500_e25077,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26500_e25077;

        let mut assign26510_loop_guard: usize = 0;
        while {
            let assign26510_cond_e25091: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26510_cond_e25091 != 0.0
        } {
            assign26510_loop_guard += 1;
            assert!(assign26510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
                s.store_sqrt(726, 726);
            }
            let (assign26510_body1_e25120,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
        let assign26510_body1_e25118: f64 = (s.v[719] + 1.0);
        (assign26510_body1_e25118,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign26510_body1_e25120;
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && (!s.b[1632])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

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
        }

        let (assign26670_e25335,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26670_e25335;

        let (assign26680_e25346,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26680_e25346;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
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

        let (assign26790_e25477,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26790_e25477;

        s.b[1640] = (2.0 == 2.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        let (assign26810_e25498,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26810_e25498;

        s.b[1641] = (2.0 == 4.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        let (assign26830_e25522,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26830_e25522;

        s.b[1642] = (2.0 == 8.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        let (assign26850_e25549,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26850_e25549;

        let (assign26860_e25562,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26860_e25562;

        let mut assign26870_loop_guard: usize = 0;
        while {
            let assign26870_cond_e25576: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26870_cond_e25576 != 0.0
        } {
            assign26870_loop_guard += 1;
            assert!(assign26870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
                s.store_sqrt(726, 726);
            }
            let (assign26870_body1_e25605,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
        let assign26870_body1_e25603: f64 = (s.v[719] + 1.0);
        (assign26870_body1_e25603,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign26870_body1_e25605;
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && (!s.b[1638])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
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
        }

        let (assign27060_e25837,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27060_e25837;

        let (assign27070_e25845,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27070_e25845;

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
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

        let (assign27180_e25949,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27180_e25949;

        s.b[1646] = (2.0 == 2.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        let (assign27200_e25967,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27200_e25967;

        s.b[1647] = (2.0 == 4.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        let (assign27220_e25988,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27220_e25988;

        s.b[1648] = (2.0 == 8.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        let (assign27240_e26012,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27240_e26012;

        let (assign27250_e26022,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27250_e26022;

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        let mut assign27260_loop_guard: usize = 0;
        while {
            let assign27260_cond_e26033: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27260_cond_e26033 != 0.0
        } {
            assign27260_loop_guard += 1;
            assert!(assign27260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
                s.store_sqrt(726, 726);
            }
            let (assign27260_body1_e26056,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
        let assign27260_body1_e26054: f64 = (s.v[719] + 1.0);
        (assign27260_body1_e26054,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27260_body1_e26056;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && (!s.b[1644])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
        }

        let (assign27420_e26226,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27420_e26226;

        let (assign27430_e26234,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27430_e26234;

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
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

        let (assign27540_e26338,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27540_e26338;

        s.b[1652] = (2.0 == 2.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        let (assign27560_e26356,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27560_e26356;

        s.b[1653] = (2.0 == 4.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        let (assign27580_e26377,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27580_e26377;

        s.b[1654] = (2.0 == 8.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        let (assign27600_e26401,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27600_e26401;

        let (assign27610_e26411,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27610_e26411;

        let mut assign27620_loop_guard: usize = 0;
        while {
            let assign27620_cond_e26422: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27620_cond_e26422 != 0.0
        } {
            assign27620_loop_guard += 1;
            assert!(assign27620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
                s.store_sqrt(726, 726);
            }
            let (assign27620_body1_e26445,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
        let assign27620_body1_e26443: f64 = (s.v[719] + 1.0);
        (assign27620_body1_e26443,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27620_body1_e26445;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && (!s.b[1650])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
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
            s.store_square(722, 781);
            s.store_scalar(723, (0.05 * 0.05));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign27780_e26614,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27780_e26614;

        let (assign27790_e26622,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27790_e26622;

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
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

        let (assign27900_e26726,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27900_e26726;

        s.b[1658] = (2.0 == 2.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        let (assign27920_e26744,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27920_e26744;

        s.b[1659] = (2.0 == 4.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        let (assign27940_e26765,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27940_e26765;

        s.b[1660] = (2.0 == 8.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        let (assign27960_e26789,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27960_e26789;

        let (assign27970_e26799,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27970_e26799;

        let mut assign27980_loop_guard: usize = 0;
        while {
            let assign27980_cond_e26810: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27980_cond_e26810 != 0.0
        } {
            assign27980_loop_guard += 1;
            assert!(assign27980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
                s.store_sqrt(726, 726);
            }
            let (assign27980_body1_e26833,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
        let assign27980_body1_e26831: f64 = (s.v[719] + 1.0);
        (assign27980_body1_e26831,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27980_body1_e26833;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && (!s.b[1656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1447, (-1.0), 1449, -1.0);
        }

        s.b[1661] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
            s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-18 * 1e-18));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign28150_e27013,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28150_e27013;

        let (assign28160_e27021,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28160_e27021;

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        s.b[1663] = (2.0 == 1.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        let (assign28270_e27125,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28270_e27125;

        s.b[1664] = (2.0 == 2.0);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        let (assign28290_e27143,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28290_e27143;

        s.b[1665] = (2.0 == 4.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        let (assign28310_e27164,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28310_e27164;

        s.b[1666] = (2.0 == 8.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        let (assign28330_e27188,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28330_e27188;

        let (assign28340_e27198,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28340_e27198;

        let mut assign28350_loop_guard: usize = 0;
        while {
            let assign28350_cond_e27209: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28350_cond_e27209 != 0.0
        } {
            assign28350_loop_guard += 1;
            assert!(assign28350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
                s.store_sqrt(726, 726);
            }
            let (assign28350_body1_e27232,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
        let assign28350_body1_e27230: f64 = (s.v[719] + 1.0);
        (assign28350_body1_e27230,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign28350_body1_e27232;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && (!s.b[1662])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
        }

        let (assign28520_e27421,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28520_e27421;

        let (assign28530_e27431,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28530_e27431;

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
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

        let (assign28640_e27553,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28640_e27553;

        s.b[1671] = (2.0 == 2.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        let (assign28660_e27573,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28660_e27573;

        s.b[1672] = (2.0 == 4.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        let (assign28680_e27596,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28680_e27596;

        s.b[1673] = (2.0 == 8.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        let (assign28700_e27622,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28700_e27622;

        let (assign28710_e27634,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28710_e27634;

        let mut assign28720_loop_guard: usize = 0;
        while {
            let assign28720_cond_e27647: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28720_cond_e27647 != 0.0
        } {
            assign28720_loop_guard += 1;
            assert!(assign28720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
                s.store_sqrt(726, 726);
            }
            let (assign28720_body1_e27674,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
        let assign28720_body1_e27672: f64 = (s.v[719] + 1.0);
        (assign28720_body1_e27672,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign28720_body1_e27674;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && (!s.b[1669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1511, (-0.8), 780);
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
        }

        let (assign28870_e27869,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28870_e27869;

        let (assign28880_e27880,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28880_e27880;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        s.b[1676] = (2.0 == 1.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        let (assign28990_e28011,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28990_e28011;

        s.b[1677] = (2.0 == 2.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        let (assign29010_e28032,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29010_e28032;

        s.b[1678] = (2.0 == 4.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        let (assign29030_e28056,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29030_e28056;

        s.b[1679] = (2.0 == 8.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        let (assign29050_e28083,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29050_e28083;

        let (assign29060_e28096,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29060_e28096;

        let mut assign29070_loop_guard: usize = 0;
        while {
            let assign29070_cond_e28110: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29070_cond_e28110 != 0.0
        } {
            assign29070_loop_guard += 1;
            assert!(assign29070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
                s.store_sqrt(726, 726);
            }
            let (assign29070_body1_e28139,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
        let assign29070_body1_e28137: f64 = (s.v[719] + 1.0);
        (assign29070_body1_e28137,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29070_body1_e28139;
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && (!s.b[1675])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1511, (-0.8), 780);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
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
        }

        let (assign29230_e28346,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29230_e28346;

        let (assign29240_e28354,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29240_e28354;

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
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

        let (assign29350_e28458,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && s.b[1682]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29350_e28458;

        s.b[1683] = (2.0 == 2.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        let (assign29370_e28476,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && s.b[1683]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29370_e28476;

        s.b[1684] = (2.0 == 4.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        let (assign29390_e28497,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && s.b[1684]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29390_e28497;

        s.b[1685] = (2.0 == 8.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        let (assign29410_e28521,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && (!s.b[1684])) && s.b[1685]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29410_e28521;

        let (assign29420_e28531,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29420_e28531;

        let mut assign29430_loop_guard: usize = 0;
        while {
            let assign29430_cond_e28542: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29430_cond_e28542 != 0.0
        } {
            assign29430_loop_guard += 1;
            assert!(assign29430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
                s.store_sqrt(726, 726);
            }
            let (assign29430_body1_e28565,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
        let assign29430_body1_e28563: f64 = (s.v[719] + 1.0);
        (assign29430_body1_e28563,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29430_body1_e28565;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && (!s.b[1681])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1515, 209, -1.0, 338);
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1540, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1686] = (s.v[790] > 1e-6);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_div_ad_rhs(336, 1502, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1438, -1.0, 2.0);
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
        }

        let (assign29670_e28842,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29670_e28842;

        let (assign29680_e28852,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29680_e28852;

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
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

        let (assign29790_e28974,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29790_e28974;

        s.b[1690] = (2.0 == 2.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        let (assign29810_e28994,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && s.b[1690]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29810_e28994;

        s.b[1691] = (2.0 == 4.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        let (assign29830_e29017,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && s.b[1691]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29830_e29017;

        s.b[1692] = (2.0 == 8.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        let (assign29850_e29043,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && (!s.b[1691])) && s.b[1692]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29850_e29043;

        let (assign29860_e29055,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29860_e29055;

        let mut assign29870_loop_guard: usize = 0;
        while {
            let assign29870_cond_e29068: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29870_cond_e29068 != 0.0
        } {
            assign29870_loop_guard += 1;
            assert!(assign29870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
                s.store_sqrt(726, 726);
            }
            let (assign29870_body1_e29095,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
        let assign29870_body1_e29093: f64 = (s.v[719] + 1.0);
        (assign29870_body1_e29093,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29870_body1_e29095;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_offset_lhs(344, 85, 2.0, 338);
        }

        s.b[1693] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_sub_from_scalar(781, (0.3 + 0.2), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign30060_e29327,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30060_e29327;

        let (assign30070_e29337,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30070_e29337;

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_mul(725, 725, 723);
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

        let (assign30220_e29507,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && s.b[1695]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30220_e29507;

        s.b[1696] = (4.0 == 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        let (assign30240_e29527,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30240_e29527;

        s.b[1697] = (4.0 == 4.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        let (assign30260_e29550,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && s.b[1697]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30260_e29550;

        s.b[1698] = (4.0 == 8.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        let (assign30280_e29576,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && (!s.b[1697])) && s.b[1698]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30280_e29576;

        let (assign30290_e29588,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30290_e29588;

        let mut assign30300_loop_guard: usize = 0;
        while {
            let assign30300_cond_e29601: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30300_cond_e29601 != 0.0
        } {
            assign30300_loop_guard += 1;
            assert!(assign30300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
                s.store_sqrt(726, 726);
            }
            let (assign30300_body1_e29628,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
        let assign30300_body1_e29626: f64 = (s.v[719] + 1.0);
        (assign30300_body1_e29626,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign30300_body1_e29628;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && (!s.b[1694])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
        }

        let (assign30530_e29915,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30530_e29915;

        let (assign30540_e29925,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30540_e29925;

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
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

        let (assign30650_e30047,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && s.b[1701]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30650_e30047;

        s.b[1702] = (2.0 == 2.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        let (assign30670_e30067,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && s.b[1702]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30670_e30067;

        s.b[1703] = (2.0 == 4.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        let (assign30690_e30090,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30690_e30090;

        s.b[1704] = (2.0 == 8.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        let (assign30710_e30116,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) && s.b[1704]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30710_e30116;

        let (assign30720_e30128,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30720_e30128;

        let mut assign30730_loop_guard: usize = 0;
        while {
            let assign30730_cond_e30141: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30730_cond_e30141 != 0.0
        } {
            assign30730_loop_guard += 1;
            assert!(assign30730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
                s.store_sqrt(726, 726);
            }
            let (assign30730_body1_e30168,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
        let assign30730_body1_e30166: f64 = (s.v[719] + 1.0);
        (assign30730_body1_e30166,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign30730_body1_e30168;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && (!s.b[1700])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1537, (-1.0), 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign30890_e30367,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30890_e30367;

        let (assign30900_e30377,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30900_e30377;

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
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

        let (assign31010_e30499,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && s.b[1707]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31010_e30499;

        s.b[1708] = (2.0 == 2.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        let (assign31030_e30519,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && s.b[1708]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31030_e30519;

        s.b[1709] = (2.0 == 4.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        let (assign31050_e30542,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1709]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31050_e30542;

        s.b[1710] = (2.0 == 8.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        let (assign31070_e30568,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1709])) && s.b[1710]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31070_e30568;

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        let (assign31080_e30580,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign31080_e30580;

        let mut assign31090_loop_guard: usize = 0;
        while {
            let assign31090_cond_e30593: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31090_cond_e30593 != 0.0
        } {
            assign31090_loop_guard += 1;
            assert!(assign31090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
                s.store_sqrt(726, 726);
            }
            let (assign31090_body1_e30620,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
        let assign31090_body1_e30618: f64 = (s.v[719] + 1.0);
        (assign31090_body1_e30618,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31090_body1_e30620;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && (!s.b[1706])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(790, 1537, 1.0, 335, (-1.0), 780, 1.0);
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
            s.copy_ad(1448, 965);
            s.copy_ad(1484, 790);
            s.copy_ad(1512, 790);
            s.store_sub_ad_rhs(1465, 1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)));
        }

        let (assign31360_e30913,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
        s.v[1510] = assign31360_e30913;

        let (assign31370_e30924,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1465],)
    } else {
        (s.v[1473],)
    }
};
        s.v[1473] = assign31370_e30924;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.store_mul(1499, 1448, 1546);
        }

        let (assign31390_e30948,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31390_e30948;

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
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
            }
            let (assign31400_body7_e31073,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31400_body7_e31073;
            let (assign31400_body8_e31086,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body8_e31086;
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
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
            let (assign31400_body19_e31235,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && s.b[1715]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body19_e31235;
            s.b[1716] = (2.0 == 2.0);
            s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };
            let (assign31400_body21_e31258,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && s.b[1716]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body21_e31258;
            s.b[1717] = (2.0 == 4.0);
            s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };
            let (assign31400_body23_e31284,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && s.b[1717]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body23_e31284;
            s.b[1718] = (2.0 == 8.0);
            s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };
            let (assign31400_body25_e31313,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && (!s.b[1717])) && s.b[1718]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body25_e31313;
            let (assign31400_body26_e31328,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31400_body26_e31328;
            let mut assign31400_body27_loop_guard: usize = 0;
            while {
                let assign31400_body27_cond_e31344: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body27_cond_e31344 != 0.0
            } {
                assign31400_body27_loop_guard += 1;
                assert!(assign31400_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31400_body27_body1_e31377,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
        let assign31400_body27_body1_e31375: f64 = (s.v[719] + 1.0);
        (assign31400_body27_body1_e31375,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31400_body27_body1_e31377;
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && (!s.b[1714])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1448, 965, (-1e-8), 780);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_add_scaled_inputs3_indices(335, 1465, 1.0, 1435, (-1.0), 1463, 1.0);
            }
            s.b[1719] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign31400_body43_e31623,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31400_body43_e31623;
            let (assign31400_body44_e31636,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body44_e31636;
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
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
            let (assign31400_body55_e31785,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && s.b[1721]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body55_e31785;
            s.b[1722] = (2.0 == 2.0);
            s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };
            let (assign31400_body57_e31808,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && s.b[1722]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body57_e31808;
            s.b[1723] = (2.0 == 4.0);
            s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };
            let (assign31400_body59_e31834,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && s.b[1723]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body59_e31834;
            s.b[1724] = (2.0 == 8.0);
            s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };
            let (assign31400_body61_e31863,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31400_body61_e31863;
            let (assign31400_body62_e31878,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31400_body62_e31878;
            let mut assign31400_body63_loop_guard: usize = 0;
            while {
                let assign31400_body63_cond_e31894: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body63_cond_e31894 != 0.0
            } {
                assign31400_body63_loop_guard += 1;
                assert!(assign31400_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31400_body63_body1_e31927,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
        let assign31400_body63_body1_e31925: f64 = (s.v[719] + 1.0);
        (assign31400_body63_body1_e31925,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31400_body63_body1_e31927;
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && (!s.b[1720])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
                s.store_add_scaled_product_right_ad(1486, 1465, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463)), (-1.0));
                s.store_scalar(1489, 0.0);
                s.store_scalar(1490, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
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
            let (assign31400_body94_e32433,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1726]) {
        let assign31400_body94_e32431: f64 = (150.0 + 1.0);
        (assign31400_body94_e32431,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31400_body94_e32433;
            let (assign31400_body95_e32444,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
            s.v[1510] = assign31400_body95_e32444;
            let (assign31400_body96_e32455,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1465],)
    } else {
        (s.v[1473],)
    }
};
            s.v[1473] = assign31400_body96_e32455;
            let (assign31400_body97_e32468,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        let assign31400_body97_e32466: f64 = (s.v[97] + 1.0);
        (assign31400_body97_e32466,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31400_body97_e32468;
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1514, 1465);
            s.store_mul(1452, 965, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_scaled_product_indices(1484, 1465, 1.0, 1548, 1543, 1.0);
            s.copy_ad(1462, 1484);
            s.copy_ad(1467, 1484);
        }

        let (assign31470_e32559,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1484],)
    } else {
        (s.v[1509],)
    }
};
        s.v[1509] = assign31470_e32559;

        s.b[1727] = (s.v[85] > s.v[1466]);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        let (assign31490_e32575,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1727]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.v[1479] = assign31490_e32575;

        s.b[1728] = (s.v[85] > s.v[1509]);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        let (assign31510_e32594,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && s.b[1728]) {
        (3.0,)
    } else {
        (s.v[1479],)
    }
};
        s.v[1479] = assign31510_e32594;

        let (assign31520_e32611,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && (!s.b[1728])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.v[1479] = assign31520_e32611;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
            s.copy_ad(1466, 790);
        }

        let (assign31540_e32635,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
        (s.v[1466],)
    } else {
        (s.v[1509],)
    }
};
        s.v[1509] = assign31540_e32635;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
            s.copy_ad(1467, 1466);
            s.copy_ad(1512, 1466);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
            s.copy_ad(1448, 1454);
            s.store_mul(1452, 1448, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_ad_lhs(1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)), 1465);
            s.copy_ad(1514, 1465);
        }

        s.b[1729] = (s.v[85] > s.v[1466]);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        let (assign31630_e32752,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1729]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.v[1479] = assign31630_e32752;

        let (assign31640_e32767,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1729])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.v[1479] = assign31640_e32767;

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1549, s.ad_value(1467), 1.0, s.ad_value(1435), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1730] = (s.v[335] > 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        let (assign31670_e32805,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1730]) {
        let assign31670_e32796: f64 = (-s.v[961]);
        let assign31670_e32798: f64 = (assign31670_e32796 + s.v[1435]);
        let assign31670_e32800: f64 = (s.v[335]).sqrt();
        let assign31670_e32802: f64 = (assign31670_e32800 / s.v[185]);
        let assign31670_e32803: f64 = (assign31670_e32798 - assign31670_e32802);
        (assign31670_e32803,)
    } else {
        (s.v[1455],)
    }
};
        s.v[1455] = assign31670_e32805;

        let (assign31680_e32820,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1730])) {
        let assign31680_e32816: f64 = (-s.v[961]);
        let assign31680_e32818: f64 = (assign31680_e32816 + s.v[1435]);
        (assign31680_e32818,)
    } else {
        (s.v[1455],)
    }
};
        s.v[1455] = assign31680_e32820;

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
            s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));
        }

        let (assign31800_e32975,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
        s.v[1471] = assign31800_e32975;

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);
        }

        s.b[1735] = (s.v[1481] > (s.v[1467] - s.v[1553]));
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1735]) {
            s.store_sub(1481, 1467, 1553);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1481)));
            s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
        }

        s.b[1736] = ((s.v[1450] + s.v[1448]) > s.v[965]);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        let (assign31870_e33102,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31870_e33102;

        let mut assign31880_loop_guard: usize = 0;
        while {
            let assign31880_cond_e33122: f64 = (150.0 + 1.0);
            let assign31880_cond_e33124: f64 = if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (s.v[97] <= assign31880_cond_e33122)) { 1.0 } else { 0.0 };
            assign31880_cond_e33124 != 0.0
        } {
            assign31880_loop_guard += 1;
            assert!(assign31880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_inputs3_indices(1468, 1450, 1.0, 1448, 1.0, 965, -1.0);
                s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1450), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1448)));
            }
            s.b[1737] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);
            s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1737]) {
                s.store_offset(1484, 1484, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1737])) {
                s.store_sub_div_rhs_indices(1484, 1484, 1468, 1508);
            }
            s.b[1738] = (((s.v[1484] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));
            s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1738]) {
                s.store_offset_sub(1484, 1435, 1463, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));
            }
            s.b[1739] = (s.v[335] > 0.0);
            s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1739]) {
                s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(s.ad_value(335)), 0.5, 1457, (-0.5), 1456, 1.0);
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1739])) {
                s.store_div_scaled_inputs_indices(1481, 1457, (-0.5), 1456, 1.0);
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
            }
            let (assign31880_body16_e33471,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {
        let assign31880_body16_e33469: f64 = (150.0 + 1.0);
        (assign31880_body16_e33469,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31880_body16_e33471;
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_sqrt_mul_ad(1450, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1481)));
                s.store_div_scaled_inputs2_mixed_aia(1465, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);
                s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
            }
            s.b[1742] = ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-8);
            s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };
            let (assign31880_body21_e33577,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1742]) {
        let assign31880_body21_e33575: f64 = (150.0 + 1.0);
        (assign31880_body21_e33575,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31880_body21_e33577;
            let (assign31880_body22_e33596,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
            s.v[1471] = assign31880_body22_e33596;
            let (assign31880_body23_e33617,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        let assign31880_body23_e33615: f64 = (s.v[97] + 1.0);
        (assign31880_body23_e33615,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31880_body23_e33617;
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && (!s.b[1734])) {
            s.copy_ad(1484, 1483);
            s.copy_ad(1465, 1464);
            s.copy_ad(1481, 1461);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.copy_ad(1482, 1484);
        }

        let (assign31930_e33689,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign31930_e33689;

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.copy_ad(1462, 1481);
            s.copy_ad(1484, 1482);
        }

        let (assign31960_e33716,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1462],)
    } else {
        (s.v[1474],)
    }
};
        s.v[1474] = assign31960_e33716;

        let (assign31970_e33725,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
        s.v[1471] = assign31970_e33725;

        let (assign31980_e33734,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31980_e33734;

    }

    pub(super) fn stamp_transient_block_26(
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
            }
            let (assign31990_body9_e33868,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body9_e33868;
            let (assign31990_body10_e33879,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body10_e33879;
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
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
            let (assign31990_body21_e34010,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && s.b[1745]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body21_e34010;
            s.b[1746] = (2.0 == 2.0);
            s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };
            let (assign31990_body23_e34031,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && s.b[1746]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body23_e34031;
            s.b[1747] = (2.0 == 4.0);
            s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };
            let (assign31990_body25_e34055,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && s.b[1747]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body25_e34055;
            s.b[1748] = (2.0 == 8.0);
            s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };
            let (assign31990_body27_e34082,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && (!s.b[1747])) && s.b[1748]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body27_e34082;
            let (assign31990_body28_e34095,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body28_e34095;
            let mut assign31990_body29_loop_guard: usize = 0;
            while {
                let assign31990_body29_cond_e34109: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body29_cond_e34109 != 0.0
            } {
                assign31990_body29_loop_guard += 1;
                assert!(assign31990_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31990_body29_body1_e34138,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
        let assign31990_body29_body1_e34136: f64 = (s.v[719] + 1.0);
        (assign31990_body29_body1_e34136,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31990_body29_body1_e34138;
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && (!s.b[1744])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
            }
            let (assign31990_body45_e34353,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body45_e34353;
            let (assign31990_body46_e34364,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body46_e34364;
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
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
            let (assign31990_body57_e34495,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && s.b[1751]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body57_e34495;
            s.b[1752] = (2.0 == 2.0);
            s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };
            let (assign31990_body59_e34516,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && s.b[1752]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body59_e34516;
            s.b[1753] = (2.0 == 4.0);
            s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };
            let (assign31990_body61_e34540,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && s.b[1753]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body61_e34540;
            s.b[1754] = (2.0 == 8.0);
            s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };
            let (assign31990_body63_e34567,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && (!s.b[1753])) && s.b[1754]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body63_e34567;
            let (assign31990_body64_e34580,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body64_e34580;
            let mut assign31990_body65_loop_guard: usize = 0;
            while {
                let assign31990_body65_cond_e34594: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body65_cond_e34594 != 0.0
            } {
                assign31990_body65_loop_guard += 1;
                assert!(assign31990_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31990_body65_body1_e34623,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
        let assign31990_body65_body1_e34621: f64 = (s.v[719] + 1.0);
        (assign31990_body65_body1_e34621,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31990_body65_body1_e34623;
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && (!s.b[1750])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1448, 965, (-1e-8), 780);
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
                s.store_mul_scaled_sqrt_ad_rhs(1476, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1524, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1476), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1526, 1524);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1755])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)));
                s.store_mul_sqrt_ad_rhs(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1476, 1.0);
                s.store_mul_add_ad_rhs(1524, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1526, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1756] = ((s.v[1520] > (s.v[1512] - s.v[1519])) && (s.v[1519] >= 0.0));
            s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
                s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1512, (-1.0), 1519, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1519);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign31990_body102_e35250,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body102_e35250;
            let (assign31990_body103_e35261,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body103_e35261;
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
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
            let (assign31990_body118_e35444,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && s.b[1758]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body118_e35444;
            s.b[1759] = (4.0 == 2.0);
            s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };
            let (assign31990_body120_e35465,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && s.b[1759]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body120_e35465;
            s.b[1760] = (4.0 == 4.0);
            s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };
            let (assign31990_body122_e35489,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && s.b[1760]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body122_e35489;
            s.b[1761] = (4.0 == 8.0);
            s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };
            let (assign31990_body124_e35516,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && (!s.b[1760])) && s.b[1761]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31990_body124_e35516;
            let (assign31990_body125_e35529,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31990_body125_e35529;
            let mut assign31990_body126_loop_guard: usize = 0;
            while {
                let assign31990_body126_cond_e35543: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body126_cond_e35543 != 0.0
            } {
                assign31990_body126_loop_guard += 1;
                assert!(assign31990_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31990_body126_body1_e35572,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
        let assign31990_body126_body1_e35570: f64 = (s.v[719] + 1.0);
        (assign31990_body126_body1_e35570,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31990_body126_body1_e35572;
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && (!s.b[1757])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1519, 726);
                s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1512, 1.0, 1519, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1476), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1462)), 1.0), 1.0, 1499, 1.0, 1500, 1.0);
                s.store_sub(1489, 1524, 185);
                s.store_add_scaled_inputs_products_indices(1490, 1526, 1.0, 1528, 1.0, 1530, 1534, 1.0, 1532, 1534, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
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
            let (assign31990_body152_e35971,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {
        let assign31990_body152_e35969: f64 = (150.0 + 1.0);
        (assign31990_body152_e35969,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31990_body152_e35971;
            let (assign31990_body153_e35982,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign31990_body153_e35982;
            let (assign31990_body154_e35991,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1462],)
    } else {
        (s.v[1474],)
    }
};
            s.v[1474] = assign31990_body154_e35991;
            let (assign31990_body155_e36000,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
            s.v[1471] = assign31990_body155_e36000;
            let (assign31990_body156_e36011,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        let assign31990_body156_e36009: f64 = (s.v[97] + 1.0);
        (assign31990_body156_e36009,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31990_body156_e36011;
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        s.b[1765] = ((s.v[1454] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        s.b[1766] = ((s.v[1484] > (s.v[1462] - 0.02)) && (0.02 >= 0.0));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
            s.store_offset_sub(781, 1484, 1462, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign32080_e36116,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32080_e36116;

        let (assign32090_e36129,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32090_e36129;

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
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

        let (assign32200_e36278,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && s.b[1768]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32200_e36278;

        s.b[1769] = (2.0 == 2.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        let (assign32220_e36301,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && s.b[1769]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32220_e36301;

        s.b[1770] = (2.0 == 4.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        let (assign32240_e36327,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && s.b[1770]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32240_e36327;

        s.b[1771] = (2.0 == 8.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        let (assign32260_e36356,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && (!s.b[1770])) && s.b[1771]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32260_e36356;

        let (assign32270_e36371,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32270_e36371;

        let mut assign32280_loop_guard: usize = 0;
        while {
            let assign32280_cond_e36387: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32280_cond_e36387 != 0.0
        } {
            assign32280_loop_guard += 1;
            assert!(assign32280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
                s.store_sqrt(726, 726);
            }
            let (assign32280_body1_e36420,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
        let assign32280_body1_e36418: f64 = (s.v[719] + 1.0);
        (assign32280_body1_e36418,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign32280_body1_e36420;
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && (!s.b[1767])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1484, 1462, (-0.02), 780);
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
            s.store_mul_scaled_sqrt_ad_rhs(1476, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
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
        }

        let (assign32520_e36764,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32520_e36764;

        let (assign32530_e36777,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32530_e36777;

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
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

        let (assign32640_e36926,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && s.b[1775]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32640_e36926;

        s.b[1776] = (2.0 == 2.0);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        let (assign32660_e36949,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && s.b[1776]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32660_e36949;

        s.b[1777] = (2.0 == 4.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        let (assign32680_e36975,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && s.b[1777]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32680_e36975;

        s.b[1778] = (2.0 == 8.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        let (assign32700_e37004,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && (!s.b[1777])) && s.b[1778]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32700_e37004;

        let (assign32710_e37019,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32710_e37019;

        let mut assign32720_loop_guard: usize = 0;
        while {
            let assign32720_cond_e37035: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32720_cond_e37035 != 0.0
        } {
            assign32720_loop_guard += 1;
            assert!(assign32720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
                s.store_sqrt(726, 726);
            }
            let (assign32720_body1_e37068,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
        let assign32720_body1_e37066: f64 = (s.v[719] + 1.0);
        (assign32720_body1_e37066,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign32720_body1_e37068;
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && (!s.b[1774])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1448, 965, (-1e-8), 780);
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
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)));
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
            s.store_add_scaled_product_right_ad(1478, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
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
        }

        let (assign33010_e37556,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33010_e37556;

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        let (assign33020_e37570,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33020_e37570;

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
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

        let (assign33130_e37728,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && s.b[1782]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33130_e37728;

        s.b[1783] = (2.0 == 2.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        let (assign33150_e37752,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && s.b[1783]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33150_e37752;

        s.b[1784] = (2.0 == 4.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        let (assign33170_e37779,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && s.b[1784]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33170_e37779;

        s.b[1785] = (2.0 == 8.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        let (assign33190_e37809,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && (!s.b[1784])) && s.b[1785]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33190_e37809;

        let (assign33200_e37825,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33200_e37825;

        let mut assign33210_loop_guard: usize = 0;
        while {
            let assign33210_cond_e37842: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33210_cond_e37842 != 0.0
        } {
            assign33210_loop_guard += 1;
            assert!(assign33210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
                s.store_sqrt(726, 726);
            }
            let (assign33210_body1_e37877,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
        let assign33210_body1_e37875: f64 = (s.v[719] + 1.0);
        (assign33210_body1_e37875,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33210_body1_e37877;
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && (!s.b[1781])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

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
        }

        let (assign33370_e38137,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33370_e38137;

        let (assign33380_e38151,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33380_e38151;

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
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

        let (assign33490_e38309,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && s.b[1788]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33490_e38309;

        s.b[1789] = (2.0 == 2.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        let (assign33510_e38333,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && s.b[1789]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33510_e38333;

        s.b[1790] = (2.0 == 4.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        let (assign33530_e38360,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && s.b[1790]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33530_e38360;

        s.b[1791] = (2.0 == 8.0);
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        let (assign33550_e38390,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && (!s.b[1790])) && s.b[1791]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33550_e38390;

        let (assign33560_e38406,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33560_e38406;

        let mut assign33570_loop_guard: usize = 0;
        while {
            let assign33570_cond_e38423: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33570_cond_e38423 != 0.0
        } {
            assign33570_loop_guard += 1;
            assert!(assign33570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
                s.store_sqrt(726, 726);
            }
            let (assign33570_body1_e38458,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
        let assign33570_body1_e38456: f64 = (s.v[719] + 1.0);
        (assign33570_body1_e38456,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33570_body1_e38458;
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && (!s.b[1787])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1448, 965, (-1e-8), 780);
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
        }

        let (assign33750_e38737,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33750_e38737;

        let (assign33760_e38748,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33760_e38748;

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
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

        let (assign33870_e38879,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && s.b[1794]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33870_e38879;

        s.b[1795] = (2.0 == 2.0);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        let (assign33890_e38900,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && s.b[1795]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33890_e38900;

        s.b[1796] = (2.0 == 4.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        let (assign33910_e38924,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && s.b[1796]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33910_e38924;

        s.b[1797] = (2.0 == 8.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        let (assign33930_e38951,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33930_e38951;

        let (assign33940_e38964,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33940_e38964;

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign33950_loop_guard: usize = 0;
        while {
            let assign33950_cond_e38978: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33950_cond_e38978 != 0.0
        } {
            assign33950_loop_guard += 1;
            assert!(assign33950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
                s.store_sqrt(726, 726);
            }
            let (assign33950_body1_e39007,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
        let assign33950_body1_e39005: f64 = (s.v[719] + 1.0);
        (assign33950_body1_e39005,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33950_body1_e39007;
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && (!s.b[1793])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1516, 209, -1.0, 338);
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
        }

        let (assign34160_e39258,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34160_e39258;

        let (assign34170_e39266,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34170_e39266;

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
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

        let (assign34280_e39370,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && s.b[1800]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34280_e39370;

        s.b[1801] = (2.0 == 2.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        let (assign34300_e39388,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && s.b[1801]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34300_e39388;

        s.b[1802] = (2.0 == 4.0);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        let (assign34320_e39409,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && s.b[1802]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34320_e39409;

        s.b[1803] = (2.0 == 8.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        let (assign34340_e39433,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && (!s.b[1802])) && s.b[1803]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34340_e39433;

        let (assign34350_e39443,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34350_e39443;

        let mut assign34360_loop_guard: usize = 0;
        while {
            let assign34360_cond_e39454: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign34360_cond_e39454 != 0.0
        } {
            assign34360_loop_guard += 1;
            assert!(assign34360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
                s.store_sqrt(726, 726);
            }
            let (assign34360_body1_e39477,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
        let assign34360_body1_e39475: f64 = (s.v[719] + 1.0);
        (assign34360_body1_e39475,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign34360_body1_e39477;
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1798]) && (!s.b[1799])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1540, 726);
            s.store_div_scaled_product3_indices(334, 1540, 725, 726, 1.0, 770, 1.0);
            s.store_sub(1556, 1540, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1798])) {
            s.copy_ad(1556, 335);
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1443] && s.b[1444]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
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
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
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
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
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
            s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1438, -1.0);
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
        }

        let (assign34970_e40147,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34970_e40147;

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign34980_e40157,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34980_e40157;

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
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

        let (assign35090_e40279,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && s.b[1811]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35090_e40279;

        s.b[1812] = (2.0 == 2.0);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        let (assign35110_e40299,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && s.b[1812]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35110_e40299;

        s.b[1813] = (2.0 == 4.0);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        let (assign35130_e40322,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && s.b[1813]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35130_e40322;

        s.b[1814] = (2.0 == 8.0);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        let (assign35150_e40348,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && (!s.b[1813])) && s.b[1814]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35150_e40348;

        let (assign35160_e40360,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35160_e40360;

        let mut assign35170_loop_guard: usize = 0;
        while {
            let assign35170_cond_e40373: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35170_cond_e40373 != 0.0
        } {
            assign35170_loop_guard += 1;
            assert!(assign35170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
                s.store_sqrt(726, 726);
            }
            let (assign35170_body1_e40400,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
        let assign35170_body1_e40398: f64 = (s.v[719] + 1.0);
        (assign35170_body1_e40398,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign35170_body1_e40400;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && (!s.b[1810])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);
            s.store_mul(344, 344, 975);
        }

        s.b[1815] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
            s.store_sub_offset_lhs(781, 972, 4.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (4.0 * 4.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign35370_e40642,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35370_e40642;

        let (assign35380_e40652,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35380_e40652;

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
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

        let (assign35530_e40822,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && s.b[1817]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35530_e40822;

        s.b[1818] = (4.0 == 2.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        let (assign35550_e40842,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && s.b[1818]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35550_e40842;

        s.b[1819] = (4.0 == 4.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        let (assign35570_e40865,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35570_e40865;

        s.b[1820] = (4.0 == 8.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        let (assign35590_e40891,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && (!s.b[1819])) && s.b[1820]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35590_e40891;

        let (assign35600_e40903,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35600_e40903;

        let mut assign35610_loop_guard: usize = 0;
        while {
            let assign35610_cond_e40916: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35610_cond_e40916 != 0.0
        } {
            assign35610_loop_guard += 1;
            assert!(assign35610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
                s.store_sqrt(726, 726);
            }
            let (assign35610_body1_e40943,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
        let assign35610_body1_e40941: f64 = (s.v[719] + 1.0);
        (assign35610_body1_e40941,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign35610_body1_e40943;
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && (!s.b[1816])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);
            s.store_sub_offset_lhs(344, 972, 4.0, 780);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1555, 1557, 170);
            s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset(337, 336, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && s.b[1444]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1555, 1557, 170);
            s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_div(1507, 254, 338);
            s.store_mul3_affine_lhs(1554, 1505, 1507, (-s.v[632]), 0.0, 1555);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1554, 1.0);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1821] = (p.p283 != 0.0);
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1821]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
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
            s.store_add_scaled_inputs3_indices(339, 1461, 1.0, 340, 1.0, 1438, -1.0);
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
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1826])) {
            s.copy_ad(341, 647);
        }

        s.b[1827] = (s.v[793] >= 0.0);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1827]) {
            s.copy_ad(369, 793);
        }

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
            s.store_add_scaled_inputs4_indices(131, 1477, (-0.5), 1478, (-0.5), 1498, (-0.5), 1500, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(1517), 1.0, s.ad_value(1518), 1.0), s.ad_value(1497)), 1499, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1538, 1539, (-0.5));
            s.store_neg(238, 1538);
            s.copy_ad(255, 1558);
        }

        s.b[1829] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        let (assign36980_e42540,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1829]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign36980_e42540;

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
        }

    }
}
