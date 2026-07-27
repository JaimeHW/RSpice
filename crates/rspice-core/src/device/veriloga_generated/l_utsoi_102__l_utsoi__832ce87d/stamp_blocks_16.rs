#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(372, 329, A::add_scaled_products(s.ad_value(236), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1760] = (s.v[6] > 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_mul(374, 170, 215);}
        if (!s.b[1760]) {s.store_scalar(374, 0.0);}
        s.store_scaled_mul(357, 13, 357, p[32]);s.store_scaled_mul(358, 13, 358, p[32]);s.store_scaled_mul(359, 13, 359, p[32]);s.store_add_scaled_inputs3_indices(360, 357, (-1.0), 358, (-1.0), 359, (-1.0));s.store_scaled_mul(375, 13, 375, p[32]);s.store_scaled_mul(376, 13, 376, p[32]);s.store_scaled_mul(377, 13, 377, p[32]);s.store_scaled_mul(378, 13, 378, p[32]);s.store_scaled_mul(366, 13, 366, p[32]);s.store_scaled_mul(367, 13, 367, p[32]);s.store_scaled_mul(368, 13, 368, p[32]);s.store_scaled_mul(369, 13, 369, p[32]);s.store_scaled_mul(370, 13, 370, p[32]);s.store_scaled_mul(373, 13, 373, p[32]);s.store_scaled_mul(372, 13, 372, p[32]);s.store_scaled_mul(371, 13, 371, p[32]);s.store_mul(374, 13, 374);s.b[1769] = (s.v[330] < 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if s.b[1769] {s.copy_ad(1768, 359);s.copy_ad(359, 360);s.copy_ad(360, 1768);s.store_neg(371, 371);s.copy_ad(1768, 376);s.copy_ad(376, 375);s.copy_ad(375, 1768);s.copy_ad(1768, 378);s.copy_ad(378, 377);s.copy_ad(377, 1768);}
        s.store_scaled_mul(1770, 386, 222, 1.0 / (1.602176565e-19));s.store_scaled_add(1771, 403, 428, (-0.5));s.store_add(1772, 411, 1771);s.store_div(0, 411, 1772);s.store_scaled_add_sqrt_square_offset_rhs(1777, 0, 0, 1e-20, 0.5);s.store_scaled_mul(1778, 432, 431, (-0.1666666666667));s.store_square(1779, 1778);s.store_offset(1780, 425, (-1.0));s.store_scale(1784, 1779, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1777, 1.0, 1784, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784), s.ad_value(1780), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1789, 452, 443, 116, 1.0, 465, 1.0);s.store_mul_scale_offset_indices(1790, 1789, 464, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1792, 1790, A::mul_scaled_lhs(s.ad_value(330), 0.25, s.ad_value(1778)), -1.0, 0.5);s.store_sub(1791, 1790, 1792);s.b[1803] = (p[6] > 0.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if s.b[1803] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1777), 0.08333333333333333, s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)), (-1.0)), A::mul3_scaled_output(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784)), s.ad_value(1780), 1.6));s.store_max_with_scalar(3, 2, 1e-40);}
        s.copy_ad(1773, 1770);s.store_mul_scale_offset_indices(1774, 1770, 411, 1.0, 1.0);s.store_mul_sub_rhs(1775, 1770, 399, 409);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5, A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1773), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1773), s.ad_value(1773)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1773), 2.0), 1.0), 1775, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(413), 1.0, s.ad_value(177), s.ad_value(414), 1.0), A::offset(s.ad_value(411), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(343), s.ad_value(344), 1.602176565e-19, s.ad_value(341), 1.0), 3, 1.0, 1773, 1.0);s.store_div_from_scalar_scaled_input(1813, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1814, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1815, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1816, 15, 1815, 1.0, 1814, (-1.0), 224, (-0.4), 0.0);s.store_add(1817, 1814, 1816);s.store_scaled_mul(1818, 1817, 1813, 0.5);s.store_sub_scaled_inputs(1819, 15, 0.05, 1816, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1820, 2, 234);s.store_div_scaled_value_offset_denominator(1821, s.ad_value(1813), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1823, 1820, 225, (2.0 * 1.602176565e-19), 0.0, 1821);s.store_add_offset_lhs_mixed_ai(1824, A::ln(A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(1823), 1.0)), (-0.6931471805599), 1818);s.store_mul_div_scaled_product_mixed_iiia(1825, 1821, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(1828, 35, 1821);s.store_scalar(1829, 0.0);s.store_scalar(1822, 0.0);s.b[1874] = (p[9] > 0.0);s.store_scalar(1874, if s.b[1874] { 1.0 } else { 0.0 });
        if s.b[1874] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1822, 1.0, 1813, A::ln(A::div(s.ad_value(24), s.ad_value(247))));}
        s.b[1875] = (p[13] > 0.0);s.store_scalar(1875, if s.b[1875] { 1.0 } else { 0.0 });s.b[1876] = (p[14] == 1.0);s.store_scalar(1876, if s.b[1876] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1875] && s.b[1876]) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p[13]) * 1.27520989));}
        if (s.b[1875] && (!s.b[1876])) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p[13]) * 1.5412087));}
        s.store_mul(1832, 332, 1821);s.store_mul_scale_offset_mixed_ia(1833, 1821, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(1834, 1832, 1833, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1805, 398, 1.0, 397, A::offset(s.ad_value(398), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 397, 1.0, 398, A::offset(s.ad_value(397), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(397), A::offset(s.ad_value(1805), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(398), A::offset(s.ad_value(1806), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 395, 1805, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(395), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(394), -1.0, s.ad_value(25), 1.0), 394);s.store_add_scaled_product_mixed_iia(1812, 21, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(390), (-1.0), s.ad_value(391), 1.0), 1.0, s.ad_value(393), (-1.0), s.ad_value(390), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p[14]);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(179), p[14], s.ad_value(1819), p[14], s.ad_value(239), p[14], s.ad_value(0), 1.0), p[34], 1822);s.store_add_scaled_inputs4_indices(1831, 180, p[14], 1819, p[14], 240, p[14], 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);s.b[1877] = (p[2] > 0.0);s.store_scalar(1877, if s.b[1877] { 1.0 } else { 0.0 });
        if s.b[1877] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p[14], 256, 1.0);}
        s.b[1878] = (s.v[0] < 0.0);s.store_scalar(1878, if s.b[1878] { 1.0 } else { 0.0 });
        if (s.b[1877] && s.b[1878]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1877] && (!s.b[1878])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if s.b[1877] {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p[14]);}
        if (!s.b[1877]) {s.copy_ad(1837, 1836);}
        s.store_mul_sub_rhs(0, 244, 1835, 1837);s.b[1879] = (p[13] > 0.0);s.store_scalar(1879, if s.b[1879] { 1.0 } else { 0.0 });
        if s.b[1879] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
        if (!s.b[1879]) {s.copy_ad(1841, 242);s.copy_ad(1842, 243);s.copy_ad(1843, 244);}
        s.store_mul_sub_rhs(1844, 1843, 1835, 1837);s.b[1880] = (s.v[1844] > 0.0);s.store_scalar(1880, if s.b[1880] { 1.0 } else { 0.0 });s.b[1881] = ((-s.v[1844]) < 80.0);s.store_scalar(1881, if s.b[1881] { 1.0 } else { 0.0 });
        if (s.b[1880] && s.b[1881]) {s.store_ln_one_plus_exp_neg_input(0, 1844);}
        if (s.b[1880] && (!s.b[1881])) {s.store_neg(0, 1844);}
        if s.b[1880] {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1882] = (s.v[1844] < 80.0);s.store_scalar(1882, if s.b[1882] { 1.0 } else { 0.0 });
        if ((!s.b[1880]) && s.b[1882]) {s.store_ln_one_plus_exp(0, 1844);}
        if ((!s.b[1880]) && (!s.b[1882])) {s.copy_ad(0, 1844);}
        if (!s.b[1880]) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);s.b[1884] = (p[11] > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_div_scaled_value_by_product_mixed_iia(1805, 453, 1.0, 452, A::offset(s.ad_value(453), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 452, 1.0, 453, A::offset(s.ad_value(452), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(452), A::offset(s.ad_value(1805), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(453), A::offset(s.ad_value(1806), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 451, 1805, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(451), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(450), -1.0, s.ad_value(25), 1.0), 450);s.store_add_scaled_product_mixed_iia(1812, 130, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(446), (-1.0), s.ad_value(447), 1.0), 1.0, s.ad_value(449), (-1.0), s.ad_value(446), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p[14]);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(181), p[14], s.ad_value(1819), p[14], s.ad_value(239), p[14], s.ad_value(0), 1.0), p[34], 1822);s.store_add_scaled_inputs4_indices(1831, 182, p[14], 1819, p[14], 240, p[14], 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);}
        s.b[1885] = (p[2] > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1885]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p[14], 256, 1.0);}
        s.b[1886] = (s.v[0] < 0.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if ((s.b[1884] && s.b[1885]) && s.b[1886]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1884] && s.b[1885]) && (!s.b[1886])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if (s.b[1884] && s.b[1885]) {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p[14]);}
        if (s.b[1884] && (!s.b[1885])) {s.copy_ad(1837, 1836);}
        if s.b[1884] {s.store_mul_sub_rhs(0, 244, 1835, 1837);}
        s.b[1887] = (p[13] > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1887]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
        if (s.b[1884] && (!s.b[1887])) {s.copy_ad(1841, 242);s.copy_ad(1842, 243);s.copy_ad(1843, 244);}
        if s.b[1884] {s.store_mul_sub_rhs(1844, 1843, 1835, 1837);}
        s.b[1888] = (s.v[1844] > 0.0);s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });s.b[1889] = ((-s.v[1844]) < 80.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
        if ((s.b[1884] && s.b[1888]) && s.b[1889]) {s.store_ln_one_plus_exp_neg_input(0, 1844);}
        if ((s.b[1884] && s.b[1888]) && (!s.b[1889])) {s.store_neg(0, 1844);}
        if (s.b[1884] && s.b[1888]) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1890] = (s.v[1844] < 80.0);s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });
        if ((s.b[1884] && (!s.b[1888])) && s.b[1890]) {s.store_ln_one_plus_exp(0, 1844);}
        if ((s.b[1884] && (!s.b[1888])) && (!s.b[1890])) {s.copy_ad(0, 1844);}
        if (s.b[1884] && (!s.b[1888])) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));}
        if s.b[1884] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1884] {s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);}
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
        let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq0_e500, eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9, eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3,) = {
    if s.b[1763] {
        let eq0_e498: f64 = (p[14] * s.v[361]);
        (eq0_e498, (p[14] * s.dn[361][0]), (p[14] * s.dn[361][1]), (p[14] * s.dn[361][2]), (p[14] * s.dn[361][3]), (p[14] * s.dn[361][4]), (p[14] * s.dn[361][5]), (p[14] * s.dn[361][6]), (p[14] * s.dn[361][7]), (p[14] * s.dn[361][8]), (p[14] * s.dn[361][9]), (p[14] * s.db[361][0]), (p[14] * s.db[361][1]), (p[14] * s.db[361][2]), (p[14] * s.db[361][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e500;let eq0_node_derivatives: [f64; 10] = [eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9];let eq0_branch_derivatives: [f64; 4] = [eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e507, eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9, eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3,) = {
    if (!s.b[1763]) {
        let eq1_e505: f64 = (p[14] * s.v[361]);
        (eq1_e505, (p[14] * s.dn[361][0]), (p[14] * s.dn[361][1]), (p[14] * s.dn[361][2]), (p[14] * s.dn[361][3]), (p[14] * s.dn[361][4]), (p[14] * s.dn[361][5]), (p[14] * s.dn[361][6]), (p[14] * s.dn[361][7]), (p[14] * s.dn[361][8]), (p[14] * s.dn[361][9]), (p[14] * s.db[361][0]), (p[14] * s.db[361][1]), (p[14] * s.db[361][2]), (p[14] * s.db[361][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e507;let eq1_node_derivatives: [f64; 10] = [eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9];let eq1_branch_derivatives: [f64; 4] = [eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );let eq2_e511: f64 = (s.v[364] - s.v[365]);let eq2_e511_d_n0: f64 = (s.dn[364][0] - s.dn[365][0]);let eq2_e511_d_n1: f64 = (s.dn[364][1] - s.dn[365][1]);let eq2_e511_d_n2: f64 = (s.dn[364][2] - s.dn[365][2]);let eq2_e511_d_n3: f64 = (s.dn[364][3] - s.dn[365][3]);let eq2_e511_d_n4: f64 = (s.dn[364][4] - s.dn[365][4]);let eq2_e511_d_n5: f64 = (s.dn[364][5] - s.dn[365][5]);let eq2_e511_d_n6: f64 = (s.dn[364][6] - s.dn[365][6]);let eq2_e511_d_n7: f64 = (s.dn[364][7] - s.dn[365][7]);let eq2_e511_d_n8: f64 = (s.dn[364][8] - s.dn[365][8]);let eq2_e511_d_n9: f64 = (s.dn[364][9] - s.dn[365][9]);let eq2_e511_d_b0: f64 = (s.db[364][0] - s.db[365][0]);let eq2_e511_d_b1: f64 = (s.db[364][1] - s.db[365][1]);let eq2_e511_d_b2: f64 = (s.db[364][2] - s.db[365][2]);let eq2_e511_d_b3: f64 = (s.db[364][3] - s.db[365][3]);let eq2_e512: f64 = (p[14] * eq2_e511);let eq2_e512_d_n0: f64 = (p[14] * eq2_e511_d_n0);let eq2_e512_d_n1: f64 = (p[14] * eq2_e511_d_n1);let eq2_e512_d_n2: f64 = (p[14] * eq2_e511_d_n2);let eq2_e512_d_n3: f64 = (p[14] * eq2_e511_d_n3);let eq2_e512_d_n4: f64 = (p[14] * eq2_e511_d_n4);let eq2_e512_d_n5: f64 = (p[14] * eq2_e511_d_n5);let eq2_e512_d_n6: f64 = (p[14] * eq2_e511_d_n6);let eq2_e512_d_n7: f64 = (p[14] * eq2_e511_d_n7);let eq2_e512_d_n8: f64 = (p[14] * eq2_e511_d_n8);let eq2_e512_d_n9: f64 = (p[14] * eq2_e511_d_n9);let eq2_e512_d_b0: f64 = (p[14] * eq2_e511_d_b0);let eq2_e512_d_b1: f64 = (p[14] * eq2_e511_d_b1);let eq2_e512_d_b2: f64 = (p[14] * eq2_e511_d_b2);let eq2_e512_d_b3: f64 = (p[14] * eq2_e511_d_b3);let eq2_value: f64 = eq2_e512;let eq2_node_derivatives: [f64; 10] = [eq2_e512_d_n0, eq2_e512_d_n1, eq2_e512_d_n2, eq2_e512_d_n3, eq2_e512_d_n4, eq2_e512_d_n5, eq2_e512_d_n6, eq2_e512_d_n7, eq2_e512_d_n8, eq2_e512_d_n9];let eq2_branch_derivatives: [f64; 4] = [eq2_e512_d_b0, eq2_e512_d_b1, eq2_e512_d_b2, eq2_e512_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );let eq3_e515: f64 = (p[14] * s.v[362]);let eq3_value: f64 = eq3_e515;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &s.dn[362],
            &s.db[362],
            (multiplicity) * (p[14]),
        );let eq4_e518: f64 = (p[14] * s.v[363]);let eq4_value: f64 = eq4_e518;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            &s.dn[363],
            &s.db[363],
            (multiplicity) * (p[14]),
        );let eq8_e524: f64 = (p[31] * s.v[471]);let eq8_e526: f64 = (eq8_e524 * (nv7 - nv6));let eq8_value: f64 = eq8_e526;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * ((-eq8_e524)),
            7,
            multiplicity * (eq8_e524),
        );let eq9_value: f64 = s.v[1761];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq9_value),
            &s.dn[1761],
            &s.db[1761],
            multiplicity,
        );let eq10_value: f64 = s.v[1762];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq10_value),
            &s.dn[1762],
            &s.db[1762],
            multiplicity,
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
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq11_e538, eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9, eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3,) = {
    if s.b[1764] {
        let eq11_e532: f64 = (p[31] * s.v[13]);let eq11_e534: f64 = (eq11_e532 * s.v[312]);let eq11_e534_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[312]) + (eq11_e532 * s.dn[312][0]));let eq11_e534_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[312]) + (eq11_e532 * s.dn[312][1]));let eq11_e534_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[312]) + (eq11_e532 * s.dn[312][2]));let eq11_e534_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[312]) + (eq11_e532 * s.dn[312][3]));let eq11_e534_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[312]) + (eq11_e532 * s.dn[312][4]));let eq11_e534_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[312]) + (eq11_e532 * s.dn[312][5]));let eq11_e534_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[312]) + (eq11_e532 * s.dn[312][6]));let eq11_e534_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[312]) + (eq11_e532 * s.dn[312][7]));let eq11_e534_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[312]) + (eq11_e532 * s.dn[312][8]));let eq11_e534_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[312]) + (eq11_e532 * s.dn[312][9]));let eq11_e534_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[312]) + (eq11_e532 * s.db[312][0]));let eq11_e534_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[312]) + (eq11_e532 * s.db[312][1]));let eq11_e534_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[312]) + (eq11_e532 * s.db[312][2]));let eq11_e534_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[312]) + (eq11_e532 * s.db[312][3]));let eq11_e536: f64 = (eq11_e534 * (nv1 - nv9));let eq11_e536_d_n0: f64 = (eq11_e534_d_n0 * (nv1 - nv9));let eq11_e536_d_n1: f64 = ((eq11_e534_d_n1 * (nv1 - nv9)) + eq11_e534);let eq11_e536_d_n2: f64 = (eq11_e534_d_n2 * (nv1 - nv9));let eq11_e536_d_n3: f64 = (eq11_e534_d_n3 * (nv1 - nv9));let eq11_e536_d_n4: f64 = (eq11_e534_d_n4 * (nv1 - nv9));let eq11_e536_d_n5: f64 = (eq11_e534_d_n5 * (nv1 - nv9));let eq11_e536_d_n6: f64 = (eq11_e534_d_n6 * (nv1 - nv9));let eq11_e536_d_n7: f64 = (eq11_e534_d_n7 * (nv1 - nv9));let eq11_e536_d_n8: f64 = (eq11_e534_d_n8 * (nv1 - nv9));let eq11_e536_d_n9: f64 = ((eq11_e534_d_n9 * (nv1 - nv9)) + (-eq11_e534));let eq11_e536_d_b0: f64 = (eq11_e534_d_b0 * (nv1 - nv9));let eq11_e536_d_b1: f64 = (eq11_e534_d_b1 * (nv1 - nv9));let eq11_e536_d_b2: f64 = (eq11_e534_d_b2 * (nv1 - nv9));let eq11_e536_d_b3: f64 = (eq11_e534_d_b3 * (nv1 - nv9));
        (eq11_e536, eq11_e536_d_n0, eq11_e536_d_n1, eq11_e536_d_n2, eq11_e536_d_n3, eq11_e536_d_n4, eq11_e536_d_n5, eq11_e536_d_n6, eq11_e536_d_n7, eq11_e536_d_n8, eq11_e536_d_n9, eq11_e536_d_b0, eq11_e536_d_b1, eq11_e536_d_b2, eq11_e536_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e538;let eq11_node_derivatives: [f64; 10] = [eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9];let eq11_branch_derivatives: [f64; 4] = [eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e553,) = {
    if (!s.b[1764]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e553;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e563, eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9, eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3,) = {
    if s.b[1765] {
        let eq14_e557: f64 = (p[31] * s.v[13]);let eq14_e559: f64 = (eq14_e557 * s.v[316]);let eq14_e559_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[316]) + (eq14_e557 * s.dn[316][0]));let eq14_e559_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[316]) + (eq14_e557 * s.dn[316][1]));let eq14_e559_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[316]) + (eq14_e557 * s.dn[316][2]));let eq14_e559_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[316]) + (eq14_e557 * s.dn[316][3]));let eq14_e559_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[316]) + (eq14_e557 * s.dn[316][4]));let eq14_e559_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[316]) + (eq14_e557 * s.dn[316][5]));let eq14_e559_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[316]) + (eq14_e557 * s.dn[316][6]));let eq14_e559_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[316]) + (eq14_e557 * s.dn[316][7]));let eq14_e559_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[316]) + (eq14_e557 * s.dn[316][8]));let eq14_e559_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[316]) + (eq14_e557 * s.dn[316][9]));let eq14_e559_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[316]) + (eq14_e557 * s.db[316][0]));let eq14_e559_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[316]) + (eq14_e557 * s.db[316][1]));let eq14_e559_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[316]) + (eq14_e557 * s.db[316][2]));let eq14_e559_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[316]) + (eq14_e557 * s.db[316][3]));let eq14_e561: f64 = (eq14_e559 * (nv2 - nv6));let eq14_e561_d_n0: f64 = (eq14_e559_d_n0 * (nv2 - nv6));let eq14_e561_d_n1: f64 = (eq14_e559_d_n1 * (nv2 - nv6));let eq14_e561_d_n2: f64 = ((eq14_e559_d_n2 * (nv2 - nv6)) + eq14_e559);let eq14_e561_d_n3: f64 = (eq14_e559_d_n3 * (nv2 - nv6));let eq14_e561_d_n4: f64 = (eq14_e559_d_n4 * (nv2 - nv6));let eq14_e561_d_n5: f64 = (eq14_e559_d_n5 * (nv2 - nv6));let eq14_e561_d_n6: f64 = ((eq14_e559_d_n6 * (nv2 - nv6)) + (-eq14_e559));let eq14_e561_d_n7: f64 = (eq14_e559_d_n7 * (nv2 - nv6));let eq14_e561_d_n8: f64 = (eq14_e559_d_n8 * (nv2 - nv6));let eq14_e561_d_n9: f64 = (eq14_e559_d_n9 * (nv2 - nv6));let eq14_e561_d_b0: f64 = (eq14_e559_d_b0 * (nv2 - nv6));let eq14_e561_d_b1: f64 = (eq14_e559_d_b1 * (nv2 - nv6));let eq14_e561_d_b2: f64 = (eq14_e559_d_b2 * (nv2 - nv6));let eq14_e561_d_b3: f64 = (eq14_e559_d_b3 * (nv2 - nv6));
        (eq14_e561, eq14_e561_d_n0, eq14_e561_d_n1, eq14_e561_d_n2, eq14_e561_d_n3, eq14_e561_d_n4, eq14_e561_d_n5, eq14_e561_d_n6, eq14_e561_d_n7, eq14_e561_d_n8, eq14_e561_d_n9, eq14_e561_d_b0, eq14_e561_d_b1, eq14_e561_d_b2, eq14_e561_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e563;let eq14_node_derivatives: [f64; 10] = [eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9];let eq14_branch_derivatives: [f64; 4] = [eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq16_e578,) = {
    if (!s.b[1765]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e578;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
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
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq17_e588, eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9, eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3,) = {
    if s.b[1766] {
        let eq17_e582: f64 = (p[31] * s.v[13]);let eq17_e584: f64 = (eq17_e582 * s.v[320]);let eq17_e584_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[320]) + (eq17_e582 * s.dn[320][0]));let eq17_e584_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[320]) + (eq17_e582 * s.dn[320][1]));let eq17_e584_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[320]) + (eq17_e582 * s.dn[320][2]));let eq17_e584_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[320]) + (eq17_e582 * s.dn[320][3]));let eq17_e584_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[320]) + (eq17_e582 * s.dn[320][4]));let eq17_e584_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[320]) + (eq17_e582 * s.dn[320][5]));let eq17_e584_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[320]) + (eq17_e582 * s.dn[320][6]));let eq17_e584_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[320]) + (eq17_e582 * s.dn[320][7]));let eq17_e584_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[320]) + (eq17_e582 * s.dn[320][8]));let eq17_e584_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[320]) + (eq17_e582 * s.dn[320][9]));let eq17_e584_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[320]) + (eq17_e582 * s.db[320][0]));let eq17_e584_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[320]) + (eq17_e582 * s.db[320][1]));let eq17_e584_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[320]) + (eq17_e582 * s.db[320][2]));let eq17_e584_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[320]) + (eq17_e582 * s.db[320][3]));let eq17_e586: f64 = (eq17_e584 * (nv0 - nv7));let eq17_e586_d_n0: f64 = ((eq17_e584_d_n0 * (nv0 - nv7)) + eq17_e584);let eq17_e586_d_n1: f64 = (eq17_e584_d_n1 * (nv0 - nv7));let eq17_e586_d_n2: f64 = (eq17_e584_d_n2 * (nv0 - nv7));let eq17_e586_d_n3: f64 = (eq17_e584_d_n3 * (nv0 - nv7));let eq17_e586_d_n4: f64 = (eq17_e584_d_n4 * (nv0 - nv7));let eq17_e586_d_n5: f64 = (eq17_e584_d_n5 * (nv0 - nv7));let eq17_e586_d_n6: f64 = (eq17_e584_d_n6 * (nv0 - nv7));let eq17_e586_d_n7: f64 = ((eq17_e584_d_n7 * (nv0 - nv7)) + (-eq17_e584));let eq17_e586_d_n8: f64 = (eq17_e584_d_n8 * (nv0 - nv7));let eq17_e586_d_n9: f64 = (eq17_e584_d_n9 * (nv0 - nv7));let eq17_e586_d_b0: f64 = (eq17_e584_d_b0 * (nv0 - nv7));let eq17_e586_d_b1: f64 = (eq17_e584_d_b1 * (nv0 - nv7));let eq17_e586_d_b2: f64 = (eq17_e584_d_b2 * (nv0 - nv7));let eq17_e586_d_b3: f64 = (eq17_e584_d_b3 * (nv0 - nv7));
        (eq17_e586, eq17_e586_d_n0, eq17_e586_d_n1, eq17_e586_d_n2, eq17_e586_d_n3, eq17_e586_d_n4, eq17_e586_d_n5, eq17_e586_d_n6, eq17_e586_d_n7, eq17_e586_d_n8, eq17_e586_d_n9, eq17_e586_d_b0, eq17_e586_d_b1, eq17_e586_d_b2, eq17_e586_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e588;let eq17_node_derivatives: [f64; 10] = [eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9];let eq17_branch_derivatives: [f64; 4] = [eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e603,) = {
    if (!s.b[1766]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e603;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
        let (eq20_e613, eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9, eq20_e613_d_b0, eq20_e613_d_b1, eq20_e613_d_b2, eq20_e613_d_b3,) = {
    if s.b[1767] {
        let eq20_e607: f64 = (p[31] * s.v[13]);let eq20_e609: f64 = (eq20_e607 * s.v[323]);let eq20_e609_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[323]) + (eq20_e607 * s.dn[323][0]));let eq20_e609_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[323]) + (eq20_e607 * s.dn[323][1]));let eq20_e609_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[323]) + (eq20_e607 * s.dn[323][2]));let eq20_e609_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[323]) + (eq20_e607 * s.dn[323][3]));let eq20_e609_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[323]) + (eq20_e607 * s.dn[323][4]));let eq20_e609_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[323]) + (eq20_e607 * s.dn[323][5]));let eq20_e609_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[323]) + (eq20_e607 * s.dn[323][6]));let eq20_e609_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[323]) + (eq20_e607 * s.dn[323][7]));let eq20_e609_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[323]) + (eq20_e607 * s.dn[323][8]));let eq20_e609_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[323]) + (eq20_e607 * s.dn[323][9]));let eq20_e609_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[323]) + (eq20_e607 * s.db[323][0]));let eq20_e609_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[323]) + (eq20_e607 * s.db[323][1]));let eq20_e609_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[323]) + (eq20_e607 * s.db[323][2]));let eq20_e609_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[323]) + (eq20_e607 * s.db[323][3]));let eq20_e611: f64 = (eq20_e609 * (nv3 - nv8));let eq20_e611_d_n0: f64 = (eq20_e609_d_n0 * (nv3 - nv8));let eq20_e611_d_n1: f64 = (eq20_e609_d_n1 * (nv3 - nv8));let eq20_e611_d_n2: f64 = (eq20_e609_d_n2 * (nv3 - nv8));let eq20_e611_d_n3: f64 = ((eq20_e609_d_n3 * (nv3 - nv8)) + eq20_e609);let eq20_e611_d_n4: f64 = (eq20_e609_d_n4 * (nv3 - nv8));let eq20_e611_d_n5: f64 = (eq20_e609_d_n5 * (nv3 - nv8));let eq20_e611_d_n6: f64 = (eq20_e609_d_n6 * (nv3 - nv8));let eq20_e611_d_n7: f64 = (eq20_e609_d_n7 * (nv3 - nv8));let eq20_e611_d_n8: f64 = ((eq20_e609_d_n8 * (nv3 - nv8)) + (-eq20_e609));let eq20_e611_d_n9: f64 = (eq20_e609_d_n9 * (nv3 - nv8));let eq20_e611_d_b0: f64 = (eq20_e609_d_b0 * (nv3 - nv8));let eq20_e611_d_b1: f64 = (eq20_e609_d_b1 * (nv3 - nv8));let eq20_e611_d_b2: f64 = (eq20_e609_d_b2 * (nv3 - nv8));let eq20_e611_d_b3: f64 = (eq20_e609_d_b3 * (nv3 - nv8));
        (eq20_e611, eq20_e611_d_n0, eq20_e611_d_n1, eq20_e611_d_n2, eq20_e611_d_n3, eq20_e611_d_n4, eq20_e611_d_n5, eq20_e611_d_n6, eq20_e611_d_n7, eq20_e611_d_n8, eq20_e611_d_n9, eq20_e611_d_b0, eq20_e611_d_b1, eq20_e611_d_b2, eq20_e611_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e613;let eq20_node_derivatives: [f64; 10] = [eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9];let eq20_branch_derivatives: [f64; 4] = [eq20_e613_d_b0, eq20_e613_d_b1, eq20_e613_d_b2, eq20_e613_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq22_e628,) = {
    if (!s.b[1767]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e628;
        stamper.stamp_potential_const_local(
            3,
            eq22_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        let eq23_e631: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[358]);let eq23_e633: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[373]);let eq23_e634: f64 = (eq23_e631 + eq23_e633);let eq23_e634_d_n0: f64 = ((s.dn[358][0] * ddt_scale) + (s.dn[373][0] * ddt_scale));let eq23_e634_d_n1: f64 = ((s.dn[358][1] * ddt_scale) + (s.dn[373][1] * ddt_scale));let eq23_e634_d_n2: f64 = ((s.dn[358][2] * ddt_scale) + (s.dn[373][2] * ddt_scale));let eq23_e634_d_n3: f64 = ((s.dn[358][3] * ddt_scale) + (s.dn[373][3] * ddt_scale));let eq23_e634_d_n4: f64 = ((s.dn[358][4] * ddt_scale) + (s.dn[373][4] * ddt_scale));let eq23_e634_d_n5: f64 = ((s.dn[358][5] * ddt_scale) + (s.dn[373][5] * ddt_scale));let eq23_e634_d_n6: f64 = ((s.dn[358][6] * ddt_scale) + (s.dn[373][6] * ddt_scale));let eq23_e634_d_n7: f64 = ((s.dn[358][7] * ddt_scale) + (s.dn[373][7] * ddt_scale));let eq23_e634_d_n8: f64 = ((s.dn[358][8] * ddt_scale) + (s.dn[373][8] * ddt_scale));let eq23_e634_d_n9: f64 = ((s.dn[358][9] * ddt_scale) + (s.dn[373][9] * ddt_scale));let eq23_e634_d_b0: f64 = ((s.db[358][0] * ddt_scale) + (s.db[373][0] * ddt_scale));let eq23_e634_d_b1: f64 = ((s.db[358][1] * ddt_scale) + (s.db[373][1] * ddt_scale));let eq23_e634_d_b2: f64 = ((s.db[358][2] * ddt_scale) + (s.db[373][2] * ddt_scale));let eq23_e634_d_b3: f64 = ((s.db[358][3] * ddt_scale) + (s.db[373][3] * ddt_scale));let eq23_e636: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[377]);let eq23_e637: f64 = (eq23_e634 + eq23_e636);let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + (s.dn[377][0] * ddt_scale));let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + (s.dn[377][1] * ddt_scale));let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + (s.dn[377][2] * ddt_scale));let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + (s.dn[377][3] * ddt_scale));let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + (s.dn[377][4] * ddt_scale));let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + (s.dn[377][5] * ddt_scale));let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + (s.dn[377][6] * ddt_scale));let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + (s.dn[377][7] * ddt_scale));let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + (s.dn[377][8] * ddt_scale));let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + (s.dn[377][9] * ddt_scale));let eq23_e637_d_b0: f64 = (eq23_e634_d_b0 + (s.db[377][0] * ddt_scale));let eq23_e637_d_b1: f64 = (eq23_e634_d_b1 + (s.db[377][1] * ddt_scale));let eq23_e637_d_b2: f64 = (eq23_e634_d_b2 + (s.db[377][2] * ddt_scale));let eq23_e637_d_b3: f64 = (eq23_e634_d_b3 + (s.db[377][3] * ddt_scale));let eq23_e638: f64 = (p[14] * eq23_e637);let eq23_e638_d_n0: f64 = (p[14] * eq23_e637_d_n0);let eq23_e638_d_n1: f64 = (p[14] * eq23_e637_d_n1);let eq23_e638_d_n2: f64 = (p[14] * eq23_e637_d_n2);let eq23_e638_d_n3: f64 = (p[14] * eq23_e637_d_n3);let eq23_e638_d_n4: f64 = (p[14] * eq23_e637_d_n4);let eq23_e638_d_n5: f64 = (p[14] * eq23_e637_d_n5);let eq23_e638_d_n6: f64 = (p[14] * eq23_e637_d_n6);let eq23_e638_d_n7: f64 = (p[14] * eq23_e637_d_n7);let eq23_e638_d_n8: f64 = (p[14] * eq23_e637_d_n8);let eq23_e638_d_n9: f64 = (p[14] * eq23_e637_d_n9);let eq23_e638_d_b0: f64 = (p[14] * eq23_e637_d_b0);let eq23_e638_d_b1: f64 = (p[14] * eq23_e637_d_b1);let eq23_e638_d_b2: f64 = (p[14] * eq23_e637_d_b2);let eq23_e638_d_b3: f64 = (p[14] * eq23_e637_d_b3);let eq23_value: f64 = eq23_e638;let eq23_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_branch_derivatives: [f64; 4] = [eq23_e638_d_b0, eq23_e638_d_b1, eq23_e638_d_b2, eq23_e638_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
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
        let eq24_e641: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[367]);let eq24_e643: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[369]);let eq24_e644: f64 = (eq24_e641 + eq24_e643);let eq24_e644_d_n0: f64 = ((s.dn[367][0] * ddt_scale) + (s.dn[369][0] * ddt_scale));let eq24_e644_d_n1: f64 = ((s.dn[367][1] * ddt_scale) + (s.dn[369][1] * ddt_scale));let eq24_e644_d_n2: f64 = ((s.dn[367][2] * ddt_scale) + (s.dn[369][2] * ddt_scale));let eq24_e644_d_n3: f64 = ((s.dn[367][3] * ddt_scale) + (s.dn[369][3] * ddt_scale));let eq24_e644_d_n4: f64 = ((s.dn[367][4] * ddt_scale) + (s.dn[369][4] * ddt_scale));let eq24_e644_d_n5: f64 = ((s.dn[367][5] * ddt_scale) + (s.dn[369][5] * ddt_scale));let eq24_e644_d_n6: f64 = ((s.dn[367][6] * ddt_scale) + (s.dn[369][6] * ddt_scale));let eq24_e644_d_n7: f64 = ((s.dn[367][7] * ddt_scale) + (s.dn[369][7] * ddt_scale));let eq24_e644_d_n8: f64 = ((s.dn[367][8] * ddt_scale) + (s.dn[369][8] * ddt_scale));let eq24_e644_d_n9: f64 = ((s.dn[367][9] * ddt_scale) + (s.dn[369][9] * ddt_scale));let eq24_e644_d_b0: f64 = ((s.db[367][0] * ddt_scale) + (s.db[369][0] * ddt_scale));let eq24_e644_d_b1: f64 = ((s.db[367][1] * ddt_scale) + (s.db[369][1] * ddt_scale));let eq24_e644_d_b2: f64 = ((s.db[367][2] * ddt_scale) + (s.db[369][2] * ddt_scale));let eq24_e644_d_b3: f64 = ((s.db[367][3] * ddt_scale) + (s.db[369][3] * ddt_scale));let eq24_e646: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[376]);let eq24_e647: f64 = (eq24_e644 + eq24_e646);let eq24_e647_d_n0: f64 = (eq24_e644_d_n0 + (s.dn[376][0] * ddt_scale));let eq24_e647_d_n1: f64 = (eq24_e644_d_n1 + (s.dn[376][1] * ddt_scale));let eq24_e647_d_n2: f64 = (eq24_e644_d_n2 + (s.dn[376][2] * ddt_scale));let eq24_e647_d_n3: f64 = (eq24_e644_d_n3 + (s.dn[376][3] * ddt_scale));let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + (s.dn[376][4] * ddt_scale));let eq24_e647_d_n5: f64 = (eq24_e644_d_n5 + (s.dn[376][5] * ddt_scale));let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + (s.dn[376][6] * ddt_scale));let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + (s.dn[376][7] * ddt_scale));let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + (s.dn[376][8] * ddt_scale));let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + (s.dn[376][9] * ddt_scale));let eq24_e647_d_b0: f64 = (eq24_e644_d_b0 + (s.db[376][0] * ddt_scale));let eq24_e647_d_b1: f64 = (eq24_e644_d_b1 + (s.db[376][1] * ddt_scale));let eq24_e647_d_b2: f64 = (eq24_e644_d_b2 + (s.db[376][2] * ddt_scale));let eq24_e647_d_b3: f64 = (eq24_e644_d_b3 + (s.db[376][3] * ddt_scale));let eq24_e648: f64 = (p[14] * eq24_e647);let eq24_e648_d_n0: f64 = (p[14] * eq24_e647_d_n0);let eq24_e648_d_n1: f64 = (p[14] * eq24_e647_d_n1);let eq24_e648_d_n2: f64 = (p[14] * eq24_e647_d_n2);let eq24_e648_d_n3: f64 = (p[14] * eq24_e647_d_n3);let eq24_e648_d_n4: f64 = (p[14] * eq24_e647_d_n4);let eq24_e648_d_n5: f64 = (p[14] * eq24_e647_d_n5);let eq24_e648_d_n6: f64 = (p[14] * eq24_e647_d_n6);let eq24_e648_d_n7: f64 = (p[14] * eq24_e647_d_n7);let eq24_e648_d_n8: f64 = (p[14] * eq24_e647_d_n8);let eq24_e648_d_n9: f64 = (p[14] * eq24_e647_d_n9);let eq24_e648_d_b0: f64 = (p[14] * eq24_e647_d_b0);let eq24_e648_d_b1: f64 = (p[14] * eq24_e647_d_b1);let eq24_e648_d_b2: f64 = (p[14] * eq24_e647_d_b2);let eq24_e648_d_b3: f64 = (p[14] * eq24_e647_d_b3);let eq24_value: f64 = eq24_e648;let eq24_node_derivatives: [f64; 10] = [eq24_e648_d_n0, eq24_e648_d_n1, eq24_e648_d_n2, eq24_e648_d_n3, eq24_e648_d_n4, eq24_e648_d_n5, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_branch_derivatives: [f64; 4] = [eq24_e648_d_b0, eq24_e648_d_b1, eq24_e648_d_b2, eq24_e648_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );let eq25_e651: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[372]);let eq25_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[378]);let eq25_e654: f64 = (eq25_e651 + eq25_e653);let eq25_e654_d_n0: f64 = ((s.dn[372][0] * ddt_scale) + (s.dn[378][0] * ddt_scale));let eq25_e654_d_n1: f64 = ((s.dn[372][1] * ddt_scale) + (s.dn[378][1] * ddt_scale));let eq25_e654_d_n2: f64 = ((s.dn[372][2] * ddt_scale) + (s.dn[378][2] * ddt_scale));let eq25_e654_d_n3: f64 = ((s.dn[372][3] * ddt_scale) + (s.dn[378][3] * ddt_scale));let eq25_e654_d_n4: f64 = ((s.dn[372][4] * ddt_scale) + (s.dn[378][4] * ddt_scale));let eq25_e654_d_n5: f64 = ((s.dn[372][5] * ddt_scale) + (s.dn[378][5] * ddt_scale));let eq25_e654_d_n6: f64 = ((s.dn[372][6] * ddt_scale) + (s.dn[378][6] * ddt_scale));let eq25_e654_d_n7: f64 = ((s.dn[372][7] * ddt_scale) + (s.dn[378][7] * ddt_scale));let eq25_e654_d_n8: f64 = ((s.dn[372][8] * ddt_scale) + (s.dn[378][8] * ddt_scale));let eq25_e654_d_n9: f64 = ((s.dn[372][9] * ddt_scale) + (s.dn[378][9] * ddt_scale));let eq25_e654_d_b0: f64 = ((s.db[372][0] * ddt_scale) + (s.db[378][0] * ddt_scale));let eq25_e654_d_b1: f64 = ((s.db[372][1] * ddt_scale) + (s.db[378][1] * ddt_scale));let eq25_e654_d_b2: f64 = ((s.db[372][2] * ddt_scale) + (s.db[378][2] * ddt_scale));let eq25_e654_d_b3: f64 = ((s.db[372][3] * ddt_scale) + (s.db[378][3] * ddt_scale));let eq25_e655: f64 = (p[14] * eq25_e654);let eq25_e655_d_n0: f64 = (p[14] * eq25_e654_d_n0);let eq25_e655_d_n1: f64 = (p[14] * eq25_e654_d_n1);let eq25_e655_d_n2: f64 = (p[14] * eq25_e654_d_n2);let eq25_e655_d_n3: f64 = (p[14] * eq25_e654_d_n3);let eq25_e655_d_n4: f64 = (p[14] * eq25_e654_d_n4);let eq25_e655_d_n5: f64 = (p[14] * eq25_e654_d_n5);let eq25_e655_d_n6: f64 = (p[14] * eq25_e654_d_n6);let eq25_e655_d_n7: f64 = (p[14] * eq25_e654_d_n7);let eq25_e655_d_n8: f64 = (p[14] * eq25_e654_d_n8);let eq25_e655_d_n9: f64 = (p[14] * eq25_e654_d_n9);let eq25_e655_d_b0: f64 = (p[14] * eq25_e654_d_b0);let eq25_e655_d_b1: f64 = (p[14] * eq25_e654_d_b1);let eq25_e655_d_b2: f64 = (p[14] * eq25_e654_d_b2);let eq25_e655_d_b3: f64 = (p[14] * eq25_e654_d_b3);let eq25_value: f64 = eq25_e655;let eq25_node_derivatives: [f64; 10] = [eq25_e655_d_n0, eq25_e655_d_n1, eq25_e655_d_n2, eq25_e655_d_n3, eq25_e655_d_n4, eq25_e655_d_n5, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];let eq25_branch_derivatives: [f64; 4] = [eq25_e655_d_b0, eq25_e655_d_b1, eq25_e655_d_b2, eq25_e655_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
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
        let eq26_e658: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[370]);let eq26_e659: f64 = (p[14] * eq26_e658);let eq26_e659_d_n0: f64 = (p[14] * (s.dn[370][0] * ddt_scale));let eq26_e659_d_n1: f64 = (p[14] * (s.dn[370][1] * ddt_scale));let eq26_e659_d_n2: f64 = (p[14] * (s.dn[370][2] * ddt_scale));let eq26_e659_d_n3: f64 = (p[14] * (s.dn[370][3] * ddt_scale));let eq26_e659_d_n4: f64 = (p[14] * (s.dn[370][4] * ddt_scale));let eq26_e659_d_n5: f64 = (p[14] * (s.dn[370][5] * ddt_scale));let eq26_e659_d_n6: f64 = (p[14] * (s.dn[370][6] * ddt_scale));let eq26_e659_d_n7: f64 = (p[14] * (s.dn[370][7] * ddt_scale));let eq26_e659_d_n8: f64 = (p[14] * (s.dn[370][8] * ddt_scale));let eq26_e659_d_n9: f64 = (p[14] * (s.dn[370][9] * ddt_scale));let eq26_e659_d_b0: f64 = (p[14] * (s.db[370][0] * ddt_scale));let eq26_e659_d_b1: f64 = (p[14] * (s.db[370][1] * ddt_scale));let eq26_e659_d_b2: f64 = (p[14] * (s.db[370][2] * ddt_scale));let eq26_e659_d_b3: f64 = (p[14] * (s.db[370][3] * ddt_scale));let eq26_value: f64 = eq26_e659;let eq26_node_derivatives: [f64; 10] = [eq26_e659_d_n0, eq26_e659_d_n1, eq26_e659_d_n2, eq26_e659_d_n3, eq26_e659_d_n4, eq26_e659_d_n5, eq26_e659_d_n6, eq26_e659_d_n7, eq26_e659_d_n8, eq26_e659_d_n9];let eq26_branch_derivatives: [f64; 4] = [eq26_e659_d_b0, eq26_e659_d_b1, eq26_e659_d_b2, eq26_e659_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );let eq27_e662: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[357]);let eq27_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[366]);let eq27_e665: f64 = (eq27_e662 + eq27_e664);let eq27_e665_d_n0: f64 = ((s.dn[357][0] * ddt_scale) + (s.dn[366][0] * ddt_scale));let eq27_e665_d_n1: f64 = ((s.dn[357][1] * ddt_scale) + (s.dn[366][1] * ddt_scale));let eq27_e665_d_n2: f64 = ((s.dn[357][2] * ddt_scale) + (s.dn[366][2] * ddt_scale));let eq27_e665_d_n3: f64 = ((s.dn[357][3] * ddt_scale) + (s.dn[366][3] * ddt_scale));let eq27_e665_d_n4: f64 = ((s.dn[357][4] * ddt_scale) + (s.dn[366][4] * ddt_scale));let eq27_e665_d_n5: f64 = ((s.dn[357][5] * ddt_scale) + (s.dn[366][5] * ddt_scale));let eq27_e665_d_n6: f64 = ((s.dn[357][6] * ddt_scale) + (s.dn[366][6] * ddt_scale));let eq27_e665_d_n7: f64 = ((s.dn[357][7] * ddt_scale) + (s.dn[366][7] * ddt_scale));let eq27_e665_d_n8: f64 = ((s.dn[357][8] * ddt_scale) + (s.dn[366][8] * ddt_scale));let eq27_e665_d_n9: f64 = ((s.dn[357][9] * ddt_scale) + (s.dn[366][9] * ddt_scale));let eq27_e665_d_b0: f64 = ((s.db[357][0] * ddt_scale) + (s.db[366][0] * ddt_scale));let eq27_e665_d_b1: f64 = ((s.db[357][1] * ddt_scale) + (s.db[366][1] * ddt_scale));let eq27_e665_d_b2: f64 = ((s.db[357][2] * ddt_scale) + (s.db[366][2] * ddt_scale));let eq27_e665_d_b3: f64 = ((s.db[357][3] * ddt_scale) + (s.db[366][3] * ddt_scale));let eq27_e667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[368]);let eq27_e668: f64 = (eq27_e665 + eq27_e667);let eq27_e668_d_n0: f64 = (eq27_e665_d_n0 + (s.dn[368][0] * ddt_scale));let eq27_e668_d_n1: f64 = (eq27_e665_d_n1 + (s.dn[368][1] * ddt_scale));let eq27_e668_d_n2: f64 = (eq27_e665_d_n2 + (s.dn[368][2] * ddt_scale));let eq27_e668_d_n3: f64 = (eq27_e665_d_n3 + (s.dn[368][3] * ddt_scale));let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + (s.dn[368][4] * ddt_scale));let eq27_e668_d_n5: f64 = (eq27_e665_d_n5 + (s.dn[368][5] * ddt_scale));let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + (s.dn[368][6] * ddt_scale));let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + (s.dn[368][7] * ddt_scale));let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + (s.dn[368][8] * ddt_scale));let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + (s.dn[368][9] * ddt_scale));let eq27_e668_d_b0: f64 = (eq27_e665_d_b0 + (s.db[368][0] * ddt_scale));let eq27_e668_d_b1: f64 = (eq27_e665_d_b1 + (s.db[368][1] * ddt_scale));let eq27_e668_d_b2: f64 = (eq27_e665_d_b2 + (s.db[368][2] * ddt_scale));let eq27_e668_d_b3: f64 = (eq27_e665_d_b3 + (s.db[368][3] * ddt_scale));let eq27_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[375]);let eq27_e671: f64 = (eq27_e668 + eq27_e670);let eq27_e671_d_n0: f64 = (eq27_e668_d_n0 + (s.dn[375][0] * ddt_scale));let eq27_e671_d_n1: f64 = (eq27_e668_d_n1 + (s.dn[375][1] * ddt_scale));let eq27_e671_d_n2: f64 = (eq27_e668_d_n2 + (s.dn[375][2] * ddt_scale));let eq27_e671_d_n3: f64 = (eq27_e668_d_n3 + (s.dn[375][3] * ddt_scale));let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + (s.dn[375][4] * ddt_scale));let eq27_e671_d_n5: f64 = (eq27_e668_d_n5 + (s.dn[375][5] * ddt_scale));let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + (s.dn[375][6] * ddt_scale));let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + (s.dn[375][7] * ddt_scale));let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + (s.dn[375][8] * ddt_scale));
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + (s.dn[375][9] * ddt_scale));let eq27_e671_d_b0: f64 = (eq27_e668_d_b0 + (s.db[375][0] * ddt_scale));let eq27_e671_d_b1: f64 = (eq27_e668_d_b1 + (s.db[375][1] * ddt_scale));let eq27_e671_d_b2: f64 = (eq27_e668_d_b2 + (s.db[375][2] * ddt_scale));let eq27_e671_d_b3: f64 = (eq27_e668_d_b3 + (s.db[375][3] * ddt_scale));let eq27_e672: f64 = (p[14] * eq27_e671);let eq27_e672_d_n0: f64 = (p[14] * eq27_e671_d_n0);let eq27_e672_d_n1: f64 = (p[14] * eq27_e671_d_n1);let eq27_e672_d_n2: f64 = (p[14] * eq27_e671_d_n2);let eq27_e672_d_n3: f64 = (p[14] * eq27_e671_d_n3);let eq27_e672_d_n4: f64 = (p[14] * eq27_e671_d_n4);let eq27_e672_d_n5: f64 = (p[14] * eq27_e671_d_n5);let eq27_e672_d_n6: f64 = (p[14] * eq27_e671_d_n6);let eq27_e672_d_n7: f64 = (p[14] * eq27_e671_d_n7);let eq27_e672_d_n8: f64 = (p[14] * eq27_e671_d_n8);let eq27_e672_d_n9: f64 = (p[14] * eq27_e671_d_n9);let eq27_e672_d_b0: f64 = (p[14] * eq27_e671_d_b0);let eq27_e672_d_b1: f64 = (p[14] * eq27_e671_d_b1);let eq27_e672_d_b2: f64 = (p[14] * eq27_e671_d_b2);let eq27_e672_d_b3: f64 = (p[14] * eq27_e671_d_b3);let eq27_value: f64 = eq27_e672;let eq27_node_derivatives: [f64; 10] = [eq27_e672_d_n0, eq27_e672_d_n1, eq27_e672_d_n2, eq27_e672_d_n3, eq27_e672_d_n4, eq27_e672_d_n5, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];let eq27_branch_derivatives: [f64; 4] = [eq27_e672_d_b0, eq27_e672_d_b1, eq27_e672_d_b2, eq27_e672_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let eq28_e675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[359]);let eq28_e677: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[371]);let eq28_e678: f64 = (eq28_e675 + eq28_e677);let eq28_e678_d_n0: f64 = ((s.dn[359][0] * ddt_scale) + (s.dn[371][0] * ddt_scale));let eq28_e678_d_n1: f64 = ((s.dn[359][1] * ddt_scale) + (s.dn[371][1] * ddt_scale));let eq28_e678_d_n2: f64 = ((s.dn[359][2] * ddt_scale) + (s.dn[371][2] * ddt_scale));let eq28_e678_d_n3: f64 = ((s.dn[359][3] * ddt_scale) + (s.dn[371][3] * ddt_scale));let eq28_e678_d_n4: f64 = ((s.dn[359][4] * ddt_scale) + (s.dn[371][4] * ddt_scale));let eq28_e678_d_n5: f64 = ((s.dn[359][5] * ddt_scale) + (s.dn[371][5] * ddt_scale));let eq28_e678_d_n6: f64 = ((s.dn[359][6] * ddt_scale) + (s.dn[371][6] * ddt_scale));let eq28_e678_d_n7: f64 = ((s.dn[359][7] * ddt_scale) + (s.dn[371][7] * ddt_scale));let eq28_e678_d_n8: f64 = ((s.dn[359][8] * ddt_scale) + (s.dn[371][8] * ddt_scale));let eq28_e678_d_n9: f64 = ((s.dn[359][9] * ddt_scale) + (s.dn[371][9] * ddt_scale));let eq28_e678_d_b0: f64 = ((s.db[359][0] * ddt_scale) + (s.db[371][0] * ddt_scale));let eq28_e678_d_b1: f64 = ((s.db[359][1] * ddt_scale) + (s.db[371][1] * ddt_scale));let eq28_e678_d_b2: f64 = ((s.db[359][2] * ddt_scale) + (s.db[371][2] * ddt_scale));let eq28_e678_d_b3: f64 = ((s.db[359][3] * ddt_scale) + (s.db[371][3] * ddt_scale));let eq28_e679: f64 = (p[14] * eq28_e678);let eq28_e679_d_n0: f64 = (p[14] * eq28_e678_d_n0);let eq28_e679_d_n1: f64 = (p[14] * eq28_e678_d_n1);let eq28_e679_d_n2: f64 = (p[14] * eq28_e678_d_n2);let eq28_e679_d_n3: f64 = (p[14] * eq28_e678_d_n3);let eq28_e679_d_n4: f64 = (p[14] * eq28_e678_d_n4);let eq28_e679_d_n5: f64 = (p[14] * eq28_e678_d_n5);let eq28_e679_d_n6: f64 = (p[14] * eq28_e678_d_n6);let eq28_e679_d_n7: f64 = (p[14] * eq28_e678_d_n7);let eq28_e679_d_n8: f64 = (p[14] * eq28_e678_d_n8);let eq28_e679_d_n9: f64 = (p[14] * eq28_e678_d_n9);let eq28_e679_d_b0: f64 = (p[14] * eq28_e678_d_b0);let eq28_e679_d_b1: f64 = (p[14] * eq28_e678_d_b1);let eq28_e679_d_b2: f64 = (p[14] * eq28_e678_d_b2);let eq28_e679_d_b3: f64 = (p[14] * eq28_e678_d_b3);let eq28_value: f64 = eq28_e679;let eq28_node_derivatives: [f64; 10] = [eq28_e679_d_n0, eq28_e679_d_n1, eq28_e679_d_n2, eq28_e679_d_n3, eq28_e679_d_n4, eq28_e679_d_n5, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];let eq28_branch_derivatives: [f64; 4] = [eq28_e679_d_b0, eq28_e679_d_b1, eq28_e679_d_b2, eq28_e679_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );let eq29_e681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[374]);let eq29_value: f64 = eq29_e681;
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq29_value),
            &s.dn[374],
            &s.db[374],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
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
        let nv5 = ctx.node_voltage(nodes[5]);let eq31_e687: f64 = (s.v[1793] * (nv5 - 0.0));let eq31_e687_d_n0: f64 = (s.dn[1793][0] * (nv5 - 0.0));let eq31_e687_d_n1: f64 = (s.dn[1793][1] * (nv5 - 0.0));let eq31_e687_d_n2: f64 = (s.dn[1793][2] * (nv5 - 0.0));let eq31_e687_d_n3: f64 = (s.dn[1793][3] * (nv5 - 0.0));let eq31_e687_d_n4: f64 = (s.dn[1793][4] * (nv5 - 0.0));let eq31_e687_d_n5: f64 = ((s.dn[1793][5] * (nv5 - 0.0)) + s.v[1793]);let eq31_e687_d_n6: f64 = (s.dn[1793][6] * (nv5 - 0.0));let eq31_e687_d_n7: f64 = (s.dn[1793][7] * (nv5 - 0.0));let eq31_e687_d_n8: f64 = (s.dn[1793][8] * (nv5 - 0.0));let eq31_e687_d_n9: f64 = (s.dn[1793][9] * (nv5 - 0.0));let eq31_e687_d_b0: f64 = (s.db[1793][0] * (nv5 - 0.0));let eq31_e687_d_b1: f64 = (s.db[1793][1] * (nv5 - 0.0));let eq31_e687_d_b2: f64 = (s.db[1793][2] * (nv5 - 0.0));let eq31_e687_d_b3: f64 = (s.db[1793][3] * (nv5 - 0.0));let eq31_value: f64 = eq31_e687;let eq31_node_derivatives: [f64; 10] = [eq31_e687_d_n0, eq31_e687_d_n1, eq31_e687_d_n2, eq31_e687_d_n3, eq31_e687_d_n4, eq31_e687_d_n5, eq31_e687_d_n6, eq31_e687_d_n7, eq31_e687_d_n8, eq31_e687_d_n9];let eq31_branch_derivatives: [f64; 4] = [eq31_e687_d_b0, eq31_e687_d_b1, eq31_e687_d_b2, eq31_e687_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));let eq32_e690_d_b0: f64 = (s.db[1790][0] * (nv5 - 0.0));let eq32_e690_d_b1: f64 = (s.db[1790][1] * (nv5 - 0.0));let eq32_e690_d_b2: f64 = (s.db[1790][2] * (nv5 - 0.0));let eq32_e690_d_b3: f64 = (s.db[1790][3] * (nv5 - 0.0));let eq32_e691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq32_e690);let eq32_value: f64 = eq32_e691;let eq32_node_derivatives: [f64; 10] = [(eq32_e690_d_n0 * ddt_scale), (eq32_e690_d_n1 * ddt_scale), (eq32_e690_d_n2 * ddt_scale), (eq32_e690_d_n3 * ddt_scale), (eq32_e690_d_n4 * ddt_scale), (eq32_e690_d_n5 * ddt_scale), (eq32_e690_d_n6 * ddt_scale), (eq32_e690_d_n7 * ddt_scale), (eq32_e690_d_n8 * ddt_scale), (eq32_e690_d_n9 * ddt_scale)];let eq32_branch_derivatives: [f64; 4] = [(eq32_e690_d_b0 * ddt_scale), (eq32_e690_d_b1 * ddt_scale), (eq32_e690_d_b2 * ddt_scale), (eq32_e690_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e693: f64 = (-s.v[1791]);let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));let eq33_e695_d_n0: f64 = ((-s.dn[1791][0]) * (nv5 - 0.0));let eq33_e695_d_n1: f64 = ((-s.dn[1791][1]) * (nv5 - 0.0));let eq33_e695_d_n2: f64 = ((-s.dn[1791][2]) * (nv5 - 0.0));let eq33_e695_d_n3: f64 = ((-s.dn[1791][3]) * (nv5 - 0.0));let eq33_e695_d_n4: f64 = ((-s.dn[1791][4]) * (nv5 - 0.0));let eq33_e695_d_n5: f64 = (((-s.dn[1791][5]) * (nv5 - 0.0)) + eq33_e693);let eq33_e695_d_n6: f64 = ((-s.dn[1791][6]) * (nv5 - 0.0));let eq33_e695_d_n7: f64 = ((-s.dn[1791][7]) * (nv5 - 0.0));let eq33_e695_d_n8: f64 = ((-s.dn[1791][8]) * (nv5 - 0.0));let eq33_e695_d_n9: f64 = ((-s.dn[1791][9]) * (nv5 - 0.0));let eq33_e695_d_b0: f64 = ((-s.db[1791][0]) * (nv5 - 0.0));let eq33_e695_d_b1: f64 = ((-s.db[1791][1]) * (nv5 - 0.0));let eq33_e695_d_b2: f64 = ((-s.db[1791][2]) * (nv5 - 0.0));let eq33_e695_d_b3: f64 = ((-s.db[1791][3]) * (nv5 - 0.0));let eq33_e696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq33_e695);let eq33_value: f64 = eq33_e696;let eq33_node_derivatives: [f64; 10] = [(eq33_e695_d_n0 * ddt_scale), (eq33_e695_d_n1 * ddt_scale), (eq33_e695_d_n2 * ddt_scale), (eq33_e695_d_n3 * ddt_scale), (eq33_e695_d_n4 * ddt_scale), (eq33_e695_d_n5 * ddt_scale), (eq33_e695_d_n6 * ddt_scale), (eq33_e695_d_n7 * ddt_scale), (eq33_e695_d_n8 * ddt_scale), (eq33_e695_d_n9 * ddt_scale)];let eq33_branch_derivatives: [f64; 4] = [(eq33_e695_d_b0 * ddt_scale), (eq33_e695_d_b1 * ddt_scale), (eq33_e695_d_b2 * ddt_scale), (eq33_e695_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e698: f64 = (-s.v[1792]);let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));let eq34_e700_d_n0: f64 = ((-s.dn[1792][0]) * (nv5 - 0.0));let eq34_e700_d_n1: f64 = ((-s.dn[1792][1]) * (nv5 - 0.0));let eq34_e700_d_n2: f64 = ((-s.dn[1792][2]) * (nv5 - 0.0));let eq34_e700_d_n3: f64 = ((-s.dn[1792][3]) * (nv5 - 0.0));let eq34_e700_d_n4: f64 = ((-s.dn[1792][4]) * (nv5 - 0.0));let eq34_e700_d_n5: f64 = (((-s.dn[1792][5]) * (nv5 - 0.0)) + eq34_e698);let eq34_e700_d_n6: f64 = ((-s.dn[1792][6]) * (nv5 - 0.0));let eq34_e700_d_n7: f64 = ((-s.dn[1792][7]) * (nv5 - 0.0));let eq34_e700_d_n8: f64 = ((-s.dn[1792][8]) * (nv5 - 0.0));let eq34_e700_d_n9: f64 = ((-s.dn[1792][9]) * (nv5 - 0.0));let eq34_e700_d_b0: f64 = ((-s.db[1792][0]) * (nv5 - 0.0));let eq34_e700_d_b1: f64 = ((-s.db[1792][1]) * (nv5 - 0.0));let eq34_e700_d_b2: f64 = ((-s.db[1792][2]) * (nv5 - 0.0));let eq34_e700_d_b3: f64 = ((-s.db[1792][3]) * (nv5 - 0.0));let eq34_e701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq34_e700);let eq34_value: f64 = eq34_e701;let eq34_node_derivatives: [f64; 10] = [(eq34_e700_d_n0 * ddt_scale), (eq34_e700_d_n1 * ddt_scale), (eq34_e700_d_n2 * ddt_scale), (eq34_e700_d_n3 * ddt_scale), (eq34_e700_d_n4 * ddt_scale), (eq34_e700_d_n5 * ddt_scale), (eq34_e700_d_n6 * ddt_scale), (eq34_e700_d_n7 * ddt_scale), (eq34_e700_d_n8 * ddt_scale), (eq34_e700_d_n9 * ddt_scale)];let eq34_branch_derivatives: [f64; 4] = [(eq34_e700_d_b0 * ddt_scale), (eq34_e700_d_b1 * ddt_scale), (eq34_e700_d_b2 * ddt_scale), (eq34_e700_d_b3 * ddt_scale)];
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
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq23_e631_q: f64 = s.v[358];let eq23_e633_q: f64 = s.v[373];let eq23_e634: f64 = (s.v[358] + s.v[373]);let eq23_e634_d_n0: f64 = (s.dn[358][0] + s.dn[373][0]);let eq23_e634_d_n1: f64 = (s.dn[358][1] + s.dn[373][1]);let eq23_e634_d_n2: f64 = (s.dn[358][2] + s.dn[373][2]);let eq23_e634_d_n3: f64 = (s.dn[358][3] + s.dn[373][3]);let eq23_e634_d_n4: f64 = (s.dn[358][4] + s.dn[373][4]);let eq23_e634_d_n5: f64 = (s.dn[358][5] + s.dn[373][5]);let eq23_e634_d_n6: f64 = (s.dn[358][6] + s.dn[373][6]);let eq23_e634_d_n7: f64 = (s.dn[358][7] + s.dn[373][7]);let eq23_e634_d_n8: f64 = (s.dn[358][8] + s.dn[373][8]);let eq23_e634_d_n9: f64 = (s.dn[358][9] + s.dn[373][9]);let eq23_e634_d_b0: f64 = (s.db[358][0] + s.db[373][0]);let eq23_e634_d_b1: f64 = (s.db[358][1] + s.db[373][1]);let eq23_e634_d_b2: f64 = (s.db[358][2] + s.db[373][2]);let eq23_e634_d_b3: f64 = (s.db[358][3] + s.db[373][3]);let eq23_e634_q: f64 = (eq23_e631_q + eq23_e633_q);let eq23_e636_q: f64 = s.v[377];let eq23_e637: f64 = (eq23_e634 + s.v[377]);let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + s.dn[377][0]);let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + s.dn[377][1]);let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + s.dn[377][2]);let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + s.dn[377][3]);let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + s.dn[377][4]);let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + s.dn[377][5]);let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + s.dn[377][6]);let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + s.dn[377][7]);let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + s.dn[377][8]);let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + s.dn[377][9]);let eq23_e637_d_b0: f64 = (eq23_e634_d_b0 + s.db[377][0]);let eq23_e637_d_b1: f64 = (eq23_e634_d_b1 + s.db[377][1]);let eq23_e637_d_b2: f64 = (eq23_e634_d_b2 + s.db[377][2]);let eq23_e637_d_b3: f64 = (eq23_e634_d_b3 + s.db[377][3]);let eq23_e637_q: f64 = (eq23_e634_q + eq23_e636_q);let eq23_e638: f64 = (p[14] * eq23_e637);let eq23_e638_d_n0: f64 = (p[14] * eq23_e637_d_n0);let eq23_e638_d_n1: f64 = (p[14] * eq23_e637_d_n1);let eq23_e638_d_n2: f64 = (p[14] * eq23_e637_d_n2);let eq23_e638_d_n3: f64 = (p[14] * eq23_e637_d_n3);let eq23_e638_d_n4: f64 = (p[14] * eq23_e637_d_n4);let eq23_e638_d_n5: f64 = (p[14] * eq23_e637_d_n5);let eq23_e638_d_n6: f64 = (p[14] * eq23_e637_d_n6);let eq23_e638_d_n7: f64 = (p[14] * eq23_e637_d_n7);let eq23_e638_d_n8: f64 = (p[14] * eq23_e637_d_n8);let eq23_e638_d_n9: f64 = (p[14] * eq23_e637_d_n9);let eq23_e638_d_b0: f64 = (p[14] * eq23_e637_d_b0);let eq23_e638_d_b1: f64 = (p[14] * eq23_e637_d_b1);let eq23_e638_d_b2: f64 = (p[14] * eq23_e637_d_b2);let eq23_e638_d_b3: f64 = (p[14] * eq23_e637_d_b3);let eq23_e638_q: f64 = (p[14] * eq23_e637_q);let eq23_reactive_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];let eq23_reactive_branch_derivatives: [f64; 4] = [eq23_e638_d_b0, eq23_e638_d_b1, eq23_e638_d_b2, eq23_e638_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq23_reactive_node_derivatives,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
