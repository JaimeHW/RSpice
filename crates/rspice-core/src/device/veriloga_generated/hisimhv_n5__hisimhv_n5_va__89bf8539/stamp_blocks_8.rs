#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1535, (-1.0), 335, 1.0);
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(790, 1535, 1.0, 335, (-1.0), 780, 1.0);
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
            s.store_sqrt_mul_ad(1452, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
        }

        s.b[1710] = (s.v[1452] > s.v[965]);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
            s.copy_ad(1464, 790);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1446, 965, (-1e-8), 780);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
                s.store_add_scaled_inputs3_indices(335, 1463, 1.0, 1433, (-1.0), 1461, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
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
                s.store_add_scaled_product_right_ad(1484, 1463, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461)), (-1.0));
                s.store_scalar(1487, 0.0);
                s.store_scalar(1488, 1.0);
                s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));
                s.store_div(1490, 1488, 1489);
                s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);
                s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);
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
            s.store_add_scaled_inputs3_mixed_aii(1463, A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), 1.0, 1433, 1.0, 1461, -1.0);
            s.store_add_scaled_product_indices(1482, 1463, 1.0, 1546, 1541, 1.0);
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
            s.store_add_scaled_inputs3_mixed_aii(1463, A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), 1.0, 1433, 1.0, 1461, -1.0);
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
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1547, s.ad_value(1465), 1.0, s.ad_value(1433), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1728] = (s.v[335] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1728]) {
            s.store_add_scaled_inputs3_mixed_iia(1453, 1433, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
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
            s.store_add_scaled_product_indices(1455, 154, 1.0, 1454, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1482, (-1.0));
            s.copy_ad(1469, 1482);
            s.store_div_scaled_inputs2_mixed_aii(1479, A::sqrt(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, 1455, (-0.5), 1454, 1.0);
        }

        s.b[1733] = (s.v[1479] > (s.v[1465] - s.v[1551]));
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
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
                s.store_add_scaled_inputs3_indices(1466, 1448, 1.0, 1446, 1.0, 965, -1.0);
                s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1448), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1446)));
            }
            s.b[1735] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);
            s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1735]) {
                s.store_offset(1482, 1482, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1735])) {
                s.store_sub_div_rhs_indices(1482, 1482, 1466, 1506);
            }
            s.b[1736] = (((s.v[1482] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));
            s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1736]) {
                s.store_offset_sub(1482, 1433, 1461, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
                s.store_add_scaled_product_value_ad(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1482, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1455, 1.0, 1454, 1456, (-4.0));
            }
            s.b[1737] = (s.v[335] > 0.0);
            s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1737]) {
                s.store_div_scaled_inputs2_mixed_aii(1479, A::sqrt(s.ad_value(335)), 0.5, 1455, (-0.5), 1454, 1.0);
            }
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1737])) {
                s.store_div_scaled_inputs_indices(1479, 1455, (-0.5), 1454, 1.0);
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
                s.store_div_scaled_inputs2_mixed_aia(1463, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);
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

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1446, 965, (-1e-8), 780);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
                s.store_mul(1497, 1446, 1544);
                s.store_mul_ad_product_lhs(1526, A::div_from_scalar(1.034943e-10, s.ad_value(1446)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1528, A::div_from_scalar((-1.034943e-10), s.ad_value(1446)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1498, 1450, 1542);
                s.store_div_from_scalar(1530, (-1.034943e-10), 1450);
                s.store_scaled_mul(335, 1500, 1541, 8.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                let assign31970_body81_ad_e34876: A = A::add(A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1463), s.ad_value(1540), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1540), s.ad_value(1460), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1460), s.ad_value(1460), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0), A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1500), s.ad_value(1541), 4.0));
                s.store_div_scaled_add_product(1518, assign31970_body81_ad_e34876, 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), s.ad_value(1541), 1.0, s.ad_value(335), 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_div_ad_lhs(1519, A::add_scaled_products3(s.ad_value(1463), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1460), (4.0 * 2.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_div_ad_lhs(1520, A::add_scaled_products3(s.ad_value(1463), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1460), (-8.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1460, 1482);
                s.store_exp(336, 335);
            }
            s.b[1753] = (s.v[1460] >= s.v[1482]);
            s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1753]) {
                s.store_mul_scaled_sqrt_ad_rhs(1474, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1522, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1474), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1524, 1522);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1753])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)));
                s.store_mul_sqrt_ad_rhs(1474, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1474, 1.0);
                s.store_mul_add_ad_rhs(1522, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1524, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1754] = ((s.v[1518] > (s.v[1510] - s.v[1517])) && (s.v[1517] >= 0.0));
            s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
                s.store_add_scaled_inputs3_indices(781, 1518, 1.0, 1510, (-1.0), 1517, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1517, 726);
                s.store_div_scaled_product3_indices(334, 1517, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1510, 1.0, 1517, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_mixed_aii(1484, A::add_scaled_product(s.ad_value(1474), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1460)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);
                s.store_sub(1487, 1522, 185);
                s.store_add_scaled_inputs_products_indices(1488, 1524, 1.0, 1526, 1.0, 1528, 1532, 1.0, 1530, 1532, 1.0);
                s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));
                s.store_div(1490, 1488, 1489);
                s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);
                s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);
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

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1482, 1460, (-0.02), 780);
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
            s.store_mul_scaled_sqrt_ad_rhs(1474, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1446, 965, (-1e-8), 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
            s.store_mul(1497, 1446, 1544);
            s.store_mul_neg_lhs(1498, 1450, 1542);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)));
            s.store_mul_sqrt_ad_rhs(1474, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1777] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1777]) {
            s.store_scalar(1476, 0.0);
            s.store_scalar(1516, 0.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1777])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1476, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1446, 965, (-1e-8), 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1790])) {
            s.store_sub(336, 1460, 1510);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1514, 209, -1.0, 338);
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1538, 726);
            s.store_div_scaled_product3_indices(334, 1538, 725, 726, 1.0, 770, 1.0);
            s.store_sub(1554, 1538, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1796])) {
            s.copy_ad(1554, 335);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
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
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
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
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
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
            s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1436, -1.0);
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);
            s.store_mul(344, 344, 975);
        }

        s.b[1813] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
            s.store_sub_offset_lhs(781, 972, 4.0, 344);
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

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && (!s.b[1814])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);
            s.store_sub_offset_lhs(344, 972, 4.0, 780);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1553, 1555, 170);
            s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1553, 1555, 170);
            s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_div(1505, 254, 338);
            s.store_mul3_affine_lhs(1552, 1503, 1505, (-s.v[632]), 0.0, 1553);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1552, 1.0);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1819] = (p.p283 != 0.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
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
            s.store_add_scaled_inputs3_indices(339, 1459, 1.0, 340, 1.0, 1436, -1.0);
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
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
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

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
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
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
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
            s.store_add_scaled_inputs4_indices(131, 1475, (-0.5), 1476, (-0.5), 1496, (-0.5), 1498, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(1515), 1.0, s.ad_value(1516), 1.0), s.ad_value(1495)), 1497, (-0.5));
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
            s.store_div_scaled_product_add_scaled_denominator_indices(962, 1906, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);
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
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
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
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(1858, 781, (-0.5), 782, (-0.5), 0.3);
        }

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add_scaled_inputs3_offset_indices(781, 1858, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1858, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(1886, 1853, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);
            s.store_offset_sub(1832, 965, 1833, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1848, 0.2);
            s.copy_ad(1851, 1858);
            s.copy_ad(1854, 1849);
            s.copy_ad(1856, 1886);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1828, 965, (-1e-8), 780);
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
                s.store_add_ad_lhs(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1851);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1935])) {
                s.store_add_ad_rhs(1837, 1851, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1828), A::sub_scaled_inputs(s.ad_value(1828), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));
                s.store_scalar(1838, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1828)), s.ad_value(334), (-1.0)), 1.0, 1840);
            }
            s.b[1936] = ((s.v[1837] > (s.v[1849] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1849, (-1.0), 1848, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1837, 1849, 1.0, 1848, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_indices(335, 1856, 1.0, 1885, (-1.0), 1853, 1.0);
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
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
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
                s.store_mul_scaled_sqrt_ad_rhs(1860, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1895, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1860), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1897, 1895);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1948])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)));
                s.store_mul_sqrt_ad_rhs(1860, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1860, 1.0);
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1860), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1851)), 1.0), 1.0, 1864, 1.0, 1865, 1.0);
                s.store_sub(1869, 1895, 185);
                s.store_add_ad_rhs(1870, 1897, A::add_scaled_value_products(s.ad_value(1842), 1.0, s.ad_value(1844), s.ad_value(1840), 1.0, s.ad_value(1846), s.ad_value(1840), 1.0));
                s.store_sub(1871, 1854, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);
                s.store_abs(335, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
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

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul_sub_rhs(335, 154, 1851, 1854);
            s.store_exp(336, 335);
        }

        s.b[1952] = (s.v[1851] >= s.v[1854]);
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1952]) {
            s.copy_ad(1890, 1860);
            s.store_scalar(1893, 0.0);
            s.store_scalar(1862, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) {
            s.store_scalar(1890, 0.0);
            s.store_mul_sqrt_ad_rhs(1893, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[1953] = (s.v[1834] > s.v[965]);
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && s.b[1953]) {
            s.store_scalar(1862, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && (!s.b[1953])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1862, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[1954] = (((s.v[1851] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1851, -1.0, 1849, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[1955] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        s.b[1956] = (4.0 == 1.0);
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && s.b[1956]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1957] = (4.0 == 2.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && s.b[1957]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1958] = (4.0 == 4.0);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && s.b[1958]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1959] = (4.0 == 8.0);
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && (!s.b[1958])) && s.b[1959]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38530_loop_guard: usize = 0;
        while {
            let assign38530_cond_e47310: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38530_cond_e47310 != 0.0
        } {
            assign38530_loop_guard += 1;
            assert!(assign38530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && (!s.b[1955])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1954])) {
            s.store_sub(336, 1851, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1887, 209, -1.0, 338);
            s.copy_ad(349, 790);
        }

        s.b[1960] = (s.v[790] > 1e-6);
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_scalar(344, 1e-25);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[1961] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
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

        s.b[1962] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        s.b[1963] = (2.0 == 1.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && s.b[1963]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1964] = (2.0 == 2.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && s.b[1964]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1965] = (2.0 == 4.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && s.b[1965]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1966] = (2.0 == 8.0);
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && (!s.b[1965])) && s.b[1966]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38950_loop_guard: usize = 0;
        while {
            let assign38950_cond_e47913: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38950_cond_e47913 != 0.0
        } {
            assign38950_loop_guard += 1;
            assert!(assign38950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && (!s.b[1962])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1904), 1.0, s.ad_value(337)));
        }

        s.b[1967] = ((s.v[344] < 1.0) && (1.0 >= 0.0));
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_sub_from_scalar(781, 1.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
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

        s.b[1968] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        s.b[1969] = (2.0 == 1.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && s.b[1969]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1970] = (2.0 == 2.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && s.b[1970]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1971] = (2.0 == 4.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && s.b[1971]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1972] = (2.0 == 8.0);
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && (!s.b[1971])) && s.b[1972]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign39320_loop_guard: usize = 0;
        while {
            let assign39320_cond_e48477: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39320_cond_e48477 != 0.0
        } {
            assign39320_loop_guard += 1;
            assert!(assign39320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && (!s.b[1968])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(344, 1.0, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_div(335, 790, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1960])) {
            s.copy_ad(348, 790);
        }

        s.b[1973] = (s.v[790] < 0.0);
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1973]) {
            s.copy_ad(1852, 1851);
            s.copy_ad(1857, 1856);
            s.copy_ad(1855, 1854);
            s.copy_ad(1863, 1862);
            s.copy_ad(1891, 1890);
            s.copy_ad(1888, 1887);
            s.copy_ad(1866, 1864);
            s.copy_ad(1867, 1865);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.copy_ad(1835, 1834);
            s.copy_ad(1850, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 1851, 1.0, 1850, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 1851, 1850, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1859, 1851, 1.0, 1850, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 1859, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1859, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1885))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_scalar(79, 0.0);
            s.copy_ad(1852, 1859);
            s.copy_ad(1855, 1850);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign39810_loop_guard: usize = 0;
        while {
            let assign39810_cond_e49257: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign39810_cond_e49257 != 0.0
        } {
            assign39810_loop_guard += 1;
            assert!(assign39810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul_sub_ad_rhs(1857, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1855), 1.0), s.ad_value(1853));
                s.store_mul(1841, 1899, 1900);
                s.store_sub(335, 1855, 1857);
            }
            s.b[1974] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
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
            s.b[1975] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };
            s.b[1976] = (2.0 == 1.0);
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && s.b[1976]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1977] = (2.0 == 2.0);
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && s.b[1977]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1978] = (2.0 == 4.0);
            s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && s.b[1978]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1979] = (2.0 == 8.0);
            s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && (!s.b[1978])) && s.b[1979]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body29_loop_guard: usize = 0;
            while {
                let assign39810_body29_cond_e49692: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body29_cond_e49692 != 0.0
            } {
                assign39810_body29_loop_guard += 1;
                assert!(assign39810_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && (!s.b[1975])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1974])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1829, 1906, 336);
            }
            s.b[1980] = ((s.v[1829] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_offset_sub(781, 1829, 965, 1e-8);
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
            s.b[1981] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            s.b[1982] = (2.0 == 1.0);
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && s.b[1982]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1983] = (2.0 == 2.0);
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && s.b[1983]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1984] = (2.0 == 4.0);
            s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && s.b[1984]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1985] = (2.0 == 8.0);
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && (!s.b[1984])) && s.b[1985]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body65_loop_guard: usize = 0;
            while {
                let assign39810_body65_cond_e50273: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body65_cond_e50273 != 0.0
            } {
                assign39810_body65_loop_guard += 1;
                assert!(assign39810_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && (!s.b[1981])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1829, 965, (-1e-8), 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1866, 1829, 1903);
                s.store_mul_ad_product_lhs(1843, A::div_from_scalar(1.034943e-10, s.ad_value(1829)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1845, A::div_from_scalar((-1.034943e-10), s.ad_value(1829)), s.ad_value(334), 337);
            }
            s.b[1986] = (p.p49 == 0.0);
            s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1986]) {
                s.store_add_ad_lhs(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1852);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1986])) {
                s.store_add_ad_rhs(1837, 1852, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1829), A::sub_scaled_inputs(s.ad_value(1829), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));
                s.store_scalar(1838, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1829)), s.ad_value(334), (-1.0)), 1.0, 1841);
            }
            s.b[1987] = ((s.v[1837] > (s.v[1850] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1850, (-1.0), 1848, 1.0);
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
            s.b[1988] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            s.b[1989] = (4.0 == 1.0);
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && s.b[1989]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1990] = (4.0 == 2.0);
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && s.b[1990]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1991] = (4.0 == 4.0);
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && s.b[1991]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1992] = (4.0 == 8.0);
            s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && (!s.b[1991])) && s.b[1992]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body114_loop_guard: usize = 0;
            while {
                let assign39810_body114_cond_e51078: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body114_cond_e51078 != 0.0
            } {
                assign39810_body114_loop_guard += 1;
                assert!(assign39810_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && (!s.b[1988])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1837, 1850, 1.0, 1848, (-1.0), 780, 1.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_add_scaled_inputs3_indices(335, 1857, 1.0, 1885, (-1.0), 1853, 1.0);
            }
            s.b[1993] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
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
            s.b[1994] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            s.b[1995] = (2.0 == 1.0);
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && s.b[1995]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1996] = (2.0 == 2.0);
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && s.b[1996]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1997] = (2.0 == 4.0);
            s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && s.b[1997]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1998] = (2.0 == 8.0);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && (!s.b[1997])) && s.b[1998]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body152_loop_guard: usize = 0;
            while {
                let assign39810_body152_cond_e51688: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body152_cond_e51688 != 0.0
            } {
                assign39810_body152_loop_guard += 1;
                assert!(assign39810_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && (!s.b[1994])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1993])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1831, 1907, 336);
                s.store_mul_neg_lhs(1867, 1831, 1902);
                s.store_mul_div_from_scalar_lhs(1847, (-1.034943e-10), 1831, 337);
                s.store_mul_sub_rhs(335, 154, 1852, 1855);
                s.store_exp(336, 335);
            }
            s.b[1999] = (s.v[1852] >= s.v[1855]);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1999]) {
                s.store_mul_scaled_sqrt_ad_rhs(1861, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1896, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1861), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1898, 1896);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1999])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)));
                s.store_mul_sqrt_ad_rhs(1861, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1861, 1.0);
                s.store_mul_add_ad_rhs(1896, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1898, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1861), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1852)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);
                s.store_sub(1869, 1896, 185);
                s.store_add_ad_rhs(1870, 1898, A::add_scaled_value_products(s.ad_value(1843), 1.0, s.ad_value(1845), s.ad_value(1841), 1.0, s.ad_value(1847), s.ad_value(1841), 1.0));
                s.store_sub(1871, 1855, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);
                s.store_abs(335, 1880);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[2000] = (s.v[335] > 0.1);
            s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2000]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[2001] = (s.v[335] < 1e-12);
            s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2001]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_add(1852, 1852, 1880);
                s.store_add(1855, 1855, 1881);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_mul_sub_rhs(335, 154, 1852, 1855);
            s.store_exp(336, 335);
        }

        s.b[2003] = (s.v[1852] >= s.v[1855]);
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2003]) {
            s.copy_ad(1891, 1861);
            s.store_scalar(1894, 0.0);
            s.store_scalar(1863, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) {
            s.store_scalar(1891, 0.0);
            s.store_mul_sqrt_ad_rhs(1894, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[2004] = (s.v[1835] > s.v[965]);
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && s.b[2004]) {
            s.store_scalar(1863, 0.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && (!s.b[2004])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1863, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[2005] = (((s.v[1852] - s.v[1850]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1852, -1.0, 1850, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[2006] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        s.b[2007] = (4.0 == 1.0);
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && s.b[2007]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2008] = (4.0 == 2.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && s.b[2008]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2009] = (4.0 == 4.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && s.b[2009]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2010] = (4.0 == 8.0);
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && (!s.b[2009])) && s.b[2010]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40250_loop_guard: usize = 0;
        while {
            let assign40250_cond_e53259: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40250_cond_e53259 != 0.0
        } {
            assign40250_loop_guard += 1;
            assert!(assign40250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && (!s.b[2006])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2005])) {
            s.store_sub(336, 1852, 1850);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1888, 209, -1.0, 338);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.copy_ad(87, 1851);
            s.copy_ad(91, 1852);
            s.store_sub(94, 1852, 1851);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2011] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
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

        s.b[2012] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        s.b[2013] = (2.0 == 1.0);
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && s.b[2013]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2014] = (2.0 == 2.0);
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && s.b[2014]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2015] = (2.0 == 4.0);
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && s.b[2015]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2016] = (2.0 == 8.0);
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && (!s.b[2015])) && s.b[2016]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40710_loop_guard: usize = 0;
        while {
            let assign40710_cond_e53989: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40710_cond_e53989 != 0.0
        } {
            assign40710_loop_guard += 1;
            assert!(assign40710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && (!s.b[2012])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add(109, 87, 110);
        }

        s.b[2017] = (((s.v[109] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 109, -1.0, 1849, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[2018] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        s.b[2019] = (4.0 == 1.0);
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && s.b[2019]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2020] = (4.0 == 2.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && s.b[2020]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2021] = (4.0 == 4.0);
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && s.b[2021]) {
            s.store_scalar(720, 3.0);
        }

    }

    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2022] = (4.0 == 8.0);
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && (!s.b[2021])) && s.b[2022]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41110_loop_guard: usize = 0;
        while {
            let assign41110_cond_e54537: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41110_cond_e54537 != 0.0
        } {
            assign41110_loop_guard += 1;
            assert!(assign41110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && (!s.b[2018])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2017])) {
            s.store_sub(336, 109, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1889, 209, -1.0, 338);
        }

        s.b[2028] = (s.v[1834] > s.v[965]);
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2028]) {
            s.copy_ad(981, 1832);
        }

        s.b[2029] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_offset(781, 87, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign41350_loop_guard: usize = 0;
        while {
            let assign41350_cond_e54907: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41350_cond_e54907 != 0.0
        } {
            assign41350_loop_guard += 1;
            assert!(assign41350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2030] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        s.b[2031] = (s.v[1912] == 1.0);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && s.b[2031]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2032] = (s.v[1912] == 2.0);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && s.b[2032]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2033] = (s.v[1912] == 4.0);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && s.b[2033]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2034] = (s.v[1912] == 8.0);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && (!s.b[2033])) && s.b[2034]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41480_loop_guard: usize = 0;
        while {
            let assign41480_cond_e55135: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41480_cond_e55135 != 0.0
        } {
            assign41480_loop_guard += 1;
            assert!(assign41480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (!s.b[2030])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_offset(983, 780, (-0.1));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2029])) {
            s.copy_ad(983, 87);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_add_scaled_inputs3_offset_indices(1914, 791, 1.0, 85, (-1.0), 1910, 1.0, (-(s.v[462] - p.p392)));
            s.store_sub(1913, 791, 1914);
        }

        s.b[2035] = ((s.v[1913] > (-s.v[1911])) && (s.v[1911] >= 0.0));
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(781, 1913, 1911);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign41700_loop_guard: usize = 0;
        while {
            let assign41700_cond_e55520: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41700_cond_e55520 != 0.0
        } {
            assign41700_loop_guard += 1;
            assert!(assign41700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2036] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        s.b[2037] = (s.v[1912] == 1.0);
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && s.b[2037]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2038] = (s.v[1912] == 2.0);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && s.b[2038]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2039] = (s.v[1912] == 4.0);
        s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && s.b[2039]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2040] = (s.v[1912] == 8.0);
        s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && (!s.b[2039])) && s.b[2040]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41830_loop_guard: usize = 0;
        while {
            let assign41830_cond_e55748: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41830_cond_e55748 != 0.0
        } {
            assign41830_loop_guard += 1;
            assert!(assign41830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (!s.b[2036])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(1913, 1911, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign41940_loop_guard: usize = 0;
        while {
            let assign41940_cond_e55964: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign41940_cond_e55964 != 0.0
        } {
            assign41940_loop_guard += 1;
            assert!(assign41940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2041] = (s.v[983] >= 0.0);
            s.v[2041] = if s.b[2041] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2041]) {
                s.store_mul_scaled_sqrt_ad_rhs(2026, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2027, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2026), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2041])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1885);
                s.store_mul_sqrt_ad_rhs(2026, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2026, 1.0);
                s.store_mul_add_ad_rhs(2027, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(1868, 2026, 1.0, 185, A::sub(s.ad_value(1913), s.ad_value(983)), 1.0);
                s.store_sub(1869, 2027, 185);
                s.store_div_scaled_inputs_indices(1880, 1868, -1.0, 1869, 1.0);
            }
            s.b[2042] = (((s.v[1880]) as f64).abs() < (1e-10 * 100.0));
            s.v[2042] = if s.b[2042] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && s.b[2042]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2043] = (s.v[1880] > 0.1);
            s.v[2043] = if s.b[2043] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && s.b[2043]) {
                s.store_scalar(1880, 0.1);
            }
            s.b[2044] = (s.v[1880] < (-0.1));
            s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && (!s.b[2043])) && s.b[2044]) {
                s.store_scalar(1880, (-0.1));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2024, 1903, 1832, (0.5 * 9662367879.197212), 0.0, 1832);
            s.store_scaled_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2024)), p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2025, A::ln(s.ad_value(335)), 2024);
            s.store_mul(332, 2025, 983);
            s.store_exp_mul_scaled_lhs_indices(334, 2025, -1.0, 2024);
        }

        s.b[2046] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2046] = if s.b[2046] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {
            s.store_mul_exp_lhs(335, 332, 334);
        }

    }
}
