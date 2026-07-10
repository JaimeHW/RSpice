#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[85] == 0.0) && (!s.b[1752])) {s.store_add_scaled_inputs3_indices(25, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);}
        s.b[1758] = (p.p64 == 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if s.b[1758] {s.store_scalar(280, 0.0);}
        if (!s.b[1758]) {s.store_add_scaled_inputs(1753, 315, s.v[97], 161, 1.0);}
        s.b[1759] = (s.v[1753] > s.v[314]);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((!s.b[1758]) && s.b[1759]) {s.copy_ad(1753, 314);}
        if (!s.b[1758]) {s.store_add_scaled_inputs3_indices(1754, 157, s.v[317], 161, s.v[317], 1753, (1.0 - s.v[317]));s.store_sqrt_div_from_scalar_ad(1755, (2.0 * 1.034943e-10), s.ad_value(229));s.store_scale(1756, 1755, 1.3);s.store_scaled_mul(1757, 108, 1756, 1.034943e-10);s.store_mul_add_scaled_inputs4_indices_rhs(280, 1757, 161, 1.0 / (p.p64), 157, 1.0 / (p.p64), 1754, (-1.0 / (p.p64)), 315, -1.0);}
        s.b[1760] = (p.p65 != 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);}
        s.b[1761] = (p.p24 == 1.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });s.b[1762] = (p.p43 == 1.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if (s.b[1761] && s.b[1762]) {s.store_add_scaled_inputs4_indices(471, 463, -1.0, 464, (-1.0), 467, -1.0, 468, -1.0);s.store_add(472, 466, 470);s.store_add(473, 465, 469);s.store_add_mixed_ia(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));s.store_add_mixed_ia(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));s.store_add_scaled_inputs4_indices(25, 25, 1.0, 457, s.v[451], 267, ((-1.0) * s.v[451]), 473, s.v[451]);}
        if (s.b[1761] && (!s.b[1762])) {s.store_add_mixed_ia(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));s.store_add_scaled_inputs4_indices(24, 24, 1.0, 280, s.v[451], 268, ((-1.0) * s.v[451]), 456, s.v[451]);s.store_add_scaled_inputs3_indices(25, 25, 1.0, 457, s.v[451], 267, (-s.v[451]));}
        s.b[1763] = (p.p43 == 1.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if s.b[1763] {s.store_scale(36, 281, s.v[451]);s.store_scale(35, 282, s.v[451]);s.store_scale(560, 284, s.v[451]);s.store_scale(561, 283, s.v[451]);}
        if (!s.b[1763]) {s.store_scalar(36, 0.0);s.store_scalar(35, 0.0);s.store_scalar(560, 0.0);s.store_scalar(561, 0.0);}
        s.b[1764] = (p.p25 != 1.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if s.b[1764] {s.store_scalar(557, 0.0);}
        if (!s.b[1764]) {s.store_scale(557, 263, s.v[451]);}
        s.store_scale(598, 292, s.v[451]);s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));s.store_scale(27, 27, p.p50);s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));s.store_scale(28, 28, p.p50);
        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }
        s.b[1773] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if s.b[1773] {s.store_scaled_mul(1767, 323, 108, (1e-6 * s.v[98]));s.store_scale(1768, 555, 1.0 / (s.v[451]));s.store_div_scaled_product3_indices(1769, 227, 1768, 1768, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);}
        s.b[1774] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (s.b[1773] && s.b[1774]) {s.store_div(1770, 251, 250);s.store_div_scaled_inputs2_mixed_aii(1771, A::div(s.ad_value(251), s.ad_value(293)), 1.0, 1770, (-1.0), 157, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1773] && s.b[1774]) {s.store_add_mixed_ia(1772, 1770, A::div_scaled_product(s.ad_value(1771), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));}
        if (s.b[1773] && (!s.b[1774])) {s.store_div(1772, 251, 293);}
        if s.b[1773] {s.store_mul3_affine_lhs(558, 1769, 299, s.v[451], 0.0, 1772);}
        if s.b[1773] {
            if (((-s.v[1768]) > s.v[1767]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }
        if (!s.b[1773]) {s.store_scalar(558, 0.0);}
        s.b[1775] = (p.p259 == 1.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if s.b[1775] {s.store_scalar(3, 1.0);}
        s.b[1795] = (s.v[3] == 1.0);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1795]) {s.store_scalar(1779, p.p266);s.store_scalar(1780, p.p268);s.store_scalar(1781, p.p273);s.store_scalar(1785, p.p258);s.store_scaled_voltage(1783, ctx, nodes, Some(7), Some(2), p.p50);}
        if (s.b[1775] && (!s.b[1795])) {s.store_scalar(1779, p.p265);s.store_scalar(1780, p.p267);s.store_scalar(1781, p.p272);s.store_scalar(1785, p.p257);s.store_scaled_voltage(1783, ctx, nodes, Some(0), Some(6), p.p50);}
        if s.b[1775] {s.store_primal_scale(1779, 1779, 0.0001);s.store_primal_scale(1780, 1780, 0.01);s.store_scale(1784, 429, 1.0 / (s.v[81]));s.store_powf(328, 1784, p.p269);s.store_div(1787, 1779, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1784), 0.4, 1.8), 1.0, s.ad_value(1784), s.ad_value(1784), 0.1), A::scale_offset(s.ad_value(1784), (-p.p270), p.p270));s.store_div(1788, 1780, 327);s.store_add_mixed_ia(1781, 1781, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));s.store_scalar(1776, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));s.store_scalar(1778, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));s.store_scalar(1777, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));s.store_mul(1787, 1787, 1776);s.store_offset_product3(1788, s.ad_value(1788), s.ad_value(1777), s.ad_value(1778), 1.0, 1e-50);s.store_div(1789, 1783, 1785);s.store_mul(1790, 1787, 1789);}
        s.b[1796] = (s.v[1783] >= 0.0);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1796]) {s.store_div(328, 1790, 1788);}
        if (s.b[1775] && (!s.b[1796])) {s.store_div_scaled_inputs_indices(328, 1790, -1.0, 1788, 1.0);}
        s.b[1797] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1797]) {s.store_scalar(330, 1.0);}
        s.b[1798] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((s.b[1775] && (!s.b[1797])) && s.b[1798]) {s.copy_ad(330, 328);}
        if ((s.b[1775] && (!s.b[1797])) && (!s.b[1798])) {s.store_pow_offset_rhs(330, 328, 1781, (-1.0));}
        if s.b[1775] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1799]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1775] && (!s.b[1799])) && s.b[1800]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1775] && (!s.b[1799])) && (!s.b[1800])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1781)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1775] {s.store_div_from_scalar(328, 1.6021918e-19, 1785);}
        s.b[1803] = (p.p260 == 1.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if s.b[1803] {s.store_scalar(3, 2.0);}
        s.b[1823] = (s.v[3] == 1.0);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1823]) {s.store_scalar(1807, p.p266);s.store_scalar(1808, p.p268);s.store_scalar(1809, p.p273);s.store_scalar(1813, p.p258);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1803] && s.b[1823]) {s.store_scaled_voltage(1811, ctx, nodes, Some(7), Some(2), p.p50);}
        if (s.b[1803] && (!s.b[1823])) {s.store_scalar(1807, p.p265);s.store_scalar(1808, p.p267);s.store_scalar(1809, p.p272);s.store_scalar(1813, p.p257);s.store_scaled_voltage(1811, ctx, nodes, Some(0), Some(6), p.p50);}
        if s.b[1803] {s.store_primal_scale(1807, 1807, 0.0001);s.store_primal_scale(1808, 1808, 0.01);s.store_scale(1812, 429, 1.0 / (s.v[81]));s.store_powf(328, 1812, p.p269);s.store_div(1815, 1807, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1812), 0.4, 1.8), 1.0, s.ad_value(1812), s.ad_value(1812), 0.1), A::scale_offset(s.ad_value(1812), (-p.p270), p.p270));s.store_div(1816, 1808, 327);s.store_add_mixed_ia(1809, 1809, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));s.store_scalar(1804, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));s.store_scalar(1806, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));s.store_scalar(1805, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));s.store_mul(1815, 1815, 1804);s.store_offset_product3(1816, s.ad_value(1816), s.ad_value(1805), s.ad_value(1806), 1.0, 1e-50);s.store_div(1817, 1811, 1813);s.store_mul(1818, 1815, 1817);}
        s.b[1824] = (s.v[1811] >= 0.0);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1824]) {s.store_div(328, 1818, 1816);}
        if (s.b[1803] && (!s.b[1824])) {s.store_div_scaled_inputs_indices(328, 1818, -1.0, 1816, 1.0);}
        s.b[1825] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1825]) {s.store_scalar(330, 1.0);}
        s.b[1826] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1826, if s.b[1826] { 1.0 } else { 0.0 });
        if ((s.b[1803] && (!s.b[1825])) && s.b[1826]) {s.copy_ad(330, 328);}
        if ((s.b[1803] && (!s.b[1825])) && (!s.b[1826])) {s.store_pow_offset_rhs(330, 328, 1809, (-1.0));}
        if s.b[1803] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1827]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1828, if s.b[1828] { 1.0 } else { 0.0 });
        if ((s.b[1803] && (!s.b[1827])) && s.b[1828]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1803] && (!s.b[1827])) && (!s.b[1828])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1809)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1803] {s.store_div_from_scalar(328, 1.6021918e-19, 1813);}
        s.b[1831] = (p.p43 == 1.0);s.store_scalar(1831, if s.b[1831] { 1.0 } else { 0.0 });
        if (s.b[1831] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }
        if (s.b[1831] && (s.v[85] != 0.0)) {s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);s.store_add_mixed_ai(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);s.store_add_scaled_inputs3_indices(586, 580, -1.0, 581, (-1.0), 471, 1.0);}
        if (s.b[1831] && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        if ((!s.b[1831]) && (s.v[85] != 0.0)) {s.store_add_scaled_inputs3_indices(586, 584, -1.0, 585, (-1.0), 581, -1.0);}
        if ((!s.b[1831]) && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        s.b[1836] = (s.v[613] == 1.0);s.store_scalar(1836, if s.b[1836] { 1.0 } else { 0.0 });
        if s.b[1836] {s.copy_ad(199, 9);s.copy_ad(263, 557);s.store_add(594, 23, 586);s.store_add(198, 24, 584);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
        if (!s.b[1836]) {s.store_neg(199, 9);s.store_scalar(263, 0.0);s.store_add(594, 23, 586);s.store_add(198, 25, 585);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1837] = (p.p43 == 1.0);s.store_scalar(1837, if s.b[1837] { 1.0 } else { 0.0 });
        if s.b[1837] {s.copy_ad(282, 35);s.copy_ad(284, 560);s.copy_ad(281, 36);s.copy_ad(283, 561);}
        s.b[1838] = ((p.p38 == 1.0) && (s.v[67] > 0.0));s.store_scalar(1838, if s.b[1838] { 1.0 } else { 0.0 });
        if s.b[1838] {s.copy_ad(563, 542);}
        if (!s.b[1838]) {s.store_scalar(563, 0.0);}
        s.copy_ad(9, 199);s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));s.store_scale(27, 27, p.p50);s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));s.store_scale(28, 28, p.p50);s.b[1840] = (p.p43 == 1.0);s.store_scalar(1840, if s.b[1840] { 1.0 } else { 0.0 });
        if s.b[1840] {s.store_scale(35, 282, p.p50);s.store_scale(36, 281, p.p50);}
        s.store_scale(610, 429, (4.0 * 1.3806226e-23));s.copy_ad(438, 439);s.store_mul(615, 610, 598);
        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_div(616, 558, 615);
        } else {
            s.store_scalar(616, 0.0);
        }
        if (s.v[613] > 0.0) {
            s.store_mul_scale_offset_indices(617, 616, 438, -1.0, 1.0);
        } else {
            s.store_mul(617, 616, 438);
        }
        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_scale_offset_indices(618, 616, 438, -1.0, 1.0);
        }
        s.b[1848] = ((p.p38 > 0.0) && (p.p242 > 0.0));s.store_scalar(1848, if s.b[1848] { 1.0 } else { 0.0 });s.b[1849] = (p.p43 == 1.0);s.store_scalar(1849, if s.b[1849] { 1.0 } else { 0.0 });s.b[1850] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));s.store_scalar(1850, if s.b[1850] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let eq0_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e316,) = {
    if s.b[627] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e316;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );let eq2_e319: f64 = (p.p50 * s.v[199]);let eq2_value: f64 = eq2_e319;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &s.dn[199],
            &s.db[199],
            (multiplicity) * (p.p50),
        );
        let (eq3_e325, eq3_e325_d_n0, eq3_e325_d_n1, eq3_e325_d_n2, eq3_e325_d_n3, eq3_e325_d_n4, eq3_e325_d_n5, eq3_e325_d_n6, eq3_e325_d_n7, eq3_e325_d_n8, eq3_e325_d_n9, eq3_e325_d_n10, eq3_e325_d_n11, eq3_e325_d_n12, eq3_e325_d_n13, eq3_e325_d_n14, eq3_e325_d_n15, eq3_e325_d_n16, eq3_e325_d_n17, eq3_e325_d_n18, eq3_e325_d_b0, eq3_e325_d_b1, eq3_e325_d_b2, eq3_e325_d_b3, eq3_e325_d_b4, eq3_e325_d_b5, eq3_e325_d_b6, eq3_e325_d_b7, eq3_e325_d_b8, eq3_e325_d_b9, eq3_e325_d_b10, eq3_e325_d_b11, eq3_e325_d_b12, eq3_e325_d_b13, eq3_e325_d_b14,) = {
    if s.b[1846] {
        let eq3_e323: f64 = (p.p50 * s.v[306]);
        (eq3_e323, (p.p50 * s.dn[306][0]), (p.p50 * s.dn[306][1]), (p.p50 * s.dn[306][2]), (p.p50 * s.dn[306][3]), (p.p50 * s.dn[306][4]), (p.p50 * s.dn[306][5]), (p.p50 * s.dn[306][6]), (p.p50 * s.dn[306][7]), (p.p50 * s.dn[306][8]), (p.p50 * s.dn[306][9]), (p.p50 * s.dn[306][10]), (p.p50 * s.dn[306][11]), (p.p50 * s.dn[306][12]), (p.p50 * s.dn[306][13]), (p.p50 * s.dn[306][14]), (p.p50 * s.dn[306][15]), (p.p50 * s.dn[306][16]), (p.p50 * s.dn[306][17]), (p.p50 * s.dn[306][18]), (p.p50 * s.db[306][0]), (p.p50 * s.db[306][1]), (p.p50 * s.db[306][2]), (p.p50 * s.db[306][3]), (p.p50 * s.db[306][4]), (p.p50 * s.db[306][5]), (p.p50 * s.db[306][6]), (p.p50 * s.db[306][7]), (p.p50 * s.db[306][8]), (p.p50 * s.db[306][9]), (p.p50 * s.db[306][10]), (p.p50 * s.db[306][11]), (p.p50 * s.db[306][12]), (p.p50 * s.db[306][13]), (p.p50 * s.db[306][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e325;let eq3_node_derivatives: [f64; 19] = [eq3_e325_d_n0, eq3_e325_d_n1, eq3_e325_d_n2, eq3_e325_d_n3, eq3_e325_d_n4, eq3_e325_d_n5, eq3_e325_d_n6, eq3_e325_d_n7, eq3_e325_d_n8, eq3_e325_d_n9, eq3_e325_d_n10, eq3_e325_d_n11, eq3_e325_d_n12, eq3_e325_d_n13, eq3_e325_d_n14, eq3_e325_d_n15, eq3_e325_d_n16, eq3_e325_d_n17, eq3_e325_d_n18];let eq3_branch_derivatives: [f64; 15] = [eq3_e325_d_b0, eq3_e325_d_b1, eq3_e325_d_b2, eq3_e325_d_b3, eq3_e325_d_b4, eq3_e325_d_b5, eq3_e325_d_b6, eq3_e325_d_b7, eq3_e325_d_b8, eq3_e325_d_b9, eq3_e325_d_b10, eq3_e325_d_b11, eq3_e325_d_b12, eq3_e325_d_b13, eq3_e325_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e331, eq4_e331_d_n0, eq4_e331_d_n1, eq4_e331_d_n2, eq4_e331_d_n3, eq4_e331_d_n4, eq4_e331_d_n5, eq4_e331_d_n6, eq4_e331_d_n7, eq4_e331_d_n8, eq4_e331_d_n9, eq4_e331_d_n10, eq4_e331_d_n11, eq4_e331_d_n12, eq4_e331_d_n13, eq4_e331_d_n14, eq4_e331_d_n15, eq4_e331_d_n16, eq4_e331_d_n17, eq4_e331_d_n18, eq4_e331_d_b0, eq4_e331_d_b1, eq4_e331_d_b2, eq4_e331_d_b3, eq4_e331_d_b4, eq4_e331_d_b5, eq4_e331_d_b6, eq4_e331_d_b7, eq4_e331_d_b8, eq4_e331_d_b9, eq4_e331_d_b10, eq4_e331_d_b11, eq4_e331_d_b12, eq4_e331_d_b13, eq4_e331_d_b14,) = {
    if s.b[1846] {
        let eq4_e329: f64 = (p.p50 * s.v[307]);
        (eq4_e329, (p.p50 * s.dn[307][0]), (p.p50 * s.dn[307][1]), (p.p50 * s.dn[307][2]), (p.p50 * s.dn[307][3]), (p.p50 * s.dn[307][4]), (p.p50 * s.dn[307][5]), (p.p50 * s.dn[307][6]), (p.p50 * s.dn[307][7]), (p.p50 * s.dn[307][8]), (p.p50 * s.dn[307][9]), (p.p50 * s.dn[307][10]), (p.p50 * s.dn[307][11]), (p.p50 * s.dn[307][12]), (p.p50 * s.dn[307][13]), (p.p50 * s.dn[307][14]), (p.p50 * s.dn[307][15]), (p.p50 * s.dn[307][16]), (p.p50 * s.dn[307][17]), (p.p50 * s.dn[307][18]), (p.p50 * s.db[307][0]), (p.p50 * s.db[307][1]), (p.p50 * s.db[307][2]), (p.p50 * s.db[307][3]), (p.p50 * s.db[307][4]), (p.p50 * s.db[307][5]), (p.p50 * s.db[307][6]), (p.p50 * s.db[307][7]), (p.p50 * s.db[307][8]), (p.p50 * s.db[307][9]), (p.p50 * s.db[307][10]), (p.p50 * s.db[307][11]), (p.p50 * s.db[307][12]), (p.p50 * s.db[307][13]), (p.p50 * s.db[307][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e331;let eq4_node_derivatives: [f64; 19] = [eq4_e331_d_n0, eq4_e331_d_n1, eq4_e331_d_n2, eq4_e331_d_n3, eq4_e331_d_n4, eq4_e331_d_n5, eq4_e331_d_n6, eq4_e331_d_n7, eq4_e331_d_n8, eq4_e331_d_n9, eq4_e331_d_n10, eq4_e331_d_n11, eq4_e331_d_n12, eq4_e331_d_n13, eq4_e331_d_n14, eq4_e331_d_n15, eq4_e331_d_n16, eq4_e331_d_n17, eq4_e331_d_n18];let eq4_branch_derivatives: [f64; 15] = [eq4_e331_d_b0, eq4_e331_d_b1, eq4_e331_d_b2, eq4_e331_d_b3, eq4_e331_d_b4, eq4_e331_d_b5, eq4_e331_d_b6, eq4_e331_d_b7, eq4_e331_d_b8, eq4_e331_d_b9, eq4_e331_d_b10, eq4_e331_d_b11, eq4_e331_d_b12, eq4_e331_d_b13, eq4_e331_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e337, eq5_e337_d_n0, eq5_e337_d_n1, eq5_e337_d_n2, eq5_e337_d_n3, eq5_e337_d_n4, eq5_e337_d_n5, eq5_e337_d_n6, eq5_e337_d_n7, eq5_e337_d_n8, eq5_e337_d_n9, eq5_e337_d_n10, eq5_e337_d_n11, eq5_e337_d_n12, eq5_e337_d_n13, eq5_e337_d_n14, eq5_e337_d_n15, eq5_e337_d_n16, eq5_e337_d_n17, eq5_e337_d_n18, eq5_e337_d_b0, eq5_e337_d_b1, eq5_e337_d_b2, eq5_e337_d_b3, eq5_e337_d_b4, eq5_e337_d_b5, eq5_e337_d_b6, eq5_e337_d_b7, eq5_e337_d_b8, eq5_e337_d_b9, eq5_e337_d_b10, eq5_e337_d_b11, eq5_e337_d_b12, eq5_e337_d_b13, eq5_e337_d_b14,) = {
    if s.b[1846] {
        let eq5_e335: f64 = (p.p50 * s.v[308]);
        (eq5_e335, (p.p50 * s.dn[308][0]), (p.p50 * s.dn[308][1]), (p.p50 * s.dn[308][2]), (p.p50 * s.dn[308][3]), (p.p50 * s.dn[308][4]), (p.p50 * s.dn[308][5]), (p.p50 * s.dn[308][6]), (p.p50 * s.dn[308][7]), (p.p50 * s.dn[308][8]), (p.p50 * s.dn[308][9]), (p.p50 * s.dn[308][10]), (p.p50 * s.dn[308][11]), (p.p50 * s.dn[308][12]), (p.p50 * s.dn[308][13]), (p.p50 * s.dn[308][14]), (p.p50 * s.dn[308][15]), (p.p50 * s.dn[308][16]), (p.p50 * s.dn[308][17]), (p.p50 * s.dn[308][18]), (p.p50 * s.db[308][0]), (p.p50 * s.db[308][1]), (p.p50 * s.db[308][2]), (p.p50 * s.db[308][3]), (p.p50 * s.db[308][4]), (p.p50 * s.db[308][5]), (p.p50 * s.db[308][6]), (p.p50 * s.db[308][7]), (p.p50 * s.db[308][8]), (p.p50 * s.db[308][9]), (p.p50 * s.db[308][10]), (p.p50 * s.db[308][11]), (p.p50 * s.db[308][12]), (p.p50 * s.db[308][13]), (p.p50 * s.db[308][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e337;let eq5_node_derivatives: [f64; 19] = [eq5_e337_d_n0, eq5_e337_d_n1, eq5_e337_d_n2, eq5_e337_d_n3, eq5_e337_d_n4, eq5_e337_d_n5, eq5_e337_d_n6, eq5_e337_d_n7, eq5_e337_d_n8, eq5_e337_d_n9, eq5_e337_d_n10, eq5_e337_d_n11, eq5_e337_d_n12, eq5_e337_d_n13, eq5_e337_d_n14, eq5_e337_d_n15, eq5_e337_d_n16, eq5_e337_d_n17, eq5_e337_d_n18];let eq5_branch_derivatives: [f64; 15] = [eq5_e337_d_b0, eq5_e337_d_b1, eq5_e337_d_b2, eq5_e337_d_b3, eq5_e337_d_b4, eq5_e337_d_b5, eq5_e337_d_b6, eq5_e337_d_b7, eq5_e337_d_b8, eq5_e337_d_b9, eq5_e337_d_b10, eq5_e337_d_b11, eq5_e337_d_b12, eq5_e337_d_b13, eq5_e337_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e343, eq6_e343_d_n0, eq6_e343_d_n1, eq6_e343_d_n2, eq6_e343_d_n3, eq6_e343_d_n4, eq6_e343_d_n5, eq6_e343_d_n6, eq6_e343_d_n7, eq6_e343_d_n8, eq6_e343_d_n9, eq6_e343_d_n10, eq6_e343_d_n11, eq6_e343_d_n12, eq6_e343_d_n13, eq6_e343_d_n14, eq6_e343_d_n15, eq6_e343_d_n16, eq6_e343_d_n17, eq6_e343_d_n18, eq6_e343_d_b0, eq6_e343_d_b1, eq6_e343_d_b2, eq6_e343_d_b3, eq6_e343_d_b4, eq6_e343_d_b5, eq6_e343_d_b6, eq6_e343_d_b7, eq6_e343_d_b8, eq6_e343_d_b9, eq6_e343_d_b10, eq6_e343_d_b11, eq6_e343_d_b12, eq6_e343_d_b13, eq6_e343_d_b14,) = {
    if (p.p259 != 0.0) {
        let eq6_e341: f64 = ((nv7 - nv2) / s.v[1]);let eq6_e341_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));let eq6_e341_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));let eq6_e341_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));let eq6_e341_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));let eq6_e341_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));let eq6_e341_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));let eq6_e341_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));let eq6_e341_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));let eq6_e341_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));let eq6_e341_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));let eq6_e341_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));let eq6_e341_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));let eq6_e341_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));let eq6_e341_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));let eq6_e341_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));let eq6_e341_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));let eq6_e341_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));let eq6_e341_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));let eq6_e341_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));let eq6_e341_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));let eq6_e341_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));let eq6_e341_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));let eq6_e341_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));let eq6_e341_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));let eq6_e341_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));let eq6_e341_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));let eq6_e341_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));let eq6_e341_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));let eq6_e341_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));let eq6_e341_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));let eq6_e341_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));let eq6_e341_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));let eq6_e341_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));let eq6_e341_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));
        (eq6_e341, eq6_e341_d_n0, eq6_e341_d_n1, eq6_e341_d_n2, eq6_e341_d_n3, eq6_e341_d_n4, eq6_e341_d_n5, eq6_e341_d_n6, eq6_e341_d_n7, eq6_e341_d_n8, eq6_e341_d_n9, eq6_e341_d_n10, eq6_e341_d_n11, eq6_e341_d_n12, eq6_e341_d_n13, eq6_e341_d_n14, eq6_e341_d_n15, eq6_e341_d_n16, eq6_e341_d_n17, eq6_e341_d_n18, eq6_e341_d_b0, eq6_e341_d_b1, eq6_e341_d_b2, eq6_e341_d_b3, eq6_e341_d_b4, eq6_e341_d_b5, eq6_e341_d_b6, eq6_e341_d_b7, eq6_e341_d_b8, eq6_e341_d_b9, eq6_e341_d_b10, eq6_e341_d_b11, eq6_e341_d_b12, eq6_e341_d_b13, eq6_e341_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e343;let eq6_node_derivatives: [f64; 19] = [eq6_e343_d_n0, eq6_e343_d_n1, eq6_e343_d_n2, eq6_e343_d_n3, eq6_e343_d_n4, eq6_e343_d_n5, eq6_e343_d_n6, eq6_e343_d_n7, eq6_e343_d_n8, eq6_e343_d_n9, eq6_e343_d_n10, eq6_e343_d_n11, eq6_e343_d_n12, eq6_e343_d_n13, eq6_e343_d_n14, eq6_e343_d_n15, eq6_e343_d_n16, eq6_e343_d_n17, eq6_e343_d_n18];let eq6_branch_derivatives: [f64; 15] = [eq6_e343_d_b0, eq6_e343_d_b1, eq6_e343_d_b2, eq6_e343_d_b3, eq6_e343_d_b4, eq6_e343_d_b5, eq6_e343_d_b6, eq6_e343_d_b7, eq6_e343_d_b8, eq6_e343_d_b9, eq6_e343_d_b10, eq6_e343_d_b11, eq6_e343_d_b12, eq6_e343_d_b13, eq6_e343_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e348,) = {
    if (p.p259 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        stamper.stamp_potential_const_local(
            2,
            eq7_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq8_e354, eq8_e354_d_n0, eq8_e354_d_n1, eq8_e354_d_n2, eq8_e354_d_n3, eq8_e354_d_n4, eq8_e354_d_n5, eq8_e354_d_n6, eq8_e354_d_n7, eq8_e354_d_n8, eq8_e354_d_n9, eq8_e354_d_n10, eq8_e354_d_n11, eq8_e354_d_n12, eq8_e354_d_n13, eq8_e354_d_n14, eq8_e354_d_n15, eq8_e354_d_n16, eq8_e354_d_n17, eq8_e354_d_n18, eq8_e354_d_b0, eq8_e354_d_b1, eq8_e354_d_b2, eq8_e354_d_b3, eq8_e354_d_b4, eq8_e354_d_b5, eq8_e354_d_b6, eq8_e354_d_b7, eq8_e354_d_b8, eq8_e354_d_b9, eq8_e354_d_b10, eq8_e354_d_b11, eq8_e354_d_b12, eq8_e354_d_b13, eq8_e354_d_b14,) = {
    if (p.p260 != 0.0) {
        let eq8_e352: f64 = ((nv0 - nv6) / s.v[0]);let eq8_e352_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));let eq8_e352_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));let eq8_e352_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));let eq8_e352_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));let eq8_e352_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));let eq8_e352_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));let eq8_e352_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));let eq8_e352_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));let eq8_e352_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));let eq8_e352_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));let eq8_e352_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));let eq8_e352_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));let eq8_e352_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));let eq8_e352_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));let eq8_e352_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));let eq8_e352_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));let eq8_e352_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));let eq8_e352_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));let eq8_e352_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));let eq8_e352_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));let eq8_e352_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));let eq8_e352_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));let eq8_e352_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));let eq8_e352_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));let eq8_e352_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));let eq8_e352_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));let eq8_e352_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));let eq8_e352_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));let eq8_e352_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));let eq8_e352_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));let eq8_e352_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));let eq8_e352_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));let eq8_e352_d_b13: f64 = (-(((nv0 - nv6) * s.db[0][13]) / (s.v[0] * s.v[0])));let eq8_e352_d_b14: f64 = (-(((nv0 - nv6) * s.db[0][14]) / (s.v[0] * s.v[0])));
        (eq8_e352, eq8_e352_d_n0, eq8_e352_d_n1, eq8_e352_d_n2, eq8_e352_d_n3, eq8_e352_d_n4, eq8_e352_d_n5, eq8_e352_d_n6, eq8_e352_d_n7, eq8_e352_d_n8, eq8_e352_d_n9, eq8_e352_d_n10, eq8_e352_d_n11, eq8_e352_d_n12, eq8_e352_d_n13, eq8_e352_d_n14, eq8_e352_d_n15, eq8_e352_d_n16, eq8_e352_d_n17, eq8_e352_d_n18, eq8_e352_d_b0, eq8_e352_d_b1, eq8_e352_d_b2, eq8_e352_d_b3, eq8_e352_d_b4, eq8_e352_d_b5, eq8_e352_d_b6, eq8_e352_d_b7, eq8_e352_d_b8, eq8_e352_d_b9, eq8_e352_d_b10, eq8_e352_d_b11, eq8_e352_d_b12, eq8_e352_d_b13, eq8_e352_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e354;let eq8_node_derivatives: [f64; 19] = [eq8_e354_d_n0, eq8_e354_d_n1, eq8_e354_d_n2, eq8_e354_d_n3, eq8_e354_d_n4, eq8_e354_d_n5, eq8_e354_d_n6, eq8_e354_d_n7, eq8_e354_d_n8, eq8_e354_d_n9, eq8_e354_d_n10, eq8_e354_d_n11, eq8_e354_d_n12, eq8_e354_d_n13, eq8_e354_d_n14, eq8_e354_d_n15, eq8_e354_d_n16, eq8_e354_d_n17, eq8_e354_d_n18];let eq8_branch_derivatives: [f64; 15] = [eq8_e354_d_b0, eq8_e354_d_b1, eq8_e354_d_b2, eq8_e354_d_b3, eq8_e354_d_b4, eq8_e354_d_b5, eq8_e354_d_b6, eq8_e354_d_b7, eq8_e354_d_b8, eq8_e354_d_b9, eq8_e354_d_b10, eq8_e354_d_b11, eq8_e354_d_b12, eq8_e354_d_b13, eq8_e354_d_b14];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e359,) = {
    if (p.p260 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e359;
        stamper.stamp_potential_const_local(
            3,
            eq9_value,
        );let eq10_e362: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[594]);let eq10_e363: f64 = (p.p50 * eq10_e362);let eq10_e363_d_n0: f64 = (p.p50 * (s.dn[594][0] * ddt_scale));let eq10_e363_d_n1: f64 = (p.p50 * (s.dn[594][1] * ddt_scale));let eq10_e363_d_n2: f64 = (p.p50 * (s.dn[594][2] * ddt_scale));let eq10_e363_d_n3: f64 = (p.p50 * (s.dn[594][3] * ddt_scale));let eq10_e363_d_n4: f64 = (p.p50 * (s.dn[594][4] * ddt_scale));let eq10_e363_d_n5: f64 = (p.p50 * (s.dn[594][5] * ddt_scale));let eq10_e363_d_n6: f64 = (p.p50 * (s.dn[594][6] * ddt_scale));let eq10_e363_d_n7: f64 = (p.p50 * (s.dn[594][7] * ddt_scale));let eq10_e363_d_n8: f64 = (p.p50 * (s.dn[594][8] * ddt_scale));let eq10_e363_d_n9: f64 = (p.p50 * (s.dn[594][9] * ddt_scale));let eq10_e363_d_n10: f64 = (p.p50 * (s.dn[594][10] * ddt_scale));let eq10_e363_d_n11: f64 = (p.p50 * (s.dn[594][11] * ddt_scale));let eq10_e363_d_n12: f64 = (p.p50 * (s.dn[594][12] * ddt_scale));let eq10_e363_d_n13: f64 = (p.p50 * (s.dn[594][13] * ddt_scale));let eq10_e363_d_n14: f64 = (p.p50 * (s.dn[594][14] * ddt_scale));let eq10_e363_d_n15: f64 = (p.p50 * (s.dn[594][15] * ddt_scale));let eq10_e363_d_n16: f64 = (p.p50 * (s.dn[594][16] * ddt_scale));let eq10_e363_d_n17: f64 = (p.p50 * (s.dn[594][17] * ddt_scale));let eq10_e363_d_n18: f64 = (p.p50 * (s.dn[594][18] * ddt_scale));let eq10_e363_d_b0: f64 = (p.p50 * (s.db[594][0] * ddt_scale));let eq10_e363_d_b1: f64 = (p.p50 * (s.db[594][1] * ddt_scale));let eq10_e363_d_b2: f64 = (p.p50 * (s.db[594][2] * ddt_scale));let eq10_e363_d_b3: f64 = (p.p50 * (s.db[594][3] * ddt_scale));let eq10_e363_d_b4: f64 = (p.p50 * (s.db[594][4] * ddt_scale));let eq10_e363_d_b5: f64 = (p.p50 * (s.db[594][5] * ddt_scale));let eq10_e363_d_b6: f64 = (p.p50 * (s.db[594][6] * ddt_scale));let eq10_e363_d_b7: f64 = (p.p50 * (s.db[594][7] * ddt_scale));let eq10_e363_d_b8: f64 = (p.p50 * (s.db[594][8] * ddt_scale));let eq10_e363_d_b9: f64 = (p.p50 * (s.db[594][9] * ddt_scale));let eq10_e363_d_b10: f64 = (p.p50 * (s.db[594][10] * ddt_scale));let eq10_e363_d_b11: f64 = (p.p50 * (s.db[594][11] * ddt_scale));let eq10_e363_d_b12: f64 = (p.p50 * (s.db[594][12] * ddt_scale));let eq10_e363_d_b13: f64 = (p.p50 * (s.db[594][13] * ddt_scale));let eq10_e363_d_b14: f64 = (p.p50 * (s.db[594][14] * ddt_scale));let eq10_value: f64 = eq10_e363;let eq10_node_derivatives: [f64; 19] = [eq10_e363_d_n0, eq10_e363_d_n1, eq10_e363_d_n2, eq10_e363_d_n3, eq10_e363_d_n4, eq10_e363_d_n5, eq10_e363_d_n6, eq10_e363_d_n7, eq10_e363_d_n8, eq10_e363_d_n9, eq10_e363_d_n10, eq10_e363_d_n11, eq10_e363_d_n12, eq10_e363_d_n13, eq10_e363_d_n14, eq10_e363_d_n15, eq10_e363_d_n16, eq10_e363_d_n17, eq10_e363_d_n18];let eq10_branch_derivatives: [f64; 15] = [eq10_e363_d_b0, eq10_e363_d_b1, eq10_e363_d_b2, eq10_e363_d_b3, eq10_e363_d_b4, eq10_e363_d_b5, eq10_e363_d_b6, eq10_e363_d_b7, eq10_e363_d_b8, eq10_e363_d_b9, eq10_e363_d_b10, eq10_e363_d_b11, eq10_e363_d_b12, eq10_e363_d_b13, eq10_e363_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);let eq11_e366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[198]);let eq11_e367: f64 = (p.p50 * eq11_e366);let eq11_e367_d_n0: f64 = (p.p50 * (s.dn[198][0] * ddt_scale));let eq11_e367_d_n1: f64 = (p.p50 * (s.dn[198][1] * ddt_scale));let eq11_e367_d_n2: f64 = (p.p50 * (s.dn[198][2] * ddt_scale));let eq11_e367_d_n3: f64 = (p.p50 * (s.dn[198][3] * ddt_scale));let eq11_e367_d_n4: f64 = (p.p50 * (s.dn[198][4] * ddt_scale));let eq11_e367_d_n5: f64 = (p.p50 * (s.dn[198][5] * ddt_scale));let eq11_e367_d_n6: f64 = (p.p50 * (s.dn[198][6] * ddt_scale));let eq11_e367_d_n7: f64 = (p.p50 * (s.dn[198][7] * ddt_scale));let eq11_e367_d_n8: f64 = (p.p50 * (s.dn[198][8] * ddt_scale));let eq11_e367_d_n9: f64 = (p.p50 * (s.dn[198][9] * ddt_scale));let eq11_e367_d_n10: f64 = (p.p50 * (s.dn[198][10] * ddt_scale));let eq11_e367_d_n11: f64 = (p.p50 * (s.dn[198][11] * ddt_scale));let eq11_e367_d_n12: f64 = (p.p50 * (s.dn[198][12] * ddt_scale));let eq11_e367_d_n13: f64 = (p.p50 * (s.dn[198][13] * ddt_scale));let eq11_e367_d_n14: f64 = (p.p50 * (s.dn[198][14] * ddt_scale));let eq11_e367_d_n15: f64 = (p.p50 * (s.dn[198][15] * ddt_scale));let eq11_e367_d_n16: f64 = (p.p50 * (s.dn[198][16] * ddt_scale));let eq11_e367_d_n17: f64 = (p.p50 * (s.dn[198][17] * ddt_scale));let eq11_e367_d_n18: f64 = (p.p50 * (s.dn[198][18] * ddt_scale));let eq11_e367_d_b0: f64 = (p.p50 * (s.db[198][0] * ddt_scale));let eq11_e367_d_b1: f64 = (p.p50 * (s.db[198][1] * ddt_scale));let eq11_e367_d_b2: f64 = (p.p50 * (s.db[198][2] * ddt_scale));let eq11_e367_d_b3: f64 = (p.p50 * (s.db[198][3] * ddt_scale));let eq11_e367_d_b4: f64 = (p.p50 * (s.db[198][4] * ddt_scale));let eq11_e367_d_b5: f64 = (p.p50 * (s.db[198][5] * ddt_scale));let eq11_e367_d_b6: f64 = (p.p50 * (s.db[198][6] * ddt_scale));let eq11_e367_d_b7: f64 = (p.p50 * (s.db[198][7] * ddt_scale));let eq11_e367_d_b8: f64 = (p.p50 * (s.db[198][8] * ddt_scale));let eq11_e367_d_b9: f64 = (p.p50 * (s.db[198][9] * ddt_scale));let eq11_e367_d_b10: f64 = (p.p50 * (s.db[198][10] * ddt_scale));let eq11_e367_d_b11: f64 = (p.p50 * (s.db[198][11] * ddt_scale));let eq11_e367_d_b12: f64 = (p.p50 * (s.db[198][12] * ddt_scale));let eq11_e367_d_b13: f64 = (p.p50 * (s.db[198][13] * ddt_scale));let eq11_e367_d_b14: f64 = (p.p50 * (s.db[198][14] * ddt_scale));let eq11_value: f64 = eq11_e367;let eq11_node_derivatives: [f64; 19] = [eq11_e367_d_n0, eq11_e367_d_n1, eq11_e367_d_n2, eq11_e367_d_n3, eq11_e367_d_n4, eq11_e367_d_n5, eq11_e367_d_n6, eq11_e367_d_n7, eq11_e367_d_n8, eq11_e367_d_n9, eq11_e367_d_n10, eq11_e367_d_n11, eq11_e367_d_n12, eq11_e367_d_n13, eq11_e367_d_n14, eq11_e367_d_n15, eq11_e367_d_n16, eq11_e367_d_n17, eq11_e367_d_n18];let eq11_branch_derivatives: [f64; 15] = [eq11_e367_d_b0, eq11_e367_d_b1, eq11_e367_d_b2, eq11_e367_d_b3, eq11_e367_d_b4, eq11_e367_d_b5, eq11_e367_d_b6, eq11_e367_d_b7, eq11_e367_d_b8, eq11_e367_d_b9, eq11_e367_d_b10, eq11_e367_d_b11, eq11_e367_d_b12, eq11_e367_d_b13, eq11_e367_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );let eq12_e370: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[196]);let eq12_e371: f64 = (p.p50 * eq12_e370);let eq12_e371_d_n0: f64 = (p.p50 * (s.dn[196][0] * ddt_scale));let eq12_e371_d_n1: f64 = (p.p50 * (s.dn[196][1] * ddt_scale));let eq12_e371_d_n2: f64 = (p.p50 * (s.dn[196][2] * ddt_scale));let eq12_e371_d_n3: f64 = (p.p50 * (s.dn[196][3] * ddt_scale));let eq12_e371_d_n4: f64 = (p.p50 * (s.dn[196][4] * ddt_scale));let eq12_e371_d_n5: f64 = (p.p50 * (s.dn[196][5] * ddt_scale));let eq12_e371_d_n6: f64 = (p.p50 * (s.dn[196][6] * ddt_scale));let eq12_e371_d_n7: f64 = (p.p50 * (s.dn[196][7] * ddt_scale));let eq12_e371_d_n8: f64 = (p.p50 * (s.dn[196][8] * ddt_scale));let eq12_e371_d_n9: f64 = (p.p50 * (s.dn[196][9] * ddt_scale));let eq12_e371_d_n10: f64 = (p.p50 * (s.dn[196][10] * ddt_scale));let eq12_e371_d_n11: f64 = (p.p50 * (s.dn[196][11] * ddt_scale));let eq12_e371_d_n12: f64 = (p.p50 * (s.dn[196][12] * ddt_scale));let eq12_e371_d_n13: f64 = (p.p50 * (s.dn[196][13] * ddt_scale));let eq12_e371_d_n14: f64 = (p.p50 * (s.dn[196][14] * ddt_scale));let eq12_e371_d_n15: f64 = (p.p50 * (s.dn[196][15] * ddt_scale));let eq12_e371_d_n16: f64 = (p.p50 * (s.dn[196][16] * ddt_scale));let eq12_e371_d_n17: f64 = (p.p50 * (s.dn[196][17] * ddt_scale));let eq12_e371_d_n18: f64 = (p.p50 * (s.dn[196][18] * ddt_scale));let eq12_e371_d_b0: f64 = (p.p50 * (s.db[196][0] * ddt_scale));let eq12_e371_d_b1: f64 = (p.p50 * (s.db[196][1] * ddt_scale));let eq12_e371_d_b2: f64 = (p.p50 * (s.db[196][2] * ddt_scale));let eq12_e371_d_b3: f64 = (p.p50 * (s.db[196][3] * ddt_scale));let eq12_e371_d_b4: f64 = (p.p50 * (s.db[196][4] * ddt_scale));let eq12_e371_d_b5: f64 = (p.p50 * (s.db[196][5] * ddt_scale));let eq12_e371_d_b6: f64 = (p.p50 * (s.db[196][6] * ddt_scale));let eq12_e371_d_b7: f64 = (p.p50 * (s.db[196][7] * ddt_scale));let eq12_e371_d_b8: f64 = (p.p50 * (s.db[196][8] * ddt_scale));let eq12_e371_d_b9: f64 = (p.p50 * (s.db[196][9] * ddt_scale));let eq12_e371_d_b10: f64 = (p.p50 * (s.db[196][10] * ddt_scale));let eq12_e371_d_b11: f64 = (p.p50 * (s.db[196][11] * ddt_scale));let eq12_e371_d_b12: f64 = (p.p50 * (s.db[196][12] * ddt_scale));let eq12_e371_d_b13: f64 = (p.p50 * (s.db[196][13] * ddt_scale));let eq12_e371_d_b14: f64 = (p.p50 * (s.db[196][14] * ddt_scale));let eq12_value: f64 = eq12_e371;let eq12_node_derivatives: [f64; 19] = [eq12_e371_d_n0, eq12_e371_d_n1, eq12_e371_d_n2, eq12_e371_d_n3, eq12_e371_d_n4, eq12_e371_d_n5, eq12_e371_d_n6, eq12_e371_d_n7, eq12_e371_d_n8, eq12_e371_d_n9, eq12_e371_d_n10, eq12_e371_d_n11, eq12_e371_d_n12, eq12_e371_d_n13, eq12_e371_d_n14, eq12_e371_d_n15, eq12_e371_d_n16, eq12_e371_d_n17, eq12_e371_d_n18];let eq12_branch_derivatives: [f64; 15] = [eq12_e371_d_b0, eq12_e371_d_b1, eq12_e371_d_b2, eq12_e371_d_b3, eq12_e371_d_b4, eq12_e371_d_b5, eq12_e371_d_b6, eq12_e371_d_b7, eq12_e371_d_b8, eq12_e371_d_b9, eq12_e371_d_b10, eq12_e371_d_b11, eq12_e371_d_b12, eq12_e371_d_b13, eq12_e371_d_b14];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );let eq14_e382: f64 = (nv14 - 0.0);let eq14_value: f64 = eq14_e382;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq14_value),
            14,
            multiplicity * (1.0),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);let eq17_e397: f64 = (s.v[614] * (nv14 - 0.0));let eq17_e397_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));let eq17_e397_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));let eq17_e397_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));let eq17_e397_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));let eq17_e397_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));let eq17_e397_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));let eq17_e397_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));let eq17_e397_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));let eq17_e397_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));let eq17_e397_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));let eq17_e397_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));let eq17_e397_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));let eq17_e397_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));let eq17_e397_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));let eq17_e397_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);let eq17_e397_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));let eq17_e397_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));let eq17_e397_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));let eq17_e397_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));let eq17_e397_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));let eq17_e397_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));let eq17_e397_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));let eq17_e397_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));let eq17_e397_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));let eq17_e397_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));let eq17_e397_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));let eq17_e397_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));let eq17_e397_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));let eq17_e397_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));let eq17_e397_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));let eq17_e397_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));let eq17_e397_d_b12: f64 = (s.db[614][12] * (nv14 - 0.0));let eq17_e397_d_b13: f64 = (s.db[614][13] * (nv14 - 0.0));let eq17_e397_d_b14: f64 = (s.db[614][14] * (nv14 - 0.0));let eq17_value: f64 = eq17_e397;let eq17_node_derivatives: [f64; 19] = [eq17_e397_d_n0, eq17_e397_d_n1, eq17_e397_d_n2, eq17_e397_d_n3, eq17_e397_d_n4, eq17_e397_d_n5, eq17_e397_d_n6, eq17_e397_d_n7, eq17_e397_d_n8, eq17_e397_d_n9, eq17_e397_d_n10, eq17_e397_d_n11, eq17_e397_d_n12, eq17_e397_d_n13, eq17_e397_d_n14, eq17_e397_d_n15, eq17_e397_d_n16, eq17_e397_d_n17, eq17_e397_d_n18];let eq17_branch_derivatives: [f64; 15] = [eq17_e397_d_b0, eq17_e397_d_b1, eq17_e397_d_b2, eq17_e397_d_b3, eq17_e397_d_b4, eq17_e397_d_b5, eq17_e397_d_b6, eq17_e397_d_b7, eq17_e397_d_b8, eq17_e397_d_b9, eq17_e397_d_b10, eq17_e397_d_b11, eq17_e397_d_b12, eq17_e397_d_b13, eq17_e397_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );let eq18_e400: f64 = ((nv14 - 0.0) * s.v[617]);let eq18_e400_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);let eq18_e400_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);let eq18_e400_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);let eq18_e400_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);let eq18_e400_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);let eq18_e400_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);let eq18_e400_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);let eq18_e400_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);let eq18_e400_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);let eq18_e400_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);let eq18_e400_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);let eq18_e400_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);let eq18_e400_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);let eq18_e400_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);let eq18_e400_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));let eq18_e400_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);let eq18_e400_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);let eq18_e400_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);let eq18_e400_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);let eq18_e400_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);let eq18_e400_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);let eq18_e400_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);let eq18_e400_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);let eq18_e400_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);let eq18_e400_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);let eq18_e400_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);let eq18_e400_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);let eq18_e400_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);let eq18_e400_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);let eq18_e400_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);let eq18_e400_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);let eq18_e400_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);let eq18_e400_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);let eq18_e400_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);let eq18_e401: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e400);let eq18_value: f64 = eq18_e401;let eq18_node_derivatives: [f64; 19] = [(eq18_e400_d_n0 * ddt_scale), (eq18_e400_d_n1 * ddt_scale), (eq18_e400_d_n2 * ddt_scale), (eq18_e400_d_n3 * ddt_scale), (eq18_e400_d_n4 * ddt_scale), (eq18_e400_d_n5 * ddt_scale), (eq18_e400_d_n6 * ddt_scale), (eq18_e400_d_n7 * ddt_scale), (eq18_e400_d_n8 * ddt_scale), (eq18_e400_d_n9 * ddt_scale), (eq18_e400_d_n10 * ddt_scale), (eq18_e400_d_n11 * ddt_scale), (eq18_e400_d_n12 * ddt_scale), (eq18_e400_d_n13 * ddt_scale), (eq18_e400_d_n14 * ddt_scale), (eq18_e400_d_n15 * ddt_scale), (eq18_e400_d_n16 * ddt_scale), (eq18_e400_d_n17 * ddt_scale), (eq18_e400_d_n18 * ddt_scale)];let eq18_branch_derivatives: [f64; 15] = [(eq18_e400_d_b0 * ddt_scale), (eq18_e400_d_b1 * ddt_scale), (eq18_e400_d_b2 * ddt_scale), (eq18_e400_d_b3 * ddt_scale), (eq18_e400_d_b4 * ddt_scale), (eq18_e400_d_b5 * ddt_scale), (eq18_e400_d_b6 * ddt_scale), (eq18_e400_d_b7 * ddt_scale), (eq18_e400_d_b8 * ddt_scale), (eq18_e400_d_b9 * ddt_scale), (eq18_e400_d_b10 * ddt_scale), (eq18_e400_d_b11 * ddt_scale), (eq18_e400_d_b12 * ddt_scale), (eq18_e400_d_b13 * ddt_scale), (eq18_e400_d_b14 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv11 = ctx.node_voltage(nodes[11]);let nv14 = ctx.node_voltage(nodes[14]);let eq19_e404: f64 = ((nv14 - 0.0) * s.v[618]);let eq19_e404_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);let eq19_e404_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);let eq19_e404_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);let eq19_e404_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);let eq19_e404_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);let eq19_e404_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);let eq19_e404_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);let eq19_e404_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);let eq19_e404_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);let eq19_e404_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);let eq19_e404_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);let eq19_e404_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);let eq19_e404_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);let eq19_e404_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);let eq19_e404_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));let eq19_e404_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);let eq19_e404_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);let eq19_e404_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);let eq19_e404_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);let eq19_e404_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);let eq19_e404_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);let eq19_e404_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);let eq19_e404_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);let eq19_e404_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);let eq19_e404_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);let eq19_e404_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);let eq19_e404_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);let eq19_e404_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);let eq19_e404_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);let eq19_e404_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);let eq19_e404_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);let eq19_e404_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);let eq19_e404_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);let eq19_e404_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);let eq19_e405: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e404);let eq19_value: f64 = eq19_e405;let eq19_node_derivatives: [f64; 19] = [(eq19_e404_d_n0 * ddt_scale), (eq19_e404_d_n1 * ddt_scale), (eq19_e404_d_n2 * ddt_scale), (eq19_e404_d_n3 * ddt_scale), (eq19_e404_d_n4 * ddt_scale), (eq19_e404_d_n5 * ddt_scale), (eq19_e404_d_n6 * ddt_scale), (eq19_e404_d_n7 * ddt_scale), (eq19_e404_d_n8 * ddt_scale), (eq19_e404_d_n9 * ddt_scale), (eq19_e404_d_n10 * ddt_scale), (eq19_e404_d_n11 * ddt_scale), (eq19_e404_d_n12 * ddt_scale), (eq19_e404_d_n13 * ddt_scale), (eq19_e404_d_n14 * ddt_scale), (eq19_e404_d_n15 * ddt_scale), (eq19_e404_d_n16 * ddt_scale), (eq19_e404_d_n17 * ddt_scale), (eq19_e404_d_n18 * ddt_scale)];let eq19_branch_derivatives: [f64; 15] = [(eq19_e404_d_b0 * ddt_scale), (eq19_e404_d_b1 * ddt_scale), (eq19_e404_d_b2 * ddt_scale), (eq19_e404_d_b3 * ddt_scale), (eq19_e404_d_b4 * ddt_scale), (eq19_e404_d_b5 * ddt_scale), (eq19_e404_d_b6 * ddt_scale), (eq19_e404_d_b7 * ddt_scale), (eq19_e404_d_b8 * ddt_scale), (eq19_e404_d_b9 * ddt_scale), (eq19_e404_d_b10 * ddt_scale), (eq19_e404_d_b11 * ddt_scale), (eq19_e404_d_b12 * ddt_scale), (eq19_e404_d_b13 * ddt_scale), (eq19_e404_d_b14 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e457, eq25_e457_d_n0, eq25_e457_d_n1, eq25_e457_d_n2, eq25_e457_d_n3, eq25_e457_d_n4, eq25_e457_d_n5, eq25_e457_d_n6, eq25_e457_d_n7, eq25_e457_d_n8, eq25_e457_d_n9, eq25_e457_d_n10, eq25_e457_d_n11, eq25_e457_d_n12, eq25_e457_d_n13, eq25_e457_d_n14, eq25_e457_d_n15, eq25_e457_d_n16, eq25_e457_d_n17, eq25_e457_d_n18, eq25_e457_d_b0, eq25_e457_d_b1, eq25_e457_d_b2, eq25_e457_d_b3, eq25_e457_d_b4, eq25_e457_d_b5, eq25_e457_d_b6, eq25_e457_d_b7, eq25_e457_d_b8, eq25_e457_d_b9, eq25_e457_d_b10, eq25_e457_d_b11, eq25_e457_d_b12, eq25_e457_d_b13, eq25_e457_d_b14,) = {
    if (p.p35 != 0.0) {
        let eq25_e455: f64 = (s.v[551] * (nv1 - nv11));let eq25_e455_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));let eq25_e455_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);let eq25_e455_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));let eq25_e455_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));let eq25_e455_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));let eq25_e455_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));let eq25_e455_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));let eq25_e455_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));let eq25_e455_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));let eq25_e455_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));let eq25_e455_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));let eq25_e455_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));let eq25_e455_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));let eq25_e455_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));let eq25_e455_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));let eq25_e455_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));let eq25_e455_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));let eq25_e455_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));let eq25_e455_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));let eq25_e455_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));let eq25_e455_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));let eq25_e455_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));let eq25_e455_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));let eq25_e455_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));let eq25_e455_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));let eq25_e455_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));let eq25_e455_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));let eq25_e455_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));let eq25_e455_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));let eq25_e455_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));let eq25_e455_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));let eq25_e455_d_b12: f64 = (s.db[551][12] * (nv1 - nv11));let eq25_e455_d_b13: f64 = (s.db[551][13] * (nv1 - nv11));let eq25_e455_d_b14: f64 = (s.db[551][14] * (nv1 - nv11));
        (eq25_e455, eq25_e455_d_n0, eq25_e455_d_n1, eq25_e455_d_n2, eq25_e455_d_n3, eq25_e455_d_n4, eq25_e455_d_n5, eq25_e455_d_n6, eq25_e455_d_n7, eq25_e455_d_n8, eq25_e455_d_n9, eq25_e455_d_n10, eq25_e455_d_n11, eq25_e455_d_n12, eq25_e455_d_n13, eq25_e455_d_n14, eq25_e455_d_n15, eq25_e455_d_n16, eq25_e455_d_n17, eq25_e455_d_n18, eq25_e455_d_b0, eq25_e455_d_b1, eq25_e455_d_b2, eq25_e455_d_b3, eq25_e455_d_b4, eq25_e455_d_b5, eq25_e455_d_b6, eq25_e455_d_b7, eq25_e455_d_b8, eq25_e455_d_b9, eq25_e455_d_b10, eq25_e455_d_b11, eq25_e455_d_b12, eq25_e455_d_b13, eq25_e455_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e457;let eq25_node_derivatives: [f64; 19] = [eq25_e457_d_n0, eq25_e457_d_n1, eq25_e457_d_n2, eq25_e457_d_n3, eq25_e457_d_n4, eq25_e457_d_n5, eq25_e457_d_n6, eq25_e457_d_n7, eq25_e457_d_n8, eq25_e457_d_n9, eq25_e457_d_n10, eq25_e457_d_n11, eq25_e457_d_n12, eq25_e457_d_n13, eq25_e457_d_n14, eq25_e457_d_n15, eq25_e457_d_n16, eq25_e457_d_n17, eq25_e457_d_n18];let eq25_branch_derivatives: [f64; 15] = [eq25_e457_d_b0, eq25_e457_d_b1, eq25_e457_d_b2, eq25_e457_d_b3, eq25_e457_d_b4, eq25_e457_d_b5, eq25_e457_d_b6, eq25_e457_d_b7, eq25_e457_d_b8, eq25_e457_d_b9, eq25_e457_d_b10, eq25_e457_d_b11, eq25_e457_d_b12, eq25_e457_d_b13, eq25_e457_d_b14];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e462,) = {
    if (p.p35 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e462;
        stamper.stamp_potential_const_local(
            4,
            eq26_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq27_e468, eq27_e468_d_n0, eq27_e468_d_n1, eq27_e468_d_n2, eq27_e468_d_n3, eq27_e468_d_n4, eq27_e468_d_n5, eq27_e468_d_n6, eq27_e468_d_n7, eq27_e468_d_n8, eq27_e468_d_n9, eq27_e468_d_n10, eq27_e468_d_n11, eq27_e468_d_n12, eq27_e468_d_n13, eq27_e468_d_n14, eq27_e468_d_n15, eq27_e468_d_n16, eq27_e468_d_n17, eq27_e468_d_n18, eq27_e468_d_b0, eq27_e468_d_b1, eq27_e468_d_b2, eq27_e468_d_b3, eq27_e468_d_b4, eq27_e468_d_b5, eq27_e468_d_b6, eq27_e468_d_b7, eq27_e468_d_b8, eq27_e468_d_b9, eq27_e468_d_b10, eq27_e468_d_b11, eq27_e468_d_b12, eq27_e468_d_b13, eq27_e468_d_b14,) = {
    if s.b[1848] {
        let eq27_e466: f64 = ((nv10 - 0.0) * s.v[589]);let eq27_e466_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);let eq27_e466_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);let eq27_e466_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);let eq27_e466_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);let eq27_e466_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);let eq27_e466_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);let eq27_e466_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);let eq27_e466_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);let eq27_e466_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);let eq27_e466_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);let eq27_e466_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));let eq27_e466_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);let eq27_e466_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);let eq27_e466_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);let eq27_e466_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);let eq27_e466_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);let eq27_e466_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);let eq27_e466_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);let eq27_e466_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);let eq27_e466_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);let eq27_e466_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);let eq27_e466_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);let eq27_e466_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);let eq27_e466_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);let eq27_e466_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);let eq27_e466_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);let eq27_e466_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);let eq27_e466_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);let eq27_e466_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);let eq27_e466_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);let eq27_e466_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);let eq27_e466_d_b12: f64 = ((nv10 - 0.0) * s.db[589][12]);let eq27_e466_d_b13: f64 = ((nv10 - 0.0) * s.db[589][13]);let eq27_e466_d_b14: f64 = ((nv10 - 0.0) * s.db[589][14]);
        (eq27_e466, eq27_e466_d_n0, eq27_e466_d_n1, eq27_e466_d_n2, eq27_e466_d_n3, eq27_e466_d_n4, eq27_e466_d_n5, eq27_e466_d_n6, eq27_e466_d_n7, eq27_e466_d_n8, eq27_e466_d_n9, eq27_e466_d_n10, eq27_e466_d_n11, eq27_e466_d_n12, eq27_e466_d_n13, eq27_e466_d_n14, eq27_e466_d_n15, eq27_e466_d_n16, eq27_e466_d_n17, eq27_e466_d_n18, eq27_e466_d_b0, eq27_e466_d_b1, eq27_e466_d_b2, eq27_e466_d_b3, eq27_e466_d_b4, eq27_e466_d_b5, eq27_e466_d_b6, eq27_e466_d_b7, eq27_e466_d_b8, eq27_e466_d_b9, eq27_e466_d_b10, eq27_e466_d_b11, eq27_e466_d_b12, eq27_e466_d_b13, eq27_e466_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e468;let eq27_node_derivatives: [f64; 19] = [eq27_e468_d_n0, eq27_e468_d_n1, eq27_e468_d_n2, eq27_e468_d_n3, eq27_e468_d_n4, eq27_e468_d_n5, eq27_e468_d_n6, eq27_e468_d_n7, eq27_e468_d_n8, eq27_e468_d_n9, eq27_e468_d_n10, eq27_e468_d_n11, eq27_e468_d_n12, eq27_e468_d_n13, eq27_e468_d_n14, eq27_e468_d_n15, eq27_e468_d_n16, eq27_e468_d_n17, eq27_e468_d_n18];let eq27_branch_derivatives: [f64; 15] = [eq27_e468_d_b0, eq27_e468_d_b1, eq27_e468_d_b2, eq27_e468_d_b3, eq27_e468_d_b4, eq27_e468_d_b5, eq27_e468_d_b6, eq27_e468_d_b7, eq27_e468_d_b8, eq27_e468_d_b9, eq27_e468_d_b10, eq27_e468_d_b11, eq27_e468_d_b12, eq27_e468_d_b13, eq27_e468_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e473, eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18, eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14,) = {
    if s.b[1848] {
        let eq28_e471: f64 = (-s.v[595]);
        (eq28_e471, (-s.dn[595][0]), (-s.dn[595][1]), (-s.dn[595][2]), (-s.dn[595][3]), (-s.dn[595][4]), (-s.dn[595][5]), (-s.dn[595][6]), (-s.dn[595][7]), (-s.dn[595][8]), (-s.dn[595][9]), (-s.dn[595][10]), (-s.dn[595][11]), (-s.dn[595][12]), (-s.dn[595][13]), (-s.dn[595][14]), (-s.dn[595][15]), (-s.dn[595][16]), (-s.dn[595][17]), (-s.dn[595][18]), (-s.db[595][0]), (-s.db[595][1]), (-s.db[595][2]), (-s.db[595][3]), (-s.db[595][4]), (-s.db[595][5]), (-s.db[595][6]), (-s.db[595][7]), (-s.db[595][8]), (-s.db[595][9]), (-s.db[595][10]), (-s.db[595][11]), (-s.db[595][12]), (-s.db[595][13]), (-s.db[595][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e473;let eq28_node_derivatives: [f64; 19] = [eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18];let eq28_branch_derivatives: [f64; 15] = [eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e479, eq29_e479_d_n10,) = {
    if s.b[1848] {
        let eq29_e477: f64 = ((nv10 - 0.0) * 1e-12);
        (eq29_e477, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e479;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            10,
            multiplicity * (eq29_e479_d_n10),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e486, eq30_e486_d_n0, eq30_e486_d_n1, eq30_e486_d_n2, eq30_e486_d_n3, eq30_e486_d_n4, eq30_e486_d_n5, eq30_e486_d_n6, eq30_e486_d_n7, eq30_e486_d_n8, eq30_e486_d_n9, eq30_e486_d_n10, eq30_e486_d_n11, eq30_e486_d_n12, eq30_e486_d_n13, eq30_e486_d_n14, eq30_e486_d_n15, eq30_e486_d_n16, eq30_e486_d_n17, eq30_e486_d_n18, eq30_e486_d_b0, eq30_e486_d_b1, eq30_e486_d_b2, eq30_e486_d_b3, eq30_e486_d_b4, eq30_e486_d_b5, eq30_e486_d_b6, eq30_e486_d_b7, eq30_e486_d_b8, eq30_e486_d_b9, eq30_e486_d_b10, eq30_e486_d_b11, eq30_e486_d_b12, eq30_e486_d_b13, eq30_e486_d_b14,) = {
    if s.b[1848] {
        let eq30_e483: f64 = (s.v[563] * (nv10 - 0.0));let eq30_e483_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));let eq30_e483_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));let eq30_e483_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));let eq30_e483_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));let eq30_e483_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));let eq30_e483_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));let eq30_e483_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));let eq30_e483_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));let eq30_e483_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));let eq30_e483_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));let eq30_e483_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);let eq30_e483_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));let eq30_e483_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));let eq30_e483_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));let eq30_e483_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));let eq30_e483_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));let eq30_e483_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));let eq30_e483_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));let eq30_e483_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));let eq30_e483_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));let eq30_e483_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));let eq30_e483_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));let eq30_e483_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));let eq30_e483_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));let eq30_e483_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));let eq30_e483_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));let eq30_e483_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));let eq30_e483_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));let eq30_e483_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));let eq30_e483_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));let eq30_e483_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));let eq30_e483_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));let eq30_e483_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));let eq30_e483_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));let eq30_e484: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e483);
        (eq30_e484, (eq30_e483_d_n0 * ddt_scale), (eq30_e483_d_n1 * ddt_scale), (eq30_e483_d_n2 * ddt_scale), (eq30_e483_d_n3 * ddt_scale), (eq30_e483_d_n4 * ddt_scale), (eq30_e483_d_n5 * ddt_scale), (eq30_e483_d_n6 * ddt_scale), (eq30_e483_d_n7 * ddt_scale), (eq30_e483_d_n8 * ddt_scale), (eq30_e483_d_n9 * ddt_scale), (eq30_e483_d_n10 * ddt_scale), (eq30_e483_d_n11 * ddt_scale), (eq30_e483_d_n12 * ddt_scale), (eq30_e483_d_n13 * ddt_scale), (eq30_e483_d_n14 * ddt_scale), (eq30_e483_d_n15 * ddt_scale), (eq30_e483_d_n16 * ddt_scale), (eq30_e483_d_n17 * ddt_scale), (eq30_e483_d_n18 * ddt_scale), (eq30_e483_d_b0 * ddt_scale), (eq30_e483_d_b1 * ddt_scale), (eq30_e483_d_b2 * ddt_scale), (eq30_e483_d_b3 * ddt_scale), (eq30_e483_d_b4 * ddt_scale), (eq30_e483_d_b5 * ddt_scale), (eq30_e483_d_b6 * ddt_scale), (eq30_e483_d_b7 * ddt_scale), (eq30_e483_d_b8 * ddt_scale), (eq30_e483_d_b9 * ddt_scale), (eq30_e483_d_b10 * ddt_scale), (eq30_e483_d_b11 * ddt_scale), (eq30_e483_d_b12 * ddt_scale), (eq30_e483_d_b13 * ddt_scale), (eq30_e483_d_b14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e486;let eq30_node_derivatives: [f64; 19] = [eq30_e486_d_n0, eq30_e486_d_n1, eq30_e486_d_n2, eq30_e486_d_n3, eq30_e486_d_n4, eq30_e486_d_n5, eq30_e486_d_n6, eq30_e486_d_n7, eq30_e486_d_n8, eq30_e486_d_n9, eq30_e486_d_n10, eq30_e486_d_n11, eq30_e486_d_n12, eq30_e486_d_n13, eq30_e486_d_n14, eq30_e486_d_n15, eq30_e486_d_n16, eq30_e486_d_n17, eq30_e486_d_n18];let eq30_branch_derivatives: [f64; 15] = [eq30_e486_d_b0, eq30_e486_d_b1, eq30_e486_d_b2, eq30_e486_d_b3, eq30_e486_d_b4, eq30_e486_d_b5, eq30_e486_d_b6, eq30_e486_d_b7, eq30_e486_d_b8, eq30_e486_d_b9, eq30_e486_d_b10, eq30_e486_d_b11, eq30_e486_d_b12, eq30_e486_d_b13, eq30_e486_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e493, eq31_e493_d_n10,) = {
    if (!s.b[1848]) {
        let eq31_e491: f64 = ((nv10 - 0.0) * 10000.0);
        (eq31_e491, 10000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e493;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e493_d_n10),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq32_e501, eq32_e501_d_n0, eq32_e501_d_n1, eq32_e501_d_n2, eq32_e501_d_n3, eq32_e501_d_n4, eq32_e501_d_n5, eq32_e501_d_n6, eq32_e501_d_n7, eq32_e501_d_n8, eq32_e501_d_n9, eq32_e501_d_n10, eq32_e501_d_n11, eq32_e501_d_n12, eq32_e501_d_n13, eq32_e501_d_n14, eq32_e501_d_n15, eq32_e501_d_n16, eq32_e501_d_n17, eq32_e501_d_n18, eq32_e501_d_b0, eq32_e501_d_b1, eq32_e501_d_b2, eq32_e501_d_b3, eq32_e501_d_b4, eq32_e501_d_b5, eq32_e501_d_b6, eq32_e501_d_b7, eq32_e501_d_b8, eq32_e501_d_b9, eq32_e501_d_b10, eq32_e501_d_b11, eq32_e501_d_b12, eq32_e501_d_b13, eq32_e501_d_b14,) = {
    if s.b[1849] {
        let eq32_e498: f64 = (s.v[311] + s.v[263]);let eq32_e498_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);let eq32_e498_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);let eq32_e498_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);let eq32_e498_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);let eq32_e498_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);let eq32_e498_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);let eq32_e498_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);let eq32_e498_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);let eq32_e498_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);let eq32_e498_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);let eq32_e498_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);let eq32_e498_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);let eq32_e498_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);let eq32_e498_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);let eq32_e498_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);let eq32_e498_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);let eq32_e498_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);let eq32_e498_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);let eq32_e498_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);let eq32_e498_d_b0: f64 = (s.db[311][0] + s.db[263][0]);let eq32_e498_d_b1: f64 = (s.db[311][1] + s.db[263][1]);let eq32_e498_d_b2: f64 = (s.db[311][2] + s.db[263][2]);let eq32_e498_d_b3: f64 = (s.db[311][3] + s.db[263][3]);let eq32_e498_d_b4: f64 = (s.db[311][4] + s.db[263][4]);let eq32_e498_d_b5: f64 = (s.db[311][5] + s.db[263][5]);let eq32_e498_d_b6: f64 = (s.db[311][6] + s.db[263][6]);let eq32_e498_d_b7: f64 = (s.db[311][7] + s.db[263][7]);let eq32_e498_d_b8: f64 = (s.db[311][8] + s.db[263][8]);let eq32_e498_d_b9: f64 = (s.db[311][9] + s.db[263][9]);let eq32_e498_d_b10: f64 = (s.db[311][10] + s.db[263][10]);let eq32_e498_d_b11: f64 = (s.db[311][11] + s.db[263][11]);let eq32_e498_d_b12: f64 = (s.db[311][12] + s.db[263][12]);let eq32_e498_d_b13: f64 = (s.db[311][13] + s.db[263][13]);let eq32_e498_d_b14: f64 = (s.db[311][14] + s.db[263][14]);let eq32_e499: f64 = (p.p50 * eq32_e498);let eq32_e499_d_n0: f64 = (p.p50 * eq32_e498_d_n0);let eq32_e499_d_n1: f64 = (p.p50 * eq32_e498_d_n1);let eq32_e499_d_n2: f64 = (p.p50 * eq32_e498_d_n2);let eq32_e499_d_n3: f64 = (p.p50 * eq32_e498_d_n3);let eq32_e499_d_n4: f64 = (p.p50 * eq32_e498_d_n4);let eq32_e499_d_n5: f64 = (p.p50 * eq32_e498_d_n5);let eq32_e499_d_n6: f64 = (p.p50 * eq32_e498_d_n6);let eq32_e499_d_n7: f64 = (p.p50 * eq32_e498_d_n7);let eq32_e499_d_n8: f64 = (p.p50 * eq32_e498_d_n8);let eq32_e499_d_n9: f64 = (p.p50 * eq32_e498_d_n9);let eq32_e499_d_n10: f64 = (p.p50 * eq32_e498_d_n10);let eq32_e499_d_n11: f64 = (p.p50 * eq32_e498_d_n11);let eq32_e499_d_n12: f64 = (p.p50 * eq32_e498_d_n12);let eq32_e499_d_n13: f64 = (p.p50 * eq32_e498_d_n13);let eq32_e499_d_n14: f64 = (p.p50 * eq32_e498_d_n14);let eq32_e499_d_n15: f64 = (p.p50 * eq32_e498_d_n15);let eq32_e499_d_n16: f64 = (p.p50 * eq32_e498_d_n16);let eq32_e499_d_n17: f64 = (p.p50 * eq32_e498_d_n17);let eq32_e499_d_n18: f64 = (p.p50 * eq32_e498_d_n18);let eq32_e499_d_b0: f64 = (p.p50 * eq32_e498_d_b0);let eq32_e499_d_b1: f64 = (p.p50 * eq32_e498_d_b1);let eq32_e499_d_b2: f64 = (p.p50 * eq32_e498_d_b2);let eq32_e499_d_b3: f64 = (p.p50 * eq32_e498_d_b3);let eq32_e499_d_b4: f64 = (p.p50 * eq32_e498_d_b4);let eq32_e499_d_b5: f64 = (p.p50 * eq32_e498_d_b5);let eq32_e499_d_b6: f64 = (p.p50 * eq32_e498_d_b6);let eq32_e499_d_b7: f64 = (p.p50 * eq32_e498_d_b7);let eq32_e499_d_b8: f64 = (p.p50 * eq32_e498_d_b8);let eq32_e499_d_b9: f64 = (p.p50 * eq32_e498_d_b9);let eq32_e499_d_b10: f64 = (p.p50 * eq32_e498_d_b10);let eq32_e499_d_b11: f64 = (p.p50 * eq32_e498_d_b11);let eq32_e499_d_b12: f64 = (p.p50 * eq32_e498_d_b12);let eq32_e499_d_b13: f64 = (p.p50 * eq32_e498_d_b13);let eq32_e499_d_b14: f64 = (p.p50 * eq32_e498_d_b14);
        (eq32_e499, eq32_e499_d_n0, eq32_e499_d_n1, eq32_e499_d_n2, eq32_e499_d_n3, eq32_e499_d_n4, eq32_e499_d_n5, eq32_e499_d_n6, eq32_e499_d_n7, eq32_e499_d_n8, eq32_e499_d_n9, eq32_e499_d_n10, eq32_e499_d_n11, eq32_e499_d_n12, eq32_e499_d_n13, eq32_e499_d_n14, eq32_e499_d_n15, eq32_e499_d_n16, eq32_e499_d_n17, eq32_e499_d_n18, eq32_e499_d_b0, eq32_e499_d_b1, eq32_e499_d_b2, eq32_e499_d_b3, eq32_e499_d_b4, eq32_e499_d_b5, eq32_e499_d_b6, eq32_e499_d_b7, eq32_e499_d_b8, eq32_e499_d_b9, eq32_e499_d_b10, eq32_e499_d_b11, eq32_e499_d_b12, eq32_e499_d_b13, eq32_e499_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e501;let eq32_node_derivatives: [f64; 19] = [eq32_e501_d_n0, eq32_e501_d_n1, eq32_e501_d_n2, eq32_e501_d_n3, eq32_e501_d_n4, eq32_e501_d_n5, eq32_e501_d_n6, eq32_e501_d_n7, eq32_e501_d_n8, eq32_e501_d_n9, eq32_e501_d_n10, eq32_e501_d_n11, eq32_e501_d_n12, eq32_e501_d_n13, eq32_e501_d_n14, eq32_e501_d_n15, eq32_e501_d_n16, eq32_e501_d_n17, eq32_e501_d_n18];let eq32_branch_derivatives: [f64; 15] = [eq32_e501_d_b0, eq32_e501_d_b1, eq32_e501_d_b2, eq32_e501_d_b3, eq32_e501_d_b4, eq32_e501_d_b5, eq32_e501_d_b6, eq32_e501_d_b7, eq32_e501_d_b8, eq32_e501_d_b9, eq32_e501_d_b10, eq32_e501_d_b11, eq32_e501_d_b12, eq32_e501_d_b13, eq32_e501_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq33_e509, eq33_e509_d_n0, eq33_e509_d_n1, eq33_e509_d_n2, eq33_e509_d_n3, eq33_e509_d_n4, eq33_e509_d_n5, eq33_e509_d_n6, eq33_e509_d_n7, eq33_e509_d_n8, eq33_e509_d_n9, eq33_e509_d_n10, eq33_e509_d_n11, eq33_e509_d_n12, eq33_e509_d_n13, eq33_e509_d_n14, eq33_e509_d_n15, eq33_e509_d_n16, eq33_e509_d_n17, eq33_e509_d_n18, eq33_e509_d_b0, eq33_e509_d_b1, eq33_e509_d_b2, eq33_e509_d_b3, eq33_e509_d_b4, eq33_e509_d_b5, eq33_e509_d_b6, eq33_e509_d_b7, eq33_e509_d_b8, eq33_e509_d_b9, eq33_e509_d_b10, eq33_e509_d_b11, eq33_e509_d_b12, eq33_e509_d_b13, eq33_e509_d_b14,) = {
    if s.b[1849] {
        let eq33_e506: f64 = (s.v[312] + s.v[573]);let eq33_e506_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);let eq33_e506_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);let eq33_e506_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);let eq33_e506_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);let eq33_e506_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);let eq33_e506_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);let eq33_e506_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);let eq33_e506_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);let eq33_e506_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);let eq33_e506_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);let eq33_e506_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);let eq33_e506_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);let eq33_e506_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);let eq33_e506_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);let eq33_e506_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);let eq33_e506_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);let eq33_e506_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);let eq33_e506_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);let eq33_e506_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);let eq33_e506_d_b0: f64 = (s.db[312][0] + s.db[573][0]);let eq33_e506_d_b1: f64 = (s.db[312][1] + s.db[573][1]);let eq33_e506_d_b2: f64 = (s.db[312][2] + s.db[573][2]);let eq33_e506_d_b3: f64 = (s.db[312][3] + s.db[573][3]);let eq33_e506_d_b4: f64 = (s.db[312][4] + s.db[573][4]);let eq33_e506_d_b5: f64 = (s.db[312][5] + s.db[573][5]);let eq33_e506_d_b6: f64 = (s.db[312][6] + s.db[573][6]);let eq33_e506_d_b7: f64 = (s.db[312][7] + s.db[573][7]);let eq33_e506_d_b8: f64 = (s.db[312][8] + s.db[573][8]);let eq33_e506_d_b9: f64 = (s.db[312][9] + s.db[573][9]);let eq33_e506_d_b10: f64 = (s.db[312][10] + s.db[573][10]);let eq33_e506_d_b11: f64 = (s.db[312][11] + s.db[573][11]);let eq33_e506_d_b12: f64 = (s.db[312][12] + s.db[573][12]);let eq33_e506_d_b13: f64 = (s.db[312][13] + s.db[573][13]);let eq33_e506_d_b14: f64 = (s.db[312][14] + s.db[573][14]);let eq33_e507: f64 = (p.p50 * eq33_e506);let eq33_e507_d_n0: f64 = (p.p50 * eq33_e506_d_n0);let eq33_e507_d_n1: f64 = (p.p50 * eq33_e506_d_n1);let eq33_e507_d_n2: f64 = (p.p50 * eq33_e506_d_n2);let eq33_e507_d_n3: f64 = (p.p50 * eq33_e506_d_n3);let eq33_e507_d_n4: f64 = (p.p50 * eq33_e506_d_n4);let eq33_e507_d_n5: f64 = (p.p50 * eq33_e506_d_n5);let eq33_e507_d_n6: f64 = (p.p50 * eq33_e506_d_n6);let eq33_e507_d_n7: f64 = (p.p50 * eq33_e506_d_n7);let eq33_e507_d_n8: f64 = (p.p50 * eq33_e506_d_n8);let eq33_e507_d_n9: f64 = (p.p50 * eq33_e506_d_n9);let eq33_e507_d_n10: f64 = (p.p50 * eq33_e506_d_n10);let eq33_e507_d_n11: f64 = (p.p50 * eq33_e506_d_n11);let eq33_e507_d_n12: f64 = (p.p50 * eq33_e506_d_n12);let eq33_e507_d_n13: f64 = (p.p50 * eq33_e506_d_n13);let eq33_e507_d_n14: f64 = (p.p50 * eq33_e506_d_n14);let eq33_e507_d_n15: f64 = (p.p50 * eq33_e506_d_n15);let eq33_e507_d_n16: f64 = (p.p50 * eq33_e506_d_n16);let eq33_e507_d_n17: f64 = (p.p50 * eq33_e506_d_n17);let eq33_e507_d_n18: f64 = (p.p50 * eq33_e506_d_n18);let eq33_e507_d_b0: f64 = (p.p50 * eq33_e506_d_b0);let eq33_e507_d_b1: f64 = (p.p50 * eq33_e506_d_b1);let eq33_e507_d_b2: f64 = (p.p50 * eq33_e506_d_b2);let eq33_e507_d_b3: f64 = (p.p50 * eq33_e506_d_b3);let eq33_e507_d_b4: f64 = (p.p50 * eq33_e506_d_b4);let eq33_e507_d_b5: f64 = (p.p50 * eq33_e506_d_b5);let eq33_e507_d_b6: f64 = (p.p50 * eq33_e506_d_b6);let eq33_e507_d_b7: f64 = (p.p50 * eq33_e506_d_b7);let eq33_e507_d_b8: f64 = (p.p50 * eq33_e506_d_b8);let eq33_e507_d_b9: f64 = (p.p50 * eq33_e506_d_b9);let eq33_e507_d_b10: f64 = (p.p50 * eq33_e506_d_b10);let eq33_e507_d_b11: f64 = (p.p50 * eq33_e506_d_b11);let eq33_e507_d_b12: f64 = (p.p50 * eq33_e506_d_b12);let eq33_e507_d_b13: f64 = (p.p50 * eq33_e506_d_b13);let eq33_e507_d_b14: f64 = (p.p50 * eq33_e506_d_b14);
        (eq33_e507, eq33_e507_d_n0, eq33_e507_d_n1, eq33_e507_d_n2, eq33_e507_d_n3, eq33_e507_d_n4, eq33_e507_d_n5, eq33_e507_d_n6, eq33_e507_d_n7, eq33_e507_d_n8, eq33_e507_d_n9, eq33_e507_d_n10, eq33_e507_d_n11, eq33_e507_d_n12, eq33_e507_d_n13, eq33_e507_d_n14, eq33_e507_d_n15, eq33_e507_d_n16, eq33_e507_d_n17, eq33_e507_d_n18, eq33_e507_d_b0, eq33_e507_d_b1, eq33_e507_d_b2, eq33_e507_d_b3, eq33_e507_d_b4, eq33_e507_d_b5, eq33_e507_d_b6, eq33_e507_d_b7, eq33_e507_d_b8, eq33_e507_d_b9, eq33_e507_d_b10, eq33_e507_d_b11, eq33_e507_d_b12, eq33_e507_d_b13, eq33_e507_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e509;let eq33_node_derivatives: [f64; 19] = [eq33_e509_d_n0, eq33_e509_d_n1, eq33_e509_d_n2, eq33_e509_d_n3, eq33_e509_d_n4, eq33_e509_d_n5, eq33_e509_d_n6, eq33_e509_d_n7, eq33_e509_d_n8, eq33_e509_d_n9, eq33_e509_d_n10, eq33_e509_d_n11, eq33_e509_d_n12, eq33_e509_d_n13, eq33_e509_d_n14, eq33_e509_d_n15, eq33_e509_d_n16, eq33_e509_d_n17, eq33_e509_d_n18];let eq33_branch_derivatives: [f64; 15] = [eq33_e509_d_b0, eq33_e509_d_b1, eq33_e509_d_b2, eq33_e509_d_b3, eq33_e509_d_b4, eq33_e509_d_b5, eq33_e509_d_b6, eq33_e509_d_b7, eq33_e509_d_b8, eq33_e509_d_b9, eq33_e509_d_b10, eq33_e509_d_b11, eq33_e509_d_b12, eq33_e509_d_b13, eq33_e509_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq34_e518, eq34_e518_d_n0, eq34_e518_d_n1, eq34_e518_d_n2, eq34_e518_d_n3, eq34_e518_d_n4, eq34_e518_d_n5, eq34_e518_d_n6, eq34_e518_d_n7, eq34_e518_d_n8, eq34_e518_d_n9, eq34_e518_d_n10, eq34_e518_d_n11, eq34_e518_d_n12, eq34_e518_d_n13, eq34_e518_d_n14, eq34_e518_d_n15, eq34_e518_d_n16, eq34_e518_d_n17, eq34_e518_d_n18, eq34_e518_d_b0, eq34_e518_d_b1, eq34_e518_d_b2, eq34_e518_d_b3, eq34_e518_d_b4, eq34_e518_d_b5, eq34_e518_d_b6, eq34_e518_d_b7, eq34_e518_d_b8, eq34_e518_d_b9, eq34_e518_d_b10, eq34_e518_d_b11, eq34_e518_d_b12, eq34_e518_d_b13, eq34_e518_d_b14,) = {
    if s.b[1849] {
        let eq34_e514: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[283]);let eq34_e515: f64 = (s.v[281] + eq34_e514);let eq34_e515_d_n0: f64 = (s.dn[281][0] + (s.dn[283][0] * ddt_scale));let eq34_e515_d_n1: f64 = (s.dn[281][1] + (s.dn[283][1] * ddt_scale));let eq34_e515_d_n2: f64 = (s.dn[281][2] + (s.dn[283][2] * ddt_scale));let eq34_e515_d_n3: f64 = (s.dn[281][3] + (s.dn[283][3] * ddt_scale));let eq34_e515_d_n4: f64 = (s.dn[281][4] + (s.dn[283][4] * ddt_scale));let eq34_e515_d_n5: f64 = (s.dn[281][5] + (s.dn[283][5] * ddt_scale));let eq34_e515_d_n6: f64 = (s.dn[281][6] + (s.dn[283][6] * ddt_scale));let eq34_e515_d_n7: f64 = (s.dn[281][7] + (s.dn[283][7] * ddt_scale));let eq34_e515_d_n8: f64 = (s.dn[281][8] + (s.dn[283][8] * ddt_scale));let eq34_e515_d_n9: f64 = (s.dn[281][9] + (s.dn[283][9] * ddt_scale));let eq34_e515_d_n10: f64 = (s.dn[281][10] + (s.dn[283][10] * ddt_scale));let eq34_e515_d_n11: f64 = (s.dn[281][11] + (s.dn[283][11] * ddt_scale));let eq34_e515_d_n12: f64 = (s.dn[281][12] + (s.dn[283][12] * ddt_scale));let eq34_e515_d_n13: f64 = (s.dn[281][13] + (s.dn[283][13] * ddt_scale));let eq34_e515_d_n14: f64 = (s.dn[281][14] + (s.dn[283][14] * ddt_scale));let eq34_e515_d_n15: f64 = (s.dn[281][15] + (s.dn[283][15] * ddt_scale));let eq34_e515_d_n16: f64 = (s.dn[281][16] + (s.dn[283][16] * ddt_scale));let eq34_e515_d_n17: f64 = (s.dn[281][17] + (s.dn[283][17] * ddt_scale));let eq34_e515_d_n18: f64 = (s.dn[281][18] + (s.dn[283][18] * ddt_scale));let eq34_e515_d_b0: f64 = (s.db[281][0] + (s.db[283][0] * ddt_scale));let eq34_e515_d_b1: f64 = (s.db[281][1] + (s.db[283][1] * ddt_scale));let eq34_e515_d_b2: f64 = (s.db[281][2] + (s.db[283][2] * ddt_scale));let eq34_e515_d_b3: f64 = (s.db[281][3] + (s.db[283][3] * ddt_scale));let eq34_e515_d_b4: f64 = (s.db[281][4] + (s.db[283][4] * ddt_scale));let eq34_e515_d_b5: f64 = (s.db[281][5] + (s.db[283][5] * ddt_scale));let eq34_e515_d_b6: f64 = (s.db[281][6] + (s.db[283][6] * ddt_scale));let eq34_e515_d_b7: f64 = (s.db[281][7] + (s.db[283][7] * ddt_scale));let eq34_e515_d_b8: f64 = (s.db[281][8] + (s.db[283][8] * ddt_scale));let eq34_e515_d_b9: f64 = (s.db[281][9] + (s.db[283][9] * ddt_scale));let eq34_e515_d_b10: f64 = (s.db[281][10] + (s.db[283][10] * ddt_scale));let eq34_e515_d_b11: f64 = (s.db[281][11] + (s.db[283][11] * ddt_scale));let eq34_e515_d_b12: f64 = (s.db[281][12] + (s.db[283][12] * ddt_scale));let eq34_e515_d_b13: f64 = (s.db[281][13] + (s.db[283][13] * ddt_scale));let eq34_e515_d_b14: f64 = (s.db[281][14] + (s.db[283][14] * ddt_scale));let eq34_e516: f64 = (p.p50 * eq34_e515);let eq34_e516_d_n0: f64 = (p.p50 * eq34_e515_d_n0);let eq34_e516_d_n1: f64 = (p.p50 * eq34_e515_d_n1);let eq34_e516_d_n2: f64 = (p.p50 * eq34_e515_d_n2);let eq34_e516_d_n3: f64 = (p.p50 * eq34_e515_d_n3);let eq34_e516_d_n4: f64 = (p.p50 * eq34_e515_d_n4);let eq34_e516_d_n5: f64 = (p.p50 * eq34_e515_d_n5);let eq34_e516_d_n6: f64 = (p.p50 * eq34_e515_d_n6);let eq34_e516_d_n7: f64 = (p.p50 * eq34_e515_d_n7);let eq34_e516_d_n8: f64 = (p.p50 * eq34_e515_d_n8);let eq34_e516_d_n9: f64 = (p.p50 * eq34_e515_d_n9);let eq34_e516_d_n10: f64 = (p.p50 * eq34_e515_d_n10);let eq34_e516_d_n11: f64 = (p.p50 * eq34_e515_d_n11);let eq34_e516_d_n12: f64 = (p.p50 * eq34_e515_d_n12);let eq34_e516_d_n13: f64 = (p.p50 * eq34_e515_d_n13);let eq34_e516_d_n14: f64 = (p.p50 * eq34_e515_d_n14);let eq34_e516_d_n15: f64 = (p.p50 * eq34_e515_d_n15);let eq34_e516_d_n16: f64 = (p.p50 * eq34_e515_d_n16);let eq34_e516_d_n17: f64 = (p.p50 * eq34_e515_d_n17);let eq34_e516_d_n18: f64 = (p.p50 * eq34_e515_d_n18);let eq34_e516_d_b0: f64 = (p.p50 * eq34_e515_d_b0);let eq34_e516_d_b1: f64 = (p.p50 * eq34_e515_d_b1);let eq34_e516_d_b2: f64 = (p.p50 * eq34_e515_d_b2);let eq34_e516_d_b3: f64 = (p.p50 * eq34_e515_d_b3);let eq34_e516_d_b4: f64 = (p.p50 * eq34_e515_d_b4);let eq34_e516_d_b5: f64 = (p.p50 * eq34_e515_d_b5);
        let eq34_e516_d_b6: f64 = (p.p50 * eq34_e515_d_b6);let eq34_e516_d_b7: f64 = (p.p50 * eq34_e515_d_b7);let eq34_e516_d_b8: f64 = (p.p50 * eq34_e515_d_b8);let eq34_e516_d_b9: f64 = (p.p50 * eq34_e515_d_b9);let eq34_e516_d_b10: f64 = (p.p50 * eq34_e515_d_b10);let eq34_e516_d_b11: f64 = (p.p50 * eq34_e515_d_b11);let eq34_e516_d_b12: f64 = (p.p50 * eq34_e515_d_b12);let eq34_e516_d_b13: f64 = (p.p50 * eq34_e515_d_b13);let eq34_e516_d_b14: f64 = (p.p50 * eq34_e515_d_b14);
        (eq34_e516, eq34_e516_d_n0, eq34_e516_d_n1, eq34_e516_d_n2, eq34_e516_d_n3, eq34_e516_d_n4, eq34_e516_d_n5, eq34_e516_d_n6, eq34_e516_d_n7, eq34_e516_d_n8, eq34_e516_d_n9, eq34_e516_d_n10, eq34_e516_d_n11, eq34_e516_d_n12, eq34_e516_d_n13, eq34_e516_d_n14, eq34_e516_d_n15, eq34_e516_d_n16, eq34_e516_d_n17, eq34_e516_d_n18, eq34_e516_d_b0, eq34_e516_d_b1, eq34_e516_d_b2, eq34_e516_d_b3, eq34_e516_d_b4, eq34_e516_d_b5, eq34_e516_d_b6, eq34_e516_d_b7, eq34_e516_d_b8, eq34_e516_d_b9, eq34_e516_d_b10, eq34_e516_d_b11, eq34_e516_d_b12, eq34_e516_d_b13, eq34_e516_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e518;let eq34_node_derivatives: [f64; 19] = [eq34_e518_d_n0, eq34_e518_d_n1, eq34_e518_d_n2, eq34_e518_d_n3, eq34_e518_d_n4, eq34_e518_d_n5, eq34_e518_d_n6, eq34_e518_d_n7, eq34_e518_d_n8, eq34_e518_d_n9, eq34_e518_d_n10, eq34_e518_d_n11, eq34_e518_d_n12, eq34_e518_d_n13, eq34_e518_d_n14, eq34_e518_d_n15, eq34_e518_d_n16, eq34_e518_d_n17, eq34_e518_d_n18];let eq34_branch_derivatives: [f64; 15] = [eq34_e518_d_b0, eq34_e518_d_b1, eq34_e518_d_b2, eq34_e518_d_b3, eq34_e518_d_b4, eq34_e518_d_b5, eq34_e518_d_b6, eq34_e518_d_b7, eq34_e518_d_b8, eq34_e518_d_b9, eq34_e518_d_b10, eq34_e518_d_b11, eq34_e518_d_b12, eq34_e518_d_b13, eq34_e518_d_b14];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq35_e527, eq35_e527_d_n0, eq35_e527_d_n1, eq35_e527_d_n2, eq35_e527_d_n3, eq35_e527_d_n4, eq35_e527_d_n5, eq35_e527_d_n6, eq35_e527_d_n7, eq35_e527_d_n8, eq35_e527_d_n9, eq35_e527_d_n10, eq35_e527_d_n11, eq35_e527_d_n12, eq35_e527_d_n13, eq35_e527_d_n14, eq35_e527_d_n15, eq35_e527_d_n16, eq35_e527_d_n17, eq35_e527_d_n18, eq35_e527_d_b0, eq35_e527_d_b1, eq35_e527_d_b2, eq35_e527_d_b3, eq35_e527_d_b4, eq35_e527_d_b5, eq35_e527_d_b6, eq35_e527_d_b7, eq35_e527_d_b8, eq35_e527_d_b9, eq35_e527_d_b10, eq35_e527_d_b11, eq35_e527_d_b12, eq35_e527_d_b13, eq35_e527_d_b14,) = {
    if s.b[1849] {
        let eq35_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[284]);let eq35_e524: f64 = (s.v[282] + eq35_e523);let eq35_e524_d_n0: f64 = (s.dn[282][0] + (s.dn[284][0] * ddt_scale));let eq35_e524_d_n1: f64 = (s.dn[282][1] + (s.dn[284][1] * ddt_scale));let eq35_e524_d_n2: f64 = (s.dn[282][2] + (s.dn[284][2] * ddt_scale));let eq35_e524_d_n3: f64 = (s.dn[282][3] + (s.dn[284][3] * ddt_scale));let eq35_e524_d_n4: f64 = (s.dn[282][4] + (s.dn[284][4] * ddt_scale));let eq35_e524_d_n5: f64 = (s.dn[282][5] + (s.dn[284][5] * ddt_scale));let eq35_e524_d_n6: f64 = (s.dn[282][6] + (s.dn[284][6] * ddt_scale));let eq35_e524_d_n7: f64 = (s.dn[282][7] + (s.dn[284][7] * ddt_scale));let eq35_e524_d_n8: f64 = (s.dn[282][8] + (s.dn[284][8] * ddt_scale));let eq35_e524_d_n9: f64 = (s.dn[282][9] + (s.dn[284][9] * ddt_scale));let eq35_e524_d_n10: f64 = (s.dn[282][10] + (s.dn[284][10] * ddt_scale));let eq35_e524_d_n11: f64 = (s.dn[282][11] + (s.dn[284][11] * ddt_scale));let eq35_e524_d_n12: f64 = (s.dn[282][12] + (s.dn[284][12] * ddt_scale));let eq35_e524_d_n13: f64 = (s.dn[282][13] + (s.dn[284][13] * ddt_scale));let eq35_e524_d_n14: f64 = (s.dn[282][14] + (s.dn[284][14] * ddt_scale));let eq35_e524_d_n15: f64 = (s.dn[282][15] + (s.dn[284][15] * ddt_scale));let eq35_e524_d_n16: f64 = (s.dn[282][16] + (s.dn[284][16] * ddt_scale));let eq35_e524_d_n17: f64 = (s.dn[282][17] + (s.dn[284][17] * ddt_scale));let eq35_e524_d_n18: f64 = (s.dn[282][18] + (s.dn[284][18] * ddt_scale));let eq35_e524_d_b0: f64 = (s.db[282][0] + (s.db[284][0] * ddt_scale));let eq35_e524_d_b1: f64 = (s.db[282][1] + (s.db[284][1] * ddt_scale));let eq35_e524_d_b2: f64 = (s.db[282][2] + (s.db[284][2] * ddt_scale));let eq35_e524_d_b3: f64 = (s.db[282][3] + (s.db[284][3] * ddt_scale));let eq35_e524_d_b4: f64 = (s.db[282][4] + (s.db[284][4] * ddt_scale));let eq35_e524_d_b5: f64 = (s.db[282][5] + (s.db[284][5] * ddt_scale));let eq35_e524_d_b6: f64 = (s.db[282][6] + (s.db[284][6] * ddt_scale));let eq35_e524_d_b7: f64 = (s.db[282][7] + (s.db[284][7] * ddt_scale));let eq35_e524_d_b8: f64 = (s.db[282][8] + (s.db[284][8] * ddt_scale));let eq35_e524_d_b9: f64 = (s.db[282][9] + (s.db[284][9] * ddt_scale));let eq35_e524_d_b10: f64 = (s.db[282][10] + (s.db[284][10] * ddt_scale));let eq35_e524_d_b11: f64 = (s.db[282][11] + (s.db[284][11] * ddt_scale));let eq35_e524_d_b12: f64 = (s.db[282][12] + (s.db[284][12] * ddt_scale));let eq35_e524_d_b13: f64 = (s.db[282][13] + (s.db[284][13] * ddt_scale));let eq35_e524_d_b14: f64 = (s.db[282][14] + (s.db[284][14] * ddt_scale));let eq35_e525: f64 = (p.p50 * eq35_e524);let eq35_e525_d_n0: f64 = (p.p50 * eq35_e524_d_n0);let eq35_e525_d_n1: f64 = (p.p50 * eq35_e524_d_n1);let eq35_e525_d_n2: f64 = (p.p50 * eq35_e524_d_n2);let eq35_e525_d_n3: f64 = (p.p50 * eq35_e524_d_n3);let eq35_e525_d_n4: f64 = (p.p50 * eq35_e524_d_n4);let eq35_e525_d_n5: f64 = (p.p50 * eq35_e524_d_n5);let eq35_e525_d_n6: f64 = (p.p50 * eq35_e524_d_n6);let eq35_e525_d_n7: f64 = (p.p50 * eq35_e524_d_n7);let eq35_e525_d_n8: f64 = (p.p50 * eq35_e524_d_n8);let eq35_e525_d_n9: f64 = (p.p50 * eq35_e524_d_n9);let eq35_e525_d_n10: f64 = (p.p50 * eq35_e524_d_n10);let eq35_e525_d_n11: f64 = (p.p50 * eq35_e524_d_n11);let eq35_e525_d_n12: f64 = (p.p50 * eq35_e524_d_n12);let eq35_e525_d_n13: f64 = (p.p50 * eq35_e524_d_n13);let eq35_e525_d_n14: f64 = (p.p50 * eq35_e524_d_n14);let eq35_e525_d_n15: f64 = (p.p50 * eq35_e524_d_n15);let eq35_e525_d_n16: f64 = (p.p50 * eq35_e524_d_n16);let eq35_e525_d_n17: f64 = (p.p50 * eq35_e524_d_n17);let eq35_e525_d_n18: f64 = (p.p50 * eq35_e524_d_n18);let eq35_e525_d_b0: f64 = (p.p50 * eq35_e524_d_b0);let eq35_e525_d_b1: f64 = (p.p50 * eq35_e524_d_b1);let eq35_e525_d_b2: f64 = (p.p50 * eq35_e524_d_b2);let eq35_e525_d_b3: f64 = (p.p50 * eq35_e524_d_b3);let eq35_e525_d_b4: f64 = (p.p50 * eq35_e524_d_b4);let eq35_e525_d_b5: f64 = (p.p50 * eq35_e524_d_b5);
        let eq35_e525_d_b6: f64 = (p.p50 * eq35_e524_d_b6);let eq35_e525_d_b7: f64 = (p.p50 * eq35_e524_d_b7);let eq35_e525_d_b8: f64 = (p.p50 * eq35_e524_d_b8);let eq35_e525_d_b9: f64 = (p.p50 * eq35_e524_d_b9);let eq35_e525_d_b10: f64 = (p.p50 * eq35_e524_d_b10);let eq35_e525_d_b11: f64 = (p.p50 * eq35_e524_d_b11);let eq35_e525_d_b12: f64 = (p.p50 * eq35_e524_d_b12);let eq35_e525_d_b13: f64 = (p.p50 * eq35_e524_d_b13);let eq35_e525_d_b14: f64 = (p.p50 * eq35_e524_d_b14);
        (eq35_e525, eq35_e525_d_n0, eq35_e525_d_n1, eq35_e525_d_n2, eq35_e525_d_n3, eq35_e525_d_n4, eq35_e525_d_n5, eq35_e525_d_n6, eq35_e525_d_n7, eq35_e525_d_n8, eq35_e525_d_n9, eq35_e525_d_n10, eq35_e525_d_n11, eq35_e525_d_n12, eq35_e525_d_n13, eq35_e525_d_n14, eq35_e525_d_n15, eq35_e525_d_n16, eq35_e525_d_n17, eq35_e525_d_n18, eq35_e525_d_b0, eq35_e525_d_b1, eq35_e525_d_b2, eq35_e525_d_b3, eq35_e525_d_b4, eq35_e525_d_b5, eq35_e525_d_b6, eq35_e525_d_b7, eq35_e525_d_b8, eq35_e525_d_b9, eq35_e525_d_b10, eq35_e525_d_b11, eq35_e525_d_b12, eq35_e525_d_b13, eq35_e525_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e527;let eq35_node_derivatives: [f64; 19] = [eq35_e527_d_n0, eq35_e527_d_n1, eq35_e527_d_n2, eq35_e527_d_n3, eq35_e527_d_n4, eq35_e527_d_n5, eq35_e527_d_n6, eq35_e527_d_n7, eq35_e527_d_n8, eq35_e527_d_n9, eq35_e527_d_n10, eq35_e527_d_n11, eq35_e527_d_n12, eq35_e527_d_n13, eq35_e527_d_n14, eq35_e527_d_n15, eq35_e527_d_n16, eq35_e527_d_n17, eq35_e527_d_n18];let eq35_branch_derivatives: [f64; 15] = [eq35_e527_d_b0, eq35_e527_d_b1, eq35_e527_d_b2, eq35_e527_d_b3, eq35_e527_d_b4, eq35_e527_d_b5, eq35_e527_d_b6, eq35_e527_d_b7, eq35_e527_d_b8, eq35_e527_d_b9, eq35_e527_d_b10, eq35_e527_d_b11, eq35_e527_d_b12, eq35_e527_d_b13, eq35_e527_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv9 = ctx.node_voltage(nodes[9]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq36_e535, eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18, eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14,) = {
    if (s.b[1849] && (p.p261 != 0.0)) {
        let eq36_e533: f64 = ((nv4 - nv12) / s.v[2]);let eq36_e533_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));let eq36_e533_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));let eq36_e533_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));let eq36_e533_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));let eq36_e533_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));let eq36_e533_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));let eq36_e533_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));let eq36_e533_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));let eq36_e533_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));let eq36_e533_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));let eq36_e533_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));let eq36_e533_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));let eq36_e533_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));let eq36_e533_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));let eq36_e533_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));let eq36_e533_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));let eq36_e533_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));let eq36_e533_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));let eq36_e533_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));let eq36_e533_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));let eq36_e533_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));let eq36_e533_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));let eq36_e533_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));let eq36_e533_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));let eq36_e533_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));let eq36_e533_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));let eq36_e533_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));let eq36_e533_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));let eq36_e533_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));let eq36_e533_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));let eq36_e533_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));let eq36_e533_d_b12: f64 = (-(((nv4 - nv12) * s.db[2][12]) / (s.v[2] * s.v[2])));let eq36_e533_d_b13: f64 = (-(((nv4 - nv12) * s.db[2][13]) / (s.v[2] * s.v[2])));let eq36_e533_d_b14: f64 = (-(((nv4 - nv12) * s.db[2][14]) / (s.v[2] * s.v[2])));
        (eq36_e533, eq36_e533_d_n0, eq36_e533_d_n1, eq36_e533_d_n2, eq36_e533_d_n3, eq36_e533_d_n4, eq36_e533_d_n5, eq36_e533_d_n6, eq36_e533_d_n7, eq36_e533_d_n8, eq36_e533_d_n9, eq36_e533_d_n10, eq36_e533_d_n11, eq36_e533_d_n12, eq36_e533_d_n13, eq36_e533_d_n14, eq36_e533_d_n15, eq36_e533_d_n16, eq36_e533_d_n17, eq36_e533_d_n18, eq36_e533_d_b0, eq36_e533_d_b1, eq36_e533_d_b2, eq36_e533_d_b3, eq36_e533_d_b4, eq36_e533_d_b5, eq36_e533_d_b6, eq36_e533_d_b7, eq36_e533_d_b8, eq36_e533_d_b9, eq36_e533_d_b10, eq36_e533_d_b11, eq36_e533_d_b12, eq36_e533_d_b13, eq36_e533_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e535;let eq36_node_derivatives: [f64; 19] = [eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18];let eq36_branch_derivatives: [f64; 15] = [eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e542,) = {
    if (s.b[1849] && (p.p261 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e542;
        stamper.stamp_potential_const_local(
            5,
            eq37_value,
        );
        let (eq38_e550, eq38_e550_d_n0, eq38_e550_d_n1, eq38_e550_d_n2, eq38_e550_d_n3, eq38_e550_d_n4, eq38_e550_d_n5, eq38_e550_d_n6, eq38_e550_d_n7, eq38_e550_d_n8, eq38_e550_d_n9, eq38_e550_d_n10, eq38_e550_d_n11, eq38_e550_d_n12, eq38_e550_d_n13, eq38_e550_d_n14, eq38_e550_d_n15, eq38_e550_d_n16, eq38_e550_d_n17, eq38_e550_d_n18, eq38_e550_d_b0, eq38_e550_d_b1, eq38_e550_d_b2, eq38_e550_d_b3, eq38_e550_d_b4, eq38_e550_d_b5, eq38_e550_d_b6, eq38_e550_d_b7, eq38_e550_d_b8, eq38_e550_d_b9, eq38_e550_d_b10, eq38_e550_d_b11, eq38_e550_d_b12, eq38_e550_d_b13, eq38_e550_d_b14,) = {
    if (s.b[1849] && (p.p262 != 0.0)) {
        let eq38_e548: f64 = (s.v[553] * (nv9 - nv12));let eq38_e548_d_n0: f64 = (s.dn[553][0] * (nv9 - nv12));let eq38_e548_d_n1: f64 = (s.dn[553][1] * (nv9 - nv12));let eq38_e548_d_n2: f64 = (s.dn[553][2] * (nv9 - nv12));let eq38_e548_d_n3: f64 = (s.dn[553][3] * (nv9 - nv12));let eq38_e548_d_n4: f64 = (s.dn[553][4] * (nv9 - nv12));let eq38_e548_d_n5: f64 = (s.dn[553][5] * (nv9 - nv12));let eq38_e548_d_n6: f64 = (s.dn[553][6] * (nv9 - nv12));let eq38_e548_d_n7: f64 = (s.dn[553][7] * (nv9 - nv12));let eq38_e548_d_n8: f64 = (s.dn[553][8] * (nv9 - nv12));let eq38_e548_d_n9: f64 = ((s.dn[553][9] * (nv9 - nv12)) + s.v[553]);let eq38_e548_d_n10: f64 = (s.dn[553][10] * (nv9 - nv12));let eq38_e548_d_n11: f64 = (s.dn[553][11] * (nv9 - nv12));let eq38_e548_d_n12: f64 = ((s.dn[553][12] * (nv9 - nv12)) + (-s.v[553]));let eq38_e548_d_n13: f64 = (s.dn[553][13] * (nv9 - nv12));let eq38_e548_d_n14: f64 = (s.dn[553][14] * (nv9 - nv12));let eq38_e548_d_n15: f64 = (s.dn[553][15] * (nv9 - nv12));let eq38_e548_d_n16: f64 = (s.dn[553][16] * (nv9 - nv12));let eq38_e548_d_n17: f64 = (s.dn[553][17] * (nv9 - nv12));let eq38_e548_d_n18: f64 = (s.dn[553][18] * (nv9 - nv12));let eq38_e548_d_b0: f64 = (s.db[553][0] * (nv9 - nv12));let eq38_e548_d_b1: f64 = (s.db[553][1] * (nv9 - nv12));let eq38_e548_d_b2: f64 = (s.db[553][2] * (nv9 - nv12));let eq38_e548_d_b3: f64 = (s.db[553][3] * (nv9 - nv12));let eq38_e548_d_b4: f64 = (s.db[553][4] * (nv9 - nv12));let eq38_e548_d_b5: f64 = (s.db[553][5] * (nv9 - nv12));let eq38_e548_d_b6: f64 = (s.db[553][6] * (nv9 - nv12));let eq38_e548_d_b7: f64 = (s.db[553][7] * (nv9 - nv12));let eq38_e548_d_b8: f64 = (s.db[553][8] * (nv9 - nv12));let eq38_e548_d_b9: f64 = (s.db[553][9] * (nv9 - nv12));let eq38_e548_d_b10: f64 = (s.db[553][10] * (nv9 - nv12));let eq38_e548_d_b11: f64 = (s.db[553][11] * (nv9 - nv12));let eq38_e548_d_b12: f64 = (s.db[553][12] * (nv9 - nv12));let eq38_e548_d_b13: f64 = (s.db[553][13] * (nv9 - nv12));let eq38_e548_d_b14: f64 = (s.db[553][14] * (nv9 - nv12));
        (eq38_e548, eq38_e548_d_n0, eq38_e548_d_n1, eq38_e548_d_n2, eq38_e548_d_n3, eq38_e548_d_n4, eq38_e548_d_n5, eq38_e548_d_n6, eq38_e548_d_n7, eq38_e548_d_n8, eq38_e548_d_n9, eq38_e548_d_n10, eq38_e548_d_n11, eq38_e548_d_n12, eq38_e548_d_n13, eq38_e548_d_n14, eq38_e548_d_n15, eq38_e548_d_n16, eq38_e548_d_n17, eq38_e548_d_n18, eq38_e548_d_b0, eq38_e548_d_b1, eq38_e548_d_b2, eq38_e548_d_b3, eq38_e548_d_b4, eq38_e548_d_b5, eq38_e548_d_b6, eq38_e548_d_b7, eq38_e548_d_b8, eq38_e548_d_b9, eq38_e548_d_b10, eq38_e548_d_b11, eq38_e548_d_b12, eq38_e548_d_b13, eq38_e548_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e550;let eq38_node_derivatives: [f64; 19] = [eq38_e550_d_n0, eq38_e550_d_n1, eq38_e550_d_n2, eq38_e550_d_n3, eq38_e550_d_n4, eq38_e550_d_n5, eq38_e550_d_n6, eq38_e550_d_n7, eq38_e550_d_n8, eq38_e550_d_n9, eq38_e550_d_n10, eq38_e550_d_n11, eq38_e550_d_n12, eq38_e550_d_n13, eq38_e550_d_n14, eq38_e550_d_n15, eq38_e550_d_n16, eq38_e550_d_n17, eq38_e550_d_n18];let eq38_branch_derivatives: [f64; 15] = [eq38_e550_d_b0, eq38_e550_d_b1, eq38_e550_d_b2, eq38_e550_d_b3, eq38_e550_d_b4, eq38_e550_d_b5, eq38_e550_d_b6, eq38_e550_d_b7, eq38_e550_d_b8, eq38_e550_d_b9, eq38_e550_d_b10, eq38_e550_d_b11, eq38_e550_d_b12, eq38_e550_d_b13, eq38_e550_d_b14];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(12),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
    }
}
