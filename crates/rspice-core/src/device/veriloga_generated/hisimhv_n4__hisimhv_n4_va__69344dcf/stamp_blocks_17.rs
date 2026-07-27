#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1732] = ((s.v[1446] + s.v[1444]) > s.v[965]);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {s.store_scalar(97, 1.0);}
        let mut t2: usize = 0;
        while {
            let t0: f64 = (150.0 + 1.0);let t1: f64 = if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (s.v[97] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;
            if t2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {s.store_add_scaled_inputs3_indices(1464, 1446, 1.0, 1444, 1.0, 965, -1.0);s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1446), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1444)));}
            s.b[1733] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1733]) {s.store_offset(1480, 1480, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1733])) {s.store_sub_div_rhs_indices(1480, 1480, 1464, 1504);}
            s.b[1734] = (((s.v[1480] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1734]) {s.store_offset_sub(1480, 1431, 1459, (10.0 * 2.220446049250313e-16));}
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {s.store_add_scaled_product_mixed_aii(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1480, (-1.0));s.store_add_scaled_square_product_indices(335, 1453, 1.0, 1452, 1454, (-4.0));}
            s.b[1735] = (s.v[335] > 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1735]) {s.store_div_scaled_inputs2_sqrt_first(1477, 335, 0.5, 1453, (-0.5), 1452, 1.0);}
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1735])) {s.store_div_scaled_inputs_indices(1477, 1453, (-0.5), 1452, 1.0);}
            s.b[1736] = (s.v[1477] > s.v[1463]);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1736]) {s.copy_ad(1477, 1463);}
            s.b[1737] = (s.v[1477] > s.v[1480]);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1737]) {s.store_sub(1477, 1480, 1549);s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {s.store_sqrt_mul_sub_rhs(1446, 1543, 1480, 1477);s.store_div_scaled_inputs2_mixed_aia(1461, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), 1.0, 1459, (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);}
            s.b[1738] = ((((s.v[1480] - s.v[1467])) as f64).abs() <= 1e-8);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1738]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {s.copy_ad(1467, 1480);s.store_primal_offset(97, 97, 1.0);}
        }
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && (!s.b[1730])) {s.copy_ad(1480, 1479);s.copy_ad(1461, 1460);s.copy_ad(1477, 1457);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.copy_ad(1478, 1480);s.store_scalar(79, 0.0);s.copy_ad(1458, 1477);s.copy_ad(1480, 1478);s.copy_ad(1470, 1458);s.copy_ad(1467, 1480);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (150.0 + 1.0);let ta: f64 = if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (s.v[97] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_mul_sub_mixed_iai(1461, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), 1459);s.store_mul(1530, 1531, 1532);s.store_sub(335, 1480, 1461);}
            s.b[1739] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1740] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });s.b[1741] = (2.0 == 1.0);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && s.b[1741]) {s.store_scalar(720, 1.0);}
            s.b[1742] = (2.0 == 2.0);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && s.b[1742]) {s.store_scalar(720, 2.0);}
            s.b[1743] = (2.0 == 4.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && s.b[1743]) {s.store_scalar(720, 3.0);}
            s.b[1744] = (2.0 == 8.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && (!s.b[1743])) && s.b[1744]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {s.store_scalar(719, 0.0);}
            let mut t6: usize = 0;
            while {
                let t5: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t5 != 0.0
            } {
                t6 += 1;
                if t6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && (!s.b[1740])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1739])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_sqrt_mul(1444, 1543, 336);}
            s.b[1745] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {s.store_offset_sub(781, 1444, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1746] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });s.b[1747] = (2.0 == 1.0);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && s.b[1747]) {s.store_scalar(720, 1.0);}
            s.b[1748] = (2.0 == 2.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {s.store_scalar(720, 2.0);}
            s.b[1749] = (2.0 == 4.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && s.b[1749]) {s.store_scalar(720, 3.0);}
            s.b[1750] = (2.0 == 8.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && (!s.b[1749])) && s.b[1750]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {s.store_scalar(719, 0.0);}
            let mut t8: usize = 0;
            while {
                let t7: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t7 != 0.0
            } {
                t8 += 1;
                if t8 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t8, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && (!s.b[1746])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1444, 965, (-1e-8), 780);}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1745])) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1745])) {s.store_scalar(337, 1.0);}
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));s.store_mul(1495, 1444, 1542);s.store_mul_ad_product_lhs_mixed_ai(1524, A::div_from_scalar(1.034943e-10, s.ad_value(1444)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1526, A::div_from_scalar((-1.034943e-10), s.ad_value(1444)), 334, 337);s.store_mul_scale_offset_indices(1496, 1540, 1448, -1.0, 0.0);s.store_div_from_scalar(1528, (-1.034943e-10), 1448);s.store_scaled_mul(335, 1498, 1539, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1516, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1461), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1538), s.ad_value(1458), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1458), s.ad_value(1458), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1458), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), 1539, 1.0, 335, 1.0);s.store_div_mixed_ai(1517, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1458), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);s.store_div_mixed_ai(1518, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1458), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1458, 1480);s.store_exp(336, 335);}
            s.b[1751] = (s.v[1458] >= s.v[1480]);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1751]) {s.store_mul_scaled_sqrt_ad_rhs(1472, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1520, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1472, 1.0);s.store_neg(1522, 1520);}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1751])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)));s.store_mul_sqrt_mixed_ia(1472, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1472, 1.0);s.store_mul_add_mixed_iaa(1520, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1522, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1752] = ((s.v[1516] > (s.v[1508] - s.v[1515])) && (s.v[1515] >= 0.0));s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {s.store_add_scaled_inputs3_indices(781, 1516, 1.0, 1508, (-1.0), 1515, 1.0);s.store_square(722, 781);s.store_square(723, 1515);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1753] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });s.b[1754] = (4.0 == 1.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && s.b[1754]) {s.store_scalar(720, 1.0);}
            s.b[1755] = (4.0 == 2.0);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && s.b[1755]) {s.store_scalar(720, 2.0);}
            s.b[1756] = (4.0 == 4.0);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && s.b[1756]) {s.store_scalar(720, 3.0);}
            s.b[1757] = (4.0 == 8.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && (!s.b[1756])) && s.b[1757]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {s.store_scalar(719, 0.0);}
            let mut t4: usize = 0;
            while {
                let t3: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t3 != 0.0
            } {
                t4 += 1;
                if t4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && (!s.b[1753])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1515, 726);s.store_div_scaled_product3_indices(334, 1515, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1508, 1.0, 1515, (-1.0), 780, 1.0);}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1752])) {s.copy_ad(335, 1516);s.store_scalar(334, 1.0);}
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_sub(1481, 1480, 335);s.store_mul_scale_offset_indices(1483, 334, 1517, -1.0, 0.0);s.store_sub_from_scalar_ad(1484, 1.0, A::mul3(s.ad_value(1518), s.ad_value(1530), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1482, A::add_scaled_product(s.ad_value(1472), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1458)), 1.0), 1.0, 1495, 1.0, 1496, 1.0);s.store_sub(1485, 1520, 185);s.store_add_scaled_inputs_products_indices(1486, 1522, 1.0, 1524, 1.0, 1526, 1530, 1.0, 1528, 1530, 1.0);s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));s.store_div(1488, 1486, 1487);s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);s.store_div(1491, 1483, 1487);}
            s.b[1758] = (((((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482]))) as f64).abs() > 0.5);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1758]) {s.store_offset(1458, 1458, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1758]) {s.store_offset(1480, 1480, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1758])) {s.store_sub_mixed_ia(1458, 1458, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));s.store_sub_mixed_ia(1480, 1480, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));}
            s.b[1759] = (((((s.v[1458] - s.v[1470])) as f64).abs() <= 1e-12) && ((((s.v[1480] - s.v[1467])) as f64).abs() <= 1e-12));s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1759]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.copy_ad(1470, 1458);s.copy_ad(1467, 1480);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1761] = ((s.v[1450] > s.v[965]) && (s.v[1475] != 2.0));s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });s.b[1762] = ((s.v[1480] > (s.v[1458] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {s.store_offset_sub(781, 1480, 1458, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1763] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = (2.0 == 1.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && s.b[1764]) {s.store_scalar(720, 1.0);}
        s.b[1765] = (2.0 == 2.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && s.b[1765]) {s.store_scalar(720, 2.0);}
        s.b[1766] = (2.0 == 4.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && s.b[1766]) {s.store_scalar(720, 3.0);}
        s.b[1767] = (2.0 == 8.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && (!s.b[1766])) && s.b[1767]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && (!s.b[1763])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1480, 1458, (-0.02), 780);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && (!s.b[1762])) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && (!s.b[1762])) {s.store_scalar(335, 1.0);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_mul_sub_mixed_iai(1461, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), 1459);s.store_mul_sub_rhs(335, 154, 1458, 1480);s.store_exp(336, 335);}
        s.b[1768] = (s.v[1458] >= s.v[1480]);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) {s.store_mul_scaled_sqrt_ad_rhs(1472, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1535, 1472);s.store_scalar(1514, 0.0);s.store_scalar(1474, 0.0);s.store_sqrt_mul_sub_rhs(1444, 1543, 1480, 1461);}
        s.b[1769] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {s.store_offset_sub(781, 1444, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1770] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });s.b[1771] = (2.0 == 1.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && s.b[1771]) {s.store_scalar(720, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1772] = (2.0 == 2.0);s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && s.b[1772]) {s.store_scalar(720, 2.0);}
        s.b[1773] = (2.0 == 4.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && s.b[1773]) {s.store_scalar(720, 3.0);}
        s.b[1774] = (2.0 == 8.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && (!s.b[1773])) && s.b[1774]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && (!s.b[1770])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1444, 965, (-1e-8), 780);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && (!s.b[1769])) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && (!s.b[1769])) {s.store_scalar(337, 1.0);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) {s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));s.store_mul(1495, 1444, 1542);s.store_mul_scale_offset_indices(1496, 1540, 1448, -1.0, 0.0);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)));s.store_mul_sqrt_mixed_ia(1472, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1775] = ((s.v[1450] > s.v[965]) && (s.v[1475] != 2.0));s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1775]) {s.store_scalar(1474, 0.0);s.store_scalar(1514, 0.0);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1775])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1474, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1514, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {s.store_scalar(1535, 0.0);s.store_sub(335, 1480, 1461);}
        s.b[1776] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1777] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });s.b[1778] = (2.0 == 1.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && s.b[1778]) {s.store_scalar(720, 1.0);}
        s.b[1779] = (2.0 == 2.0);s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && s.b[1779]) {s.store_scalar(720, 2.0);}
        s.b[1780] = (2.0 == 4.0);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && s.b[1780]) {s.store_scalar(720, 3.0);}
        s.b[1781] = (2.0 == 8.0);s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && (!s.b[1780])) && s.b[1781]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && (!s.b[1777])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1776])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {s.store_sqrt_mul(1444, 1543, 336);}
        s.b[1782] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {s.store_offset_sub(781, 1444, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1783] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });s.b[1784] = (2.0 == 1.0);s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && s.b[1784]) {s.store_scalar(720, 1.0);}
        s.b[1785] = (2.0 == 2.0);s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && s.b[1785]) {s.store_scalar(720, 2.0);}
        s.b[1786] = (2.0 == 4.0);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && s.b[1786]) {s.store_scalar(720, 3.0);}
        s.b[1787] = (2.0 == 8.0);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && (!s.b[1786])) && s.b[1787]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && (!s.b[1783])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1444, 965, (-1e-8), 780);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1782])) {
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1782])) {s.store_scalar(337, 1.0);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));s.store_mul(1495, 1444, 1542);s.store_mul_scale_offset_indices(1496, 1540, 1448, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1788] = (((s.v[1458] - s.v[1508]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1458), s.ad_value(1508)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1789] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });s.b[1790] = (2.0 == 1.0);s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && s.b[1790]) {s.store_scalar(720, 1.0);}
        s.b[1791] = (2.0 == 2.0);s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && s.b[1791]) {s.store_scalar(720, 2.0);}
        s.b[1792] = (2.0 == 4.0);s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1792]) {s.store_scalar(720, 3.0);}
        s.b[1793] = (2.0 == 8.0);s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1792])) && s.b[1793]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && (!s.b[1789])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
        }
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1788])) {s.store_sub(336, 1458, 1508);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1512, 209, -1.0, 338);}
        if (s.b[1439] && s.b[1440]) {s.copy_ad(87, 1457);s.copy_ad(91, 1458);s.store_sub(94, 1458, 1457);s.store_neg_add(335, 1471, 1472);}
        s.b[1794] = ((s.v[335] < s.v[1536]) && (s.v[1536] >= 0.0));s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {s.store_sub(781, 1536, 335);s.store_square(722, 781);s.store_square(723, 1536);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1795] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });s.b[1796] = (2.0 == 1.0);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {s.store_scalar(720, 1.0);}
        s.b[1797] = (2.0 == 2.0);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && s.b[1797]) {s.store_scalar(720, 2.0);}
        s.b[1798] = (2.0 == 4.0);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && s.b[1798]) {s.store_scalar(720, 3.0);}
        s.b[1799] = (2.0 == 8.0);s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && (!s.b[1798])) && s.b[1799]) {s.store_scalar(720, 4.0);}
        if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t17: usize = 0;
        while {
            let t16: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;
            if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1794]) && (!s.b[1795])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1536, 726);s.store_div_scaled_product3_indices(334, 1536, 725, 726, 1.0, 770, 1.0);s.store_sub(1552, 1536, 780);}
        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
        }
        if ((s.b[1439] && s.b[1440]) && (!s.b[1794])) {s.copy_ad(1552, 335);s.store_scalar(334, 1.0);}
        if (s.b[1439] && s.b[1440]) {s.store_mul3_affine_lhs(1499, 154, 1552, 1.0 / (2.0), 0.0, 94);s.store_sub(1500, 1512, 1511);s.store_add(248, 1499, 1500);s.store_neg(133, 1511);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);s.store_mul(339, 336, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_mul(342, 339, 251);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(133), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);s.copy_ad(1554, 255);}
        s.b[1800] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1800]) {s.store_scalar(337, 1.0);}
        s.b[1801] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1800])) && s.b[1801]) {s.copy_ad(337, 335);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1800])) && (!s.b[1801])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[1802] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1802]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[1803] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && s.b[1803]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && (!s.b[1803])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && (!s.b[1803])) {s.store_mul(339, 338, 340);}
        if (s.b[1439] && s.b[1440]) {s.store_mul(253, 254, 339);}
        s.b[1804] = (s.v[349] > 1e-6);s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_div_square_rhs(336, 1498, 185);s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1434, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1805] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1806] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });s.b[1807] = (2.0 == 1.0);s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && s.b[1807]) {s.store_scalar(720, 1.0);}
        s.b[1808] = (2.0 == 2.0);s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && s.b[1808]) {s.store_scalar(720, 2.0);}
        s.b[1809] = (2.0 == 4.0);s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && s.b[1809]) {s.store_scalar(720, 3.0);}
        s.b[1810] = (2.0 == 8.0);s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && (!s.b[1809])) && s.b[1810]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;
            if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && (!s.b[1806])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1805])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);s.store_mul(344, 344, 975);}
        s.b[1811] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {s.store_sub_offset_lhs(781, 972, 4.0, 344);s.store_square(722, 781);s.store_scalar(723, (4.0 * 4.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1812] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });s.b[1813] = (4.0 == 1.0);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && s.b[1813]) {s.store_scalar(720, 1.0);}
        s.b[1814] = (4.0 == 2.0);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && s.b[1814]) {s.store_scalar(720, 2.0);}
        s.b[1815] = (4.0 == 4.0);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && s.b[1815]) {s.store_scalar(720, 3.0);}
        s.b[1816] = (4.0 == 8.0);s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && (!s.b[1815])) && s.b[1816]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && (!s.b[1812])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 4.0);s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);s.store_sub_offset_lhs(344, 972, 4.0, 780);}
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1811])) {
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1811])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_div(335, 349, 344);}
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {s.store_mul(340, 338, 337);s.store_div(1553, 349, 340);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1804])) {s.copy_ad(1553, 349);}
        if (s.b[1439] && s.b[1440]) {s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);s.store_neg(133, 1492);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[376] - 1.0));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1551, 1553, 170);s.store_div_scaled_product_indices(335, 254, 1551, 1.0, 973, 1.0);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_div(1502, 254, 338);s.store_mul3_affine_lhs(987, 1492, 1502, (-s.v[632]), 0.0, 1551);s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);s.store_neg(133, 1501);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[376] - 1.0));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && s.b[1440]) {s.store_div(1551, 1553, 170);s.store_div_scaled_product_indices(335, 254, 1551, 1.0, 973, 1.0);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1439] && s.b[1440]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1439] && s.b[1440]) {s.store_div(1503, 254, 338);s.store_mul3_affine_lhs(1550, 1501, 1503, (-s.v[632]), 0.0, 1551);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1550, 1.0);s.store_mul3_lhs(986, 115, 248, 253);s.copy_ad(984, 253);s.copy_ad(790, 349);}
        s.b[1817] = (p[283] != 0.0);s.store_scalar(1817, if s.b[1817] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1457), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[1818] = (s.v[336] < 0.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1817]) && s.b[1818]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1435, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1457, 1.0, 340, 1.0, 1434, -1.0);s.store_add_product3_rhs_indices(338, 338, 1435, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1817])) {s.store_scalar(343, 0.0);}
        s.b[1819] = (p[287] != 0.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1819]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1435);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1819])) {s.store_scalar(342, 0.0);}
        s.b[1820] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1820]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[1821] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });s.b[1822] = (p[296] > 0.0);s.store_scalar(1822, if s.b[1822] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p[300]), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p[296] + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1822])) {s.copy_ad(341, 647);}
        s.b[1823] = (s.v[793] >= 0.0);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1823]) {s.copy_ad(369, 793);}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1823])) {s.store_scalar(369, 0.0);}
        s.b[1824] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1824]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p[297] - 1.0)) * ((20.0 + 1.0) - ((0.5 * p[297]) * 20.0))) * ((1e-12) as f64).powf(p[297])));s.store_scalar(379, ((((0.5 * p[297]) * (((20.0 + 1.0)) as f64).powf((p[297] - 1.0))) / 20.0) * ((1e-12) as f64).powf((p[297] - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1824])) {s.store_powf_offset_input(335, 369, 1e-12, p[297]);}
        if ((s.b[1439] && s.b[1440]) && s.b[1821]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1821])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1439] && s.b[1440]) {s.store_add_scaled_inputs4_indices(131, 1473, (-0.5), 1474, (-0.5), 1494, (-0.5), 1496, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(1513), 1.0, s.ad_value(1514), 1.0), s.ad_value(1493)), 1495, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1534, 1535, (-0.5));s.store_neg(238, 1534);s.copy_ad(255, 1554);}
        s.b[1825] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1825]) {s.store_scalar(78, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.copy_ad(1851, 960);s.store_scale(1901, 964, 1.6021918e-19);s.store_scale(1880, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1900, 622, 1.6021918e-19);s.store_square(1899, 965);s.store_div_from_scalar(1904, (2.0 * 1.034943e-10), 1901);s.store_div_from_scalar(1905, (2.0 * 1.034943e-10), 1900);s.store_div(1898, 964, 622);s.store_div_from_scalar_offset_input(1897, 1.0, 1898, 1.0);s.store_div_square_rhs(1902, 1880, 185);s.store_div_from_scalar(1903, 2.0, 1902);s.store_scalar(1906, 4.0);s.store_scalar(1907, 0.1);s.store_scalar(1908, 0.1);s.store_offset(1909, 961, p[407]);s.store_scalar(1910, 3.0);s.store_scalar(1849, 0.0);s.store_scalar(1850, 0.0);s.store_scalar(1858, 0.0);s.store_scalar(1859, 0.0);s.store_scalar(1891, 0.0);s.store_scalar(1892, 0.0);s.store_scalar(1862, 0.0);s.store_scalar(1864, 0.0);s.store_scalar(1863, 0.0);s.store_scalar(1865, 0.0);s.store_scalar(1835, 0.0);s.store_scalar(1830, 0.0);s.copy_ad(1883, 1431);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));s.store_div_scaled_product_add_scaled_denominator_indices(962, 1904, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);s.store_sub(335, 1851, 1434);}
        s.b[1913] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1914] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });s.b[1915] = (4.0 == 1.0);s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && s.b[1915]) {s.store_scalar(720, 1.0);}
        s.b[1916] = (4.0 == 2.0);s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && s.b[1916]) {s.store_scalar(720, 2.0);}
        s.b[1917] = (4.0 == 4.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && s.b[1917]) {s.store_scalar(720, 3.0);}
        s.b[1918] = (4.0 == 8.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && (!s.b[1917])) && s.b[1918]) {s.store_scalar(720, 4.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && (!s.b[1914])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1913])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1834, 962, 336);s.store_sqrt(1832, 1834);}
        s.b[1919] = (p[345] != 0.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p[345]), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_scalar(1847, 0.0);}
        s.b[1920] = (s.v[1832] > s.v[965]);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1920]) {s.copy_ad(1831, 965);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1920])) {s.copy_ad(1831, 1832);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(1856, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 1856, 1.0, 1883, -1.0, 1851, 1.0, (-0.01));s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_square_add(782, 781, 782);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1856, 1883, 1.0, 1851, (-1.0), 781, 0.5, 782, 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(1884, 1851, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);s.store_offset_sub(1830, 965, 1831, 1e-15);s.store_scalar(79, 0.0);s.store_scalar(1846, 0.2);s.copy_ad(1849, 1856);s.copy_ad(1852, 1847);s.copy_ad(1854, 1884);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t27: usize = 0;
        while {
            let t26: f64 = if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;
            if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul_sub_mixed_iai(1854, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1852), 1.0), 1851);s.store_mul(1838, 1897, 1898);s.store_sub(335, 1852, 1854);}
            s.b[1921] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1922] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });s.b[1923] = (2.0 == 1.0);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && s.b[1923]) {s.store_scalar(720, 1.0);}
            s.b[1924] = (2.0 == 2.0);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && s.b[1924]) {s.store_scalar(720, 2.0);}
            s.b[1925] = (2.0 == 4.0);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && s.b[1925]) {s.store_scalar(720, 3.0);}
            s.b[1926] = (2.0 == 8.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && (!s.b[1925])) && s.b[1926]) {s.store_scalar(720, 4.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {s.store_scalar(719, 0.0);}
            let mut t23: usize = 0;
            while {
                let t22: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t22 != 0.0
            } {
                t23 += 1;
                if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && (!s.b[1922])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1921])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_mul(1826, 1904, 336);}
            s.b[1927] = ((s.v[1826] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {s.store_offset_sub(781, 1826, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1928] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });s.b[1929] = (2.0 == 1.0);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && s.b[1929]) {s.store_scalar(720, 1.0);}
            s.b[1930] = (2.0 == 2.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && s.b[1930]) {s.store_scalar(720, 2.0);}
            s.b[1931] = (2.0 == 4.0);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && s.b[1931]) {s.store_scalar(720, 3.0);}
            s.b[1932] = (2.0 == 8.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && (!s.b[1931])) && s.b[1932]) {s.store_scalar(720, 4.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {s.store_scalar(719, 0.0);}
            let mut t25: usize = 0;
            while {
                let t24: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t24 != 0.0
            } {
                t25 += 1;
                if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && (!s.b[1928])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1826, 965, (-1e-8), 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {s.store_scalar(337, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1862, 1826, 1901);s.store_mul_ad_product_lhs_mixed_ai(1840, A::div_from_scalar(1.034943e-10, s.ad_value(1826)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1842, A::div_from_scalar((-1.034943e-10), s.ad_value(1826)), 334, 337);}
            s.b[1933] = (p[49] == 0.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1933]) {s.store_add_mixed_ai(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1849);s.store_scalar(1836, 1.0);s.store_scalar(1837, 0.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1933])) {s.store_add_mixed_ia(1835, 1849, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1826), A::sub_scaled_inputs(s.ad_value(1826), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));s.store_scalar(1836, 1.0);s.store_mul_scale_offset_mixed_ai(1837, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1826)), s.ad_value(334), (-1.0)), 1838, -1.0, 1.0);}
            s.b[1934] = ((s.v[1835] > (s.v[1847] - s.v[1846])) && (s.v[1846] >= 0.0));s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {s.store_add_scaled_inputs3_indices(781, 1835, 1.0, 1847, (-1.0), 1846, 1.0);s.store_square(722, 781);s.store_square(723, 1846);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1935] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });s.b[1936] = (4.0 == 1.0);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && s.b[1936]) {s.store_scalar(720, 1.0);}
            s.b[1937] = (4.0 == 2.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && s.b[1937]) {s.store_scalar(720, 2.0);}
            s.b[1938] = (4.0 == 4.0);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && s.b[1938]) {s.store_scalar(720, 3.0);}
            s.b[1939] = (4.0 == 8.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && (!s.b[1938])) && s.b[1939]) {s.store_scalar(720, 4.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {s.store_scalar(719, 0.0);}
            let mut t1f: usize = 0;
            while {
                let t1e: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t1e != 0.0
            } {
                t1f += 1;
                if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && (!s.b[1935])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1846, 726);s.store_div_scaled_product3_indices(334, 1846, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1835, 1847, 1.0, 1846, (-1.0), 780, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {s.store_scalar(334, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1836, 1836, 334);s.store_mul(1837, 1837, 334);s.store_add_scaled_inputs3_indices(335, 1854, 1.0, 1883, (-1.0), 1851, 1.0);}
            s.b[1940] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });s.b[1942] = (2.0 == 1.0);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && s.b[1942]) {s.store_scalar(720, 1.0);}
            s.b[1943] = (2.0 == 2.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && s.b[1943]) {s.store_scalar(720, 2.0);}
            s.b[1944] = (2.0 == 4.0);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && s.b[1944]) {s.store_scalar(720, 3.0);}
            s.b[1945] = (2.0 == 8.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && (!s.b[1944])) && s.b[1945]) {s.store_scalar(720, 4.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {s.store_scalar(719, 0.0);}
            let mut t21: usize = 0;
            while {
                let t20: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t20 != 0.0
            } {
                t21 += 1;
                if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && (!s.b[1941])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1940])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_mul(1828, 1905, 336);s.store_mul_scale_offset_indices(1863, 1900, 1828, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1844, (-1.034943e-10), 1828, 337);s.store_mul_sub_rhs(335, 154, 1849, 1852);s.store_exp(336, 335);}
            s.b[1946] = (s.v[1849] >= s.v[1852]);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1946]) {s.store_mul_scaled_sqrt_ad_rhs(1858, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1893, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1858, 1.0);s.store_neg(1895, 1893);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1946])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)));s.store_mul_sqrt_mixed_ia(1858, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1858, 1.0);s.store_mul_add_mixed_iaa(1893, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1895, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1866, A::add_scaled_product(s.ad_value(1858), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1849)), 1.0), 1.0, 1862, 1.0, 1863, 1.0);s.store_sub(1867, 1893, 185);s.store_add_mixed_ia(1868, 1895, A::add_scaled_value_products(s.ad_value(1840), 1.0, s.ad_value(1842), s.ad_value(1838), 1.0, s.ad_value(1844), s.ad_value(1838), 1.0));s.store_sub(1869, 1852, 1835);s.store_neg(1870, 1836);s.store_sub_from_scalar(1871, 1.0, 1837);s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[1872] > 0.0) {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, (-1e-25));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.copy_ad(1874, 1871);s.store_neg(1875, 1868);s.store_neg(1876, 1870);s.copy_ad(1877, 1867);s.store_mul_add_scaled_products_indices_rhs(1878, 1873, 1874, 1866, -1.0, 1875, 1869, -1.0);s.store_mul_add_scaled_products_indices_rhs(1879, 1873, 1876, 1866, -1.0, 1877, 1869, -1.0);s.store_abs(335, 1878);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                    s.store_abs(335, 1879);
                } else {
                }
            }
            s.b[1947] = (s.v[335] > 0.1);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1947]) {s.store_mul_div_from_scalar_lhs_ad_indices(1878, 0.1, 335, 1878);s.store_mul_div_from_scalar_lhs_ad_indices(1879, 0.1, 335, 1879);}
            s.b[1948] = (s.v[335] < 1e-12);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1948]) {s.store_scalar(79, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.store_add(1849, 1849, 1878);s.store_add(1852, 1852, 1879);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul_sub_rhs(335, 154, 1849, 1852);s.store_exp(336, 335);}
        s.b[1950] = (s.v[1849] >= s.v[1852]);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1950]) {s.copy_ad(1888, 1858);s.store_scalar(1891, 0.0);s.store_scalar(1860, 0.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) {s.store_scalar(1888, 0.0);s.store_mul_sqrt_mixed_ia(1891, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[1951] = (s.v[1832] > s.v[965]);s.store_scalar(1951, if s.b[1951] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && s.b[1951]) {s.store_scalar(1860, 0.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && (!s.b[1951])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1860, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[1952] = (((s.v[1849] - s.v[1847]) < s.v[1907]) && (s.v[1907] >= 0.0));s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 1849, -1.0, 1847, 1.0);s.store_square(722, 781);s.store_square(723, 1907);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1953] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1953, if s.b[1953] { 1.0 } else { 0.0 });s.b[1954] = (4.0 == 1.0);s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && s.b[1954]) {s.store_scalar(720, 1.0);}
        s.b[1955] = (4.0 == 2.0);s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && s.b[1955]) {s.store_scalar(720, 2.0);}
        s.b[1956] = (4.0 == 4.0);s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && s.b[1956]) {s.store_scalar(720, 3.0);}
        s.b[1957] = (4.0 == 8.0);s.store_scalar(1957, if s.b[1957] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && (!s.b[1956])) && s.b[1957]) {s.store_scalar(720, 4.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {s.store_scalar(719, 0.0);}
        let mut t29: usize = 0;
        while {
            let t28: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && (!s.b[1953])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1907, 726);s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1907, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1952])) {s.store_sub(336, 1849, 1847);s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1885, 209, -1.0, 338);s.copy_ad(349, 790);}
        s.b[1958] = (s.v[790] > 1e-6);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_scalar(344, 1e-25);s.store_offset_mul_ad(338, s.ad_value(1903), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1903, 1.0);}
        s.b[1959] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(1959, if s.b[1959] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1960] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1960, if s.b[1960] { 1.0 } else { 0.0 });s.b[1961] = (2.0 == 1.0);s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && s.b[1961]) {s.store_scalar(720, 1.0);}
        s.b[1962] = (2.0 == 2.0);s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && s.b[1962]) {s.store_scalar(720, 2.0);}
        s.b[1963] = (2.0 == 4.0);s.store_scalar(1963, if s.b[1963] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && s.b[1963]) {s.store_scalar(720, 3.0);}
        s.b[1964] = (2.0 == 8.0);s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && (!s.b[1963])) && s.b[1964]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {s.store_scalar(719, 0.0);}
        let mut t2b: usize = 0;
        while {
            let t2a: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2a != 0.0
        } {
            t2b += 1;
            if t2b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && (!s.b[1960])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1902, 1.0, 337);}
        s.b[1965] = ((s.v[344] < 1.0) && (1.0 >= 0.0));s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {s.store_sub_from_scalar(781, 1.0, 344);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1966] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1966, if s.b[1966] { 1.0 } else { 0.0 });s.b[1967] = (2.0 == 1.0);s.store_scalar(1967, if s.b[1967] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && s.b[1967]) {s.store_scalar(720, 1.0);}
        s.b[1968] = (2.0 == 2.0);s.store_scalar(1968, if s.b[1968] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && s.b[1968]) {s.store_scalar(720, 2.0);}
    }
}
