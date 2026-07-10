#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_add_scaled_inputs3_mixed_aii(1591, A::add_scaled_product(s.ad_value(1590), 1.0, s.ad_value(1544), s.ad_value(1545), 1.0), 1.0, 1466, 1.0, 1531, -1.0);}
        s.b[1760] = (s.v[1466] > 1e-6);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });s.b[1761] = (s.v[1591] > 1e-30);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1760]) && s.b[1761]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1475, A::div(s.ad_value(1471), s.ad_value(1466)), 1.0, 1478, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1593, 1539, A::div(s.ad_value(1535), s.ad_value(1531)), 1.0, 1542, -1.0);s.store_div_scaled_inputs2_indices(1594, 1592, 1.0, 1593, (-1.0), 1591, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1595, 1476, A::div(s.ad_value(1472), s.ad_value(1466)), 1.0, 1478, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1596, 1540, A::div(s.ad_value(1536), s.ad_value(1531)), 1.0, 1542, -1.0);s.store_div_scaled_inputs2_indices(1597, 1595, 1.0, 1596, (-1.0), 1591, 1.0);}
        if ((s.b[1608] && s.b[1760]) && (!s.b[1761])) {s.store_scalar(1594, 0.0);s.store_scalar(1597, 0.0);}
        if (s.b[1608] && (!s.b[1760])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1598, 1497, A::div(s.ad_value(1434), s.ad_value(1500)), (-2.0), 1503, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1599, 1498, A::div(s.ad_value(1435), s.ad_value(1501)), (-2.0), 1503, (-2.0));s.store_mul_sub_lhs(0, 1599, 1598, 1503);s.store_mul(2, 1598, 1434);s.store_mul(3, 1599, 1435);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1497), s.ad_value(1434), 2.0, s.ad_value(1498), s.ad_value(1435), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1600, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(1500)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1601, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(1501)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1594, 1500, 1503, -1.0, 1600, 1500, -1.0);s.store_mul_add_scaled_product_rhs_indices(1597, 1501, 1503, -1.0, 1601, 1501, -1.0);}
        if s.b[1608] {s.store_mul(1602, 1594, 1581);s.store_mul(1603, 1597, 1581);s.store_scaled_sub(1604, 1532, 1467, 0.5);s.store_scaled_sub(1605, 1533, 1468, 0.5);s.store_mul(1606, 1604, 1602);s.store_mul(1607, 1605, 1603);s.copy_ad(440, 1428);s.copy_ad(441, 1432);s.copy_ad(442, 1433);s.copy_ad(443, 1434);s.copy_ad(444, 1435);s.copy_ad(445, 1462);s.copy_ad(446, 1463);s.copy_ad(447, 1447);s.copy_ad(448, 1446);s.copy_ad(449, 1450);s.copy_ad(450, 1451);s.copy_ad(451, 1452);s.copy_ad(452, 1453);s.copy_ad(453, 1454);s.copy_ad(454, 1457);s.copy_ad(455, 1459);s.copy_ad(456, 1460);s.copy_ad(457, 1461);s.copy_ad(458, 1467);s.copy_ad(459, 1468);s.copy_ad(460, 1479);s.copy_ad(461, 1532);s.copy_ad(462, 1533);s.copy_ad(463, 1543);s.copy_ad(464, 1544);s.copy_ad(465, 1548);s.copy_ad(466, 1557);s.copy_ad(467, 1558);s.copy_ad(468, 1579);s.copy_ad(469, 1582);s.copy_ad(470, 1583);s.copy_ad(471, 1604);s.copy_ad(472, 1605);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1608] {s.copy_ad(473, 1606);s.copy_ad(474, 1607);}
        if (!s.b[1608]) {s.copy_ad(440, 383);s.copy_ad(441, 384);s.copy_ad(442, 385);s.copy_ad(443, 386);s.copy_ad(444, 387);s.copy_ad(445, 388);s.copy_ad(446, 389);s.copy_ad(447, 390);s.copy_ad(448, 391);s.copy_ad(449, 393);s.copy_ad(450, 394);s.copy_ad(451, 395);s.copy_ad(452, 396);s.copy_ad(453, 397);s.copy_ad(454, 398);s.copy_ad(455, 399);s.copy_ad(456, 401);s.copy_ad(457, 402);s.copy_ad(458, 404);s.copy_ad(459, 405);s.copy_ad(460, 406);s.copy_ad(461, 408);s.copy_ad(462, 409);s.copy_ad(463, 414);s.copy_ad(464, 415);s.copy_ad(465, 416);s.copy_ad(466, 419);s.copy_ad(467, 420);s.copy_ad(468, 428);s.copy_ad(469, 430);s.copy_ad(470, 431);s.copy_ad(471, 436);s.copy_ad(472, 437);s.copy_ad(473, 438);s.copy_ad(474, 439);}
        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(448), s.ad_value(446)), 1.0, A::scale_offset(s.ad_value(464), 0.25, 1.0), 1.0);s.store_add_scaled_inputs3_indices(1324, 458, 0.5, 461, 0.5, 0, 1.0);s.store_add_scaled_inputs3_indices(1325, 459, 0.5, 462, 0.5, 0, -1.0);s.b[1762] = (p.p13 > 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if s.b[1762] {s.store_add_scaled_inputs3_mixed_iai(1326, 1324, 1.0, A::div(s.ad_value(466), s.ad_value(469)), 1.0, 466, -1.0);s.store_add_scaled_inputs3_mixed_iai(1327, 1325, 1.0, A::div(s.ad_value(467), s.ad_value(470)), 1.0, 467, -1.0);}
        if (!s.b[1762]) {s.copy_ad(1326, 1324);s.copy_ad(1327, 1325);}
        s.store_scaled_mul(2, 471, 473, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 471, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(473), 1.0, A::scale(s.ad_value(473), 0.2)), 1.0);s.store_add_scaled_product_indices(1328, 3, 1.0, 1326, 465, 0.5);s.store_add_scaled_product_indices(1326, 2, 1.0, 1326, 465, 1.0);s.store_scaled_mul(2, 472, 474, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 472, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(474), 1.0, A::scale(s.ad_value(474), 0.2)), 1.0);s.store_add_scaled_inputs(1329, 1327, 0.5, 3, 1.0);s.store_add(1327, 1327, 2);s.store_mul(0, 447, 287);s.store_mul(361, 0, 1326);s.store_mul(362, 0, 1327);s.store_mul_add_scaled_inputs_rhs_indices(363, 0, 1328, -1.0, 1329, -1.0);s.b[1763] = (s.v[119] > 0.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if s.b[1763] {s.store_offset(0, 254, (2.0 * 0.6931471805599));s.store_add(1330, 460, 0);s.store_add(1331, 463, 0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1332, 1330, 0.5, 254, 0.5, 1330, 254, 9.0, (-0.5));s.store_add_scaled_inputs4_mixed_iiia(1333, 1331, 0.5, 254, 0.5, 339, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0), 9.0), (-0.5));s.store_mul_sqrt_mixed_ia(1334, 294, A::mul_offset_rhs(s.ad_value(445), s.ad_value(444), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if s.b[1763] {s.store_mul_sqrt_mixed_ia(1335, 294, A::mul_offset_rhs(A::mul3(s.ad_value(445), s.ad_value(456), s.ad_value(444)), s.ad_value(443), 0.5));s.store_mul_square_lhs(1336, 1334, 291);s.store_mul_square_lhs(1337, 1335, 291);s.store_sub(2, 292, 1332);s.store_add_scaled_inputs3_indices(3, 292, 1.0, 339, 1.0, 1333, -1.0);s.store_scale(0, 1336, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1338, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1339, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_scale(0, 1337, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1340, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1341, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_mul(0, 293, 447);s.store_mul_product3_indices(2, 451, 0, 1334, 456, -1.0);s.store_mul_product3_indices(3, 452, 0, 1335, 457, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1338, 0.5, 1330, ((-1.0) * 0.5), 1338, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(379, 2, 0, 0, 1.0, A::sub(s.ad_value(1338), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1339, 0.5, 1331, ((-1.0) * 0.5), 1339, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(380, 2, 0, 0, 1.0, A::sub(s.ad_value(1339), s.ad_value(1333)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1340, 0.5, 1330, ((-1.0) * 0.5), 1340, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(381, 3, 0, 0, 1.0, A::sub(s.ad_value(1340), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1341, 0.5, 1331, ((-1.0) * 0.5), 1341, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(382, 3, 0, 0, 1.0, A::sub(s.ad_value(1341), s.ad_value(1333)), 1.0);}
        if (!s.b[1763]) {s.store_scalar(379, 0.0);s.store_scalar(380, 0.0);s.store_scalar(381, 0.0);s.store_scalar(382, 0.0);}
        s.store_mul(370, 164, 330);s.store_mul(371, 165, 332);s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), 0.2, 0.5);s.store_mul3_lhs(372, 159, 349, 0);s.store_mul3_lhs(373, 160, 350, 0);s.store_mul(374, 117, 338);s.store_mul(375, 166, 336);s.store_mul_scale_offset_mixed_ia(377, 331, A::add_scaled_products(s.ad_value(240), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(376, 333, A::add_scaled_products(s.ad_value(240), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1764] = (s.v[6] > 0.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if s.b[1764] {s.store_mul(378, 170, 219);}
        if (!s.b[1764]) {s.store_scalar(378, 0.0);}
        s.store_mul_add_scaled_inputs3_offset_rhs_indices(365, 13, 348, p.p31, 356, p.p31, 358, p.p31, 0.0);s.store_scaled_mul(366, 13, 352, p.p31);s.store_scaled_mul(367, 13, 353, p.p31);s.store_scaled_mul(368, 13, 354, p.p31);s.store_scaled_mul(369, 13, 355, p.p31);s.store_mul(1765, 13, 359);s.store_mul(1766, 13, 360);s.b[1767] = (s.v[334] < 0.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });s.b[1768] = (s.v[311] > 0.0);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });s.b[1769] = (s.v[318] > 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });s.b[1770] = (s.v[322] > 0.0);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });s.b[1771] = (s.v[326] > 0.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });s.copy_ad(1774, 361);s.copy_ad(1775, 362);s.copy_ad(1776, 363);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.b[1777] = (s.v[334] < 0.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if s.b[1777] {s.copy_ad(1776, 364);}
        s.store_scaled_mul(361, 13, 361, p.p32);s.store_scaled_mul(362, 13, 362, p.p32);s.store_scaled_mul(363, 13, 363, p.p32);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.store_scaled_mul(379, 13, 379, p.p32);s.store_scaled_mul(380, 13, 380, p.p32);s.store_scaled_mul(381, 13, 381, p.p32);s.store_scaled_mul(382, 13, 382, p.p32);s.store_scaled_mul(370, 13, 370, p.p32);s.store_scaled_mul(371, 13, 371, p.p32);s.store_scaled_mul(372, 13, 372, p.p32);s.store_scaled_mul(373, 13, 373, p.p32);s.store_scaled_mul(374, 13, 374, p.p32);s.store_scaled_mul(377, 13, 377, p.p32);s.store_scaled_mul(376, 13, 376, p.p32);s.store_scaled_mul(375, 13, 375, p.p32);s.store_mul(378, 13, 378);s.b[1778] = (s.v[334] < 0.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if s.b[1778] {s.copy_ad(1772, 363);s.copy_ad(363, 364);s.copy_ad(364, 1772);s.store_neg(375, 375);s.copy_ad(1772, 380);s.copy_ad(380, 379);s.copy_ad(379, 1772);s.copy_ad(1772, 382);s.copy_ad(382, 381);s.copy_ad(381, 1772);}
        s.b[1779] = (s.v[13] > 0.0);s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if s.b[1779] {s.store_mul_div_scaled_inputs_mixed_aia(1773, A::add_scaled_product(A::div_scaled_product_by_product(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775)), 1.0, s.ad_value(116), s.ad_value(239), 1.0), 1.0, s.ad_value(180), s.ad_value(226), 1.0), 342, 1e-9, A::mul(s.ad_value(345), s.ad_value(116)), 1.0);}
        if (!s.b[1779]) {s.store_scalar(1773, 0.0);}
        s.store_scaled_mul(1780, 390, 226, 1.0 / (1.602176565e-19));s.store_scaled_add(1781, 407, 432, (-0.5));s.store_add(1782, 415, 1781);s.store_div(0, 415, 1782);s.store_scaled_add_mixed_ia(1787, 0, A::sqrt_square_offset(s.ad_value(0), 1e-20), 0.5);s.store_scaled_mul(1788, 436, 435, (-0.1666666666667));s.store_square(1789, 1788);s.store_offset(1790, 429, (-1.0));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_max_with_scalar_ad(1791, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1790), 12.0, s.ad_value(1789))), 1e-20);s.store_div_from_scalar_square_ad(1792, 1.0, s.ad_value(1791));s.store_div_scaled_product3_by_product_mixed_aiiii(1793, A::mul3(s.ad_value(342), s.ad_value(390), s.ad_value(226)), 1782, 344, 1.0, 345, 346, 1.0);s.store_scale(1794, 1789, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1787, 1.0, 1794, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794), s.ad_value(1790), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);let t0: f64 = (s.v[1793] * s.v[1792]);let t1: f64 = (t0 * s.v[3]);s.store_scalar(1795, t1);s.b[1812] = (s.v[172] > 0.0);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        let (t3,) = {
    if s.b[1812] {
        let t2: f64 = (s.v[427] / s.v[422]);
        (t2,)
    } else {
        (s.v[1796],)
    }
};
        s.store_scalar(1796, t3);
        let (tc,) = {
    if s.b[1812] {
        let t4: f64 = (s.v[309] * s.v[348]);let t5: f64 = (t4 * s.v[411]);let t6: f64 = (t5 * s.v[223]);let t7: f64 = (s.v[1796] * s.v[1796]);let t8: f64 = (1.0 + t7);let t9: f64 = (t8 * s.v[1791]);let ta: f64 = (t9 * s.v[1791]);let tb: f64 = (t6 / ta);
        (tb,)
    } else {
        (s.v[1797],)
    }
};
        s.store_scalar(1797, tc);
        let (tf,) = {
    if s.b[1812] {
        let td: f64 = (s.v[1797] / s.v[308]);let te: f64 = (s.v[1795] + td);
        (te,)
    } else {
        (s.v[1795],)
    }
};
        s.store_scalar(1795, tf);s.store_div_scaled_product3_indices(1799, 456, 447, 116, 1.0, 469, 1.0);s.store_mul_scale_offset_indices(1800, 1799, 468, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1802, 1800, A::mul_scaled_lhs(s.ad_value(334), 0.25, s.ad_value(1788)), -1.0, 0.5);s.store_sub(1801, 1800, 1802);s.store_scalar(1805, 0.0);s.b[1813] = (p.p6 > 0.0);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if s.b[1813] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1787), 0.08333333333333333, s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)), (-1.0)), A::mul3_scaled_output(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794)), s.ad_value(1790), 1.6));s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1803, 1793, 1791, 1791, 1.0, 3, 1.0);}
        s.b[1814] = (s.v[1795] > 0.0);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        if (s.b[1813] && s.b[1814]) {s.store_mul_ad_product_rhs_mixed_ia(1805, 1792, 1788, A::add_scaled_sub_value_product(1.0, s.ad_value(1794), 1.0, A::add_scaled_inputs_product(s.ad_value(1787), 1.0, s.ad_value(1789), 19.2, s.ad_value(1787), s.ad_value(1794), (-1.0)), s.ad_value(1790), (-1.0)));}
        if (!s.b[1813]) {s.store_scalar(1803, 1.0);}
        s.copy_ad(1783, 1780);s.store_mul_scale_offset_indices(1784, 1780, 415, 1.0, 1.0);s.store_mul_sub_rhs(1785, 1780, 403, 413);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5, A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1783), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1783), s.ad_value(1783)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1783), 2.0), 1.0), 1785, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(417), 1.0, s.ad_value(177), s.ad_value(418), 1.0), A::offset(s.ad_value(415), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(347), s.ad_value(348), 1.602176565e-19, s.ad_value(345), 1.0), 3, 1.0, 1783, 1.0);s.store_div_from_scalar_scaled_input(1823, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1824, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1825, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1826, 15, 1825, 1.0, 1824, (-1.0), 228, (-0.4), 0.0);s.store_add(1827, 1824, 1826);s.store_scaled_mul(1828, 1827, 1823, 0.5);s.store_sub_scaled_inputs(1829, 15, 0.05, 1826, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1830, 2, 238);s.store_div_scaled_value_offset_denominator(1831, s.ad_value(1823), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1833, 1830, 229, (2.0 * 1.602176565e-19), 0.0, 1831);s.store_add_offset_lhs_mixed_ai(1834, A::ln(A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(1833), 1.0)), (-0.6931471805599), 1828);s.store_mul_div_scaled_product_mixed_iiia(1835, 1831, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(1838, 35, 1831);s.store_scalar(1839, 0.0);s.store_scalar(1832, 0.0);s.b[1884] = (p.p9 > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1832, 1.0, 1823, A::ln(A::div(s.ad_value(24), s.ad_value(251))));}
        s.b[1885] = (p.p13 > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });s.b[1886] = (p.p14 == 1.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1885] && s.b[1886]) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[1885] && (!s.b[1886])) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_mul(1842, 336, 1831);s.store_mul_scale_offset_mixed_ia(1843, 1831, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(1844, 1842, 1843, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1815, 402, 1.0, 401, A::offset(s.ad_value(402), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 401, 1.0, 402, A::offset(s.ad_value(401), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(401), A::offset(s.ad_value(1815), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(402), A::offset(s.ad_value(1816), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 399, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(399), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(398), -1.0, s.ad_value(25), 1.0), 398);s.store_add_scaled_product_mixed_iia(1822, 21, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(394), (-1.0), s.ad_value(395), 1.0), 1.0, s.ad_value(397), (-1.0), s.ad_value(394), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(183), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);s.store_add_scaled_inputs4_indices(1841, 184, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);s.b[1887] = (p.p2 > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if s.b[1887] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);}
        s.b[1888] = (s.v[0] < 0.0);s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });
        if (s.b[1887] && s.b[1888]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1887] && (!s.b[1888])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if s.b[1887] {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);}
        if (!s.b[1887]) {s.copy_ad(1847, 1846);}
        s.store_mul_sub_rhs(0, 248, 1845, 1847);s.b[1889] = (p.p13 > 0.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
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
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);s.b[1894] = (p.p11 > 0.0);s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });
        if s.b[1894] {s.store_div_scaled_value_by_product_mixed_iia(1815, 457, 1.0, 456, A::offset(s.ad_value(457), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 456, 1.0, 457, A::offset(s.ad_value(456), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(456), A::offset(s.ad_value(1815), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(457), A::offset(s.ad_value(1816), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 455, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(455), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(454), -1.0, s.ad_value(25), 1.0), 454);s.store_add_scaled_product_mixed_iia(1822, 130, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(450), (-1.0), s.ad_value(451), 1.0), 1.0, s.ad_value(453), (-1.0), s.ad_value(450), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(185), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);s.store_add_scaled_inputs4_indices(1841, 186, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);}
        s.b[1895] = (p.p2 > 0.0);s.store_scalar(1895, if s.b[1895] { 1.0 } else { 0.0 });
        if (s.b[1894] && s.b[1895]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);}
        s.b[1896] = (s.v[0] < 0.0);s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });
        if ((s.b[1894] && s.b[1895]) && s.b[1896]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1894] && s.b[1895]) && (!s.b[1896])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if (s.b[1894] && s.b[1895]) {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);}
        if (s.b[1894] && (!s.b[1895])) {s.copy_ad(1847, 1846);}
        if s.b[1894] {s.store_mul_sub_rhs(0, 248, 1845, 1847);}
        s.b[1897] = (p.p13 > 0.0);s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });
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
        if s.b[1894] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        s: &mut Scratch,
    ) {
        if s.b[1894] {s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(7, (273.15 + p.p15));s.store_scalar(0, ((ctx_temp + p.p36)).min(1000.0));s.b[529] = (p.p10 == 1.0);s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });
        if s.b[529] {s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));s.store_scaled_add_offset_sqrt_square_offset_ad(225, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (!s.b[529]) {s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));s.store_scalar(225, 600.0);}
        s.b[530] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0)));s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });
        if s.b[530] {s.store_scalar(6, p.p5);}
        if (!s.b[530]) {s.store_scalar(6, 0.0);}
        s.store_scalar(219, 0.0);s.copy_ad(217, 8);s.store_square(218, 217);s.store_offset(220, 217, (-s.v[7]));s.store_scale(221, 217, 1.0 / (s.v[7]));s.store_div_from_scalar(222, s.v[7], 217);s.store_scale(223, 217, 8.617332384961e-5);s.store_div_from_scalar(224, 1.0, 223);s.b[611] = (p.p0 == 0.0);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_scalar(10, p.p23);s.store_scalar(9, p.p22);s.store_scalar(12, p.p25);s.store_scalar(11, p.p24);s.store_scalar(13, p.p30);s.store_scalar(533, p.p41);s.store_scalar(14, p.p42);s.store_scalar(15, p.p43);s.store_scalar(534, p.p44);s.store_scalar(535, 1.0);}
        s.b[612] = (p.p45 < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[612]) {s.store_scalar(535, (-1.0));}
        if s.b[611] {s.store_scalar(536, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[613] = (p.p46 < 0.0);s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[613]) {s.store_scalar(16, (-1.0));}
        if s.b[611] {s.store_scalar(537, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p47);s.store_scalar(18, p.p48);s.store_scalar(19, (p.p49 * 1000000.0));s.store_scalar(20, (p.p50 * 1000000.0));s.store_scalar(183, p.p51);s.store_scalar(184, p.p52);s.store_scalar(23, p.p53);s.store_scalar(24, (p.p54 * 1000000.0));s.store_scalar(25, p.p55);s.store_scalar(26, p.p56);s.store_scalar(27, p.p57);s.store_primal_div_scaled_product_indices(28, 27, 534, p.p58, 533, 1.0);s.store_scalar(29, (p.p59 * 1000000.0));s.store_scalar(30, p.p60);s.store_scalar(538, p.p61);s.store_scalar(187, p.p62);s.store_div_scaled_product_indices(188, 187, 534, p.p63, 533, 1.0);s.store_scalar(34, p.p64);s.store_scalar(35, p.p65);s.store_scalar(36, p.p66);s.store_scalar(37, p.p67);s.store_scalar(191, p.p68);s.store_scale(192, 191, p.p69);s.store_scalar(40, p.p70);s.store_scalar(195, p.p71);s.store_scalar(41, p.p72);s.store_scalar(42, p.p73);s.store_scalar(43, p.p74);s.store_scalar(196, p.p75);s.store_scalar(45, p.p76);s.store_scalar(539, p.p77);s.store_scalar(540, p.p78);s.store_scalar(193, p.p79);s.store_scalar(48, p.p80);s.store_scalar(194, p.p81);s.store_scalar(49, p.p82);s.store_scalar(197, p.p83);s.store_scalar(51, p.p84);s.store_scalar(52, p.p85);s.store_scalar(541, p.p86);s.store_scalar(198, p.p87);s.store_scalar(54, p.p88);s.store_scalar(55, p.p89);s.store_scalar(56, p.p90);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[611] {s.store_scalar(57, p.p91);s.store_scalar(58, p.p92);s.store_scalar(199, p.p93);s.store_scalar(60, p.p94);s.store_scalar(61, p.p95);s.store_scalar(62, p.p96);s.store_scalar(542, p.p97);s.store_scalar(63, p.p98);s.store_scalar(64, p.p99);s.store_scalar(65, p.p100);s.store_scalar(66, p.p101);s.store_scalar(67, p.p102);s.store_scalar(75, p.p103);s.store_scalar(201, p.p104);s.store_scalar(202, p.p105);s.store_scalar(203, p.p106);s.store_scalar(204, p.p107);s.store_scalar(205, p.p108);s.store_scalar(76, p.p109);s.store_scalar(77, p.p123);s.store_scalar(78, p.p110);s.store_scalar(79, p.p111);s.store_scalar(80, p.p112);s.store_scalar(81, p.p122);s.store_scalar(82, p.p113);s.store_scalar(83, p.p114);s.store_scalar(84, p.p115);s.store_scalar(85, p.p116);s.store_scalar(86, p.p117);s.store_scalar(87, p.p118);s.store_scalar(88, p.p119);s.store_scalar(89, p.p124);s.store_scalar(90, p.p125);s.store_scalar(208, p.p126);s.store_scalar(209, p.p127);s.store_scalar(93, p.p128);s.store_scalar(94, p.p129);s.store_scalar(95, p.p130);s.store_scalar(96, p.p131);s.store_scalar(97, p.p132);s.store_scalar(98, p.p133);s.store_scalar(210, p.p148);s.store_scalar(114, p.p149);s.store_scalar(115, p.p150);s.store_scalar(99, p.p134);s.store_scalar(211, p.p135);s.store_scalar(212, p.p136);s.store_scalar(102, p.p137);s.store_scalar(103, p.p138);s.store_scalar(104, p.p139);s.store_scalar(105, p.p140);s.store_div_scaled_product_indices(106, 105, 534, p.p141, 533, 1.0);s.store_scalar(107, p.p142);s.store_div_scaled_product_indices(108, 107, 534, p.p143, 533, 1.0);s.store_scalar(109, p.p144);s.store_scalar(213, p.p145);s.store_scalar(111, p.p146);s.store_scalar(116, p.p151);s.store_scalar(117, p.p152);s.store_scalar(118, (p.p153 * 1000000.0));s.store_scalar(119, p.p154);s.store_scalar(120, p.p155);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(189, 187);s.copy_ad(190, 188);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[614] = (p.p11 > 0.0);s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[614]) {s.store_scalar(185, p.p51);}
        s.b[615] = param_given[156];s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[615]) {s.store_scalar(185, p.p156);}
        if (s.b[611] && s.b[614]) {s.store_scalar(186, p.p52);}
        s.b[616] = param_given[157];s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[616]) {s.store_scalar(186, p.p157);}
        if (s.b[611] && s.b[614]) {s.store_scalar(135, p.p57);}
        s.b[617] = param_given[158];s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[617]) {s.store_scalar(135, p.p158);}
        if (s.b[611] && s.b[614]) {s.store_primal_div_scaled_product_indices(136, 135, 534, p.p58, 533, 1.0);s.store_scalar(189, p.p62);}
        s.b[618] = param_given[159];s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[618]) {s.store_scalar(189, p.p159);}
        if (s.b[611] && s.b[614]) {s.store_div_scaled_product_indices(190, 189, 534, p.p63, 533, 1.0);s.store_scalar(200, p.p93);}
        s.b[619] = param_given[160];s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[619]) {s.store_scalar(200, p.p160);}
        if (s.b[611] && s.b[614]) {s.store_scalar(543, p.p97);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[620] = param_given[161];s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[620]) {s.store_scalar(543, p.p161);}
        if (s.b[611] && s.b[614]) {s.store_scalar(158, p.p98);}
        s.b[621] = param_given[162];s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[621]) {s.store_scalar(158, p.p162);}
        if s.b[611] {s.store_scalar(159, p.p163);s.store_scalar(160, p.p164);s.store_scalar(161, p.p165);s.store_scalar(162, p.p166);s.store_scalar(163, p.p167);s.store_scalar(164, p.p168);s.store_scalar(165, p.p169);s.store_scalar(166, p.p170);s.store_scalar(167, p.p171);s.store_scalar(214, p.p172);s.store_scalar(169, p.p173);s.store_scalar(170, p.p174);s.store_scalar(173, p.p177);s.store_scalar(174, p.p178);s.store_scalar(175, p.p179);s.store_scalar(176, p.p180);s.store_scalar(177, p.p181);s.store_scalar(179, p.p183);s.store_scalar(180, p.p184);s.store_scalar(181, p.p185);s.store_scalar(182, p.p186);}
        if (!s.b[611]) {s.store_scalar(588, (1.0 / p.p29));s.store_primal_max_with_scalar_ad(532, A::scale(s.ad_value(588), p.p21), 1e-9);s.store_primal_scale(10, 588, p.p23);s.store_primal_scale(9, 588, p.p22);s.store_primal_scale(12, 588, p.p25);s.store_primal_scale(11, 588, p.p24);s.store_scalar(13, (p.p30 * p.p29));s.store_scalar(569, 1e-6);s.store_scalar(570, 1e-6);s.store_primal_scale(571, 569, 1.0 / (p.p20));s.store_primal_div(572, 570, 532);s.store_primal_scaled_mul_scale_offset_inputs(573, 571, p.p192, 1.0, 572, p.p193, 1.0, p.p191);s.store_primal_scaled_mul_scale_offset_inputs(574, 572, p.p197, 1.0, 571, p.p196, 1.0, p.p195);s.store_primal_max_with_scalar_ad(575, A::offset(s.ad_value(573), ((p.p20) + ((-(2.0 * p.p194))))), 1e-9);s.store_primal_max_with_scalar_ad(576, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p.p198))), 1e-9);s.store_primal_max_with_scalar_ad(577, A::offset(s.ad_value(573), ((((p.p20) + ((-(2.0 * p.p194))))) + (p.p199))), 1e-9);s.store_primal_max_with_scalar_ad(578, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (((-(2.0 * p.p198))) + (p.p200))), 1e-9);s.store_primal_div(579, 569, 575);s.store_primal_div(580, 570, 576);s.store_primal_mul(581, 579, 580);s.store_max_with_scalar_ad(0, A::offset(s.ad_value(573), p.p20), 1e-9);s.store_div(582, 0, 569);s.store_max_with_scalar_ad(0, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);s.store_div(583, 0, 570);s.store_scalar(533, p.p201);s.store_scalar(14, p.p202);s.store_scalar(15, p.p203);s.store_scalar(534, p.p204);s.store_scalar(535, 1.0);}
        s.b[622] = (p.p205 < 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[622]) {s.store_scalar(535, (-1.0));}
        if (!s.b[611]) {s.store_scalar(536, ((((p.p205) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[623] = (p.p206 < 0.0);s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[623]) {s.store_scalar(16, (-1.0));}
        if (!s.b[611]) {s.store_scalar(537, (((((p.p206) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p207);s.store_scalar(18, p.p208);s.store_scalar(19, (p.p209 * 1000000.0));s.store_scalar(20, (p.p210 * 1000000.0));s.store_div_scaled_inputs(0, A::powf(s.ad_value(579), p.p213), p.p212, A::scale_offset(A::powf(s.ad_value(579), p.p215), p.p214, 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(183, 0, 1.0, 580, p.p216, 581, p.p217, p.p211);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_offset_mul_ad(184, A::div_scaled_inputs(s.ad_value(534), p.p219, s.ad_value(533), 1.0), s.ad_value(0), p.p218);s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(579), p.p221, 1.0), A::scale_offset(s.ad_value(580), p.p222, 1.0), A::scale_offset(s.ad_value(581), p.p223, 1.0), p.p220);s.store_offset_scaled(607, 579, ((p.p225) * ((p.p224 * 1000000.0))), (p.p224 * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(607), 1e25), 1e28);s.store_scalar(25, p.p226);s.store_scalar(26, p.p227);s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(584, A::sqrt(A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11))), 575);s.store_primal_mul_powf_scale_offset_lhs(544, 584, 580, p.p229, (p.p230) * ((p.p228 * 2.0)), (1.0) * ((p.p228 * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(544), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 534, p.p231, 533, 1.0);s.store_scalar(29, (p.p232 * 1000000.0));s.store_scalar(30, p.p233);s.store_primal_scale(549, 580, p.p234);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(549), (-1.0)), 1.0);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p236, p.p237, 1.0);s.store_scale(546, 0, p.p235);s.store_max_with_scalar(187, 546, 0.0);s.store_div_scaled_product_indices(188, 187, 534, p.p238, 533, 1.0);s.store_scale(34, 0, p.p239);s.store_scalar(35, p.p240);s.store_primal_div_scaled_inputs_mixed_ia(36, 579, p.p241, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p242, 1.0), 0.001), 1.0);s.store_scalar(37, p.p243);s.store_div_scaled_inputs_mixed_ia(2, 575, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p248, 1.0), 0.001), p.p247);}
        s.b[624] = (s.v[2] > (-80.0));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[624]) {s.store_exp(3, 2);}
        if ((!s.b[611]) && (!s.b[624])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_scale(4, 575, (-1.0 / (p.p250)));}
        s.b[625] = (s.v[4] > (-80.0));s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[625]) {s.store_exp(5, 4);}
        if ((!s.b[611]) && (!s.b[625])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_max_with_scalar_ad(585, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(580), p.p246, 1.0), s.ad_value(3), (-1.0), p.p245, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p249, ((-1.0) * p.p249), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(586, A::add_scaled_product(A::scale_offset(s.ad_value(580), p.p251, 1.0), 1.0, s.ad_value(580), A::ln(A::scale_offset(s.ad_value(576), 1.0 / (p.p253), 1.0)), p.p252), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(587, p.p244, 585, 586);s.store_div_scaled_product_indices(548, 587, 576, 1.0, 575, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(579), p.p256, 1.0), A::scale_offset(s.ad_value(580), p.p257, 1.0), A::scale_offset(s.ad_value(581), p.p258, 1.0), p.p255);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_primal_mul3_ad(550, A::scale_offset(A::powf(s.ad_value(579), p.p261), p.p260, p.p259), A::scale_offset(s.ad_value(580), p.p262, 1.0), A::scale_offset(s.ad_value(581), p.p263, 1.0));s.store_primal_max_with_scalar(195, 550, 0.0);s.store_scalar(41, p.p264);s.store_scalar(42, p.p265);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(579), p.p267, 1.0), A::scale_offset(s.ad_value(580), p.p268, 1.0), A::scale_offset(s.ad_value(581), p.p269, 1.0), p.p266);s.store_scalar(196, p.p270);s.store_scalar(45, p.p271);s.store_scalar(539, p.p272);s.store_scalar(540, p.p273);s.store_scalar(193, p.p274);s.store_scalar(48, p.p275);s.store_scalar(194, p.p276);s.store_scalar(49, p.p277);s.store_primal_mul3_ad(197, A::scale_offset(A::powf(s.ad_value(579), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(580), p.p281, 1.0), A::scale_offset(s.ad_value(581), p.p282, 1.0));s.store_scalar(51, p.p283);s.store_scalar(52, p.p284);s.store_scalar(541, p.p285);s.store_primal_mul_scale_offset_rhs(551, 580, 580, ((p.p287) * (p.p286)), p.p286);s.store_primal_max_with_scalar(198, 551, 0.0);s.store_scalar(54, p.p288);s.store_scalar(55, p.p289);s.store_scalar(56, p.p290);s.store_scalar(57, p.p291);s.store_scalar(58, p.p292);s.store_mul_scale_offset_mixed_ai(552, A::mul3(s.ad_value(587), A::scale_offset(A::powf(s.ad_value(579), p.p295), p.p294, p.p293), A::scale_offset(s.ad_value(580), p.p296, 1.0)), 581, p.p297, 1.0);s.store_max_with_scalar(199, 552, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(579), p.p299, 1.0), A::scale_offset(s.ad_value(580), p.p300, 1.0), A::scale_offset(s.ad_value(581), p.p301, 1.0), p.p298);s.store_scalar(61, p.p302);s.store_scalar(62, p.p303);s.store_primal_div_from_scalar_offset_ad(554, p.p304, A::div_scaled_inputs(A::powf(s.ad_value(579), p.p306), p.p305, A::scale_offset(A::powf(s.ad_value(579), p.p308), p.p307, 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(542, A::max_with_scalar(s.ad_value(554), 1.0), 16.0);s.store_primal_div_scaled_product(557, A::powf(s.ad_value(579), p.p310), A::scale_offset(s.ad_value(580), p.p313, 1.0), p.p309, A::scale_offset(A::powf(s.ad_value(579), p.p312), p.p311, 1.0), 1.0);s.store_primal_max_with_scalar(63, 557, 0.0);s.store_primal_div_scaled_product(558, A::powf(s.ad_value(579), p.p315), A::scale_offset(s.ad_value(580), p.p318, 1.0), p.p314, A::scale_offset(A::powf(s.ad_value(579), p.p317), p.p316, 1.0), 1.0);s.store_primal_max_with_scalar(64, 558, 0.0);s.store_scalar(65, p.p319);s.store_scalar(66, p.p320);s.store_scalar(67, p.p321);s.store_scalar(75, p.p322);s.store_primal_div_from_scalar(201, p.p323, 581);s.store_primal_div_from_scalar(202, p.p324, 580);}
    }
}
