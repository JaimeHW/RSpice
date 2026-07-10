#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1705] = (p.p76 != 2.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });s.b[1706] = (p.p65 == 1.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });s.b[1707] = (p.p78 == 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });s.b[1708] = (p.p65 == 1.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });s.b[1709] = (p.p78 == 1.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });s.b[1710] = (p.p61 != 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });s.b[1711] = (p.p64 == 1.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });s.b[1712] = (p.p1910 > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (s.b[1711] && s.b[1712]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }
        s.b[1713] = (p.p75 != 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if ((s.b[1711] && s.b[1712]) && s.b[1713]) {s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));}
        if ((s.b[1711] && s.b[1712]) && (!s.b[1713])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }
        if (s.b[1711] && s.b[1712]) {s.store_offset(168, 392, (-p.p1906));s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);}
        if (s.b[1711] && s.b[1712]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_mixed_ia(1045, 1045, A::sqrt_square_offset(s.ad_value(1045), ((4.0 * 10.0) * 10.0)), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }
        if (s.b[1711] && s.b[1712]) {s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));s.store_abs_voltage(174, ctx, nodes, Some(9), Some(7));}
        s.b[1714] = (p.p1917 == 0.0);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if ((s.b[1711] && s.b[1712]) && s.b[1714]) {s.store_scalar(171, 1.0);}
        if ((s.b[1711] && s.b[1712]) && (!s.b[1714])) {s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(s.ad_value(174), (-p.p1916)), ((0.25 * 0.5) * 0.5), 0.5);s.store_offset_scaled(171, 171, p.p1917, 1.0);}
        if (s.b[1711] && s.b[1712]) {s.store_scaled_mul(1047, 170, 171, p.p1903);s.store_scaled_mul(172, 1039, 189, p.p1910);s.store_mul(1048, 1047, 172);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1711] && s.b[1712]) {let t0: A = A::powf(s.ad_value(174), (4.0 - p.p1908));s.store_div_ad(1050, t0, A::add_scaled_inputs(t0, 1.0, A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914));s.store_div_scaled_product_mixed_aii(175, A::powf(s.ad_value(1050), (1.0 / p.p1908)), 174, 1.0, 1048, 1.0);s.store_mul_powf_mixed_ia(1041, 172, A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908));}
        s.b[1715] = (p.p1911 > 0.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });s.b[1716] = (p.p1910 == 0.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }
        s.b[1717] = (p.p75 != 0.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && s.b[1717]) {s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));}
        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && (!s.b[1717])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }
        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {s.store_offset(168, 392, (-p.p1906));s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);}
        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_mixed_ia(1045, 1045, A::sqrt_square_offset(s.ad_value(1045), ((4.0 * 10.0) * 10.0)), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }
        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));}
        if (s.b[1711] && s.b[1715]) {s.store_scale(1046, 170, p.p1909);s.store_scaled_mul(172, 1039, 189, p.p1911);s.store_mul(1049, 1046, 172);s.store_abs_voltage(174, ctx, nodes, Some(6), Some(8));let t1: A = A::powf(s.ad_value(174), (4.0 - p.p1908));s.store_div_ad(1051, t1, A::add_scaled_inputs(t1, 1.0, A::powf(s.ad_value(1049), (4.0 - p.p1908)), p.p1915));s.store_div_scaled_product_mixed_aii(175, A::powf(s.ad_value(1051), (1.0 / p.p1908)), 174, 1.0, 1049, 1.0);s.store_mul_powf_mixed_ia(1040, 172, A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1718] = ((p.p64 != 2.0) && (s.v[191] > 0.0));s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if s.b[1718] {s.store_div_from_scalar(596, 1.0, 192);}
        s.b[1719] = ((p.p64 == 1.0) && (p.p1910 > 0.0));s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (s.b[1718] && s.b[1719]) {s.store_div_from_scalar(1042, 1.0, 1041);}
        s.b[1720] = ((p.p64 != 2.0) && (s.v[190] > 0.0));s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if s.b[1720] {s.store_div_from_scalar(595, 1.0, 193);}
        s.b[1721] = ((p.p64 == 1.0) && (p.p1911 > 0.0));s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if (s.b[1720] && s.b[1721]) {s.store_div_from_scalar(1043, 1.0, 1040);}
        s.b[1722] = ((p.p73 == 1.0) && (s.v[873] != 0.0));s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });s.b[1723] = (p.p73 == 2.0);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });s.b[1724] = (p.p76 != 0.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });s.b[1725] = (p.p76 == 2.0);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });s.b[1726] = ((p.p64 != 2.0) && (s.v[191] > 0.0));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });s.b[1727] = ((p.p64 == 1.0) && (p.p1910 > 0.0));s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });s.b[1728] = ((p.p64 != 2.0) && (s.v[190] > 0.0));s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });s.b[1729] = ((p.p64 == 1.0) && (p.p1911 > 0.0));s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });s.b[1730] = (p.p76 != 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });s.b[1731] = (p.p72 == 0.0);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });s.b[1732] = (p.p68 != 0.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });s.b[1733] = (s.v[128] > 0.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });s.b[1734] = (p.p69 != 0.0);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });s.b[1735] = (p.p61 != 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });s.b[1736] = ((p.p74 != 0.0) && (p.p1791 > 0.0));s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });s.b[1737] = ((p.p64 != 2.0) && (s.v[191] > 0.0));s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });s.b[1738] = ((p.p64 == 1.0) && (p.p1910 > 0.0));s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });s.b[1739] = ((p.p64 != 2.0) && (s.v[190] > 0.0));s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });s.b[1740] = ((p.p64 == 1.0) && (p.p1911 > 0.0));s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });s.store_add_scaled_inputs_mixed_ai(339, A::div_scaled_inputs(s.ad_value(179), 10.0, s.ad_value(898), 1.0), 1.0, 396, 2.0);s.store_mul_add_rhs(169, 179, 179, 339);s.store_mul_square_lhs(170, 163, 169);s.store_scaled_mul(171, 141, 179, ((2.0 * 1.60219e-19) * s.v[143]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(188, 0.0);s.store_scalar(197, 0.0);s.store_scalar(263, 0.0);s.store_scalar(264, 0.0);s.store_scalar(265, 0.0);s.store_scalar(266, 0.0);s.store_scalar(267, 0.0);s.store_scalar(268, 0.0);s.store_scalar(269, 0.0);s.store_scalar(270, 0.0);s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);s.store_scalar(273, 0.0);s.store_scalar(274, 0.0);s.store_scalar(275, 0.0);s.store_scalar(276, 0.0);s.store_scalar(277, 0.0);s.store_scalar(278, 0.0);s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);s.store_scalar(286, 0.0);s.store_scalar(287, 0.0);s.store_scalar(288, 0.0);s.store_scalar(289, 0.0);s.store_scalar(290, 0.0);s.store_scalar(291, 0.0);s.store_scalar(292, 0.0);s.store_scalar(300, 0.0);s.store_scalar(302, 0.0);s.store_scalar(305, 0.0);s.store_scalar(314, 0.0);s.store_scalar(315, 0.0);s.store_scalar(316, 0.0);s.store_scalar(320, 0.0);s.store_scalar(333, 0.0);s.store_scalar(335, 0.0);s.store_scalar(338, 0.0);s.store_scalar(258, 0.0);s.store_scalar(857, 0.0);s.store_scalar(373, 0.0);s.store_scalar(401, 0.0);s.store_scalar(417, 0.0);s.store_scalar(453, 0.0);s.store_scalar(756, 0.0);s.store_scalar(757, 0.0);s.store_scalar(255, 0.0);s.store_scalar(758, 0.0);s.store_scalar(759, 0.0);s.store_scalar(760, 0.0);s.store_scalar(770, 0.0);s.store_scalar(771, 0.0);s.store_scalar(251, 0.0);s.store_scalar(772, 0.0);s.store_scalar(773, 0.0);s.store_scalar(774, 0.0);s.store_scalar(494, 0.0);s.store_scalar(495, 0.0);s.store_scalar(496, 0.0);s.store_scalar(498, 0.0);s.store_scalar(499, 0.0);s.store_scalar(523, 0.0);s.store_scalar(524, 0.0);s.store_scalar(525, 0.0);s.store_scalar(526, 0.0);s.store_scalar(527, 0.0);s.store_scalar(528, 0.0);s.store_scalar(529, 0.0);s.store_scalar(533, 0.0);s.store_scalar(537, 0.0);s.store_scalar(538, 0.0);s.store_scalar(539, 0.0);s.store_scalar(540, 0.0);s.store_scalar(546, 0.0);s.store_scalar(547, 0.0);s.store_scalar(541, 0.0);s.store_scalar(542, 0.0);s.store_scalar(543, 0.0);s.store_scalar(553, 0.0);s.store_scalar(554, 0.0);s.store_scalar(548, 0.0);s.store_scalar(549, 0.0);s.store_scalar(550, 0.0);s.store_scalar(557, 0.0);s.store_scalar(558, 0.0);s.store_scalar(559, 0.0);s.store_scalar(560, 0.0);s.store_scalar(561, 0.0);s.store_scalar(562, 0.0);s.store_scalar(563, 0.0);s.store_scalar(564, 0.0);s.store_scalar(565, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(566, 0.0);s.store_scalar(567, 0.0);s.store_scalar(568, 0.0);s.store_scalar(589, 0.0);s.store_scalar(574, 0.0);s.store_scalar(575, 0.0);s.store_scalar(620, 0.0);s.store_scalar(632, 0.0);s.store_scalar(634, 0.0);s.store_scalar(668, 0.0);s.store_scalar(665, 0.0);s.store_scalar(677, 0.0);s.store_scalar(806, 0.0);s.store_scalar(370, 0.0);s.store_scalar(689, 0.0);s.store_scalar(690, 0.0);s.store_scalar(691, 0.0);s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);s.store_scalar(871, 0.0);s.store_scalar(872, 0.0);s.store_scalar(680, 0.0);s.store_scalar(699, 0.0);s.store_scalar(658, 0.0);s.store_scalar(791, 0.0);s.store_scalar(701, 0.0);s.store_scalar(851, 0.0);s.store_scalar(706, 0.0);s.store_scalar(710, 0.0);s.store_scalar(815, 0.0);s.store_scalar(809, 0.0);s.store_scalar(817, 0.0);s.store_scalar(816, 0.0);s.store_scalar(818, 0.0);s.store_scalar(845, 0.0);s.store_scalar(846, 0.0);s.store_scalar(825, 0.0);s.store_scalar(828, 0.0);s.store_scalar(843, 0.0);s.store_scalar(844, 0.0);s.store_scalar(715, 0.0);s.store_scalar(717, 0.0);s.store_scalar(796, 0.0);s.store_scalar(646, 0.0);s.store_scalar(647, 0.0);s.store_scalar(645, 0.0);s.store_scalar(644, 0.0);s.store_scalar(893, 0.0);s.store_scalar(894, 0.0);s.store_scalar(895, 0.0);s.store_scalar(896, 0.0);s.store_scalar(898, 0.0);s.store_scalar(903, 0.0);s.store_scalar(904, 0.0);s.store_scalar(923, 0.0);s.store_scalar(392, 0.0);s.store_scalar(393, 0.0);s.store_scalar(503, 0.0);s.store_scalar(504, 0.0);s.store_scalar(949, 0.0);s.store_scalar(950, 0.0);s.store_scalar(951, 0.0);s.store_scalar(952, 0.0);s.store_scalar(953, 0.0);s.store_scalar(955, 0.0);s.store_scalar(956, 0.0);s.store_scalar(957, 0.0);s.store_scalar(958, 0.0);s.store_scalar(959, 0.0);s.store_scalar(1004, 0.0);s.store_scalar(1005, 0.0);s.store_scalar(1006, 0.0);s.store_scalar(1007, 0.0);s.store_scalar(1008, 0.0);s.store_scalar(1009, 0.0);s.store_scalar(983, 1.0);s.store_scalar(960, 0.0);s.store_scalar(961, 0.0);s.store_scalar(962, 0.0);s.store_scalar(963, 0.0);s.store_scalar(964, 0.0);s.store_scalar(965, 0.0);s.store_scalar(984, 0.0);s.store_scalar(985, 0.0);s.store_scalar(986, 0.0);s.store_scalar(1010, 0.0);s.store_scalar(1011, 0.0);s.store_scalar(1012, 0.0);s.store_scalar(882, 0.0);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);s.store_scalar(885, 0.0);s.store_scalar(886, 0.0);s.store_scalar(887, 0.0);s.store_scalar(888, 0.0);s.store_scalar(889, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(890, 0.0);s.store_scalar(891, 0.0);s.store_scalar(892, 0.0);s.store_scalar(119, 0.0);s.store_scalar(120, 0.0);s.store_scalar(118, 0.0);s.store_scalar(117, 0.0);s.store_scalar(233, 0.0);s.store_scalar(234, 0.0);s.store_scalar(182, 0.0);s.store_scalar(142, 0.0);s.store_scalar(324, 0.0);s.store_scalar(327, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(310, 0.0);s.store_scalar(311, 0.0);s.store_scalar(313, 0.0);s.store_scalar(312, 0.0);s.store_scalar(331, 0.0);s.store_scalar(330, 0.0);s.store_scalar(1039, 0.0);s.store_scalar(446, 0.0);s.store_scalar(576, 0.0);s.b[1057] = (p.p60 == 1.0);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if s.b[1057] {s.store_scalar(114, 1.0);}
        if (!s.b[1057]) {s.store_scalar(114, (-1.0));}
        s.store_scalar(143, (p.p103 * 8.8542e-12));s.store_scalar(144, (p.p1088 * 8.8542e-12));s.store_scalar(165, ((p.p102 * 8.8542e-12) / p.p91));s.store_scalar(145, (p.p103 / p.p102));s.store_scalar(381, (0.916 * 9.11e-31));s.store_scalar(382, (0.19 * 9.11e-31));s.store_scalar(383, (0.19 * 9.11e-31));s.store_scalar(384, (0.417 * 9.11e-31));s.store_scalar(385, 4.0);s.store_scalar(386, 2.0);s.store_scalar(876, (((p.p109 + ((1e-6 * p.p110) / p.p0)) + (p.p111 / p.p5)) + ((p.p112 * 1e-6) / (p.p0 * p.p5))));s.store_scalar(878, (((p.p117 + ((1e-6 * p.p118) / p.p0)) + (p.p119 / p.p5)) + ((p.p120 * 1e-6) / (p.p0 * p.p5))));s.store_scalar(877, (((p.p113 + ((1e-6 * p.p114) / p.p0)) + (p.p115 / p.p5)) + ((p.p116 * 1e-6) / (p.p0 * p.p5))));s.store_scalar(149, (p.p0 + s.v[876]));s.b[1058] = (s.v[149] <= 0.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if s.b[1058] {s.store_scalar(149, p.p0);}
        s.store_powf(168, 149, (-p.p84));s.store_offset_scaled(150, 168, p.p83, s.v[877]);s.store_primal_offset_scaled_ad(151, A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83, s.v[877]);s.store_offset_scaled(152, 168, p.p88, p.p85);s.store_sub_scaled_inputs(153, 149, 1.0, 150, 2.0);s.store_primal_sub_scaled_inputs_mixed_ai(155, A::offset(s.ad_value(149), s.v[878]), 1.0, 151, 2.0);s.store_sub_scaled_inputs(156, 149, 1.0, 152, 2.0);s.store_offset(157, 156, (-p.p86));s.b[1059] = (s.v[153] <= 0.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if s.b[1059] {s.copy_ad(153, 149);}
        s.b[1061] = (s.v[155] <= 0.0);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if s.b[1061] {s.copy_ad(155, 149);}
        s.b[1063] = (s.v[156] <= 0.0);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if s.b[1063] {s.copy_ad(156, 149);}
        s.b[1065] = (p.p61 != 0.0);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });s.b[1066] = (s.v[157] <= 0.0);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1065] && s.b[1066]) {s.copy_ad(157, 149);}
        s.b[1068] = (p.p62 == 5.0);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if s.b[1068] {s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));}
        if (!s.b[1068]) {s.store_scalar(879, 0.0);s.store_scalar(880, 0.0);}
        s.store_primal_offset(161, 879, p.p43);s.store_primal_add(162, 161, 880);s.b[1069] = (p.p62 == 5.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });s.b[1070] = (s.v[162] <= 0.0);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1069] && s.b[1070]) {s.store_scalar(162, p.p43);}
        s.store_scalar(115, (p.p5 * p.p59));s.store_primal_div_from_scalar(635, 1e-6, 155);s.store_scalar(636, (1.0 / p.p5));s.store_primal_div_from_scalar_scaled_input(637, 1e-6, 155, p.p5);s.b[1072] = (p.p62 == 5.0);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if s.b[1072] {s.store_primal_div_from_scalar(638, 1e-6, 162);s.store_primal_div_scalar_by_product_indices(639, 1e-12, 162, 155, 1.0);}
        if (!s.b[1072]) {s.store_scalar(638, 0.0);s.store_scalar(639, 0.0);}
        s.store_primal_add_scaled_inputs4_offset_indices(640, 635, p.p134, 637, p.p136, 638, 0.0, 639, 0.0, ((p.p133) + ((s.v[636] * p.p135))));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1073] = (p.p95 != 0.0);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if s.b[1073] {s.store_primal_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1074] = (s.v[640] <= 0.0);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if s.b[1074] {s.store_scalar(640, 1e22);}
        s.b[1076] = (p.p62 == 0.0);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });s.b[1077] = (p.p62 == 1.0);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });s.b[1078] = (p.p62 == 2.0);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });s.b[1079] = (p.p62 == 3.0);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });s.b[1080] = (p.p62 == 4.0);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });s.b[1081] = (p.p62 == 5.0);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });s.b[1082] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1076] && s.b[1082]) {s.store_scalar(895, (2.0 * p.p92));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, (p.p92 * p.p3));}
        if (s.b[1076] && (!s.b[1082])) {s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));}
        s.b[1083] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if ((s.b[1077] && (!s.b[1076])) && s.b[1083]) {s.store_scalar(895, ((2.0 * p.p92) + p.p3));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, (p.p92 * p.p3));}
        if ((s.b[1077] && (!s.b[1076])) && (!s.b[1083])) {s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));}
        s.b[1084] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && s.b[1084]) {s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, (p.p92 * p.p3));}
        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && (!s.b[1084])) {s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));}
        if (s.b[1078] && (!(s.b[1076] || s.b[1077]))) {s.store_scalar(896, p.p1803);}
        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {s.store_scalar(895, (3.141592653589793 * p.p2));}
        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));}
        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));s.store_scalar(896, p.p2);}
        if (s.b[1080] && (!(((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]))) {s.store_scalar(895, p.p1801);s.store_scalar(893, p.p1800);s.store_scalar(894, p.p1799);}
        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {s.store_primal_offset_scaled(954, 161, 2.0, ((((p.p40) * (2.0))) + (p.p44)));s.store_primal_offset_scaled(948, 161, p.p40, p.p45);s.copy_ad(895, 954);s.copy_ad(894, 948);}
        s.b[1085] = (p.p56 > 1.0);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1085]) {s.store_primal_offset_scaled(955, 161, 2.0, ((((p.p40) * (2.0))) + (p.p46)));s.store_primal_offset_scaled(949, 161, p.p40, p.p47);s.store_primal_add(895, 954, 955);s.store_primal_add(894, 948, 949);}
        s.b[1086] = (p.p56 > 2.0);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1086]) {s.store_primal_offset_scaled(956, 161, 2.0, ((((p.p40) * (2.0))) + (p.p48)));s.store_primal_offset_scaled(950, 161, p.p40, p.p49);s.store_primal_add_scaled_inputs3_indices(895, 954, 1.0, 955, 1.0, 956, 1.0);s.store_primal_add_scaled_inputs3_indices(894, 948, 1.0, 949, 1.0, 950, 1.0);}
        s.b[1087] = (p.p56 > 3.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1087]) {s.store_primal_offset_scaled(957, 161, 2.0, ((((p.p40) * (2.0))) + (p.p50)));s.store_primal_offset_scaled(951, 161, p.p40, p.p51);s.store_primal_add_scaled_inputs4_indices(895, 954, 1.0, 955, 1.0, 956, 1.0, 957, 1.0);s.store_primal_add_scaled_inputs4_indices(894, 948, 1.0, 949, 1.0, 950, 1.0, 951, 1.0);}
        s.b[1088] = (p.p56 > 4.0);s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1088]) {s.store_primal_offset_scaled(958, 161, 2.0, ((((p.p40) * (2.0))) + (p.p52)));s.store_primal_offset_scaled(952, 161, p.p40, p.p53);s.store_primal_add_mixed_ai(895, A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), 958);s.store_primal_add_mixed_ai(894, A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), 952);}
        s.b[1089] = (p.p56 > 5.0);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1089]) {s.store_primal_offset_scaled(959, 161, 2.0, ((((p.p40) * (2.0))) + (p.p54)));s.store_primal_offset_scaled(953, 161, p.p40, p.p55);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1089]) {s.store_primal_add_mixed_ai(895, A::add(A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), s.ad_value(958)), 959);s.store_primal_add_mixed_ai(894, A::add(A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), s.ad_value(952)), 953);}
        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {s.store_scalar(896, p.p43);s.store_primal_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));}
        s.store_primal_div_scaled_inputs_mixed_ia(898, 893, 2.0, A::div_scaled_inputs(A::square(s.ad_value(895)), s.v[143], s.ad_value(894), 1.0), 1.0);s.store_primal_div_scaled_product_indices(903, 640, 894, (-1.60219e-19), 893, 1.0);s.store_primal_div(163, 893, 895);s.b[1090] = (p.p61 != 0.0);s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if s.b[1090] {s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));}
        s.store_primal_offset(158, 895, (-p.p93));s.store_primal_offset(159, 895, (-p.p94));s.b[1091] = (p.p62 == 5.0);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if s.b[1091] {s.store_primal_offset(160, 158, (-((2.0 * p.p56) * p.p87)));}
        if (!s.b[1091]) {s.copy_ad(160, 158);}
        s.b[1092] = (p.p62 == 5.0);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });s.b[1093] = (p.p61 != 0.0);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });s.b[1094] = (s.v[160] <= 0.0);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if ((s.b[1092] && s.b[1093]) && s.b[1094]) {s.copy_ad(160, 895);}
        s.store_scalar(446, p.p1085);s.store_add_scaled_inputs4_offset_indices(641, 635, p.p138, 637, p.p140, 638, p.p141, 639, p.p142, ((p.p137) + ((s.v[636] * p.p139))));s.store_primal_add_scaled_inputs4_offset_indices(666, 635, p.p189, 637, p.p191, 638, p.p192, 639, p.p193, ((p.p188) + ((s.v[636] * p.p190))));s.store_primal_add_scaled_inputs4_offset_indices(662, 635, p.p201, 637, p.p203, 638, p.p204, 639, p.p205, ((p.p200) + ((s.v[636] * p.p202))));s.store_primal_add_scaled_inputs4_offset_indices(663, 635, p.p207, 637, p.p209, 638, p.p210, 639, p.p211, ((p.p206) + ((s.v[636] * p.p208))));s.store_primal_add_scaled_inputs4_offset_indices(667, 635, p.p219, 637, p.p221, 638, p.p222, 639, p.p223, ((p.p218) + ((s.v[636] * p.p220))));s.store_primal_add_scaled_inputs4_offset_indices(670, 635, p.p225, 637, p.p227, 638, p.p228, 639, p.p229, ((p.p224) + ((s.v[636] * p.p226))));s.store_primal_add_scaled_inputs4_offset_indices(671, 635, p.p231, 637, p.p233, 638, p.p234, 639, p.p235, ((p.p230) + ((s.v[636] * p.p232))));s.store_primal_add_scaled_inputs4_offset_indices(672, 635, p.p237, 637, p.p239, 638, p.p240, 639, p.p241, ((p.p236) + ((s.v[636] * p.p238))));s.store_add_scaled_inputs4_offset_indices(673, 635, p.p243, 637, p.p245, 638, p.p246, 639, p.p247, ((p.p242) + ((s.v[636] * p.p244))));s.store_primal_add_scaled_inputs4_offset_indices(674, 635, p.p249, 637, p.p251, 638, p.p252, 639, p.p253, ((p.p248) + ((s.v[636] * p.p250))));s.store_primal_add_scaled_inputs4_offset_indices(678, 635, p.p267, 637, p.p269, 638, p.p270, 639, p.p271, ((p.p266) + ((s.v[636] * p.p268))));s.store_primal_add_scaled_inputs4_offset_indices(802, 635, p.p273, 637, p.p275, 638, p.p276, 639, p.p277, ((p.p272) + ((s.v[636] * p.p274))));s.store_primal_add_scaled_inputs4_offset_indices(803, 635, p.p279, 637, p.p281, 638, p.p282, 639, p.p283, ((p.p278) + ((s.v[636] * p.p280))));s.store_primal_add_scaled_inputs4_offset_indices(804, 635, p.p285, 637, p.p287, 638, p.p288, 639, p.p289, ((p.p284) + ((s.v[636] * p.p286))));s.store_primal_add_scaled_inputs4_offset_indices(683, 635, p.p297, 637, p.p299, 638, p.p300, 639, p.p301, ((p.p296) + ((s.v[636] * p.p298))));s.store_primal_add_scaled_inputs4_offset_indices(684, 635, p.p303, 637, p.p305, 638, p.p306, 639, p.p307, ((p.p302) + ((s.v[636] * p.p304))));s.store_primal_add_scaled_inputs4_offset_indices(685, 635, p.p309, 637, p.p311, 638, p.p312, 639, p.p313, ((p.p308) + ((s.v[636] * p.p310))));s.store_primal_add_scaled_inputs4_offset_indices(686, 635, p.p315, 637, p.p317, 638, p.p318, 639, p.p319, ((p.p314) + ((s.v[636] * p.p316))));s.store_primal_add_scaled_inputs4_offset_indices(687, 635, p.p321, 637, p.p323, 638, p.p324, 639, p.p325, ((p.p320) + ((s.v[636] * p.p322))));s.store_primal_add_scaled_inputs4_offset_indices(688, 635, p.p327, 637, p.p329, 638, p.p330, 639, p.p331, ((p.p326) + ((s.v[636] * p.p328))));s.store_primal_add_scaled_inputs4_offset_indices(867, 635, p.p333, 637, p.p335, 638, p.p336, 639, p.p337, ((p.p332) + ((s.v[636] * p.p334))));s.store_primal_add_scaled_inputs4_offset_indices(868, 635, p.p339, 637, p.p341, 638, p.p342, 639, p.p343, ((p.p338) + ((s.v[636] * p.p340))));s.store_primal_add_scaled_inputs4_offset_indices(869, 635, p.p345, 637, p.p347, 638, p.p348, 639, p.p349, ((p.p344) + ((s.v[636] * p.p346))));s.store_primal_add_scaled_inputs4_offset_indices(870, 635, p.p351, 637, p.p353, 638, p.p354, 639, p.p355, ((p.p350) + ((s.v[636] * p.p352))));s.store_primal_add_scaled_inputs4_offset_indices(654, 635, p.p404, 637, p.p406, 638, p.p407, 639, p.p408, ((p.p403) + ((s.v[636] * p.p405))));s.store_primal_add_scaled_inputs4_offset_indices(655, 635, p.p410, 637, p.p412, 638, p.p413, 639, p.p414, ((p.p409) + ((s.v[636] * p.p411))));s.store_primal_add_scaled_inputs4_offset_indices(656, 635, p.p416, 637, p.p418, 638, p.p419, 639, p.p420, ((p.p415) + ((s.v[636] * p.p417))));s.store_add_scaled_inputs4_offset_indices(661, 635, p.p422, 637, p.p424, 638, p.p425, 639, p.p426, ((p.p421) + ((s.v[636] * p.p423))));
        s.store_add_scaled_inputs4_offset_indices(679, 635, p.p456, 637, p.p458, 638, p.p459, 639, p.p460, ((p.p455) + ((s.v[636] * p.p457))));s.store_add_scaled_inputs4_offset_indices(698, 635, p.p468, 637, p.p470, 638, p.p471, 639, p.p472, ((p.p467) + ((s.v[636] * p.p469))));s.store_add_scaled_inputs4_offset_indices(702, 635, p.p507, 637, p.p509, 638, p.p510, 639, p.p511, ((p.p506) + ((s.v[636] * p.p508))));s.store_primal_add_scaled_inputs4_offset_indices(881, 635, p.p513, 637, p.p515, 638, p.p516, 639, p.p517, ((p.p512) + ((s.v[636] * p.p514))));s.store_primal_add_scaled_inputs4_offset_indices(694, 635, p.p480, 637, p.p482, 638, p.p483, 639, p.p484, ((p.p479) + ((s.v[636] * p.p481))));s.store_add_scaled_inputs4_offset_indices(695, 635, p.p486, 637, p.p488, 638, p.p489, 639, p.p490, ((p.p485) + ((s.v[636] * p.p487))));s.store_primal_add_scaled_inputs4_offset_indices(696, 635, p.p519, 637, p.p521, 638, p.p522, 639, p.p523, ((p.p518) + ((s.v[636] * p.p520))));s.store_add_scaled_inputs4_offset_indices(697, 635, p.p525, 637, p.p527, 638, p.p528, 639, p.p529, ((p.p524) + ((s.v[636] * p.p526))));s.store_primal_add_scaled_inputs4_offset_indices(657, 635, p.p493, 637, p.p495, 638, p.p496, 639, p.p497, ((p.p492) + ((s.v[636] * p.p494))));s.store_add_scaled_inputs4_offset_indices(790, 635, p.p532, 637, p.p534, 638, p.p535, 639, p.p536, ((p.p531) + ((s.v[636] * p.p533))));s.store_add_scaled_inputs4_offset_indices(700, 635, p.p544, 637, p.p546, 638, p.p547, 639, p.p548, ((p.p543) + ((s.v[636] * p.p545))));s.store_add_scaled_inputs4_offset_indices(704, 635, p.p606, 637, p.p608, 638, p.p609, 639, p.p610, ((p.p605) + ((s.v[636] * p.p607))));s.store_primal_add_scaled_inputs4_offset_indices(707, 635, p.p624, 637, p.p626, 638, p.p627, 639, p.p628, ((p.p623) + ((s.v[636] * p.p625))));s.store_primal_add_scaled_inputs4_offset_indices(703, 635, p.p630, 637, p.p632, 638, p.p633, 639, p.p634, ((p.p629) + ((s.v[636] * p.p631))));s.store_add_scaled_inputs4_offset_indices(807, 635, p.p642, 637, p.p644, 638, p.p645, 639, p.p646, ((p.p641) + ((s.v[636] * p.p643))));s.store_add_scaled_inputs4_offset_indices(811, 635, p.p678, 637, p.p680, 638, p.p681, 639, p.p682, ((p.p677) + ((s.v[636] * p.p679))));s.store_add_scaled_inputs4_offset_indices(812, 635, p.p690, 637, p.p692, 638, p.p693, 639, p.p694, ((p.p689) + ((s.v[636] * p.p691))));s.store_primal_add_scaled_inputs4_offset_indices(814, 635, p.p708, 637, p.p710, 638, p.p711, 639, p.p712, ((p.p707) + ((s.v[636] * p.p709))));s.store_primal_add_scaled_inputs4_offset_indices(325, 635, p.p714, 637, p.p716, 638, p.p717, 639, p.p718, ((p.p713) + ((s.v[636] * p.p715))));s.store_primal_add_scaled_inputs4_offset_indices(326, 635, p.p720, 637, p.p722, 638, p.p723, 639, p.p724, ((p.p719) + ((s.v[636] * p.p721))));s.store_primal_add_scaled_inputs4_offset_indices(328, 635, p.p726, 637, p.p728, 638, p.p729, 639, p.p730, ((p.p725) + ((s.v[636] * p.p727))));s.store_primal_add_scaled_inputs4_offset_indices(329, 635, p.p732, 637, p.p734, 638, p.p735, 639, p.p736, ((p.p731) + ((s.v[636] * p.p733))));s.store_add_scaled_inputs4_offset_indices(792, 635, p.p1027, 637, p.p1029, 638, p.p1030, 639, p.p1031, ((p.p1025) + ((s.v[636] * p.p1028))));s.store_primal_add_scaled_inputs4_offset_indices(793, 635, p.p1039, 637, p.p1041, 638, p.p1042, 639, p.p1043, ((p.p1038) + ((s.v[636] * p.p1040))));s.store_primal_add_scaled_inputs4_offset_indices(794, 635, p.p1045, 637, p.p1047, 638, p.p1048, 639, p.p1049, ((p.p1044) + ((s.v[636] * p.p1046))));s.store_primal_add_scaled_inputs4_offset_indices(798, 635, p.p1051, 637, p.p1053, 638, p.p1054, 639, p.p1055, ((p.p1050) + ((s.v[636] * p.p1052))));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs4_offset_indices(800, 635, p.p1057, 637, p.p1059, 638, p.p1060, 639, p.p1061, ((p.p1056) + ((s.v[636] * p.p1058))));s.store_primal_add_scaled_inputs4_offset_indices(799, 635, p.p1063, 637, p.p1065, 638, p.p1066, 639, p.p1067, ((p.p1062) + ((s.v[636] * p.p1064))));s.store_primal_add_scaled_inputs4_offset_indices(801, 635, p.p1069, 637, p.p1071, 638, p.p1072, 639, p.p1073, ((p.p1068) + ((s.v[636] * p.p1070))));s.store_add_scaled_inputs4_offset_indices(709, 635, p.p926, 637, p.p928, 638, p.p929, 639, p.p930, ((p.p925) + ((s.v[636] * p.p927))));s.store_add_scaled_inputs4_offset_indices(853, 635, p.p932, 637, p.p934, 638, p.p935, 639, p.p936, ((p.p931) + ((s.v[636] * p.p933))));s.store_add_scaled_inputs4_offset_indices(852, 635, p.p938, 637, p.p940, 638, p.p941, 639, p.p942, ((p.p937) + ((s.v[636] * p.p939))));s.store_primal_add_scaled_inputs4_offset_indices(712, 635, p.p950, 637, p.p952, 638, p.p953, 639, p.p954, ((p.p949) + ((s.v[636] * p.p951))));s.store_primal_add_scaled_inputs4_offset_indices(711, 635, p.p944, 637, p.p946, 638, p.p947, 639, p.p948, ((p.p943) + ((s.v[636] * p.p945))));s.store_primal_add_scaled_inputs4_offset_indices(713, 635, p.p956, 637, p.p958, 638, p.p959, 639, p.p960, ((p.p955) + ((s.v[636] * p.p957))));s.store_primal_add_scaled_inputs4_offset_indices(714, 635, p.p986, 637, p.p988, 638, p.p989, 639, p.p990, ((p.p985) + ((s.v[636] * p.p987))));s.store_primal_add_scaled_inputs4_offset_indices(716, 635, p.p992, 637, p.p994, 638, p.p995, 639, p.p996, ((p.p991) + ((s.v[636] * p.p993))));s.store_primal_add_scaled_inputs4_offset_indices(719, 635, p.p1010, 637, p.p1012, 638, p.p1013, 639, p.p1014, ((p.p1009) + ((s.v[636] * p.p1011))));s.store_primal_add_scaled_inputs4_offset_indices(720, 635, p.p1016, 637, p.p1018, 638, p.p1019, 639, p.p1020, ((p.p1015) + ((s.v[636] * p.p1017))));s.store_primal_add_scaled_inputs4_offset_indices(721, 635, p.p1120, 637, p.p1122, 638, p.p1123, 639, p.p1124, ((p.p1119) + ((s.v[636] * p.p1121))));s.store_primal_add_scaled_inputs4_offset_indices(722, 635, p.p1126, 637, p.p1128, 638, p.p1129, 639, p.p1130, ((p.p1125) + ((s.v[636] * p.p1127))));s.store_primal_add_scaled_inputs4_offset_indices(723, 635, p.p1132, 637, p.p1134, 638, p.p1135, 639, p.p1136, ((p.p1131) + ((s.v[636] * p.p1133))));s.store_primal_add_scaled_inputs4_offset_indices(724, 635, p.p1138, 637, p.p1140, 638, p.p1141, 639, p.p1142, ((p.p1137) + ((s.v[636] * p.p1139))));s.store_primal_add_scaled_inputs4_offset_indices(725, 635, p.p1144, 637, p.p1146, 638, p.p1147, 639, p.p1148, ((p.p1143) + ((s.v[636] * p.p1145))));s.store_primal_add_scaled_inputs4_offset_indices(726, 635, p.p1150, 637, p.p1152, 638, p.p1153, 639, p.p1154, ((p.p1149) + ((s.v[636] * p.p1151))));s.store_primal_add_scaled_inputs4_offset_indices(727, 635, p.p1156, 637, p.p1158, 638, p.p1159, 639, p.p1160, ((p.p1155) + ((s.v[636] * p.p1157))));s.store_primal_add_scaled_inputs4_offset_indices(728, 635, p.p1162, 637, p.p1164, 638, p.p1165, 639, p.p1166, ((p.p1161) + ((s.v[636] * p.p1163))));s.store_primal_add_scaled_inputs4_offset_indices(729, 635, p.p1168, 637, p.p1170, 638, p.p1171, 639, p.p1172, ((p.p1167) + ((s.v[636] * p.p1169))));s.store_primal_add_scaled_inputs4_offset_indices(730, 635, p.p1174, 637, p.p1176, 638, p.p1177, 639, p.p1178, ((p.p1173) + ((s.v[636] * p.p1175))));s.store_primal_add_scaled_inputs4_offset_indices(731, 635, p.p1180, 637, p.p1182, 638, p.p1183, 639, p.p1184, ((p.p1179) + ((s.v[636] * p.p1181))));s.store_primal_add_scaled_inputs4_offset_indices(732, 635, p.p1186, 637, p.p1188, 638, p.p1189, 639, p.p1190, ((p.p1185) + ((s.v[636] * p.p1187))));s.store_primal_add_scaled_inputs4_offset_indices(733, 635, p.p1192, 637, p.p1194, 638, p.p1195, 639, p.p1196, ((p.p1191) + ((s.v[636] * p.p1193))));s.store_primal_add_scaled_inputs4_offset_indices(734, 635, p.p1198, 637, p.p1200, 638, p.p1201, 639, p.p1202, ((p.p1197) + ((s.v[636] * p.p1199))));s.store_primal_add_scaled_inputs4_offset_indices(735, 635, p.p1204, 637, p.p1206, 638, p.p1207, 639, p.p1208, ((p.p1203) + ((s.v[636] * p.p1205))));
        s.store_primal_add_scaled_inputs4_offset_indices(736, 635, p.p1210, 637, p.p1212, 638, p.p1213, 639, p.p1214, ((p.p1209) + ((s.v[636] * p.p1211))));s.store_primal_add_scaled_inputs4_offset_indices(737, 635, p.p1216, 637, p.p1218, 638, p.p1219, 639, p.p1220, ((p.p1215) + ((s.v[636] * p.p1217))));s.store_primal_add_scaled_inputs4_offset_indices(738, 635, p.p1222, 637, p.p1224, 638, p.p1225, 639, p.p1226, ((p.p1221) + ((s.v[636] * p.p1223))));s.store_primal_add_scaled_inputs4_offset_indices(739, 635, p.p1228, 637, p.p1230, 638, p.p1231, 639, p.p1232, ((p.p1227) + ((s.v[636] * p.p1229))));s.store_primal_add_scaled_inputs4_offset_indices(740, 635, p.p1234, 637, p.p1236, 638, p.p1237, 639, p.p1238, ((p.p1233) + ((s.v[636] * p.p1235))));s.store_primal_add_scaled_inputs4_offset_indices(743, 635, p.p1240, 637, p.p1242, 638, p.p1243, 639, p.p1244, ((p.p1239) + ((s.v[636] * p.p1241))));s.store_primal_add_scaled_inputs4_offset_indices(744, 635, p.p1246, 637, p.p1248, 638, p.p1249, 639, p.p1250, ((p.p1245) + ((s.v[636] * p.p1247))));s.store_primal_add_scaled_inputs4_offset_indices(745, 635, p.p1252, 637, p.p1254, 638, p.p1255, 639, p.p1256, ((p.p1251) + ((s.v[636] * p.p1253))));s.store_primal_add_scaled_inputs4_offset_indices(746, 635, p.p1258, 637, p.p1260, 638, p.p1261, 639, p.p1262, ((p.p1257) + ((s.v[636] * p.p1259))));s.store_primal_add_scaled_inputs4_offset_indices(742, 635, p.p1264, 637, p.p1266, 638, p.p1267, 639, p.p1268, ((p.p1263) + ((s.v[636] * p.p1265))));s.store_primal_add_scaled_inputs4_offset_indices(747, 635, p.p1270, 637, p.p1272, 638, p.p1273, 639, p.p1274, ((p.p1269) + ((s.v[636] * p.p1271))));s.store_primal_add_scaled_inputs4_offset_indices(748, 635, p.p1276, 637, p.p1278, 638, p.p1279, 639, p.p1280, ((p.p1275) + ((s.v[636] * p.p1277))));s.store_primal_add_scaled_inputs4_offset_indices(749, 635, p.p1282, 637, p.p1284, 638, p.p1285, 639, p.p1286, ((p.p1281) + ((s.v[636] * p.p1283))));s.store_primal_add_scaled_inputs4_offset_indices(750, 635, p.p1288, 637, p.p1290, 638, p.p1291, 639, p.p1292, ((p.p1287) + ((s.v[636] * p.p1289))));s.store_primal_add_scaled_inputs4_offset_indices(751, 635, p.p1294, 637, p.p1296, 638, p.p1297, 639, p.p1298, ((p.p1293) + ((s.v[636] * p.p1295))));s.store_primal_add_scaled_inputs4_offset_indices(752, 635, p.p1330, 637, p.p1332, 638, p.p1333, 639, p.p1334, ((p.p1329) + ((s.v[636] * p.p1331))));s.store_primal_add_scaled_inputs4_offset_indices(753, 635, p.p1336, 637, p.p1338, 638, p.p1339, 639, p.p1340, ((p.p1335) + ((s.v[636] * p.p1337))));s.store_primal_add_scaled_inputs4_offset_indices(754, 635, p.p1342, 637, p.p1344, 638, p.p1345, 639, p.p1346, ((p.p1341) + ((s.v[636] * p.p1343))));s.store_primal_add_scaled_inputs4_offset_indices(755, 635, p.p1348, 637, p.p1350, 638, p.p1351, 639, p.p1352, ((p.p1347) + ((s.v[636] * p.p1349))));s.store_primal_add_scaled_inputs4_offset_indices(761, 635, p.p1300, 637, p.p1302, 638, p.p1303, 639, p.p1304, ((p.p1299) + ((s.v[636] * p.p1301))));s.store_primal_add_scaled_inputs4_offset_indices(762, 635, p.p1306, 637, p.p1308, 638, p.p1309, 639, p.p1310, ((p.p1305) + ((s.v[636] * p.p1307))));s.store_primal_add_scaled_inputs4_offset_indices(763, 635, p.p1312, 637, p.p1314, 638, p.p1315, 639, p.p1316, ((p.p1311) + ((s.v[636] * p.p1313))));s.store_primal_add_scaled_inputs4_offset_indices(764, 635, p.p1318, 637, p.p1320, 638, p.p1321, 639, p.p1322, ((p.p1317) + ((s.v[636] * p.p1319))));s.store_primal_add_scaled_inputs4_offset_indices(765, 635, p.p1324, 637, p.p1326, 638, p.p1327, 639, p.p1328, ((p.p1323) + ((s.v[636] * p.p1325))));s.store_primal_add_scaled_inputs4_offset_indices(766, 635, p.p1354, 637, p.p1356, 638, p.p1357, 639, p.p1358, ((p.p1353) + ((s.v[636] * p.p1355))));s.store_primal_add_scaled_inputs4_offset_indices(767, 635, p.p1360, 637, p.p1362, 638, p.p1363, 639, p.p1364, ((p.p1359) + ((s.v[636] * p.p1361))));s.store_primal_add_scaled_inputs4_offset_indices(768, 635, p.p1366, 637, p.p1368, 638, p.p1369, 639, p.p1370, ((p.p1365) + ((s.v[636] * p.p1367))));
        s.store_primal_add_scaled_inputs4_offset_indices(769, 635, p.p1372, 637, p.p1374, 638, p.p1375, 639, p.p1376, ((p.p1371) + ((s.v[636] * p.p1373))));s.store_primal_add_scaled_inputs4_offset_indices(775, 635, p.p1445, 637, p.p1447, 638, p.p1448, 639, p.p1449, ((p.p1444) + ((s.v[636] * p.p1446))));s.store_primal_add_scaled_inputs4_offset_indices(776, 635, p.p1451, 637, p.p1453, 638, p.p1454, 639, p.p1455, ((p.p1450) + ((s.v[636] * p.p1452))));s.store_primal_add_scaled_inputs4_offset_indices(777, 635, p.p1463, 637, p.p1465, 638, p.p1466, 639, p.p1467, ((p.p1462) + ((s.v[636] * p.p1464))));s.store_primal_add_scaled_inputs4_offset_indices(778, 635, p.p1469, 637, p.p1471, 638, p.p1472, 639, p.p1473, ((p.p1468) + ((s.v[636] * p.p1470))));s.store_primal_add_scaled_inputs4_offset_indices(779, 635, p.p1457, 637, p.p1459, 638, p.p1460, 639, p.p1461, ((p.p1456) + ((s.v[636] * p.p1458))));s.store_primal_add_scaled_inputs4_offset_indices(780, 635, p.p1475, 637, p.p1477, 638, p.p1478, 639, p.p1479, ((p.p1474) + ((s.v[636] * p.p1476))));s.store_primal_add_scaled_inputs4_offset_indices(781, 635, p.p1481, 637, p.p1483, 638, p.p1484, 639, p.p1485, ((p.p1480) + ((s.v[636] * p.p1482))));s.store_primal_add_scaled_inputs4_offset_indices(782, 635, p.p1487, 637, p.p1489, 638, p.p1490, 639, p.p1491, ((p.p1486) + ((s.v[636] * p.p1488))));s.store_primal_add_scaled_inputs4_offset_indices(783, 635, p.p1493, 637, p.p1495, 638, p.p1496, 639, p.p1497, ((p.p1492) + ((s.v[636] * p.p1494))));s.store_primal_add_scaled_inputs4_offset_indices(784, 635, p.p1499, 637, p.p1501, 638, p.p1502, 639, p.p1503, ((p.p1498) + ((s.v[636] * p.p1500))));s.store_primal_add_scaled_inputs4_offset_indices(785, 635, p.p1505, 637, p.p1507, 638, p.p1508, 639, p.p1509, ((p.p1504) + ((s.v[636] * p.p1506))));s.store_primal_add_scaled_inputs4_offset_indices(786, 635, p.p1511, 637, p.p1513, 638, p.p1514, 639, p.p1515, ((p.p1510) + ((s.v[636] * p.p1512))));s.store_primal_add_scaled_inputs4_offset_indices(787, 635, p.p1517, 637, p.p1519, 638, p.p1520, 639, p.p1521, ((p.p1516) + ((s.v[636] * p.p1518))));s.store_primal_add_scaled_inputs4_offset_indices(788, 635, p.p1523, 637, p.p1525, 638, p.p1526, 639, p.p1527, ((p.p1522) + ((s.v[636] * p.p1524))));s.store_primal_add_scaled_inputs4_offset_indices(789, 635, p.p1763, 637, p.p1765, 638, p.p1766, 639, p.p1767, ((p.p1762) + ((s.v[636] * p.p1764))));s.store_primal_add_scaled_inputs4_offset_indices(643, 635, p.p1531, 637, p.p1533, 638, p.p1534, 639, p.p1535, ((p.p1530) + ((s.v[636] * p.p1532))));s.store_primal_add_scaled_inputs4_offset_indices(642, 635, p.p1537, 637, p.p1539, 638, p.p1540, 639, p.p1541, ((p.p1536) + ((s.v[636] * p.p1538))));s.store_add_scaled_inputs4_offset_indices(644, 635, p.p29, 637, p.p31, 638, p.p32, 639, p.p33, ((p.p28) + ((s.v[636] * p.p30))));s.store_add_scaled_inputs4_offset_indices(645, 635, p.p35, 637, p.p37, 638, p.p38, 639, p.p39, ((p.p34) + ((s.v[636] * p.p36))));s.store_primal_add_scaled_inputs4_offset_indices(648, 635, p.p1548, 637, p.p1550, 638, p.p1551, 639, p.p1552, ((p.p1547) + ((s.v[636] * p.p1549))));s.store_primal_add_scaled_inputs4_offset_indices(649, 635, p.p1554, 637, p.p1556, 638, p.p1557, 639, p.p1558, ((p.p1553) + ((s.v[636] * p.p1555))));s.store_primal_add_scaled_inputs4_offset_indices(650, 635, p.p1560, 637, p.p1562, 638, p.p1563, 639, p.p1564, ((p.p1559) + ((s.v[636] * p.p1561))));s.store_primal_add_scaled_inputs4_offset_indices(651, 635, p.p1566, 637, p.p1568, 638, p.p1569, 639, p.p1570, ((p.p1565) + ((s.v[636] * p.p1567))));s.store_primal_add_scaled_inputs4_offset_indices(652, 635, p.p1572, 637, p.p1574, 638, p.p1575, 639, p.p1576, ((p.p1571) + ((s.v[636] * p.p1573))));s.store_primal_add_scaled_inputs4_offset_indices(653, 635, p.p1578, 637, p.p1580, 638, p.p1581, 639, p.p1582, ((p.p1577) + ((s.v[636] * p.p1579))));s.store_primal_add_scaled_inputs4_offset_indices(865, 635, p.p1657, 637, p.p1659, 638, p.p1660, 639, p.p1661, ((p.p1656) + ((s.v[636] * p.p1658))));
        s.store_primal_add_scaled_inputs4_offset_indices(866, 635, p.p1663, 637, p.p1665, 638, p.p1666, 639, p.p1667, ((p.p1662) + ((s.v[636] * p.p1664))));s.store_primal_add_scaled_inputs4_offset_indices(836, 635, p.p738, 637, p.p740, 638, p.p741, 639, p.p742, ((p.p737) + ((s.v[636] * p.p739))));s.store_primal_add_scaled_inputs4_offset_indices(837, 635, p.p756, 637, p.p758, 638, p.p759, 639, p.p760, ((p.p755) + ((s.v[636] * p.p757))));s.store_primal_add_scaled_inputs4_offset_indices(838, 635, p.p768, 637, p.p770, 638, p.p771, 639, p.p772, ((p.p767) + ((s.v[636] * p.p769))));s.store_primal_add_scaled_inputs4_offset_indices(842, 635, p.p786, 637, p.p788, 638, p.p789, 639, p.p790, ((p.p785) + ((s.v[636] * p.p787))));s.store_primal_add_scaled_inputs4_offset_indices(823, 635, p.p792, 637, p.p794, 638, p.p795, 639, p.p796, ((p.p791) + ((s.v[636] * p.p793))));s.store_primal_add_scaled_inputs4_offset_indices(824, 635, p.p810, 637, p.p812, 638, p.p813, 639, p.p814, ((p.p809) + ((s.v[636] * p.p811))));s.store_primal_add_scaled_inputs4_offset_indices(847, 635, p.p822, 637, p.p824, 638, p.p825, 639, p.p826, ((p.p821) + ((s.v[636] * p.p823))));s.store_primal_add_scaled_inputs4_offset_indices(830, 635, p.p846, 637, p.p848, 638, p.p849, 639, p.p850, ((p.p845) + ((s.v[636] * p.p847))));s.store_primal_add_scaled_inputs4_offset_indices(831, 635, p.p864, 637, p.p866, 638, p.p867, 639, p.p868, ((p.p863) + ((s.v[636] * p.p865))));s.store_primal_add_scaled_inputs4_offset_indices(834, 635, p.p876, 637, p.p878, 638, p.p879, 639, p.p880, ((p.p875) + ((s.v[636] * p.p877))));s.store_primal_add_scaled_inputs4_offset_indices(835, 635, p.p882, 637, p.p884, 638, p.p885, 639, p.p886, ((p.p881) + ((s.v[636] * p.p883))));s.store_primal_add_scaled_inputs4_offset_indices(848, 635, p.p576, 637, p.p578, 638, p.p579, 639, p.p580, ((p.p575) + ((s.v[636] * p.p577))));s.store_primal_add_scaled_inputs4_offset_indices(849, 635, p.p556, 637, p.p558, 638, p.p559, 639, p.p560, ((p.p555) + ((s.v[636] * p.p557))));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs4_offset_indices(850, 635, p.p569, 637, p.p571, 638, p.p572, 639, p.p573, ((p.p568) + ((s.v[636] * p.p570))));s.store_primal_add_scaled_inputs4_offset_indices(854, 635, p.p962, 637, p.p964, 638, p.p965, 639, p.p966, ((p.p961) + ((s.v[636] * p.p963))));s.store_primal_add_scaled_inputs4_offset_indices(855, 635, p.p968, 637, p.p970, 638, p.p971, 639, p.p972, ((p.p967) + ((s.v[636] * p.p969))));s.store_primal_add_scaled_inputs4_offset_indices(856, 635, p.p974, 637, p.p976, 638, p.p977, 639, p.p978, ((p.p973) + ((s.v[636] * p.p975))));s.store_primal_add_scaled_inputs4_offset_indices(857, 635, p.p980, 637, p.p982, 638, p.p983, 639, p.p984, ((p.p979) + ((s.v[636] * p.p981))));s.store_primal_add_scaled_inputs4_offset_indices(858, 635, p.p1742, 637, p.p1744, 638, p.p1745, 639, p.p1746, ((p.p1741) + ((s.v[636] * p.p1743))));s.store_primal_add_scaled_inputs4_offset_indices(859, 635, p.p1751, 637, p.p1753, 638, p.p1754, 639, p.p1755, ((p.p1750) + ((s.v[636] * p.p1752))));s.store_primal_add_scaled_inputs4_offset_indices(860, 635, p.p1757, 637, p.p1759, 638, p.p1760, 639, p.p1761, ((p.p1756) + ((s.v[636] * p.p1758))));s.store_primal_add_scaled_inputs4_offset_indices(862, 635, p.p1769, 637, p.p1771, 638, p.p1772, 639, p.p1773, ((p.p1768) + ((s.v[636] * p.p1770))));s.store_primal_add_scaled_inputs4_offset_indices(863, 635, p.p1775, 637, p.p1777, 638, p.p1778, 639, p.p1779, ((p.p1774) + ((s.v[636] * p.p1776))));s.store_add_scaled_inputs4_offset_indices(681, 635, p.p177, 637, p.p179, 638, p.p180, 639, p.p181, ((p.p176) + ((s.v[636] * p.p178))));s.store_add_scaled_inputs4_offset_indices(682, 635, p.p183, 637, p.p185, 638, p.p186, 639, p.p187, ((p.p182) + ((s.v[636] * p.p184))));s.store_primal_add_scaled_inputs4_offset_indices(574, 635, p.p1690, 637, p.p1692, 638, p.p1693, 639, p.p1694, ((p.p1689) + ((s.v[636] * p.p1691))));s.store_primal_add_scaled_inputs4_offset_indices(576, 635, p.p1702, 637, p.p1704, 638, p.p1705, 639, p.p1706, ((p.p1701) + ((s.v[636] * p.p1703))));s.store_primal_add_scaled_inputs4_offset_indices(575, 635, p.p1696, 637, p.p1698, 638, p.p1699, 639, p.p1700, ((p.p1695) + ((s.v[636] * p.p1697))));s.b[1096] = (p.p61 != 0.0);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if s.b[1096] {s.store_primal_add_scaled_inputs4_offset_indices(689, 635, p.p357, 637, p.p359, 638, p.p360, 639, p.p361, ((p.p356) + ((s.v[636] * p.p358))));s.store_primal_add_scaled_inputs4_offset_indices(690, 635, p.p363, 637, p.p365, 638, p.p366, 639, p.p367, ((p.p362) + ((s.v[636] * p.p364))));s.store_primal_add_scaled_inputs4_offset_indices(691, 635, p.p369, 637, p.p371, 638, p.p372, 639, p.p373, ((p.p368) + ((s.v[636] * p.p370))));s.store_primal_add_scaled_inputs4_offset_indices(809, 635, p.p660, 637, p.p662, 638, p.p663, 639, p.p664, ((p.p659) + ((s.v[636] * p.p661))));s.store_primal_add_scaled_inputs4_offset_indices(828, 635, p.p828, 637, p.p830, 638, p.p831, 639, p.p832, ((p.p827) + ((s.v[636] * p.p829))));}
        s.b[1097] = (p.p61 == 2.0);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1096] && s.b[1097]) {s.store_primal_add_scaled_inputs4_offset_indices(871, 635, p.p387, 637, p.p389, 638, p.p390, 639, p.p391, ((p.p386) + ((s.v[636] * p.p388))));s.store_primal_add_scaled_inputs4_offset_indices(872, 635, p.p393, 637, p.p395, 638, p.p396, 639, p.p397, ((p.p392) + ((s.v[636] * p.p394))));s.store_primal_add_scaled_inputs4_offset_indices(692, 635, p.p375, 637, p.p377, 638, p.p378, 639, p.p379, ((p.p374) + ((s.v[636] * p.p376))));s.store_primal_add_scaled_inputs4_offset_indices(693, 635, p.p381, 637, p.p383, 638, p.p384, 639, p.p385, ((p.p380) + ((s.v[636] * p.p382))));}
        s.b[1098] = (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1096] && s.b[1098]) {s.store_primal_add_scaled_inputs4_offset_indices(756, 635, p.p1378, 637, p.p1380, 638, p.p1381, 639, p.p1382, ((p.p1377) + ((s.v[636] * p.p1379))));s.store_primal_add_scaled_inputs4_offset_indices(757, 635, p.p1384, 637, p.p1386, 638, p.p1387, 639, p.p1388, ((p.p1383) + ((s.v[636] * p.p1385))));s.store_primal_add_scaled_inputs4_offset_indices(758, 635, p.p1390, 637, p.p1392, 638, p.p1393, 639, p.p1394, ((p.p1389) + ((s.v[636] * p.p1391))));s.store_primal_add_scaled_inputs4_offset_indices(759, 635, p.p1396, 637, p.p1398, 638, p.p1399, 639, p.p1400, ((p.p1395) + ((s.v[636] * p.p1397))));s.store_primal_add_scaled_inputs4_offset_indices(760, 635, p.p1402, 637, p.p1404, 638, p.p1405, 639, p.p1406, ((p.p1401) + ((s.v[636] * p.p1403))));s.store_primal_add_scaled_inputs4_offset_indices(770, 635, p.p1408, 637, p.p1410, 638, p.p1411, 639, p.p1412, ((p.p1407) + ((s.v[636] * p.p1409))));s.store_primal_add_scaled_inputs4_offset_indices(771, 635, p.p1414, 637, p.p1416, 638, p.p1417, 639, p.p1418, ((p.p1413) + ((s.v[636] * p.p1415))));s.store_primal_add_scaled_inputs4_offset_indices(772, 635, p.p1420, 637, p.p1422, 638, p.p1423, 639, p.p1424, ((p.p1419) + ((s.v[636] * p.p1421))));s.store_primal_add_scaled_inputs4_offset_indices(773, 635, p.p1426, 637, p.p1428, 638, p.p1429, 639, p.p1430, ((p.p1425) + ((s.v[636] * p.p1427))));s.store_primal_add_scaled_inputs4_offset_indices(774, 635, p.p1432, 637, p.p1434, 638, p.p1435, 639, p.p1436, ((p.p1431) + ((s.v[636] * p.p1433))));}
        s.b[1099] = (p.p66 != 0.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if s.b[1099] {s.store_primal_add_scaled_inputs4_offset_indices(665, 635, p.p213, 637, p.p215, 638, p.p216, 639, p.p217, ((p.p212) + ((s.v[636] * p.p214))));s.store_primal_add_scaled_inputs4_offset_indices(668, 635, p.p195, 637, p.p197, 638, p.p198, 639, p.p199, ((p.p194) + ((s.v[636] * p.p196))));s.store_primal_add_scaled_inputs4_offset_indices(677, 635, p.p255, 637, p.p257, 638, p.p258, 639, p.p259, ((p.p254) + ((s.v[636] * p.p256))));s.store_add_scaled_inputs4_offset_indices(699, 635, p.p474, 637, p.p476, 638, p.p477, 639, p.p478, ((p.p473) + ((s.v[636] * p.p475))));s.store_add_scaled_inputs4_offset_indices(791, 635, p.p538, 637, p.p540, 638, p.p541, 639, p.p542, ((p.p537) + ((s.v[636] * p.p539))));s.store_add_scaled_inputs4_offset_indices(701, 635, p.p550, 637, p.p552, 638, p.p553, 639, p.p554, ((p.p549) + ((s.v[636] * p.p551))));s.store_primal_add_scaled_inputs4_offset_indices(715, 635, p.p998, 637, p.p1000, 638, p.p1001, 639, p.p1002, ((p.p997) + ((s.v[636] * p.p999))));s.store_primal_add_scaled_inputs4_offset_indices(717, 635, p.p1004, 637, p.p1006, 638, p.p1007, 639, p.p1008, ((p.p1003) + ((s.v[636] * p.p1005))));s.store_add_scaled_inputs4_offset_indices(796, 635, p.p1033, 637, p.p1035, 638, p.p1036, 639, p.p1037, ((p.p1032) + ((s.v[636] * p.p1034))));s.store_primal_add_scaled_inputs4_offset_indices(806, 635, p.p291, 637, p.p293, 638, p.p294, 639, p.p295, ((p.p290) + ((s.v[636] * p.p292))));s.store_primal_add_scaled_inputs4_offset_indices(680, 635, p.p462, 637, p.p464, 638, p.p465, 639, p.p466, ((p.p461) + ((s.v[636] * p.p463))));s.store_primal_add_scaled_inputs4_offset_indices(658, 635, p.p501, 637, p.p503, 638, p.p504, 639, p.p505, ((p.p500) + ((s.v[636] * p.p502))));s.store_add_scaled_inputs4_offset_indices(706, 635, p.p612, 637, p.p614, 638, p.p615, 639, p.p616, ((p.p611) + ((s.v[636] * p.p613))));s.store_add_scaled_inputs4_offset_indices(815, 635, p.p648, 637, p.p650, 638, p.p651, 639, p.p652, ((p.p647) + ((s.v[636] * p.p649))));s.store_primal_add_scaled_inputs4_offset_indices(710, 635, p.p636, 637, p.p638, 638, p.p639, 639, p.p640, ((p.p635) + ((s.v[636] * p.p637))));s.store_add_scaled_inputs4_offset_indices(816, 635, p.p684, 637, p.p686, 638, p.p687, 639, p.p688, ((p.p683) + ((s.v[636] * p.p685))));s.store_add_scaled_inputs4_offset_indices(818, 635, p.p696, 637, p.p698, 638, p.p699, 639, p.p700, ((p.p695) + ((s.v[636] * p.p697))));s.store_primal_add_scaled_inputs4_offset_indices(845, 635, p.p744, 637, p.p746, 638, p.p747, 639, p.p748, ((p.p743) + ((s.v[636] * p.p745))));s.store_primal_add_scaled_inputs4_offset_indices(846, 635, p.p774, 637, p.p776, 638, p.p777, 639, p.p778, ((p.p773) + ((s.v[636] * p.p775))));s.store_primal_add_scaled_inputs4_offset_indices(825, 635, p.p798, 637, p.p800, 638, p.p801, 639, p.p802, ((p.p797) + ((s.v[636] * p.p799))));s.store_primal_add_scaled_inputs4_offset_indices(844, 635, p.p852, 637, p.p854, 638, p.p855, 639, p.p856, ((p.p851) + ((s.v[636] * p.p853))));s.store_primal_add_scaled_inputs4_offset_indices(851, 635, p.p563, 637, p.p565, 638, p.p566, 639, p.p567, ((p.p562) + ((s.v[636] * p.p564))));}
        s.b[1100] = (p.p61 != 0.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if (s.b[1099] && s.b[1100]) {s.store_primal_add_scaled_inputs4_offset_indices(817, 635, p.p666, 637, p.p668, 638, p.p669, 639, p.p670, ((p.p665) + ((s.v[636] * p.p667))));s.store_primal_add_scaled_inputs4_offset_indices(843, 635, p.p834, 637, p.p836, 638, p.p837, 639, p.p838, ((p.p833) + ((s.v[636] * p.p835))));}
        s.b[1101] = (p.p67 == 1.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if s.b[1101] {s.store_add_scaled_inputs4_offset_indices(705, 635, p.p618, 637, p.p620, 638, p.p621, 639, p.p622, ((p.p617) + ((s.v[636] * p.p619))));}
        s.b[1102] = (p.p582 != 0.0);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if (s.b[1101] && s.b[1102]) {s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));}
        if s.b[1101] {s.store_primal_add_scaled_inputs4_offset_indices(808, 635, p.p654, 637, p.p656, 638, p.p657, 639, p.p658, ((p.p653) + ((s.v[636] * p.p655))));s.store_primal_add_scaled_inputs4_offset_indices(813, 635, p.p702, 637, p.p704, 638, p.p705, 639, p.p706, ((p.p701) + ((s.v[636] * p.p703))));s.store_primal_add_scaled_inputs4_offset_indices(839, 635, p.p750, 637, p.p752, 638, p.p753, 639, p.p754, ((p.p749) + ((s.v[636] * p.p751))));s.store_primal_add_scaled_inputs4_offset_indices(840, 635, p.p762, 637, p.p764, 638, p.p765, 639, p.p766, ((p.p761) + ((s.v[636] * p.p763))));s.store_primal_add_scaled_inputs4_offset_indices(841, 635, p.p780, 637, p.p782, 638, p.p783, 639, p.p784, ((p.p779) + ((s.v[636] * p.p781))));s.store_primal_add_scaled_inputs4_offset_indices(826, 635, p.p804, 637, p.p806, 638, p.p807, 639, p.p808, ((p.p803) + ((s.v[636] * p.p805))));s.store_primal_add_scaled_inputs4_offset_indices(827, 635, p.p816, 637, p.p818, 638, p.p819, 639, p.p820, ((p.p815) + ((s.v[636] * p.p817))));s.store_primal_add_scaled_inputs4_offset_indices(832, 635, p.p858, 637, p.p860, 638, p.p861, 639, p.p862, ((p.p857) + ((s.v[636] * p.p859))));s.store_primal_add_scaled_inputs4_offset_indices(833, 635, p.p870, 637, p.p872, 638, p.p873, 639, p.p874, ((p.p869) + ((s.v[636] * p.p871))));}
        s.b[1103] = (p.p61 != 0.0);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (s.b[1101] && s.b[1103]) {s.store_primal_add_scaled_inputs4_offset_indices(810, 635, p.p672, 637, p.p674, 638, p.p675, 639, p.p676, ((p.p671) + ((s.v[636] * p.p673))));s.store_primal_add_scaled_inputs4_offset_indices(829, 635, p.p840, 637, p.p842, 638, p.p843, 639, p.p844, ((p.p839) + ((s.v[636] * p.p841))));}
        if s.b[1101] {s.store_add_scaled_inputs4_offset_indices(675, 635, p.p261, 637, p.p263, 638, p.p264, 639, p.p265, ((p.p260) + ((s.v[636] * p.p262))));}
        s.b[1104] = (p.p161 != 0.0);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if (s.b[1101] && s.b[1104]) {s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1105] = (p.p21 != 0.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if (s.b[1101] && s.b[1105]) {s.store_mul_scale_offset_rhs(705, 705, 153, ((p.p5 - p.p21) * p.p588), 1.0);s.store_mul_scale_offset_rhs(675, 675, 153, ((p.p5 - p.p21) * p.p163), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1107] = (p.p57 == 1.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if s.b[1107] {s.store_primal_add_scaled_inputs4_offset_indices(882, 635, p.p1808, 637, p.p1810, 638, p.p1811, 639, p.p1812, ((p.p1807) + ((s.v[636] * p.p1809))));s.store_primal_add_scaled_inputs4_offset_indices(883, 635, p.p1815, 637, p.p1817, 638, p.p1818, 639, p.p1819, ((p.p1814) + ((s.v[636] * p.p1816))));s.store_primal_add_scaled_inputs4_offset_indices(884, 635, p.p1822, 637, p.p1824, 638, p.p1825, 639, p.p1826, ((p.p1821) + ((s.v[636] * p.p1823))));s.store_primal_add_scaled_inputs4_offset_indices(885, 635, p.p1830, 637, p.p1832, 638, p.p1833, 639, p.p1834, ((p.p1829) + ((s.v[636] * p.p1831))));s.store_primal_add_scaled_inputs4_offset_indices(886, 635, p.p1836, 637, p.p1838, 638, p.p1839, 639, p.p1840, ((p.p1835) + ((s.v[636] * p.p1837))));s.store_primal_add_scaled_inputs4_offset_indices(887, 635, p.p1842, 637, p.p1844, 638, p.p1845, 639, p.p1846, ((p.p1841) + ((s.v[636] * p.p1843))));s.store_primal_add_scaled_inputs4_offset_indices(888, 635, p.p1854, 637, p.p1856, 638, p.p1857, 639, p.p1858, ((p.p1853) + ((s.v[636] * p.p1855))));s.store_primal_add_scaled_inputs4_offset_indices(889, 635, p.p1860, 637, p.p1862, 638, p.p1863, 639, p.p1864, ((p.p1859) + ((s.v[636] * p.p1861))));s.store_primal_add_scaled_inputs4_offset_indices(890, 635, p.p1870, 637, p.p1872, 638, p.p1873, 639, p.p1874, ((p.p1869) + ((s.v[636] * p.p1871))));s.store_primal_add_scaled_inputs4_offset_indices(891, 635, p.p1876, 637, p.p1878, 638, p.p1879, 639, p.p1880, ((p.p1875) + ((s.v[636] * p.p1877))));s.store_primal_add_scaled_inputs4_offset_indices(892, 635, p.p1882, 637, p.p1884, 638, p.p1885, 639, p.p1886, ((p.p1881) + ((s.v[636] * p.p1883))));}
        s.b[1108] = (p.p100 != 0.0);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if s.b[1108] {s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1109] = (p.p158 != 0.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if s.b[1109] {s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1110] = (p.p152 != 0.0);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if s.b[1110] {s.store_primal_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1111] = (p.p154 != 0.0);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if s.b[1111] {s.store_primal_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1112] = (p.p156 != 0.0);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if s.b[1112] {s.store_primal_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1113] = (p.p428 != 0.0);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if s.b[1113] {s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1114] = (p.p432 != 0.0);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if s.b[1114] {s.store_scale(698, 698, (1.0 + ((p.p432 / p.p5) * (if (!((1.0 + (p.p5 / p.p433)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p433)) > 1e-38) { (((1.0 + (p.p5 / p.p433))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1115] = (p.p434 != 0.0);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if s.b[1115] {s.store_scale(699, 699, (1.0 + ((p.p434 / p.p5) * (if (!((1.0 + (p.p5 / p.p435)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p435)) > 1e-38) { (((1.0 + (p.p5 / p.p435))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1116] = (p.p581 != 0.0);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if s.b[1116] {s.store_scale(704, 704, (1.0 + ((p.p581 / p.p5) * (if (!((1.0 + (p.p5 / p.p584)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p584)) > 1e-38) { (((1.0 + (p.p5 / p.p584))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1117] = (p.p583 != 0.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if s.b[1117] {s.store_scale(706, 706, (1.0 + ((p.p583 / p.p5) * (if (!((1.0 + (p.p5 / p.p586)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p586)) > 1e-38) { (((1.0 + (p.p5 / p.p586))) as f64).ln() } else { 0.0 }) }))));}
        s.b[1118] = (p.p21 != 0.0);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if s.b[1118] {s.store_mul_scale_offset_rhs(641, 641, 153, ((p.p5 - p.p21) * p.p99), 1.0);s.store_mul_scale_offset_rhs(673, 673, 153, ((p.p5 - p.p21) * p.p160), 1.0);s.store_mul_scale_offset_rhs(704, 704, 153, ((p.p5 - p.p21) * p.p587), 1.0);}
        s.store_ln(154, 153);s.store_add_scaled_inputs(641, 641, 1.0, 153, p.p98);s.store_add_scaled_inputs(661, 661, 1.0, 153, p.p427);s.b[1119] = (p.p589 > 0.0);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if s.b[1119] {s.store_mul_scale_offset_mixed_ia(704, 704, A::mul(s.ad_value(703), A::exp_scaled_input(s.ad_value(154), (-p.p589))), -1.0, 1.0);}
        if (!s.b[1119]) {s.store_mul_scale_offset_indices(704, 704, 703, -1.0, 1.0);}
        s.store_add_scaled_inputs_mixed_ia(807, 807, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p593))), p.p591);s.store_add_scaled_inputs_mixed_ia(812, 812, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p601))), p.p599);s.store_add_scaled_inputs_mixed_ia(811, 811, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p597))), p.p595);s.b[1120] = (p.p66 != 0.0);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        if s.b[1120] {s.store_add_scaled_inputs_mixed_ia(815, 815, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p594))), p.p592);s.store_add_scaled_inputs_mixed_ia(818, 818, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p602))), p.p600);s.store_add_scaled_inputs_mixed_ia(816, 816, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p598))), p.p596);}
        s.b[1121] = (p.p590 > 0.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1120] && s.b[1121]) {s.store_mul_scale_offset_mixed_ia(706, 706, A::mul(s.ad_value(710), A::exp_scaled_input(s.ad_value(154), (-p.p590))), -1.0, 1.0);}
        if (s.b[1120] && (!s.b[1121])) {s.store_mul_scale_offset_indices(706, 706, 710, -1.0, 1.0);}
        s.b[1122] = (p.p64 == 1.0);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if s.b[1122] {s.store_add_scaled_inputs_mixed_ia(853, 853, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p913))), p.p912);s.store_add_scaled_inputs_mixed_ia(852, 852, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p916))), p.p915);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1122]) {s.store_add_scaled_inputs_mixed_ia(709, 709, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p910))), p.p909);}
        s.store_add_scaled_inputs_mixed_ia(792, 792, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p1023))), p.p1021);s.b[1123] = (p.p66 != 0.0);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if s.b[1123] {s.store_add_scaled_inputs_mixed_ia(796, 796, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p1024)), p.p1022);}
        s.store_add_scaled_inputs_mixed_ia(790, 790, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p445)), p.p444);s.b[1124] = (p.p66 != 0.0);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if s.b[1124] {s.store_add_scaled_inputs_mixed_ia(791, 791, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p447)), p.p446);}
        s.store_add_scaled_inputs_mixed_ia(700, 700, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);s.b[1125] = (p.p66 != 0.0);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if s.b[1125] {s.store_add_scaled_inputs_mixed_ia(701, 701, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);}
        s.store_add_scaled_inputs_mixed_ia(679, 679, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p431))), p.p430);s.store_add_scaled_inputs_mixed_ia(698, 698, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);s.b[1126] = (p.p66 != 0.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if s.b[1126] {s.store_add_scaled_inputs_mixed_ia(699, 699, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);}
        s.store_add_scaled_inputs_mixed_ia(695, 695, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p439))), p.p438);s.store_add_scaled_inputs_mixed_ia(697, 697, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p443))), p.p442);s.store_add_scaled_inputs_mixed_ia(702, 702, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p441))), p.p440);s.store_add_scaled_inputs_mixed_ia(681, 681, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p168))), p.p167);s.store_add_scaled_inputs_mixed_ia(682, 682, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p170))), p.p169);s.b[1127] = ((s.v[655] > 0.0) || (s.v[656] > 0.0));s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if s.b[1127] {s.store_primal_offset_scaled_ad(376, A::limited_exp_scaled_input(A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), (-1.0 / (p.p399))), p.p398, 1.0);s.store_primal_mul_div_scaled_inputs_indices(373, 376, 894, 2.0, 895, 1.0);}
        s.b[1130] = (s.v[576] <= 0.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.store_scalar(576, 0.05);}
        s.b[1135] = (s.v[641] <= 0.0);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if s.b[1135] {s.store_scalar(641, 4.61);}
        s.b[1136] = (p.p61 != 0.0);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });s.b[1137] = (s.v[690] < 1e-6);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (s.b[1136] && s.b[1137]) {s.store_scalar(690, 1e-6);}
        s.b[1138] = (s.v[857] < 0.0);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if s.b[1138] {s.store_scalar(857, 0.01);}
        s.b[1139] = (s.v[576] < 0.0);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if s.b[1139] {s.store_scalar(576, 0.05);}
        s.b[1140] = (s.v[574] < 0.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_scalar(574, p.p1682);}
        s.b[1141] = (s.v[575] < 0.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_scalar(575, 1.2);}
        s.b[1142] = (s.v[644] < 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_scalar(644, 0.0);}
        s.b[1143] = (s.v[645] < 0.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_scalar(645, 0.0);}
        s.b[1144] = (s.v[679] <= 0.0);s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_scalar(679, 85000.0);}
        s.b[1145] = (s.v[698] <= 0.0);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_scalar(698, 85000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1146] = ((p.p66 != 0.0) && (s.v[699] <= 0.0));s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_scalar(699, 85000.0);}
        s.b[1147] = (s.v[670] <= 0.0);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_scalar(670, 0.6);}
        s.b[1148] = (s.v[671] <= 0.0);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_scalar(671, 0.6);}
        s.b[1152] = (s.v[678] <= 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_scalar(678, 1.06);}
        s.b[1153] = (s.v[673] < 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_scalar(673, 0.0);}
        s.b[1154] = (s.v[677] < 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_scalar(677, 0.0);}
        s.b[1155] = (s.v[803] < (-s.v[153]));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_scalar(803, 0.0);}
        s.b[1156] = (s.v[685] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_scalar(685, 0.0);}
        s.b[1157] = (s.v[687] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_scalar(687, 0.0);}
        s.b[1158] = ((p.p61 != 0.0) && (s.v[689] < 0.2));s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_scalar(689, 0.2);}
        s.b[1159] = ((p.p61 != 0.0) && (s.v[689] > 1.2));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_scalar(689, 1.2);}
        s.b[1160] = (s.v[695] < 2.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_scalar(695, 2.0);}
        s.b[1161] = (s.v[697] < 2.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_scalar(697, 2.0);}
        s.b[1162] = (s.v[704] < 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if s.b[1162] {s.store_scalar(704, 0.03);}
        s.b[1163] = (s.v[807] < 0.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_scalar(807, 0.0);}
        s.b[1164] = (s.v[811] < 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_scalar(811, 0.0);}
        s.b[1165] = (s.v[812] < 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_scalar(812, 0.0);}
        s.b[1166] = (s.v[814] < 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_scalar(814, 0.0);}
        s.b[1167] = (s.v[707] < 0.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(707, 0.0);}
        s.b[1168] = (s.v[709] < 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_scalar(709, 0.0);}
        s.b[1169] = (s.v[853] < 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_scalar(853, 0.0);}
        s.b[1170] = (s.v[852] < 0.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {s.store_scalar(852, 0.0);}
        s.b[1171] = (s.v[712] < 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if s.b[1171] {s.store_scalar(712, 0.0);}
        s.b[1172] = (s.v[711] < 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if s.b[1172] {s.store_scalar(711, 0.0);}
        s.b[1175] = (p.p66 != 0.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });s.b[1178] = (s.v[706] < 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1178]) {s.store_scalar(706, 0.0);}
        s.b[1179] = (s.v[815] < 0.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1179]) {s.store_scalar(815, 0.0);}
        s.b[1180] = (s.v[816] < 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1180]) {s.store_scalar(816, 0.0);}
        s.b[1181] = (s.v[818] < 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1181]) {s.store_scalar(818, 0.0);}
        s.b[1183] = (s.v[719] <= 0.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if s.b[1183] {s.store_scalar(719, 1.06);}
        s.b[1184] = (s.v[790] < 2.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if s.b[1184] {s.store_scalar(790, 2.0);}
        s.b[1185] = (p.p66 != 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });s.b[1186] = (s.v[791] < 2.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1185] && s.b[1186]) {s.store_scalar(791, 2.0);}
        s.b[1187] = (s.v[700] < 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if s.b[1187] {s.store_scalar(700, 0.0);}
        s.b[1188] = (s.v[749] < 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if s.b[1188] {s.store_scalar(749, 0.0);}
        s.b[1189] = (s.v[763] < 0.0);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if s.b[1189] {s.store_scalar(763, 0.0);}
        s.b[1190] = (p.p69 != 0.0);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });s.b[1191] = (s.v[726] <= 0.0);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if (s.b[1190] && s.b[1191]) {s.store_scalar(726, 3.0);}
        s.b[1192] = (s.v[731] <= 0.0);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
        if (s.b[1190] && s.b[1192]) {s.store_scalar(731, 1.0);}
        s.b[1193] = (p.p68 != 0.0);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });s.b[1194] = (s.v[742] <= 0.0);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if (s.b[1193] && s.b[1194]) {s.store_scalar(742, 1.0);}
        s.b[1195] = (s.v[736] <= 0.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if (s.b[1193] && s.b[1195]) {s.store_scalar(736, 1.0);}
        s.b[1213] = (s.v[648] < 0.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if s.b[1213] {s.store_scalar(648, 0.0);}
        s.b[1214] = (s.v[649] < 0.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if s.b[1214] {s.store_scalar(649, 0.0);}
        s.b[1215] = (s.v[643] < 0.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if s.b[1215] {s.store_scalar(643, 0.0);}
        s.b[1216] = (s.v[642] < 0.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if s.b[1216] {s.store_scalar(642, 0.0);}
        s.b[1217] = (s.v[650] < 0.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if s.b[1217] {s.store_scalar(650, 0.0);}
        s.b[1218] = (s.v[651] <= 0.02);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if s.b[1218] {s.store_scalar(651, 0.02);}
        s.b[1219] = (s.v[652] <= 0.02);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if s.b[1219] {s.store_scalar(652, 0.02);}
        s.b[1220] = (s.v[653] <= 0.02);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if s.b[1220] {s.store_scalar(653, 0.02);}
        s.b[1221] = (s.v[446] < (-p.p4));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if s.b[1221] {s.store_scalar(446, 0.0);}
        s.b[1222] = (p.p57 == 1.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = ((s.v[882] < 1.0) || (s.v[882] > 3.0));s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1223]) {s.store_scalar(882, 2.0);}
        s.b[1224] = ((s.v[883] < 1.0) || (s.v[883] > 3.0));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1224]) {s.store_scalar(883, 2.6);}
        s.b[1225] = ((s.v[884] < 1.0) || (s.v[884] > 3.0));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1225]) {s.store_scalar(884, 2.6);}
        s.b[1226] = (s.v[885] < 0.0);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1226]) {s.store_scalar(885, 14.0);}
        s.b[1227] = (s.v[886] < 0.0);s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1227]) {s.store_scalar(886, 24.0);}
        s.b[1228] = (s.v[887] < 0.0);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1228]) {s.store_scalar(887, 24.0);}
        s.b[1229] = (s.v[888] < 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1229]) {s.store_scalar(888, 0.139);}
        s.b[1230] = (s.v[889] < 0.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1230]) {s.store_scalar(889, 2.0);}
        s.b[1231] = (s.v[890] < 0.0);s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1231]) {s.store_scalar(890, 11.2);}
        s.b[1232] = (s.v[891] < 0.0);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1232]) {s.store_scalar(891, 8.02);}
        s.b[1233] = (s.v[892] < 0.0);s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1233]) {s.store_scalar(892, 6.18);}
        s.b[1234] = ((p.p74 != 0.0) && (p.p1791 > 0.0));s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });s.b[1235] = (p.p1795 != 0.0);s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if (s.b[1234] && s.b[1235]) {s.store_scalar(169, (p.p1793 * ((p.p59) as f64).powf(p.p1795)));}
        if (s.b[1234] && (!s.b[1235])) {s.store_scalar(169, p.p1793);}
        s.b[1236] = (p.p1794 != 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1234] && s.b[1236]) {s.store_scalar(170, ((p.p1797 * p.p4) * ((s.v[115]) as f64).powf(p.p1794)));}
        if (s.b[1234] && (!s.b[1236])) {s.store_scalar(170, (p.p1797 * p.p4));}
        s.b[1237] = (p.p62 == 5.0);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });s.b[1238] = (p.p1796 != 0.0);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((s.b[1234] && s.b[1237]) && s.b[1238]) {s.store_scalar(171, (((p.p1798 * p.p59) * p.p43) * ((p.p56) as f64).powf(p.p1796)));}
        if ((s.b[1234] && s.b[1237]) && (!s.b[1238])) {s.store_scalar(171, ((p.p1798 * p.p59) * p.p43));}
        if (s.b[1234] && (!s.b[1237])) {s.store_scalar(171, 0.0);}
        if s.b[1234] {s.store_add_scaled_inputs3_indices(634, 169, p.p1792, 170, p.p1792, 171, p.p1792);}
        s.b[1241] = (p.p77 == 0.0);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if s.b[1241] {s.store_scalar(190, (p.p1078 * p.p18));s.store_scalar(191, (p.p1079 * p.p19));}
        s.b[1242] = (p.p1080 > 0.0);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1242]) {s.store_scalar(444, ((p.p4 * p.p92) + ((p.p3 + ((p.p4 - p.p3) * p.p1084)) * p.p1080)));}
        if ((!s.b[1241]) && (!s.b[1242])) {s.store_scalar(444, (p.p4 * (1e-9_f64).max((p.p92 + p.p1080))));}
        if (!s.b[1241]) {s.store_primal_offset(445, 446, p.p4);}
        s.b[1243] = param_given[1083];s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1243]) {s.store_scalar(431, p.p1083);}
        if ((!s.b[1241]) && (!s.b[1243])) {s.store_scalar(429, (if (p.p60 == 1.0) { 1417.0 } else { 470.5 }));}
        s.b[1244] = (p.p60 == 1.0);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if (((!s.b[1241]) && (!s.b[1243])) && s.b[1244]) {s.store_scalar(168, (((p.p97 / 9.68e22)) as f64).powf(0.68));s.store_scalar(169, (3.43e26 / p.p97));s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-52.2), A::offset(s.ad_value(168), 1.0), 1.0), 52.2), A::div_scalar_offset_denominator(43.4, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);}
        if (((!s.b[1241]) && (!s.b[1243])) && (!s.b[1244])) {s.store_scalar(168, (((p.p97 / 2.23e22)) as f64).powf(0.719));s.store_scalar(169, (6.1e26 / p.p97));s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-44.9), A::offset(s.ad_value(168), 1.0), 1.0), 44.9), A::div_scalar_offset_denominator(29.0, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);}
        if ((!s.b[1241]) && (!s.b[1243])) {s.store_div_from_scalar_scaled_input(431, 1.0, 430, (1.60219e-19 * p.p97));}
        if (!s.b[1241]) {s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));s.store_primal_min_with_scalar(432, 444, (1e-18_f64).max((p.p3 * (p.p92 + (0.0_f64).min(p.p1080)))));s.store_scaled_mul_ad(434, A::div(s.ad_value(431), A::tan(s.ad_value(433))), A::add_scaled_inputs3(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), 1.0, A::div_from_scalar(2.0, A::sqrt(s.ad_value(444))), (-1.0), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444)))), 1.0), 1.0 / ((((3.141592653589793) as f64).sqrt() * p.p5)));s.store_primal_offset_scaled(436, 444, p.p5, p.p1092);s.store_primal_offset_scaled(437, 445, p.p5, p.p1093);s.store_sqrt_ad(435, A::div_scaled_inputs(s.ad_value(436), p.p1082, A::mul(s.ad_value(431), s.ad_value(437)), 1.0));s.store_div_from_scalar(438, p.p20, 435);s.store_limited_exp_scaled_input(168, 438, 2.0);}
        s.b[1245] = (p.p1086 == 1.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1245]) {s.store_scaled_mul(439, 431, 435, 1.0 / (p.p1082));s.store_mul_scale_offset_indices(169, 168, 439, 1.0, 1.0);s.store_sub_offset_lhs(170, 169, 1.0, 439);s.store_add_offset_lhs(171, 169, (-1.0), 439);}
        if ((!s.b[1241]) && (!s.b[1245])) {s.store_offset(170, 168, 1.0);s.store_offset(171, 168, (-1.0));}
        if (!s.b[1241]) {s.store_div_scaled_product3_by_product_indices(440, 431, 435, 170, 1.0, 436, 171, 1.0);}
        s.b[1246] = (p.p1080 < (-1e-10));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1246]) {s.store_scalar(441, (p.p1082 / (((-p.p1080) * p.p3) * p.p5)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[1241]) && s.b[1246]) {s.store_div_scaled_product_mixed_aia(442, A::add(s.ad_value(440), s.ad_value(434)), 441, 1.0, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(434), 1.0, s.ad_value(441), 1.0), 1.0);}
        if ((!s.b[1241]) && (!s.b[1246])) {s.store_add(442, 440, 434);}
        if (!s.b[1241]) {s.store_scale(443, 442, (1.0 / (p.p59) * (0.0_f64).max(((((p.p1094 + (p.p1095 * p.p3)) + (p.p1096 * p.p4)) + (p.p1097 * p.p20)) + (p.p1098 * p.p1080)))));s.copy_ad(190, 443);s.copy_ad(191, 443);}
        s.b[1247] = (p.p64 == 0.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });s.b[1248] = (s.v[190] < p.p151);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (s.b[1247] && s.b[1248]) {s.store_scalar(190, 0.0);}
        s.b[1249] = (s.v[191] < p.p151);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if (s.b[1247] && s.b[1249]) {s.store_scalar(191, 0.0);}
        s.b[1250] = (s.v[190] <= p.p151);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((!s.b[1247]) && s.b[1250]) {s.store_scalar(190, p.p151);}
        s.b[1251] = (s.v[191] <= p.p151);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if ((!s.b[1247]) && s.b[1251]) {s.store_scalar(191, p.p151);}
        s.b[1252] = (p.p78 != 1.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });s.b[1253] = param_given[1542];s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (s.b[1252] && s.b[1253]) {s.store_scalar(646, p.p1542);}
        s.b[1254] = (param_given[85] && (p.p85 > 0.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((s.b[1252] && (!s.b[1253])) && s.b[1254]) {s.store_primal_max_from_scalar_ad(646, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(648), 1.0));}
        s.b[1255] = (p.p78 == 3.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && s.b[1255]) {s.store_primal_scale(646, 163, (0.3 * p.p43));}
        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && (!s.b[1255])) {s.store_primal_scale(646, 163, (0.3 * p.p3));}
        s.b[1256] = param_given[1543];s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (s.b[1252] && s.b[1256]) {s.store_scalar(647, p.p1543);}
        s.b[1257] = (param_given[85] && (p.p85 > 0.0));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if ((s.b[1252] && (!s.b[1256])) && s.b[1257]) {s.store_primal_max_from_scalar_ad(647, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(649), 1.0));}
        s.b[1258] = (p.p78 == 3.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && s.b[1258]) {s.store_primal_scale(647, 163, (0.3 * p.p43));}
        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && (!s.b[1258])) {s.store_primal_scale(647, 163, (0.3 * p.p3));}
        s.b[1259] = (p.p78 == 2.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if s.b[1259] {s.store_scalar(447, (p.p1089 + p.p1090));s.store_scalar(449, (0.5 * (p.p4 - p.p3)));s.store_primal_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));}
        s.b[1260] = (p.p1090 > 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1260]) {s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));}
        if (s.b[1259] && s.b[1260]) {s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));s.store_scaled_add(451, 168, 169, (p.p3 + ((p.p4 - p.p3) * p.p1084)));}
        if (s.b[1259] && (!s.b[1260])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 447, 0.2, (p.p90 * 0.2), 450, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));s.store_primal_scale(928, 926, p.p1087);s.store_primal_min_offset_rhs(929, 450, 447, p.p90);s.store_primal_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));s.store_mul(933, 931, 932);}
        s.b[1261] = (s.v[933] > 80.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1260])) && s.b[1261]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && (!s.b[1260])) && (!s.b[1261])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
    }
}
