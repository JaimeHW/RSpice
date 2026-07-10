#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_add_scaled_inputs3_mixed_aii(1587, A::add_scaled_product(s.ad_value(1586), 1.0, s.ad_value(1540), s.ad_value(1541), 1.0), 1.0, 1462, 1.0, 1527, -1.0);}
        s.b[1756] = (s.v[1462] > 1e-6);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });s.b[1757] = (s.v[1587] > 1e-30);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1756]) && s.b[1757]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1588, 1471, A::div(s.ad_value(1467), s.ad_value(1462)), 1.0, 1474, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1589, 1535, A::div(s.ad_value(1531), s.ad_value(1527)), 1.0, 1538, -1.0);s.store_div_scaled_inputs2_indices(1590, 1588, 1.0, 1589, (-1.0), 1587, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1591, 1472, A::div(s.ad_value(1468), s.ad_value(1462)), 1.0, 1474, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1536, A::div(s.ad_value(1532), s.ad_value(1527)), 1.0, 1538, -1.0);s.store_div_scaled_inputs2_indices(1593, 1591, 1.0, 1592, (-1.0), 1587, 1.0);}
        if ((s.b[1604] && s.b[1756]) && (!s.b[1757])) {s.store_scalar(1590, 0.0);s.store_scalar(1593, 0.0);}
        if (s.b[1604] && (!s.b[1756])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1594, 1493, A::div(s.ad_value(1430), s.ad_value(1496)), (-2.0), 1499, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1595, 1494, A::div(s.ad_value(1431), s.ad_value(1497)), (-2.0), 1499, (-2.0));s.store_mul_sub_lhs(0, 1595, 1594, 1499);s.store_mul(2, 1594, 1430);s.store_mul(3, 1595, 1431);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1493), s.ad_value(1430), 2.0, s.ad_value(1494), s.ad_value(1431), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1596, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(1496)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1597, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(1497)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1590, 1496, 1499, -1.0, 1596, 1496, -1.0);s.store_mul_add_scaled_product_rhs_indices(1593, 1497, 1499, -1.0, 1597, 1497, -1.0);}
        if s.b[1604] {s.store_mul(1598, 1590, 1577);s.store_mul(1599, 1593, 1577);s.store_scaled_sub(1600, 1528, 1463, 0.5);s.store_scaled_sub(1601, 1529, 1464, 0.5);s.store_mul(1602, 1600, 1598);s.store_mul(1603, 1601, 1599);s.copy_ad(436, 1424);s.copy_ad(437, 1428);s.copy_ad(438, 1429);s.copy_ad(439, 1430);s.copy_ad(440, 1431);s.copy_ad(441, 1458);s.copy_ad(442, 1459);s.copy_ad(443, 1443);s.copy_ad(444, 1442);s.copy_ad(445, 1446);s.copy_ad(446, 1447);s.copy_ad(447, 1448);s.copy_ad(448, 1449);s.copy_ad(449, 1450);s.copy_ad(450, 1453);s.copy_ad(451, 1455);s.copy_ad(452, 1456);s.copy_ad(453, 1457);s.copy_ad(454, 1463);s.copy_ad(455, 1464);s.copy_ad(456, 1475);s.copy_ad(457, 1528);s.copy_ad(458, 1529);s.copy_ad(459, 1539);s.copy_ad(460, 1540);s.copy_ad(461, 1544);s.copy_ad(462, 1553);s.copy_ad(463, 1554);s.copy_ad(464, 1575);s.copy_ad(465, 1578);s.copy_ad(466, 1579);s.copy_ad(467, 1600);s.copy_ad(468, 1601);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1604] {s.copy_ad(469, 1602);s.copy_ad(470, 1603);}
        if (!s.b[1604]) {s.copy_ad(436, 379);s.copy_ad(437, 380);s.copy_ad(438, 381);s.copy_ad(439, 382);s.copy_ad(440, 383);s.copy_ad(441, 384);s.copy_ad(442, 385);s.copy_ad(443, 386);s.copy_ad(444, 387);s.copy_ad(445, 389);s.copy_ad(446, 390);s.copy_ad(447, 391);s.copy_ad(448, 392);s.copy_ad(449, 393);s.copy_ad(450, 394);s.copy_ad(451, 395);s.copy_ad(452, 397);s.copy_ad(453, 398);s.copy_ad(454, 400);s.copy_ad(455, 401);s.copy_ad(456, 402);s.copy_ad(457, 404);s.copy_ad(458, 405);s.copy_ad(459, 410);s.copy_ad(460, 411);s.copy_ad(461, 412);s.copy_ad(462, 415);s.copy_ad(463, 416);s.copy_ad(464, 424);s.copy_ad(465, 426);s.copy_ad(466, 427);s.copy_ad(467, 432);s.copy_ad(468, 433);s.copy_ad(469, 434);s.copy_ad(470, 435);}
        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(444), s.ad_value(442)), 1.0, A::scale_offset(s.ad_value(460), 0.25, 1.0), 1.0);s.store_add_scaled_inputs3_indices(1320, 454, 0.5, 457, 0.5, 0, 1.0);s.store_add_scaled_inputs3_indices(1321, 455, 0.5, 458, 0.5, 0, -1.0);s.b[1758] = (p.p13 > 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if s.b[1758] {s.store_add_scaled_inputs3_mixed_iai(1322, 1320, 1.0, A::div(s.ad_value(462), s.ad_value(465)), 1.0, 462, -1.0);s.store_add_scaled_inputs3_mixed_iai(1323, 1321, 1.0, A::div(s.ad_value(463), s.ad_value(466)), 1.0, 463, -1.0);}
        if (!s.b[1758]) {s.copy_ad(1322, 1320);s.copy_ad(1323, 1321);}
        s.store_scaled_mul(2, 467, 469, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 467, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(469), 1.0, A::scale(s.ad_value(469), 0.2)), 1.0);s.store_add_scaled_product_indices(1324, 3, 1.0, 1322, 461, 0.5);s.store_add_scaled_product_indices(1322, 2, 1.0, 1322, 461, 1.0);s.store_scaled_mul(2, 468, 470, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 468, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, A::scale(s.ad_value(470), 0.2)), 1.0);s.store_add_scaled_inputs(1325, 1323, 0.5, 3, 1.0);s.store_add(1323, 1323, 2);s.store_mul(0, 443, 283);s.store_mul(357, 0, 1322);s.store_mul(358, 0, 1323);s.store_mul_add_scaled_inputs_rhs_indices(359, 0, 1324, -1.0, 1325, -1.0);s.b[1759] = (s.v[119] > 0.0);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if s.b[1759] {s.store_offset(0, 250, (2.0 * 0.6931471805599));s.store_add(1326, 456, 0);s.store_add(1327, 459, 0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1328, 1326, 0.5, 250, 0.5, 1326, 250, 9.0, (-0.5));s.store_add_scaled_inputs4_mixed_iiia(1329, 1327, 0.5, 250, 0.5, 335, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1327), 1.0, s.ad_value(250), -1.0, s.ad_value(335), -1.0), 9.0), (-0.5));s.store_mul_sqrt_mixed_ia(1330, 290, A::mul_offset_rhs(s.ad_value(441), s.ad_value(440), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if s.b[1759] {s.store_mul_sqrt_mixed_ia(1331, 290, A::mul_offset_rhs(A::mul3(s.ad_value(441), s.ad_value(452), s.ad_value(440)), s.ad_value(439), 0.5));s.store_mul_square_lhs(1332, 1330, 287);s.store_mul_square_lhs(1333, 1331, 287);s.store_sub(2, 288, 1328);s.store_add_scaled_inputs3_indices(3, 288, 1.0, 335, 1.0, 1329, -1.0);s.store_scale(0, 1332, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1334, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1332)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1335, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1332)), 1.0)), (-1.0), 1.0);s.store_scale(0, 1333, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1336, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1333)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1337, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1333)), 1.0)), (-1.0), 1.0);s.store_mul(0, 289, 443);s.store_mul_product3_indices(2, 447, 0, 1330, 452, -1.0);s.store_mul_product3_indices(3, 448, 0, 1331, 453, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1334, 0.5, 1326, ((-1.0) * 0.5), 1334, 1326, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(375, 2, 0, 0, 1.0, A::sub(s.ad_value(1334), s.ad_value(1328)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1335, 0.5, 1327, ((-1.0) * 0.5), 1335, 1327, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(376, 2, 0, 0, 1.0, A::sub(s.ad_value(1335), s.ad_value(1329)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1336, 0.5, 1326, ((-1.0) * 0.5), 1336, 1326, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(377, 3, 0, 0, 1.0, A::sub(s.ad_value(1336), s.ad_value(1328)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1337, 0.5, 1327, ((-1.0) * 0.5), 1337, 1327, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(378, 3, 0, 0, 1.0, A::sub(s.ad_value(1337), s.ad_value(1329)), 1.0);}
        if (!s.b[1759]) {s.store_scalar(375, 0.0);s.store_scalar(376, 0.0);s.store_scalar(377, 0.0);s.store_scalar(378, 0.0);}
        s.store_mul(366, 164, 326);s.store_mul(367, 165, 328);s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(445), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), 0.2, 0.5);s.store_mul3_lhs(368, 159, 345, 0);s.store_mul3_lhs(369, 160, 346, 0);s.store_mul(370, 117, 334);s.store_mul(371, 166, 332);s.store_mul_scale_offset_mixed_ia(373, 327, A::add_scaled_products(s.ad_value(236), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(372, 329, A::add_scaled_products(s.ad_value(236), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1760] = (s.v[6] > 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_mul(374, 170, 215);}
        if (!s.b[1760]) {s.store_scalar(374, 0.0);}
        s.store_mul_add_scaled_inputs3_offset_rhs_indices(361, 13, 344, p.p31, 352, p.p31, 354, p.p31, 0.0);s.store_scaled_mul(362, 13, 348, p.p31);s.store_scaled_mul(363, 13, 349, p.p31);s.store_scaled_mul(364, 13, 350, p.p31);s.store_scaled_mul(365, 13, 351, p.p31);s.store_mul(1761, 13, 355);s.store_mul(1762, 13, 356);s.b[1763] = (s.v[330] < 0.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = (s.v[307] > 0.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });s.b[1765] = (s.v[314] > 0.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });s.b[1766] = (s.v[318] > 0.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });s.b[1767] = (s.v[322] > 0.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });s.store_scaled_mul(357, 13, 357, p.p32);s.store_scaled_mul(358, 13, 358, p.p32);s.store_scaled_mul(359, 13, 359, p.p32);s.store_add_scaled_inputs3_indices(360, 357, (-1.0), 358, (-1.0), 359, (-1.0));s.store_scaled_mul(375, 13, 375, p.p32);s.store_scaled_mul(376, 13, 376, p.p32);s.store_scaled_mul(377, 13, 377, p.p32);s.store_scaled_mul(378, 13, 378, p.p32);s.store_scaled_mul(366, 13, 366, p.p32);s.store_scaled_mul(367, 13, 367, p.p32);s.store_scaled_mul(368, 13, 368, p.p32);s.store_scaled_mul(369, 13, 369, p.p32);s.store_scaled_mul(370, 13, 370, p.p32);s.store_scaled_mul(373, 13, 373, p.p32);s.store_scaled_mul(372, 13, 372, p.p32);s.store_scaled_mul(371, 13, 371, p.p32);s.store_mul(374, 13, 374);s.b[1769] = (s.v[330] < 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if s.b[1769] {s.copy_ad(1768, 359);s.copy_ad(359, 360);s.copy_ad(360, 1768);s.store_neg(371, 371);s.copy_ad(1768, 376);s.copy_ad(376, 375);s.copy_ad(375, 1768);s.copy_ad(1768, 378);s.copy_ad(378, 377);s.copy_ad(377, 1768);}
        s.store_scaled_mul(1770, 386, 222, 1.0 / (1.602176565e-19));s.store_scaled_add(1771, 403, 428, (-0.5));s.store_add(1772, 411, 1771);s.store_div(0, 411, 1772);s.store_scaled_add_mixed_ia(1777, 0, A::sqrt_square_offset(s.ad_value(0), 1e-20), 0.5);s.store_scaled_mul(1778, 432, 431, (-0.1666666666667));s.store_square(1779, 1778);s.store_offset(1780, 425, (-1.0));s.store_max_with_scalar_ad(1781, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1780), 12.0, s.ad_value(1779))), 1e-20);s.store_div_from_scalar_square_ad(1782, 1.0, s.ad_value(1781));s.store_div_scaled_product3_by_product_mixed_aiiii(1783, A::mul3(s.ad_value(338), s.ad_value(386), s.ad_value(222)), 1772, 340, 1.0, 341, 342, 1.0);s.store_scale(1784, 1779, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1777, 1.0, 1784, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784), s.ad_value(1780), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);let t0: f64 = (s.v[1783] * s.v[1782]);let t1: f64 = (t0 * s.v[3]);s.store_scalar(1785, t1);s.b[1802] = (s.v[172] > 0.0);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        let (t3,) = {
    if s.b[1802] {
        let t2: f64 = (s.v[423] / s.v[418]);
        (t2,)
    } else {
        (s.v[1786],)
    }
};
        s.store_scalar(1786, t3);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (tc,) = {
    if s.b[1802] {
        let t4: f64 = (s.v[305] * s.v[344]);let t5: f64 = (t4 * s.v[407]);let t6: f64 = (t5 * s.v[219]);let t7: f64 = (s.v[1786] * s.v[1786]);let t8: f64 = (1.0 + t7);let t9: f64 = (t8 * s.v[1781]);let ta: f64 = (t9 * s.v[1781]);let tb: f64 = (t6 / ta);
        (tb,)
    } else {
        (s.v[1787],)
    }
};
        s.store_scalar(1787, tc);
        let (tf,) = {
    if s.b[1802] {
        let td: f64 = (s.v[1787] / s.v[304]);let te: f64 = (s.v[1785] + td);
        (te,)
    } else {
        (s.v[1785],)
    }
};
        s.store_scalar(1785, tf);s.store_div_scaled_product3_indices(1789, 452, 443, 116, 1.0, 465, 1.0);s.store_mul_scale_offset_indices(1790, 1789, 464, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1792, 1790, A::mul_scaled_lhs(s.ad_value(330), 0.25, s.ad_value(1778)), -1.0, 0.5);s.store_sub(1791, 1790, 1792);s.store_scalar(1795, 0.0);s.b[1803] = (p.p6 > 0.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if s.b[1803] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1777), 0.08333333333333333, s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)), (-1.0)), A::mul3_scaled_output(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784)), s.ad_value(1780), 1.6));s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1793, 1783, 1781, 1781, 1.0, 3, 1.0);}
        s.b[1804] = (s.v[1785] > 0.0);s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1804]) {s.store_mul_ad_product_rhs_mixed_ia(1795, 1782, 1778, A::add_scaled_sub_value_product(1.0, s.ad_value(1784), 1.0, A::add_scaled_inputs_product(s.ad_value(1777), 1.0, s.ad_value(1779), 19.2, s.ad_value(1777), s.ad_value(1784), (-1.0)), s.ad_value(1780), (-1.0)));}
        if (!s.b[1803]) {s.store_scalar(1793, 1.0);}
        s.copy_ad(1773, 1770);s.store_mul_scale_offset_indices(1774, 1770, 411, 1.0, 1.0);s.store_mul_sub_rhs(1775, 1770, 399, 409);s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5, A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1773), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1773), s.ad_value(1773)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1773), 2.0), 1.0), 1775, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(413), 1.0, s.ad_value(177), s.ad_value(414), 1.0), A::offset(s.ad_value(411), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(343), s.ad_value(344), 1.602176565e-19, s.ad_value(341), 1.0), 3, 1.0, 1773, 1.0);s.store_div_from_scalar_scaled_input(1813, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1814, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1815, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1816, 15, 1815, 1.0, 1814, (-1.0), 224, (-0.4), 0.0);s.store_add(1817, 1814, 1816);s.store_scaled_mul(1818, 1817, 1813, 0.5);s.store_sub_scaled_inputs(1819, 15, 0.05, 1816, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1820, 2, 234);s.store_div_scaled_value_offset_denominator(1821, s.ad_value(1813), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1823, 1820, 225, (2.0 * 1.602176565e-19), 0.0, 1821);s.store_add_offset_lhs_mixed_ai(1824, A::ln(A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(1823), 1.0)), (-0.6931471805599), 1818);s.store_mul_div_scaled_product_mixed_iiia(1825, 1821, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(1828, 35, 1821);s.store_scalar(1829, 0.0);s.store_scalar(1822, 0.0);s.b[1874] = (p.p9 > 0.0);s.store_scalar(1874, if s.b[1874] { 1.0 } else { 0.0 });
        if s.b[1874] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1822, 1.0, 1813, A::ln(A::div(s.ad_value(24), s.ad_value(247))));}
        s.b[1875] = (p.p13 > 0.0);s.store_scalar(1875, if s.b[1875] { 1.0 } else { 0.0 });s.b[1876] = (p.p14 == 1.0);s.store_scalar(1876, if s.b[1876] { 1.0 } else { 0.0 });
        if (s.b[1875] && s.b[1876]) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[1875] && (!s.b[1876])) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_mul(1832, 332, 1821);s.store_mul_scale_offset_mixed_ia(1833, 1821, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(1834, 1832, 1833, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1805, 398, 1.0, 397, A::offset(s.ad_value(398), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 397, 1.0, 398, A::offset(s.ad_value(397), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(397), A::offset(s.ad_value(1805), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(398), A::offset(s.ad_value(1806), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 395, 1805, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(395), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(394), -1.0, s.ad_value(25), 1.0), 394);s.store_add_scaled_product_mixed_iia(1812, 21, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(390), (-1.0), s.ad_value(391), 1.0), 1.0, s.ad_value(393), (-1.0), s.ad_value(390), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(179), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);s.store_add_scaled_inputs4_indices(1831, 180, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);s.b[1877] = (p.p2 > 0.0);s.store_scalar(1877, if s.b[1877] { 1.0 } else { 0.0 });
        if s.b[1877] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);}
        s.b[1878] = (s.v[0] < 0.0);s.store_scalar(1878, if s.b[1878] { 1.0 } else { 0.0 });
        if (s.b[1877] && s.b[1878]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if (s.b[1877] && (!s.b[1878])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if s.b[1877] {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);}
        if (!s.b[1877]) {s.copy_ad(1837, 1836);}
        s.store_mul_sub_rhs(0, 244, 1835, 1837);s.b[1879] = (p.p13 > 0.0);s.store_scalar(1879, if s.b[1879] { 1.0 } else { 0.0 });
        if s.b[1879] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1879] {s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
        if (!s.b[1879]) {s.copy_ad(1841, 242);s.copy_ad(1842, 243);s.copy_ad(1843, 244);}
        s.store_mul_sub_rhs(1844, 1843, 1835, 1837);s.b[1880] = (s.v[1844] > 0.0);s.store_scalar(1880, if s.b[1880] { 1.0 } else { 0.0 });s.b[1881] = ((-s.v[1844]) < 80.0);s.store_scalar(1881, if s.b[1881] { 1.0 } else { 0.0 });
        if (s.b[1880] && s.b[1881]) {s.store_ln_one_plus_exp_neg_input(0, 1844);}
        if (s.b[1880] && (!s.b[1881])) {s.store_neg(0, 1844);}
        if s.b[1880] {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1882] = (s.v[1844] < 80.0);s.store_scalar(1882, if s.b[1882] { 1.0 } else { 0.0 });
        if ((!s.b[1880]) && s.b[1882]) {s.store_ln_one_plus_exp(0, 1844);}
        if ((!s.b[1880]) && (!s.b[1882])) {s.copy_ad(0, 1844);}
        if (!s.b[1880]) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);s.b[1884] = (p.p11 > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_div_scaled_value_by_product_mixed_iia(1805, 453, 1.0, 452, A::offset(s.ad_value(453), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 452, 1.0, 453, A::offset(s.ad_value(452), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(452), A::offset(s.ad_value(1805), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(453), A::offset(s.ad_value(1806), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 451, 1805, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1884] {s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(451), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(450), -1.0, s.ad_value(25), 1.0), 450);s.store_add_scaled_product_mixed_iia(1812, 130, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(446), (-1.0), s.ad_value(447), 1.0), 1.0, s.ad_value(449), (-1.0), s.ad_value(446), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(181), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);s.store_add_scaled_inputs4_indices(1831, 182, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);}
        s.b[1885] = (p.p2 > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1885]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);}
        s.b[1886] = (s.v[0] < 0.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if ((s.b[1884] && s.b[1885]) && s.b[1886]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if ((s.b[1884] && s.b[1885]) && (!s.b[1886])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if (s.b[1884] && s.b[1885]) {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);}
        if (s.b[1884] && (!s.b[1885])) {s.copy_ad(1837, 1836);}
        if s.b[1884] {s.store_mul_sub_rhs(0, 244, 1835, 1837);}
        s.b[1887] = (p.p13 > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1887]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
    ) {
        if (s.b[1884] && s.b[1887]) {s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
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
        if s.b[1884] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(7, (273.15 + p.p15));s.store_scalar(0, ((ctx_temp + p.p36)).min(1000.0));s.b[525] = (p.p10 == 1.0);s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });
        if s.b[525] {s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));s.store_scaled_add_offset_sqrt_square_offset_ad(221, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (!s.b[525]) {s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));s.store_scalar(221, 600.0);}
        s.b[526] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0)));s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });
        if s.b[526] {s.store_scalar(6, p.p5);}
        if (!s.b[526]) {s.store_scalar(6, 0.0);}
        s.store_scalar(215, 0.0);s.copy_ad(213, 8);s.store_square(214, 213);s.store_offset(216, 213, (-s.v[7]));s.store_scale(217, 213, 1.0 / (s.v[7]));s.store_div_from_scalar(218, s.v[7], 213);s.store_scale(219, 213, 8.617332384961e-5);s.store_div_from_scalar(220, 1.0, 219);s.b[607] = (p.p0 == 0.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
        if s.b[607] {s.store_scalar(10, p.p23);s.store_scalar(9, p.p22);s.store_scalar(12, p.p25);s.store_scalar(11, p.p24);s.store_scalar(13, p.p30);s.store_scalar(529, p.p41);s.store_scalar(14, p.p42);s.store_scalar(15, p.p43);s.store_scalar(530, p.p44);s.store_scalar(531, 1.0);}
        s.b[608] = (p.p45 < 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });
        if (s.b[607] && s.b[608]) {s.store_scalar(531, (-1.0));}
        if s.b[607] {s.store_scalar(532, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[609] = (p.p46 < 0.0);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });
        if (s.b[607] && s.b[609]) {s.store_scalar(16, (-1.0));}
        if s.b[607] {s.store_scalar(533, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p47);s.store_scalar(18, p.p48);s.store_scalar(19, (p.p49 * 1000000.0));s.store_scalar(20, (p.p50 * 1000000.0));s.store_scalar(179, p.p51);s.store_scalar(180, p.p52);s.store_scalar(23, p.p53);s.store_scalar(24, (p.p54 * 1000000.0));s.store_scalar(25, p.p55);s.store_scalar(26, p.p56);s.store_scalar(27, p.p57);s.store_primal_div_scaled_product_indices(28, 27, 530, p.p58, 529, 1.0);s.store_scalar(29, (p.p59 * 1000000.0));s.store_scalar(30, p.p60);s.store_scalar(534, p.p61);s.store_scalar(183, p.p62);s.store_div_scaled_product_indices(184, 183, 530, p.p63, 529, 1.0);s.store_scalar(34, p.p64);s.store_scalar(35, p.p65);s.store_scalar(36, p.p66);s.store_scalar(37, p.p67);s.store_scalar(187, p.p68);s.store_scale(188, 187, p.p69);s.store_scalar(40, p.p70);s.store_scalar(191, p.p71);s.store_scalar(41, p.p72);s.store_scalar(42, p.p73);s.store_scalar(43, p.p74);s.store_scalar(192, p.p75);s.store_scalar(45, p.p76);s.store_scalar(535, p.p77);s.store_scalar(536, p.p78);s.store_scalar(189, p.p79);s.store_scalar(48, p.p80);s.store_scalar(190, p.p81);s.store_scalar(49, p.p82);s.store_scalar(193, p.p83);s.store_scalar(51, p.p84);s.store_scalar(52, p.p85);s.store_scalar(537, p.p86);s.store_scalar(194, p.p87);s.store_scalar(54, p.p88);s.store_scalar(55, p.p89);s.store_scalar(56, p.p90);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[607] {s.store_scalar(57, p.p91);s.store_scalar(58, p.p92);s.store_scalar(195, p.p93);s.store_scalar(60, p.p94);s.store_scalar(61, p.p95);s.store_scalar(62, p.p96);s.store_scalar(538, p.p97);s.store_scalar(63, p.p98);s.store_scalar(64, p.p99);s.store_scalar(65, p.p100);s.store_scalar(66, p.p101);s.store_scalar(67, p.p102);s.store_scalar(75, p.p103);s.store_scalar(197, p.p104);s.store_scalar(198, p.p105);s.store_scalar(199, p.p106);s.store_scalar(200, p.p107);s.store_scalar(201, p.p108);s.store_scalar(76, p.p109);s.store_scalar(77, p.p123);s.store_scalar(78, p.p110);s.store_scalar(79, p.p111);s.store_scalar(80, p.p112);s.store_scalar(81, p.p122);s.store_scalar(82, p.p113);s.store_scalar(83, p.p114);s.store_scalar(84, p.p115);s.store_scalar(85, p.p116);s.store_scalar(86, p.p117);s.store_scalar(87, p.p118);s.store_scalar(88, p.p119);s.store_scalar(89, p.p124);s.store_scalar(90, p.p125);s.store_scalar(204, p.p126);s.store_scalar(205, p.p127);s.store_scalar(93, p.p128);s.store_scalar(94, p.p129);s.store_scalar(95, p.p130);s.store_scalar(96, p.p131);s.store_scalar(97, p.p132);s.store_scalar(98, p.p133);s.store_scalar(206, p.p148);s.store_scalar(114, p.p149);s.store_scalar(115, p.p150);s.store_scalar(99, p.p134);s.store_scalar(207, p.p135);s.store_scalar(208, p.p136);s.store_scalar(102, p.p137);s.store_scalar(103, p.p138);s.store_scalar(104, p.p139);s.store_scalar(105, p.p140);s.store_div_scaled_product_indices(106, 105, 530, p.p141, 529, 1.0);s.store_scalar(107, p.p142);s.store_div_scaled_product_indices(108, 107, 530, p.p143, 529, 1.0);s.store_scalar(109, p.p144);s.store_scalar(209, p.p145);s.store_scalar(111, p.p146);s.store_scalar(116, p.p151);s.store_scalar(117, p.p152);s.store_scalar(118, (p.p153 * 1000000.0));s.store_scalar(119, p.p154);s.store_scalar(120, p.p155);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
        s.b[610] = (p.p11 > 0.0);s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
        if (s.b[607] && s.b[610]) {s.store_scalar(181, p.p51);}
        s.b[611] = param_given[156];s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[611]) {s.store_scalar(181, p.p156);}
        if (s.b[607] && s.b[610]) {s.store_scalar(182, p.p52);}
        s.b[612] = param_given[157];s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[612]) {s.store_scalar(182, p.p157);}
        if (s.b[607] && s.b[610]) {s.store_scalar(135, p.p57);}
        s.b[613] = param_given[158];s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[613]) {s.store_scalar(135, p.p158);}
        if (s.b[607] && s.b[610]) {s.store_primal_div_scaled_product_indices(136, 135, 530, p.p58, 529, 1.0);s.store_scalar(185, p.p62);}
        s.b[614] = param_given[159];s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[614]) {s.store_scalar(185, p.p159);}
        if (s.b[607] && s.b[610]) {s.store_div_scaled_product_indices(186, 185, 530, p.p63, 529, 1.0);s.store_scalar(196, p.p93);}
        s.b[615] = param_given[160];s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[615]) {s.store_scalar(196, p.p160);}
        if (s.b[607] && s.b[610]) {s.store_scalar(539, p.p97);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[616] = param_given[161];s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[616]) {s.store_scalar(539, p.p161);}
        if (s.b[607] && s.b[610]) {s.store_scalar(158, p.p98);}
        s.b[617] = param_given[162];s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[617]) {s.store_scalar(158, p.p162);}
        if s.b[607] {s.store_scalar(159, p.p163);s.store_scalar(160, p.p164);s.store_scalar(161, p.p165);s.store_scalar(162, p.p166);s.store_scalar(163, p.p167);s.store_scalar(164, p.p168);s.store_scalar(165, p.p169);s.store_scalar(166, p.p170);s.store_scalar(167, p.p171);s.store_scalar(210, p.p172);s.store_scalar(169, p.p173);s.store_scalar(170, p.p174);s.store_scalar(173, p.p177);s.store_scalar(174, p.p178);s.store_scalar(175, p.p179);s.store_scalar(176, p.p180);s.store_scalar(177, p.p181);}
        if (!s.b[607]) {s.store_scalar(584, (1.0 / p.p29));s.store_primal_max_with_scalar_ad(528, A::scale(s.ad_value(584), p.p21), 1e-9);s.store_primal_scale(10, 584, p.p23);s.store_primal_scale(9, 584, p.p22);s.store_primal_scale(12, 584, p.p25);s.store_primal_scale(11, 584, p.p24);s.store_scalar(13, (p.p30 * p.p29));s.store_scalar(565, 1e-6);s.store_scalar(566, 1e-6);s.store_primal_scale(567, 565, 1.0 / (p.p20));s.store_primal_div(568, 566, 528);s.store_primal_scaled_mul_scale_offset_inputs(569, 567, p.p188, 1.0, 568, p.p189, 1.0, p.p187);s.store_primal_scaled_mul_scale_offset_inputs(570, 568, p.p193, 1.0, 567, p.p192, 1.0, p.p191);s.store_primal_max_with_scalar_ad(571, A::offset(s.ad_value(569), ((p.p20) + ((-(2.0 * p.p190))))), 1e-9);s.store_primal_max_with_scalar_ad(572, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (-(2.0 * p.p194))), 1e-9);s.store_primal_max_with_scalar_ad(573, A::offset(s.ad_value(569), ((((p.p20) + ((-(2.0 * p.p190))))) + (p.p195))), 1e-9);s.store_primal_max_with_scalar_ad(574, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (((-(2.0 * p.p194))) + (p.p196))), 1e-9);s.store_primal_div(575, 565, 571);s.store_primal_div(576, 566, 572);s.store_primal_mul(577, 575, 576);s.store_max_with_scalar_ad(0, A::offset(s.ad_value(569), p.p20), 1e-9);s.store_div(578, 0, 565);s.store_max_with_scalar_ad(0, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);s.store_div(579, 0, 566);s.store_scalar(529, p.p197);s.store_scalar(14, p.p198);s.store_scalar(15, p.p199);s.store_scalar(530, p.p200);s.store_scalar(531, 1.0);}
        s.b[618] = (p.p201 < 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[618]) {s.store_scalar(531, (-1.0));}
        if (!s.b[607]) {s.store_scalar(532, ((((p.p201) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[619] = (p.p202 < 0.0);s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[619]) {s.store_scalar(16, (-1.0));}
        if (!s.b[607]) {s.store_scalar(533, (((((p.p202) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p203);s.store_scalar(18, p.p204);s.store_scalar(19, (p.p205 * 1000000.0));s.store_scalar(20, (p.p206 * 1000000.0));s.store_div_scaled_inputs(0, A::powf(s.ad_value(575), p.p209), p.p208, A::scale_offset(A::powf(s.ad_value(575), p.p211), p.p210, 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(179, 0, 1.0, 576, p.p212, 577, p.p213, p.p207);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_offset_mul_ad(180, A::div_scaled_inputs(s.ad_value(530), p.p215, s.ad_value(529), 1.0), s.ad_value(0), p.p214);s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(575), p.p217, 1.0), A::scale_offset(s.ad_value(576), p.p218, 1.0), A::scale_offset(s.ad_value(577), p.p219, 1.0), p.p216);s.store_offset_scaled(603, 575, ((p.p221) * ((p.p220 * 1000000.0))), (p.p220 * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(603), 1e25), 1e28);s.store_scalar(25, p.p222);s.store_scalar(26, p.p223);s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(580, A::sqrt(A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11))), 571);s.store_primal_mul_powf_scale_offset_lhs(540, 580, 576, p.p225, (p.p226) * ((p.p224 * 2.0)), (1.0) * ((p.p224 * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(540), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 530, p.p227, 529, 1.0);s.store_scalar(29, (p.p228 * 1000000.0));s.store_scalar(30, p.p229);s.store_primal_scale(545, 576, p.p230);s.store_primal_min_with_scalar_ad(534, A::max_with_scalar(s.ad_value(545), (-1.0)), 1.0);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p.p232, p.p233, 1.0);s.store_scale(542, 0, p.p231);s.store_max_with_scalar(183, 542, 0.0);s.store_div_scaled_product_indices(184, 183, 530, p.p234, 529, 1.0);s.store_scale(34, 0, p.p235);s.store_scalar(35, p.p236);s.store_primal_div_scaled_inputs_mixed_ia(36, 575, p.p237, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p238, 1.0), 0.001), 1.0);s.store_scalar(37, p.p239);s.store_div_scaled_inputs_mixed_ia(2, 571, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p244, 1.0), 0.001), p.p243);}
        s.b[620] = (s.v[2] > (-80.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[620]) {s.store_exp(3, 2);}
        if ((!s.b[607]) && (!s.b[620])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_scale(4, 571, (-1.0 / (p.p246)));}
        s.b[621] = (s.v[4] > (-80.0));s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[621]) {s.store_exp(5, 4);}
        if ((!s.b[607]) && (!s.b[621])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_max_with_scalar_ad(581, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(576), p.p242, 1.0), s.ad_value(3), (-1.0), p.p241, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p245, ((-1.0) * p.p245), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(582, A::add_scaled_product(A::scale_offset(s.ad_value(576), p.p247, 1.0), 1.0, s.ad_value(576), A::ln(A::scale_offset(s.ad_value(572), 1.0 / (p.p249), 1.0)), p.p248), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(583, p.p240, 581, 582);s.store_div_scaled_product_indices(544, 583, 572, 1.0, 571, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p.p250);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(575), p.p252, 1.0), A::scale_offset(s.ad_value(576), p.p253, 1.0), A::scale_offset(s.ad_value(577), p.p254, 1.0), p.p251);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_mul3_ad(546, A::scale_offset(A::powf(s.ad_value(575), p.p257), p.p256, p.p255), A::scale_offset(s.ad_value(576), p.p258, 1.0), A::scale_offset(s.ad_value(577), p.p259, 1.0));s.store_primal_max_with_scalar(191, 546, 0.0);s.store_scalar(41, p.p260);s.store_scalar(42, p.p261);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(575), p.p263, 1.0), A::scale_offset(s.ad_value(576), p.p264, 1.0), A::scale_offset(s.ad_value(577), p.p265, 1.0), p.p262);s.store_scalar(192, p.p266);s.store_scalar(45, p.p267);s.store_scalar(535, p.p268);s.store_scalar(536, p.p269);s.store_scalar(189, p.p270);s.store_scalar(48, p.p271);s.store_scalar(190, p.p272);s.store_scalar(49, p.p273);s.store_primal_mul3_ad(193, A::scale_offset(A::powf(s.ad_value(575), p.p276), p.p275, p.p274), A::scale_offset(s.ad_value(576), p.p277, 1.0), A::scale_offset(s.ad_value(577), p.p278, 1.0));s.store_scalar(51, p.p279);s.store_scalar(52, p.p280);s.store_scalar(537, p.p281);s.store_primal_mul_scale_offset_rhs(547, 576, 576, ((p.p283) * (p.p282)), p.p282);s.store_primal_max_with_scalar(194, 547, 0.0);s.store_scalar(54, p.p284);s.store_scalar(55, p.p285);s.store_scalar(56, p.p286);s.store_scalar(57, p.p287);s.store_scalar(58, p.p288);s.store_mul_scale_offset_mixed_ai(548, A::mul3(s.ad_value(583), A::scale_offset(A::powf(s.ad_value(575), p.p291), p.p290, p.p289), A::scale_offset(s.ad_value(576), p.p292, 1.0)), 577, p.p293, 1.0);s.store_max_with_scalar(195, 548, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(575), p.p295, 1.0), A::scale_offset(s.ad_value(576), p.p296, 1.0), A::scale_offset(s.ad_value(577), p.p297, 1.0), p.p294);s.store_scalar(61, p.p298);s.store_scalar(62, p.p299);s.store_primal_div_from_scalar_offset_ad(550, p.p300, A::div_scaled_inputs(A::powf(s.ad_value(575), p.p302), p.p301, A::scale_offset(A::powf(s.ad_value(575), p.p304), p.p303, 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(550), 1.0), 16.0);s.store_primal_div_scaled_product(553, A::powf(s.ad_value(575), p.p306), A::scale_offset(s.ad_value(576), p.p309, 1.0), p.p305, A::scale_offset(A::powf(s.ad_value(575), p.p308), p.p307, 1.0), 1.0);s.store_primal_max_with_scalar(63, 553, 0.0);s.store_primal_div_scaled_product(554, A::powf(s.ad_value(575), p.p311), A::scale_offset(s.ad_value(576), p.p314, 1.0), p.p310, A::scale_offset(A::powf(s.ad_value(575), p.p313), p.p312, 1.0), 1.0);s.store_primal_max_with_scalar(64, 554, 0.0);s.store_scalar(65, p.p315);s.store_scalar(66, p.p316);s.store_scalar(67, p.p317);s.store_scalar(75, p.p318);s.store_primal_div_from_scalar(197, p.p319, 577);s.store_primal_div_from_scalar(198, p.p320, 576);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_div_from_scalar(199, p.p321, 576);s.store_primal_div_from_scalar(200, p.p322, 576);s.store_primal_div_from_scalar(201, p.p323, 576);s.store_scalar(76, p.p324);s.store_scalar(77, p.p338);s.store_scalar(78, p.p325);s.store_scalar(79, p.p326);s.store_scalar(80, p.p327);s.store_scalar(81, p.p337);s.store_scalar(82, p.p328);s.store_scalar(83, p.p329);s.store_scalar(84, p.p330);s.store_primal_scale(85, 575, p.p331);s.store_scalar(86, p.p332);s.store_scalar(87, p.p333);s.store_scalar(88, p.p334);s.store_primal_offset_div_from_scalar_ad(555, p.p341, s.ad_value(576), p.p339);s.store_max_with_scalar(89, 555, 0.0);s.store_primal_offset_div_from_scalar_ad(556, p.p342, s.ad_value(576), p.p340);s.store_max_with_scalar(90, 556, 0.0);s.store_scalar(204, p.p343);s.store_scalar(205, p.p344);s.store_scalar(93, p.p345);s.store_scalar(94, p.p346);s.store_scalar(95, p.p347);s.store_scalar(96, p.p348);s.store_primal_offset_scaled(97, 575, p.p351, p.p349);s.store_primal_offset_scaled(98, 575, p.p352, p.p350);s.store_scalar(206, p.p387);s.store_scalar(114, p.p388);s.store_primal_scaled_mul_scale_offset_inputs(558, 575, p.p390, 1.0, 576, p.p391, 1.0, p.p389);s.store_primal_max_with_scalar(115, 558, 0.0);s.store_primal_offset_scaled(585, 572, p.p354, (2.0 * p.p353));s.store_scalar(99, p.p355);s.store_scale_ad(0, A::powf(s.ad_value(575), p.p358), p.p357);s.store_add_scaled_inputs3_offset_indices(207, 0, 1.0, 576, p.p359, 577, p.p360, p.p356);s.store_scalar(208, p.p361);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(575), p.p363, 1.0), A::scale_offset(s.ad_value(576), p.p364, 1.0), A::scale_offset(s.ad_value(577), p.p365, 1.0), p.p362);s.store_scalar(103, p.p366);s.store_scalar(104, p.p367);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p.p369, (p.p370) * ((p.p368 * 2.0)), (1.0) * ((p.p368 * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 530, p.p371, 529, 1.0);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p.p373, p.p374, 1.0);s.store_scale(0, 0, p.p372);s.store_max_with_scalar(107, 0, 0.0);s.store_div_scaled_product_indices(108, 107, 530, p.p375, 529, 1.0);s.store_scalar(109, p.p376);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p377 * p.p378), s.ad_value(571)), 1.0, A::exp_scaled_input(s.ad_value(571), (-1.0 / (p.p378)))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(209, A::scale_offset(s.ad_value(576), p.p379, 1.0), 585, p.p240, A::mul(s.ad_value(0), s.ad_value(571)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(575), p.p381, p.p380), 1.0, 576, p.p382, 575, 576, p.p383);s.store_primal_mul(116, 574, 573);s.store_offset_scaled(559, 578, p.p393, p.p392);s.store_max_with_scalar(117, 559, 0.0);s.store_scalar(118, (p.p394 * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 574, p.p395, 566, 1.0);s.store_scalar(120, p.p396);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(543, 542);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(549, 548);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
        s.b[622] = (p.p11 > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(121, p.p207);}
    }
}
