#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1608] {s.copy_ad(473, 1606);s.copy_ad(474, 1607);}
        if (!s.b[1608]) {s.copy_ad(440, 383);s.copy_ad(441, 384);s.copy_ad(442, 385);s.copy_ad(443, 386);s.copy_ad(444, 387);s.copy_ad(445, 388);s.copy_ad(446, 389);s.copy_ad(447, 390);s.copy_ad(448, 391);s.copy_ad(449, 393);s.copy_ad(450, 394);s.copy_ad(451, 395);s.copy_ad(452, 396);s.copy_ad(453, 397);s.copy_ad(454, 398);s.copy_ad(455, 399);s.copy_ad(456, 401);s.copy_ad(457, 402);s.copy_ad(458, 404);s.copy_ad(459, 405);s.copy_ad(460, 406);s.copy_ad(461, 408);s.copy_ad(462, 409);s.copy_ad(463, 414);s.copy_ad(464, 415);s.copy_ad(465, 416);s.copy_ad(466, 419);s.copy_ad(467, 420);s.copy_ad(468, 428);s.copy_ad(469, 430);s.copy_ad(470, 431);s.copy_ad(471, 436);s.copy_ad(472, 437);s.copy_ad(473, 438);s.copy_ad(474, 439);}
        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(448), s.ad_value(446)), 1.0, A::scale_offset(s.ad_value(464), 0.25, 1.0), 1.0);s.store_add_scaled_inputs3_indices(1324, 458, 0.5, 461, 0.5, 0, 1.0);s.store_add_scaled_inputs3_indices(1325, 459, 0.5, 462, 0.5, 0, -1.0);s.b[1762] = (p[13] > 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if s.b[1762] {s.store_add_scaled_inputs3_mixed_iai(1326, 1324, 1.0, A::div(s.ad_value(466), s.ad_value(469)), 1.0, 466, -1.0);s.store_add_scaled_inputs3_mixed_iai(1327, 1325, 1.0, A::div(s.ad_value(467), s.ad_value(470)), 1.0, 467, -1.0);}
        if (!s.b[1762]) {s.copy_ad(1326, 1324);s.copy_ad(1327, 1325);}
        s.store_scaled_mul(2, 471, 473, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 471, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(473), 1.0, A::scale(s.ad_value(473), 0.2)), 1.0);s.store_add_scaled_product_indices(1328, 3, 1.0, 1326, 465, 0.5);s.store_add_scaled_product_indices(1326, 2, 1.0, 1326, 465, 1.0);s.store_scaled_mul(2, 472, 474, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 472, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(474), 1.0, A::scale(s.ad_value(474), 0.2)), 1.0);s.store_add_scaled_inputs(1329, 1327, 0.5, 3, 1.0);s.store_add(1327, 1327, 2);s.store_mul(0, 447, 287);s.store_mul(361, 0, 1326);s.store_mul(362, 0, 1327);s.store_mul_add_scaled_inputs_rhs_indices(363, 0, 1328, -1.0, 1329, -1.0);s.b[1763] = (s.v[119] > 0.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if s.b[1763] {s.store_offset(0, 254, (2.0 * 0.6931471805599));s.store_add(1330, 460, 0);s.store_add(1331, 463, 0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1332, 1330, 0.5, 254, 0.5, 1330, 254, 9.0, (-0.5));s.store_add_scaled_inputs4_mixed_iiia(1333, 1331, 0.5, 254, 0.5, 339, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0), 9.0), (-0.5));s.store_mul_sqrt_mixed_ia(1334, 294, A::mul_offset_rhs(s.ad_value(445), s.ad_value(444), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1763] {s.store_mul_sqrt_mixed_ia(1335, 294, A::mul_offset_rhs(A::mul3(s.ad_value(445), s.ad_value(456), s.ad_value(444)), s.ad_value(443), 0.5));s.store_mul_square_lhs(1336, 1334, 291);s.store_mul_square_lhs(1337, 1335, 291);s.store_sub(2, 292, 1332);s.store_add_scaled_inputs3_indices(3, 292, 1.0, 339, 1.0, 1333, -1.0);s.store_scale(0, 1336, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1338, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1339, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_scale(0, 1337, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1340, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1341, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_mul(0, 293, 447);s.store_mul_product3_indices(2, 451, 0, 1334, 456, -1.0);s.store_mul_product3_indices(3, 452, 0, 1335, 457, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1338, 0.5, 1330, ((-1.0) * 0.5), 1338, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(379, 2, 0, 0, 1.0, A::sub(s.ad_value(1338), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1339, 0.5, 1331, ((-1.0) * 0.5), 1339, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(380, 2, 0, 0, 1.0, A::sub(s.ad_value(1339), s.ad_value(1333)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1340, 0.5, 1330, ((-1.0) * 0.5), 1340, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(381, 3, 0, 0, 1.0, A::sub(s.ad_value(1340), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1341, 0.5, 1331, ((-1.0) * 0.5), 1341, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(382, 3, 0, 0, 1.0, A::sub(s.ad_value(1341), s.ad_value(1333)), 1.0);}
        if (!s.b[1763]) {s.store_scalar(379, 0.0);s.store_scalar(380, 0.0);s.store_scalar(381, 0.0);s.store_scalar(382, 0.0);}
        s.store_mul(370, 164, 330);s.store_mul(371, 165, 332);s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), 0.2, 0.5);s.store_mul3_lhs(372, 159, 349, 0);s.store_mul3_lhs(373, 160, 350, 0);s.store_mul(374, 117, 338);s.store_mul(375, 166, 336);s.store_mul_scale_offset_mixed_ia(377, 331, A::add_scaled_products(s.ad_value(240), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(376, 333, A::add_scaled_products(s.ad_value(240), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1764] = (s.v[6] > 0.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if s.b[1764] {s.store_mul(378, 170, 219);}
        if (!s.b[1764]) {s.store_scalar(378, 0.0);}
        s.copy_ad(1774, 361);s.copy_ad(1775, 362);s.copy_ad(1776, 363);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.b[1777] = (s.v[334] < 0.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if s.b[1777] {s.copy_ad(1776, 364);}
        s.store_scaled_mul(361, 13, 361, p[32]);s.store_scaled_mul(362, 13, 362, p[32]);s.store_scaled_mul(363, 13, 363, p[32]);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.store_scaled_mul(379, 13, 379, p[32]);s.store_scaled_mul(380, 13, 380, p[32]);s.store_scaled_mul(381, 13, 381, p[32]);s.store_scaled_mul(382, 13, 382, p[32]);s.store_scaled_mul(370, 13, 370, p[32]);s.store_scaled_mul(371, 13, 371, p[32]);s.store_scaled_mul(372, 13, 372, p[32]);s.store_scaled_mul(373, 13, 373, p[32]);s.store_scaled_mul(374, 13, 374, p[32]);s.store_scaled_mul(377, 13, 377, p[32]);s.store_scaled_mul(376, 13, 376, p[32]);s.store_scaled_mul(375, 13, 375, p[32]);s.store_mul(378, 13, 378);s.b[1778] = (s.v[334] < 0.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if s.b[1778] {s.copy_ad(1772, 363);s.copy_ad(363, 364);s.copy_ad(364, 1772);s.store_neg(375, 375);s.copy_ad(1772, 380);s.copy_ad(380, 379);s.copy_ad(379, 1772);s.copy_ad(1772, 382);s.copy_ad(382, 381);s.copy_ad(381, 1772);}
        s.b[1779] = (s.v[13] > 0.0);s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if s.b[1779] {s.store_mul_div_scaled_inputs_mixed_aia(1773, A::add_scaled_product(A::div_scaled_product_by_product(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775)), 1.0, s.ad_value(116), s.ad_value(239), 1.0), 1.0, s.ad_value(180), s.ad_value(226), 1.0), 342, 1e-9, A::mul(s.ad_value(345), s.ad_value(116)), 1.0);}
        if (!s.b[1779]) {s.store_scalar(1773, 0.0);}
        s.store_scaled_mul(1780, 390, 226, 1.0 / (1.602176565e-19));s.store_scaled_add(1781, 407, 432, (-0.5));s.store_add(1782, 415, 1781);s.store_div(0, 415, 1782);s.store_scaled_add_sqrt_square_offset_rhs(1787, 0, 0, 1e-20, 0.5);s.store_scaled_mul(1788, 436, 435, (-0.1666666666667));s.store_square(1789, 1788);s.store_offset(1790, 429, (-1.0));s.store_scale(1794, 1789, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1787, 1.0, 1794, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794), s.ad_value(1790), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1799, 456, 447, 116, 1.0, 469, 1.0);s.store_mul_scale_offset_indices(1800, 1799, 468, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1802, 1800, A::mul_scaled_lhs(s.ad_value(334), 0.25, s.ad_value(1788)), -1.0, 0.5);s.store_sub(1801, 1800, 1802);s.b[1813] = (p[6] > 0.0);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1813] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1787), 0.08333333333333333, s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)), (-1.0)), A::mul3_scaled_output(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794)), s.ad_value(1790), 1.6));s.store_max_with_scalar(3, 2, 1e-40);}
        s.copy_ad(1783, 1780);s.store_mul_scale_offset_indices(1784, 1780, 415, 1.0, 1.0);s.store_mul_sub_rhs(1785, 1780, 403, 413);s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5, A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1783), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1783), s.ad_value(1783)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1783), 2.0), 1.0), 1785, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(417), 1.0, s.ad_value(177), s.ad_value(418), 1.0), A::offset(s.ad_value(415), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(347), s.ad_value(348), 1.602176565e-19, s.ad_value(345), 1.0), 3, 1.0, 1783, 1.0);s.store_div_from_scalar_scaled_input(1823, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1824, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1825, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1826, 15, 1825, 1.0, 1824, (-1.0), 228, (-0.4), 0.0);s.store_add(1827, 1824, 1826);s.store_scaled_mul(1828, 1827, 1823, 0.5);s.store_sub_scaled_inputs(1829, 15, 0.05, 1826, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1830, 2, 238);s.store_div_scaled_value_offset_denominator(1831, s.ad_value(1823), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1833, 1830, 229, (2.0 * 1.602176565e-19), 0.0, 1831);s.store_add_offset_lhs_mixed_ai(1834, A::ln(A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(1833), 1.0)), (-0.6931471805599), 1828);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_div_scaled_product_mixed_iiia(1835, 1831, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(1838, 35, 1831);s.store_scalar(1839, 0.0);s.store_scalar(1832, 0.0);s.b[1884] = (p[9] > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1832, 1.0, 1823, A::ln(A::div(s.ad_value(24), s.ad_value(251))));}
        s.b[1885] = (p[13] > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });s.b[1886] = (p[14] == 1.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if (s.b[1885] && s.b[1886]) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p[13]) * 1.27520989));}
        if (s.b[1885] && (!s.b[1886])) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p[13]) * 1.5412087));}
        s.store_mul(1842, 336, 1831);s.store_mul_scale_offset_mixed_ia(1843, 1831, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(1844, 1842, 1843, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1815, 402, 1.0, 401, A::offset(s.ad_value(402), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 401, 1.0, 402, A::offset(s.ad_value(401), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(401), A::offset(s.ad_value(1815), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(402), A::offset(s.ad_value(1816), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 399, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(399), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(398), -1.0, s.ad_value(25), 1.0), 398);s.store_add_scaled_product_mixed_iia(1822, 21, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(394), (-1.0), s.ad_value(395), 1.0), 1.0, s.ad_value(397), (-1.0), s.ad_value(394), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p[14]);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(183), p[14], s.ad_value(1829), p[14], s.ad_value(243), p[14], s.ad_value(0), 1.0), p[34], 1832);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs4_indices(1841, 184, p[14], 1829, p[14], 244, p[14], 0, 1.0);s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);s.b[1887] = (p[2] > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if s.b[1887] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p[14], 260, 1.0);}
        s.b[1888] = (s.v[0] < 0.0);s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });
        if (s.b[1887] && s.b[1888]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if (s.b[1887] && (!s.b[1888])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if s.b[1887] {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p[14]);}
        if (!s.b[1887]) {s.copy_ad(1847, 1846);}
        s.store_mul_sub_rhs(0, 248, 1845, 1847);s.b[1889] = (p[13] > 0.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
        if s.b[1889] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1851, 246, 4, 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1852, 247, 4, 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));}
        if (!s.b[1889]) {s.copy_ad(1851, 246);s.copy_ad(1852, 247);s.copy_ad(1853, 248);}
        s.store_mul_sub_rhs(1854, 1853, 1845, 1847);s.b[1890] = (s.v[1854] > 0.0);s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });s.b[1891] = ((-s.v[1854]) < 80.0);s.store_scalar(1891, if s.b[1891] { 1.0 } else { 0.0 });
        if (s.b[1890] && s.b[1891]) {s.store_ln_one_plus_exp_neg_input(0, 1854);}
        if (s.b[1890] && (!s.b[1891])) {s.store_neg(0, 1854);}
        if s.b[1890] {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1892] = (s.v[1854] < 80.0);s.store_scalar(1892, if s.b[1892] { 1.0 } else { 0.0 });
        if ((!s.b[1890]) && s.b[1892]) {s.store_ln_one_plus_exp(0, 1854);}
        if ((!s.b[1890]) && (!s.b[1892])) {s.copy_ad(0, 1854);}
        if (!s.b[1890]) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);s.b[1894] = (p[11] > 0.0);s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });
        if s.b[1894] {s.store_div_scaled_value_by_product_mixed_iia(1815, 457, 1.0, 456, A::offset(s.ad_value(457), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 456, 1.0, 457, A::offset(s.ad_value(456), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(456), A::offset(s.ad_value(1815), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(457), A::offset(s.ad_value(1816), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 455, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(455), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(454), -1.0, s.ad_value(25), 1.0), 454);s.store_add_scaled_product_mixed_iia(1822, 130, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(450), (-1.0), s.ad_value(451), 1.0), 1.0, s.ad_value(453), (-1.0), s.ad_value(450), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p[14]);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(185), p[14], s.ad_value(1829), p[14], s.ad_value(243), p[14], s.ad_value(0), 1.0), p[34], 1832);s.store_add_scaled_inputs4_indices(1841, 186, p[14], 1829, p[14], 244, p[14], 0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1894] {s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);}
        s.b[1895] = (p[2] > 0.0);s.store_scalar(1895, if s.b[1895] { 1.0 } else { 0.0 });
        if (s.b[1894] && s.b[1895]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p[14], 260, 1.0);}
        s.b[1896] = (s.v[0] < 0.0);s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });
        if ((s.b[1894] && s.b[1895]) && s.b[1896]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if ((s.b[1894] && s.b[1895]) && (!s.b[1896])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if (s.b[1894] && s.b[1895]) {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p[14]);}
        if (s.b[1894] && (!s.b[1895])) {s.copy_ad(1847, 1846);}
        if s.b[1894] {s.store_mul_sub_rhs(0, 248, 1845, 1847);}
        s.b[1897] = (p[13] > 0.0);s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });
        if (s.b[1894] && s.b[1897]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1851, 246, 4, 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1852, 247, 4, 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));}
        if (s.b[1894] && (!s.b[1897])) {s.copy_ad(1851, 246);s.copy_ad(1852, 247);s.copy_ad(1853, 248);}
        if s.b[1894] {s.store_mul_sub_rhs(1854, 1853, 1845, 1847);}
        s.b[1898] = (s.v[1854] > 0.0);s.store_scalar(1898, if s.b[1898] { 1.0 } else { 0.0 });s.b[1899] = ((-s.v[1854]) < 80.0);s.store_scalar(1899, if s.b[1899] { 1.0 } else { 0.0 });
        if ((s.b[1894] && s.b[1898]) && s.b[1899]) {s.store_ln_one_plus_exp_neg_input(0, 1854);}
        if ((s.b[1894] && s.b[1898]) && (!s.b[1899])) {s.store_neg(0, 1854);}
        if (s.b[1894] && s.b[1898]) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1900] = (s.v[1854] < 80.0);s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });
        if ((s.b[1894] && (!s.b[1898])) && s.b[1900]) {s.store_ln_one_plus_exp(0, 1854);}
        if ((s.b[1894] && (!s.b[1898])) && (!s.b[1900])) {s.copy_ad(0, 1854);}
        if (s.b[1894] && (!s.b[1898])) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));}
        if s.b[1894] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1894] {s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);}
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
        let (eq0_e510, eq0_e510_d_n0, eq0_e510_d_n1, eq0_e510_d_n2, eq0_e510_d_n3, eq0_e510_d_n4, eq0_e510_d_n5, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9, eq0_e510_d_n10, eq0_e510_d_n11, eq0_e510_d_n12, eq0_e510_d_n13, eq0_e510_d_b0, eq0_e510_d_b1, eq0_e510_d_b2, eq0_e510_d_b3,) = {
    if s.b[1767] {
        let eq0_e508: f64 = (p[14] * s.v[365]);
        (eq0_e508, (p[14] * s.dn[365][0]), (p[14] * s.dn[365][1]), (p[14] * s.dn[365][2]), (p[14] * s.dn[365][3]), (p[14] * s.dn[365][4]), (p[14] * s.dn[365][5]), (p[14] * s.dn[365][6]), (p[14] * s.dn[365][7]), (p[14] * s.dn[365][8]), (p[14] * s.dn[365][9]), (p[14] * s.dn[365][10]), (p[14] * s.dn[365][11]), (p[14] * s.dn[365][12]), (p[14] * s.dn[365][13]), (p[14] * s.db[365][0]), (p[14] * s.db[365][1]), (p[14] * s.db[365][2]), (p[14] * s.db[365][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e510;let eq0_node_derivatives: [f64; 14] = [eq0_e510_d_n0, eq0_e510_d_n1, eq0_e510_d_n2, eq0_e510_d_n3, eq0_e510_d_n4, eq0_e510_d_n5, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9, eq0_e510_d_n10, eq0_e510_d_n11, eq0_e510_d_n12, eq0_e510_d_n13];let eq0_branch_derivatives: [f64; 4] = [eq0_e510_d_b0, eq0_e510_d_b1, eq0_e510_d_b2, eq0_e510_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e517, eq1_e517_d_n0, eq1_e517_d_n1, eq1_e517_d_n2, eq1_e517_d_n3, eq1_e517_d_n4, eq1_e517_d_n5, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9, eq1_e517_d_n10, eq1_e517_d_n11, eq1_e517_d_n12, eq1_e517_d_n13, eq1_e517_d_b0, eq1_e517_d_b1, eq1_e517_d_b2, eq1_e517_d_b3,) = {
    if (!s.b[1767]) {
        let eq1_e515: f64 = (p[14] * s.v[365]);
        (eq1_e515, (p[14] * s.dn[365][0]), (p[14] * s.dn[365][1]), (p[14] * s.dn[365][2]), (p[14] * s.dn[365][3]), (p[14] * s.dn[365][4]), (p[14] * s.dn[365][5]), (p[14] * s.dn[365][6]), (p[14] * s.dn[365][7]), (p[14] * s.dn[365][8]), (p[14] * s.dn[365][9]), (p[14] * s.dn[365][10]), (p[14] * s.dn[365][11]), (p[14] * s.dn[365][12]), (p[14] * s.dn[365][13]), (p[14] * s.db[365][0]), (p[14] * s.db[365][1]), (p[14] * s.db[365][2]), (p[14] * s.db[365][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e517;let eq1_node_derivatives: [f64; 14] = [eq1_e517_d_n0, eq1_e517_d_n1, eq1_e517_d_n2, eq1_e517_d_n3, eq1_e517_d_n4, eq1_e517_d_n5, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9, eq1_e517_d_n10, eq1_e517_d_n11, eq1_e517_d_n12, eq1_e517_d_n13];let eq1_branch_derivatives: [f64; 4] = [eq1_e517_d_b0, eq1_e517_d_b1, eq1_e517_d_b2, eq1_e517_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );let eq2_e521: f64 = (s.v[368] - s.v[369]);let eq2_e521_d_n0: f64 = (s.dn[368][0] - s.dn[369][0]);let eq2_e521_d_n1: f64 = (s.dn[368][1] - s.dn[369][1]);let eq2_e521_d_n2: f64 = (s.dn[368][2] - s.dn[369][2]);let eq2_e521_d_n3: f64 = (s.dn[368][3] - s.dn[369][3]);let eq2_e521_d_n4: f64 = (s.dn[368][4] - s.dn[369][4]);let eq2_e521_d_n5: f64 = (s.dn[368][5] - s.dn[369][5]);let eq2_e521_d_n6: f64 = (s.dn[368][6] - s.dn[369][6]);let eq2_e521_d_n7: f64 = (s.dn[368][7] - s.dn[369][7]);let eq2_e521_d_n8: f64 = (s.dn[368][8] - s.dn[369][8]);let eq2_e521_d_n9: f64 = (s.dn[368][9] - s.dn[369][9]);let eq2_e521_d_n10: f64 = (s.dn[368][10] - s.dn[369][10]);let eq2_e521_d_n11: f64 = (s.dn[368][11] - s.dn[369][11]);let eq2_e521_d_n12: f64 = (s.dn[368][12] - s.dn[369][12]);let eq2_e521_d_n13: f64 = (s.dn[368][13] - s.dn[369][13]);let eq2_e521_d_b0: f64 = (s.db[368][0] - s.db[369][0]);let eq2_e521_d_b1: f64 = (s.db[368][1] - s.db[369][1]);let eq2_e521_d_b2: f64 = (s.db[368][2] - s.db[369][2]);let eq2_e521_d_b3: f64 = (s.db[368][3] - s.db[369][3]);let eq2_e522: f64 = (p[14] * eq2_e521);let eq2_e522_d_n0: f64 = (p[14] * eq2_e521_d_n0);let eq2_e522_d_n1: f64 = (p[14] * eq2_e521_d_n1);let eq2_e522_d_n2: f64 = (p[14] * eq2_e521_d_n2);let eq2_e522_d_n3: f64 = (p[14] * eq2_e521_d_n3);let eq2_e522_d_n4: f64 = (p[14] * eq2_e521_d_n4);let eq2_e522_d_n5: f64 = (p[14] * eq2_e521_d_n5);let eq2_e522_d_n6: f64 = (p[14] * eq2_e521_d_n6);let eq2_e522_d_n7: f64 = (p[14] * eq2_e521_d_n7);let eq2_e522_d_n8: f64 = (p[14] * eq2_e521_d_n8);let eq2_e522_d_n9: f64 = (p[14] * eq2_e521_d_n9);let eq2_e522_d_n10: f64 = (p[14] * eq2_e521_d_n10);let eq2_e522_d_n11: f64 = (p[14] * eq2_e521_d_n11);let eq2_e522_d_n12: f64 = (p[14] * eq2_e521_d_n12);let eq2_e522_d_n13: f64 = (p[14] * eq2_e521_d_n13);let eq2_e522_d_b0: f64 = (p[14] * eq2_e521_d_b0);let eq2_e522_d_b1: f64 = (p[14] * eq2_e521_d_b1);let eq2_e522_d_b2: f64 = (p[14] * eq2_e521_d_b2);let eq2_e522_d_b3: f64 = (p[14] * eq2_e521_d_b3);let eq2_value: f64 = eq2_e522;let eq2_node_derivatives: [f64; 14] = [eq2_e522_d_n0, eq2_e522_d_n1, eq2_e522_d_n2, eq2_e522_d_n3, eq2_e522_d_n4, eq2_e522_d_n5, eq2_e522_d_n6, eq2_e522_d_n7, eq2_e522_d_n8, eq2_e522_d_n9, eq2_e522_d_n10, eq2_e522_d_n11, eq2_e522_d_n12, eq2_e522_d_n13];let eq2_branch_derivatives: [f64; 4] = [eq2_e522_d_b0, eq2_e522_d_b1, eq2_e522_d_b2, eq2_e522_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );let eq3_e525: f64 = (p[14] * s.v[366]);let eq3_value: f64 = eq3_e525;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &s.dn[366],
            &s.db[366],
            (multiplicity) * (p[14]),
        );let eq4_e528: f64 = (p[14] * s.v[367]);let eq4_value: f64 = eq4_e528;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            &s.dn[367],
            &s.db[367],
            (multiplicity) * (p[14]),
        );let eq8_e534: f64 = (p[31] * s.v[475]);let eq8_e536: f64 = (eq8_e534 * (nv7 - nv6));let eq8_value: f64 = eq8_e536;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * ((-eq8_e534)),
            7,
            multiplicity * (eq8_e534),
        );let eq9_value: f64 = s.v[1765];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq9_value),
            &s.dn[1765],
            &s.db[1765],
            multiplicity,
        );let eq10_value: f64 = s.v[1766];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq10_value),
            &s.dn[1766],
            &s.db[1766],
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
        let (eq11_e548, eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13, eq11_e548_d_b0, eq11_e548_d_b1, eq11_e548_d_b2, eq11_e548_d_b3,) = {
    if s.b[1768] {
        let eq11_e542: f64 = (p[31] * s.v[13]);let eq11_e544: f64 = (eq11_e542 * s.v[316]);let eq11_e544_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[316]) + (eq11_e542 * s.dn[316][0]));let eq11_e544_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[316]) + (eq11_e542 * s.dn[316][1]));let eq11_e544_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[316]) + (eq11_e542 * s.dn[316][2]));let eq11_e544_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[316]) + (eq11_e542 * s.dn[316][3]));let eq11_e544_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[316]) + (eq11_e542 * s.dn[316][4]));let eq11_e544_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[316]) + (eq11_e542 * s.dn[316][5]));let eq11_e544_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[316]) + (eq11_e542 * s.dn[316][6]));let eq11_e544_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[316]) + (eq11_e542 * s.dn[316][7]));let eq11_e544_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[316]) + (eq11_e542 * s.dn[316][8]));let eq11_e544_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[316]) + (eq11_e542 * s.dn[316][9]));let eq11_e544_d_n10: f64 = (((p[31] * s.dn[13][10]) * s.v[316]) + (eq11_e542 * s.dn[316][10]));let eq11_e544_d_n11: f64 = (((p[31] * s.dn[13][11]) * s.v[316]) + (eq11_e542 * s.dn[316][11]));let eq11_e544_d_n12: f64 = (((p[31] * s.dn[13][12]) * s.v[316]) + (eq11_e542 * s.dn[316][12]));let eq11_e544_d_n13: f64 = (((p[31] * s.dn[13][13]) * s.v[316]) + (eq11_e542 * s.dn[316][13]));let eq11_e544_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[316]) + (eq11_e542 * s.db[316][0]));let eq11_e544_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[316]) + (eq11_e542 * s.db[316][1]));let eq11_e544_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[316]) + (eq11_e542 * s.db[316][2]));let eq11_e544_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[316]) + (eq11_e542 * s.db[316][3]));let eq11_e546: f64 = (eq11_e544 * (nv1 - nv9));let eq11_e546_d_n0: f64 = (eq11_e544_d_n0 * (nv1 - nv9));let eq11_e546_d_n1: f64 = ((eq11_e544_d_n1 * (nv1 - nv9)) + eq11_e544);let eq11_e546_d_n2: f64 = (eq11_e544_d_n2 * (nv1 - nv9));let eq11_e546_d_n3: f64 = (eq11_e544_d_n3 * (nv1 - nv9));let eq11_e546_d_n4: f64 = (eq11_e544_d_n4 * (nv1 - nv9));let eq11_e546_d_n5: f64 = (eq11_e544_d_n5 * (nv1 - nv9));let eq11_e546_d_n6: f64 = (eq11_e544_d_n6 * (nv1 - nv9));let eq11_e546_d_n7: f64 = (eq11_e544_d_n7 * (nv1 - nv9));let eq11_e546_d_n8: f64 = (eq11_e544_d_n8 * (nv1 - nv9));let eq11_e546_d_n9: f64 = ((eq11_e544_d_n9 * (nv1 - nv9)) + (-eq11_e544));let eq11_e546_d_n10: f64 = (eq11_e544_d_n10 * (nv1 - nv9));let eq11_e546_d_n11: f64 = (eq11_e544_d_n11 * (nv1 - nv9));let eq11_e546_d_n12: f64 = (eq11_e544_d_n12 * (nv1 - nv9));let eq11_e546_d_n13: f64 = (eq11_e544_d_n13 * (nv1 - nv9));let eq11_e546_d_b0: f64 = (eq11_e544_d_b0 * (nv1 - nv9));let eq11_e546_d_b1: f64 = (eq11_e544_d_b1 * (nv1 - nv9));let eq11_e546_d_b2: f64 = (eq11_e544_d_b2 * (nv1 - nv9));let eq11_e546_d_b3: f64 = (eq11_e544_d_b3 * (nv1 - nv9));
        (eq11_e546, eq11_e546_d_n0, eq11_e546_d_n1, eq11_e546_d_n2, eq11_e546_d_n3, eq11_e546_d_n4, eq11_e546_d_n5, eq11_e546_d_n6, eq11_e546_d_n7, eq11_e546_d_n8, eq11_e546_d_n9, eq11_e546_d_n10, eq11_e546_d_n11, eq11_e546_d_n12, eq11_e546_d_n13, eq11_e546_d_b0, eq11_e546_d_b1, eq11_e546_d_b2, eq11_e546_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e548;let eq11_node_derivatives: [f64; 14] = [eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13];let eq11_branch_derivatives: [f64; 4] = [eq11_e548_d_b0, eq11_e548_d_b1, eq11_e548_d_b2, eq11_e548_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e563,) = {
    if (!s.b[1768]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e563;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e573, eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13, eq14_e573_d_b0, eq14_e573_d_b1, eq14_e573_d_b2, eq14_e573_d_b3,) = {
    if s.b[1769] {
        let eq14_e567: f64 = (p[31] * s.v[13]);let eq14_e569: f64 = (eq14_e567 * s.v[320]);let eq14_e569_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[320]) + (eq14_e567 * s.dn[320][0]));let eq14_e569_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[320]) + (eq14_e567 * s.dn[320][1]));let eq14_e569_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[320]) + (eq14_e567 * s.dn[320][2]));let eq14_e569_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[320]) + (eq14_e567 * s.dn[320][3]));let eq14_e569_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[320]) + (eq14_e567 * s.dn[320][4]));let eq14_e569_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[320]) + (eq14_e567 * s.dn[320][5]));let eq14_e569_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[320]) + (eq14_e567 * s.dn[320][6]));let eq14_e569_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[320]) + (eq14_e567 * s.dn[320][7]));let eq14_e569_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[320]) + (eq14_e567 * s.dn[320][8]));let eq14_e569_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[320]) + (eq14_e567 * s.dn[320][9]));let eq14_e569_d_n10: f64 = (((p[31] * s.dn[13][10]) * s.v[320]) + (eq14_e567 * s.dn[320][10]));let eq14_e569_d_n11: f64 = (((p[31] * s.dn[13][11]) * s.v[320]) + (eq14_e567 * s.dn[320][11]));let eq14_e569_d_n12: f64 = (((p[31] * s.dn[13][12]) * s.v[320]) + (eq14_e567 * s.dn[320][12]));let eq14_e569_d_n13: f64 = (((p[31] * s.dn[13][13]) * s.v[320]) + (eq14_e567 * s.dn[320][13]));let eq14_e569_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[320]) + (eq14_e567 * s.db[320][0]));let eq14_e569_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[320]) + (eq14_e567 * s.db[320][1]));let eq14_e569_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[320]) + (eq14_e567 * s.db[320][2]));let eq14_e569_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[320]) + (eq14_e567 * s.db[320][3]));let eq14_e571: f64 = (eq14_e569 * (nv2 - nv6));let eq14_e571_d_n0: f64 = (eq14_e569_d_n0 * (nv2 - nv6));let eq14_e571_d_n1: f64 = (eq14_e569_d_n1 * (nv2 - nv6));let eq14_e571_d_n2: f64 = ((eq14_e569_d_n2 * (nv2 - nv6)) + eq14_e569);let eq14_e571_d_n3: f64 = (eq14_e569_d_n3 * (nv2 - nv6));let eq14_e571_d_n4: f64 = (eq14_e569_d_n4 * (nv2 - nv6));let eq14_e571_d_n5: f64 = (eq14_e569_d_n5 * (nv2 - nv6));let eq14_e571_d_n6: f64 = ((eq14_e569_d_n6 * (nv2 - nv6)) + (-eq14_e569));let eq14_e571_d_n7: f64 = (eq14_e569_d_n7 * (nv2 - nv6));let eq14_e571_d_n8: f64 = (eq14_e569_d_n8 * (nv2 - nv6));let eq14_e571_d_n9: f64 = (eq14_e569_d_n9 * (nv2 - nv6));let eq14_e571_d_n10: f64 = (eq14_e569_d_n10 * (nv2 - nv6));let eq14_e571_d_n11: f64 = (eq14_e569_d_n11 * (nv2 - nv6));let eq14_e571_d_n12: f64 = (eq14_e569_d_n12 * (nv2 - nv6));let eq14_e571_d_n13: f64 = (eq14_e569_d_n13 * (nv2 - nv6));let eq14_e571_d_b0: f64 = (eq14_e569_d_b0 * (nv2 - nv6));let eq14_e571_d_b1: f64 = (eq14_e569_d_b1 * (nv2 - nv6));let eq14_e571_d_b2: f64 = (eq14_e569_d_b2 * (nv2 - nv6));let eq14_e571_d_b3: f64 = (eq14_e569_d_b3 * (nv2 - nv6));
        (eq14_e571, eq14_e571_d_n0, eq14_e571_d_n1, eq14_e571_d_n2, eq14_e571_d_n3, eq14_e571_d_n4, eq14_e571_d_n5, eq14_e571_d_n6, eq14_e571_d_n7, eq14_e571_d_n8, eq14_e571_d_n9, eq14_e571_d_n10, eq14_e571_d_n11, eq14_e571_d_n12, eq14_e571_d_n13, eq14_e571_d_b0, eq14_e571_d_b1, eq14_e571_d_b2, eq14_e571_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e573;let eq14_node_derivatives: [f64; 14] = [eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13];let eq14_branch_derivatives: [f64; 4] = [eq14_e573_d_b0, eq14_e573_d_b1, eq14_e573_d_b2, eq14_e573_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
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
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq16_e588,) = {
    if (!s.b[1769]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e588;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
        );
        let (eq17_e598, eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13, eq17_e598_d_b0, eq17_e598_d_b1, eq17_e598_d_b2, eq17_e598_d_b3,) = {
    if s.b[1770] {
        let eq17_e592: f64 = (p[31] * s.v[13]);let eq17_e594: f64 = (eq17_e592 * s.v[324]);let eq17_e594_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[324]) + (eq17_e592 * s.dn[324][0]));let eq17_e594_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[324]) + (eq17_e592 * s.dn[324][1]));let eq17_e594_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[324]) + (eq17_e592 * s.dn[324][2]));let eq17_e594_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[324]) + (eq17_e592 * s.dn[324][3]));let eq17_e594_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[324]) + (eq17_e592 * s.dn[324][4]));let eq17_e594_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[324]) + (eq17_e592 * s.dn[324][5]));let eq17_e594_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[324]) + (eq17_e592 * s.dn[324][6]));let eq17_e594_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[324]) + (eq17_e592 * s.dn[324][7]));let eq17_e594_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[324]) + (eq17_e592 * s.dn[324][8]));let eq17_e594_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[324]) + (eq17_e592 * s.dn[324][9]));let eq17_e594_d_n10: f64 = (((p[31] * s.dn[13][10]) * s.v[324]) + (eq17_e592 * s.dn[324][10]));let eq17_e594_d_n11: f64 = (((p[31] * s.dn[13][11]) * s.v[324]) + (eq17_e592 * s.dn[324][11]));let eq17_e594_d_n12: f64 = (((p[31] * s.dn[13][12]) * s.v[324]) + (eq17_e592 * s.dn[324][12]));let eq17_e594_d_n13: f64 = (((p[31] * s.dn[13][13]) * s.v[324]) + (eq17_e592 * s.dn[324][13]));let eq17_e594_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[324]) + (eq17_e592 * s.db[324][0]));let eq17_e594_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[324]) + (eq17_e592 * s.db[324][1]));let eq17_e594_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[324]) + (eq17_e592 * s.db[324][2]));let eq17_e594_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[324]) + (eq17_e592 * s.db[324][3]));let eq17_e596: f64 = (eq17_e594 * (nv0 - nv7));let eq17_e596_d_n0: f64 = ((eq17_e594_d_n0 * (nv0 - nv7)) + eq17_e594);let eq17_e596_d_n1: f64 = (eq17_e594_d_n1 * (nv0 - nv7));let eq17_e596_d_n2: f64 = (eq17_e594_d_n2 * (nv0 - nv7));let eq17_e596_d_n3: f64 = (eq17_e594_d_n3 * (nv0 - nv7));let eq17_e596_d_n4: f64 = (eq17_e594_d_n4 * (nv0 - nv7));let eq17_e596_d_n5: f64 = (eq17_e594_d_n5 * (nv0 - nv7));let eq17_e596_d_n6: f64 = (eq17_e594_d_n6 * (nv0 - nv7));let eq17_e596_d_n7: f64 = ((eq17_e594_d_n7 * (nv0 - nv7)) + (-eq17_e594));let eq17_e596_d_n8: f64 = (eq17_e594_d_n8 * (nv0 - nv7));let eq17_e596_d_n9: f64 = (eq17_e594_d_n9 * (nv0 - nv7));let eq17_e596_d_n10: f64 = (eq17_e594_d_n10 * (nv0 - nv7));let eq17_e596_d_n11: f64 = (eq17_e594_d_n11 * (nv0 - nv7));let eq17_e596_d_n12: f64 = (eq17_e594_d_n12 * (nv0 - nv7));let eq17_e596_d_n13: f64 = (eq17_e594_d_n13 * (nv0 - nv7));let eq17_e596_d_b0: f64 = (eq17_e594_d_b0 * (nv0 - nv7));let eq17_e596_d_b1: f64 = (eq17_e594_d_b1 * (nv0 - nv7));let eq17_e596_d_b2: f64 = (eq17_e594_d_b2 * (nv0 - nv7));let eq17_e596_d_b3: f64 = (eq17_e594_d_b3 * (nv0 - nv7));
        (eq17_e596, eq17_e596_d_n0, eq17_e596_d_n1, eq17_e596_d_n2, eq17_e596_d_n3, eq17_e596_d_n4, eq17_e596_d_n5, eq17_e596_d_n6, eq17_e596_d_n7, eq17_e596_d_n8, eq17_e596_d_n9, eq17_e596_d_n10, eq17_e596_d_n11, eq17_e596_d_n12, eq17_e596_d_n13, eq17_e596_d_b0, eq17_e596_d_b1, eq17_e596_d_b2, eq17_e596_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e598;let eq17_node_derivatives: [f64; 14] = [eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13];let eq17_branch_derivatives: [f64; 4] = [eq17_e598_d_b0, eq17_e598_d_b1, eq17_e598_d_b2, eq17_e598_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e613,) = {
    if (!s.b[1770]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e613;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq20_e623, eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13, eq20_e623_d_b0, eq20_e623_d_b1, eq20_e623_d_b2, eq20_e623_d_b3,) = {
    if s.b[1771] {
        let eq20_e617: f64 = (p[31] * s.v[13]);let eq20_e619: f64 = (eq20_e617 * s.v[327]);let eq20_e619_d_n0: f64 = (((p[31] * s.dn[13][0]) * s.v[327]) + (eq20_e617 * s.dn[327][0]));let eq20_e619_d_n1: f64 = (((p[31] * s.dn[13][1]) * s.v[327]) + (eq20_e617 * s.dn[327][1]));let eq20_e619_d_n2: f64 = (((p[31] * s.dn[13][2]) * s.v[327]) + (eq20_e617 * s.dn[327][2]));let eq20_e619_d_n3: f64 = (((p[31] * s.dn[13][3]) * s.v[327]) + (eq20_e617 * s.dn[327][3]));let eq20_e619_d_n4: f64 = (((p[31] * s.dn[13][4]) * s.v[327]) + (eq20_e617 * s.dn[327][4]));let eq20_e619_d_n5: f64 = (((p[31] * s.dn[13][5]) * s.v[327]) + (eq20_e617 * s.dn[327][5]));let eq20_e619_d_n6: f64 = (((p[31] * s.dn[13][6]) * s.v[327]) + (eq20_e617 * s.dn[327][6]));let eq20_e619_d_n7: f64 = (((p[31] * s.dn[13][7]) * s.v[327]) + (eq20_e617 * s.dn[327][7]));let eq20_e619_d_n8: f64 = (((p[31] * s.dn[13][8]) * s.v[327]) + (eq20_e617 * s.dn[327][8]));let eq20_e619_d_n9: f64 = (((p[31] * s.dn[13][9]) * s.v[327]) + (eq20_e617 * s.dn[327][9]));let eq20_e619_d_n10: f64 = (((p[31] * s.dn[13][10]) * s.v[327]) + (eq20_e617 * s.dn[327][10]));let eq20_e619_d_n11: f64 = (((p[31] * s.dn[13][11]) * s.v[327]) + (eq20_e617 * s.dn[327][11]));let eq20_e619_d_n12: f64 = (((p[31] * s.dn[13][12]) * s.v[327]) + (eq20_e617 * s.dn[327][12]));let eq20_e619_d_n13: f64 = (((p[31] * s.dn[13][13]) * s.v[327]) + (eq20_e617 * s.dn[327][13]));let eq20_e619_d_b0: f64 = (((p[31] * s.db[13][0]) * s.v[327]) + (eq20_e617 * s.db[327][0]));let eq20_e619_d_b1: f64 = (((p[31] * s.db[13][1]) * s.v[327]) + (eq20_e617 * s.db[327][1]));let eq20_e619_d_b2: f64 = (((p[31] * s.db[13][2]) * s.v[327]) + (eq20_e617 * s.db[327][2]));let eq20_e619_d_b3: f64 = (((p[31] * s.db[13][3]) * s.v[327]) + (eq20_e617 * s.db[327][3]));let eq20_e621: f64 = (eq20_e619 * (nv3 - nv8));let eq20_e621_d_n0: f64 = (eq20_e619_d_n0 * (nv3 - nv8));let eq20_e621_d_n1: f64 = (eq20_e619_d_n1 * (nv3 - nv8));let eq20_e621_d_n2: f64 = (eq20_e619_d_n2 * (nv3 - nv8));let eq20_e621_d_n3: f64 = ((eq20_e619_d_n3 * (nv3 - nv8)) + eq20_e619);let eq20_e621_d_n4: f64 = (eq20_e619_d_n4 * (nv3 - nv8));let eq20_e621_d_n5: f64 = (eq20_e619_d_n5 * (nv3 - nv8));let eq20_e621_d_n6: f64 = (eq20_e619_d_n6 * (nv3 - nv8));let eq20_e621_d_n7: f64 = (eq20_e619_d_n7 * (nv3 - nv8));let eq20_e621_d_n8: f64 = ((eq20_e619_d_n8 * (nv3 - nv8)) + (-eq20_e619));let eq20_e621_d_n9: f64 = (eq20_e619_d_n9 * (nv3 - nv8));let eq20_e621_d_n10: f64 = (eq20_e619_d_n10 * (nv3 - nv8));let eq20_e621_d_n11: f64 = (eq20_e619_d_n11 * (nv3 - nv8));let eq20_e621_d_n12: f64 = (eq20_e619_d_n12 * (nv3 - nv8));let eq20_e621_d_n13: f64 = (eq20_e619_d_n13 * (nv3 - nv8));let eq20_e621_d_b0: f64 = (eq20_e619_d_b0 * (nv3 - nv8));let eq20_e621_d_b1: f64 = (eq20_e619_d_b1 * (nv3 - nv8));let eq20_e621_d_b2: f64 = (eq20_e619_d_b2 * (nv3 - nv8));let eq20_e621_d_b3: f64 = (eq20_e619_d_b3 * (nv3 - nv8));
        (eq20_e621, eq20_e621_d_n0, eq20_e621_d_n1, eq20_e621_d_n2, eq20_e621_d_n3, eq20_e621_d_n4, eq20_e621_d_n5, eq20_e621_d_n6, eq20_e621_d_n7, eq20_e621_d_n8, eq20_e621_d_n9, eq20_e621_d_n10, eq20_e621_d_n11, eq20_e621_d_n12, eq20_e621_d_n13, eq20_e621_d_b0, eq20_e621_d_b1, eq20_e621_d_b2, eq20_e621_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e623;let eq20_node_derivatives: [f64; 14] = [eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13];let eq20_branch_derivatives: [f64; 4] = [eq20_e623_d_b0, eq20_e623_d_b1, eq20_e623_d_b2, eq20_e623_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq22_e638,) = {
    if (!s.b[1771]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e638;
        stamper.stamp_potential_const_local(
            3,
            eq22_value,
        );let eq23_e642: f64 = (s.v[1774] + s.v[1775]);let eq23_e642_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);let eq23_e642_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);let eq23_e642_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);let eq23_e642_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);let eq23_e642_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);let eq23_e642_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);let eq23_e642_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);let eq23_e642_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);let eq23_e642_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);let eq23_e642_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);let eq23_e642_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);let eq23_e642_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);let eq23_e642_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);let eq23_e642_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);let eq23_e642_d_b0: f64 = (s.db[1774][0] + s.db[1775][0]);let eq23_e642_d_b1: f64 = (s.db[1774][1] + s.db[1775][1]);let eq23_e642_d_b2: f64 = (s.db[1774][2] + s.db[1775][2]);let eq23_e642_d_b3: f64 = (s.db[1774][3] + s.db[1775][3]);let eq23_e643: f64 = (s.v[181] * eq23_e642);let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_n0));let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_n1));let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_n2));let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_n3));let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * eq23_e642_d_n4));let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * eq23_e642_d_n5));let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * eq23_e642_d_n6));let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * eq23_e642_d_n7));let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * eq23_e642_d_n8));let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * eq23_e642_d_n9));let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * eq23_e642_d_n10));let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * eq23_e642_d_n11));let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * eq23_e642_d_n12));let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * eq23_e642_d_n13));let eq23_e643_d_b0: f64 = ((s.db[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_b0));let eq23_e643_d_b1: f64 = ((s.db[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_b1));let eq23_e643_d_b2: f64 = ((s.db[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_b2));let eq23_e643_d_b3: f64 = ((s.db[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_b3));let eq23_e644: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq23_e643);let eq23_value: f64 = eq23_e644;let eq23_node_derivatives: [f64; 14] = [(eq23_e643_d_n0 * ddt_scale), (eq23_e643_d_n1 * ddt_scale), (eq23_e643_d_n2 * ddt_scale), (eq23_e643_d_n3 * ddt_scale), (eq23_e643_d_n4 * ddt_scale), (eq23_e643_d_n5 * ddt_scale), (eq23_e643_d_n6 * ddt_scale), (eq23_e643_d_n7 * ddt_scale), (eq23_e643_d_n8 * ddt_scale), (eq23_e643_d_n9 * ddt_scale), (eq23_e643_d_n10 * ddt_scale), (eq23_e643_d_n11 * ddt_scale), (eq23_e643_d_n12 * ddt_scale), (eq23_e643_d_n13 * ddt_scale)];let eq23_branch_derivatives: [f64; 4] = [(eq23_e643_d_b0 * ddt_scale), (eq23_e643_d_b1 * ddt_scale), (eq23_e643_d_b2 * ddt_scale), (eq23_e643_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(13),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
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
        let nv10 = ctx.node_voltage(nodes[10]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let eq24_e647: f64 = (s.v[1773] * (nv10 - nv13));let eq24_e647_d_n0: f64 = (s.dn[1773][0] * (nv10 - nv13));let eq24_e647_d_n1: f64 = (s.dn[1773][1] * (nv10 - nv13));let eq24_e647_d_n2: f64 = (s.dn[1773][2] * (nv10 - nv13));let eq24_e647_d_n3: f64 = (s.dn[1773][3] * (nv10 - nv13));let eq24_e647_d_n4: f64 = (s.dn[1773][4] * (nv10 - nv13));let eq24_e647_d_n5: f64 = (s.dn[1773][5] * (nv10 - nv13));let eq24_e647_d_n6: f64 = (s.dn[1773][6] * (nv10 - nv13));let eq24_e647_d_n7: f64 = (s.dn[1773][7] * (nv10 - nv13));let eq24_e647_d_n8: f64 = (s.dn[1773][8] * (nv10 - nv13));let eq24_e647_d_n9: f64 = (s.dn[1773][9] * (nv10 - nv13));let eq24_e647_d_n10: f64 = ((s.dn[1773][10] * (nv10 - nv13)) + s.v[1773]);let eq24_e647_d_n11: f64 = (s.dn[1773][11] * (nv10 - nv13));let eq24_e647_d_n12: f64 = (s.dn[1773][12] * (nv10 - nv13));let eq24_e647_d_n13: f64 = ((s.dn[1773][13] * (nv10 - nv13)) + (-s.v[1773]));let eq24_e647_d_b0: f64 = (s.db[1773][0] * (nv10 - nv13));let eq24_e647_d_b1: f64 = (s.db[1773][1] * (nv10 - nv13));let eq24_e647_d_b2: f64 = (s.db[1773][2] * (nv10 - nv13));let eq24_e647_d_b3: f64 = (s.db[1773][3] * (nv10 - nv13));let eq24_value: f64 = eq24_e647;let eq24_node_derivatives: [f64; 14] = [eq24_e647_d_n0, eq24_e647_d_n1, eq24_e647_d_n2, eq24_e647_d_n3, eq24_e647_d_n4, eq24_e647_d_n5, eq24_e647_d_n6, eq24_e647_d_n7, eq24_e647_d_n8, eq24_e647_d_n9, eq24_e647_d_n10, eq24_e647_d_n11, eq24_e647_d_n12, eq24_e647_d_n13];let eq24_branch_derivatives: [f64; 4] = [eq24_e647_d_b0, eq24_e647_d_b1, eq24_e647_d_b2, eq24_e647_d_b3];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(13),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );let eq25_e650: f64 = (1e-9 * (nv10 - nv13));let eq25_e651: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq25_e650);let eq25_value: f64 = eq25_e651;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(13),
            multiplicity * (eq25_value),
            10,
            multiplicity * ((1e-9 * ddt_scale)),
            13,
            multiplicity * (((-1e-9) * ddt_scale)),
        );let eq26_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[1776]);let eq26_value: f64 = eq26_e653;
        stamper.stamp_current_dense_local(
            Some(12),
            Some(13),
            multiplicity * (eq26_value),
            &s.dn[1776],
            &s.db[1776],
            (multiplicity) * (ddt_scale),
        );let eq27_e656: f64 = (s.v[1773] * (nv12 - nv13));let eq27_e656_d_n0: f64 = (s.dn[1773][0] * (nv12 - nv13));let eq27_e656_d_n1: f64 = (s.dn[1773][1] * (nv12 - nv13));let eq27_e656_d_n2: f64 = (s.dn[1773][2] * (nv12 - nv13));let eq27_e656_d_n3: f64 = (s.dn[1773][3] * (nv12 - nv13));let eq27_e656_d_n4: f64 = (s.dn[1773][4] * (nv12 - nv13));let eq27_e656_d_n5: f64 = (s.dn[1773][5] * (nv12 - nv13));let eq27_e656_d_n6: f64 = (s.dn[1773][6] * (nv12 - nv13));let eq27_e656_d_n7: f64 = (s.dn[1773][7] * (nv12 - nv13));let eq27_e656_d_n8: f64 = (s.dn[1773][8] * (nv12 - nv13));let eq27_e656_d_n9: f64 = (s.dn[1773][9] * (nv12 - nv13));let eq27_e656_d_n10: f64 = (s.dn[1773][10] * (nv12 - nv13));let eq27_e656_d_n11: f64 = (s.dn[1773][11] * (nv12 - nv13));let eq27_e656_d_n12: f64 = ((s.dn[1773][12] * (nv12 - nv13)) + s.v[1773]);let eq27_e656_d_n13: f64 = ((s.dn[1773][13] * (nv12 - nv13)) + (-s.v[1773]));let eq27_e656_d_b0: f64 = (s.db[1773][0] * (nv12 - nv13));let eq27_e656_d_b1: f64 = (s.db[1773][1] * (nv12 - nv13));let eq27_e656_d_b2: f64 = (s.db[1773][2] * (nv12 - nv13));let eq27_e656_d_b3: f64 = (s.db[1773][3] * (nv12 - nv13));let eq27_value: f64 = eq27_e656;let eq27_node_derivatives: [f64; 14] = [eq27_e656_d_n0, eq27_e656_d_n1, eq27_e656_d_n2, eq27_e656_d_n3, eq27_e656_d_n4, eq27_e656_d_n5, eq27_e656_d_n6, eq27_e656_d_n7, eq27_e656_d_n8, eq27_e656_d_n9, eq27_e656_d_n10, eq27_e656_d_n11, eq27_e656_d_n12, eq27_e656_d_n13];let eq27_branch_derivatives: [f64; 4] = [eq27_e656_d_b0, eq27_e656_d_b1, eq27_e656_d_b2, eq27_e656_d_b3];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(13),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );let eq28_e659: f64 = (1e-9 * (nv12 - nv13));let eq28_e660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq28_e659);let eq28_value: f64 = eq28_e660;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(13),
            multiplicity * (eq28_value),
            12,
            multiplicity * ((1e-9 * ddt_scale)),
            13,
            multiplicity * (((-1e-9) * ddt_scale)),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq29_e662: f64 = (s.v[182]).sqrt();let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq29_e662);let eq29_e662_d_n0: f64 = (s.dn[182][0] * __rspice_inv_cse_0);let eq29_e662_d_n1: f64 = (s.dn[182][1] * __rspice_inv_cse_0);let eq29_e662_d_n2: f64 = (s.dn[182][2] * __rspice_inv_cse_0);let eq29_e662_d_n3: f64 = (s.dn[182][3] * __rspice_inv_cse_0);let eq29_e662_d_n4: f64 = (s.dn[182][4] * __rspice_inv_cse_0);let eq29_e662_d_n5: f64 = (s.dn[182][5] * __rspice_inv_cse_0);let eq29_e662_d_n6: f64 = (s.dn[182][6] * __rspice_inv_cse_0);let eq29_e662_d_n7: f64 = (s.dn[182][7] * __rspice_inv_cse_0);let eq29_e662_d_n8: f64 = (s.dn[182][8] * __rspice_inv_cse_0);let eq29_e662_d_n9: f64 = (s.dn[182][9] * __rspice_inv_cse_0);let eq29_e662_d_n10: f64 = (s.dn[182][10] * __rspice_inv_cse_0);let eq29_e662_d_n11: f64 = (s.dn[182][11] * __rspice_inv_cse_0);let eq29_e662_d_n12: f64 = (s.dn[182][12] * __rspice_inv_cse_0);let eq29_e662_d_n13: f64 = (s.dn[182][13] * __rspice_inv_cse_0);let eq29_e662_d_b0: f64 = (s.db[182][0] * __rspice_inv_cse_0);let eq29_e662_d_b1: f64 = (s.db[182][1] * __rspice_inv_cse_0);let eq29_e662_d_b2: f64 = (s.db[182][2] * __rspice_inv_cse_0);let eq29_e662_d_b3: f64 = (s.db[182][3] * __rspice_inv_cse_0);let eq29_e665: f64 = (1.0 - s.v[181]);let eq29_e668: f64 = (s.v[1774] + s.v[1775]);let eq29_e668_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);let eq29_e668_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);let eq29_e668_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);let eq29_e668_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);let eq29_e668_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);let eq29_e668_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);let eq29_e668_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);let eq29_e668_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);let eq29_e668_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);let eq29_e668_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);let eq29_e668_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);let eq29_e668_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);let eq29_e668_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);let eq29_e668_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);let eq29_e668_d_b0: f64 = (s.db[1774][0] + s.db[1775][0]);let eq29_e668_d_b1: f64 = (s.db[1774][1] + s.db[1775][1]);let eq29_e668_d_b2: f64 = (s.db[1774][2] + s.db[1775][2]);let eq29_e668_d_b3: f64 = (s.db[1774][3] + s.db[1775][3]);let eq29_e669: f64 = (eq29_e665 * eq29_e668);let eq29_e669_d_n0: f64 = (((-s.dn[181][0]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n0));let eq29_e669_d_n1: f64 = (((-s.dn[181][1]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n1));let eq29_e669_d_n2: f64 = (((-s.dn[181][2]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n2));let eq29_e669_d_n3: f64 = (((-s.dn[181][3]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n3));let eq29_e669_d_n4: f64 = (((-s.dn[181][4]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n4));let eq29_e669_d_n5: f64 = (((-s.dn[181][5]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n5));let eq29_e669_d_n6: f64 = (((-s.dn[181][6]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n6));let eq29_e669_d_n7: f64 = (((-s.dn[181][7]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n7));let eq29_e669_d_n8: f64 = (((-s.dn[181][8]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n8));let eq29_e669_d_n9: f64 = (((-s.dn[181][9]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n9));let eq29_e669_d_n10: f64 = (((-s.dn[181][10]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n10));let eq29_e669_d_n11: f64 = (((-s.dn[181][11]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n11));let eq29_e669_d_n12: f64 = (((-s.dn[181][12]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n12));let eq29_e669_d_n13: f64 = (((-s.dn[181][13]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n13));let eq29_e669_d_b0: f64 = (((-s.db[181][0]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b0));let eq29_e669_d_b1: f64 = (((-s.db[181][1]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b1));let eq29_e669_d_b2: f64 = (((-s.db[181][2]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b2));let eq29_e669_d_b3: f64 = (((-s.db[181][3]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b3));
        let eq29_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq29_e669);let eq29_e671: f64 = (eq29_e662 * eq29_e670);let eq29_e671_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n0 * ddt_scale)));let eq29_e671_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n1 * ddt_scale)));let eq29_e671_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n2 * ddt_scale)));let eq29_e671_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n3 * ddt_scale)));let eq29_e671_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n4 * ddt_scale)));let eq29_e671_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n5 * ddt_scale)));let eq29_e671_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n6 * ddt_scale)));let eq29_e671_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n7 * ddt_scale)));let eq29_e671_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n8 * ddt_scale)));let eq29_e671_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n9 * ddt_scale)));let eq29_e671_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n10 * ddt_scale)));let eq29_e671_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n11 * ddt_scale)));let eq29_e671_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n12 * ddt_scale)));let eq29_e671_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n13 * ddt_scale)));let eq29_e671_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b0 * ddt_scale)));let eq29_e671_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b1 * ddt_scale)));let eq29_e671_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b2 * ddt_scale)));let eq29_e671_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b3 * ddt_scale)));let eq29_value: f64 = eq29_e671;let eq29_node_derivatives: [f64; 14] = [eq29_e671_d_n0, eq29_e671_d_n1, eq29_e671_d_n2, eq29_e671_d_n3, eq29_e671_d_n4, eq29_e671_d_n5, eq29_e671_d_n6, eq29_e671_d_n7, eq29_e671_d_n8, eq29_e671_d_n9, eq29_e671_d_n10, eq29_e671_d_n11, eq29_e671_d_n12, eq29_e671_d_n13];let eq29_branch_derivatives: [f64; 4] = [eq29_e671_d_b0, eq29_e671_d_b1, eq29_e671_d_b2, eq29_e671_d_b3];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
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
        let nv11 = ctx.node_voltage(nodes[11]);let nv13 = ctx.node_voltage(nodes[13]);let eq30_e674: f64 = (s.v[1773] * (nv11 - nv13));let eq30_e674_d_n0: f64 = (s.dn[1773][0] * (nv11 - nv13));let eq30_e674_d_n1: f64 = (s.dn[1773][1] * (nv11 - nv13));let eq30_e674_d_n2: f64 = (s.dn[1773][2] * (nv11 - nv13));let eq30_e674_d_n3: f64 = (s.dn[1773][3] * (nv11 - nv13));let eq30_e674_d_n4: f64 = (s.dn[1773][4] * (nv11 - nv13));let eq30_e674_d_n5: f64 = (s.dn[1773][5] * (nv11 - nv13));let eq30_e674_d_n6: f64 = (s.dn[1773][6] * (nv11 - nv13));let eq30_e674_d_n7: f64 = (s.dn[1773][7] * (nv11 - nv13));let eq30_e674_d_n8: f64 = (s.dn[1773][8] * (nv11 - nv13));let eq30_e674_d_n9: f64 = (s.dn[1773][9] * (nv11 - nv13));let eq30_e674_d_n10: f64 = (s.dn[1773][10] * (nv11 - nv13));let eq30_e674_d_n11: f64 = ((s.dn[1773][11] * (nv11 - nv13)) + s.v[1773]);let eq30_e674_d_n12: f64 = (s.dn[1773][12] * (nv11 - nv13));let eq30_e674_d_n13: f64 = ((s.dn[1773][13] * (nv11 - nv13)) + (-s.v[1773]));let eq30_e674_d_b0: f64 = (s.db[1773][0] * (nv11 - nv13));let eq30_e674_d_b1: f64 = (s.db[1773][1] * (nv11 - nv13));let eq30_e674_d_b2: f64 = (s.db[1773][2] * (nv11 - nv13));let eq30_e674_d_b3: f64 = (s.db[1773][3] * (nv11 - nv13));let eq30_value: f64 = eq30_e674;let eq30_node_derivatives: [f64; 14] = [eq30_e674_d_n0, eq30_e674_d_n1, eq30_e674_d_n2, eq30_e674_d_n3, eq30_e674_d_n4, eq30_e674_d_n5, eq30_e674_d_n6, eq30_e674_d_n7, eq30_e674_d_n8, eq30_e674_d_n9, eq30_e674_d_n10, eq30_e674_d_n11, eq30_e674_d_n12, eq30_e674_d_n13];let eq30_branch_derivatives: [f64; 4] = [eq30_e674_d_b0, eq30_e674_d_b1, eq30_e674_d_b2, eq30_e674_d_b3];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );let eq31_e678: f64 = (1e-9 * (nv11 - nv13));let eq31_e679: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e678);let eq31_e680: f64 = (s.v[182] * eq31_e679);let eq31_e680_d_n11: f64 = ((s.dn[182][11] * eq31_e679) + (s.v[182] * (1e-9 * ddt_scale)));let eq31_e680_d_n13: f64 = ((s.dn[182][13] * eq31_e679) + (s.v[182] * ((-1e-9) * ddt_scale)));let eq31_value: f64 = eq31_e680;let eq31_node_derivatives: [f64; 14] = [(s.dn[182][0] * eq31_e679), (s.dn[182][1] * eq31_e679), (s.dn[182][2] * eq31_e679), (s.dn[182][3] * eq31_e679), (s.dn[182][4] * eq31_e679), (s.dn[182][5] * eq31_e679), (s.dn[182][6] * eq31_e679), (s.dn[182][7] * eq31_e679), (s.dn[182][8] * eq31_e679), (s.dn[182][9] * eq31_e679), (s.dn[182][10] * eq31_e679), eq31_e680_d_n11, (s.dn[182][12] * eq31_e679), eq31_e680_d_n13];let eq31_branch_derivatives: [f64; 4] = [(s.db[182][0] * eq31_e679), (s.db[182][1] * eq31_e679), (s.db[182][2] * eq31_e679), (s.db[182][3] * eq31_e679)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
    }
}
