#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul_ad(84, A::div(s.ad_value(81), s.ad_value(83)), A::div(s.ad_value(341), s.ad_value(343)));
        }

        if s.b[1620] {
            s.store_mul_ad_product_lhs(380, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(740), s.ad_value(81), s.ad_value(250), ((p.p2 * (s.v[183] / s.v[184])) * s.v[199])), A::div_scaled_product(s.ad_value(354), s.ad_value(344), 1.0, s.ad_value(458), 1.0), s.ad_value(363), 1.0, s.ad_value(334), 1.0), s.ad_value(834), 853);
            s.store_mul3_lhs(340, 339, 343, 458);
            s.store_div(337, 740, 340);
            s.store_scalar(467, 0.0);
        }

        s.b[1667] = (p.p7 > 1.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1667]) {
            s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scale(176, 271, p.p1009);
            s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));
        }

        s.b[1668] = (p.p7 == 2.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        s.b[1669] = (s.v[466] < p.p1347);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1667]) && s.b[1668]) && s.b[1669]) {
            s.store_scalar(466, p.p1347);
            s.store_div_from_scalar(465, 1.0, 466);
        }

        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {
            s.store_add(178, 465, 467);
            s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);
        }

        if s.b[1620] {
            s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));
            s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));
            s.store_scale(545, 543, p.p74);
            s.store_scale(546, 544, p.p74);
            s.store_mul(593, 637, 590);
            s.store_div(167, 498, 593);
            s.store_limited_exp(595, 167);
            s.store_mul(594, 637, 590);
            s.store_div(167, 499, 594);
            s.store_limited_exp(596, 167);
            s.store_mul_offset_rhs_ad_lhs(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, (-1.0));
        }

        s.b[1670] = (s.v[550] == 0.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1670]) {
            s.store_scalar(535, 0.0);
        }

        if (s.b[1620] && (!s.b[1670])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(548, 550, 168);
            s.store_mul(167, 545, 548);
            s.store_mul_offset_rhs(535, 167, 595, (-1.0));
        }

        s.b[1671] = (s.v[551] == 0.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1671]) {
            s.store_scalar(536, 0.0);
        }

        if (s.b[1620] && (!s.b[1671])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(549, 551, 168);
            s.store_mul(167, 546, 549);
            s.store_mul_offset_rhs(536, 167, 596, (-1.0));
        }

        s.b[1672] = (s.v[552] == 0.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1672]) {
            s.store_scalar(537, 0.0);
        }

        if (s.b[1620] && (!s.b[1672])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(554, 552, 169);
            s.store_mul_scaled_ad_rhs(562, 557, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(563, 564, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0));
            s.store_div(167, 498, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1673] = ((s.v[558] - s.v[498]) < 0.001);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1672])) && s.b[1673]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((s.b[1620] && (!s.b[1672])) && (!s.b[1673])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (s.b[1620] && (!s.b[1672])) {
            s.store_mul(170, 545, 554);
            s.store_mul_add_rhs(537, 170, 177, 178);
        }

        s.b[1674] = (s.v[553] == 0.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1674]) {
            s.store_scalar(538, 0.0);
        }

        if (s.b[1620] && (!s.b[1674])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(555, 553, 169);
            s.store_mul_scaled_ad_rhs(562, 557, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(563, 564, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0));
            s.store_div(167, 499, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1675] = ((s.v[559] - s.v[499]) < 0.001);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1674])) && s.b[1675]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((s.b[1620] && (!s.b[1674])) && (!s.b[1675])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (s.b[1620] && (!s.b[1674])) {
            s.store_mul(170, 546, 555);
            s.store_mul_add_rhs(538, 170, 177, 178);
        }

        if s.b[1620] {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.b[1676] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1676]) {
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(579, 0.0);
        }

        if (s.b[1620] && (!s.b[1676])) {
            s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);
            s.store_limited_exp(167, 174);
            s.store_mul(585, 587, 167);
            s.store_mul(578, 598, 167);
            s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);
            s.store_limited_exp(167, 174);
            s.store_mul(586, 588, 167);
            s.store_mul(577, 597, 167);
            s.store_mul_offset_rhs(583, 585, 595, (-1.0));
        }

        s.b[1677] = (s.v[583] < 1e-5);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1676])) && s.b[1677]) {
            s.store_scalar(583, 0.0);
            s.store_scalar(591, 1.0);
        }

        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1677])) {
            s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));
        }

        if (s.b[1620] && (!s.b[1676])) {
            s.store_mul_offset_rhs(584, 586, 596, (-1.0));
        }

        s.b[1678] = (s.v[584] < 1e-5);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1676])) && s.b[1678]) {
            s.store_scalar(584, 0.0);
            s.store_scalar(592, 1.0);
        }

        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1678])) {
            s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));
        }

        if (s.b[1620] && (!s.b[1676])) {
            s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));
            s.store_limited_exp(603, 167);
            s.store_sub_from_scalar(169, 1.0, 603);
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
            s.store_pow_ad(599, s.ad_value(167), s.ad_value(600));
            s.store_mul3_lhs(604, 602, 578, 599);
            s.store_mul(168, 167, 604);
            s.store_mul_ad_product_lhs(539, s.ad_value(168), A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(604, 602, 577, 599);
            s.store_mul(168, 167, 604);
            s.store_mul_ad_product_lhs(540, s.ad_value(168), A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);
            s.store_mul3_lhs(532, 602, 578, 531);
            s.store_mul_ad_product_lhs(533, s.ad_value(532), A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(532, 602, 577, 531);
            s.store_mul_ad_product_lhs(534, s.ad_value(532), A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);
        }

        s.b[1679] = (s.v[580] < 1.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1676])) && s.b[1679]) {
            s.store_scalar(580, 1.0);
        }

        s.b[1680] = (p.p554 == 1.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1676])) && s.b[1680]) {
            s.store_scalar(579, 0.0);
        }

        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) {
            s.store_offset_ad(167, A::div_scaled_inputs2(s.ad_value(498), 1.0, s.ad_value(499), 1.0, s.ad_value(580), 1.0), 1.0);
            s.store_add(168, 583, 584);
            s.store_sqrt_ad(170, A::add_scaled_inputs(A::square(s.ad_value(167)), 1.0, s.ad_value(168), 4.0));
            s.store_scaled_add(169, 167, 170, 0.5);
        }

        s.b[1681] = (s.v[169] < 0.1);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(605, 10.0);
        }

        if (((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) && (!s.b[1681])) {
            s.store_div_from_scalar(605, 1.0, 169);
        }

        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) {
            s.store_mul(167, 603, 604);
            s.store_mul_ad_affine_product_lhs(579, s.ad_value(167), A::sub(s.ad_value(595), s.ad_value(596)), p.p2, 0.0, 605);
        }

        s.b[1682] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1682]) {
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
        }

        if (s.b[1620] && (!s.b[1682])) {
            s.store_mul_offset_rhs(174, 569, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(571, 567, 167);
            s.store_mul_offset_rhs(174, 570, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(572, 568, 167);
            s.store_scale(594, 573, p.p925);
        }

        s.b[1683] = ((s.v[575] - s.v[498]) < 0.001);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1682])) && s.b[1683]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
            s.store_mul_sub_from_scalar_rhs(541, 170, 1.0, 168);
        }

        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1683])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
            s.store_mul_sub_from_scalar_rhs(541, 170, 1.0, 168);
        }

        if (s.b[1620] && (!s.b[1682])) {
            s.store_scale(594, 574, p.p925);
        }

        s.b[1684] = ((s.v[576] - s.v[499]) < 0.001);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1682])) && s.b[1684]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
            s.store_mul_sub_from_scalar_rhs(542, 170, 1.0, 168);
        }

        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1684])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
            s.store_mul_sub_from_scalar_rhs(542, 170, 1.0, 168);
        }

        if s.b[1620] {
            s.store_add_scaled_inputs4(496, s.ad_value(535), p.p2, s.ad_value(537), p.p2, s.ad_value(539), p.p2, s.ad_value(541), p.p2);
            s.store_add_scaled_inputs4(497, s.ad_value(536), p.p2, s.ad_value(538), p.p2, s.ad_value(540), p.p2, s.ad_value(542), p.p2);
            s.store_scalar(375, 0.0);
            s.store_scalar(374, 0.0);
        }

        s.b[1685] = (p.p36 == 0.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1685]) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        s.b[1686] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1685]) && s.b[1686]) {
            s.store_scalar(173, 0.0);
        }

        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {
            s.store_div_scaled_inputs3(168, s.ad_value(204), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1687] = (s.v[894] != 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && s.b[1687]) {
            s.store_mul_square_lhs(170, 201, 201);
            s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && (!s.b[1687])) {
            s.store_scalar(172, 1.0);
        }

        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        if (s.b[1620] && s.b[1685]) {
            s.copy_ad(374, 173);
        }

        s.b[1688] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1685]) && s.b[1688]) {
            s.store_scalar(173, 0.0);
        }

        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {
            s.store_div_scaled_inputs3(168, s.ad_value(203), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1689] = (s.v[898] != 0.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && s.b[1689]) {
            s.store_mul_square_lhs(170, 202, 202);
            s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_scalar(172, 1.0);
        }

        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        if (s.b[1620] && s.b[1685]) {
            s.copy_ad(375, 173);
        }

        if (s.b[1620] && (!s.b[1685])) {
            s.store_scalar(167, (s.v[200] * p.p76));
            s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);
            s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1690] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1685])) && s.b[1690]) {
            s.store_scalar(173, 0.0);
        }

        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {
            s.store_div_scaled_inputs3(168, s.ad_value(207), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1691] = (s.v[903] != 0.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && s.b[1691]) {
            s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(903), s.ad_value(171)), A::div(s.ad_value(903), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && (!s.b[1691])) {
            s.store_scalar(172, 1.0);
        }

        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {
            s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));
        }

        if (s.b[1620] && (!s.b[1685])) {
            s.copy_ad(374, 173);
        }

        s.b[1692] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1685])) && s.b[1692]) {
            s.store_scalar(173, 0.0);
        }

        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {
            s.store_div_scaled_inputs3(168, s.ad_value(206), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1693] = (s.v[906] != 0.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && s.b[1693]) {
            s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(906), s.ad_value(171)), A::div(s.ad_value(906), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && (!s.b[1693])) {
            s.store_scalar(172, 1.0);
        }

        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {
            s.store_mul3_ad(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));
        }

        if (s.b[1620] && (!s.b[1685])) {
            s.copy_ad(375, 173);
        }

        if s.b[1620] {
            s.store_scaled_mul(1096, 379, 374, p.p2);
            s.store_scaled_mul(1097, 379, 375, p.p2);
        }

        s.b[1694] = (p.p44 == 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        s.b[1695] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1694]) && s.b[1695]) {
            s.store_scalar(373, 0.0);
        }

        s.b[1696] = (s.v[355] > (s.v[659] / 80.0));
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {
            s.store_div_scaled_inputs(168, s.ad_value(659), -1.0, s.ad_value(355), 1.0);
            s.store_div_scaled_product_mixed_aai(373, A::mul3(s.ad_value(865), s.ad_value(355), s.ad_value(380)), A::limited_exp(s.ad_value(168)), 1.0, 365, 1.0);
        }

        if (((s.b[1620] && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) {
            s.store_div_scaled_product3_indices(373, 865, 355, 380, 1.804851387e-35, 365, 1.0);
        }

        s.b[1697] = (p.p44 == 1.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        s.b[1698] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && s.b[1698]) {
            s.store_scalar(373, 0.0);
        }

        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {
            s.store_add_scaled_product_right_ad(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);
            s.store_scale(167, 875, s.v[184]);
            s.store_div_scaled_product_offset_denominator(168, s.ad_value(870), s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0);
            s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269))), ((4.0 * p.p643) * p.p643))), 0.5), 1.0);
            s.store_add(170, 167, 872);
            s.store_scaled_add_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170))), ((4.0 * p.p644) * p.p644))), 0.5);
            s.store_div_from_scalar_offset_ad(170, 1.0, A::mul(s.ad_value(873), s.ad_value(227)), 1.0);
            s.store_mul3_lhs(368, 168, 169, 170);
            s.store_add(369, 370, 368);
            s.store_sub(371, 227, 369);
            s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));
            s.store_sqrt_square_offset(168, 167, 1e-10);
        }

        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {
            let assign27360_ad_e41197: A = A::add(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))), A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::scale_offset(assign27360_ad_e41197, 0.5, (-10.0)));
        }

        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {
            s.store_mul_add_ad_rhs(373, 372, s.ad_value(380), A::mul3(s.ad_value(876), s.ad_value(211), s.ad_value(579)));
        }

        s.b[1699] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1699]) {
            s.store_scalar(373, 0.0);
        }

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {
            s.store_add_scaled_product_right_ad(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);
            s.store_scale(167, 875, s.v[184]);
            s.store_div_scaled_product_offset_denominator(168, s.ad_value(870), s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0);
            s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269))), ((4.0 * p.p643) * p.p643))), 0.5), 1.0);
            s.store_add(170, 167, 872);
            s.store_scaled_add_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170))), ((4.0 * p.p644) * p.p644))), 0.5);
            s.store_div_from_scalar_offset_ad(170, 1.0, A::mul(s.ad_value(873), s.ad_value(227)), 1.0);
            s.store_mul3_lhs(368, 168, 169, 170);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {
            s.store_add(369, 370, 368);
            s.store_sub(371, 227, 369);
            s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));
            s.store_sqrt_square_offset(168, 167, 1e-10);
        }

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {
            let assign27520_ad_e41569: A = A::add(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))), A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::scale_offset(assign27520_ad_e41569, 0.5, (-10.0)));
        }

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {
            s.store_mul(376, 372, 380);
        }

        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {
            s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));
            s.store_mul_ad_rhs(378, 880, A::scale_offset(s.ad_value(639), p.p666, (((((-1.0)) * (p.p666))) + (1.0))));
        }

        s.b[1700] = (s.v[211] > 0.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1700]) {
            s.store_sub(168, 378, 499);
        }

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1700])) {
            s.store_sub(168, 378, 498);
        }

        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {
            s.store_offset(169, 881, (-1.0));
        }

        s.b[1701] = (s.v[168] > 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1701]) {
            s.store_mul_scaled_ad_rhs(170, 879, -1.0, A::pow(s.ad_value(168), s.ad_value(169)));
        }

        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1701])) {
            s.store_scalar(170, 0.0);
        }

        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {
            s.store_limited_exp(171, 170);
            s.store_mul_ad_product_lhs(377, A::mul3(s.ad_value(167), s.ad_value(211), s.ad_value(579)), s.ad_value(168), 171);
            s.store_add(373, 376, 377);
        }

        if s.b[1620] {
            s.store_mul(1095, 373, 379);
            s.store_mul(502, 666, 463);
            s.store_mul(505, 667, 494);
            s.store_scale(508, 671, (s.v[189] * p.p2));
            s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));
        }

        s.b[1702] = (p.p913 == 1.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1702]) {
            s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1702])) {
            s.store_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));
        }

        if s.b[1620] {
            s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));
        }

        s.b[1703] = (p.p915 == 1.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1703]) {
            s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1703])) {
            s.store_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));
        }

        if s.b[1620] {
            s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));
        }

        s.b[1704] = (p.p917 == 1.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1704]) {
            s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1704])) {
            s.store_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));
        }

        s.b[1705] = (s.v[502] > 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1705]) {
            s.store_div(168, 498, 672);
        }

        s.b[1706] = (s.v[168] < 0.9);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1705]) && s.b[1706]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1707] = (p.p913 != 1.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        s.b[1708] = (p.p913 == 0.5);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && s.b[1708]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && (!s.b[1708])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p913));
        }

        if (((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p913)), 0.0);
        }

        if (((s.b[1620] && s.b[1705]) && s.b[1706]) && (!s.b[1707])) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1705]) && (!s.b[1706])) {
            s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p913), (((((-1.0)) * ((5.0 * p.p913)))) + ((1.0 + p.p913)))));
            s.store_mul_ad_product_rhs(521, 672, s.ad_value(502), A::add(s.ad_value(169), s.ad_value(504)));
        }

        if (s.b[1620] && (!s.b[1705])) {
            s.store_scalar(521, 0.0);
        }

        s.b[1709] = (s.v[505] > 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1709]) {
            s.store_div(168, 498, 673);
        }

        s.b[1710] = (s.v[168] < 0.9);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1709]) && s.b[1710]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1711] = (p.p915 != 1.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        s.b[1712] = (p.p915 == 0.5);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) && s.b[1712]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) && (!s.b[1712])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p915));
        }

        if (((s.b[1620] && s.b[1709]) && s.b[1710]) && s.b[1711]) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p915)), 0.0);
        }

        if (((s.b[1620] && s.b[1709]) && s.b[1710]) && (!s.b[1711])) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1709]) && (!s.b[1710])) {
            s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p915), (((((-1.0)) * ((5.0 * p.p915)))) + ((1.0 + p.p915)))));
            s.store_mul_ad_product_rhs(522, 673, s.ad_value(505), A::add(s.ad_value(169), s.ad_value(507)));
        }

        if (s.b[1620] && (!s.b[1709])) {
            s.store_scalar(522, 0.0);
        }

        s.b[1713] = (s.v[508] > 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1713]) {
            s.store_div(168, 498, 674);
        }

        s.b[1714] = (s.v[168] < 0.9);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1713]) && s.b[1714]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1715] = (p.p917 != 1.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        s.b[1716] = (p.p917 == 0.5);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) && s.b[1716]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) && (!s.b[1716])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p917));
        }

        if (((s.b[1620] && s.b[1713]) && s.b[1714]) && s.b[1715]) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p917)), 0.0);
        }

        if (((s.b[1620] && s.b[1713]) && s.b[1714]) && (!s.b[1715])) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1713]) && (!s.b[1714])) {
            s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p917), (((((-1.0)) * ((5.0 * p.p917)))) + ((1.0 + p.p917)))));
            s.store_mul_ad_product_rhs(523, 674, s.ad_value(508), A::add(s.ad_value(169), s.ad_value(510)));
        }

        if (s.b[1620] && (!s.b[1713])) {
            s.store_scalar(523, 0.0);
        }

        if s.b[1620] {
            s.store_scale(524, 533, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(520, s.ad_value(521), 1.0, s.ad_value(522), 1.0, s.ad_value(523), 1.0, s.ad_value(524), 1.0);
            s.store_mul(511, 669, 464);
            s.store_mul(514, 670, 495);
            s.store_scale(517, 668, (s.v[189] * p.p2));
            s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));
        }

        s.b[1717] = (p.p914 == 1.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1717]) {
            s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1717])) {
            s.store_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));
        }

        if s.b[1620] {
            s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));
        }

        s.b[1718] = (p.p916 == 1.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1718]) {
            s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1718])) {
            s.store_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));
        }

        if s.b[1620] {
            s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));
        }

        s.b[1719] = (p.p918 == 1.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1719]) {
            s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));
        }

        if (s.b[1620] && (!s.b[1719])) {
            s.store_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));
        }

        s.b[1720] = (s.v[511] > 0.0);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1720]) {
            s.store_div(168, 499, 675);
        }

        s.b[1721] = (s.v[168] < 0.9);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1720]) && s.b[1721]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1722] = (p.p914 != 1.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        s.b[1723] = (p.p914 == 0.5);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && s.b[1723]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && (!s.b[1723])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p914));
        }

        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);
        }

        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1720]) && (!s.b[1721])) {
            s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));
            s.store_mul_ad_product_rhs(526, 675, s.ad_value(511), A::add(s.ad_value(169), s.ad_value(513)));
        }

        if (s.b[1620] && (!s.b[1720])) {
            s.store_scalar(526, 0.0);
        }

        s.b[1724] = (s.v[514] > 0.0);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1724]) {
            s.store_div(168, 499, 676);
        }

        s.b[1725] = (s.v[168] < 0.9);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1724]) && s.b[1725]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1726] = (p.p916 != 1.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        s.b[1727] = (p.p916 == 0.5);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && (!s.b[1727])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p916));
        }

        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);
        }

        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1724]) && (!s.b[1725])) {
            s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));
            s.store_mul_ad_product_rhs(527, 676, s.ad_value(514), A::add(s.ad_value(169), s.ad_value(516)));
        }

        if (s.b[1620] && (!s.b[1724])) {
            s.store_scalar(527, 0.0);
        }

        s.b[1728] = (s.v[517] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1728]) {
            s.store_div(168, 499, 677);
        }

        s.b[1729] = (s.v[168] < 0.9);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1728]) && s.b[1729]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1730] = (p.p918 != 1.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        s.b[1731] = (p.p918 == 0.5);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && s.b[1731]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && (!s.b[1731])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p918));
        }

        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);
        }

        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && (!s.b[1730])) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1728]) && (!s.b[1729])) {
            s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));
            s.store_mul_ad_product_rhs(528, 677, s.ad_value(517), A::add(s.ad_value(169), s.ad_value(519)));
        }

        if (s.b[1620] && (!s.b[1728])) {
            s.store_scalar(528, 0.0);
        }

        if s.b[1620] {
            s.store_scale(529, 534, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(525, s.ad_value(526), 1.0, s.ad_value(527), 1.0, s.ad_value(528), 1.0, s.ad_value(529), 1.0);
        }

        s.b[1732] = (s.v[22] <= 0.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1732]) {
            s.copy_ad(1078, 52);
            s.store_scalar(1077, 0.0);
            s.copy_ad(1075, 1078);
            s.store_scalar(1076, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1620] && (!s.b[1732])) {
            s.store_scaled_div(26, 250, 84, 0.5);
            s.store_square(27, 26);
            s.store_mul_sub_from_scalar_lhs_ad_rhs(366, 1.0, 354, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(74), s.ad_value(250), (-0.5)));
            s.store_add_ad_rhs(1078, 52, A::mul3_scaled_output(s.ad_value(87), s.ad_value(250), A::add(A::offset(A::mul_scaled_output(s.ad_value(26), s.ad_value(354), 0.3333333333333333), (-1.0)), s.ad_value(354)), 0.5));
            s.store_scaled_mul(54, 74, 250, 0.16666666666666666);
            s.store_add_scaled_product_right_ad(25, 366, 1.0, 354, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), s.ad_value(26), 1.0), 1.0);
            s.store_add_scaled_products_mixed_aaia(1077, A::square(s.ad_value(354)), A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), (-1.0)), 0.5, 366, A::offset(s.ad_value(354), 1.0), 0.5);
            s.store_sub(1075, 1078, 25);
            s.store_add_scaled_inputs3(1076, s.ad_value(1078), 1.0, s.ad_value(1075), (-1.0), s.ad_value(1077), -1.0);
        }

        if s.b[1620] {
            s.store_scaled_add_ad_rhs(246, 1075, A::sqrt(A::offset(A::mul(s.ad_value(1075), s.ad_value(1075)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_add(245, 1076, 1077);
            s.store_add_scaled_inputs(167, 245, 1.0 / (p.p230), 246, (p.p231 * 1.0 / (p.p230)));
            s.store_scaled_add_sqrt_square_offset_rhs(167, 167, 167, ((4.0 * 0.001) * 0.001), 0.5);
            s.store_offset_powf_ad(168, s.ad_value(167), (0.7 * p.p229), 1.0);
            s.store_div_from_scalar(427, (p.p228 * 1.9e-9), 168);
            s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add_scaled_inputs(s.ad_value(429), (3.9 * 1.0 / (p.p110)), s.ad_value(427), 1.0 / (s.v[200])));
            s.store_mul_scale_ad_lhs(387, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), (-(((p.p2 * s.v[187]) * s.v[188]) + p.p1379)), 1075);
            s.store_scale(391, 428, (((p.p2 * s.v[187]) * s.v[188]) + p.p1379));
        }

        s.b[1733] = (s.v[211] > 0.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1733]) {
            s.store_mul_neg_lhs(388, 391, 1076);
            s.store_mul_neg_lhs(389, 391, 1077);
        }

        if (s.b[1620] && (!s.b[1733])) {
            s.store_mul_neg_lhs(388, 391, 1077);
            s.store_mul_neg_lhs(389, 391, 1076);
        }

        if s.b[1620] {
            s.store_neg_ad(390, A::add_scaled_inputs3(s.ad_value(387), 1.0, s.ad_value(388), 1.0, s.ad_value(389), 1.0));
        }

        s.b[1734] = (!param_given[867]);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1734]) {
            s.store_scalar(788, ((((2.0 * p.p110) * 8.8541878128e-12) / 3.141592653589793) * ((((p.p871 * (1.0 + (4e-7 / p.p76)))).max(1e-38)) as f64).ln()));
        }

        if s.b[1620] {
            s.store_offset(425, 788, p.p872);
            s.store_offset(426, 788, p.p873);
            s.store_scalar(561, ((s.v[187] / p.p1373) + p.p1378));
            s.store_scalar(560, ((s.v[187] / p.p1373) + p.p1377));
        }

        s.b[1735] = (p.p32 == 0.0);
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1735]) {
            s.store_mul3_affine_lhs(423, 561, 425, (-p.p2), 0.0, 431);
            s.store_mul3_affine_lhs(424, 560, 426, (-p.p2), 0.0, 430);
        }

        if (s.b[1620] && (!s.b[1735])) {
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(431), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(419, s.ad_value(431), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 419, A::powf(A::offset(A::powf(A::scale(s.ad_value(419), (-1.0 / (p.p893))), p.p894), 1.0), (1.0 / p.p894)));
            s.store_sqrt_sub_from_scalar_ad(168, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(791), 1.0));
            s.store_mul_scaled_ad_rhs(423, 561, (-p.p2), A::add_scaled_products(s.ad_value(425), s.ad_value(431), 1.0, s.ad_value(789), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(431), 1.0, s.ad_value(219), (-1.0), s.ad_value(419), -1.0), 1.0, s.ad_value(791), s.ad_value(168), (-1.0), (-0.5)), 1.0));
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(430), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(420, s.ad_value(430), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 420, A::powf(A::offset(A::powf(A::scale(s.ad_value(420), (-1.0 / (p.p891))), p.p892), 1.0), (1.0 / p.p892)));
            s.store_sqrt_sub_from_scalar_ad(169, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(792), 1.0));
            s.store_mul_scaled_ad_rhs(424, 560, (-p.p2), A::add_scaled_products(s.ad_value(426), s.ad_value(430), 1.0, s.ad_value(790), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(219), (-1.0), s.ad_value(420), -1.0), 1.0, s.ad_value(792), s.ad_value(169), (-1.0), (-0.5)), 1.0));
        }

        if s.b[1620] {
            s.store_mul_scaled_voltage(421, 379, (((-p.p2) * s.v[188]) * p.p874), ctx, nodes, Some(9), Some(10));
            s.store_neg_ad(422, A::add_scaled_inputs3(s.ad_value(423), 1.0, s.ad_value(424), 1.0, s.ad_value(421), 1.0));
            s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p.p1394));
            s.store_offset(1036, 1035, (2.0 * p.p1393));
        }

        s.b[1736] = (s.v[908] > 0.0);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1736]) {
            s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if (s.b[1620] && (!s.b[1736])) {
            s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product_by_product(s.ad_value(706), s.ad_value(908), -1.0, s.ad_value(182), s.ad_value(182), 1.0), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if s.b[1620] {
            s.store_sub(1032, 235, 215);
            s.store_scalar(1034, (3.453133e-11 / p.p75));
            s.store_mul_ad_affine_product_rhs(1037, 909, s.ad_value(1034), A::scale_offset(s.ad_value(1036), ((s.v[187] / p.p1373) * p.p2), p.p1382), p.p1388, 0.0);
            s.store_mul_sub_rhs(1038, 1037, 1032, 1033);
            s.copy_ad(1039, 1038);
        }

        s.b[1737] = (p.p47 != 0.0);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1737]) {
            s.store_scalar(167, (p.p1395 * ((((p.p871 * (1.0 + (p.p74 / p.p75)))).max(1e-38)) as f64).ln()));
            s.store_scalar(168, (p.p19 - p.p1));
        }

        s.b[1738] = (s.v[168] > 0.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1737]) && s.b[1738]) {
            s.store_mul(1040, 167, 168);
        }

        if ((s.b[1620] && s.b[1737]) && (!s.b[1738])) {
            s.store_scalar(1040, 0.0);
        }

        if (s.b[1620] && s.b[1737]) {
            s.store_scalar(168, (p.p20 - p.p1));
        }

        s.b[1739] = (s.v[168] > 0.0);
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1737]) && s.b[1739]) {
            s.store_mul(1041, 167, 168);
        }

        if ((s.b[1620] && s.b[1737]) && (!s.b[1739])) {
            s.store_scalar(1041, 0.0);
        }

        if (s.b[1620] && s.b[1737]) {
            s.store_scale(1042, 1034, p.p17);
            s.store_scalar(1043, (p.p1396 * p.p17));
            s.store_scale(1044, 1034, p.p18);
            s.store_scalar(1045, (p.p1396 * p.p18));
            s.store_mul_neg_lhs(177, 379, 236);
            s.store_mul_neg_lhs(178, 379, 237);
        }

        s.b[1740] = (p.p1396 != 0.0);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1737]) && s.b[1740]) {
            s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p.p1399)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(178), (-p.p1399), p.p1400)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1044), s.ad_value(1045)), 0.5, 178);
            s.store_add_scaled_product_indices(1047, 170, 1.0, 168, 169, 1.0);
            s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p.p1397)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(177), (-p.p1397), p.p1398)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1042), s.ad_value(1043)), 0.5, 177);
            s.store_add_scaled_product_indices(1046, 170, 1.0, 168, 169, 1.0);
        }

        if ((s.b[1620] && s.b[1737]) && (!s.b[1740])) {
            s.store_mul(1046, 1042, 177);
            s.store_mul(1047, 1044, 178);
        }

        if (s.b[1620] && s.b[1737]) {
            s.store_add_scaled_product_indices(1046, 1046, 1.0, 1040, 177, 1.0);
            s.store_add_scaled_product_indices(1047, 1047, 1.0, 1041, 178, 1.0);
        }

        if (s.b[1620] && (!s.b[1737])) {
            s.store_scalar(1046, 0.0);
            s.store_scalar(1047, 0.0);
        }

        s.b[1741] = (p.p45 == 1.0);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1741]) {
            s.store_scalar(795, (p.p140 + p.p25));
            s.store_mul(231, 230, 272);
            s.store_mul(233, 232, 272);
            s.store_mul(212, 795, 272);
            s.store_mul(240, 239, 272);
            s.store_sub(434, 231, 212);
            s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141), 1.0 / (s.v[199]));
            s.copy_ad(294, 436);
            s.copy_ad(214, 434);
            s.store_mul(215, 708, 272);
            s.store_sub(216, 240, 215);
            s.store_div_from_scalar(295, 1.0, 294);
            s.store_square(296, 294);
            s.store_div_from_scalar(297, 1.0, 296);
            s.copy_ad(251, 435);
            s.store_scalar(706, p.p141);
            s.store_div(124, 294, 2);
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
            s.store_scale(126, 125, 1e-7);
            s.store_scalar(127, (5.0 / 4.0));
            s.store_div_from_scalar(128, 1.0, 124);
            s.store_square(129, 124);
            s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));
        }

        s.b[1742] = (((s.v[216]) as f64).abs() <= s.v[126]);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1742]) {
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        s.b[1743] = (s.v[216] < (-s.v[126]));
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && s.b[1743]) {
            s.store_neg(132, 216);
            s.store_mul3_lhs(133, 127, 132, 128);
            s.store_scaled_sub_ad(134, A::offset(s.ad_value(133), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(133), (-6.0), A::offset(s.ad_value(133), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(135, A::sub(s.ad_value(132), s.ad_value(134)), A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);
            s.store_add_scaled_inputs3(137, s.ad_value(132), 2.0, s.ad_value(134), (-2.0), s.ad_value(129), -1.0);
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
            s.store_add(0, 135, 137);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);
            s.store_add_ad_rhs(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));
            s.store_limited_exp(141, 140);
            s.store_sub(142, 132, 140);
            s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);
            s.store_sub_from_scalar_ad(144, 1.0, A::mul_scaled_lhs(s.ad_value(129), 0.5, s.ad_value(141)));
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && s.b[1743]) {
            s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));
            s.store_scaled_div_ad_rhs(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);
            s.store_neg_ad(131, A::add(s.ad_value(140), s.ad_value(145)));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && (!s.b[1743])) {
            s.store_mul_offset_ad_lhs(146, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), (-1.0), 130);
            s.store_mul_ad_product_rhs(147, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
            s.store_limited_exp_neg_input(150, 147);
            s.store_sub_from_scalar(149, 1.0, 150);
            s.store_add_scaled_inputs_product_right_ad(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));
            s.store_limited_exp_neg_input(151, 148);
            s.store_add_scaled_inputs3(152, s.ad_value(216), 2.0, s.ad_value(148), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);
            s.store_add_scaled_products_mixed_aaia(153, A::sub(s.ad_value(216), s.ad_value(148)), A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));
            s.store_sub_from_scalar_ad(154, 1.0, A::mul_scaled_lhs(s.ad_value(129), 0.5, s.ad_value(151)));
            s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));
            s.store_scaled_div_ad_rhs(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);
            s.store_add(131, 148, 139);
        }

        s.b[1744] = (((s.v[216]) as f64).abs() < s.v[126]);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1744]) {
            s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1744])) {
            s.store_add_scaled_inputs3_offset(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, s.ad_value(131), -1.0, (-(-1.0)));
            s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));
            s.store_sub_ad_rhs(46, 131, A::div(s.ad_value(19), s.ad_value(20)));
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_mul(46, 46, 271);
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
            s.store_div_from_scalar(96, 1.0, 95);
            s.store_add_ad_lhs(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 233);
            s.store_limited_exp_neg_input(99, 97);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(271)), 1.0, 3, 216, (-1.0));
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1745] = (s.v[4] < s.v[97]);
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        s.b[1746] = (s.v[214] < s.v[104]);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        s.b[1747] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && s.b[1747]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1748] = (s.v[214] < (-s.v[101]));
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(97), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 272, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_add_scaled_inputs3(106, s.ad_value(105), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0)), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1749] = (s.v[107] < 0.0);
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) && s.b[1749]) {
            s.store_scalar(107, 0.0);
        }

        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {
            s.store_add_scaled_inputs3(49, s.ad_value(97), 1.0, s.ad_value(106), (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 97);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_scaled_ad_rhs(121, 120, 2.0, A::add_scaled_sub_value_product(2.0, A::scale(s.ad_value(48), 2.0), 1.0, s.ad_value(296), s.ad_value(118), (-1.0)));
            s.store_div_scaled_inputs(122, s.ad_value(120), 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1750] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1745])) && s.b[1750]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1751] = (s.v[214] < (-s.v[101]));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && s.b[1751]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && s.b[1751]) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && (!s.b[1751])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(97), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_add_scaled_value_products(6, s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), s.ad_value(727), (-1.0), A::offset(s.ad_value(3), 1.0), s.ad_value(46), 1.0);
        }

        s.b[1752] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1752]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            let assign32090_ad_e48629: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32090_ad_e48628: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32090_ad_e48628
                }
            };
            let assign32090_ad_e48711: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32090_ad_e48710: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32090_ad_e48710
                }
            };
            s.store_sub_ad(169, assign32090_ad_e48629, assign32090_ad_e48711);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            let assign32180_ad_e48919: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign32180_ad_e48919, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            let assign32190_ad_e48988: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign32190_ad_e49047: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign32190_ad_e49076: A = A::sub(A::add_scaled_product(assign32190_ad_e48988, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign32190_ad_e49047, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32190_ad_e49076, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 22);
        }

        s.b[1753] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1753]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            let assign32280_ad_e49304: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32280_ad_e49303: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32280_ad_e49303
                }
            };
            let assign32280_ad_e49386: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32280_ad_e49385: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32280_ad_e49385
                }
            };
            s.store_sub_ad(169, assign32280_ad_e49304, assign32280_ad_e49386);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            let assign32370_ad_e49594: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign32370_ad_e49594, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            let assign32380_ad_e49663: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign32380_ad_e49722: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign32380_ad_e49751: A = A::sub(A::add_scaled_product(assign32380_ad_e49663, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign32380_ad_e49722, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32380_ad_e49751, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 22);
        }

        s.b[1754] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1754]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            let assign32470_ad_e49979: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32470_ad_e49978: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32470_ad_e49978
                }
            };
            let assign32470_ad_e50061: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign32470_ad_e50060: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32470_ad_e50060
                }
            };
            s.store_sub_ad(169, assign32470_ad_e49979, assign32470_ad_e50061);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            let assign32560_ad_e50269: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign32560_ad_e50269, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            let assign32570_ad_e50338: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign32570_ad_e50397: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign32570_ad_e50426: A = A::sub(A::add_scaled_product(assign32570_ad_e50338, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign32570_ad_e50397, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32570_ad_e50426, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_scale(50, 271, 3.912023005);
        }

        s.b[1755] = (s.v[22] <= 0.0);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1755]) {
            s.store_scalar(306, 0.0);
            s.store_sub(51, 214, 22);
            s.store_mul(52, 51, 271);
            s.copy_ad(312, 50);
            s.store_scalar(458, 1.0);
            s.store_scalar(834, 1.0);
            s.store_scalar(853, 1.0);
            s.store_scalar(343, 1.0);
            s.store_scalar(339, 1.0);
            s.store_scalar(363, 1.0);
            s.store_scalar(365, 1.0);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_div_from_scalar_offset_ad(54, 1.0, A::square(s.ad_value(22)), 2.0);
            s.store_mul_square_lhs(55, 22, 54);
            s.store_limited_exp(53, 22);
            s.store_div_from_scalar(56, 1.0, 53);
            s.store_limited_exp_sub(53, 22, 97);
            s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));
            s.store_sub_ad_lhs(58, A::mul3(A::sub(s.ad_value(214), s.ad_value(22)), A::sub(s.ad_value(214), s.ad_value(22)), A::div_from_scalar(1.0, s.ad_value(296))), 57);
            s.store_offset_ad(58, A::add_scaled_inputs(A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(58), (-0.001), A::offset(s.ad_value(58), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(59, 58);
            s.store_mul_sqrt_ad_rhs(61, 294, A::add(s.ad_value(58), s.ad_value(57)));
            s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 271, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);
            s.store_mul3_lhs(247, 59, 294, 271);
            s.copy_ad(76, 56);
            s.copy_ad(78, 57);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_ad_rhs(308, 335, A::add_scaled_inputs(s.ad_value(247), 1.0, s.ad_value(306), s.v[338]));
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(241), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
            s.store_add_scaled_inputs3_offset(273, s.ad_value(298), 0.5, s.ad_value(241), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(298), s.ad_value(241)), (-0.05), A::offset(A::sub(s.ad_value(298), s.ad_value(241)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
        }

        s.b[1756] = (p.p33 == 1.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1756]) {
            s.store_scalar(456, 0.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1756])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        s.b[1757] = (p.p33 == 2.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1756])) && s.b[1757]) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul_ad_lhs(310, A::div_scaled_inputs(s.ad_value(746), 2.0, s.ad_value(740), 1.0), 309);
            s.store_scale(311, 310, s.v[184]);
            s.store_mul_ad_rhs(173, 742, A::add_scaled_inputs(s.ad_value(306), 1.0, s.ad_value(271), 2.0));
        }

        s.b[1758] = (s.v[456] > 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1758]) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
            s.store_mul(167, 324, 456);
            s.store_scale(325, 167, 2.0);
            s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);
            s.store_mul_ad_rhs(327, 173, A::add_scaled_product(s.ad_value(311), 1.0, s.ad_value(173), s.ad_value(167), 2.0));
            s.store_div_scaled_inputs2(312, s.ad_value(326), 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), s.ad_value(325), 1.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1758])) {
            s.store_div_scaled_product_denominator_ad(312, 311, 173, 1.0, A::add(s.ad_value(311), s.ad_value(173)), 1.0);
        }

        s.b[1759] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1759]) {
            s.store_scalar(1019, 1.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1759])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_ad(1019, A::div_scaled_inputs2(s.ad_value(168), p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(271), p.p1350), (-1.0), A::scale_offset(s.ad_value(241), p.p1352, 1.0), 1.0), 1.0);
            s.store_scaled_add_ad(1019, A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1019), (-0.1), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_offset_ad(312, A::add_scaled_inputs(A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(312), (-0.001), A::offset(s.ad_value(312), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_div(312, 312, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 232, 272);
            s.store_add_ad_lhs(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);
            s.store_limited_exp_neg_input(100, 98);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(272), 1.0, s.ad_value(724), s.ad_value(272), 1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(271)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1760] = (s.v[4] < s.v[98]);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        s.b[1761] = (s.v[214] < s.v[104]);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        s.b[1762] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && s.b[1762]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1763] = (s.v[214] < (-s.v[101]));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && (!s.b[1762])) && s.b[1763]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && (!s.b[1762])) && (!s.b[1763])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && (!s.b[1762])) && (!s.b[1763])) {
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(98), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 272, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_add_scaled_inputs3(106, s.ad_value(105), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0)), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1764] = (s.v[107] < 0.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) && s.b[1764]) {
            s.store_scalar(107, 0.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) {
            s.store_add_scaled_inputs3(49, s.ad_value(98), 1.0, s.ad_value(106), (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 98);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_scaled_ad_rhs(121, 120, 2.0, A::add_scaled_sub_value_product(2.0, A::scale(s.ad_value(48), 2.0), 1.0, s.ad_value(296), s.ad_value(118), (-1.0)));
            s.store_div_scaled_inputs(122, s.ad_value(120), 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1765] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && s.b[1765]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1766] = (s.v[214] < (-s.v[101]));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && (!s.b[1765])) && s.b[1766]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && (!s.b[1765])) && (!s.b[1766])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(98), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(272), 1.0, s.ad_value(724), s.ad_value(272), 1.0));
            s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(271)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        s.b[1767] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1767]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            let assign34990_ad_e55610: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign34990_ad_e55609: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign34990_ad_e55609
                }
            };
            let assign34990_ad_e55692: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign34990_ad_e55691: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign34990_ad_e55691
                }
            };
            s.store_sub_ad(169, assign34990_ad_e55610, assign34990_ad_e55692);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            let assign35080_ad_e55927: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign35080_ad_e55927, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            let assign35090_ad_e55999: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign35090_ad_e56058: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign35090_ad_e56087: A = A::sub(A::add_scaled_product(assign35090_ad_e55999, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign35090_ad_e56058, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35090_ad_e56087, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 23);
        }

        s.b[1768] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1768]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            let assign35180_ad_e56339: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35180_ad_e56338: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35180_ad_e56338
                }
            };
            let assign35180_ad_e56421: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35180_ad_e56420: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35180_ad_e56420
                }
            };
            s.store_sub_ad(169, assign35180_ad_e56339, assign35180_ad_e56421);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            let assign35270_ad_e56656: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign35270_ad_e56656, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            let assign35280_ad_e56728: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign35280_ad_e56787: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign35280_ad_e56816: A = A::sub(A::add_scaled_product(assign35280_ad_e56728, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign35280_ad_e56787, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35280_ad_e56816, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 23);
        }

        s.b[1769] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1769]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            let assign35370_ad_e57068: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35370_ad_e57067: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35370_ad_e57067
                }
            };
            let assign35370_ad_e57150: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35370_ad_e57149: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35370_ad_e57149
                }
            };
            s.store_sub_ad(169, assign35370_ad_e57068, assign35370_ad_e57150);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            let assign35460_ad_e57385: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign35460_ad_e57385, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            let assign35470_ad_e57457: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign35470_ad_e57516: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign35470_ad_e57545: A = A::sub(A::add_scaled_product(assign35470_ad_e57457, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign35470_ad_e57516, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35470_ad_e57545, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_sub(62, 23, 22);
            s.store_mul(63, 226, 272);
            s.store_limited_exp_neg_input(64, 63);
        }

        s.b[1770] = (s.v[62] < 1e-10);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            let assign35540_ad_e57721: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35540_ad_e57720: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35540_ad_e57720
                }
            };
            let assign35540_ad_e57803: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                } else {
                    let assign35540_ad_e57802: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35540_ad_e57802
                }
            };
            s.store_sub_ad(169, assign35540_ad_e57721, assign35540_ad_e57803);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)));
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0));
            s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0), s.ad_value(170)));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            let assign35640_ad_e58024: A = A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0);
            let assign35640_ad_e58050: A = A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(assign35640_ad_e58024, 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0));
            s.store_neg_ad(65, assign35640_ad_e58050);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_mul_ad_lhs(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            let assign35660_ad_e58121: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product3(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))), 1.0));
            let assign35660_ad_e58166: A = A::mul(A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))));
            let assign35660_ad_e58191: A = A::add(A::add_scaled_inputs4(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, assign35660_ad_e58166, 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))));
            let assign35660_ad_e58233: A = A::add_scaled_inputs4(assign35660_ad_e58191, 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0, A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0, A::div(s.ad_value(179), A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0);
            s.store_offset_sub_ad(54, A::add_scaled_product(assign35660_ad_e58121, 1.0, s.ad_value(296), assign35660_ad_e58233, (-1.0)), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));
        }

        s.b[1771] = (s.v[54] >= 0.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) && s.b[1771]) {
            s.store_scaled_div_ad_rhs(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_add(23, 22, 62);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul(250, 62, 271);
            s.store_div_scaled_product_offset_denominator(67, s.ad_value(23), s.ad_value(23), 1.0, A::square(s.ad_value(23)), 2.0, 1.0);
            s.store_limited_exp_neg_input(68, 23);
            s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));
            s.store_sub_ad_lhs(70, A::mul3(A::sub(s.ad_value(214), s.ad_value(23)), A::sub(s.ad_value(214), s.ad_value(23)), A::div_from_scalar(1.0, s.ad_value(296))), 69);
        }

    }

    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_offset_ad(70, A::add_scaled_inputs(A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(70), (-0.001), A::offset(s.ad_value(70), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(60, 70);
            s.store_mul_sqrt_ad_rhs(72, 294, A::add(s.ad_value(70), s.ad_value(69)));
            s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 271, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);
            s.store_scaled_add(75, 22, 23, 0.5);
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
            s.store_sqrt(76, 54);
            s.store_scaled_add(77, 57, 69, 0.5);
            s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);
            s.store_sub_ad_lhs(79, A::mul3(A::sub(s.ad_value(214), s.ad_value(75)), A::sub(s.ad_value(214), s.ad_value(75)), A::div_from_scalar(1.0, s.ad_value(296))), 78);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_offset_ad(79, A::add_scaled_inputs(A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(79), (-0.001), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1772] = (((s.v[250]) as f64).abs() > 1e-35);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1772]) {
            s.store_div_scaled_inputs2(74, s.ad_value(306), 1.0, s.ad_value(73), (-1.0), s.ad_value(250), 1.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul_ad_rhs(80, 271, A::div_scaled_product(s.ad_value(296), s.ad_value(78), 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0));
            s.store_mul(52, 51, 271);
            s.copy_ad(83, 74);
            s.store_offset_ad(83, A::add_scaled_inputs(A::offset(s.ad_value(83), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(83), (-0.001), A::offset(s.ad_value(83), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_add_scaled_product_indices(81, 80, 1.0, 271, 83, 1.0);
            s.store_div(84, 81, 83);
        }

        s.b[1773] = (s.v[22] <= 0.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1773]) {
            s.copy_ad(447, 52);
            s.store_scalar(444, 0.0);
            s.copy_ad(445, 447);
            s.store_scalar(446, 0.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1773])) {
            s.store_scaled_div(26, 250, 84, 0.5);
            s.store_square(27, 26);
            s.store_add_scaled_product_indices(447, 52, 1.0, 250, 26, (0.3333333333333333 * 0.5));
            s.store_scaled_mul(54, 74, 250, 0.16666666666666666);
            s.store_add_scaled_product_indices(443, 80, 1.0, 54, 26, 1.0);
            s.store_add_scaled_product_right_ad(444, 80, 0.5, 54, A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), ((-1.0) * 0.5));
            s.store_sub(445, 447, 443);
            s.store_add_scaled_inputs3(446, s.ad_value(447), 1.0, s.ad_value(445), (-1.0), s.ad_value(444), -1.0);
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_scale_ad(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380);
            s.copy_ad(391, 437);
            s.store_mul_neg_lhs(440, 391, 445);
        }

        s.b[1774] = (s.v[211] > 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1774]) {
            s.store_mul_neg_lhs(441, 391, 446);
            s.store_mul_neg_lhs(439, 391, 444);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1774])) {
            s.store_mul_neg_lhs(441, 391, 444);
            s.store_mul_neg_lhs(439, 391, 446);
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_neg_ad(442, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(441), 1.0, s.ad_value(439), 1.0));
        }

        if (s.b[1620] && (!s.b[1741])) {
            s.store_scalar(440, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(438, 0.0);
            s.store_scalar(441, 0.0);
            s.store_scalar(442, 0.0);
        }

        if s.b[1620] {
            s.store_mul_add_ad_rhs(1075, 379, A::add_scaled_inputs4(s.ad_value(387), 1.0, s.ad_value(440), 1.0, s.ad_value(421), 1.0, s.ad_value(520), 1.0), s.ad_value(525));
            s.store_mul_add_rhs(1050, 379, 388, 441);
            s.store_mul_add_rhs(1053, 379, 389, 439);
            s.store_mul_ad_rhs(1076, 379, A::add_scaled_inputs4(s.ad_value(388), 1.0, s.ad_value(441), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0));
            s.store_mul_ad_rhs(1077, 379, A::add_scaled_inputs4(s.ad_value(389), 1.0, s.ad_value(439), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0));
            s.store_mul_ad_rhs(1078, 379, A::add_scaled_inputs3(s.ad_value(390), 1.0, s.ad_value(442), 1.0, s.ad_value(422), 1.0));
            s.store_mul(1057, 379, 390);
            s.store_mul(1058, 379, 442);
            s.store_mul(1051, 379, 388);
            s.store_mul(1052, 379, 441);
            s.store_mul(1054, 379, 389);
            s.store_mul(1055, 379, 439);
            s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);
            s.store_scalar(477, 0.0);
            s.store_scalar(479, 0.0);
            s.store_scalar(480, 0.0);
            s.store_scalar(483, 0.0);
            s.store_scalar(484, 0.0);
        }

        s.b[1775] = ((p.p37 != 0.0) || (p.p38 != 0.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1775]) {
            s.store_mul_ad_rhs(469, 269, A::add_scaled_inputs3(s.ad_value(213), 1.0, s.ad_value(22), (-0.5), s.ad_value(23), (-0.5)));
            s.store_sqrt_square_offset(168, 469, 0.0001);
            s.store_scaled_sub(471, 168, 469, 0.5);
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.b[1776] = (p.p38 != 0.0);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            let assign36560_ad_e59308: A = {
                if ((!((-s.v[168]) > 37.0)) && (!((-s.v[168]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::neg(s.ad_value(168)))
                } else {
                    {
                        if ((!((-s.v[168]) > 37.0)) && ((-s.v[168]) < (-37.0))) {
                            A::exp_scaled_input(s.ad_value(168), -1.0)
                        } else {
                            {
                                if ((-s.v[168]) > 37.0) {
                                    A::neg(s.ad_value(168))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_scale_ad(474, assign36560_ad_e59308, p.p671);
        }

        s.b[1777] = (p.p696 != 0.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1777]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1777])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1778] = (s.v[167] < 0.01);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1778]) {
            s.store_scalar(167, 0.01);
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
            s.store_scalar(169, (p.p701 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_mul_ad_lhs(476, A::mul3(s.ad_value(168), s.ad_value(221), s.ad_value(474)), 171);
            s.store_mul(476, 476, 662);
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            let assign36690_ad_e59483: A = {
                if ((!(s.v[168] > 37.0)) && (!(s.v[168] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(168))
                } else {
                    {
                        if ((!(s.v[168] > 37.0)) && (s.v[168] < (-37.0))) {
                            A::exp(s.ad_value(168))
                        } else {
                            {
                                if (s.v[168] > 37.0) {
                                    s.ad_value(168)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_scale_ad(473, assign36690_ad_e59483, p.p671);
        }

        s.b[1779] = (p.p697 != 0.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1779]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1779])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1780] = (s.v[167] < 0.01);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1780]) {
            s.store_scalar(167, 0.01);
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
            s.store_scalar(169, (p.p699 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_mul_ad_lhs(475, A::mul3(s.ad_value(168), s.ad_value(221), s.ad_value(473)), 171);
            s.store_mul(475, 475, 662);
            s.store_scaled_add(477, 476, 475, p.p2);
            s.store_offset_mul(478, 212, 269, p.p1383);
        }

        s.b[1781] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));
            s.store_sub(167, 208, 478);
            s.store_sqrt_square_offset(168, 167, 0.0001);
            s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_mul(169, 208, 209);
            s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);
            s.store_mul(171, 889, 890);
            s.store_mul_scaled_ad_rhs(172, 179, (-p.p76), A::sub(A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209))));
            s.store_limited_exp(173, 172);
            s.store_scaled_mul(178, 178, 492, p.p1380);
            s.store_mul_ad_lhs(210, A::mul3(s.ad_value(178), s.ad_value(169), s.ad_value(173)), 662);
        }

        if ((s.b[1620] && s.b[1775]) && (!s.b[1781])) {
            s.store_scalar(210, 0.0);
        }

        s.b[1782] = (p.p37 != 0.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));
            s.store_offset_mul(169, 812, 470, 1.0);
            s.store_scaled_mul(170, 168, 169, s.v[488]);
            s.store_mul_ad(171, A::mul3(s.ad_value(253), s.ad_value(269), s.ad_value(243)), A::limited_exp(s.ad_value(170)));
            s.store_mul_ad_lhs(481, A::mul3_scaled_output(s.ad_value(487), s.ad_value(171), A::add_scaled_inputs4(s.ad_value(221), 1.0, s.ad_value(227), 0.5, s.ad_value(224), (-0.5), s.ad_value(223), (-0.5)), p.p2), 662);
            s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_scale(168, 472, s.v[823]);
            s.store_limited_exp_neg_input(482, 168);
            s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);
            s.store_offset_square(172, 168, 0.0002);
        }

        s.b[1783] = (s.v[211] > 0.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1783]) {
            s.store_div_scaled_product_indices(480, 481, 171, 1.0, 172, 1.0);
            s.store_div_scaled_product_indices(479, 481, 170, 1.0, 172, 1.0);
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && (!s.b[1783])) {
            s.store_div_scaled_product_indices(479, 481, 171, 1.0, 172, 1.0);
            s.store_div_scaled_product_indices(480, 481, 170, 1.0, 172, 1.0);
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1784] = (p.p1295 == 1.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1784]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1785] = (s.v[818] < 0.01);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1784]) && s.b[1785]) {
            s.store_scalar(818, 0.01);
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && (!s.b[1784])) {
            s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_offset_mul(169, 818, 228, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_mul3_affine_lhs(485, 662, 489, p.p2, 0.0, 824);
            s.store_mul_ad_lhs(483, A::mul3(s.ad_value(485), s.ad_value(203), s.ad_value(228)), 171);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(229, 169, 0.0001);
        }

        s.b[1786] = (p.p1295 == 1.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1786]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1787] = (s.v[821] < 0.01);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1786]) && s.b[1787]) {
            s.store_scalar(821, 0.01);
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && (!s.b[1786])) {
            s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_offset_mul(169, 821, 229, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_mul3_affine_lhs(486, 662, 490, p.p2, 0.0, 825);
            s.store_mul_ad_lhs(484, A::mul3(s.ad_value(486), s.ad_value(204), s.ad_value(229)), 171);
        }

        if s.b[1620] {
            s.store_mul(1098, 379, 483);
            s.store_mul(1099, 379, 484);
            s.store_mul(1102, 379, 477);
            s.store_mul(1100, 379, 479);
            s.store_mul(1101, 379, 480);
            s.store_scale(621, 271, (4.0 * 1.602176462e-19));
            s.store_div_scaled_inputs(607, s.ad_value(746), 2.0, s.ad_value(337), 1.0);
        }

        s.b[1788] = (p.p1011 <= 0.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1788]) {
            s.store_scalar(610, 0.0);
        }

        if (s.b[1620] && (!s.b[1788])) {
            s.store_div_scaled_offset_numerator(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, s.ad_value(607), 1.0);
            s.store_mul_ln_ad_rhs(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));
        }

        s.b[1789] = (s.v[610] < 0.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1788])) && s.b[1789]) {
            s.store_scalar(610, 0.0);
        }

        if s.b[1620] {
            s.store_mul_scaled_ad_rhs(613, 271, 1.0 / (1.602176462e-19), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(709)));
            s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(73), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);
            s.store_add_scaled_product_value_ad(1006, A::scale_offset(s.ad_value(612), p.p1013, p.p1012), 1.0, 612, 612, p.p1014);
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.b[1790] = (p.p1015 >= (s.v[184] / 2.0));
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1790]) {
            s.store_scalar(606, 0.0);
        }

        if (s.b[1620] && (!s.b[1790])) {
            s.store_scalar(606, p.p1015);
        }

        s.b[1791] = (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1791]) {
            s.store_sub_from_scalar_scaled_input(608, s.v[184], 606, 2.0);
            s.store_square(609, 608);
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
            s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(306), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(169, 611, 612, p.p1013);
            s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(615, A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(380), 380);
            s.store_add(173, 615, 614);
        }

        s.b[1792] = (s.v[173] > 0.0);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1791]) && s.b[1792]) {
            s.store_div_scaled_product_by_product(616, s.ad_value(614), s.ad_value(615), 1.0, s.ad_value(173), A::scale_offset(A::powf(A::sub(s.ad_value(306), s.ad_value(73)), p.p1017), p.p1016, 1.0), 1.0);
        }

        if ((s.b[1620] && s.b[1791]) && (!s.b[1792])) {
            s.store_scalar(616, 0.0);
        }

        if (s.b[1620] && (!s.b[1791])) {
            s.store_scalar(616, 0.0);
        }

        if s.b[1620] {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
            s.store_square(168, 167);
            s.store_offset_scaled(170, 168, (((p.p1022 * s.v[184])) * (p.p1019)), p.p1019);
            s.store_offset_scaled(171, 168, (((p.p1023 * s.v[184])) * (p.p1020)), p.p1020);
            s.store_offset_scaled(172, 168, (((p.p1298 * s.v[184])) * (p.p1297)), p.p1297);
            s.store_offset_scaled(630, 168, (((p.p1024 * s.v[184])) * (p.p1021)), p.p1021);
            s.store_scaled_mul(631, 170, 170, 3.0);
        }

        if s.b[1620] {
            s.store_offset_scaled(631, 631, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
        }

        if s.b[1620] {
            s.store_square(633, 172);
            s.store_square(632, 171);
            s.store_scalar(627, 0.0);
            s.copy_ad(345, 343);
        }

        s.b[1793] = (p.p39 == 0.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        s.b[1794] = (p.p39 == 1.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1793]) {
            s.store_scaled_mul(388, 271, 1050, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_scaled_mul(389, 271, 1053, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_mul_abs_ad_rhs(167, 337, A::add(s.ad_value(388), s.ad_value(389)));
            s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));
            s.store_scaled_div(619, 167, 168, p.p1018);
            s.store_mul(620, 621, 619);
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_scaled_mul(626, 253, 269, 2.0);
            s.store_mul_scale_ad_lhs(167, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 626);
            s.store_scaled_add(168, 306, 73, 0.5);
            s.store_offset(170, 168, 0.5);
            s.store_square(171, 170);
            s.store_mul(172, 171, 170);
            s.store_sub(173, 306, 73);
            s.store_square(174, 173);
            s.store_mul(175, 174, 173);
            s.store_mul_ad_lhs(176, A::scale_offset(s.ad_value(168), 6.0, 0.5), 174);
            s.store_scale(625, 345, s.v[184]);
            s.store_scale(177, 625, 1.0 / (s.v[184]));
            s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(312), A::offset(s.ad_value(243), p.p1299), 1.0), 1.0);
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_offset_scaled(179, 179, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_scaled_add_ad_rhs(179, 179, A::sqrt(A::offset(A::mul(s.ad_value(179), s.ad_value(179)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul_ad(624, A::div_scaled_inputs(s.ad_value(167), (p.p2 * s.v[183]), s.ad_value(625), 1.0), A::add_scaled_product(A::div_scaled_product(s.ad_value(174), s.ad_value(631), 1.0, s.ad_value(170), 12.0), 1.0, s.ad_value(168), s.ad_value(179), 1.0));
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            let assign38150_ad_e61223: A = A::mul3_scaled_output(A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), s.ad_value(632), (15.0 * 1.0 / (4.0)));
            s.store_div_scaled_inputs(622, assign38150_ad_e61223, 1.0, s.ad_value(167), ((p.p2 * s.v[183]) * 12.0));
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_mul_ad_affine_product_lhs(623, s.ad_value(177), A::sub(A::div_scaled_inputs(s.ad_value(173), 1.0, s.ad_value(170), 12.0), A::div_scaled_inputs(s.ad_value(175), 1.0, s.ad_value(172), 144.0)), 2.531645569620253, 0.0, 630);
            s.store_sqrt_mul(628, 621, 624);
        }

        s.b[1795] = (s.v[622] > 0.0);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (s.b[1794] && (!s.b[1793]))) && s.b[1795]) {
            s.store_sqrt_div(629, 621, 622);
        }

        s.b[1796] = (s.v[628] > 0.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (s.b[1794] && (!s.b[1793]))) && s.b[1795]) && s.b[1796]) {
            s.store_div_scaled_product_indices(627, 623, 629, 1.0, 628, 1.0);
        }

        if (((s.b[1620] && (s.b[1794] && (!s.b[1793]))) && s.b[1795]) && (!s.b[1796])) {
            s.store_scalar(627, 0.0);
        }

        if ((s.b[1620] && (s.b[1794] && (!s.b[1793]))) && (!s.b[1795])) {
            s.store_scalar(629, 0.0);
            s.store_scalar(627, 0.0);
        }

        s.b[1797] = (p.p37 != 0.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        s.b[1798] = (p.p38 != 0.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        s.b[1799] = (p.p27 == 1.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1799]) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && s.b[1799]) {
            s.store_sqrt(299, 298);
            s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19));
            s.store_mul_scale_ad_rhs(941, 835, A::add(A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5);
            s.store_mul_offset_ad_rhs(940, 841, A::mul_offset_rhs(s.ad_value(848), s.ad_value(639), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_offset(273, s.ad_value(298), 0.5, s.ad_value(218), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05), A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
            s.store_mul(275, 277, 274);
            s.store_div_from_scalar(260, s.v[180], 275);
            s.store_div_scaled_product_denominator_ad(169, 5, 7, 1.0, A::add(s.ad_value(5), s.ad_value(7)), 1.0);
            s.store_mul_ad_lhs(170, A::add_scaled_inputs3(s.ad_value(838), 1.0, s.ad_value(220), p.p1183, s.ad_value(218), (-p.p1195)), 227);
            s.store_add_scaled_inputs_products_mixed_aiiiia(171, A::add_scaled_product(s.ad_value(220), p.p1181, s.ad_value(220), s.ad_value(220), p.p1182), 1.0, 218, (-p.p1184), 218, 218, (-p.p1185), 955, A::add(A::add_scaled_product(A::add_scaled_value_products3(s.ad_value(715), 1.0, s.ad_value(712), s.ad_value(220), 1.0, s.ad_value(220), s.ad_value(220), p.p1180, s.ad_value(716), s.ad_value(218), 1.0), 1.0, s.ad_value(218), s.ad_value(218), p.p1190), s.ad_value(170)), 1.0);
            s.store_div_ad(168, A::add_scaled_inputs4_offset(s.ad_value(169), 1.0, s.ad_value(836), 1.0, s.ad_value(941), 1.0, s.ad_value(171), 1.0, s.v[199]), A::offset(s.ad_value(169), s.v[199]));
            s.store_scaled_add_ad(267, A::offset(s.ad_value(168), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(168), (-1.0), A::offset(s.ad_value(168), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
            s.store_mul(269, 267, 271);
            s.store_div_from_scalar(270, 1.0, 269);
            s.store_mul(222, 221, 270);
            s.store_mul(225, 224, 270);
            s.store_mul(212, 707, 270);
            s.store_mul(215, 708, 270);
            s.store_mul(238, 234, 270);
            s.store_add_scaled_products_left_right_ad(291, 736, A::sub(s.ad_value(274), s.ad_value(299)), 1.0, 849, 218, (-1.0));
            s.store_mul_neg_ad_lhs(944, A::add_scaled_product(s.ad_value(940), 1.0, s.ad_value(842), s.ad_value(218), 1.0), 227);
            s.store_mul_offset_rhs_ad(293, A::add_scaled_inputs_product(s.ad_value(843), 1.0, s.ad_value(844), 1.0 / (s.v[184]), s.ad_value(845), s.ad_value(218), 1.0), A::pow(s.ad_value(639), s.ad_value(846)), (-1.0));
            s.store_mul_ad_rhs(946, 300, A::scale_offset(s.ad_value(218), p.p1264, 1.0));
        }

        s.b[1800] = (s.v[946] > 0.0);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1800]) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.b[1801] = (s.v[167] < 40.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && s.b[1801]) {
            s.store_div_from_scalar_offset_ad(943, (0.5 * p.p1262), A::cosh(s.ad_value(167)), (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && (!s.b[1801])) {
            s.store_scaled_limited_exp_scaled_input(943, 167, -1.0, p.p1262);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1800])) {
            s.store_scalar(943, 0.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_mul_sub_rhs(945, 943, 942, 298);
        }

        s.b[1802] = (s.v[280] > 0.0);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {
            s.store_mul_neg_lhs(167, 282, 227);
        }

        s.b[1803] = (s.v[167] < (-80.0));
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && s.b[1803]) {
            s.store_scalar(169, 1.804851387e-35);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && (!s.b[1803])) {
            s.store_limited_exp(169, 167);
        }

        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {
            s.store_offset_mul_offset_rhs(170, 280, 169, 1.0, s.v[184]);
            s.store_mul_scaled_ad_rhs(278, 269, -1.0, A::ln(A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38)));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1802])) {
            s.store_scalar(278, 0.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_add_ad_rhs(171, 290, A::div(s.ad_value(284), A::pow_from_scalar(s.v[184], s.ad_value(286))));
            s.store_add_scaled_product_right_ad(278, 278, 1.0, 171, A::tanh(A::mul(s.ad_value(288), s.ad_value(227))), (-1.0));
            s.store_add_scaled_inputs3(242, A::offset(A::add(A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(278), 1.0, s.ad_value(944), 1.0, s.ad_value(293), -1.0), s.ad_value(945)), p.p1151), 1.0, s.ad_value(956), 1.0, s.ad_value(932), 1.0);
            s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_add_scaled_inputs_product_first_ad(367, A::add_scaled_product(s.ad_value(222), 1.0, s.ad_value(218), s.ad_value(270), (-1.0)), 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_add_scaled_inputs_product_indices(214, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_sub(216, 238, 215);
            s.store_scaled_sqrt_ad(294, A::mul_scaled_lhs(s.ad_value(953), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(270)), 1.0 / (s.v[199]));
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
            s.store_mul_offset_rhs(294, 294, 947, 1.0);
            s.store_div_from_scalar(295, 1.0, 294);
            s.store_square(296, 294);
            s.store_div_from_scalar(297, 1.0, 296);
            s.store_scalar(5, (s.v[180] / p.p74));
            s.store_scalar(7, (s.v[181] / p.p75));
            s.store_div_scaled_inputs2(3, s.ad_value(7), 1.0, s.ad_value(728), 1.0, s.ad_value(5), 1.0);
            s.store_scalar(2, (p.p76 / p.p75));
            s.store_div(124, 294, 2);
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
            s.store_scale(126, 125, 1e-7);
            s.store_scalar(127, (5.0 / 4.0));
            s.store_div_from_scalar(128, 1.0, 124);
            s.store_square(129, 124);
            s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));
        }

        s.b[1804] = (((s.v[216]) as f64).abs() <= s.v[126]);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1804]) {
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        s.b[1805] = (s.v[216] < (-s.v[126]));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && s.b[1805]) {
            s.store_neg(132, 216);
            s.store_mul3_lhs(133, 127, 132, 128);
            s.store_scaled_sub_ad(134, A::offset(s.ad_value(133), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(133), (-6.0), A::offset(s.ad_value(133), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(135, A::sub(s.ad_value(132), s.ad_value(134)), A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);
            s.store_add_scaled_inputs3(137, s.ad_value(132), 2.0, s.ad_value(134), (-2.0), s.ad_value(129), -1.0);
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
            s.store_add(0, 135, 137);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);
            s.store_add_ad_rhs(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));
            s.store_limited_exp(141, 140);
            s.store_sub(142, 132, 140);
            s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);
            s.store_sub_from_scalar_ad(144, 1.0, A::mul_scaled_lhs(s.ad_value(129), 0.5, s.ad_value(141)));
            s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));
            s.store_scaled_div_ad_rhs(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);
            s.store_neg_ad(131, A::add(s.ad_value(140), s.ad_value(145)));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_mul_offset_ad_lhs(146, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), (-1.0), 130);
            s.store_mul_ad_product_rhs(147, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
            s.store_limited_exp_neg_input(150, 147);
            s.store_sub_from_scalar(149, 1.0, 150);
            s.store_add_scaled_inputs_product_right_ad(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));
            s.store_limited_exp_neg_input(151, 148);
            s.store_add_scaled_inputs3(152, s.ad_value(216), 2.0, s.ad_value(148), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);
            s.store_add_scaled_products_mixed_aaia(153, A::sub(s.ad_value(216), s.ad_value(148)), A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));
            s.store_sub_from_scalar_ad(154, 1.0, A::mul_scaled_lhs(s.ad_value(129), 0.5, s.ad_value(151)));
            s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));
            s.store_scaled_div_ad_rhs(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);
            s.store_add(131, 148, 139);
        }

        s.b[1806] = (((s.v[216]) as f64).abs() < s.v[126]);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1806]) {
            s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1806])) {
            s.store_add_scaled_inputs3_offset(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, s.ad_value(131), -1.0, (-(-1.0)));
            s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));
            s.store_sub_ad_rhs(46, 131, A::div(s.ad_value(19), s.ad_value(20)));
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_mul(46, 46, 269);
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
            s.store_div_from_scalar(96, 1.0, 95);
            s.store_add_ad_lhs(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 225);
            s.store_limited_exp_neg_input(99, 97);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_lhs(168, s.ad_value(725), A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, 3, 216, (-1.0));
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1807] = (s.v[4] < s.v[97]);
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        s.b[1808] = (s.v[214] < s.v[104]);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        s.b[1809] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1810] = (s.v[214] < (-s.v[101]));
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(97), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_add_scaled_inputs3(106, s.ad_value(105), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0)), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1811] = (s.v[107] < 0.0);
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) && s.b[1811]) {
            s.store_scalar(107, 0.0);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {
            s.store_add_scaled_inputs3(49, s.ad_value(97), 1.0, s.ad_value(106), (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 97);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_scaled_ad_rhs(121, 120, 2.0, A::add_scaled_sub_value_product(2.0, A::scale(s.ad_value(48), 2.0), 1.0, s.ad_value(296), s.ad_value(118), (-1.0)));
            s.store_div_scaled_inputs(122, s.ad_value(120), 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1812] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1807])) && s.b[1812]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1813] = (s.v[214] < (-s.v[101]));
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && s.b[1813]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(97), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_lhs(168, s.ad_value(725), A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_add_scaled_value_products(6, s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0), A::offset(s.ad_value(3), 1.0), s.ad_value(46), 1.0);
        }

        s.b[1814] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1814]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            let assign41080_ad_e66595: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41080_ad_e66594: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41080_ad_e66594
                }
            };
            let assign41080_ad_e66677: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41080_ad_e66676: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41080_ad_e66676
                }
            };
            s.store_sub_ad(169, assign41080_ad_e66595, assign41080_ad_e66677);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            let assign41170_ad_e66885: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign41170_ad_e66885, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            let assign41180_ad_e66954: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign41180_ad_e67013: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign41180_ad_e67042: A = A::sub(A::add_scaled_product(assign41180_ad_e66954, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign41180_ad_e67013, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41180_ad_e67042, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 22);
        }

        s.b[1815] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1815]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            let assign41280_ad_e67286: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41280_ad_e67285: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41280_ad_e67285
                }
            };
            let assign41280_ad_e67368: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41280_ad_e67367: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41280_ad_e67367
                }
            };
            s.store_sub_ad(169, assign41280_ad_e67286, assign41280_ad_e67368);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            let assign41370_ad_e67576: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign41370_ad_e67576, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            let assign41380_ad_e67645: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign41380_ad_e67704: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign41380_ad_e67733: A = A::sub(A::add_scaled_product(assign41380_ad_e67645, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign41380_ad_e67704, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41380_ad_e67733, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 22);
        }

        s.b[1816] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1816]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            let assign41480_ad_e67977: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41480_ad_e67976: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41480_ad_e67976
                }
            };
            let assign41480_ad_e68059: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign41480_ad_e68058: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41480_ad_e68058
                }
            };
            s.store_sub_ad(169, assign41480_ad_e67977, assign41480_ad_e68059);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            let assign41570_ad_e68267: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign41570_ad_e68267, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            let assign41580_ad_e68336: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign41580_ad_e68395: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign41580_ad_e68424: A = A::sub(A::add_scaled_product(assign41580_ad_e68336, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign41580_ad_e68395, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41580_ad_e68424, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_scale(50, 269, 3.912023005);
        }

        s.b[1817] = (s.v[22] <= 0.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1817]) {
            s.store_scalar(306, 0.0);
            s.store_sub(51, 214, 22);
            s.copy_ad(312, 50);
            s.store_scalar(458, 1.0);
            s.store_scalar(334, 1.0);
            s.store_scalar(853, 1.0);
            s.store_scalar(343, 1.0);
            s.store_scalar(339, 1.0);
            s.store_scalar(363, 1.0);
            s.store_scalar(365, 1.0);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_div_from_scalar_offset_ad(54, 1.0, A::square(s.ad_value(22)), 2.0);
            s.store_mul_square_lhs(55, 22, 54);
            s.store_limited_exp(53, 22);
            s.store_div_from_scalar(56, 1.0, 53);
            s.store_limited_exp_sub(53, 22, 97);
            s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));
            s.store_sub_ad_lhs(58, A::mul3(A::sub(s.ad_value(214), s.ad_value(22)), A::sub(s.ad_value(214), s.ad_value(22)), A::div_from_scalar(1.0, s.ad_value(296))), 57);
            s.store_offset_ad(58, A::add_scaled_inputs(A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(58), (-0.001), A::offset(s.ad_value(58), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_sqrt(59, 58);
            s.store_mul_sqrt_ad_rhs(61, 294, A::add(s.ad_value(58), s.ad_value(57)));
            s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 269, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);
            s.store_mul3_lhs(247, 59, 294, 269);
            s.copy_ad(76, 56);
            s.copy_ad(78, 57);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_ad_rhs(308, 335, A::add_scaled_inputs(s.ad_value(247), 1.0, s.ad_value(306), s.v[338]));
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
        }

        s.b[1818] = (p.p33 == 1.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1818]) {
            s.store_scalar(456, 0.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        s.b[1819] = (p.p33 == 2.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_mul_ad_lhs(310, A::div_scaled_inputs(s.ad_value(746), 2.0, s.ad_value(740), 1.0), 309);
            s.store_scale(311, 310, s.v[184]);
            s.store_mul_ad_rhs(173, 742, A::add_scaled_inputs(s.ad_value(306), 1.0, s.ad_value(269), 2.0));
        }

        s.b[1820] = (s.v[456] > 0.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1820]) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
            s.store_mul(167, 324, 456);
            s.store_scale(325, 167, 2.0);
            s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);
            s.store_mul_ad_rhs(327, 173, A::add_scaled_product(s.ad_value(311), 1.0, s.ad_value(173), s.ad_value(167), 2.0));
            s.store_div_scaled_inputs2(312, s.ad_value(326), 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), s.ad_value(325), 1.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1820])) {
            s.store_div_scaled_product_denominator_ad(312, 311, 173, 1.0, A::add(s.ad_value(311), s.ad_value(173)), 1.0);
        }

        s.b[1821] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1821]) {
            s.store_scalar(1019, 1.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1821])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_ad(1019, A::div_scaled_inputs2(s.ad_value(168), p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0), 1.0);
            s.store_scaled_add_ad(1019, A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1019), (-0.1), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_offset_ad(312, A::add_scaled_inputs(A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(312), (-0.001), A::offset(s.ad_value(312), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_div(312, 312, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_add_ad_lhs(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);
            s.store_limited_exp_neg_input(100, 98);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1822] = (s.v[4] < s.v[98]);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        s.b[1823] = (s.v[214] < s.v[104]);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        s.b[1824] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && s.b[1824]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1825] = (s.v[214] < (-s.v[101]));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && s.b[1825]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && (!s.b[1825])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(98), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_add_scaled_inputs3(106, s.ad_value(105), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0)), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1826] = (s.v[107] < 0.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) && s.b[1826]) {
            s.store_scalar(107, 0.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {
            s.store_add_scaled_inputs3(49, s.ad_value(98), 1.0, s.ad_value(106), (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 98);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_scaled_ad_rhs(121, 120, 2.0, A::add_scaled_sub_value_product(2.0, A::scale(s.ad_value(48), 2.0), 1.0, s.ad_value(296), s.ad_value(118), (-1.0)));
            s.store_div_scaled_inputs(122, s.ad_value(120), 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1827] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && s.b[1827]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1828] = (s.v[214] < (-s.v[101]));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && s.b[1828]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_ad(12, A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(11), (-6.0), A::offset(s.ad_value(11), (-6.0))), 64.0)), 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && (!s.b[1828])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0)), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(15, 1.0, A::mul_scaled_output(s.ad_value(296), A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(17, s.ad_value(98), 1.0, s.ad_value(12), (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0))));
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_div_scaled_inputs(167, s.ad_value(726), (-s.v[184]), s.ad_value(300), 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(269)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);
        }

        s.b[1829] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1829]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            let assign43980_ad_e73575: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign43980_ad_e73574: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign43980_ad_e73574
                }
            };
            let assign43980_ad_e73657: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign43980_ad_e73656: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign43980_ad_e73656
                }
            };
            s.store_sub_ad(169, assign43980_ad_e73575, assign43980_ad_e73657);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            let assign44070_ad_e73892: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign44070_ad_e73892, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            let assign44080_ad_e73964: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign44080_ad_e74023: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign44080_ad_e74052: A = A::sub(A::add_scaled_product(assign44080_ad_e73964, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign44080_ad_e74023, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44080_ad_e74052, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 23);
        }

        s.b[1830] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1830]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            let assign44180_ad_e74325: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44180_ad_e74324: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign44180_ad_e74324
                }
            };
            let assign44180_ad_e74407: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44180_ad_e74406: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign44180_ad_e74406
                }
            };
            s.store_sub_ad(169, assign44180_ad_e74325, assign44180_ad_e74407);
        }

    }
}
