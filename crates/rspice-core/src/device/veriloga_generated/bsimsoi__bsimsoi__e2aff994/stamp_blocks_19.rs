#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);s.store_offset(171, 170, 1.0);s.store_scaled_add_offset_sqrt_square_offset(339, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);s.store_div_scaled_inputs_mixed_ia(310, 746, 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);s.store_scale(311, 310, s.v[184]);}
        s.b[1653] = (s.v[781] > 0.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1653]) {s.store_offset_div_scaled_product_indices(360, 781, 80, 1.0, 311, 1.0, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1653])) {s.store_div_from_scalar_sub_from_scalar_ad(360, 1.0, 1.0, A::div_scaled_product(s.ad_value(781), s.ad_value(80), 1.0, s.ad_value(311), 1.0));}
        if (s.b[1620] && (!s.b[1634])) {s.copy_ad(359, 763);s.store_sub(355, 226, 315);s.store_add_scaled_inputs(362, 80, 1.0, 269, 2.0);}
        s.b[1654] = (s.v[359] > 0.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1654]) {s.store_div_add_scaled_inputs_rhs_indices(170, 362, 312, 1.0, 362, 1.0);s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);s.store_div_from_scalar(172, 1.0, 171);s.store_mul_product3_mixed_iaii(361, 172, A::div(s.ad_value(362), s.ad_value(359)), 170, 360, 1.0);s.store_offset_div(363, 355, 361, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1654])) {s.store_scalar(363, 1.0);}
        s.b[1655] = (s.v[769] <= 0.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1655]) {s.store_scalar(268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1655])) {s.store_div_scaled_inputs_indices(176, 769, ((s.v[184]) as f64).sqrt(), 362, 1.0);s.store_div_from_scalar_offset_input(268, 1.0, 176, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_add(358, 312, 311);}
        s.b[1656] = (s.v[785] > 0.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });s.b[1657] = (p[414] < 0.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && s.b[1657]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p[414], s.ad_value(311), 1.0)), 268, 1.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && (!s.b[1657])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(80), p[414], s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && s.b[1656]) {s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);}
        s.b[1658] = (p[414] < 0.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && s.b[1658]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p[414], s.ad_value(311), 1.0)), 268, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && (!s.b[1658])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(80), p[414], s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) {s.store_offset(364, 168, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul(363, 363, 364);s.store_limited_exp_mul(168, 768, 226);}
        s.b[1659] = (s.v[767] > 0.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1659]) {s.store_scalar(169, (1.0 + (p[433] * s.v[184])));s.store_div_scaled_offset_numerator_mixed_ai(356, A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, 767, 1.0);s.store_mul(356, 356, 268);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1659])) {s.store_scalar(356, 5.540622384e34);}
        if (s.b[1620] && (!s.b[1634])) {s.store_div(171, 355, 356);s.store_offset(167, 171, 1.0);s.store_mul(363, 363, 167);}
        s.b[1660] = (s.v[766] > 0.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });s.b[1661] = (s.v[355] > ((s.v[765] * s.v[300]) / 80.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && s.b[1661]) {s.store_div_scaled_product_indices(167, 765, 300, 1.0, 355, 1.0);s.store_div_scaled_inputs_limited_exp_lhs(357, 167, s.v[184], 766, 1.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && (!s.b[1661])) {s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1660])) {s.store_scalar(357, 5.540622384e34);}
        if (s.b[1620] && (!s.b[1634])) {s.store_offset_div(365, 355, 357, 1.0);s.store_mul(363, 363, 365);}
        s.b[1662] = (s.v[678] < 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1662]) {s.store_div_from_scalar_sub_from_scalar_ad(349, 1.0, 1.0, A::mul(s.ad_value(678), s.ad_value(218)));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1662])) {s.store_offset_mul(349, 678, 218, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul(167, 80, 349);s.store_div_scaled_value_offset_denominator(350, s.ad_value(167), 100.0, s.ad_value(167), 100.0, 1.0);s.store_scalar(352, (1.0 / p[503]));s.store_ln_ad(167, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(226), s.ad_value(250)), s.ad_value(352)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(315), s.ad_value(250)), s.ad_value(352)), 1.0), 1.0));s.store_scale(353, 167, p[504]);s.store_div_from_scalar_add_ad(354, 1.0, A::offset(s.ad_value(353), 1.0), A::square(s.ad_value(353)));s.store_mul(341, 339, 354);}
        s.b[1663] = (s.v[346] < 0.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1663]) {s.store_div_from_scalar_sub_from_scalar_ad(168, 1.0, 1.0, A::mul(s.ad_value(346), s.ad_value(350)));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1663])) {s.store_offset_mul(168, 346, 350, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_div_rhs(351, 744, 168, 341);s.store_mul_ad_product_lhs_mixed_ai(342, A::square(s.ad_value(351)), 250, 250);}
        s.b[1664] = (p[30] == (-1.0));s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1664]) {s.store_div_scaled_value_offset_denominator(342, s.ad_value(342), 1.0, A::mul(s.ad_value(351), s.ad_value(250)), 1.0, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_scale_offset_mixed_ia(343, 341, A::sqrt(A::scale_offset(s.ad_value(342), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_from_scalar(344, 1.0, 343);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);s.store_add(243, 306, 73);}
        s.b[1665] = (p[33] == 1.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1665]) {s.store_scalar(457, 0.0);s.store_scalar(458, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(228, 169, 170, 0.5);s.store_offset_mul(172, 770, 228, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1665]) {s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 202, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(454, 652, 452, 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), 448, 1.0);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(229, 169, 170, 0.5);s.store_offset_mul(172, 770, 229, 1.0);s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(455, 652, 453, 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), 448, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) {s.store_offset_mul(167, 770, 243, 1.0);s.store_mul_sub_rhs(168, 787, 274, 299);s.store_add_mixed_ai(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);s.store_scaled_add_mixed_ia(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p[2], 0.0, 448);s.copy_ad(455, 453);s.copy_ad(454, 452);s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        s.b[1666] = (p[33] == 2.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) && s.b[1666]) {s.store_mul_add_mixed_iai(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p[2]), 453);s.store_scalar(455, 0.0);s.store_scalar(454, 0.0);s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_add_div_rhs_mixed_ia(167, 330, 333, A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_sub(416, 306, 73);s.store_mul3_lhs(168, 167, 416, 416);s.store_offset(169, 168, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt_square_offset(s.ad_value(169), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(334), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(167, 306, 73);s.store_sub(168, 306, 73);s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);s.store_mul3_lhs(170, 832, 169, 169);s.store_offset(834, 170, 1.0);s.store_div_mixed_ia(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_limited_exp_neg_input(853, 176);s.store_mul(167, 341, 344);s.store_mul_scale_offset_mixed_ia(83, 74, A::mul3_scaled_output(s.ad_value(342), s.ad_value(167), s.ad_value(167), 0.5), 1.0, 1.0);s.store_offset_add_scaled_inputs(83, A::offset(s.ad_value(83), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(83), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_add_scaled_product_indices(81, 80, 1.0, 269, 74, 1.0);s.store_mul_div_scaled_inputs_mixed_aii(84, A::div(s.ad_value(341), s.ad_value(343)), 81, 1.0, 83, 1.0);}
        if s.b[1620] {s.store_mul_ad_product_lhs_mixed_ai(380, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(740), s.ad_value(81), s.ad_value(250), ((p[2] * (s.v[183] / s.v[184])) * s.v[199])), A::div_scaled_product(s.ad_value(354), s.ad_value(344), 1.0, s.ad_value(458), 1.0), s.ad_value(363), 1.0, s.ad_value(334), 1.0), 834, 853);s.store_mul3_lhs(340, 339, 343, 458);s.store_div(337, 740, 340);s.store_scalar(467, 0.0);}
        s.b[1667] = (p[7] > 1.0);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1667]) {s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scale(176, 271, p[1009]);s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scaled_add(467, 167, 468, (p[1008] * p[2]));}
        s.b[1668] = (p[7] == 2.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {s.store_primal_div_from_scalar(466, 1.0, 465);}
        s.b[1669] = (s.v[466] < p[1347]);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_scalar(466, p[1347]);s.store_primal_div_from_scalar(465, 1.0, 466);}
        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {s.store_add(178, 465, 467);s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);}
        if s.b[1620] {s.store_scalar(544, ((s.v[183] / p[1373]) + p[1377]));s.store_scalar(543, ((s.v[183] / p[1373]) + p[1378]));s.store_primal_scale(545, 543, p[74]);s.store_primal_scale(546, 544, p[74]);s.store_mul(593, 637, 590);s.store_div(167, 498, 593);s.store_limited_exp(595, 167);s.store_mul(594, 637, 590);s.store_div(167, 499, 594);s.store_limited_exp(596, 167);s.store_mul_scale_offset_mixed_ai(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, 1.0, (-1.0));}
        s.b[1670] = (s.v[550] == 0.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1670])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);s.store_mul(548, 550, 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1670])) {s.store_mul(167, 545, 548);}
        s.b[1671] = (s.v[551] == 0.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1671])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);s.store_mul(549, 551, 168);s.store_mul(167, 546, 549);}
        s.b[1672] = (s.v[552] == 0.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1672])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(554, 552, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p[925], A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p[925], A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 498, 562);s.store_limited_exp(177, 167);}
        s.b[1673] = ((s.v[558] - s.v[498]) < 0.001);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1672])) && s.b[1673]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((s.b[1620] && (!s.b[1672])) && (!s.b[1673])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (s.b[1620] && (!s.b[1672])) {s.store_mul(170, 545, 554);}
        s.b[1674] = (s.v[553] == 0.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1674])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(555, 553, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p[925], A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p[925], A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 499, 562);s.store_limited_exp(177, 167);}
        s.b[1675] = ((s.v[559] - s.v[499]) < 0.001);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1674])) && s.b[1675]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((s.b[1620] && (!s.b[1674])) && (!s.b[1675])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (s.b[1620] && (!s.b[1674])) {s.store_mul(170, 546, 555);}
        if s.b[1620] {s.store_scalar(602, ((s.v[183] / p[1373]) * p[74]));}
        s.b[1676] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1676])) {s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(585, 587, 167);s.store_mul(578, 598, 167);s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(586, 588, 167);s.store_mul(577, 597, 167);s.store_mul_scale_offset_indices(583, 585, 595, 1.0, (-1.0));}
        s.b[1677] = (s.v[583] < 1e-5);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1677]) {s.store_scalar(583, 0.0);s.store_scalar(591, 1.0);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1677])) {s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));}
        if (s.b[1620] && (!s.b[1676])) {s.store_mul_scale_offset_indices(584, 586, 596, 1.0, (-1.0));}
        s.b[1678] = (s.v[584] < 1e-5);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1678]) {s.store_scalar(584, 0.0);s.store_scalar(592, 1.0);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1678])) {s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));}
        if (s.b[1620] && (!s.b[1676])) {s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p[595]) / p[595]));s.store_limited_exp(603, 167);s.store_sub_from_scalar(169, 1.0, 603);s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p[595])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1676])) {s.store_pow_indices(599, 167, 600);s.store_mul3_lhs(604, 602, 578, 599);s.store_mul(168, 167, 604);s.store_mul3_lhs(604, 602, 577, 599);s.store_mul(168, 167, 604);s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p[920], 1.0);s.store_mul3_lhs(532, 602, 578, 531);s.store_mul_ad_product_lhs_mixed_ia(533, 532, A::offset(s.ad_value(595), (-1.0)), 591);s.store_mul3_lhs(532, 602, 577, 531);s.store_mul_ad_product_lhs_mixed_ia(534, 532, A::offset(s.ad_value(596), (-1.0)), 592);s.store_primal_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);}
        s.b[1679] = (s.v[580] < 1.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1679]) {s.store_scalar(580, 1.0);}
        s.b[1680] = (p[554] == 1.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) {s.store_offset_div_scaled_inputs2_indices(167, 498, 1.0, 499, 1.0, 580, 1.0, 1.0);s.store_add(168, 583, 584);s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);s.store_scaled_add(169, 167, 170, 0.5);s.store_mul(167, 603, 604);}
        s.b[1682] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1620] && (!s.b[1682])) {s.store_mul_scale_offset_indices(174, 569, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(571, 567, 167);s.store_mul_scale_offset_indices(174, 570, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(572, 568, 167);s.store_scale(594, 573, p[925]);}
        s.b[1683] = ((s.v[575] - s.v[498]) < 0.001);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1682])) && s.b[1683]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1683])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if (s.b[1620] && (!s.b[1682])) {s.store_scale(594, 574, p[925]);}
        s.b[1684] = ((s.v[576] - s.v[499]) < 0.001);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1682])) && s.b[1684]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1684])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        s.b[1685] = (p[36] == 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1685]) {s.store_scalar(167, (s.v[200] * p[76]));}
        s.b[1686] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1685]) && s.b[1686]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {s.store_div_scaled_inputs3_indices(168, 204, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1687] = (s.v[894] != 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && s.b[1687]) {s.store_mul_square_lhs(170, 201, 201);s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && (!s.b[1687])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        s.b[1688] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1685]) && s.b[1688]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {s.store_div_scaled_inputs3_indices(168, 203, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1689] = (s.v[898] != 0.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && s.b[1689]) {s.store_mul_square_lhs(170, 202, 202);s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && (!s.b[1689])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        if (s.b[1620] && (!s.b[1685])) {s.store_scalar(167, (s.v[200] * p[76]));s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1690] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1685])) && s.b[1690]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {s.store_div_scaled_inputs3_indices(168, 207, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1691] = (s.v[903] != 0.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && s.b[1691]) {s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(903), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && (!s.b[1691])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        s.b[1692] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1685])) && s.b[1692]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {s.store_div_scaled_inputs3_indices(168, 206, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1693] = (s.v[906] != 0.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && s.b[1693]) {s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(906), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && (!s.b[1693])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {s.store_mul3_ad(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        s.b[1694] = (p[44] == 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });s.b[1696] = (s.v[355] > (s.v[659] / 80.0));s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {s.store_div_scaled_inputs_indices(168, 659, -1.0, 355, 1.0);}
        s.b[1697] = (p[44] == 1.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });s.b[1698] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p[600], (((((-1.0)) * (p[600]))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p[643]) * p[643])), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p[644]) * p[644]), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        s.b[1699] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p[600], (((((-1.0)) * (p[600]))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p[643]) * p[643])), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p[644]) * p[644]), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));s.store_mul_scale_offset_rhs(378, 880, 639, p[666], (((((-1.0)) * (p[666]))) + (1.0)));}
        s.b[1700] = (s.v[211] > 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1700]) {s.store_sub(168, 378, 499);}
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1700])) {s.store_sub(168, 378, 498);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_offset(169, 881, (-1.0));}
        s.b[1701] = (s.v[168] > 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1701]) {s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));}
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1701])) {s.store_scalar(170, 0.0);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_limited_exp(171, 170);}
        if s.b[1620] {s.store_mul(502, 666, 463);s.store_mul(505, 667, 494);s.store_scale(508, 671, (s.v[189] * p[2]));s.store_scalar(503, ((0.1) as f64).powf((-p[913])));}
        s.b[1702] = (p[913] == 1.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1702]) {s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1702])) {s.store_primal_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p[913]) * (1.0 + p[913]))), (-(1.0 / (1.0 - p[913]))), (1.0 / (1.0 - p[913])));}
        if s.b[1620] {s.store_scalar(506, ((0.1) as f64).powf((-p[915])));}
        s.b[1703] = (p[915] == 1.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1703]) {s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1703])) {s.store_primal_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p[915]) * (1.0 + p[915]))), (-(1.0 / (1.0 - p[915]))), (1.0 / (1.0 - p[915])));}
        if s.b[1620] {s.store_scalar(509, ((0.1) as f64).powf((-p[917])));}
        s.b[1704] = (p[917] == 1.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1704]) {s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1704])) {s.store_primal_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p[917]) * (1.0 + p[917]))), (-(1.0 / (1.0 - p[917]))), (1.0 / (1.0 - p[917])));}
        s.b[1705] = (s.v[502] > 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1705]) {s.store_div(168, 498, 672);}
        s.b[1706] = (s.v[168] < 0.9);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1705]) && s.b[1706]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1707] = (p[913] != 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });s.b[1708] = (p[913] == 0.5);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && s.b[1708]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && (!s.b[1708])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[913]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[913])), 0.0);}
        if (((s.b[1620] && s.b[1705]) && s.b[1706]) && (!s.b[1707])) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1705]) && (!s.b[1706])) {s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[913]), (((((-1.0)) * ((5.0 * p[913])))) + ((1.0 + p[913])))));s.store_mul_ad_product_rhs_mixed_ia(521, 672, 502, A::add(s.ad_value(169), s.ad_value(504)));}
        if (s.b[1620] && (!s.b[1705])) {s.store_scalar(521, 0.0);}
        s.b[1709] = (s.v[505] > 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1709]) {s.store_div(168, 498, 673);}
        s.b[1710] = (s.v[168] < 0.9);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1709]) && s.b[1710]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1711] = (p[915] != 1.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });s.b[1712] = (p[915] == 0.5);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) && s.b[1712]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) && (!s.b[1712])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[915]));}
        if (((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[915])), 0.0);}
        if (((s.b[1620] && s.b[1709]) && s.b[1710]) && (!s.b[1711])) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1709]) && (!s.b[1710])) {s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[915]), (((((-1.0)) * ((5.0 * p[915])))) + ((1.0 + p[915])))));s.store_mul_ad_product_rhs_mixed_ia(522, 673, 505, A::add(s.ad_value(169), s.ad_value(507)));}
        if (s.b[1620] && (!s.b[1709])) {s.store_scalar(522, 0.0);}
        s.b[1713] = (s.v[508] > 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1713]) {s.store_div(168, 498, 674);}
        s.b[1714] = (s.v[168] < 0.9);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1713]) && s.b[1714]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1715] = (p[917] != 1.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });s.b[1716] = (p[917] == 0.5);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) && s.b[1716]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) && (!s.b[1716])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[917]));}
        if (((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[917])), 0.0);}
        if (((s.b[1620] && s.b[1713]) && s.b[1714]) && (!s.b[1715])) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1713]) && (!s.b[1714])) {s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[917]), (((((-1.0)) * ((5.0 * p[917])))) + ((1.0 + p[917])))));s.store_mul_ad_product_rhs_mixed_ia(523, 674, 508, A::add(s.ad_value(169), s.ad_value(510)));}
        if (s.b[1620] && (!s.b[1713])) {s.store_scalar(523, 0.0);}
        if s.b[1620] {s.store_scale(524, 533, (p[919] * p[2]));s.store_add_scaled_inputs4_indices(520, 521, 1.0, 522, 1.0, 523, 1.0, 524, 1.0);s.store_mul(511, 669, 464);s.store_mul(514, 670, 495);s.store_scale(517, 668, (s.v[189] * p[2]));s.store_scalar(512, ((0.1) as f64).powf((-p[914])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1717] = (p[914] == 1.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1717]) {s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1717])) {s.store_primal_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p[914]) * (1.0 + p[914]))), (-(1.0 / (1.0 - p[914]))), (1.0 / (1.0 - p[914])));}
        if s.b[1620] {s.store_scalar(515, ((0.1) as f64).powf((-p[916])));}
        s.b[1718] = (p[916] == 1.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1718]) {s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1718])) {s.store_primal_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p[916]) * (1.0 + p[916]))), (-(1.0 / (1.0 - p[916]))), (1.0 / (1.0 - p[916])));}
        if s.b[1620] {s.store_scalar(518, ((0.1) as f64).powf((-p[918])));}
        s.b[1719] = (p[918] == 1.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1719]) {s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1719])) {s.store_primal_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p[918]) * (1.0 + p[918]))), (-(1.0 / (1.0 - p[918]))), (1.0 / (1.0 - p[918])));}
        s.b[1720] = (s.v[511] > 0.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1720]) {s.store_div(168, 499, 675);}
        s.b[1721] = (s.v[168] < 0.9);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1720]) && s.b[1721]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1722] = (p[914] != 1.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });s.b[1723] = (p[914] == 0.5);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && s.b[1723]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && (!s.b[1723])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[914]));}
        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[914])), 0.0);}
        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1720]) && (!s.b[1721])) {s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[914]), (((((-1.0)) * ((5.0 * p[914])))) + ((1.0 + p[914])))));s.store_mul_ad_product_rhs_mixed_ia(526, 675, 511, A::add(s.ad_value(169), s.ad_value(513)));}
        if (s.b[1620] && (!s.b[1720])) {s.store_scalar(526, 0.0);}
        s.b[1724] = (s.v[514] > 0.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1724]) {s.store_div(168, 499, 676);}
        s.b[1725] = (s.v[168] < 0.9);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1724]) && s.b[1725]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1726] = (p[916] != 1.0);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });s.b[1727] = (p[916] == 0.5);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && (!s.b[1727])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[916]));}
        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[916])), 0.0);}
        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1724]) && (!s.b[1725])) {s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[916]), (((((-1.0)) * ((5.0 * p[916])))) + ((1.0 + p[916])))));s.store_mul_ad_product_rhs_mixed_ia(527, 676, 514, A::add(s.ad_value(169), s.ad_value(516)));}
        if (s.b[1620] && (!s.b[1724])) {s.store_scalar(527, 0.0);}
        s.b[1728] = (s.v[517] > 0.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1728]) {s.store_div(168, 499, 677);}
        s.b[1729] = (s.v[168] < 0.9);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1728]) && s.b[1729]) {s.store_sub_from_scalar(500, 1.0, 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1730] = (p[918] != 1.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });s.b[1731] = (p[918] == 0.5);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && s.b[1731]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && (!s.b[1731])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[918]));}
        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[918])), 0.0);}
        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && (!s.b[1730])) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1728]) && (!s.b[1729])) {s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[918]), (((((-1.0)) * ((5.0 * p[918])))) + ((1.0 + p[918])))));s.store_mul_ad_product_rhs_mixed_ia(528, 677, 517, A::add(s.ad_value(169), s.ad_value(519)));}
        if (s.b[1620] && (!s.b[1728])) {s.store_scalar(528, 0.0);}
        if s.b[1620] {s.store_scale(529, 534, (p[919] * p[2]));s.store_add_scaled_inputs4_indices(525, 526, 1.0, 527, 1.0, 528, 1.0, 529, 1.0);}
        s.b[1732] = (s.v[22] <= 0.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1732]) {s.copy_ad(1078, 52);s.store_scalar(1077, 0.0);s.copy_ad(1075, 1078);s.store_scalar(1076, 0.0);}
        if (s.b[1620] && (!s.b[1732])) {s.store_scaled_div(26, 250, 84, 0.5);s.store_square(27, 26);s.store_mul_scale_offset_mixed_ai(366, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(74), s.ad_value(250), (-0.5)), 354, -1.0, 1.0);s.store_add_product3_rhs_mixed_iia(1078, 52, 87, 250, A::add(A::offset(A::mul_scaled_output(s.ad_value(26), s.ad_value(354), 0.3333333333333333), (-1.0)), s.ad_value(354)), 0.5);s.store_scaled_mul(54, 74, 250, 0.16666666666666666);s.store_add_scaled_product_mixed_iia(25, 366, 1.0, 354, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), s.ad_value(26), 1.0), 1.0);s.store_add_scaled_products_mixed_aaia(1077, A::square(s.ad_value(354)), A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), (-1.0)), 0.5, 366, A::offset(s.ad_value(354), 1.0), 0.5);s.store_sub(1075, 1078, 25);s.store_add_scaled_inputs3_indices(1076, 1078, 1.0, 1075, (-1.0), 1077, -1.0);}
        if s.b[1620] {s.store_scaled_add_mixed_ia(246, 1075, A::sqrt_square_offset(s.ad_value(1075), ((0.25 * 0.1) * 0.1)), 0.5);s.store_add(245, 1076, 1077);s.store_add_scaled_inputs(167, 245, 1.0 / (p[230]), 246, (p[231] * 1.0 / (p[230])));s.store_scaled_add_mixed_ia(167, 167, A::sqrt_square_offset(s.ad_value(167), ((4.0 * 0.001) * 0.001)), 0.5);s.store_offset_powf_ad(168, s.ad_value(167), (0.7 * p[229]), 1.0);s.store_div_from_scalar(427, (p[228] * 1.9e-9), 168);s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add_scaled_inputs(s.ad_value(429), (3.9 * 1.0 / (p[110])), s.ad_value(427), 1.0 / (s.v[200])));s.store_mul_scale_offset_mixed_ia(387, 1075, A::div_from_scalar((8.8541878128e-12 * p[110]), s.ad_value(429)), (-(((p[2] * s.v[187]) * s.v[188]) + p[1379])), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1620] {s.store_scale(391, 428, (((p[2] * s.v[187]) * s.v[188]) + p[1379]));}
        s.b[1733] = (s.v[211] > 0.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1733]) {s.store_mul_scale_offset_indices(388, 1076, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(389, 1077, 391, -1.0, 0.0);}
        if (s.b[1620] && (!s.b[1733])) {s.store_mul_scale_offset_indices(388, 1077, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(389, 1076, 391, -1.0, 0.0);}
        if s.b[1620] {s.store_add_scaled_inputs3_indices(390, 387, (-1.0), 388, (-1.0), 389, (-1.0));}
        s.b[1734] = (!param_given[867]);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1734]) {s.store_scalar(788, ((((2.0 * p[110]) * 8.8541878128e-12) / 3.141592653589793) * ((((p[871] * (1.0 + (4e-7 / p[76])))).max(1e-38)) as f64).ln()));}
        if s.b[1620] {s.store_primal_offset(425, 788, p[872]);s.store_primal_offset(426, 788, p[873]);s.store_scalar(561, ((s.v[187] / p[1373]) + p[1378]));s.store_scalar(560, ((s.v[187] / p[1373]) + p[1377]));}
        s.b[1735] = (p[32] == 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1735]) {s.store_mul3_affine_lhs(423, 561, 425, (-p[2]), 0.0, 431);s.store_mul3_affine_lhs(424, 560, 426, (-p[2]), 0.0, 430);}
        if (s.b[1620] && (!s.b[1735])) {s.store_sqrt_offset_ad(167, A::square(A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(419, 431, 0.5, 219, ((-1.0) * 0.5), 167, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(173, 419, A::powf(A::offset(A::powf(A::scale(s.ad_value(419), (-1.0 / (p[893]))), p[894]), 1.0), (1.0 / p[894])));s.store_sqrt_sub_from_scalar_ad(168, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(791), 1.0));s.store_mul_add_scaled_products_rhs_mixed_iiia(423, 561, 425, 431, (-p[2]), 789, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(431), 1.0, s.ad_value(219), (-1.0), s.ad_value(419), -1.0), 1.0, s.ad_value(791), s.ad_value(168), (-1.0), (-0.5)), (-p[2]));s.store_sqrt_offset_ad(167, A::square(A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(420, 430, 0.5, 219, ((-1.0) * 0.5), 167, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(173, 420, A::powf(A::offset(A::powf(A::scale(s.ad_value(420), (-1.0 / (p[891]))), p[892]), 1.0), (1.0 / p[892])));s.store_sqrt_sub_from_scalar_ad(169, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(792), 1.0));s.store_mul_add_scaled_products_rhs_mixed_iiia(424, 560, 426, 430, (-p[2]), 790, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(219), (-1.0), s.ad_value(420), -1.0), 1.0, s.ad_value(792), s.ad_value(169), (-1.0), (-0.5)), (-p[2]));}
        if s.b[1620] {s.store_mul_scaled_voltage(421, 379, (((-p[2]) * s.v[188]) * p[874]), ctx, nodes, Some(9), Some(10));s.store_add_scaled_inputs3_indices(422, 423, (-1.0), 424, (-1.0), 421, (-1.0));s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p[1394]));s.store_primal_offset(1036, 1035, (2.0 * p[1393]));}
        s.b[1736] = (s.v[908] > 0.0);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1736]) {s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);}
        if (s.b[1620] && (!s.b[1736])) {s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product_by_product(s.ad_value(706), s.ad_value(908), -1.0, s.ad_value(182), s.ad_value(182), 1.0), 1e-38));s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);}
        if s.b[1620] {s.store_sub(1032, 235, 215);s.store_scalar(1034, (3.453133e-11 / p[75]));s.store_primal_mul_ad_affine_product_rhs(1037, 909, s.ad_value(1034), A::scale_offset(s.ad_value(1036), ((s.v[187] / p[1373]) * p[2]), p[1382]), p[1388], 0.0);s.store_mul_sub_rhs(1038, 1037, 1032, 1033);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1620] {s.copy_ad(1039, 1038);}
        s.b[1737] = (p[47] != 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1737]) {s.store_scalar(167, (p[1395] * ((((p[871] * (1.0 + (p[74] / p[75])))).max(1e-38)) as f64).ln()));s.store_scalar(168, (p[19] - p[1]));}
        s.b[1738] = (s.v[168] > 0.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1738]) {s.store_mul(1040, 167, 168);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1738])) {s.store_scalar(1040, 0.0);}
        if (s.b[1620] && s.b[1737]) {s.store_scalar(168, (p[20] - p[1]));}
        s.b[1739] = (s.v[168] > 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1739]) {s.store_mul(1041, 167, 168);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1739])) {s.store_scalar(1041, 0.0);}
        if (s.b[1620] && s.b[1737]) {s.store_primal_scale(1042, 1034, p[17]);s.store_scalar(1043, (p[1396] * p[17]));s.store_primal_scale(1044, 1034, p[18]);s.store_scalar(1045, (p[1396] * p[18]));s.store_mul_scale_offset_indices(177, 236, 379, -1.0, 0.0);s.store_mul_scale_offset_indices(178, 237, 379, -1.0, 0.0);}
        s.b[1740] = (p[1396] != 0.0);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1740]) {s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p[1399])));s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(178), (-p[1399]), p[1400])), 1e-38));s.store_mul_scale_offset_mixed_ia(170, 178, A::add(s.ad_value(1044), s.ad_value(1045)), 0.5, 0.0);s.store_add_scaled_product_indices(1047, 170, 1.0, 168, 169, 1.0);s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p[1397])));s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(177), (-p[1397]), p[1398])), 1e-38));s.store_mul_scale_offset_mixed_ia(170, 177, A::add(s.ad_value(1042), s.ad_value(1043)), 0.5, 0.0);s.store_add_scaled_product_indices(1046, 170, 1.0, 168, 169, 1.0);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1740])) {s.store_mul(1046, 1042, 177);s.store_mul(1047, 1044, 178);}
        if (s.b[1620] && s.b[1737]) {s.store_add_scaled_product_indices(1046, 1046, 1.0, 1040, 177, 1.0);s.store_add_scaled_product_indices(1047, 1047, 1.0, 1041, 178, 1.0);}
        if (s.b[1620] && (!s.b[1737])) {s.store_scalar(1046, 0.0);s.store_scalar(1047, 0.0);}
        s.b[1741] = (p[45] == 1.0);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1741]) {s.store_scalar(795, (p[140] + p[25]));s.store_mul(231, 230, 272);s.store_mul(233, 232, 272);s.store_mul(212, 795, 272);s.store_mul(240, 239, 272);s.store_sub(434, 231, 212);s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p[141], s.ad_value(182)), 1e-38));s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p[141]), 1.0 / (s.v[199]));s.copy_ad(294, 436);s.copy_ad(214, 434);s.store_mul(215, 708, 272);s.store_sub(216, 240, 215);s.store_div_from_scalar(295, 1.0, 294);s.store_square(296, 294);s.store_div_from_scalar(297, 1.0, 296);s.copy_ad(251, 435);s.store_scalar(706, p[141]);s.store_div(124, 294, 2);s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);s.store_scale(126, 125, 1e-7);s.store_scalar(127, (5.0 / 4.0));s.store_div_from_scalar(128, 1.0, 124);s.store_square(129, 124);s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));}
        s.b[1742] = (((s.v[216]) as f64).abs() <= s.v[126]);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1742]) {s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        s.b[1743] = (s.v[216] < (-s.v[126]));s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && s.b[1743]) {s.store_neg(132, 216);s.store_mul3_lhs(133, 127, 132, 128);s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(135, A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);s.store_sub_mixed_ai(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);s.store_add(0, 135, 137);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);s.store_add_mixed_ia(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));s.store_limited_exp(141, 140);s.store_sub(142, 132, 140);s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));s.store_scaled_div_mixed_ia(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);s.store_neg_add(131, 140, 145);}
        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && (!s.b[1743])) {s.store_mul_scale_offset_mixed_ia(146, 130, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(147, 216, 128, A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));s.store_limited_exp_neg_input(150, 147);s.store_sub_from_scalar(149, 1.0, 150);s.store_add_scaled_inputs_product_mixed_iiia(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));s.store_limited_exp_neg_input(151, 148);s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);s.store_add_scaled_square_product_mixed_aia(153, A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_83(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && (!s.b[1743])) {s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));s.store_scaled_div_mixed_ia(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);s.store_add(131, 148, 139);}
        s.b[1744] = (((s.v[216]) as f64).abs() < s.v[126]);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1744]) {s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1744])) {s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));s.store_sub_div_rhs_indices(46, 131, 19, 20);}
        if (s.b[1620] && s.b[1741]) {s.store_mul(46, 46, 271);s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);s.store_div_from_scalar(96, 1.0, 95);s.store_add_mixed_ai(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 233);s.store_limited_exp_neg_input(99, 97);s.store_scale(101, 95, 0.001);s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p[74]) * p[74]), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p[294], s.ad_value(271)), 1.0, 3, 216, (-1.0));s.store_add_scaled_product_mixed_iia(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);}
        s.b[1745] = (s.v[4] < s.v[97]);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });s.b[1746] = (s.v[214] < s.v[104]);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });s.b[1747] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && s.b[1747]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
}
