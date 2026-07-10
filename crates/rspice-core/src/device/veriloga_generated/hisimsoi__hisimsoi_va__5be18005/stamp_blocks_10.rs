#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[85] == 0.0) && (!s.b[1754])) {s.store_add_scaled_inputs3_indices(25, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);}
        s.b[1760] = (p.p64 == 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_scalar(280, 0.0);}
        if (!s.b[1760]) {s.store_add_scaled_inputs(1755, 315, s.v[97], 161, 1.0);}
        s.b[1761] = (s.v[1755] > s.v[314]);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((!s.b[1760]) && s.b[1761]) {s.copy_ad(1755, 314);}
        if (!s.b[1760]) {s.store_add_scaled_inputs3_indices(1756, 157, s.v[317], 161, s.v[317], 1755, (1.0 - s.v[317]));s.store_sqrt_div_from_scalar_ad(1757, (2.0 * 1.034943e-10), s.ad_value(229));s.store_scale(1758, 1757, 1.3);s.store_scaled_mul(1759, 108, 1758, 1.034943e-10);s.store_mul_add_scaled_inputs4_indices_rhs(280, 1759, 161, 1.0 / (p.p64), 157, 1.0 / (p.p64), 1756, (-1.0 / (p.p64)), 315, -1.0);}
        s.b[1762] = (p.p65 != 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if s.b[1762] {s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);}
        s.b[1763] = (p.p24 == 1.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = (p.p43 == 1.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if (s.b[1763] && s.b[1764]) {s.store_add_scaled_inputs4_indices(471, 463, -1.0, 464, (-1.0), 467, -1.0, 468, -1.0);s.store_add(472, 466, 470);s.store_add(473, 465, 469);s.store_add_mixed_ia(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));s.store_add_mixed_ia(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));s.store_add_scaled_inputs4_indices(25, 25, 1.0, 457, s.v[451], 267, ((-1.0) * s.v[451]), 473, s.v[451]);}
        if (s.b[1763] && (!s.b[1764])) {s.store_add_mixed_ia(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));s.store_add_scaled_inputs4_indices(24, 24, 1.0, 280, s.v[451], 268, ((-1.0) * s.v[451]), 456, s.v[451]);s.store_add_scaled_inputs3_indices(25, 25, 1.0, 457, s.v[451], 267, (-s.v[451]));}
        s.b[1765] = (p.p43 == 1.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if s.b[1765] {s.store_scale(36, 281, s.v[451]);s.store_scale(35, 282, s.v[451]);s.store_scale(560, 284, s.v[451]);s.store_scale(561, 283, s.v[451]);}
        if (!s.b[1765]) {s.store_scalar(36, 0.0);s.store_scalar(35, 0.0);s.store_scalar(560, 0.0);s.store_scalar(561, 0.0);}
        s.b[1766] = (p.p25 != 1.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if s.b[1766] {s.store_scalar(557, 0.0);}
        if (!s.b[1766]) {s.store_scale(557, 263, s.v[451]);}
        s.store_scale(598, 292, s.v[451]);s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));s.store_scale(27, 27, p.p50);s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));s.store_scale(28, 28, p.p50);
        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }
        s.b[1775] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if s.b[1775] {s.store_scaled_mul(1769, 323, 108, (1e-6 * s.v[98]));s.store_scale(1770, 555, 1.0 / (s.v[451]));s.store_div_scaled_product3_indices(1771, 227, 1770, 1770, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);}
        s.b[1776] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1776]) {s.store_div(1772, 251, 250);s.store_div_scaled_inputs2_mixed_aii(1773, A::div(s.ad_value(251), s.ad_value(293)), 1.0, 1772, (-1.0), 157, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1775] && s.b[1776]) {s.store_add_mixed_ia(1774, 1772, A::div_scaled_product(s.ad_value(1773), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));}
        if (s.b[1775] && (!s.b[1776])) {s.store_div(1774, 251, 293);}
        if s.b[1775] {s.store_mul3_affine_lhs(558, 1771, 299, s.v[451], 0.0, 1774);}
        if s.b[1775] {
            if (((-s.v[1770]) > s.v[1769]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }
        if (!s.b[1775]) {s.store_scalar(558, 0.0);}
        s.b[1777] = (p.p259 == 1.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if s.b[1777] {s.store_scalar(3, 1.0);}
        s.b[1797] = (s.v[3] == 1.0);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if (s.b[1777] && s.b[1797]) {s.store_scalar(1781, p.p266);s.store_scalar(1782, p.p268);s.store_scalar(1783, p.p273);s.store_scalar(1787, p.p258);s.store_scaled_voltage(1785, ctx, nodes, Some(7), Some(2), p.p50);}
        if (s.b[1777] && (!s.b[1797])) {s.store_scalar(1781, p.p265);s.store_scalar(1782, p.p267);s.store_scalar(1783, p.p272);s.store_scalar(1787, p.p257);s.store_scaled_voltage(1785, ctx, nodes, Some(0), Some(6), p.p50);}
        if s.b[1777] {s.store_primal_scale(1781, 1781, 0.0001);s.store_primal_scale(1782, 1782, 0.01);s.store_scale(1786, 429, 1.0 / (s.v[81]));s.store_powf(328, 1786, p.p269);s.store_div(1789, 1781, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1786), 0.4, 1.8), 1.0, s.ad_value(1786), s.ad_value(1786), 0.1), A::scale_offset(s.ad_value(1786), (-p.p270), p.p270));s.store_div(1790, 1782, 327);s.store_add_mixed_ia(1783, 1783, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));s.store_scalar(1778, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));s.store_scalar(1780, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));s.store_scalar(1779, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));s.store_mul(1789, 1789, 1778);s.store_offset_product3(1790, s.ad_value(1790), s.ad_value(1779), s.ad_value(1780), 1.0, 1e-50);s.store_div(1791, 1785, 1787);s.store_mul(1792, 1789, 1791);}
        s.b[1798] = (s.v[1785] >= 0.0);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if (s.b[1777] && s.b[1798]) {s.store_div(328, 1792, 1790);}
        if (s.b[1777] && (!s.b[1798])) {s.store_div_scaled_inputs_indices(328, 1792, -1.0, 1790, 1.0);}
        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (s.b[1777] && s.b[1799]) {s.store_scalar(330, 1.0);}
        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1777] && (!s.b[1799])) && s.b[1800]) {s.copy_ad(330, 328);}
        if ((s.b[1777] && (!s.b[1799])) && (!s.b[1800])) {s.store_pow_offset_rhs(330, 328, 1783, (-1.0));}
        if s.b[1777] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1801] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if (s.b[1777] && s.b[1801]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1802] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1777] && (!s.b[1801])) && s.b[1802]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1777] && (!s.b[1801])) && (!s.b[1802])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1783)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1777] {s.store_div_from_scalar(328, 1.6021918e-19, 1787);}
        s.b[1805] = (p.p260 == 1.0);s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if s.b[1805] {s.store_scalar(3, 2.0);}
        s.b[1825] = (s.v[3] == 1.0);s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if (s.b[1805] && s.b[1825]) {s.store_scalar(1809, p.p266);s.store_scalar(1810, p.p268);s.store_scalar(1811, p.p273);s.store_scalar(1815, p.p258);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1805] && s.b[1825]) {s.store_scaled_voltage(1813, ctx, nodes, Some(7), Some(2), p.p50);}
        if (s.b[1805] && (!s.b[1825])) {s.store_scalar(1809, p.p265);s.store_scalar(1810, p.p267);s.store_scalar(1811, p.p272);s.store_scalar(1815, p.p257);s.store_scaled_voltage(1813, ctx, nodes, Some(0), Some(6), p.p50);}
        if s.b[1805] {s.store_primal_scale(1809, 1809, 0.0001);s.store_primal_scale(1810, 1810, 0.01);s.store_scale(1814, 429, 1.0 / (s.v[81]));s.store_powf(328, 1814, p.p269);s.store_div(1817, 1809, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1814), 0.4, 1.8), 1.0, s.ad_value(1814), s.ad_value(1814), 0.1), A::scale_offset(s.ad_value(1814), (-p.p270), p.p270));s.store_div(1818, 1810, 327);s.store_add_mixed_ia(1811, 1811, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));s.store_scalar(1806, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));s.store_scalar(1808, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));s.store_scalar(1807, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));s.store_mul(1817, 1817, 1806);s.store_offset_product3(1818, s.ad_value(1818), s.ad_value(1807), s.ad_value(1808), 1.0, 1e-50);s.store_div(1819, 1813, 1815);s.store_mul(1820, 1817, 1819);}
        s.b[1826] = (s.v[1813] >= 0.0);s.store_scalar(1826, if s.b[1826] { 1.0 } else { 0.0 });
        if (s.b[1805] && s.b[1826]) {s.store_div(328, 1820, 1818);}
        if (s.b[1805] && (!s.b[1826])) {s.store_div_scaled_inputs_indices(328, 1820, -1.0, 1818, 1.0);}
        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if (s.b[1805] && s.b[1827]) {s.store_scalar(330, 1.0);}
        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1828, if s.b[1828] { 1.0 } else { 0.0 });
        if ((s.b[1805] && (!s.b[1827])) && s.b[1828]) {s.copy_ad(330, 328);}
        if ((s.b[1805] && (!s.b[1827])) && (!s.b[1828])) {s.store_pow_offset_rhs(330, 328, 1811, (-1.0));}
        if s.b[1805] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1829] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1829, if s.b[1829] { 1.0 } else { 0.0 });
        if (s.b[1805] && s.b[1829]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1830] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1830, if s.b[1830] { 1.0 } else { 0.0 });
        if ((s.b[1805] && (!s.b[1829])) && s.b[1830]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1805] && (!s.b[1829])) && (!s.b[1830])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1811)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1805] {s.store_div_from_scalar(328, 1.6021918e-19, 1815);}
        s.b[1833] = (p.p43 == 1.0);s.store_scalar(1833, if s.b[1833] { 1.0 } else { 0.0 });
        if (s.b[1833] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }
        if (s.b[1833] && (s.v[85] != 0.0)) {s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);s.store_add_mixed_ai(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);s.store_add_scaled_inputs3_indices(586, 580, -1.0, 581, (-1.0), 471, 1.0);}
        if (s.b[1833] && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        if ((!s.b[1833]) && (s.v[85] != 0.0)) {s.store_add_scaled_inputs3_indices(586, 584, -1.0, 585, (-1.0), 581, -1.0);}
        if ((!s.b[1833]) && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        s.b[1838] = (s.v[613] == 1.0);s.store_scalar(1838, if s.b[1838] { 1.0 } else { 0.0 });
        if s.b[1838] {s.copy_ad(199, 9);s.copy_ad(263, 557);s.store_add(594, 23, 586);s.store_add(198, 24, 584);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
        if (!s.b[1838]) {s.store_neg(199, 9);s.store_scalar(263, 0.0);s.store_add(594, 23, 586);s.store_add(198, 25, 585);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1839] = (p.p43 == 1.0);s.store_scalar(1839, if s.b[1839] { 1.0 } else { 0.0 });
        if s.b[1839] {s.copy_ad(282, 35);s.copy_ad(284, 560);s.copy_ad(281, 36);s.copy_ad(283, 561);}
        s.b[1840] = ((p.p38 == 1.0) && (s.v[67] > 0.0));s.store_scalar(1840, if s.b[1840] { 1.0 } else { 0.0 });
        if s.b[1840] {s.copy_ad(563, 542);}
        if (!s.b[1840]) {s.store_scalar(563, 0.0);}
        s.copy_ad(9, 199);s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));s.store_scale(27, 27, p.p50);s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));s.store_scale(28, 28, p.p50);s.b[1842] = (p.p43 == 1.0);s.store_scalar(1842, if s.b[1842] { 1.0 } else { 0.0 });
        if s.b[1842] {s.store_scale(35, 282, p.p50);s.store_scale(36, 281, p.p50);}
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
        s.b[1850] = ((p.p38 > 0.0) && (p.p242 > 0.0));s.store_scalar(1850, if s.b[1850] { 1.0 } else { 0.0 });s.b[1851] = (p.p43 == 1.0);s.store_scalar(1851, if s.b[1851] { 1.0 } else { 0.0 });s.b[1852] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));s.store_scalar(1852, if s.b[1852] { 1.0 } else { 0.0 });
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq0_e315,) = {
    if s.b[625] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e315;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e320,) = {
    if (!s.b[625]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e320;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e324,) = {
    if s.b[629] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e324;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );let eq3_e327: f64 = (p.p50 * s.v[199]);let eq3_value: f64 = eq3_e327;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq3_value),
            &s.dn[199],
            &s.db[199],
            (multiplicity) * (p.p50),
        );
        let (eq4_e333, eq4_e333_d_n0, eq4_e333_d_n1, eq4_e333_d_n2, eq4_e333_d_n3, eq4_e333_d_n4, eq4_e333_d_n5, eq4_e333_d_n6, eq4_e333_d_n7, eq4_e333_d_n8, eq4_e333_d_n9, eq4_e333_d_n10, eq4_e333_d_n11, eq4_e333_d_n12, eq4_e333_d_n13, eq4_e333_d_n14, eq4_e333_d_n15, eq4_e333_d_n16, eq4_e333_d_n17, eq4_e333_d_n18, eq4_e333_d_b0, eq4_e333_d_b1, eq4_e333_d_b2, eq4_e333_d_b3, eq4_e333_d_b4, eq4_e333_d_b5, eq4_e333_d_b6, eq4_e333_d_b7, eq4_e333_d_b8, eq4_e333_d_b9, eq4_e333_d_b10, eq4_e333_d_b11, eq4_e333_d_b12, eq4_e333_d_b13, eq4_e333_d_b14, eq4_e333_d_b15,) = {
    if s.b[1848] {
        let eq4_e331: f64 = (p.p50 * s.v[306]);
        (eq4_e331, (p.p50 * s.dn[306][0]), (p.p50 * s.dn[306][1]), (p.p50 * s.dn[306][2]), (p.p50 * s.dn[306][3]), (p.p50 * s.dn[306][4]), (p.p50 * s.dn[306][5]), (p.p50 * s.dn[306][6]), (p.p50 * s.dn[306][7]), (p.p50 * s.dn[306][8]), (p.p50 * s.dn[306][9]), (p.p50 * s.dn[306][10]), (p.p50 * s.dn[306][11]), (p.p50 * s.dn[306][12]), (p.p50 * s.dn[306][13]), (p.p50 * s.dn[306][14]), (p.p50 * s.dn[306][15]), (p.p50 * s.dn[306][16]), (p.p50 * s.dn[306][17]), (p.p50 * s.dn[306][18]), (p.p50 * s.db[306][0]), (p.p50 * s.db[306][1]), (p.p50 * s.db[306][2]), (p.p50 * s.db[306][3]), (p.p50 * s.db[306][4]), (p.p50 * s.db[306][5]), (p.p50 * s.db[306][6]), (p.p50 * s.db[306][7]), (p.p50 * s.db[306][8]), (p.p50 * s.db[306][9]), (p.p50 * s.db[306][10]), (p.p50 * s.db[306][11]), (p.p50 * s.db[306][12]), (p.p50 * s.db[306][13]), (p.p50 * s.db[306][14]), (p.p50 * s.db[306][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e333;let eq4_node_derivatives: [f64; 19] = [eq4_e333_d_n0, eq4_e333_d_n1, eq4_e333_d_n2, eq4_e333_d_n3, eq4_e333_d_n4, eq4_e333_d_n5, eq4_e333_d_n6, eq4_e333_d_n7, eq4_e333_d_n8, eq4_e333_d_n9, eq4_e333_d_n10, eq4_e333_d_n11, eq4_e333_d_n12, eq4_e333_d_n13, eq4_e333_d_n14, eq4_e333_d_n15, eq4_e333_d_n16, eq4_e333_d_n17, eq4_e333_d_n18];let eq4_branch_derivatives: [f64; 16] = [eq4_e333_d_b0, eq4_e333_d_b1, eq4_e333_d_b2, eq4_e333_d_b3, eq4_e333_d_b4, eq4_e333_d_b5, eq4_e333_d_b6, eq4_e333_d_b7, eq4_e333_d_b8, eq4_e333_d_b9, eq4_e333_d_b10, eq4_e333_d_b11, eq4_e333_d_b12, eq4_e333_d_b13, eq4_e333_d_b14, eq4_e333_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e339, eq5_e339_d_n0, eq5_e339_d_n1, eq5_e339_d_n2, eq5_e339_d_n3, eq5_e339_d_n4, eq5_e339_d_n5, eq5_e339_d_n6, eq5_e339_d_n7, eq5_e339_d_n8, eq5_e339_d_n9, eq5_e339_d_n10, eq5_e339_d_n11, eq5_e339_d_n12, eq5_e339_d_n13, eq5_e339_d_n14, eq5_e339_d_n15, eq5_e339_d_n16, eq5_e339_d_n17, eq5_e339_d_n18, eq5_e339_d_b0, eq5_e339_d_b1, eq5_e339_d_b2, eq5_e339_d_b3, eq5_e339_d_b4, eq5_e339_d_b5, eq5_e339_d_b6, eq5_e339_d_b7, eq5_e339_d_b8, eq5_e339_d_b9, eq5_e339_d_b10, eq5_e339_d_b11, eq5_e339_d_b12, eq5_e339_d_b13, eq5_e339_d_b14, eq5_e339_d_b15,) = {
    if s.b[1848] {
        let eq5_e337: f64 = (p.p50 * s.v[307]);
        (eq5_e337, (p.p50 * s.dn[307][0]), (p.p50 * s.dn[307][1]), (p.p50 * s.dn[307][2]), (p.p50 * s.dn[307][3]), (p.p50 * s.dn[307][4]), (p.p50 * s.dn[307][5]), (p.p50 * s.dn[307][6]), (p.p50 * s.dn[307][7]), (p.p50 * s.dn[307][8]), (p.p50 * s.dn[307][9]), (p.p50 * s.dn[307][10]), (p.p50 * s.dn[307][11]), (p.p50 * s.dn[307][12]), (p.p50 * s.dn[307][13]), (p.p50 * s.dn[307][14]), (p.p50 * s.dn[307][15]), (p.p50 * s.dn[307][16]), (p.p50 * s.dn[307][17]), (p.p50 * s.dn[307][18]), (p.p50 * s.db[307][0]), (p.p50 * s.db[307][1]), (p.p50 * s.db[307][2]), (p.p50 * s.db[307][3]), (p.p50 * s.db[307][4]), (p.p50 * s.db[307][5]), (p.p50 * s.db[307][6]), (p.p50 * s.db[307][7]), (p.p50 * s.db[307][8]), (p.p50 * s.db[307][9]), (p.p50 * s.db[307][10]), (p.p50 * s.db[307][11]), (p.p50 * s.db[307][12]), (p.p50 * s.db[307][13]), (p.p50 * s.db[307][14]), (p.p50 * s.db[307][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e339;let eq5_node_derivatives: [f64; 19] = [eq5_e339_d_n0, eq5_e339_d_n1, eq5_e339_d_n2, eq5_e339_d_n3, eq5_e339_d_n4, eq5_e339_d_n5, eq5_e339_d_n6, eq5_e339_d_n7, eq5_e339_d_n8, eq5_e339_d_n9, eq5_e339_d_n10, eq5_e339_d_n11, eq5_e339_d_n12, eq5_e339_d_n13, eq5_e339_d_n14, eq5_e339_d_n15, eq5_e339_d_n16, eq5_e339_d_n17, eq5_e339_d_n18];let eq5_branch_derivatives: [f64; 16] = [eq5_e339_d_b0, eq5_e339_d_b1, eq5_e339_d_b2, eq5_e339_d_b3, eq5_e339_d_b4, eq5_e339_d_b5, eq5_e339_d_b6, eq5_e339_d_b7, eq5_e339_d_b8, eq5_e339_d_b9, eq5_e339_d_b10, eq5_e339_d_b11, eq5_e339_d_b12, eq5_e339_d_b13, eq5_e339_d_b14, eq5_e339_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e345, eq6_e345_d_n0, eq6_e345_d_n1, eq6_e345_d_n2, eq6_e345_d_n3, eq6_e345_d_n4, eq6_e345_d_n5, eq6_e345_d_n6, eq6_e345_d_n7, eq6_e345_d_n8, eq6_e345_d_n9, eq6_e345_d_n10, eq6_e345_d_n11, eq6_e345_d_n12, eq6_e345_d_n13, eq6_e345_d_n14, eq6_e345_d_n15, eq6_e345_d_n16, eq6_e345_d_n17, eq6_e345_d_n18, eq6_e345_d_b0, eq6_e345_d_b1, eq6_e345_d_b2, eq6_e345_d_b3, eq6_e345_d_b4, eq6_e345_d_b5, eq6_e345_d_b6, eq6_e345_d_b7, eq6_e345_d_b8, eq6_e345_d_b9, eq6_e345_d_b10, eq6_e345_d_b11, eq6_e345_d_b12, eq6_e345_d_b13, eq6_e345_d_b14, eq6_e345_d_b15,) = {
    if s.b[1848] {
        let eq6_e343: f64 = (p.p50 * s.v[308]);
        (eq6_e343, (p.p50 * s.dn[308][0]), (p.p50 * s.dn[308][1]), (p.p50 * s.dn[308][2]), (p.p50 * s.dn[308][3]), (p.p50 * s.dn[308][4]), (p.p50 * s.dn[308][5]), (p.p50 * s.dn[308][6]), (p.p50 * s.dn[308][7]), (p.p50 * s.dn[308][8]), (p.p50 * s.dn[308][9]), (p.p50 * s.dn[308][10]), (p.p50 * s.dn[308][11]), (p.p50 * s.dn[308][12]), (p.p50 * s.dn[308][13]), (p.p50 * s.dn[308][14]), (p.p50 * s.dn[308][15]), (p.p50 * s.dn[308][16]), (p.p50 * s.dn[308][17]), (p.p50 * s.dn[308][18]), (p.p50 * s.db[308][0]), (p.p50 * s.db[308][1]), (p.p50 * s.db[308][2]), (p.p50 * s.db[308][3]), (p.p50 * s.db[308][4]), (p.p50 * s.db[308][5]), (p.p50 * s.db[308][6]), (p.p50 * s.db[308][7]), (p.p50 * s.db[308][8]), (p.p50 * s.db[308][9]), (p.p50 * s.db[308][10]), (p.p50 * s.db[308][11]), (p.p50 * s.db[308][12]), (p.p50 * s.db[308][13]), (p.p50 * s.db[308][14]), (p.p50 * s.db[308][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e345;let eq6_node_derivatives: [f64; 19] = [eq6_e345_d_n0, eq6_e345_d_n1, eq6_e345_d_n2, eq6_e345_d_n3, eq6_e345_d_n4, eq6_e345_d_n5, eq6_e345_d_n6, eq6_e345_d_n7, eq6_e345_d_n8, eq6_e345_d_n9, eq6_e345_d_n10, eq6_e345_d_n11, eq6_e345_d_n12, eq6_e345_d_n13, eq6_e345_d_n14, eq6_e345_d_n15, eq6_e345_d_n16, eq6_e345_d_n17, eq6_e345_d_n18];let eq6_branch_derivatives: [f64; 16] = [eq6_e345_d_b0, eq6_e345_d_b1, eq6_e345_d_b2, eq6_e345_d_b3, eq6_e345_d_b4, eq6_e345_d_b5, eq6_e345_d_b6, eq6_e345_d_b7, eq6_e345_d_b8, eq6_e345_d_b9, eq6_e345_d_b10, eq6_e345_d_b11, eq6_e345_d_b12, eq6_e345_d_b13, eq6_e345_d_b14, eq6_e345_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e351, eq7_e351_d_n0, eq7_e351_d_n1, eq7_e351_d_n2, eq7_e351_d_n3, eq7_e351_d_n4, eq7_e351_d_n5, eq7_e351_d_n6, eq7_e351_d_n7, eq7_e351_d_n8, eq7_e351_d_n9, eq7_e351_d_n10, eq7_e351_d_n11, eq7_e351_d_n12, eq7_e351_d_n13, eq7_e351_d_n14, eq7_e351_d_n15, eq7_e351_d_n16, eq7_e351_d_n17, eq7_e351_d_n18, eq7_e351_d_b0, eq7_e351_d_b1, eq7_e351_d_b2, eq7_e351_d_b3, eq7_e351_d_b4, eq7_e351_d_b5, eq7_e351_d_b6, eq7_e351_d_b7, eq7_e351_d_b8, eq7_e351_d_b9, eq7_e351_d_b10, eq7_e351_d_b11, eq7_e351_d_b12, eq7_e351_d_b13, eq7_e351_d_b14, eq7_e351_d_b15,) = {
    if (p.p259 != 0.0) {
        let eq7_e349: f64 = ((nv7 - nv2) / s.v[1]);let eq7_e349_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));let eq7_e349_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));let eq7_e349_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));let eq7_e349_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));let eq7_e349_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));let eq7_e349_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));let eq7_e349_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));let eq7_e349_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));let eq7_e349_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));let eq7_e349_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));let eq7_e349_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));let eq7_e349_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));let eq7_e349_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));let eq7_e349_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));let eq7_e349_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));let eq7_e349_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));let eq7_e349_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));let eq7_e349_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));let eq7_e349_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));let eq7_e349_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));let eq7_e349_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));let eq7_e349_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));let eq7_e349_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));let eq7_e349_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));let eq7_e349_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));let eq7_e349_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));let eq7_e349_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));let eq7_e349_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));let eq7_e349_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));let eq7_e349_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));let eq7_e349_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));let eq7_e349_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));let eq7_e349_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));let eq7_e349_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));let eq7_e349_d_b15: f64 = (-(((nv7 - nv2) * s.db[1][15]) / (s.v[1] * s.v[1])));
        (eq7_e349, eq7_e349_d_n0, eq7_e349_d_n1, eq7_e349_d_n2, eq7_e349_d_n3, eq7_e349_d_n4, eq7_e349_d_n5, eq7_e349_d_n6, eq7_e349_d_n7, eq7_e349_d_n8, eq7_e349_d_n9, eq7_e349_d_n10, eq7_e349_d_n11, eq7_e349_d_n12, eq7_e349_d_n13, eq7_e349_d_n14, eq7_e349_d_n15, eq7_e349_d_n16, eq7_e349_d_n17, eq7_e349_d_n18, eq7_e349_d_b0, eq7_e349_d_b1, eq7_e349_d_b2, eq7_e349_d_b3, eq7_e349_d_b4, eq7_e349_d_b5, eq7_e349_d_b6, eq7_e349_d_b7, eq7_e349_d_b8, eq7_e349_d_b9, eq7_e349_d_b10, eq7_e349_d_b11, eq7_e349_d_b12, eq7_e349_d_b13, eq7_e349_d_b14, eq7_e349_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e351;let eq7_node_derivatives: [f64; 19] = [eq7_e351_d_n0, eq7_e351_d_n1, eq7_e351_d_n2, eq7_e351_d_n3, eq7_e351_d_n4, eq7_e351_d_n5, eq7_e351_d_n6, eq7_e351_d_n7, eq7_e351_d_n8, eq7_e351_d_n9, eq7_e351_d_n10, eq7_e351_d_n11, eq7_e351_d_n12, eq7_e351_d_n13, eq7_e351_d_n14, eq7_e351_d_n15, eq7_e351_d_n16, eq7_e351_d_n17, eq7_e351_d_n18];let eq7_branch_derivatives: [f64; 16] = [eq7_e351_d_b0, eq7_e351_d_b1, eq7_e351_d_b2, eq7_e351_d_b3, eq7_e351_d_b4, eq7_e351_d_b5, eq7_e351_d_b6, eq7_e351_d_b7, eq7_e351_d_b8, eq7_e351_d_b9, eq7_e351_d_b10, eq7_e351_d_b11, eq7_e351_d_b12, eq7_e351_d_b13, eq7_e351_d_b14, eq7_e351_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e356,) = {
    if (p.p259 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e356;
        stamper.stamp_potential_const_local(
            3,
            eq8_value,
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
        let (eq9_e362, eq9_e362_d_n0, eq9_e362_d_n1, eq9_e362_d_n2, eq9_e362_d_n3, eq9_e362_d_n4, eq9_e362_d_n5, eq9_e362_d_n6, eq9_e362_d_n7, eq9_e362_d_n8, eq9_e362_d_n9, eq9_e362_d_n10, eq9_e362_d_n11, eq9_e362_d_n12, eq9_e362_d_n13, eq9_e362_d_n14, eq9_e362_d_n15, eq9_e362_d_n16, eq9_e362_d_n17, eq9_e362_d_n18, eq9_e362_d_b0, eq9_e362_d_b1, eq9_e362_d_b2, eq9_e362_d_b3, eq9_e362_d_b4, eq9_e362_d_b5, eq9_e362_d_b6, eq9_e362_d_b7, eq9_e362_d_b8, eq9_e362_d_b9, eq9_e362_d_b10, eq9_e362_d_b11, eq9_e362_d_b12, eq9_e362_d_b13, eq9_e362_d_b14, eq9_e362_d_b15,) = {
    if (p.p260 != 0.0) {
        let eq9_e360: f64 = ((nv0 - nv6) / s.v[0]);let eq9_e360_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));let eq9_e360_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));let eq9_e360_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));let eq9_e360_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));let eq9_e360_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));let eq9_e360_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));let eq9_e360_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));let eq9_e360_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));let eq9_e360_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));let eq9_e360_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));let eq9_e360_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));let eq9_e360_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));let eq9_e360_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));let eq9_e360_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));let eq9_e360_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));let eq9_e360_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));let eq9_e360_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));let eq9_e360_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));let eq9_e360_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));let eq9_e360_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));let eq9_e360_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));let eq9_e360_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));let eq9_e360_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));let eq9_e360_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));let eq9_e360_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));let eq9_e360_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));let eq9_e360_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));let eq9_e360_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));let eq9_e360_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));let eq9_e360_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));let eq9_e360_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));let eq9_e360_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));let eq9_e360_d_b13: f64 = (-(((nv0 - nv6) * s.db[0][13]) / (s.v[0] * s.v[0])));let eq9_e360_d_b14: f64 = (-(((nv0 - nv6) * s.db[0][14]) / (s.v[0] * s.v[0])));let eq9_e360_d_b15: f64 = (-(((nv0 - nv6) * s.db[0][15]) / (s.v[0] * s.v[0])));
        (eq9_e360, eq9_e360_d_n0, eq9_e360_d_n1, eq9_e360_d_n2, eq9_e360_d_n3, eq9_e360_d_n4, eq9_e360_d_n5, eq9_e360_d_n6, eq9_e360_d_n7, eq9_e360_d_n8, eq9_e360_d_n9, eq9_e360_d_n10, eq9_e360_d_n11, eq9_e360_d_n12, eq9_e360_d_n13, eq9_e360_d_n14, eq9_e360_d_n15, eq9_e360_d_n16, eq9_e360_d_n17, eq9_e360_d_n18, eq9_e360_d_b0, eq9_e360_d_b1, eq9_e360_d_b2, eq9_e360_d_b3, eq9_e360_d_b4, eq9_e360_d_b5, eq9_e360_d_b6, eq9_e360_d_b7, eq9_e360_d_b8, eq9_e360_d_b9, eq9_e360_d_b10, eq9_e360_d_b11, eq9_e360_d_b12, eq9_e360_d_b13, eq9_e360_d_b14, eq9_e360_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e362;let eq9_node_derivatives: [f64; 19] = [eq9_e362_d_n0, eq9_e362_d_n1, eq9_e362_d_n2, eq9_e362_d_n3, eq9_e362_d_n4, eq9_e362_d_n5, eq9_e362_d_n6, eq9_e362_d_n7, eq9_e362_d_n8, eq9_e362_d_n9, eq9_e362_d_n10, eq9_e362_d_n11, eq9_e362_d_n12, eq9_e362_d_n13, eq9_e362_d_n14, eq9_e362_d_n15, eq9_e362_d_n16, eq9_e362_d_n17, eq9_e362_d_n18];let eq9_branch_derivatives: [f64; 16] = [eq9_e362_d_b0, eq9_e362_d_b1, eq9_e362_d_b2, eq9_e362_d_b3, eq9_e362_d_b4, eq9_e362_d_b5, eq9_e362_d_b6, eq9_e362_d_b7, eq9_e362_d_b8, eq9_e362_d_b9, eq9_e362_d_b10, eq9_e362_d_b11, eq9_e362_d_b12, eq9_e362_d_b13, eq9_e362_d_b14, eq9_e362_d_b15];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e367,) = {
    if (p.p260 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e367;
        stamper.stamp_potential_const_local(
            4,
            eq10_value,
        );let eq11_e370: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[594]);let eq11_e371: f64 = (p.p50 * eq11_e370);let eq11_e371_d_n0: f64 = (p.p50 * (s.dn[594][0] * ddt_scale));let eq11_e371_d_n1: f64 = (p.p50 * (s.dn[594][1] * ddt_scale));let eq11_e371_d_n2: f64 = (p.p50 * (s.dn[594][2] * ddt_scale));let eq11_e371_d_n3: f64 = (p.p50 * (s.dn[594][3] * ddt_scale));let eq11_e371_d_n4: f64 = (p.p50 * (s.dn[594][4] * ddt_scale));let eq11_e371_d_n5: f64 = (p.p50 * (s.dn[594][5] * ddt_scale));let eq11_e371_d_n6: f64 = (p.p50 * (s.dn[594][6] * ddt_scale));let eq11_e371_d_n7: f64 = (p.p50 * (s.dn[594][7] * ddt_scale));let eq11_e371_d_n8: f64 = (p.p50 * (s.dn[594][8] * ddt_scale));let eq11_e371_d_n9: f64 = (p.p50 * (s.dn[594][9] * ddt_scale));let eq11_e371_d_n10: f64 = (p.p50 * (s.dn[594][10] * ddt_scale));let eq11_e371_d_n11: f64 = (p.p50 * (s.dn[594][11] * ddt_scale));let eq11_e371_d_n12: f64 = (p.p50 * (s.dn[594][12] * ddt_scale));let eq11_e371_d_n13: f64 = (p.p50 * (s.dn[594][13] * ddt_scale));let eq11_e371_d_n14: f64 = (p.p50 * (s.dn[594][14] * ddt_scale));let eq11_e371_d_n15: f64 = (p.p50 * (s.dn[594][15] * ddt_scale));let eq11_e371_d_n16: f64 = (p.p50 * (s.dn[594][16] * ddt_scale));let eq11_e371_d_n17: f64 = (p.p50 * (s.dn[594][17] * ddt_scale));let eq11_e371_d_n18: f64 = (p.p50 * (s.dn[594][18] * ddt_scale));let eq11_e371_d_b0: f64 = (p.p50 * (s.db[594][0] * ddt_scale));let eq11_e371_d_b1: f64 = (p.p50 * (s.db[594][1] * ddt_scale));let eq11_e371_d_b2: f64 = (p.p50 * (s.db[594][2] * ddt_scale));let eq11_e371_d_b3: f64 = (p.p50 * (s.db[594][3] * ddt_scale));let eq11_e371_d_b4: f64 = (p.p50 * (s.db[594][4] * ddt_scale));let eq11_e371_d_b5: f64 = (p.p50 * (s.db[594][5] * ddt_scale));let eq11_e371_d_b6: f64 = (p.p50 * (s.db[594][6] * ddt_scale));let eq11_e371_d_b7: f64 = (p.p50 * (s.db[594][7] * ddt_scale));let eq11_e371_d_b8: f64 = (p.p50 * (s.db[594][8] * ddt_scale));let eq11_e371_d_b9: f64 = (p.p50 * (s.db[594][9] * ddt_scale));let eq11_e371_d_b10: f64 = (p.p50 * (s.db[594][10] * ddt_scale));let eq11_e371_d_b11: f64 = (p.p50 * (s.db[594][11] * ddt_scale));let eq11_e371_d_b12: f64 = (p.p50 * (s.db[594][12] * ddt_scale));let eq11_e371_d_b13: f64 = (p.p50 * (s.db[594][13] * ddt_scale));let eq11_e371_d_b14: f64 = (p.p50 * (s.db[594][14] * ddt_scale));let eq11_e371_d_b15: f64 = (p.p50 * (s.db[594][15] * ddt_scale));let eq11_value: f64 = eq11_e371;let eq11_node_derivatives: [f64; 19] = [eq11_e371_d_n0, eq11_e371_d_n1, eq11_e371_d_n2, eq11_e371_d_n3, eq11_e371_d_n4, eq11_e371_d_n5, eq11_e371_d_n6, eq11_e371_d_n7, eq11_e371_d_n8, eq11_e371_d_n9, eq11_e371_d_n10, eq11_e371_d_n11, eq11_e371_d_n12, eq11_e371_d_n13, eq11_e371_d_n14, eq11_e371_d_n15, eq11_e371_d_n16, eq11_e371_d_n17, eq11_e371_d_n18];let eq11_branch_derivatives: [f64; 16] = [eq11_e371_d_b0, eq11_e371_d_b1, eq11_e371_d_b2, eq11_e371_d_b3, eq11_e371_d_b4, eq11_e371_d_b5, eq11_e371_d_b6, eq11_e371_d_b7, eq11_e371_d_b8, eq11_e371_d_b9, eq11_e371_d_b10, eq11_e371_d_b11, eq11_e371_d_b12, eq11_e371_d_b13, eq11_e371_d_b14, eq11_e371_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq12_e374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[198]);let eq12_e375: f64 = (p.p50 * eq12_e374);let eq12_e375_d_n0: f64 = (p.p50 * (s.dn[198][0] * ddt_scale));let eq12_e375_d_n1: f64 = (p.p50 * (s.dn[198][1] * ddt_scale));let eq12_e375_d_n2: f64 = (p.p50 * (s.dn[198][2] * ddt_scale));let eq12_e375_d_n3: f64 = (p.p50 * (s.dn[198][3] * ddt_scale));let eq12_e375_d_n4: f64 = (p.p50 * (s.dn[198][4] * ddt_scale));let eq12_e375_d_n5: f64 = (p.p50 * (s.dn[198][5] * ddt_scale));let eq12_e375_d_n6: f64 = (p.p50 * (s.dn[198][6] * ddt_scale));let eq12_e375_d_n7: f64 = (p.p50 * (s.dn[198][7] * ddt_scale));let eq12_e375_d_n8: f64 = (p.p50 * (s.dn[198][8] * ddt_scale));let eq12_e375_d_n9: f64 = (p.p50 * (s.dn[198][9] * ddt_scale));let eq12_e375_d_n10: f64 = (p.p50 * (s.dn[198][10] * ddt_scale));let eq12_e375_d_n11: f64 = (p.p50 * (s.dn[198][11] * ddt_scale));let eq12_e375_d_n12: f64 = (p.p50 * (s.dn[198][12] * ddt_scale));let eq12_e375_d_n13: f64 = (p.p50 * (s.dn[198][13] * ddt_scale));let eq12_e375_d_n14: f64 = (p.p50 * (s.dn[198][14] * ddt_scale));let eq12_e375_d_n15: f64 = (p.p50 * (s.dn[198][15] * ddt_scale));let eq12_e375_d_n16: f64 = (p.p50 * (s.dn[198][16] * ddt_scale));let eq12_e375_d_n17: f64 = (p.p50 * (s.dn[198][17] * ddt_scale));let eq12_e375_d_n18: f64 = (p.p50 * (s.dn[198][18] * ddt_scale));let eq12_e375_d_b0: f64 = (p.p50 * (s.db[198][0] * ddt_scale));let eq12_e375_d_b1: f64 = (p.p50 * (s.db[198][1] * ddt_scale));let eq12_e375_d_b2: f64 = (p.p50 * (s.db[198][2] * ddt_scale));let eq12_e375_d_b3: f64 = (p.p50 * (s.db[198][3] * ddt_scale));let eq12_e375_d_b4: f64 = (p.p50 * (s.db[198][4] * ddt_scale));let eq12_e375_d_b5: f64 = (p.p50 * (s.db[198][5] * ddt_scale));let eq12_e375_d_b6: f64 = (p.p50 * (s.db[198][6] * ddt_scale));let eq12_e375_d_b7: f64 = (p.p50 * (s.db[198][7] * ddt_scale));let eq12_e375_d_b8: f64 = (p.p50 * (s.db[198][8] * ddt_scale));let eq12_e375_d_b9: f64 = (p.p50 * (s.db[198][9] * ddt_scale));let eq12_e375_d_b10: f64 = (p.p50 * (s.db[198][10] * ddt_scale));let eq12_e375_d_b11: f64 = (p.p50 * (s.db[198][11] * ddt_scale));let eq12_e375_d_b12: f64 = (p.p50 * (s.db[198][12] * ddt_scale));let eq12_e375_d_b13: f64 = (p.p50 * (s.db[198][13] * ddt_scale));let eq12_e375_d_b14: f64 = (p.p50 * (s.db[198][14] * ddt_scale));let eq12_e375_d_b15: f64 = (p.p50 * (s.db[198][15] * ddt_scale));let eq12_value: f64 = eq12_e375;let eq12_node_derivatives: [f64; 19] = [eq12_e375_d_n0, eq12_e375_d_n1, eq12_e375_d_n2, eq12_e375_d_n3, eq12_e375_d_n4, eq12_e375_d_n5, eq12_e375_d_n6, eq12_e375_d_n7, eq12_e375_d_n8, eq12_e375_d_n9, eq12_e375_d_n10, eq12_e375_d_n11, eq12_e375_d_n12, eq12_e375_d_n13, eq12_e375_d_n14, eq12_e375_d_n15, eq12_e375_d_n16, eq12_e375_d_n17, eq12_e375_d_n18];let eq12_branch_derivatives: [f64; 16] = [eq12_e375_d_b0, eq12_e375_d_b1, eq12_e375_d_b2, eq12_e375_d_b3, eq12_e375_d_b4, eq12_e375_d_b5, eq12_e375_d_b6, eq12_e375_d_b7, eq12_e375_d_b8, eq12_e375_d_b9, eq12_e375_d_b10, eq12_e375_d_b11, eq12_e375_d_b12, eq12_e375_d_b13, eq12_e375_d_b14, eq12_e375_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );let eq13_e378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[196]);let eq13_e379: f64 = (p.p50 * eq13_e378);let eq13_e379_d_n0: f64 = (p.p50 * (s.dn[196][0] * ddt_scale));let eq13_e379_d_n1: f64 = (p.p50 * (s.dn[196][1] * ddt_scale));let eq13_e379_d_n2: f64 = (p.p50 * (s.dn[196][2] * ddt_scale));let eq13_e379_d_n3: f64 = (p.p50 * (s.dn[196][3] * ddt_scale));let eq13_e379_d_n4: f64 = (p.p50 * (s.dn[196][4] * ddt_scale));let eq13_e379_d_n5: f64 = (p.p50 * (s.dn[196][5] * ddt_scale));let eq13_e379_d_n6: f64 = (p.p50 * (s.dn[196][6] * ddt_scale));let eq13_e379_d_n7: f64 = (p.p50 * (s.dn[196][7] * ddt_scale));let eq13_e379_d_n8: f64 = (p.p50 * (s.dn[196][8] * ddt_scale));let eq13_e379_d_n9: f64 = (p.p50 * (s.dn[196][9] * ddt_scale));let eq13_e379_d_n10: f64 = (p.p50 * (s.dn[196][10] * ddt_scale));let eq13_e379_d_n11: f64 = (p.p50 * (s.dn[196][11] * ddt_scale));let eq13_e379_d_n12: f64 = (p.p50 * (s.dn[196][12] * ddt_scale));let eq13_e379_d_n13: f64 = (p.p50 * (s.dn[196][13] * ddt_scale));let eq13_e379_d_n14: f64 = (p.p50 * (s.dn[196][14] * ddt_scale));let eq13_e379_d_n15: f64 = (p.p50 * (s.dn[196][15] * ddt_scale));let eq13_e379_d_n16: f64 = (p.p50 * (s.dn[196][16] * ddt_scale));let eq13_e379_d_n17: f64 = (p.p50 * (s.dn[196][17] * ddt_scale));let eq13_e379_d_n18: f64 = (p.p50 * (s.dn[196][18] * ddt_scale));let eq13_e379_d_b0: f64 = (p.p50 * (s.db[196][0] * ddt_scale));let eq13_e379_d_b1: f64 = (p.p50 * (s.db[196][1] * ddt_scale));let eq13_e379_d_b2: f64 = (p.p50 * (s.db[196][2] * ddt_scale));let eq13_e379_d_b3: f64 = (p.p50 * (s.db[196][3] * ddt_scale));let eq13_e379_d_b4: f64 = (p.p50 * (s.db[196][4] * ddt_scale));let eq13_e379_d_b5: f64 = (p.p50 * (s.db[196][5] * ddt_scale));let eq13_e379_d_b6: f64 = (p.p50 * (s.db[196][6] * ddt_scale));let eq13_e379_d_b7: f64 = (p.p50 * (s.db[196][7] * ddt_scale));let eq13_e379_d_b8: f64 = (p.p50 * (s.db[196][8] * ddt_scale));let eq13_e379_d_b9: f64 = (p.p50 * (s.db[196][9] * ddt_scale));let eq13_e379_d_b10: f64 = (p.p50 * (s.db[196][10] * ddt_scale));let eq13_e379_d_b11: f64 = (p.p50 * (s.db[196][11] * ddt_scale));let eq13_e379_d_b12: f64 = (p.p50 * (s.db[196][12] * ddt_scale));let eq13_e379_d_b13: f64 = (p.p50 * (s.db[196][13] * ddt_scale));let eq13_e379_d_b14: f64 = (p.p50 * (s.db[196][14] * ddt_scale));let eq13_e379_d_b15: f64 = (p.p50 * (s.db[196][15] * ddt_scale));let eq13_value: f64 = eq13_e379;let eq13_node_derivatives: [f64; 19] = [eq13_e379_d_n0, eq13_e379_d_n1, eq13_e379_d_n2, eq13_e379_d_n3, eq13_e379_d_n4, eq13_e379_d_n5, eq13_e379_d_n6, eq13_e379_d_n7, eq13_e379_d_n8, eq13_e379_d_n9, eq13_e379_d_n10, eq13_e379_d_n11, eq13_e379_d_n12, eq13_e379_d_n13, eq13_e379_d_n14, eq13_e379_d_n15, eq13_e379_d_n16, eq13_e379_d_n17, eq13_e379_d_n18];let eq13_branch_derivatives: [f64; 16] = [eq13_e379_d_b0, eq13_e379_d_b1, eq13_e379_d_b2, eq13_e379_d_b3, eq13_e379_d_b4, eq13_e379_d_b5, eq13_e379_d_b6, eq13_e379_d_b7, eq13_e379_d_b8, eq13_e379_d_b9, eq13_e379_d_b10, eq13_e379_d_b11, eq13_e379_d_b12, eq13_e379_d_b13, eq13_e379_d_b14, eq13_e379_d_b15];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );let eq15_e390: f64 = (nv14 - 0.0);let eq15_value: f64 = eq15_e390;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq15_value),
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq18_e405: f64 = (s.v[614] * (nv14 - 0.0));let eq18_e405_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));let eq18_e405_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));let eq18_e405_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));let eq18_e405_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));let eq18_e405_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));let eq18_e405_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));let eq18_e405_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));let eq18_e405_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));let eq18_e405_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));let eq18_e405_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));let eq18_e405_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));let eq18_e405_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));let eq18_e405_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));let eq18_e405_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));let eq18_e405_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);let eq18_e405_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));let eq18_e405_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));let eq18_e405_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));let eq18_e405_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));let eq18_e405_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));let eq18_e405_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));let eq18_e405_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));let eq18_e405_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));let eq18_e405_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));let eq18_e405_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));let eq18_e405_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));let eq18_e405_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));let eq18_e405_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));let eq18_e405_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));let eq18_e405_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));let eq18_e405_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));let eq18_e405_d_b12: f64 = (s.db[614][12] * (nv14 - 0.0));let eq18_e405_d_b13: f64 = (s.db[614][13] * (nv14 - 0.0));let eq18_e405_d_b14: f64 = (s.db[614][14] * (nv14 - 0.0));let eq18_e405_d_b15: f64 = (s.db[614][15] * (nv14 - 0.0));let eq18_value: f64 = eq18_e405;let eq18_node_derivatives: [f64; 19] = [eq18_e405_d_n0, eq18_e405_d_n1, eq18_e405_d_n2, eq18_e405_d_n3, eq18_e405_d_n4, eq18_e405_d_n5, eq18_e405_d_n6, eq18_e405_d_n7, eq18_e405_d_n8, eq18_e405_d_n9, eq18_e405_d_n10, eq18_e405_d_n11, eq18_e405_d_n12, eq18_e405_d_n13, eq18_e405_d_n14, eq18_e405_d_n15, eq18_e405_d_n16, eq18_e405_d_n17, eq18_e405_d_n18];let eq18_branch_derivatives: [f64; 16] = [eq18_e405_d_b0, eq18_e405_d_b1, eq18_e405_d_b2, eq18_e405_d_b3, eq18_e405_d_b4, eq18_e405_d_b5, eq18_e405_d_b6, eq18_e405_d_b7, eq18_e405_d_b8, eq18_e405_d_b9, eq18_e405_d_b10, eq18_e405_d_b11, eq18_e405_d_b12, eq18_e405_d_b13, eq18_e405_d_b14, eq18_e405_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );let eq19_e408: f64 = ((nv14 - 0.0) * s.v[617]);let eq19_e408_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);let eq19_e408_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);let eq19_e408_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);let eq19_e408_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);let eq19_e408_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);let eq19_e408_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);let eq19_e408_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);let eq19_e408_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);let eq19_e408_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);let eq19_e408_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);let eq19_e408_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);let eq19_e408_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);let eq19_e408_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);let eq19_e408_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);let eq19_e408_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));let eq19_e408_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);let eq19_e408_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);let eq19_e408_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);let eq19_e408_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);let eq19_e408_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);let eq19_e408_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);let eq19_e408_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);let eq19_e408_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);let eq19_e408_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);let eq19_e408_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);let eq19_e408_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);let eq19_e408_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);let eq19_e408_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);let eq19_e408_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);let eq19_e408_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);let eq19_e408_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);let eq19_e408_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);let eq19_e408_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);let eq19_e408_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);let eq19_e408_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);let eq19_e409: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e408);let eq19_value: f64 = eq19_e409;let eq19_node_derivatives: [f64; 19] = [(eq19_e408_d_n0 * ddt_scale), (eq19_e408_d_n1 * ddt_scale), (eq19_e408_d_n2 * ddt_scale), (eq19_e408_d_n3 * ddt_scale), (eq19_e408_d_n4 * ddt_scale), (eq19_e408_d_n5 * ddt_scale), (eq19_e408_d_n6 * ddt_scale), (eq19_e408_d_n7 * ddt_scale), (eq19_e408_d_n8 * ddt_scale), (eq19_e408_d_n9 * ddt_scale), (eq19_e408_d_n10 * ddt_scale), (eq19_e408_d_n11 * ddt_scale), (eq19_e408_d_n12 * ddt_scale), (eq19_e408_d_n13 * ddt_scale), (eq19_e408_d_n14 * ddt_scale), (eq19_e408_d_n15 * ddt_scale), (eq19_e408_d_n16 * ddt_scale), (eq19_e408_d_n17 * ddt_scale), (eq19_e408_d_n18 * ddt_scale)];let eq19_branch_derivatives: [f64; 16] = [(eq19_e408_d_b0 * ddt_scale), (eq19_e408_d_b1 * ddt_scale), (eq19_e408_d_b2 * ddt_scale), (eq19_e408_d_b3 * ddt_scale), (eq19_e408_d_b4 * ddt_scale), (eq19_e408_d_b5 * ddt_scale), (eq19_e408_d_b6 * ddt_scale), (eq19_e408_d_b7 * ddt_scale), (eq19_e408_d_b8 * ddt_scale), (eq19_e408_d_b9 * ddt_scale), (eq19_e408_d_b10 * ddt_scale), (eq19_e408_d_b11 * ddt_scale), (eq19_e408_d_b12 * ddt_scale), (eq19_e408_d_b13 * ddt_scale), (eq19_e408_d_b14 * ddt_scale), (eq19_e408_d_b15 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
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
        let nv1 = ctx.node_voltage(nodes[1]);let nv11 = ctx.node_voltage(nodes[11]);let nv14 = ctx.node_voltage(nodes[14]);let eq20_e412: f64 = ((nv14 - 0.0) * s.v[618]);let eq20_e412_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);let eq20_e412_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);let eq20_e412_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);let eq20_e412_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);let eq20_e412_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);let eq20_e412_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);let eq20_e412_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);let eq20_e412_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);let eq20_e412_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);let eq20_e412_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);let eq20_e412_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);let eq20_e412_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);let eq20_e412_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);let eq20_e412_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);let eq20_e412_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));let eq20_e412_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);let eq20_e412_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);let eq20_e412_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);let eq20_e412_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);let eq20_e412_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);let eq20_e412_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);let eq20_e412_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);let eq20_e412_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);let eq20_e412_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);let eq20_e412_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);let eq20_e412_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);let eq20_e412_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);let eq20_e412_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);let eq20_e412_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);let eq20_e412_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);let eq20_e412_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);let eq20_e412_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);let eq20_e412_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);let eq20_e412_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);let eq20_e412_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);let eq20_e413: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e412);let eq20_value: f64 = eq20_e413;let eq20_node_derivatives: [f64; 19] = [(eq20_e412_d_n0 * ddt_scale), (eq20_e412_d_n1 * ddt_scale), (eq20_e412_d_n2 * ddt_scale), (eq20_e412_d_n3 * ddt_scale), (eq20_e412_d_n4 * ddt_scale), (eq20_e412_d_n5 * ddt_scale), (eq20_e412_d_n6 * ddt_scale), (eq20_e412_d_n7 * ddt_scale), (eq20_e412_d_n8 * ddt_scale), (eq20_e412_d_n9 * ddt_scale), (eq20_e412_d_n10 * ddt_scale), (eq20_e412_d_n11 * ddt_scale), (eq20_e412_d_n12 * ddt_scale), (eq20_e412_d_n13 * ddt_scale), (eq20_e412_d_n14 * ddt_scale), (eq20_e412_d_n15 * ddt_scale), (eq20_e412_d_n16 * ddt_scale), (eq20_e412_d_n17 * ddt_scale), (eq20_e412_d_n18 * ddt_scale)];let eq20_branch_derivatives: [f64; 16] = [(eq20_e412_d_b0 * ddt_scale), (eq20_e412_d_b1 * ddt_scale), (eq20_e412_d_b2 * ddt_scale), (eq20_e412_d_b3 * ddt_scale), (eq20_e412_d_b4 * ddt_scale), (eq20_e412_d_b5 * ddt_scale), (eq20_e412_d_b6 * ddt_scale), (eq20_e412_d_b7 * ddt_scale), (eq20_e412_d_b8 * ddt_scale), (eq20_e412_d_b9 * ddt_scale), (eq20_e412_d_b10 * ddt_scale), (eq20_e412_d_b11 * ddt_scale), (eq20_e412_d_b12 * ddt_scale), (eq20_e412_d_b13 * ddt_scale), (eq20_e412_d_b14 * ddt_scale), (eq20_e412_d_b15 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq26_e465, eq26_e465_d_n0, eq26_e465_d_n1, eq26_e465_d_n2, eq26_e465_d_n3, eq26_e465_d_n4, eq26_e465_d_n5, eq26_e465_d_n6, eq26_e465_d_n7, eq26_e465_d_n8, eq26_e465_d_n9, eq26_e465_d_n10, eq26_e465_d_n11, eq26_e465_d_n12, eq26_e465_d_n13, eq26_e465_d_n14, eq26_e465_d_n15, eq26_e465_d_n16, eq26_e465_d_n17, eq26_e465_d_n18, eq26_e465_d_b0, eq26_e465_d_b1, eq26_e465_d_b2, eq26_e465_d_b3, eq26_e465_d_b4, eq26_e465_d_b5, eq26_e465_d_b6, eq26_e465_d_b7, eq26_e465_d_b8, eq26_e465_d_b9, eq26_e465_d_b10, eq26_e465_d_b11, eq26_e465_d_b12, eq26_e465_d_b13, eq26_e465_d_b14, eq26_e465_d_b15,) = {
    if (p.p35 != 0.0) {
        let eq26_e463: f64 = (s.v[551] * (nv1 - nv11));let eq26_e463_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));let eq26_e463_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);let eq26_e463_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));let eq26_e463_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));let eq26_e463_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));let eq26_e463_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));let eq26_e463_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));let eq26_e463_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));let eq26_e463_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));let eq26_e463_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));let eq26_e463_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));let eq26_e463_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));let eq26_e463_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));let eq26_e463_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));let eq26_e463_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));let eq26_e463_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));let eq26_e463_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));let eq26_e463_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));let eq26_e463_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));let eq26_e463_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));let eq26_e463_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));let eq26_e463_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));let eq26_e463_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));let eq26_e463_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));let eq26_e463_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));let eq26_e463_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));let eq26_e463_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));let eq26_e463_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));let eq26_e463_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));let eq26_e463_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));let eq26_e463_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));let eq26_e463_d_b12: f64 = (s.db[551][12] * (nv1 - nv11));let eq26_e463_d_b13: f64 = (s.db[551][13] * (nv1 - nv11));let eq26_e463_d_b14: f64 = (s.db[551][14] * (nv1 - nv11));let eq26_e463_d_b15: f64 = (s.db[551][15] * (nv1 - nv11));
        (eq26_e463, eq26_e463_d_n0, eq26_e463_d_n1, eq26_e463_d_n2, eq26_e463_d_n3, eq26_e463_d_n4, eq26_e463_d_n5, eq26_e463_d_n6, eq26_e463_d_n7, eq26_e463_d_n8, eq26_e463_d_n9, eq26_e463_d_n10, eq26_e463_d_n11, eq26_e463_d_n12, eq26_e463_d_n13, eq26_e463_d_n14, eq26_e463_d_n15, eq26_e463_d_n16, eq26_e463_d_n17, eq26_e463_d_n18, eq26_e463_d_b0, eq26_e463_d_b1, eq26_e463_d_b2, eq26_e463_d_b3, eq26_e463_d_b4, eq26_e463_d_b5, eq26_e463_d_b6, eq26_e463_d_b7, eq26_e463_d_b8, eq26_e463_d_b9, eq26_e463_d_b10, eq26_e463_d_b11, eq26_e463_d_b12, eq26_e463_d_b13, eq26_e463_d_b14, eq26_e463_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e465;let eq26_node_derivatives: [f64; 19] = [eq26_e465_d_n0, eq26_e465_d_n1, eq26_e465_d_n2, eq26_e465_d_n3, eq26_e465_d_n4, eq26_e465_d_n5, eq26_e465_d_n6, eq26_e465_d_n7, eq26_e465_d_n8, eq26_e465_d_n9, eq26_e465_d_n10, eq26_e465_d_n11, eq26_e465_d_n12, eq26_e465_d_n13, eq26_e465_d_n14, eq26_e465_d_n15, eq26_e465_d_n16, eq26_e465_d_n17, eq26_e465_d_n18];let eq26_branch_derivatives: [f64; 16] = [eq26_e465_d_b0, eq26_e465_d_b1, eq26_e465_d_b2, eq26_e465_d_b3, eq26_e465_d_b4, eq26_e465_d_b5, eq26_e465_d_b6, eq26_e465_d_b7, eq26_e465_d_b8, eq26_e465_d_b9, eq26_e465_d_b10, eq26_e465_d_b11, eq26_e465_d_b12, eq26_e465_d_b13, eq26_e465_d_b14, eq26_e465_d_b15];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e470,) = {
    if (p.p35 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e470;
        stamper.stamp_potential_const_local(
            5,
            eq27_value,
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
        let (eq28_e476, eq28_e476_d_n0, eq28_e476_d_n1, eq28_e476_d_n2, eq28_e476_d_n3, eq28_e476_d_n4, eq28_e476_d_n5, eq28_e476_d_n6, eq28_e476_d_n7, eq28_e476_d_n8, eq28_e476_d_n9, eq28_e476_d_n10, eq28_e476_d_n11, eq28_e476_d_n12, eq28_e476_d_n13, eq28_e476_d_n14, eq28_e476_d_n15, eq28_e476_d_n16, eq28_e476_d_n17, eq28_e476_d_n18, eq28_e476_d_b0, eq28_e476_d_b1, eq28_e476_d_b2, eq28_e476_d_b3, eq28_e476_d_b4, eq28_e476_d_b5, eq28_e476_d_b6, eq28_e476_d_b7, eq28_e476_d_b8, eq28_e476_d_b9, eq28_e476_d_b10, eq28_e476_d_b11, eq28_e476_d_b12, eq28_e476_d_b13, eq28_e476_d_b14, eq28_e476_d_b15,) = {
    if s.b[1850] {
        let eq28_e474: f64 = ((nv10 - 0.0) * s.v[589]);let eq28_e474_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);let eq28_e474_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);let eq28_e474_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);let eq28_e474_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);let eq28_e474_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);let eq28_e474_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);let eq28_e474_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);let eq28_e474_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);let eq28_e474_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);let eq28_e474_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);let eq28_e474_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));let eq28_e474_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);let eq28_e474_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);let eq28_e474_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);let eq28_e474_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);let eq28_e474_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);let eq28_e474_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);let eq28_e474_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);let eq28_e474_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);let eq28_e474_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);let eq28_e474_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);let eq28_e474_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);let eq28_e474_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);let eq28_e474_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);let eq28_e474_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);let eq28_e474_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);let eq28_e474_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);let eq28_e474_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);let eq28_e474_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);let eq28_e474_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);let eq28_e474_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);let eq28_e474_d_b12: f64 = ((nv10 - 0.0) * s.db[589][12]);let eq28_e474_d_b13: f64 = ((nv10 - 0.0) * s.db[589][13]);let eq28_e474_d_b14: f64 = ((nv10 - 0.0) * s.db[589][14]);let eq28_e474_d_b15: f64 = ((nv10 - 0.0) * s.db[589][15]);
        (eq28_e474, eq28_e474_d_n0, eq28_e474_d_n1, eq28_e474_d_n2, eq28_e474_d_n3, eq28_e474_d_n4, eq28_e474_d_n5, eq28_e474_d_n6, eq28_e474_d_n7, eq28_e474_d_n8, eq28_e474_d_n9, eq28_e474_d_n10, eq28_e474_d_n11, eq28_e474_d_n12, eq28_e474_d_n13, eq28_e474_d_n14, eq28_e474_d_n15, eq28_e474_d_n16, eq28_e474_d_n17, eq28_e474_d_n18, eq28_e474_d_b0, eq28_e474_d_b1, eq28_e474_d_b2, eq28_e474_d_b3, eq28_e474_d_b4, eq28_e474_d_b5, eq28_e474_d_b6, eq28_e474_d_b7, eq28_e474_d_b8, eq28_e474_d_b9, eq28_e474_d_b10, eq28_e474_d_b11, eq28_e474_d_b12, eq28_e474_d_b13, eq28_e474_d_b14, eq28_e474_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e476;let eq28_node_derivatives: [f64; 19] = [eq28_e476_d_n0, eq28_e476_d_n1, eq28_e476_d_n2, eq28_e476_d_n3, eq28_e476_d_n4, eq28_e476_d_n5, eq28_e476_d_n6, eq28_e476_d_n7, eq28_e476_d_n8, eq28_e476_d_n9, eq28_e476_d_n10, eq28_e476_d_n11, eq28_e476_d_n12, eq28_e476_d_n13, eq28_e476_d_n14, eq28_e476_d_n15, eq28_e476_d_n16, eq28_e476_d_n17, eq28_e476_d_n18];let eq28_branch_derivatives: [f64; 16] = [eq28_e476_d_b0, eq28_e476_d_b1, eq28_e476_d_b2, eq28_e476_d_b3, eq28_e476_d_b4, eq28_e476_d_b5, eq28_e476_d_b6, eq28_e476_d_b7, eq28_e476_d_b8, eq28_e476_d_b9, eq28_e476_d_b10, eq28_e476_d_b11, eq28_e476_d_b12, eq28_e476_d_b13, eq28_e476_d_b14, eq28_e476_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e481, eq29_e481_d_n0, eq29_e481_d_n1, eq29_e481_d_n2, eq29_e481_d_n3, eq29_e481_d_n4, eq29_e481_d_n5, eq29_e481_d_n6, eq29_e481_d_n7, eq29_e481_d_n8, eq29_e481_d_n9, eq29_e481_d_n10, eq29_e481_d_n11, eq29_e481_d_n12, eq29_e481_d_n13, eq29_e481_d_n14, eq29_e481_d_n15, eq29_e481_d_n16, eq29_e481_d_n17, eq29_e481_d_n18, eq29_e481_d_b0, eq29_e481_d_b1, eq29_e481_d_b2, eq29_e481_d_b3, eq29_e481_d_b4, eq29_e481_d_b5, eq29_e481_d_b6, eq29_e481_d_b7, eq29_e481_d_b8, eq29_e481_d_b9, eq29_e481_d_b10, eq29_e481_d_b11, eq29_e481_d_b12, eq29_e481_d_b13, eq29_e481_d_b14, eq29_e481_d_b15,) = {
    if s.b[1850] {
        let eq29_e479: f64 = (-s.v[595]);
        (eq29_e479, (-s.dn[595][0]), (-s.dn[595][1]), (-s.dn[595][2]), (-s.dn[595][3]), (-s.dn[595][4]), (-s.dn[595][5]), (-s.dn[595][6]), (-s.dn[595][7]), (-s.dn[595][8]), (-s.dn[595][9]), (-s.dn[595][10]), (-s.dn[595][11]), (-s.dn[595][12]), (-s.dn[595][13]), (-s.dn[595][14]), (-s.dn[595][15]), (-s.dn[595][16]), (-s.dn[595][17]), (-s.dn[595][18]), (-s.db[595][0]), (-s.db[595][1]), (-s.db[595][2]), (-s.db[595][3]), (-s.db[595][4]), (-s.db[595][5]), (-s.db[595][6]), (-s.db[595][7]), (-s.db[595][8]), (-s.db[595][9]), (-s.db[595][10]), (-s.db[595][11]), (-s.db[595][12]), (-s.db[595][13]), (-s.db[595][14]), (-s.db[595][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e481;let eq29_node_derivatives: [f64; 19] = [eq29_e481_d_n0, eq29_e481_d_n1, eq29_e481_d_n2, eq29_e481_d_n3, eq29_e481_d_n4, eq29_e481_d_n5, eq29_e481_d_n6, eq29_e481_d_n7, eq29_e481_d_n8, eq29_e481_d_n9, eq29_e481_d_n10, eq29_e481_d_n11, eq29_e481_d_n12, eq29_e481_d_n13, eq29_e481_d_n14, eq29_e481_d_n15, eq29_e481_d_n16, eq29_e481_d_n17, eq29_e481_d_n18];let eq29_branch_derivatives: [f64; 16] = [eq29_e481_d_b0, eq29_e481_d_b1, eq29_e481_d_b2, eq29_e481_d_b3, eq29_e481_d_b4, eq29_e481_d_b5, eq29_e481_d_b6, eq29_e481_d_b7, eq29_e481_d_b8, eq29_e481_d_b9, eq29_e481_d_b10, eq29_e481_d_b11, eq29_e481_d_b12, eq29_e481_d_b13, eq29_e481_d_b14, eq29_e481_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e487, eq30_e487_d_n10,) = {
    if s.b[1850] {
        let eq30_e485: f64 = ((nv10 - 0.0) * 1e-12);
        (eq30_e485, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e487;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e487_d_n10),
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
        let (eq31_e494, eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18, eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15,) = {
    if s.b[1850] {
        let eq31_e491: f64 = (s.v[563] * (nv10 - 0.0));let eq31_e491_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));let eq31_e491_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));let eq31_e491_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));let eq31_e491_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));let eq31_e491_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));let eq31_e491_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));let eq31_e491_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));let eq31_e491_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));let eq31_e491_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));let eq31_e491_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));let eq31_e491_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);let eq31_e491_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));let eq31_e491_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));let eq31_e491_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));let eq31_e491_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));let eq31_e491_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));let eq31_e491_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));let eq31_e491_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));let eq31_e491_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));let eq31_e491_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));let eq31_e491_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));let eq31_e491_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));let eq31_e491_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));let eq31_e491_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));let eq31_e491_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));let eq31_e491_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));let eq31_e491_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));let eq31_e491_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));let eq31_e491_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));let eq31_e491_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));let eq31_e491_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));let eq31_e491_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));let eq31_e491_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));let eq31_e491_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));let eq31_e491_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));let eq31_e492: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e491);
        (eq31_e492, (eq31_e491_d_n0 * ddt_scale), (eq31_e491_d_n1 * ddt_scale), (eq31_e491_d_n2 * ddt_scale), (eq31_e491_d_n3 * ddt_scale), (eq31_e491_d_n4 * ddt_scale), (eq31_e491_d_n5 * ddt_scale), (eq31_e491_d_n6 * ddt_scale), (eq31_e491_d_n7 * ddt_scale), (eq31_e491_d_n8 * ddt_scale), (eq31_e491_d_n9 * ddt_scale), (eq31_e491_d_n10 * ddt_scale), (eq31_e491_d_n11 * ddt_scale), (eq31_e491_d_n12 * ddt_scale), (eq31_e491_d_n13 * ddt_scale), (eq31_e491_d_n14 * ddt_scale), (eq31_e491_d_n15 * ddt_scale), (eq31_e491_d_n16 * ddt_scale), (eq31_e491_d_n17 * ddt_scale), (eq31_e491_d_n18 * ddt_scale), (eq31_e491_d_b0 * ddt_scale), (eq31_e491_d_b1 * ddt_scale), (eq31_e491_d_b2 * ddt_scale), (eq31_e491_d_b3 * ddt_scale), (eq31_e491_d_b4 * ddt_scale), (eq31_e491_d_b5 * ddt_scale), (eq31_e491_d_b6 * ddt_scale), (eq31_e491_d_b7 * ddt_scale), (eq31_e491_d_b8 * ddt_scale), (eq31_e491_d_b9 * ddt_scale), (eq31_e491_d_b10 * ddt_scale), (eq31_e491_d_b11 * ddt_scale), (eq31_e491_d_b12 * ddt_scale), (eq31_e491_d_b13 * ddt_scale), (eq31_e491_d_b14 * ddt_scale), (eq31_e491_d_b15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e494;let eq31_node_derivatives: [f64; 19] = [eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18];let eq31_branch_derivatives: [f64; 16] = [eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e501, eq32_e501_d_n10,) = {
    if (!s.b[1850]) {
        let eq32_e499: f64 = ((nv10 - 0.0) * 10000.0);
        (eq32_e499, 10000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e501;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq32_value),
            10,
            multiplicity * (eq32_e501_d_n10),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq33_e509, eq33_e509_d_n0, eq33_e509_d_n1, eq33_e509_d_n2, eq33_e509_d_n3, eq33_e509_d_n4, eq33_e509_d_n5, eq33_e509_d_n6, eq33_e509_d_n7, eq33_e509_d_n8, eq33_e509_d_n9, eq33_e509_d_n10, eq33_e509_d_n11, eq33_e509_d_n12, eq33_e509_d_n13, eq33_e509_d_n14, eq33_e509_d_n15, eq33_e509_d_n16, eq33_e509_d_n17, eq33_e509_d_n18, eq33_e509_d_b0, eq33_e509_d_b1, eq33_e509_d_b2, eq33_e509_d_b3, eq33_e509_d_b4, eq33_e509_d_b5, eq33_e509_d_b6, eq33_e509_d_b7, eq33_e509_d_b8, eq33_e509_d_b9, eq33_e509_d_b10, eq33_e509_d_b11, eq33_e509_d_b12, eq33_e509_d_b13, eq33_e509_d_b14, eq33_e509_d_b15,) = {
    if s.b[1851] {
        let eq33_e506: f64 = (s.v[311] + s.v[263]);let eq33_e506_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);let eq33_e506_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);let eq33_e506_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);let eq33_e506_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);let eq33_e506_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);let eq33_e506_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);let eq33_e506_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);let eq33_e506_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);let eq33_e506_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);let eq33_e506_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);let eq33_e506_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);let eq33_e506_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);let eq33_e506_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);let eq33_e506_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);let eq33_e506_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);let eq33_e506_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);let eq33_e506_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);let eq33_e506_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);let eq33_e506_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);let eq33_e506_d_b0: f64 = (s.db[311][0] + s.db[263][0]);let eq33_e506_d_b1: f64 = (s.db[311][1] + s.db[263][1]);let eq33_e506_d_b2: f64 = (s.db[311][2] + s.db[263][2]);let eq33_e506_d_b3: f64 = (s.db[311][3] + s.db[263][3]);let eq33_e506_d_b4: f64 = (s.db[311][4] + s.db[263][4]);let eq33_e506_d_b5: f64 = (s.db[311][5] + s.db[263][5]);let eq33_e506_d_b6: f64 = (s.db[311][6] + s.db[263][6]);let eq33_e506_d_b7: f64 = (s.db[311][7] + s.db[263][7]);let eq33_e506_d_b8: f64 = (s.db[311][8] + s.db[263][8]);let eq33_e506_d_b9: f64 = (s.db[311][9] + s.db[263][9]);let eq33_e506_d_b10: f64 = (s.db[311][10] + s.db[263][10]);let eq33_e506_d_b11: f64 = (s.db[311][11] + s.db[263][11]);let eq33_e506_d_b12: f64 = (s.db[311][12] + s.db[263][12]);let eq33_e506_d_b13: f64 = (s.db[311][13] + s.db[263][13]);let eq33_e506_d_b14: f64 = (s.db[311][14] + s.db[263][14]);let eq33_e506_d_b15: f64 = (s.db[311][15] + s.db[263][15]);let eq33_e507: f64 = (p.p50 * eq33_e506);let eq33_e507_d_n0: f64 = (p.p50 * eq33_e506_d_n0);let eq33_e507_d_n1: f64 = (p.p50 * eq33_e506_d_n1);let eq33_e507_d_n2: f64 = (p.p50 * eq33_e506_d_n2);let eq33_e507_d_n3: f64 = (p.p50 * eq33_e506_d_n3);let eq33_e507_d_n4: f64 = (p.p50 * eq33_e506_d_n4);let eq33_e507_d_n5: f64 = (p.p50 * eq33_e506_d_n5);let eq33_e507_d_n6: f64 = (p.p50 * eq33_e506_d_n6);let eq33_e507_d_n7: f64 = (p.p50 * eq33_e506_d_n7);let eq33_e507_d_n8: f64 = (p.p50 * eq33_e506_d_n8);let eq33_e507_d_n9: f64 = (p.p50 * eq33_e506_d_n9);let eq33_e507_d_n10: f64 = (p.p50 * eq33_e506_d_n10);let eq33_e507_d_n11: f64 = (p.p50 * eq33_e506_d_n11);let eq33_e507_d_n12: f64 = (p.p50 * eq33_e506_d_n12);let eq33_e507_d_n13: f64 = (p.p50 * eq33_e506_d_n13);let eq33_e507_d_n14: f64 = (p.p50 * eq33_e506_d_n14);let eq33_e507_d_n15: f64 = (p.p50 * eq33_e506_d_n15);let eq33_e507_d_n16: f64 = (p.p50 * eq33_e506_d_n16);let eq33_e507_d_n17: f64 = (p.p50 * eq33_e506_d_n17);let eq33_e507_d_n18: f64 = (p.p50 * eq33_e506_d_n18);let eq33_e507_d_b0: f64 = (p.p50 * eq33_e506_d_b0);let eq33_e507_d_b1: f64 = (p.p50 * eq33_e506_d_b1);let eq33_e507_d_b2: f64 = (p.p50 * eq33_e506_d_b2);let eq33_e507_d_b3: f64 = (p.p50 * eq33_e506_d_b3);let eq33_e507_d_b4: f64 = (p.p50 * eq33_e506_d_b4);let eq33_e507_d_b5: f64 = (p.p50 * eq33_e506_d_b5);let eq33_e507_d_b6: f64 = (p.p50 * eq33_e506_d_b6);let eq33_e507_d_b7: f64 = (p.p50 * eq33_e506_d_b7);let eq33_e507_d_b8: f64 = (p.p50 * eq33_e506_d_b8);let eq33_e507_d_b9: f64 = (p.p50 * eq33_e506_d_b9);let eq33_e507_d_b10: f64 = (p.p50 * eq33_e506_d_b10);let eq33_e507_d_b11: f64 = (p.p50 * eq33_e506_d_b11);let eq33_e507_d_b12: f64 = (p.p50 * eq33_e506_d_b12);let eq33_e507_d_b13: f64 = (p.p50 * eq33_e506_d_b13);let eq33_e507_d_b14: f64 = (p.p50 * eq33_e506_d_b14);let eq33_e507_d_b15: f64 = (p.p50 * eq33_e506_d_b15);
        (eq33_e507, eq33_e507_d_n0, eq33_e507_d_n1, eq33_e507_d_n2, eq33_e507_d_n3, eq33_e507_d_n4, eq33_e507_d_n5, eq33_e507_d_n6, eq33_e507_d_n7, eq33_e507_d_n8, eq33_e507_d_n9, eq33_e507_d_n10, eq33_e507_d_n11, eq33_e507_d_n12, eq33_e507_d_n13, eq33_e507_d_n14, eq33_e507_d_n15, eq33_e507_d_n16, eq33_e507_d_n17, eq33_e507_d_n18, eq33_e507_d_b0, eq33_e507_d_b1, eq33_e507_d_b2, eq33_e507_d_b3, eq33_e507_d_b4, eq33_e507_d_b5, eq33_e507_d_b6, eq33_e507_d_b7, eq33_e507_d_b8, eq33_e507_d_b9, eq33_e507_d_b10, eq33_e507_d_b11, eq33_e507_d_b12, eq33_e507_d_b13, eq33_e507_d_b14, eq33_e507_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e509;let eq33_node_derivatives: [f64; 19] = [eq33_e509_d_n0, eq33_e509_d_n1, eq33_e509_d_n2, eq33_e509_d_n3, eq33_e509_d_n4, eq33_e509_d_n5, eq33_e509_d_n6, eq33_e509_d_n7, eq33_e509_d_n8, eq33_e509_d_n9, eq33_e509_d_n10, eq33_e509_d_n11, eq33_e509_d_n12, eq33_e509_d_n13, eq33_e509_d_n14, eq33_e509_d_n15, eq33_e509_d_n16, eq33_e509_d_n17, eq33_e509_d_n18];let eq33_branch_derivatives: [f64; 16] = [eq33_e509_d_b0, eq33_e509_d_b1, eq33_e509_d_b2, eq33_e509_d_b3, eq33_e509_d_b4, eq33_e509_d_b5, eq33_e509_d_b6, eq33_e509_d_b7, eq33_e509_d_b8, eq33_e509_d_b9, eq33_e509_d_b10, eq33_e509_d_b11, eq33_e509_d_b12, eq33_e509_d_b13, eq33_e509_d_b14, eq33_e509_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
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
        let (eq34_e517, eq34_e517_d_n0, eq34_e517_d_n1, eq34_e517_d_n2, eq34_e517_d_n3, eq34_e517_d_n4, eq34_e517_d_n5, eq34_e517_d_n6, eq34_e517_d_n7, eq34_e517_d_n8, eq34_e517_d_n9, eq34_e517_d_n10, eq34_e517_d_n11, eq34_e517_d_n12, eq34_e517_d_n13, eq34_e517_d_n14, eq34_e517_d_n15, eq34_e517_d_n16, eq34_e517_d_n17, eq34_e517_d_n18, eq34_e517_d_b0, eq34_e517_d_b1, eq34_e517_d_b2, eq34_e517_d_b3, eq34_e517_d_b4, eq34_e517_d_b5, eq34_e517_d_b6, eq34_e517_d_b7, eq34_e517_d_b8, eq34_e517_d_b9, eq34_e517_d_b10, eq34_e517_d_b11, eq34_e517_d_b12, eq34_e517_d_b13, eq34_e517_d_b14, eq34_e517_d_b15,) = {
    if s.b[1851] {
        let eq34_e514: f64 = (s.v[312] + s.v[573]);let eq34_e514_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);let eq34_e514_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);let eq34_e514_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);let eq34_e514_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);let eq34_e514_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);let eq34_e514_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);let eq34_e514_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);let eq34_e514_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);let eq34_e514_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);let eq34_e514_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);let eq34_e514_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);let eq34_e514_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);let eq34_e514_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);let eq34_e514_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);let eq34_e514_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);let eq34_e514_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);let eq34_e514_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);let eq34_e514_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);let eq34_e514_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);let eq34_e514_d_b0: f64 = (s.db[312][0] + s.db[573][0]);let eq34_e514_d_b1: f64 = (s.db[312][1] + s.db[573][1]);let eq34_e514_d_b2: f64 = (s.db[312][2] + s.db[573][2]);let eq34_e514_d_b3: f64 = (s.db[312][3] + s.db[573][3]);let eq34_e514_d_b4: f64 = (s.db[312][4] + s.db[573][4]);let eq34_e514_d_b5: f64 = (s.db[312][5] + s.db[573][5]);let eq34_e514_d_b6: f64 = (s.db[312][6] + s.db[573][6]);let eq34_e514_d_b7: f64 = (s.db[312][7] + s.db[573][7]);let eq34_e514_d_b8: f64 = (s.db[312][8] + s.db[573][8]);let eq34_e514_d_b9: f64 = (s.db[312][9] + s.db[573][9]);let eq34_e514_d_b10: f64 = (s.db[312][10] + s.db[573][10]);let eq34_e514_d_b11: f64 = (s.db[312][11] + s.db[573][11]);let eq34_e514_d_b12: f64 = (s.db[312][12] + s.db[573][12]);let eq34_e514_d_b13: f64 = (s.db[312][13] + s.db[573][13]);let eq34_e514_d_b14: f64 = (s.db[312][14] + s.db[573][14]);let eq34_e514_d_b15: f64 = (s.db[312][15] + s.db[573][15]);let eq34_e515: f64 = (p.p50 * eq34_e514);let eq34_e515_d_n0: f64 = (p.p50 * eq34_e514_d_n0);let eq34_e515_d_n1: f64 = (p.p50 * eq34_e514_d_n1);let eq34_e515_d_n2: f64 = (p.p50 * eq34_e514_d_n2);let eq34_e515_d_n3: f64 = (p.p50 * eq34_e514_d_n3);let eq34_e515_d_n4: f64 = (p.p50 * eq34_e514_d_n4);let eq34_e515_d_n5: f64 = (p.p50 * eq34_e514_d_n5);let eq34_e515_d_n6: f64 = (p.p50 * eq34_e514_d_n6);let eq34_e515_d_n7: f64 = (p.p50 * eq34_e514_d_n7);let eq34_e515_d_n8: f64 = (p.p50 * eq34_e514_d_n8);let eq34_e515_d_n9: f64 = (p.p50 * eq34_e514_d_n9);let eq34_e515_d_n10: f64 = (p.p50 * eq34_e514_d_n10);let eq34_e515_d_n11: f64 = (p.p50 * eq34_e514_d_n11);let eq34_e515_d_n12: f64 = (p.p50 * eq34_e514_d_n12);let eq34_e515_d_n13: f64 = (p.p50 * eq34_e514_d_n13);let eq34_e515_d_n14: f64 = (p.p50 * eq34_e514_d_n14);let eq34_e515_d_n15: f64 = (p.p50 * eq34_e514_d_n15);let eq34_e515_d_n16: f64 = (p.p50 * eq34_e514_d_n16);let eq34_e515_d_n17: f64 = (p.p50 * eq34_e514_d_n17);let eq34_e515_d_n18: f64 = (p.p50 * eq34_e514_d_n18);let eq34_e515_d_b0: f64 = (p.p50 * eq34_e514_d_b0);let eq34_e515_d_b1: f64 = (p.p50 * eq34_e514_d_b1);let eq34_e515_d_b2: f64 = (p.p50 * eq34_e514_d_b2);let eq34_e515_d_b3: f64 = (p.p50 * eq34_e514_d_b3);let eq34_e515_d_b4: f64 = (p.p50 * eq34_e514_d_b4);let eq34_e515_d_b5: f64 = (p.p50 * eq34_e514_d_b5);let eq34_e515_d_b6: f64 = (p.p50 * eq34_e514_d_b6);let eq34_e515_d_b7: f64 = (p.p50 * eq34_e514_d_b7);let eq34_e515_d_b8: f64 = (p.p50 * eq34_e514_d_b8);let eq34_e515_d_b9: f64 = (p.p50 * eq34_e514_d_b9);let eq34_e515_d_b10: f64 = (p.p50 * eq34_e514_d_b10);let eq34_e515_d_b11: f64 = (p.p50 * eq34_e514_d_b11);let eq34_e515_d_b12: f64 = (p.p50 * eq34_e514_d_b12);let eq34_e515_d_b13: f64 = (p.p50 * eq34_e514_d_b13);let eq34_e515_d_b14: f64 = (p.p50 * eq34_e514_d_b14);let eq34_e515_d_b15: f64 = (p.p50 * eq34_e514_d_b15);
        (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18, eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_d_b12, eq34_e515_d_b13, eq34_e515_d_b14, eq34_e515_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e517;let eq34_node_derivatives: [f64; 19] = [eq34_e517_d_n0, eq34_e517_d_n1, eq34_e517_d_n2, eq34_e517_d_n3, eq34_e517_d_n4, eq34_e517_d_n5, eq34_e517_d_n6, eq34_e517_d_n7, eq34_e517_d_n8, eq34_e517_d_n9, eq34_e517_d_n10, eq34_e517_d_n11, eq34_e517_d_n12, eq34_e517_d_n13, eq34_e517_d_n14, eq34_e517_d_n15, eq34_e517_d_n16, eq34_e517_d_n17, eq34_e517_d_n18];let eq34_branch_derivatives: [f64; 16] = [eq34_e517_d_b0, eq34_e517_d_b1, eq34_e517_d_b2, eq34_e517_d_b3, eq34_e517_d_b4, eq34_e517_d_b5, eq34_e517_d_b6, eq34_e517_d_b7, eq34_e517_d_b8, eq34_e517_d_b9, eq34_e517_d_b10, eq34_e517_d_b11, eq34_e517_d_b12, eq34_e517_d_b13, eq34_e517_d_b14, eq34_e517_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
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
        let (eq35_e526, eq35_e526_d_n0, eq35_e526_d_n1, eq35_e526_d_n2, eq35_e526_d_n3, eq35_e526_d_n4, eq35_e526_d_n5, eq35_e526_d_n6, eq35_e526_d_n7, eq35_e526_d_n8, eq35_e526_d_n9, eq35_e526_d_n10, eq35_e526_d_n11, eq35_e526_d_n12, eq35_e526_d_n13, eq35_e526_d_n14, eq35_e526_d_n15, eq35_e526_d_n16, eq35_e526_d_n17, eq35_e526_d_n18, eq35_e526_d_b0, eq35_e526_d_b1, eq35_e526_d_b2, eq35_e526_d_b3, eq35_e526_d_b4, eq35_e526_d_b5, eq35_e526_d_b6, eq35_e526_d_b7, eq35_e526_d_b8, eq35_e526_d_b9, eq35_e526_d_b10, eq35_e526_d_b11, eq35_e526_d_b12, eq35_e526_d_b13, eq35_e526_d_b14, eq35_e526_d_b15,) = {
    if s.b[1851] {
        let eq35_e522: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[283]);let eq35_e523: f64 = (s.v[281] + eq35_e522);let eq35_e523_d_n0: f64 = (s.dn[281][0] + (s.dn[283][0] * ddt_scale));let eq35_e523_d_n1: f64 = (s.dn[281][1] + (s.dn[283][1] * ddt_scale));let eq35_e523_d_n2: f64 = (s.dn[281][2] + (s.dn[283][2] * ddt_scale));let eq35_e523_d_n3: f64 = (s.dn[281][3] + (s.dn[283][3] * ddt_scale));let eq35_e523_d_n4: f64 = (s.dn[281][4] + (s.dn[283][4] * ddt_scale));let eq35_e523_d_n5: f64 = (s.dn[281][5] + (s.dn[283][5] * ddt_scale));let eq35_e523_d_n6: f64 = (s.dn[281][6] + (s.dn[283][6] * ddt_scale));let eq35_e523_d_n7: f64 = (s.dn[281][7] + (s.dn[283][7] * ddt_scale));let eq35_e523_d_n8: f64 = (s.dn[281][8] + (s.dn[283][8] * ddt_scale));let eq35_e523_d_n9: f64 = (s.dn[281][9] + (s.dn[283][9] * ddt_scale));let eq35_e523_d_n10: f64 = (s.dn[281][10] + (s.dn[283][10] * ddt_scale));let eq35_e523_d_n11: f64 = (s.dn[281][11] + (s.dn[283][11] * ddt_scale));let eq35_e523_d_n12: f64 = (s.dn[281][12] + (s.dn[283][12] * ddt_scale));let eq35_e523_d_n13: f64 = (s.dn[281][13] + (s.dn[283][13] * ddt_scale));let eq35_e523_d_n14: f64 = (s.dn[281][14] + (s.dn[283][14] * ddt_scale));let eq35_e523_d_n15: f64 = (s.dn[281][15] + (s.dn[283][15] * ddt_scale));let eq35_e523_d_n16: f64 = (s.dn[281][16] + (s.dn[283][16] * ddt_scale));let eq35_e523_d_n17: f64 = (s.dn[281][17] + (s.dn[283][17] * ddt_scale));let eq35_e523_d_n18: f64 = (s.dn[281][18] + (s.dn[283][18] * ddt_scale));let eq35_e523_d_b0: f64 = (s.db[281][0] + (s.db[283][0] * ddt_scale));let eq35_e523_d_b1: f64 = (s.db[281][1] + (s.db[283][1] * ddt_scale));let eq35_e523_d_b2: f64 = (s.db[281][2] + (s.db[283][2] * ddt_scale));let eq35_e523_d_b3: f64 = (s.db[281][3] + (s.db[283][3] * ddt_scale));let eq35_e523_d_b4: f64 = (s.db[281][4] + (s.db[283][4] * ddt_scale));let eq35_e523_d_b5: f64 = (s.db[281][5] + (s.db[283][5] * ddt_scale));let eq35_e523_d_b6: f64 = (s.db[281][6] + (s.db[283][6] * ddt_scale));let eq35_e523_d_b7: f64 = (s.db[281][7] + (s.db[283][7] * ddt_scale));let eq35_e523_d_b8: f64 = (s.db[281][8] + (s.db[283][8] * ddt_scale));let eq35_e523_d_b9: f64 = (s.db[281][9] + (s.db[283][9] * ddt_scale));let eq35_e523_d_b10: f64 = (s.db[281][10] + (s.db[283][10] * ddt_scale));let eq35_e523_d_b11: f64 = (s.db[281][11] + (s.db[283][11] * ddt_scale));let eq35_e523_d_b12: f64 = (s.db[281][12] + (s.db[283][12] * ddt_scale));let eq35_e523_d_b13: f64 = (s.db[281][13] + (s.db[283][13] * ddt_scale));let eq35_e523_d_b14: f64 = (s.db[281][14] + (s.db[283][14] * ddt_scale));let eq35_e523_d_b15: f64 = (s.db[281][15] + (s.db[283][15] * ddt_scale));let eq35_e524: f64 = (p.p50 * eq35_e523);let eq35_e524_d_n0: f64 = (p.p50 * eq35_e523_d_n0);let eq35_e524_d_n1: f64 = (p.p50 * eq35_e523_d_n1);let eq35_e524_d_n2: f64 = (p.p50 * eq35_e523_d_n2);let eq35_e524_d_n3: f64 = (p.p50 * eq35_e523_d_n3);let eq35_e524_d_n4: f64 = (p.p50 * eq35_e523_d_n4);let eq35_e524_d_n5: f64 = (p.p50 * eq35_e523_d_n5);let eq35_e524_d_n6: f64 = (p.p50 * eq35_e523_d_n6);let eq35_e524_d_n7: f64 = (p.p50 * eq35_e523_d_n7);let eq35_e524_d_n8: f64 = (p.p50 * eq35_e523_d_n8);let eq35_e524_d_n9: f64 = (p.p50 * eq35_e523_d_n9);let eq35_e524_d_n10: f64 = (p.p50 * eq35_e523_d_n10);let eq35_e524_d_n11: f64 = (p.p50 * eq35_e523_d_n11);let eq35_e524_d_n12: f64 = (p.p50 * eq35_e523_d_n12);let eq35_e524_d_n13: f64 = (p.p50 * eq35_e523_d_n13);let eq35_e524_d_n14: f64 = (p.p50 * eq35_e523_d_n14);let eq35_e524_d_n15: f64 = (p.p50 * eq35_e523_d_n15);let eq35_e524_d_n16: f64 = (p.p50 * eq35_e523_d_n16);let eq35_e524_d_n17: f64 = (p.p50 * eq35_e523_d_n17);let eq35_e524_d_n18: f64 = (p.p50 * eq35_e523_d_n18);let eq35_e524_d_b0: f64 = (p.p50 * eq35_e523_d_b0);let eq35_e524_d_b1: f64 = (p.p50 * eq35_e523_d_b1);let eq35_e524_d_b2: f64 = (p.p50 * eq35_e523_d_b2);let eq35_e524_d_b3: f64 = (p.p50 * eq35_e523_d_b3);
        let eq35_e524_d_b4: f64 = (p.p50 * eq35_e523_d_b4);let eq35_e524_d_b5: f64 = (p.p50 * eq35_e523_d_b5);let eq35_e524_d_b6: f64 = (p.p50 * eq35_e523_d_b6);let eq35_e524_d_b7: f64 = (p.p50 * eq35_e523_d_b7);let eq35_e524_d_b8: f64 = (p.p50 * eq35_e523_d_b8);let eq35_e524_d_b9: f64 = (p.p50 * eq35_e523_d_b9);let eq35_e524_d_b10: f64 = (p.p50 * eq35_e523_d_b10);let eq35_e524_d_b11: f64 = (p.p50 * eq35_e523_d_b11);let eq35_e524_d_b12: f64 = (p.p50 * eq35_e523_d_b12);let eq35_e524_d_b13: f64 = (p.p50 * eq35_e523_d_b13);let eq35_e524_d_b14: f64 = (p.p50 * eq35_e523_d_b14);let eq35_e524_d_b15: f64 = (p.p50 * eq35_e523_d_b15);
        (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14, eq35_e524_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e526;let eq35_node_derivatives: [f64; 19] = [eq35_e526_d_n0, eq35_e526_d_n1, eq35_e526_d_n2, eq35_e526_d_n3, eq35_e526_d_n4, eq35_e526_d_n5, eq35_e526_d_n6, eq35_e526_d_n7, eq35_e526_d_n8, eq35_e526_d_n9, eq35_e526_d_n10, eq35_e526_d_n11, eq35_e526_d_n12, eq35_e526_d_n13, eq35_e526_d_n14, eq35_e526_d_n15, eq35_e526_d_n16, eq35_e526_d_n17, eq35_e526_d_n18];let eq35_branch_derivatives: [f64; 16] = [eq35_e526_d_b0, eq35_e526_d_b1, eq35_e526_d_b2, eq35_e526_d_b3, eq35_e526_d_b4, eq35_e526_d_b5, eq35_e526_d_b6, eq35_e526_d_b7, eq35_e526_d_b8, eq35_e526_d_b9, eq35_e526_d_b10, eq35_e526_d_b11, eq35_e526_d_b12, eq35_e526_d_b13, eq35_e526_d_b14, eq35_e526_d_b15];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
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
        let (eq36_e535, eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18, eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14, eq36_e535_d_b15,) = {
    if s.b[1851] {
        let eq36_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[284]);let eq36_e532: f64 = (s.v[282] + eq36_e531);let eq36_e532_d_n0: f64 = (s.dn[282][0] + (s.dn[284][0] * ddt_scale));let eq36_e532_d_n1: f64 = (s.dn[282][1] + (s.dn[284][1] * ddt_scale));let eq36_e532_d_n2: f64 = (s.dn[282][2] + (s.dn[284][2] * ddt_scale));let eq36_e532_d_n3: f64 = (s.dn[282][3] + (s.dn[284][3] * ddt_scale));let eq36_e532_d_n4: f64 = (s.dn[282][4] + (s.dn[284][4] * ddt_scale));let eq36_e532_d_n5: f64 = (s.dn[282][5] + (s.dn[284][5] * ddt_scale));let eq36_e532_d_n6: f64 = (s.dn[282][6] + (s.dn[284][6] * ddt_scale));let eq36_e532_d_n7: f64 = (s.dn[282][7] + (s.dn[284][7] * ddt_scale));let eq36_e532_d_n8: f64 = (s.dn[282][8] + (s.dn[284][8] * ddt_scale));let eq36_e532_d_n9: f64 = (s.dn[282][9] + (s.dn[284][9] * ddt_scale));let eq36_e532_d_n10: f64 = (s.dn[282][10] + (s.dn[284][10] * ddt_scale));let eq36_e532_d_n11: f64 = (s.dn[282][11] + (s.dn[284][11] * ddt_scale));let eq36_e532_d_n12: f64 = (s.dn[282][12] + (s.dn[284][12] * ddt_scale));let eq36_e532_d_n13: f64 = (s.dn[282][13] + (s.dn[284][13] * ddt_scale));let eq36_e532_d_n14: f64 = (s.dn[282][14] + (s.dn[284][14] * ddt_scale));let eq36_e532_d_n15: f64 = (s.dn[282][15] + (s.dn[284][15] * ddt_scale));let eq36_e532_d_n16: f64 = (s.dn[282][16] + (s.dn[284][16] * ddt_scale));let eq36_e532_d_n17: f64 = (s.dn[282][17] + (s.dn[284][17] * ddt_scale));let eq36_e532_d_n18: f64 = (s.dn[282][18] + (s.dn[284][18] * ddt_scale));let eq36_e532_d_b0: f64 = (s.db[282][0] + (s.db[284][0] * ddt_scale));let eq36_e532_d_b1: f64 = (s.db[282][1] + (s.db[284][1] * ddt_scale));let eq36_e532_d_b2: f64 = (s.db[282][2] + (s.db[284][2] * ddt_scale));let eq36_e532_d_b3: f64 = (s.db[282][3] + (s.db[284][3] * ddt_scale));let eq36_e532_d_b4: f64 = (s.db[282][4] + (s.db[284][4] * ddt_scale));let eq36_e532_d_b5: f64 = (s.db[282][5] + (s.db[284][5] * ddt_scale));let eq36_e532_d_b6: f64 = (s.db[282][6] + (s.db[284][6] * ddt_scale));let eq36_e532_d_b7: f64 = (s.db[282][7] + (s.db[284][7] * ddt_scale));let eq36_e532_d_b8: f64 = (s.db[282][8] + (s.db[284][8] * ddt_scale));let eq36_e532_d_b9: f64 = (s.db[282][9] + (s.db[284][9] * ddt_scale));let eq36_e532_d_b10: f64 = (s.db[282][10] + (s.db[284][10] * ddt_scale));let eq36_e532_d_b11: f64 = (s.db[282][11] + (s.db[284][11] * ddt_scale));let eq36_e532_d_b12: f64 = (s.db[282][12] + (s.db[284][12] * ddt_scale));let eq36_e532_d_b13: f64 = (s.db[282][13] + (s.db[284][13] * ddt_scale));let eq36_e532_d_b14: f64 = (s.db[282][14] + (s.db[284][14] * ddt_scale));let eq36_e532_d_b15: f64 = (s.db[282][15] + (s.db[284][15] * ddt_scale));let eq36_e533: f64 = (p.p50 * eq36_e532);let eq36_e533_d_n0: f64 = (p.p50 * eq36_e532_d_n0);let eq36_e533_d_n1: f64 = (p.p50 * eq36_e532_d_n1);let eq36_e533_d_n2: f64 = (p.p50 * eq36_e532_d_n2);let eq36_e533_d_n3: f64 = (p.p50 * eq36_e532_d_n3);let eq36_e533_d_n4: f64 = (p.p50 * eq36_e532_d_n4);let eq36_e533_d_n5: f64 = (p.p50 * eq36_e532_d_n5);let eq36_e533_d_n6: f64 = (p.p50 * eq36_e532_d_n6);let eq36_e533_d_n7: f64 = (p.p50 * eq36_e532_d_n7);let eq36_e533_d_n8: f64 = (p.p50 * eq36_e532_d_n8);let eq36_e533_d_n9: f64 = (p.p50 * eq36_e532_d_n9);let eq36_e533_d_n10: f64 = (p.p50 * eq36_e532_d_n10);let eq36_e533_d_n11: f64 = (p.p50 * eq36_e532_d_n11);let eq36_e533_d_n12: f64 = (p.p50 * eq36_e532_d_n12);let eq36_e533_d_n13: f64 = (p.p50 * eq36_e532_d_n13);let eq36_e533_d_n14: f64 = (p.p50 * eq36_e532_d_n14);let eq36_e533_d_n15: f64 = (p.p50 * eq36_e532_d_n15);let eq36_e533_d_n16: f64 = (p.p50 * eq36_e532_d_n16);let eq36_e533_d_n17: f64 = (p.p50 * eq36_e532_d_n17);let eq36_e533_d_n18: f64 = (p.p50 * eq36_e532_d_n18);let eq36_e533_d_b0: f64 = (p.p50 * eq36_e532_d_b0);let eq36_e533_d_b1: f64 = (p.p50 * eq36_e532_d_b1);let eq36_e533_d_b2: f64 = (p.p50 * eq36_e532_d_b2);let eq36_e533_d_b3: f64 = (p.p50 * eq36_e532_d_b3);
        let eq36_e533_d_b4: f64 = (p.p50 * eq36_e532_d_b4);let eq36_e533_d_b5: f64 = (p.p50 * eq36_e532_d_b5);let eq36_e533_d_b6: f64 = (p.p50 * eq36_e532_d_b6);let eq36_e533_d_b7: f64 = (p.p50 * eq36_e532_d_b7);let eq36_e533_d_b8: f64 = (p.p50 * eq36_e532_d_b8);let eq36_e533_d_b9: f64 = (p.p50 * eq36_e532_d_b9);let eq36_e533_d_b10: f64 = (p.p50 * eq36_e532_d_b10);let eq36_e533_d_b11: f64 = (p.p50 * eq36_e532_d_b11);let eq36_e533_d_b12: f64 = (p.p50 * eq36_e532_d_b12);let eq36_e533_d_b13: f64 = (p.p50 * eq36_e532_d_b13);let eq36_e533_d_b14: f64 = (p.p50 * eq36_e532_d_b14);let eq36_e533_d_b15: f64 = (p.p50 * eq36_e532_d_b15);
        (eq36_e533, eq36_e533_d_n0, eq36_e533_d_n1, eq36_e533_d_n2, eq36_e533_d_n3, eq36_e533_d_n4, eq36_e533_d_n5, eq36_e533_d_n6, eq36_e533_d_n7, eq36_e533_d_n8, eq36_e533_d_n9, eq36_e533_d_n10, eq36_e533_d_n11, eq36_e533_d_n12, eq36_e533_d_n13, eq36_e533_d_n14, eq36_e533_d_n15, eq36_e533_d_n16, eq36_e533_d_n17, eq36_e533_d_n18, eq36_e533_d_b0, eq36_e533_d_b1, eq36_e533_d_b2, eq36_e533_d_b3, eq36_e533_d_b4, eq36_e533_d_b5, eq36_e533_d_b6, eq36_e533_d_b7, eq36_e533_d_b8, eq36_e533_d_b9, eq36_e533_d_b10, eq36_e533_d_b11, eq36_e533_d_b12, eq36_e533_d_b13, eq36_e533_d_b14, eq36_e533_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e535;let eq36_node_derivatives: [f64; 19] = [eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18];let eq36_branch_derivatives: [f64; 16] = [eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14, eq36_e535_d_b15];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
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
        let (eq37_e543, eq37_e543_d_n0, eq37_e543_d_n1, eq37_e543_d_n2, eq37_e543_d_n3, eq37_e543_d_n4, eq37_e543_d_n5, eq37_e543_d_n6, eq37_e543_d_n7, eq37_e543_d_n8, eq37_e543_d_n9, eq37_e543_d_n10, eq37_e543_d_n11, eq37_e543_d_n12, eq37_e543_d_n13, eq37_e543_d_n14, eq37_e543_d_n15, eq37_e543_d_n16, eq37_e543_d_n17, eq37_e543_d_n18, eq37_e543_d_b0, eq37_e543_d_b1, eq37_e543_d_b2, eq37_e543_d_b3, eq37_e543_d_b4, eq37_e543_d_b5, eq37_e543_d_b6, eq37_e543_d_b7, eq37_e543_d_b8, eq37_e543_d_b9, eq37_e543_d_b10, eq37_e543_d_b11, eq37_e543_d_b12, eq37_e543_d_b13, eq37_e543_d_b14, eq37_e543_d_b15,) = {
    if (s.b[1851] && (p.p261 != 0.0)) {
        let eq37_e541: f64 = ((nv4 - nv12) / s.v[2]);let eq37_e541_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));let eq37_e541_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));let eq37_e541_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));let eq37_e541_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));let eq37_e541_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));let eq37_e541_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));let eq37_e541_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));let eq37_e541_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));let eq37_e541_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));let eq37_e541_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));let eq37_e541_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));let eq37_e541_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));let eq37_e541_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));let eq37_e541_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));let eq37_e541_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));let eq37_e541_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));let eq37_e541_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));let eq37_e541_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));let eq37_e541_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));let eq37_e541_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));let eq37_e541_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));let eq37_e541_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));let eq37_e541_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));let eq37_e541_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));let eq37_e541_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));let eq37_e541_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));let eq37_e541_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));let eq37_e541_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));let eq37_e541_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));let eq37_e541_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));let eq37_e541_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));let eq37_e541_d_b12: f64 = (-(((nv4 - nv12) * s.db[2][12]) / (s.v[2] * s.v[2])));let eq37_e541_d_b13: f64 = (-(((nv4 - nv12) * s.db[2][13]) / (s.v[2] * s.v[2])));let eq37_e541_d_b14: f64 = (-(((nv4 - nv12) * s.db[2][14]) / (s.v[2] * s.v[2])));let eq37_e541_d_b15: f64 = (-(((nv4 - nv12) * s.db[2][15]) / (s.v[2] * s.v[2])));
        (eq37_e541, eq37_e541_d_n0, eq37_e541_d_n1, eq37_e541_d_n2, eq37_e541_d_n3, eq37_e541_d_n4, eq37_e541_d_n5, eq37_e541_d_n6, eq37_e541_d_n7, eq37_e541_d_n8, eq37_e541_d_n9, eq37_e541_d_n10, eq37_e541_d_n11, eq37_e541_d_n12, eq37_e541_d_n13, eq37_e541_d_n14, eq37_e541_d_n15, eq37_e541_d_n16, eq37_e541_d_n17, eq37_e541_d_n18, eq37_e541_d_b0, eq37_e541_d_b1, eq37_e541_d_b2, eq37_e541_d_b3, eq37_e541_d_b4, eq37_e541_d_b5, eq37_e541_d_b6, eq37_e541_d_b7, eq37_e541_d_b8, eq37_e541_d_b9, eq37_e541_d_b10, eq37_e541_d_b11, eq37_e541_d_b12, eq37_e541_d_b13, eq37_e541_d_b14, eq37_e541_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e543;let eq37_node_derivatives: [f64; 19] = [eq37_e543_d_n0, eq37_e543_d_n1, eq37_e543_d_n2, eq37_e543_d_n3, eq37_e543_d_n4, eq37_e543_d_n5, eq37_e543_d_n6, eq37_e543_d_n7, eq37_e543_d_n8, eq37_e543_d_n9, eq37_e543_d_n10, eq37_e543_d_n11, eq37_e543_d_n12, eq37_e543_d_n13, eq37_e543_d_n14, eq37_e543_d_n15, eq37_e543_d_n16, eq37_e543_d_n17, eq37_e543_d_n18];let eq37_branch_derivatives: [f64; 16] = [eq37_e543_d_b0, eq37_e543_d_b1, eq37_e543_d_b2, eq37_e543_d_b3, eq37_e543_d_b4, eq37_e543_d_b5, eq37_e543_d_b6, eq37_e543_d_b7, eq37_e543_d_b8, eq37_e543_d_b9, eq37_e543_d_b10, eq37_e543_d_b11, eq37_e543_d_b12, eq37_e543_d_b13, eq37_e543_d_b14, eq37_e543_d_b15];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(12),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e550,) = {
    if (s.b[1851] && (p.p261 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e550;
        stamper.stamp_potential_const_local(
            6,
            eq38_value,
        );
        let (eq39_e558, eq39_e558_d_n0, eq39_e558_d_n1, eq39_e558_d_n2, eq39_e558_d_n3, eq39_e558_d_n4, eq39_e558_d_n5, eq39_e558_d_n6, eq39_e558_d_n7, eq39_e558_d_n8, eq39_e558_d_n9, eq39_e558_d_n10, eq39_e558_d_n11, eq39_e558_d_n12, eq39_e558_d_n13, eq39_e558_d_n14, eq39_e558_d_n15, eq39_e558_d_n16, eq39_e558_d_n17, eq39_e558_d_n18, eq39_e558_d_b0, eq39_e558_d_b1, eq39_e558_d_b2, eq39_e558_d_b3, eq39_e558_d_b4, eq39_e558_d_b5, eq39_e558_d_b6, eq39_e558_d_b7, eq39_e558_d_b8, eq39_e558_d_b9, eq39_e558_d_b10, eq39_e558_d_b11, eq39_e558_d_b12, eq39_e558_d_b13, eq39_e558_d_b14, eq39_e558_d_b15,) = {
    if (s.b[1851] && (p.p262 != 0.0)) {
        let eq39_e556: f64 = (s.v[553] * (nv9 - nv12));let eq39_e556_d_n0: f64 = (s.dn[553][0] * (nv9 - nv12));let eq39_e556_d_n1: f64 = (s.dn[553][1] * (nv9 - nv12));let eq39_e556_d_n2: f64 = (s.dn[553][2] * (nv9 - nv12));let eq39_e556_d_n3: f64 = (s.dn[553][3] * (nv9 - nv12));let eq39_e556_d_n4: f64 = (s.dn[553][4] * (nv9 - nv12));let eq39_e556_d_n5: f64 = (s.dn[553][5] * (nv9 - nv12));let eq39_e556_d_n6: f64 = (s.dn[553][6] * (nv9 - nv12));let eq39_e556_d_n7: f64 = (s.dn[553][7] * (nv9 - nv12));let eq39_e556_d_n8: f64 = (s.dn[553][8] * (nv9 - nv12));let eq39_e556_d_n9: f64 = ((s.dn[553][9] * (nv9 - nv12)) + s.v[553]);let eq39_e556_d_n10: f64 = (s.dn[553][10] * (nv9 - nv12));let eq39_e556_d_n11: f64 = (s.dn[553][11] * (nv9 - nv12));let eq39_e556_d_n12: f64 = ((s.dn[553][12] * (nv9 - nv12)) + (-s.v[553]));let eq39_e556_d_n13: f64 = (s.dn[553][13] * (nv9 - nv12));let eq39_e556_d_n14: f64 = (s.dn[553][14] * (nv9 - nv12));let eq39_e556_d_n15: f64 = (s.dn[553][15] * (nv9 - nv12));let eq39_e556_d_n16: f64 = (s.dn[553][16] * (nv9 - nv12));let eq39_e556_d_n17: f64 = (s.dn[553][17] * (nv9 - nv12));let eq39_e556_d_n18: f64 = (s.dn[553][18] * (nv9 - nv12));let eq39_e556_d_b0: f64 = (s.db[553][0] * (nv9 - nv12));let eq39_e556_d_b1: f64 = (s.db[553][1] * (nv9 - nv12));let eq39_e556_d_b2: f64 = (s.db[553][2] * (nv9 - nv12));let eq39_e556_d_b3: f64 = (s.db[553][3] * (nv9 - nv12));let eq39_e556_d_b4: f64 = (s.db[553][4] * (nv9 - nv12));let eq39_e556_d_b5: f64 = (s.db[553][5] * (nv9 - nv12));let eq39_e556_d_b6: f64 = (s.db[553][6] * (nv9 - nv12));let eq39_e556_d_b7: f64 = (s.db[553][7] * (nv9 - nv12));let eq39_e556_d_b8: f64 = (s.db[553][8] * (nv9 - nv12));let eq39_e556_d_b9: f64 = (s.db[553][9] * (nv9 - nv12));let eq39_e556_d_b10: f64 = (s.db[553][10] * (nv9 - nv12));let eq39_e556_d_b11: f64 = (s.db[553][11] * (nv9 - nv12));let eq39_e556_d_b12: f64 = (s.db[553][12] * (nv9 - nv12));let eq39_e556_d_b13: f64 = (s.db[553][13] * (nv9 - nv12));let eq39_e556_d_b14: f64 = (s.db[553][14] * (nv9 - nv12));let eq39_e556_d_b15: f64 = (s.db[553][15] * (nv9 - nv12));
        (eq39_e556, eq39_e556_d_n0, eq39_e556_d_n1, eq39_e556_d_n2, eq39_e556_d_n3, eq39_e556_d_n4, eq39_e556_d_n5, eq39_e556_d_n6, eq39_e556_d_n7, eq39_e556_d_n8, eq39_e556_d_n9, eq39_e556_d_n10, eq39_e556_d_n11, eq39_e556_d_n12, eq39_e556_d_n13, eq39_e556_d_n14, eq39_e556_d_n15, eq39_e556_d_n16, eq39_e556_d_n17, eq39_e556_d_n18, eq39_e556_d_b0, eq39_e556_d_b1, eq39_e556_d_b2, eq39_e556_d_b3, eq39_e556_d_b4, eq39_e556_d_b5, eq39_e556_d_b6, eq39_e556_d_b7, eq39_e556_d_b8, eq39_e556_d_b9, eq39_e556_d_b10, eq39_e556_d_b11, eq39_e556_d_b12, eq39_e556_d_b13, eq39_e556_d_b14, eq39_e556_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e558;let eq39_node_derivatives: [f64; 19] = [eq39_e558_d_n0, eq39_e558_d_n1, eq39_e558_d_n2, eq39_e558_d_n3, eq39_e558_d_n4, eq39_e558_d_n5, eq39_e558_d_n6, eq39_e558_d_n7, eq39_e558_d_n8, eq39_e558_d_n9, eq39_e558_d_n10, eq39_e558_d_n11, eq39_e558_d_n12, eq39_e558_d_n13, eq39_e558_d_n14, eq39_e558_d_n15, eq39_e558_d_n16, eq39_e558_d_n17, eq39_e558_d_n18];let eq39_branch_derivatives: [f64; 16] = [eq39_e558_d_b0, eq39_e558_d_b1, eq39_e558_d_b2, eq39_e558_d_b3, eq39_e558_d_b4, eq39_e558_d_b5, eq39_e558_d_b6, eq39_e558_d_b7, eq39_e558_d_b8, eq39_e558_d_b9, eq39_e558_d_b10, eq39_e558_d_b11, eq39_e558_d_b12, eq39_e558_d_b13, eq39_e558_d_b14, eq39_e558_d_b15];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(12),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
    }
}
