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
            }
            let (assign25250_body9_e21808,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body9_e21808;
            let (assign25250_body10_e21816,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body10_e21816;
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
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
            let (assign25250_body21_e21920,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && s.b[1592]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body21_e21920;
            s.b[1593] = (2.0 == 2.0);
            s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };
            let (assign25250_body23_e21938,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && s.b[1593]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body23_e21938;
            s.b[1594] = (2.0 == 4.0);
            s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };
            let (assign25250_body25_e21959,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && s.b[1594]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body25_e21959;
            s.b[1595] = (2.0 == 8.0);
            s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };
            let (assign25250_body27_e21983,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && (!s.b[1594])) && s.b[1595]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body27_e21983;
            let (assign25250_body28_e21993,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body28_e21993;
            let mut assign25250_body29_loop_guard: usize = 0;
            while {
                let assign25250_body29_cond_e22004: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body29_cond_e22004 != 0.0
            } {
                assign25250_body29_loop_guard += 1;
                assert!(assign25250_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25250_body29_body1_e22027,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
        let assign25250_body29_body1_e22025: f64 = (s.v[719] + 1.0);
        (assign25250_body29_body1_e22025,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25250_body29_body1_e22027;
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1590]) && (!s.b[1591])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
            }
            let (assign25250_body45_e22197,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body45_e22197;
            let (assign25250_body46_e22205,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body46_e22205;
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
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
            let (assign25250_body57_e22309,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body57_e22309;
            s.b[1599] = (2.0 == 2.0);
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            let (assign25250_body59_e22327,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && s.b[1599]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body59_e22327;
            s.b[1600] = (2.0 == 4.0);
            s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };
            let (assign25250_body61_e22348,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && s.b[1600]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body61_e22348;
            s.b[1601] = (2.0 == 8.0);
            s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };
            let (assign25250_body63_e22372,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && (!s.b[1600])) && s.b[1601]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body63_e22372;
            let (assign25250_body64_e22382,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body64_e22382;
            let mut assign25250_body65_loop_guard: usize = 0;
            while {
                let assign25250_body65_cond_e22393: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body65_cond_e22393 != 0.0
            } {
                assign25250_body65_loop_guard += 1;
                assert!(assign25250_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25250_body65_body1_e22416,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
        let assign25250_body65_body1_e22414: f64 = (s.v[719] + 1.0);
        (assign25250_body65_body1_e22414,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25250_body65_body1_e22416;
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1596]) && (!s.b[1597])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
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
                s.store_div_scaled_inputs_product(1516, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1460), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1538), s.ad_value(1457), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1457), s.ad_value(1457), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1457), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), s.ad_value(1539), 1.0, s.ad_value(335), 1.0);
                s.store_div_ad_lhs(1517, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1457), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_div_ad_lhs(1518, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1457), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1457, 1479);
                s.store_exp(336, 335);
            }
            s.b[1602] = (s.v[1457] >= s.v[1479]);
            s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1602]) {
                s.store_mul_scaled_sqrt_ad_rhs(1471, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(1519, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 1471, 1.0);
                s.store_neg(1521, 1519);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1602])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)));
                s.store_mul_sqrt_ad_rhs(1471, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1471, 1.0);
                s.store_mul_add_ad_rhs(1519, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1521, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1603] = ((s.v[1516] > (s.v[1507] - s.v[1515])) && (s.v[1515] >= 0.0));
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_add_scaled_inputs3_indices(781, 1516, 1.0, 1507, (-1.0), 1515, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1515);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign25250_body102_e22938,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body102_e22938;
            let (assign25250_body103_e22946,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body103_e22946;
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
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
            let (assign25250_body118_e23090,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && s.b[1605]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body118_e23090;
            s.b[1606] = (4.0 == 2.0);
            s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };
            let (assign25250_body120_e23108,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body120_e23108;
            s.b[1607] = (4.0 == 4.0);
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            let (assign25250_body122_e23129,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1607]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body122_e23129;
            s.b[1608] = (4.0 == 8.0);
            s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };
            let (assign25250_body124_e23153,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1607])) && s.b[1608]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign25250_body124_e23153;
            let (assign25250_body125_e23163,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25250_body125_e23163;
            let mut assign25250_body126_loop_guard: usize = 0;
            while {
                let assign25250_body126_cond_e23174: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body126_cond_e23174 != 0.0
            } {
                assign25250_body126_loop_guard += 1;
                assert!(assign25250_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
                    s.store_sqrt(726, 726);
                }
                let (assign25250_body126_body1_e23197,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
        let assign25250_body126_body1_e23195: f64 = (s.v[719] + 1.0);
        (assign25250_body126_body1_e23195,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign25250_body126_body1_e23197;
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1603]) && (!s.b[1604])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1515, 726);
                s.store_div_scaled_product3_indices(334, 1515, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1507, 1.0, 1515, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_mixed_aii(1482, A::add_scaled_product(s.ad_value(1471), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1457)), 1.0), 1.0, 1493, 1.0, 1494, 1.0);
                s.store_sub(1485, 1519, 185);
                s.store_add_scaled_inputs_products_indices(1486, 1521, 1.0, 1523, 1.0, 1525, 1529, 1.0, 1527, 1529, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
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
            let (assign25250_body152_e23524,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1610]) {
        let assign25250_body152_e23522: f64 = (150.0 + 1.0);
        (assign25250_body152_e23522,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign25250_body152_e23524;
            let (assign25250_body153_e23532,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1610]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign25250_body153_e23532;
            let (assign25250_body154_e23538,) = {
    if (s.b[1439] && s.b[1440]) {
        (s.v[1457],)
    } else {
        (s.v[1465],)
    }
};
            s.v[1465] = assign25250_body154_e23538;
            let (assign25250_body155_e23544,) = {
    if (s.b[1439] && s.b[1440]) {
        (s.v[1479],)
    } else {
        (s.v[1466],)
    }
};
            s.v[1466] = assign25250_body155_e23544;
            let (assign25250_body156_e23552,) = {
    if (s.b[1439] && s.b[1440]) {
        let assign25250_body156_e23550: f64 = (s.v[97] + 1.0);
        (assign25250_body156_e23550,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign25250_body156_e23552;
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        s.b[1612] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        s.b[1613] = ((s.v[1479] > (s.v[1457] - 0.02)) && (0.02 >= 0.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_offset_sub(781, 1479, 1457, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign25340_e23639,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25340_e23639;

        let (assign25350_e23649,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25350_e23649;

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
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

        let (assign25460_e23771,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && s.b[1615]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25460_e23771;

        s.b[1616] = (2.0 == 2.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        let (assign25480_e23791,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && s.b[1616]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25480_e23791;

        s.b[1617] = (2.0 == 4.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        let (assign25500_e23814,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1617]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25500_e23814;

        s.b[1618] = (2.0 == 8.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        let (assign25520_e23840,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1617])) && s.b[1618]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25520_e23840;

        let (assign25530_e23852,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25530_e23852;

        let mut assign25540_loop_guard: usize = 0;
        while {
            let assign25540_cond_e23865: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25540_cond_e23865 != 0.0
        } {
            assign25540_loop_guard += 1;
            assert!(assign25540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
                s.store_sqrt(726, 726);
            }
            let (assign25540_body1_e23892,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
        let assign25540_body1_e23890: f64 = (s.v[719] + 1.0);
        (assign25540_body1_e23890,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25540_body1_e23892;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && (!s.b[1614])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1479, 1457, (-0.02), 780);
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
            s.store_mul_scaled_sqrt_ad_rhs(1471, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
            s.copy_ad(1534, 1471);
            s.store_scalar(1513, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
        }

        s.b[1620] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign25780_e24170,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25780_e24170;

        let (assign25790_e24180,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25790_e24180;

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
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

        let (assign25900_e24302,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && s.b[1622]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25900_e24302;

        s.b[1623] = (2.0 == 2.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        let (assign25920_e24322,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && s.b[1623]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25920_e24322;

        s.b[1624] = (2.0 == 4.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        let (assign25940_e24345,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1624]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25940_e24345;

        s.b[1625] = (2.0 == 8.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        let (assign25960_e24371,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1624])) && s.b[1625]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign25960_e24371;

        let (assign25970_e24383,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign25970_e24383;

        let mut assign25980_loop_guard: usize = 0;
        while {
            let assign25980_cond_e24396: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25980_cond_e24396 != 0.0
        } {
            assign25980_loop_guard += 1;
            assert!(assign25980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
                s.store_sqrt(726, 726);
            }
            let (assign25980_body1_e24423,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
        let assign25980_body1_e24421: f64 = (s.v[719] + 1.0);
        (assign25980_body1_e24421,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign25980_body1_e24423;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
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
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)));
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
            s.store_add_scaled_product_right_ad(1473, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
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
        }

        let (assign26270_e24830,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26270_e24830;

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        let (assign26280_e24841,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26280_e24841;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
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

        let (assign26390_e24972,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && s.b[1629]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26390_e24972;

        s.b[1630] = (2.0 == 2.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        let (assign26410_e24993,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && s.b[1630]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26410_e24993;

        s.b[1631] = (2.0 == 4.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        let (assign26430_e25017,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && s.b[1631]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26430_e25017;

        s.b[1632] = (2.0 == 8.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        let (assign26450_e25044,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && (!s.b[1631])) && s.b[1632]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26450_e25044;

        let (assign26460_e25057,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26460_e25057;

        let mut assign26470_loop_guard: usize = 0;
        while {
            let assign26470_cond_e25071: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26470_cond_e25071 != 0.0
        } {
            assign26470_loop_guard += 1;
            assert!(assign26470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
                s.store_sqrt(726, 726);
            }
            let (assign26470_body1_e25100,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
        let assign26470_body1_e25098: f64 = (s.v[719] + 1.0);
        (assign26470_body1_e25098,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign26470_body1_e25100;
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && (!s.b[1628])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
        }

        let (assign26630_e25315,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26630_e25315;

        let (assign26640_e25326,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26640_e25326;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
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

        let (assign26750_e25457,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && s.b[1635]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26750_e25457;

        s.b[1636] = (2.0 == 2.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        let (assign26770_e25478,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && s.b[1636]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26770_e25478;

        s.b[1637] = (2.0 == 4.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        let (assign26790_e25502,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && s.b[1637]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26790_e25502;

        s.b[1638] = (2.0 == 8.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        let (assign26810_e25529,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && (!s.b[1637])) && s.b[1638]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign26810_e25529;

        let (assign26820_e25542,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign26820_e25542;

        let mut assign26830_loop_guard: usize = 0;
        while {
            let assign26830_cond_e25556: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26830_cond_e25556 != 0.0
        } {
            assign26830_loop_guard += 1;
            assert!(assign26830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
                s.store_sqrt(726, 726);
            }
            let (assign26830_body1_e25585,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
        let assign26830_body1_e25583: f64 = (s.v[719] + 1.0);
        (assign26830_body1_e25583,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign26830_body1_e25585;
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && (!s.b[1634])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
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
        }

        let (assign27020_e25817,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27020_e25817;

        let (assign27030_e25825,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27030_e25825;

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
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

        let (assign27140_e25929,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27140_e25929;

        s.b[1642] = (2.0 == 2.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        let (assign27160_e25947,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27160_e25947;

        s.b[1643] = (2.0 == 4.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        let (assign27180_e25968,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && s.b[1643]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27180_e25968;

        s.b[1644] = (2.0 == 8.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        let (assign27200_e25992,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && (!s.b[1643])) && s.b[1644]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27200_e25992;

        let (assign27210_e26002,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27210_e26002;

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        let mut assign27220_loop_guard: usize = 0;
        while {
            let assign27220_cond_e26013: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27220_cond_e26013 != 0.0
        } {
            assign27220_loop_guard += 1;
            assert!(assign27220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
                s.store_sqrt(726, 726);
            }
            let (assign27220_body1_e26036,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
        let assign27220_body1_e26034: f64 = (s.v[719] + 1.0);
        (assign27220_body1_e26034,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27220_body1_e26036;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1639]) && (!s.b[1640])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
        }

        let (assign27380_e26206,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27380_e26206;

        let (assign27390_e26214,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27390_e26214;

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
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

        let (assign27500_e26318,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && s.b[1647]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27500_e26318;

        s.b[1648] = (2.0 == 2.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        let (assign27520_e26336,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && s.b[1648]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27520_e26336;

        s.b[1649] = (2.0 == 4.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        let (assign27540_e26357,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && s.b[1649]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27540_e26357;

        s.b[1650] = (2.0 == 8.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        let (assign27560_e26381,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && (!s.b[1649])) && s.b[1650]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27560_e26381;

        let (assign27570_e26391,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27570_e26391;

        let mut assign27580_loop_guard: usize = 0;
        while {
            let assign27580_cond_e26402: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27580_cond_e26402 != 0.0
        } {
            assign27580_loop_guard += 1;
            assert!(assign27580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
                s.store_sqrt(726, 726);
            }
            let (assign27580_body1_e26425,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
        let assign27580_body1_e26423: f64 = (s.v[719] + 1.0);
        (assign27580_body1_e26423,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27580_body1_e26425;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1645]) && (!s.b[1646])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
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
            s.store_square(722, 781);
            s.store_scalar(723, (0.05 * 0.05));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign27740_e26594,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27740_e26594;

        let (assign27750_e26602,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27750_e26602;

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
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

        let (assign27860_e26706,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && s.b[1653]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27860_e26706;

        s.b[1654] = (2.0 == 2.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        let (assign27880_e26724,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && s.b[1654]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27880_e26724;

        s.b[1655] = (2.0 == 4.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        let (assign27900_e26745,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && s.b[1655]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27900_e26745;

        s.b[1656] = (2.0 == 8.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        let (assign27920_e26769,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1655])) && s.b[1656]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign27920_e26769;

        let (assign27930_e26779,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign27930_e26779;

        let mut assign27940_loop_guard: usize = 0;
        while {
            let assign27940_cond_e26790: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27940_cond_e26790 != 0.0
        } {
            assign27940_loop_guard += 1;
            assert!(assign27940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
                s.store_sqrt(726, 726);
            }
            let (assign27940_body1_e26813,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
        let assign27940_body1_e26811: f64 = (s.v[719] + 1.0);
        (assign27940_body1_e26811,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign27940_body1_e26813;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1651]) && (!s.b[1652])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1443, (-1.0), 1445, -1.0);
        }

        s.b[1657] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-18 * 1e-18));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign28110_e26993,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28110_e26993;

        let (assign28120_e27001,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28120_e27001;

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1658] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        s.b[1659] = (2.0 == 1.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        let (assign28230_e27105,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && s.b[1659]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28230_e27105;

        s.b[1660] = (2.0 == 2.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        let (assign28250_e27123,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && s.b[1660]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28250_e27123;

        s.b[1661] = (2.0 == 4.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        let (assign28270_e27144,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && s.b[1661]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28270_e27144;

        s.b[1662] = (2.0 == 8.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        let (assign28290_e27168,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && (!s.b[1661])) && s.b[1662]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28290_e27168;

        let (assign28300_e27178,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28300_e27178;

        let mut assign28310_loop_guard: usize = 0;
        while {
            let assign28310_cond_e27189: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28310_cond_e27189 != 0.0
        } {
            assign28310_loop_guard += 1;
            assert!(assign28310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
                s.store_sqrt(726, 726);
            }
            let (assign28310_body1_e27212,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
        let assign28310_body1_e27210: f64 = (s.v[719] + 1.0);
        (assign28310_body1_e27210,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign28310_body1_e27212;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1657]) && (!s.b[1658])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);
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
        }

        let (assign28480_e27401,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28480_e27401;

        let (assign28490_e27411,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28490_e27411;

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
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

        let (assign28600_e27533,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && s.b[1666]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28600_e27533;

        s.b[1667] = (2.0 == 2.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        let (assign28620_e27553,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && s.b[1667]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28620_e27553;

        s.b[1668] = (2.0 == 4.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        let (assign28640_e27576,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1668]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28640_e27576;

        s.b[1669] = (2.0 == 8.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        let (assign28660_e27602,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28660_e27602;

        let (assign28670_e27614,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28670_e27614;

        let mut assign28680_loop_guard: usize = 0;
        while {
            let assign28680_cond_e27627: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28680_cond_e27627 != 0.0
        } {
            assign28680_loop_guard += 1;
            assert!(assign28680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
                s.store_sqrt(726, 726);
            }
            let (assign28680_body1_e27654,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
        let assign28680_body1_e27652: f64 = (s.v[719] + 1.0);
        (assign28680_body1_e27652,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign28680_body1_e27654;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1507, (-0.8), 780);
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
        }

        let (assign28830_e27849,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign28830_e27849;

        let (assign28840_e27860,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28840_e27860;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        s.b[1672] = (2.0 == 1.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        let (assign28950_e27991,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && s.b[1672]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28950_e27991;

        s.b[1673] = (2.0 == 2.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        let (assign28970_e28012,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && s.b[1673]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28970_e28012;

        s.b[1674] = (2.0 == 4.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        let (assign28990_e28036,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && s.b[1674]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign28990_e28036;

        s.b[1675] = (2.0 == 8.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        let (assign29010_e28063,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && (!s.b[1674])) && s.b[1675]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29010_e28063;

        let (assign29020_e28076,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29020_e28076;

        let mut assign29030_loop_guard: usize = 0;
        while {
            let assign29030_cond_e28090: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29030_cond_e28090 != 0.0
        } {
            assign29030_loop_guard += 1;
            assert!(assign29030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
                s.store_sqrt(726, 726);
            }
            let (assign29030_body1_e28119,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
        let assign29030_body1_e28117: f64 = (s.v[719] + 1.0);
        (assign29030_body1_e28117,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29030_body1_e28119;
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && (!s.b[1671])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1507, (-0.8), 780);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
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
        }

        let (assign29190_e28326,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29190_e28326;

        let (assign29200_e28334,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29200_e28334;

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
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

        let (assign29310_e28438,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && s.b[1678]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29310_e28438;

        s.b[1679] = (2.0 == 2.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        let (assign29330_e28456,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && s.b[1679]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29330_e28456;

        s.b[1680] = (2.0 == 4.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        let (assign29350_e28477,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && s.b[1680]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29350_e28477;

        s.b[1681] = (2.0 == 8.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        let (assign29370_e28501,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && (!s.b[1680])) && s.b[1681]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29370_e28501;

        let (assign29380_e28511,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29380_e28511;

        let mut assign29390_loop_guard: usize = 0;
        while {
            let assign29390_cond_e28522: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29390_cond_e28522 != 0.0
        } {
            assign29390_loop_guard += 1;
            assert!(assign29390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
                s.store_sqrt(726, 726);
            }
            let (assign29390_body1_e28545,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
        let assign29390_body1_e28543: f64 = (s.v[719] + 1.0);
        (assign29390_body1_e28543,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29390_body1_e28545;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1676]) && (!s.b[1677])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1676])) {
            s.store_sub(336, 1457, 1507);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1511, 209, -1.0, 338);
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1536, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1682] = (s.v[790] > 1e-6);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_div_square_rhs(336, 1498, 185);
            s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1434, -1.0, 2.0);
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
        }

        let (assign29630_e28822,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29630_e28822;

        let (assign29640_e28832,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29640_e28832;

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
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

        let (assign29750_e28954,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && s.b[1685]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29750_e28954;

        s.b[1686] = (2.0 == 2.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        let (assign29770_e28974,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && s.b[1686]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29770_e28974;

        s.b[1687] = (2.0 == 4.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        let (assign29790_e28997,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && s.b[1687]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29790_e28997;

        s.b[1688] = (2.0 == 8.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        let (assign29810_e29023,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && (!s.b[1687])) && s.b[1688]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign29810_e29023;

        let (assign29820_e29035,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign29820_e29035;

        let mut assign29830_loop_guard: usize = 0;
        while {
            let assign29830_cond_e29048: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29830_cond_e29048 != 0.0
        } {
            assign29830_loop_guard += 1;
            assert!(assign29830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
                s.store_sqrt(726, 726);
            }
            let (assign29830_body1_e29075,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
        let assign29830_body1_e29073: f64 = (s.v[719] + 1.0);
        (assign29830_body1_e29073,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign29830_body1_e29075;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && (!s.b[1684])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
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
            s.store_add_offset_lhs(344, 85, 2.0, 338);
        }

        s.b[1689] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_sub_from_scalar(781, (0.3 + 0.2), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign30020_e29307,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30020_e29307;

        let (assign30030_e29317,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30030_e29317;

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
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
        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_mul(725, 725, 723);
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

        let (assign30180_e29487,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && s.b[1691]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30180_e29487;

        s.b[1692] = (4.0 == 2.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        let (assign30200_e29507,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && s.b[1692]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30200_e29507;

        s.b[1693] = (4.0 == 4.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        let (assign30220_e29530,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && (!s.b[1692])) && s.b[1693]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30220_e29530;

        s.b[1694] = (4.0 == 8.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        let (assign30240_e29556,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && (!s.b[1692])) && (!s.b[1693])) && s.b[1694]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30240_e29556;

        let (assign30250_e29568,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30250_e29568;

        let mut assign30260_loop_guard: usize = 0;
        while {
            let assign30260_cond_e29581: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30260_cond_e29581 != 0.0
        } {
            assign30260_loop_guard += 1;
            assert!(assign30260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) {
                s.store_sqrt(726, 726);
            }
            let (assign30260_body1_e29608,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) {
        let assign30260_body1_e29606: f64 = (s.v[719] + 1.0);
        (assign30260_body1_e29606,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign30260_body1_e29608;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && (!s.b[1690])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
        }

        let (assign30490_e29895,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30490_e29895;

        let (assign30500_e29905,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30500_e29905;

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
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

        let (assign30610_e30027,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && s.b[1697]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30610_e30027;

        s.b[1698] = (2.0 == 2.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        let (assign30630_e30047,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && s.b[1698]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30630_e30047;

        s.b[1699] = (2.0 == 4.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        let (assign30650_e30070,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && (!s.b[1698])) && s.b[1699]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30650_e30070;

        s.b[1700] = (2.0 == 8.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        let (assign30670_e30096,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && (!s.b[1698])) && (!s.b[1699])) && s.b[1700]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30670_e30096;

        let (assign30680_e30108,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30680_e30108;

        let mut assign30690_loop_guard: usize = 0;
        while {
            let assign30690_cond_e30121: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30690_cond_e30121 != 0.0
        } {
            assign30690_loop_guard += 1;
            assert!(assign30690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) {
                s.store_sqrt(726, 726);
            }
            let (assign30690_body1_e30148,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) {
        let assign30690_body1_e30146: f64 = (s.v[719] + 1.0);
        (assign30690_body1_e30146,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign30690_body1_e30148;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && (!s.b[1696])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1533, (-1.0), 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign30850_e30347,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign30850_e30347;

        let (assign30860_e30357,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30860_e30357;

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
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

        let (assign30970_e30479,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && s.b[1703]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30970_e30479;

        s.b[1704] = (2.0 == 2.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        let (assign30990_e30499,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && s.b[1704]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign30990_e30499;

        s.b[1705] = (2.0 == 4.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        let (assign31010_e30522,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && (!s.b[1704])) && s.b[1705]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31010_e30522;

        s.b[1706] = (2.0 == 8.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        let (assign31030_e30548,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && (!s.b[1704])) && (!s.b[1705])) && s.b[1706]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign31030_e30548;

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        let (assign31040_e30560,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign31040_e30560;

        let mut assign31050_loop_guard: usize = 0;
        while {
            let assign31050_cond_e30573: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31050_cond_e30573 != 0.0
        } {
            assign31050_loop_guard += 1;
            assert!(assign31050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) {
                s.store_sqrt(726, 726);
            }
            let (assign31050_body1_e30600,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) {
        let assign31050_body1_e30598: f64 = (s.v[719] + 1.0);
        (assign31050_body1_e30598,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31050_body1_e30600;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && (!s.b[1702])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(790, 1533, 1.0, 335, (-1.0), 780, 1.0);
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
            s.copy_ad(1444, 965);
            s.copy_ad(1480, 790);
            s.copy_ad(1508, 790);
            s.store_sub_ad_rhs(1461, 1480, A::mul3(s.ad_value(1544), s.ad_value(1444), s.ad_value(1444)));
        }

        let (assign31320_e30893,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (s.v[1462],)
    } else {
        (s.v[1506],)
    }
};
        s.v[1506] = assign31320_e30893;

        let (assign31330_e30904,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (s.v[1461],)
    } else {
        (s.v[1469],)
    }
};
        s.v[1469] = assign31330_e30904;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
            s.store_mul(1495, 1444, 1542);
        }

        let (assign31350_e30928,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31350_e30928;

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        let mut assign31360_loop_guard: usize = 0;
        while {
            let assign31360_cond_e30940: f64 = (150.0 + 1.0);
            let assign31360_cond_e30942: f64 = if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (s.v[97] <= assign31360_cond_e30940)) { 1.0 } else { 0.0 };
            assign31360_cond_e30942 != 0.0
        } {
            assign31360_loop_guard += 1;
            assert!(assign31360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);
            }
            s.b[1709] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
                s.store_offset_sub(781, 1444, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign31360_body7_e31053,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31360_body7_e31053;
            let (assign31360_body8_e31066,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body8_e31066;
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
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
            let (assign31360_body19_e31215,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && s.b[1711]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body19_e31215;
            s.b[1712] = (2.0 == 2.0);
            s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };
            let (assign31360_body21_e31238,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && s.b[1712]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body21_e31238;
            s.b[1713] = (2.0 == 4.0);
            s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };
            let (assign31360_body23_e31264,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1713]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body23_e31264;
            s.b[1714] = (2.0 == 8.0);
            s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };
            let (assign31360_body25_e31293,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1713])) && s.b[1714]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body25_e31293;
            let (assign31360_body26_e31308,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31360_body26_e31308;
            let mut assign31360_body27_loop_guard: usize = 0;
            while {
                let assign31360_body27_cond_e31324: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31360_body27_cond_e31324 != 0.0
            } {
                assign31360_body27_loop_guard += 1;
                assert!(assign31360_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31360_body27_body1_e31357,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) {
        let assign31360_body27_body1_e31355: f64 = (s.v[719] + 1.0);
        (assign31360_body27_body1_e31355,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31360_body27_body1_e31357;
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && (!s.b[1710])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1444, 965, (-1e-8), 780);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1709])) {
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1709])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.store_add_scaled_inputs3_indices(335, 1461, 1.0, 1431, (-1.0), 1459, 1.0);
            }
            s.b[1715] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign31360_body43_e31603,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31360_body43_e31603;
            let (assign31360_body44_e31616,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body44_e31616;
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
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
            let (assign31360_body55_e31765,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && s.b[1717]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body55_e31765;
            s.b[1718] = (2.0 == 2.0);
            s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };
            let (assign31360_body57_e31788,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && s.b[1718]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body57_e31788;
            s.b[1719] = (2.0 == 4.0);
            s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };
            let (assign31360_body59_e31814,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && (!s.b[1718])) && s.b[1719]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body59_e31814;
            s.b[1720] = (2.0 == 8.0);
            s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };
            let (assign31360_body61_e31843,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && (!s.b[1718])) && (!s.b[1719])) && s.b[1720]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31360_body61_e31843;
            let (assign31360_body62_e31858,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31360_body62_e31858;
            let mut assign31360_body63_loop_guard: usize = 0;
            while {
                let assign31360_body63_cond_e31874: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31360_body63_cond_e31874 != 0.0
            } {
                assign31360_body63_loop_guard += 1;
                assert!(assign31360_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31360_body63_body1_e31907,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) {
        let assign31360_body63_body1_e31905: f64 = (s.v[719] + 1.0);
        (assign31360_body63_body1_e31905,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31360_body63_body1_e31907;
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && (!s.b[1716])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
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
                s.store_add_scaled_product_right_ad(1482, 1461, 1.0, 1531, A::sub(A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), s.ad_value(1459)), (-1.0));
                s.store_scalar(1485, 0.0);
                s.store_scalar(1486, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
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
            let (assign31360_body94_e32413,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1722]) {
        let assign31360_body94_e32411: f64 = (150.0 + 1.0);
        (assign31360_body94_e32411,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31360_body94_e32413;
            let (assign31360_body95_e32424,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (s.v[1462],)
    } else {
        (s.v[1506],)
    }
};
            s.v[1506] = assign31360_body95_e32424;
            let (assign31360_body96_e32435,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (s.v[1461],)
    } else {
        (s.v[1469],)
    }
};
            s.v[1469] = assign31360_body96_e32435;
            let (assign31360_body97_e32448,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        let assign31360_body97_e32446: f64 = (s.v[97] + 1.0);
        (assign31360_body97_e32446,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31360_body97_e32448;
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
            s.copy_ad(1510, 1461);
            s.store_mul(1448, 965, 1532);
            s.store_add_scaled_inputs3_mixed_aii(1461, A::mul3(s.ad_value(1547), s.ad_value(1448), s.ad_value(1448)), 1.0, 1431, 1.0, 1459, -1.0);
            s.store_add_scaled_product_indices(1480, 1461, 1.0, 1544, 1539, 1.0);
            s.copy_ad(1458, 1480);
            s.copy_ad(1463, 1480);
        }

        let (assign31430_e32539,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
        (s.v[1480],)
    } else {
        (s.v[1505],)
    }
};
        s.v[1505] = assign31430_e32539;

        s.b[1723] = (s.v[85] > s.v[1462]);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        let (assign31450_e32555,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1723]) {
        (1.0,)
    } else {
        (s.v[1475],)
    }
};
        s.v[1475] = assign31450_e32555;

        s.b[1724] = (s.v[85] > s.v[1505]);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        let (assign31470_e32574,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1723])) && s.b[1724]) {
        (3.0,)
    } else {
        (s.v[1475],)
    }
};
        s.v[1475] = assign31470_e32574;

        let (assign31480_e32591,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1723])) && (!s.b[1724])) {
        (2.0,)
    } else {
        (s.v[1475],)
    }
};
        s.v[1475] = assign31480_e32591;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) {
            s.copy_ad(1462, 790);
        }

        let (assign31500_e32615,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) {
        (s.v[1462],)
    } else {
        (s.v[1505],)
    }
};
        s.v[1505] = assign31500_e32615;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) {
            s.copy_ad(1463, 1462);
            s.copy_ad(1508, 1462);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) {
            s.copy_ad(1444, 1450);
            s.store_mul(1448, 1444, 1532);
            s.store_add_scaled_inputs3_mixed_aii(1461, A::mul3(s.ad_value(1547), s.ad_value(1448), s.ad_value(1448)), 1.0, 1431, 1.0, 1459, -1.0);
            s.store_add_ad_lhs(1480, A::mul3(s.ad_value(1544), s.ad_value(1444), s.ad_value(1444)), 1461);
            s.copy_ad(1510, 1461);
        }

        s.b[1725] = (s.v[85] > s.v[1462]);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        let (assign31590_e32732,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1725]) {
        (1.0,)
    } else {
        (s.v[1475],)
    }
};
        s.v[1475] = assign31590_e32732;

        let (assign31600_e32747,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1725])) {
        (2.0,)
    } else {
        (s.v[1475],)
    }
};
        s.v[1475] = assign31600_e32747;

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1545, s.ad_value(1463), 1.0, s.ad_value(1431), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1726] = (s.v[335] > 0.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        let (assign31630_e32785,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1726]) {
        let assign31630_e32776: f64 = (-s.v[961]);
        let assign31630_e32778: f64 = (assign31630_e32776 + s.v[1431]);
        let assign31630_e32780: f64 = (s.v[335]).sqrt();
        let assign31630_e32782: f64 = (assign31630_e32780 / s.v[185]);
        let assign31630_e32783: f64 = (assign31630_e32778 - assign31630_e32782);
        (assign31630_e32783,)
    } else {
        (s.v[1451],)
    }
};
        s.v[1451] = assign31630_e32785;

        let (assign31640_e32800,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1726])) {
        let assign31640_e32796: f64 = (-s.v[961]);
        let assign31640_e32798: f64 = (assign31640_e32796 + s.v[1431]);
        (assign31640_e32798,)
    } else {
        (s.v[1451],)
    }
};
        s.v[1451] = assign31640_e32800;

        s.b[1727] = (s.v[85] > s.v[1462]);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1727]) {
            s.copy_ad(1461, 1510);
            s.copy_ad(1480, 790);
            s.store_add_div_lhs(1477, A::ln(A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 790);
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
            s.store_add_scaled_product_indices(1453, 154, 1.0, 1452, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1480, (-1.0));
        }

        let (assign31760_e32955,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
        (s.v[1480],)
    } else {
        (s.v[1467],)
    }
};
        s.v[1467] = assign31760_e32955;

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
            s.store_div_scaled_inputs2_mixed_aii(1477, A::sqrt(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1454), (-4.0))), 0.5, 1453, (-0.5), 1452, 1.0);
        }

        s.b[1731] = (s.v[1477] > (s.v[1463] - s.v[1549]));
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1731]) {
            s.store_sub(1477, 1463, 1549);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
            s.store_sqrt_mul_sub_rhs(1446, 1543, 1480, 1477);
            s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);
        }

        s.b[1732] = ((s.v[1446] + s.v[1444]) > s.v[965]);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        let (assign31830_e33082,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31830_e33082;

        let mut assign31840_loop_guard: usize = 0;
        while {
            let assign31840_cond_e33102: f64 = (150.0 + 1.0);
            let assign31840_cond_e33104: f64 = if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (s.v[97] <= assign31840_cond_e33102)) { 1.0 } else { 0.0 };
            assign31840_cond_e33104 != 0.0
        } {
            assign31840_loop_guard += 1;
            assert!(assign31840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_add_scaled_inputs3_indices(1464, 1446, 1.0, 1444, 1.0, 965, -1.0);
                s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1446), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1444)));
            }
            s.b[1733] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);
            s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1733]) {
                s.store_offset(1480, 1480, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1733])) {
                s.store_sub_div_rhs_indices(1480, 1480, 1464, 1504);
            }
            s.b[1734] = (((s.v[1480] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));
            s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1734]) {
                s.store_offset_sub(1480, 1431, 1459, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_add_scaled_product_value_ad(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1480, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1453, 1.0, 1452, 1454, (-4.0));
            }
            s.b[1735] = (s.v[335] > 0.0);
            s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1735]) {
                s.store_div_scaled_inputs2_mixed_aii(1477, A::sqrt(s.ad_value(335)), 0.5, 1453, (-0.5), 1452, 1.0);
            }
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1735])) {
                s.store_div_scaled_inputs_indices(1477, 1453, (-0.5), 1452, 1.0);
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
            }
            let (assign31840_body16_e33451,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1737]) {
        let assign31840_body16_e33449: f64 = (150.0 + 1.0);
        (assign31840_body16_e33449,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31840_body16_e33451;
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_sqrt_mul_sub_rhs(1446, 1543, 1480, 1477);
                s.store_div_scaled_inputs2_mixed_aia(1461, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), 1.0, 1459, (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);
            }
            s.b[1738] = ((((s.v[1480] - s.v[1467])) as f64).abs() <= 1e-8);
            s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };
            let (assign31840_body21_e33557,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1738]) {
        let assign31840_body21_e33555: f64 = (150.0 + 1.0);
        (assign31840_body21_e33555,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31840_body21_e33557;
            let (assign31840_body22_e33576,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
        (s.v[1480],)
    } else {
        (s.v[1467],)
    }
};
            s.v[1467] = assign31840_body22_e33576;
            let (assign31840_body23_e33597,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
        let assign31840_body23_e33595: f64 = (s.v[97] + 1.0);
        (assign31840_body23_e33595,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31840_body23_e33597;
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && (!s.b[1730])) {
            s.copy_ad(1480, 1479);
            s.copy_ad(1461, 1460);
            s.copy_ad(1477, 1457);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.copy_ad(1478, 1480);
        }

        let (assign31890_e33669,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign31890_e33669;

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.copy_ad(1458, 1477);
            s.copy_ad(1480, 1478);
        }

        let (assign31920_e33696,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (s.v[1458],)
    } else {
        (s.v[1470],)
    }
};
        s.v[1470] = assign31920_e33696;

        let (assign31930_e33705,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (s.v[1480],)
    } else {
        (s.v[1467],)
    }
};
        s.v[1467] = assign31930_e33705;

        let (assign31940_e33714,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign31940_e33714;

    }

    pub(super) fn stamp_transient_block_26(
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
            }
            let (assign31950_body9_e33848,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body9_e33848;
            let (assign31950_body10_e33859,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body10_e33859;
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
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
            let (assign31950_body21_e33990,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && s.b[1741]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body21_e33990;
            s.b[1742] = (2.0 == 2.0);
            s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };
            let (assign31950_body23_e34011,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && s.b[1742]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body23_e34011;
            s.b[1743] = (2.0 == 4.0);
            s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };
            let (assign31950_body25_e34035,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && s.b[1743]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body25_e34035;
            s.b[1744] = (2.0 == 8.0);
            s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };
            let (assign31950_body27_e34062,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && (!s.b[1743])) && s.b[1744]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body27_e34062;
            let (assign31950_body28_e34075,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body28_e34075;
            let mut assign31950_body29_loop_guard: usize = 0;
            while {
                let assign31950_body29_cond_e34089: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body29_cond_e34089 != 0.0
            } {
                assign31950_body29_loop_guard += 1;
                assert!(assign31950_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31950_body29_body1_e34118,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {
        let assign31950_body29_body1_e34116: f64 = (s.v[719] + 1.0);
        (assign31950_body29_body1_e34116,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31950_body29_body1_e34118;
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && (!s.b[1740])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
            }
            let (assign31950_body45_e34333,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body45_e34333;
            let (assign31950_body46_e34344,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body46_e34344;
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
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
            let (assign31950_body57_e34475,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && s.b[1747]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body57_e34475;
            s.b[1748] = (2.0 == 2.0);
            s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };
            let (assign31950_body59_e34496,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body59_e34496;
            s.b[1749] = (2.0 == 4.0);
            s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };
            let (assign31950_body61_e34520,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && s.b[1749]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body61_e34520;
            s.b[1750] = (2.0 == 8.0);
            s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };
            let (assign31950_body63_e34547,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && (!s.b[1749])) && s.b[1750]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body63_e34547;
            let (assign31950_body64_e34560,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body64_e34560;
            let mut assign31950_body65_loop_guard: usize = 0;
            while {
                let assign31950_body65_cond_e34574: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body65_cond_e34574 != 0.0
            } {
                assign31950_body65_loop_guard += 1;
                assert!(assign31950_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31950_body65_body1_e34603,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {
        let assign31950_body65_body1_e34601: f64 = (s.v[719] + 1.0);
        (assign31950_body65_body1_e34601,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31950_body65_body1_e34603;
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && (!s.b[1746])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1444, 965, (-1e-8), 780);
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
                s.store_div_scaled_inputs_product(1516, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1461), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1538), s.ad_value(1458), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1458), s.ad_value(1458), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1458), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), s.ad_value(1539), 1.0, s.ad_value(335), 1.0);
                s.store_div_ad_lhs(1517, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1458), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_div_ad_lhs(1518, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1458), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1458, 1480);
                s.store_exp(336, 335);
            }
            s.b[1751] = (s.v[1458] >= s.v[1480]);
            s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1751]) {
                s.store_mul_scaled_sqrt_ad_rhs(1472, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(1520, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 1472, 1.0);
                s.store_neg(1522, 1520);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1751])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)));
                s.store_mul_sqrt_ad_rhs(1472, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1472, 1.0);
                s.store_mul_add_ad_rhs(1520, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1522, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1752] = ((s.v[1516] > (s.v[1508] - s.v[1515])) && (s.v[1515] >= 0.0));
            s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
                s.store_add_scaled_inputs3_indices(781, 1516, 1.0, 1508, (-1.0), 1515, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1515);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign31950_body102_e35230,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body102_e35230;
            let (assign31950_body103_e35241,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body103_e35241;
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
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
            let (assign31950_body118_e35424,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && s.b[1754]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body118_e35424;
            s.b[1755] = (4.0 == 2.0);
            s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };
            let (assign31950_body120_e35445,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && s.b[1755]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body120_e35445;
            s.b[1756] = (4.0 == 4.0);
            s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };
            let (assign31950_body122_e35469,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && s.b[1756]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body122_e35469;
            s.b[1757] = (4.0 == 8.0);
            s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };
            let (assign31950_body124_e35496,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && (!s.b[1756])) && s.b[1757]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign31950_body124_e35496;
            let (assign31950_body125_e35509,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign31950_body125_e35509;
            let mut assign31950_body126_loop_guard: usize = 0;
            while {
                let assign31950_body126_cond_e35523: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body126_cond_e35523 != 0.0
            } {
                assign31950_body126_loop_guard += 1;
                assert!(assign31950_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {
                    s.store_sqrt(726, 726);
                }
                let (assign31950_body126_body1_e35552,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {
        let assign31950_body126_body1_e35550: f64 = (s.v[719] + 1.0);
        (assign31950_body126_body1_e35550,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign31950_body126_body1_e35552;
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && (!s.b[1753])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1515, 726);
                s.store_div_scaled_product3_indices(334, 1515, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1508, 1.0, 1515, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_mixed_aii(1482, A::add_scaled_product(s.ad_value(1472), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1458)), 1.0), 1.0, 1495, 1.0, 1496, 1.0);
                s.store_sub(1485, 1520, 185);
                s.store_add_scaled_inputs_products_indices(1486, 1522, 1.0, 1524, 1.0, 1526, 1530, 1.0, 1528, 1530, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
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
            let (assign31950_body152_e35951,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1759]) {
        let assign31950_body152_e35949: f64 = (150.0 + 1.0);
        (assign31950_body152_e35949,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31950_body152_e35951;
            let (assign31950_body153_e35962,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1759]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign31950_body153_e35962;
            let (assign31950_body154_e35971,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (s.v[1458],)
    } else {
        (s.v[1470],)
    }
};
            s.v[1470] = assign31950_body154_e35971;
            let (assign31950_body155_e35980,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        (s.v[1480],)
    } else {
        (s.v[1467],)
    }
};
            s.v[1467] = assign31950_body155_e35980;
            let (assign31950_body156_e35991,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
        let assign31950_body156_e35989: f64 = (s.v[97] + 1.0);
        (assign31950_body156_e35989,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign31950_body156_e35991;
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        s.b[1761] = ((s.v[1450] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        s.b[1762] = ((s.v[1480] > (s.v[1458] - 0.02)) && (0.02 >= 0.0));
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
            s.store_offset_sub(781, 1480, 1458, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign32040_e36096,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32040_e36096;

        let (assign32050_e36109,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32050_e36109;

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
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

        let (assign32160_e36258,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && s.b[1764]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32160_e36258;

        s.b[1765] = (2.0 == 2.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        let (assign32180_e36281,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && s.b[1765]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32180_e36281;

        s.b[1766] = (2.0 == 4.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        let (assign32200_e36307,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && s.b[1766]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32200_e36307;

        s.b[1767] = (2.0 == 8.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        let (assign32220_e36336,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && (!s.b[1766])) && s.b[1767]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32220_e36336;

        let (assign32230_e36351,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32230_e36351;

        let mut assign32240_loop_guard: usize = 0;
        while {
            let assign32240_cond_e36367: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32240_cond_e36367 != 0.0
        } {
            assign32240_loop_guard += 1;
            assert!(assign32240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {
                s.store_sqrt(726, 726);
            }
            let (assign32240_body1_e36400,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {
        let assign32240_body1_e36398: f64 = (s.v[719] + 1.0);
        (assign32240_body1_e36398,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign32240_body1_e36400;
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && (!s.b[1763])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1480, 1458, (-0.02), 780);
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
            s.store_mul_scaled_sqrt_ad_rhs(1472, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
            s.copy_ad(1535, 1472);
            s.store_scalar(1514, 0.0);
            s.store_scalar(1474, 0.0);
            s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);
        }

        s.b[1769] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
            s.store_offset_sub(781, 1444, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign32480_e36744,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32480_e36744;

        let (assign32490_e36757,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32490_e36757;

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
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

        let (assign32600_e36906,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && s.b[1771]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32600_e36906;

        s.b[1772] = (2.0 == 2.0);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        let (assign32620_e36929,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && s.b[1772]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32620_e36929;

        s.b[1773] = (2.0 == 4.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        let (assign32640_e36955,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && s.b[1773]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32640_e36955;

        s.b[1774] = (2.0 == 8.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        let (assign32660_e36984,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && (!s.b[1773])) && s.b[1774]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32660_e36984;

        let (assign32670_e36999,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32670_e36999;

        let mut assign32680_loop_guard: usize = 0;
        while {
            let assign32680_cond_e37015: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32680_cond_e37015 != 0.0
        } {
            assign32680_loop_guard += 1;
            assert!(assign32680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {
                s.store_sqrt(726, 726);
            }
            let (assign32680_body1_e37048,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {
        let assign32680_body1_e37046: f64 = (s.v[719] + 1.0);
        (assign32680_body1_e37046,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign32680_body1_e37048;
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && (!s.b[1770])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1444, 965, (-1e-8), 780);
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
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)));
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
            s.store_add_scaled_product_right_ad(1474, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
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
        }

        let (assign32970_e37536,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign32970_e37536;

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        let (assign32980_e37550,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign32980_e37550;

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
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

        let (assign33090_e37708,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && s.b[1778]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33090_e37708;

        s.b[1779] = (2.0 == 2.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        let (assign33110_e37732,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && s.b[1779]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33110_e37732;

        s.b[1780] = (2.0 == 4.0);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        let (assign33130_e37759,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && s.b[1780]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33130_e37759;

        s.b[1781] = (2.0 == 8.0);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        let (assign33150_e37789,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && (!s.b[1780])) && s.b[1781]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33150_e37789;

        let (assign33160_e37805,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33160_e37805;

        let mut assign33170_loop_guard: usize = 0;
        while {
            let assign33170_cond_e37822: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33170_cond_e37822 != 0.0
        } {
            assign33170_loop_guard += 1;
            assert!(assign33170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {
                s.store_sqrt(726, 726);
            }
            let (assign33170_body1_e37857,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {
        let assign33170_body1_e37855: f64 = (s.v[719] + 1.0);
        (assign33170_body1_e37855,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33170_body1_e37857;
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && (!s.b[1777])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
        }

        let (assign33330_e38117,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33330_e38117;

        let (assign33340_e38131,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33340_e38131;

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
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

        let (assign33450_e38289,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && s.b[1784]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33450_e38289;

        s.b[1785] = (2.0 == 2.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        let (assign33470_e38313,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && s.b[1785]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33470_e38313;

        s.b[1786] = (2.0 == 4.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        let (assign33490_e38340,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && s.b[1786]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33490_e38340;

        s.b[1787] = (2.0 == 8.0);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        let (assign33510_e38370,) = {
    if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && (!s.b[1786])) && s.b[1787]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33510_e38370;

        let (assign33520_e38386,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33520_e38386;

        let mut assign33530_loop_guard: usize = 0;
        while {
            let assign33530_cond_e38403: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33530_cond_e38403 != 0.0
        } {
            assign33530_loop_guard += 1;
            assert!(assign33530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {
                s.store_sqrt(726, 726);
            }
            let (assign33530_body1_e38438,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {
        let assign33530_body1_e38436: f64 = (s.v[719] + 1.0);
        (assign33530_body1_e38436,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33530_body1_e38438;
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && (!s.b[1783])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1444, 965, (-1e-8), 780);
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
        }

        let (assign33710_e38717,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33710_e38717;

        let (assign33720_e38728,) = {
    if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33720_e38728;

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
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

        let (assign33830_e38859,) = {
    if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && s.b[1790]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33830_e38859;

        s.b[1791] = (2.0 == 2.0);
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        let (assign33850_e38880,) = {
    if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && s.b[1791]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33850_e38880;

        s.b[1792] = (2.0 == 4.0);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        let (assign33870_e38904,) = {
    if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1792]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33870_e38904;

        s.b[1793] = (2.0 == 8.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        let (assign33890_e38931,) = {
    if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1792])) && s.b[1793]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign33890_e38931;

        let (assign33900_e38944,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign33900_e38944;

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign33910_loop_guard: usize = 0;
        while {
            let assign33910_cond_e38958: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33910_cond_e38958 != 0.0
        } {
            assign33910_loop_guard += 1;
            assert!(assign33910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {
                s.store_sqrt(726, 726);
            }
            let (assign33910_body1_e38987,) = {
    if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {
        let assign33910_body1_e38985: f64 = (s.v[719] + 1.0);
        (assign33910_body1_e38985,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign33910_body1_e38987;
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && (!s.b[1789])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1788])) {
            s.store_sub(336, 1458, 1508);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1512, 209, -1.0, 338);
        }

        if (s.b[1439] && s.b[1440]) {
            s.copy_ad(87, 1457);
            s.copy_ad(91, 1458);
            s.store_sub(94, 1458, 1457);
            s.store_neg_add(335, 1471, 1472);
        }

        s.b[1794] = ((s.v[335] < s.v[1536]) && (s.v[1536] >= 0.0));
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
            s.store_sub(781, 1536, 335);
            s.store_square(722, 781);
            s.store_square(723, 1536);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign34120_e39238,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34120_e39238;

        let (assign34130_e39246,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34130_e39246;

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
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

        let (assign34240_e39350,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34240_e39350;

        s.b[1797] = (2.0 == 2.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        let (assign34260_e39368,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && s.b[1797]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34260_e39368;

        s.b[1798] = (2.0 == 4.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        let (assign34280_e39389,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && s.b[1798]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34280_e39389;

        s.b[1799] = (2.0 == 8.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        let (assign34300_e39413,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && (!s.b[1798])) && s.b[1799]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34300_e39413;

        let (assign34310_e39423,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34310_e39423;

        let mut assign34320_loop_guard: usize = 0;
        while {
            let assign34320_cond_e39434: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign34320_cond_e39434 != 0.0
        } {
            assign34320_loop_guard += 1;
            assert!(assign34320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {
                s.store_sqrt(726, 726);
            }
            let (assign34320_body1_e39457,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {
        let assign34320_body1_e39455: f64 = (s.v[719] + 1.0);
        (assign34320_body1_e39455,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign34320_body1_e39457;
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1794]) && (!s.b[1795])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1536, 726);
            s.store_div_scaled_product3_indices(334, 1536, 725, 726, 1.0, 770, 1.0);
            s.store_sub(1552, 1536, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1794])) {
            s.copy_ad(1552, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul3_affine_lhs(1499, 154, 1552, 1.0 / (2.0), 0.0, 94);
            s.store_sub(1500, 1512, 1511);
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1439] && s.b[1440]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
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
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
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
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
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
            s.store_div_square_rhs(336, 1498, 185);
            s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1434, -1.0);
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
        }

        let (assign34930_e40127,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign34930_e40127;

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign34940_e40137,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign34940_e40137;

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
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

        let (assign35050_e40259,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && s.b[1807]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35050_e40259;

        s.b[1808] = (2.0 == 2.0);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        let (assign35070_e40279,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && s.b[1808]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35070_e40279;

        s.b[1809] = (2.0 == 4.0);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        let (assign35090_e40302,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && s.b[1809]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35090_e40302;

        s.b[1810] = (2.0 == 8.0);
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        let (assign35110_e40328,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && (!s.b[1809])) && s.b[1810]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35110_e40328;

        let (assign35120_e40340,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35120_e40340;

        let mut assign35130_loop_guard: usize = 0;
        while {
            let assign35130_cond_e40353: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35130_cond_e40353 != 0.0
        } {
            assign35130_loop_guard += 1;
            assert!(assign35130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {
                s.store_sqrt(726, 726);
            }
            let (assign35130_body1_e40380,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {
        let assign35130_body1_e40378: f64 = (s.v[719] + 1.0);
        (assign35130_body1_e40378,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign35130_body1_e40380;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && (!s.b[1806])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);
            s.store_mul(344, 344, 975);
        }

        s.b[1811] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
            s.store_sub_offset_lhs(781, 972, 4.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (4.0 * 4.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign35330_e40622,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35330_e40622;

        let (assign35340_e40632,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35340_e40632;

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
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

        let (assign35490_e40802,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && s.b[1813]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35490_e40802;

        s.b[1814] = (4.0 == 2.0);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        let (assign35510_e40822,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && s.b[1814]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35510_e40822;

        s.b[1815] = (4.0 == 4.0);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        let (assign35530_e40845,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && s.b[1815]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35530_e40845;

        s.b[1816] = (4.0 == 8.0);
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        let (assign35550_e40871,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && (!s.b[1815])) && s.b[1816]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign35550_e40871;

        let (assign35560_e40883,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign35560_e40883;

        let mut assign35570_loop_guard: usize = 0;
        while {
            let assign35570_cond_e40896: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35570_cond_e40896 != 0.0
        } {
            assign35570_loop_guard += 1;
            assert!(assign35570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {
                s.store_sqrt(726, 726);
            }
            let (assign35570_body1_e40923,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {
        let assign35570_body1_e40921: f64 = (s.v[719] + 1.0);
        (assign35570_body1_e40921,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign35570_body1_e40923;
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && (!s.b[1812])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);
            s.store_sub_offset_lhs(344, 972, 4.0, 780);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1551, 1553, 170);
            s.store_div_scaled_product_indices(335, 254, 1551, 1.0, 973, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset(337, 336, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && s.b[1440]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1551, 1553, 170);
            s.store_div_scaled_product_indices(335, 254, 1551, 1.0, 973, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_div(1503, 254, 338);
            s.store_mul3_affine_lhs(1550, 1501, 1503, (-s.v[632]), 0.0, 1551);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1550, 1.0);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1817] = (p.p283 != 0.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
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
            s.store_add_scaled_inputs3_indices(339, 1457, 1.0, 340, 1.0, 1434, -1.0);
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
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1822])) {
            s.copy_ad(341, 647);
        }

        s.b[1823] = (s.v[793] >= 0.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1823]) {
            s.copy_ad(369, 793);
        }

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
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1821]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
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
            s.store_add_scaled_inputs4_indices(131, 1473, (-0.5), 1474, (-0.5), 1494, (-0.5), 1496, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(1513), 1.0, s.ad_value(1514), 1.0), s.ad_value(1493)), 1495, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1534, 1535, (-0.5));
            s.store_neg(238, 1534);
            s.copy_ad(255, 1554);
        }

        s.b[1825] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        let (assign36940_e42520,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1825]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign36940_e42520;

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
            s.store_div_square_rhs(1902, 1880, 185);
            s.store_div_from_scalar(1903, 2.0, 1902);
            s.store_scalar(1906, 4.0);
            s.store_scalar(1907, 0.1);
            s.store_scalar(1908, 0.1);
            s.store_offset(1909, 961, p.p407);
        }

    }
}
