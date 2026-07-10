#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        s.b[1763] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = ((s.v[1482] > (s.v[1460] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {s.store_offset_sub(781, 1482, 1460, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1765] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });s.b[1766] = (2.0 == 1.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && s.b[1766]) {s.store_scalar(720, 1.0);}
        s.b[1767] = (2.0 == 2.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && s.b[1767]) {s.store_scalar(720, 2.0);}
        s.b[1768] = (2.0 == 4.0);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && s.b[1768]) {s.store_scalar(720, 3.0);}
        s.b[1769] = (2.0 == 8.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && (!s.b[1768])) && s.b[1769]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && (!s.b[1765])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1482, 1460, (-0.02), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {s.store_scalar(335, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_mul_sub_mixed_iai(1463, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), 1461);s.store_mul_sub_rhs(335, 154, 1460, 1482);s.store_exp(336, 335);}
        s.b[1770] = (s.v[1460] >= s.v[1482]);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {s.store_mul_scaled_sqrt_ad_rhs(1474, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1537, 1474);s.store_scalar(1516, 0.0);s.store_scalar(1476, 0.0);s.store_sqrt_mul_sub_rhs(1446, 1545, 1482, 1463);}
        s.b[1771] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1772] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });s.b[1773] = (2.0 == 1.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && s.b[1773]) {s.store_scalar(720, 1.0);}
        s.b[1774] = (2.0 == 2.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && s.b[1774]) {s.store_scalar(720, 2.0);}
        s.b[1775] = (2.0 == 4.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && s.b[1775]) {s.store_scalar(720, 3.0);}
        s.b[1776] = (2.0 == 8.0);s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && (!s.b[1775])) && s.b[1776]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && (!s.b[1772])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {s.store_scalar(337, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1497, 1446, 1544);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1474, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1777] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1777]) {s.store_scalar(1476, 0.0);s.store_scalar(1516, 0.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1777])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1476, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1516, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_scalar(1537, 0.0);s.store_sub(335, 1482, 1463);}
        s.b[1778] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1779] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });s.b[1780] = (2.0 == 1.0);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && s.b[1780]) {s.store_scalar(720, 1.0);}
        s.b[1781] = (2.0 == 2.0);s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && s.b[1781]) {s.store_scalar(720, 2.0);}
        s.b[1782] = (2.0 == 4.0);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && s.b[1782]) {s.store_scalar(720, 3.0);}
        s.b[1783] = (2.0 == 8.0);s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && (!s.b[1782])) && s.b[1783]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && (!s.b[1779])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1778])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_sqrt_mul(1446, 1545, 336);}
        s.b[1784] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1785] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });s.b[1786] = (2.0 == 1.0);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && s.b[1786]) {s.store_scalar(720, 1.0);}
        s.b[1787] = (2.0 == 2.0);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && s.b[1787]) {s.store_scalar(720, 2.0);}
        s.b[1788] = (2.0 == 4.0);s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && s.b[1788]) {s.store_scalar(720, 3.0);}
        s.b[1789] = (2.0 == 8.0);s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && (!s.b[1788])) && s.b[1789]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && (!s.b[1785])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {s.store_scalar(337, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1497, 1446, 1544);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);}
        s.b[1790] = (((s.v[1460] - s.v[1510]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1460), s.ad_value(1510)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });s.b[1792] = (2.0 == 1.0);s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && s.b[1792]) {s.store_scalar(720, 1.0);}
        s.b[1793] = (2.0 == 2.0);s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {s.store_scalar(720, 2.0);}
        s.b[1794] = (2.0 == 4.0);s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {s.store_scalar(720, 3.0);}
        s.b[1795] = (2.0 == 8.0);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) && s.b[1795]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && (!s.b[1791])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1790])) {s.store_sub(336, 1460, 1510);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1514, 209, -1.0, 338);}
        if (s.b[1441] && s.b[1442]) {s.copy_ad(87, 1459);s.copy_ad(91, 1460);s.store_sub(94, 1460, 1459);s.store_neg_add(335, 1473, 1474);}
        s.b[1796] = ((s.v[335] < s.v[1538]) && (s.v[1538] >= 0.0));s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {s.store_sub(781, 1538, 335);s.store_square(722, 781);s.store_square(723, 1538);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1797] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });s.b[1798] = (2.0 == 1.0);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && s.b[1798]) {s.store_scalar(720, 1.0);}
        s.b[1799] = (2.0 == 2.0);s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && s.b[1799]) {s.store_scalar(720, 2.0);}
        s.b[1800] = (2.0 == 4.0);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && s.b[1800]) {s.store_scalar(720, 3.0);}
        s.b[1801] = (2.0 == 8.0);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {s.store_scalar(719, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && (!s.b[1797])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1538, 726);s.store_div_scaled_product3_indices(334, 1538, 725, 726, 1.0, 770, 1.0);s.store_sub(1554, 1538, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1796])) {s.copy_ad(1554, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul3_affine_lhs(1501, 154, 1554, 1.0 / (2.0), 0.0, 94);s.store_sub(1502, 1514, 1513);s.store_add(248, 1501, 1502);s.store_neg(133, 1513);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_mul(339, 336, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(133), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);s.copy_ad(1556, 255);}
        s.b[1802] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1802]) {s.store_scalar(337, 1.0);}
        s.b[1803] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && s.b[1803]) {s.copy_ad(337, 335);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && (!s.b[1803])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[1804] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1804]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[1805] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && s.b[1805]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {s.store_mul(339, 338, 340);}
        if (s.b[1441] && s.b[1442]) {s.store_mul(253, 254, 339);}
        s.b[1806] = (s.v[349] > 1e-6);s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_div_square_rhs(336, 1500, 185);s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1436, -1.0);s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1807] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });s.b[1809] = (2.0 == 1.0);s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {s.store_scalar(720, 1.0);}
        s.b[1810] = (2.0 == 2.0);s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {s.store_scalar(720, 2.0);}
        s.b[1811] = (2.0 == 4.0);s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && s.b[1811]) {s.store_scalar(720, 3.0);}
        s.b[1812] = (2.0 == 8.0);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && (!s.b[1811])) && s.b[1812]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && (!s.b[1808])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1807])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);s.store_mul(344, 344, 975);}
        s.b[1813] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {s.store_sub_offset_lhs(781, 972, 4.0, 344);s.store_square(722, 781);s.store_scalar(723, (4.0 * 4.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1814] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });s.b[1815] = (4.0 == 1.0);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && s.b[1815]) {s.store_scalar(720, 1.0);}
        s.b[1816] = (4.0 == 2.0);s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && s.b[1816]) {s.store_scalar(720, 2.0);}
        s.b[1817] = (4.0 == 4.0);s.store_scalar(1817, if s.b[1817] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && s.b[1817]) {s.store_scalar(720, 3.0);}
        s.b[1818] = (4.0 == 8.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && (!s.b[1817])) && s.b[1818]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && (!s.b[1814])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 4.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);s.store_sub_offset_lhs(344, 972, 4.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_div(335, 349, 344);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_mul(340, 338, 337);s.store_div(1555, 349, 340);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1806])) {s.copy_ad(1555, 349);}
        if (s.b[1441] && s.b[1442]) {s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_neg(133, 1494);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1553, 1555, 170);s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_div(1504, 254, 338);s.store_mul3_affine_lhs(987, 1494, 1504, (-s.v[632]), 0.0, 1553);s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_neg(133, 1503);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1553, 1555, 170);s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_div(1505, 254, 338);s.store_mul3_affine_lhs(1552, 1503, 1505, (-s.v[632]), 0.0, 1553);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && s.b[1442]) {s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1552, 1.0);s.store_mul3_lhs(986, 115, 248, 253);s.copy_ad(984, 253);s.copy_ad(790, 349);}
        s.b[1819] = (p.p283 != 0.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1459), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[1820] = (s.v[336] < 0.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1819]) && s.b[1820]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p.p284);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1437, p.p285, 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1459, 1.0, 340, 1.0, 1436, -1.0);s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1819])) {s.store_scalar(343, 0.0);}
        s.b[1821] = (p.p287 != 0.0);s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1821]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1437);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1821])) {s.store_scalar(342, 0.0);}
        s.b[1822] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(1822, if s.b[1822] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1822]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[1823] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });s.b[1824] = (p.p296 > 0.0);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p.p296 + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1824])) {s.copy_ad(341, 647);}
        s.b[1825] = (s.v[793] >= 0.0);s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1825]) {s.copy_ad(369, 793);}
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1825])) {s.store_scalar(369, 0.0);}
        s.b[1826] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(1826, if s.b[1826] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1826]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1826])) {s.store_powf_offset_input(335, 369, 1e-12, p.p297);}
        if ((s.b[1441] && s.b[1442]) && s.b[1823]) {s.store_powf_offset_input(343, 369, 1e-12, p.p299);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1823])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1441] && s.b[1442]) {s.store_add_scaled_inputs4_indices(131, 1475, (-0.5), 1476, (-0.5), 1496, (-0.5), 1498, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(1515), 1.0, s.ad_value(1516), 1.0), s.ad_value(1495)), 1497, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1536, 1537, (-0.5));s.store_neg(238, 1536);s.copy_ad(255, 1556);}
        s.b[1827] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1827]) {s.store_scalar(78, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.copy_ad(1853, 960);s.store_scale(1903, 964, 1.6021918e-19);s.store_scale(1882, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1902, 622, 1.6021918e-19);s.store_square(1901, 965);s.store_div_from_scalar(1906, (2.0 * 1.034943e-10), 1903);s.store_div_from_scalar(1907, (2.0 * 1.034943e-10), 1902);s.store_div(1900, 964, 622);s.store_div_from_scalar_offset_input(1899, 1.0, 1900, 1.0);s.store_div_square_rhs(1904, 1882, 185);s.store_div_from_scalar(1905, 2.0, 1904);s.store_scalar(1908, 4.0);s.store_scalar(1909, 0.1);s.store_scalar(1910, 0.1);s.store_offset(1911, 961, p.p407);s.store_scalar(1912, 3.0);s.store_scalar(1851, 0.0);s.store_scalar(1852, 0.0);s.store_scalar(1860, 0.0);s.store_scalar(1861, 0.0);s.store_scalar(1893, 0.0);s.store_scalar(1894, 0.0);s.store_scalar(1864, 0.0);s.store_scalar(1866, 0.0);s.store_scalar(1865, 0.0);s.store_scalar(1867, 0.0);s.store_scalar(1837, 0.0);s.store_scalar(1832, 0.0);s.copy_ad(1885, 1433);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));s.store_div_scaled_product_add_scaled_denominator_indices(962, 1906, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);s.store_sub(335, 1853, 1436);}
        s.b[1915] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1916] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });s.b[1917] = (4.0 == 1.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && s.b[1917]) {s.store_scalar(720, 1.0);}
        s.b[1918] = (4.0 == 2.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && s.b[1918]) {s.store_scalar(720, 2.0);}
        s.b[1919] = (4.0 == 4.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && s.b[1919]) {s.store_scalar(720, 3.0);}
        s.b[1920] = (4.0 == 8.0);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && (!s.b[1919])) && s.b[1920]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;assert!(t11 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && (!s.b[1916])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1915])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(1836, 962, 336);s.store_sqrt(1834, 1836);}
        s.b[1921] = (p.p345 != 0.0);s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p.p345), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_scalar(1849, 0.0);}
        s.b[1922] = (s.v[1834] > s.v[965]);s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1922]) {s.copy_ad(1833, 965);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1922])) {s.copy_ad(1833, 1834);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(1858, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 1858, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1858, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(1886, 1853, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);s.store_offset_sub(1832, 965, 1833, 1e-15);s.store_scalar(79, 0.0);s.store_scalar(1848, 0.2);s.copy_ad(1851, 1858);s.copy_ad(1854, 1849);s.copy_ad(1856, 1886);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul_sub_mixed_iai(1856, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1854), 1.0), 1853);s.store_mul(1840, 1899, 1900);s.store_sub(335, 1854, 1856);}
            s.b[1923] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1924] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });s.b[1925] = (2.0 == 1.0);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && s.b[1925]) {s.store_scalar(720, 1.0);}
            s.b[1926] = (2.0 == 2.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && s.b[1926]) {s.store_scalar(720, 2.0);}
            s.b[1927] = (2.0 == 4.0);s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && s.b[1927]) {s.store_scalar(720, 3.0);}
            s.b[1928] = (2.0 == 8.0);s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && (!s.b[1927])) && s.b[1928]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {s.store_scalar(719, 0.0);}
            let mut t17: usize = 0;
            while {
                let t16: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t16 != 0.0
            } {
                t17 += 1;assert!(t17 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && (!s.b[1924])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1923])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sqrt_mul(1828, 1906, 336);}
            s.b[1929] = ((s.v[1828] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {s.store_offset_sub(781, 1828, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1930] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });s.b[1931] = (2.0 == 1.0);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && s.b[1931]) {s.store_scalar(720, 1.0);}
            s.b[1932] = (2.0 == 2.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && s.b[1932]) {s.store_scalar(720, 2.0);}
            s.b[1933] = (2.0 == 4.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && s.b[1933]) {s.store_scalar(720, 3.0);}
            s.b[1934] = (2.0 == 8.0);s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && (!s.b[1933])) && s.b[1934]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {s.store_scalar(719, 0.0);}
            let mut t19: usize = 0;
            while {
                let t18: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t18 != 0.0
            } {
                t19 += 1;assert!(t19 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && (!s.b[1930])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1828, 965, (-1e-8), 780);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {s.store_scalar(337, 1.0);}
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(1864, 1828, 1903);s.store_mul_ad_product_lhs_mixed_ai(1842, A::div_from_scalar(1.034943e-10, s.ad_value(1828)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1844, A::div_from_scalar((-1.034943e-10), s.ad_value(1828)), 334, 337);}
            s.b[1935] = (p.p49 == 0.0);s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1935]) {s.store_add_mixed_ai(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1851);s.store_scalar(1838, 1.0);s.store_scalar(1839, 0.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1935])) {s.store_add_mixed_ia(1837, 1851, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1828), A::sub_scaled_inputs(s.ad_value(1828), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));s.store_scalar(1838, 1.0);s.store_mul_scale_offset_mixed_ai(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1828)), s.ad_value(334), (-1.0)), 1840, -1.0, 1.0);}
            s.b[1936] = ((s.v[1837] > (s.v[1849] - s.v[1848])) && (s.v[1848] >= 0.0));s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1849, (-1.0), 1848, 1.0);s.store_square(722, 781);s.store_square(723, 1848);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1937] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });s.b[1938] = (4.0 == 1.0);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && s.b[1938]) {s.store_scalar(720, 1.0);}
            s.b[1939] = (4.0 == 2.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && s.b[1939]) {s.store_scalar(720, 2.0);}
            s.b[1940] = (4.0 == 4.0);s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && s.b[1940]) {s.store_scalar(720, 3.0);}
            s.b[1941] = (4.0 == 8.0);s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && (!s.b[1940])) && s.b[1941]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {s.store_scalar(719, 0.0);}
            let mut t13: usize = 0;
            while {
                let t12: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t12 != 0.0
            } {
                t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && (!s.b[1937])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1848, 726);s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1837, 1849, 1.0, 1848, (-1.0), 780, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {s.store_scalar(334, 1.0);}
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(1838, 1838, 334);s.store_mul(1839, 1839, 334);s.store_add_scaled_inputs3_indices(335, 1856, 1.0, 1885, (-1.0), 1853, 1.0);}
            s.b[1942] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1943] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });s.b[1944] = (2.0 == 1.0);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && s.b[1944]) {s.store_scalar(720, 1.0);}
            s.b[1945] = (2.0 == 2.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && s.b[1945]) {s.store_scalar(720, 2.0);}
            s.b[1946] = (2.0 == 4.0);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && s.b[1946]) {s.store_scalar(720, 3.0);}
            s.b[1947] = (2.0 == 8.0);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && (!s.b[1946])) && s.b[1947]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {s.store_scalar(719, 0.0);}
            let mut t15: usize = 0;
            while {
                let t14: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t14 != 0.0
            } {
                t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && (!s.b[1943])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1942])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sqrt_mul(1830, 1907, 336);s.store_mul_scale_offset_indices(1865, 1902, 1830, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1846, (-1.034943e-10), 1830, 337);s.store_mul_sub_rhs(335, 154, 1851, 1854);s.store_exp(336, 335);}
            s.b[1948] = (s.v[1851] >= s.v[1854]);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1948]) {s.store_mul_scaled_sqrt_ad_rhs(1860, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1895, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1860, 1.0);s.store_neg(1897, 1895);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1948])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)));s.store_mul_sqrt_mixed_ia(1860, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1860, 1.0);s.store_mul_add_mixed_iaa(1895, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1897, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1860), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1851)), 1.0), 1.0, 1864, 1.0, 1865, 1.0);s.store_sub(1869, 1895, 185);s.store_add_mixed_ia(1870, 1897, A::add_scaled_value_products(s.ad_value(1842), 1.0, s.ad_value(1844), s.ad_value(1840), 1.0, s.ad_value(1846), s.ad_value(1840), 1.0));s.store_sub(1871, 1854, 1837);s.store_neg(1872, 1838);s.store_sub_from_scalar(1873, 1.0, 1839);s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {s.copy_ad(1876, 1873);s.store_neg(1877, 1870);s.store_neg(1878, 1872);s.copy_ad(1879, 1869);s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);s.store_abs(335, 1880);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[1949] = (s.v[335] > 0.1);s.store_scalar(1949, if s.b[1949] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1949]) {s.store_mul_div_from_scalar_lhs_ad_indices(1880, 0.1, 335, 1880);s.store_mul_div_from_scalar_lhs_ad_indices(1881, 0.1, 335, 1881);}
            s.b[1950] = (s.v[335] < 1e-12);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1950]) {s.store_scalar(79, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {s.store_add(1851, 1851, 1880);s.store_add(1854, 1854, 1881);}
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul_sub_rhs(335, 154, 1851, 1854);s.store_exp(336, 335);}
        s.b[1952] = (s.v[1851] >= s.v[1854]);s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1952]) {s.copy_ad(1890, 1860);s.store_scalar(1893, 0.0);s.store_scalar(1862, 0.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) {s.store_scalar(1890, 0.0);s.store_mul_sqrt_mixed_ia(1893, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[1953] = (s.v[1834] > s.v[965]);s.store_scalar(1953, if s.b[1953] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && s.b[1953]) {s.store_scalar(1862, 0.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && (!s.b[1953])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1862, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[1954] = (((s.v[1851] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1851, -1.0, 1849, 1.0);s.store_square(722, 781);s.store_square(723, 1909);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1955] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });s.b[1956] = (4.0 == 1.0);s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && s.b[1956]) {s.store_scalar(720, 1.0);}
        s.b[1957] = (4.0 == 2.0);s.store_scalar(1957, if s.b[1957] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && s.b[1957]) {s.store_scalar(720, 2.0);}
        s.b[1958] = (4.0 == 4.0);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && s.b[1958]) {s.store_scalar(720, 3.0);}
        s.b[1959] = (4.0 == 8.0);s.store_scalar(1959, if s.b[1959] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && (!s.b[1958])) && s.b[1959]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;assert!(t1d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && (!s.b[1955])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1909, 726);s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1909, 780);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1954])) {s.store_sub(336, 1851, 1849);s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul_scaled_sqrt_rhs(1887, 209, -1.0, 338);s.copy_ad(349, 790);}
        s.b[1960] = (s.v[790] > 1e-6);s.store_scalar(1960, if s.b[1960] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_scalar(344, 1e-25);s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1905, 1.0);}
        s.b[1961] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1962] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });s.b[1963] = (2.0 == 1.0);s.store_scalar(1963, if s.b[1963] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && s.b[1963]) {s.store_scalar(720, 1.0);}
        s.b[1964] = (2.0 == 2.0);s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && s.b[1964]) {s.store_scalar(720, 2.0);}
        s.b[1965] = (2.0 == 4.0);s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && s.b[1965]) {s.store_scalar(720, 3.0);}
        s.b[1966] = (2.0 == 8.0);s.store_scalar(1966, if s.b[1966] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && (!s.b[1965])) && s.b[1966]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {s.store_scalar(719, 0.0);}
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;assert!(t1f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && (!s.b[1962])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1904, 1.0, 337);}
        s.b[1967] = ((s.v[344] < 1.0) && (1.0 >= 0.0));s.store_scalar(1967, if s.b[1967] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {s.store_sub_from_scalar(781, 1.0, 344);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1968] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1968, if s.b[1968] { 1.0 } else { 0.0 });s.b[1969] = (2.0 == 1.0);s.store_scalar(1969, if s.b[1969] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && s.b[1969]) {s.store_scalar(720, 1.0);}
        s.b[1970] = (2.0 == 2.0);s.store_scalar(1970, if s.b[1970] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && s.b[1970]) {s.store_scalar(720, 2.0);}
        s.b[1971] = (2.0 == 4.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && s.b[1971]) {s.store_scalar(720, 3.0);}
        s.b[1972] = (2.0 == 8.0);s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && (!s.b[1971])) && s.b[1972]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
    ) {
        let mut t21: usize = 0;
        while {
            let t20: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && (!s.b[1968])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(344, 1.0, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_div(335, 790, 344);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1960])) {s.copy_ad(348, 790);}
        s.b[1973] = (s.v[790] < 0.0);s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1973]) {s.copy_ad(1852, 1851);s.copy_ad(1857, 1856);s.copy_ad(1855, 1854);s.copy_ad(1863, 1862);s.copy_ad(1891, 1890);s.copy_ad(1888, 1887);s.copy_ad(1866, 1864);s.copy_ad(1867, 1865);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.copy_ad(1835, 1834);s.copy_ad(1850, 790);s.store_add_scaled_inputs3_offset_indices(781, 1851, 1.0, 1850, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 1851, 1850, (4.0 * 0.01));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1859, 1851, 1.0, 1850, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 1859, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1859, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1885))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_add_product3_rhs_mixed_iia(92, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_scalar(79, 0.0);s.copy_ad(1852, 1859);s.copy_ad(1855, 1850);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2b: usize = 0;
        while {
            let t2a: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t2a != 0.0
        } {
            t2b += 1;assert!(t2b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul_sub_mixed_iai(1857, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1855), 1.0), 1853);s.store_mul(1841, 1899, 1900);s.store_sub(335, 1855, 1857);}
            s.b[1974] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1975] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1975, if s.b[1975] { 1.0 } else { 0.0 });s.b[1976] = (2.0 == 1.0);s.store_scalar(1976, if s.b[1976] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && s.b[1976]) {s.store_scalar(720, 1.0);}
            s.b[1977] = (2.0 == 2.0);s.store_scalar(1977, if s.b[1977] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && s.b[1977]) {s.store_scalar(720, 2.0);}
            s.b[1978] = (2.0 == 4.0);s.store_scalar(1978, if s.b[1978] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && s.b[1978]) {s.store_scalar(720, 3.0);}
            s.b[1979] = (2.0 == 8.0);s.store_scalar(1979, if s.b[1979] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && (!s.b[1978])) && s.b[1979]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {s.store_scalar(719, 0.0);}
            let mut t27: usize = 0;
            while {
                let t26: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t26 != 0.0
            } {
                t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && (!s.b[1975])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1974])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_mul(1829, 1906, 336);}
            s.b[1980] = ((s.v[1829] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1980, if s.b[1980] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {s.store_offset_sub(781, 1829, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1981] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1981, if s.b[1981] { 1.0 } else { 0.0 });s.b[1982] = (2.0 == 1.0);s.store_scalar(1982, if s.b[1982] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && s.b[1982]) {s.store_scalar(720, 1.0);}
            s.b[1983] = (2.0 == 2.0);s.store_scalar(1983, if s.b[1983] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && s.b[1983]) {s.store_scalar(720, 2.0);}
            s.b[1984] = (2.0 == 4.0);s.store_scalar(1984, if s.b[1984] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && s.b[1984]) {s.store_scalar(720, 3.0);}
            s.b[1985] = (2.0 == 8.0);s.store_scalar(1985, if s.b[1985] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && (!s.b[1984])) && s.b[1985]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {s.store_scalar(719, 0.0);}
            let mut t29: usize = 0;
            while {
                let t28: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t28 != 0.0
            } {
                t29 += 1;assert!(t29 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && (!s.b[1981])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1829, 965, (-1e-8), 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {s.store_scalar(337, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul(1866, 1829, 1903);s.store_mul_ad_product_lhs_mixed_ai(1843, A::div_from_scalar(1.034943e-10, s.ad_value(1829)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1845, A::div_from_scalar((-1.034943e-10), s.ad_value(1829)), 334, 337);}
            s.b[1986] = (p.p49 == 0.0);s.store_scalar(1986, if s.b[1986] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1986]) {s.store_add_mixed_ai(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1852);s.store_scalar(1838, 1.0);s.store_scalar(1839, 0.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1986])) {s.store_add_mixed_ia(1837, 1852, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1829), A::sub_scaled_inputs(s.ad_value(1829), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));s.store_scalar(1838, 1.0);s.store_mul_scale_offset_mixed_ai(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1829)), s.ad_value(334), (-1.0)), 1841, -1.0, 1.0);}
            s.b[1987] = ((s.v[1837] > (s.v[1850] - s.v[1848])) && (s.v[1848] >= 0.0));s.store_scalar(1987, if s.b[1987] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1850, (-1.0), 1848, 1.0);s.store_square(722, 781);s.store_square(723, 1848);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1988] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1988, if s.b[1988] { 1.0 } else { 0.0 });s.b[1989] = (4.0 == 1.0);s.store_scalar(1989, if s.b[1989] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && s.b[1989]) {s.store_scalar(720, 1.0);}
            s.b[1990] = (4.0 == 2.0);s.store_scalar(1990, if s.b[1990] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && s.b[1990]) {s.store_scalar(720, 2.0);}
            s.b[1991] = (4.0 == 4.0);s.store_scalar(1991, if s.b[1991] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && s.b[1991]) {s.store_scalar(720, 3.0);}
            s.b[1992] = (4.0 == 8.0);s.store_scalar(1992, if s.b[1992] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && (!s.b[1991])) && s.b[1992]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {s.store_scalar(719, 0.0);}
            let mut t23: usize = 0;
            while {
                let t22: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t22 != 0.0
            } {
                t23 += 1;assert!(t23 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && (!s.b[1988])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1848, 726);s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1837, 1850, 1.0, 1848, (-1.0), 780, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {s.store_scalar(334, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul(1838, 1838, 334);s.store_mul(1839, 1839, 334);s.store_add_scaled_inputs3_indices(335, 1857, 1.0, 1885, (-1.0), 1853, 1.0);}
            s.b[1993] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1993, if s.b[1993] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1994] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });s.b[1995] = (2.0 == 1.0);s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && s.b[1995]) {s.store_scalar(720, 1.0);}
            s.b[1996] = (2.0 == 2.0);s.store_scalar(1996, if s.b[1996] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && s.b[1996]) {s.store_scalar(720, 2.0);}
            s.b[1997] = (2.0 == 4.0);s.store_scalar(1997, if s.b[1997] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && s.b[1997]) {s.store_scalar(720, 3.0);}
            s.b[1998] = (2.0 == 8.0);s.store_scalar(1998, if s.b[1998] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && (!s.b[1997])) && s.b[1998]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {s.store_scalar(719, 0.0);}
            let mut t25: usize = 0;
            while {
                let t24: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t24 != 0.0
            } {
                t25 += 1;assert!(t25 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && (!s.b[1994])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1993])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_mul(1831, 1907, 336);s.store_mul_scale_offset_indices(1867, 1902, 1831, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1847, (-1.034943e-10), 1831, 337);s.store_mul_sub_rhs(335, 154, 1852, 1855);s.store_exp(336, 335);}
            s.b[1999] = (s.v[1852] >= s.v[1855]);s.store_scalar(1999, if s.b[1999] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1999]) {s.store_mul_scaled_sqrt_ad_rhs(1861, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1896, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1861, 1.0);s.store_neg(1898, 1896);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1999])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)));s.store_mul_sqrt_mixed_ia(1861, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1861, 1.0);s.store_mul_add_mixed_iaa(1896, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1898, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1861), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1852)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);s.store_sub(1869, 1896, 185);s.store_add_mixed_ia(1870, 1898, A::add_scaled_value_products(s.ad_value(1843), 1.0, s.ad_value(1845), s.ad_value(1841), 1.0, s.ad_value(1847), s.ad_value(1841), 1.0));s.store_sub(1871, 1855, 1837);s.store_neg(1872, 1838);s.store_sub_from_scalar(1873, 1.0, 1839);s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.copy_ad(1876, 1873);s.store_neg(1877, 1870);s.store_neg(1878, 1872);s.copy_ad(1879, 1869);s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);s.store_abs(335, 1880);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[2000] = (s.v[335] > 0.1);s.store_scalar(2000, if s.b[2000] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2000]) {s.store_mul_div_from_scalar_lhs_ad_indices(1880, 0.1, 335, 1880);s.store_mul_div_from_scalar_lhs_ad_indices(1881, 0.1, 335, 1881);}
            s.b[2001] = (s.v[335] < 1e-12);s.store_scalar(2001, if s.b[2001] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2001]) {s.store_scalar(79, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.store_add(1852, 1852, 1880);s.store_add(1855, 1855, 1881);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul_sub_rhs(335, 154, 1852, 1855);s.store_exp(336, 335);}
        s.b[2003] = (s.v[1852] >= s.v[1855]);s.store_scalar(2003, if s.b[2003] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2003]) {s.copy_ad(1891, 1861);s.store_scalar(1894, 0.0);s.store_scalar(1863, 0.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) {s.store_scalar(1891, 0.0);s.store_mul_sqrt_mixed_ia(1894, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[2004] = (s.v[1835] > s.v[965]);s.store_scalar(2004, if s.b[2004] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && s.b[2004]) {s.store_scalar(1863, 0.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && (!s.b[2004])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1863, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[2005] = (((s.v[1852] - s.v[1850]) < s.v[1909]) && (s.v[1909] >= 0.0));s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1852, -1.0, 1850, 1.0);s.store_square(722, 781);s.store_square(723, 1909);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2006] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });s.b[2007] = (4.0 == 1.0);s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && s.b[2007]) {s.store_scalar(720, 1.0);}
        s.b[2008] = (4.0 == 2.0);s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && s.b[2008]) {s.store_scalar(720, 2.0);}
        s.b[2009] = (4.0 == 4.0);s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && s.b[2009]) {s.store_scalar(720, 3.0);}
        s.b[2010] = (4.0 == 8.0);s.store_scalar(2010, if s.b[2010] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && (!s.b[2009])) && s.b[2010]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {s.store_scalar(719, 0.0);}
        let mut t2d: usize = 0;
        while {
            let t2c: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2c != 0.0
        } {
            t2d += 1;assert!(t2d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && (!s.b[2006])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1909, 726);s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1909, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2005])) {s.store_sub(336, 1852, 1850);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);}
    }
}
