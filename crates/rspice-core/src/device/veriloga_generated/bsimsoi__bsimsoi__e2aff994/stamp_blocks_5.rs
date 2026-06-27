#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1867]) {
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(454, 652, s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(229, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 229, 1.0);
            s.store_add_scaled_product_value_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(455, 652, s.ad_value(453), 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1867])) {
            s.store_offset_mul(167, 770, 243, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_scaled_add_sqrt_square_offset_rhs(170, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
            s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        s.b[1868] = (p.p33 == 2.0);
        s.v[1868] = if s.b[1868] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1867])) && s.b[1868]) {
            s.store_mul_add_ad_rhs(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453));
            s.store_scalar(455, 0.0);
            s.store_scalar(454, 0.0);
            s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        if (!s.b[1620]) {
            s.store_add_ad_rhs(167, 330, A::div(s.ad_value(333), A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0)));
            s.store_sub(416, 400, 320);
            s.store_mul3_lhs(168, 167, 416, 416);
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
            s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.004)), 0.5, (-1.0));
            s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);
            s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(334), (-1.0), A::offset(s.ad_value(334), (-1.0))), ((0.25 * 0.01) * 0.01))), 0.5, (0.25 * 0.01));
            s.store_add(167, 400, 320);
            s.store_sub(168, 400, 320);
            s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);
            s.store_mul3_lhs(170, 832, 169, 169);
            s.store_offset(834, 170, 1.0);
            s.store_div_ad_rhs(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));
            s.store_limited_exp_neg_input(853, 176);
            s.store_mul3_lhs(340, 339, 343, 458);
            s.store_div(337, 740, 340);
            s.store_mul_ad_product_lhs(380, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(253), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363), 1.0, s.ad_value(334), 1.0), s.ad_value(834), 853);
            s.store_scale(380, 380, p.p26);
            s.store_scalar(467, 0.0);
        }

        s.b[1869] = (p.p7 > 1.0);
        s.v[1869] = if s.b[1869] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1869]) {
            s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scale(176, 271, p.p1009);
            s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));
        }

        s.b[1870] = (p.p7 == 2.0);
        s.v[1870] = if s.b[1870] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        s.b[1871] = (s.v[466] < p.p1347);
        s.v[1871] = if s.b[1871] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1869]) && s.b[1870]) && s.b[1871]) {
            s.store_scalar(466, p.p1347);
            s.store_div_from_scalar(465, 1.0, 466);
        }

        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {
            s.store_add(178, 465, 467);
            s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);
        }

        if (!s.b[1620]) {
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

        s.b[1872] = (s.v[550] == 0.0);
        s.v[1872] = if s.b[1872] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1872])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(548, 550, 168);
            s.store_mul(167, 545, 548);
        }

        s.b[1873] = (s.v[551] == 0.0);
        s.v[1873] = if s.b[1873] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1873])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(549, 551, 168);
            s.store_mul(167, 546, 549);
        }

        s.b[1874] = (s.v[552] == 0.0);
        s.v[1874] = if s.b[1874] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(554, 552, 169);
            s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);
            s.store_div(167, 498, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1875] = ((s.v[558] - s.v[498]) < 0.001);
        s.v[1875] = if s.b[1875] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1874])) && s.b[1875]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1874])) && (!s.b[1875])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_mul(170, 545, 554);
        }

        s.b[1876] = (s.v[553] == 0.0);
        s.v[1876] = if s.b[1876] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(555, 553, 169);
            s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);
            s.store_div(167, 499, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1877] = ((s.v[559] - s.v[499]) < 0.001);
        s.v[1877] = if s.b[1877] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1876])) && s.b[1877]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1876])) && (!s.b[1877])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_mul(170, 546, 555);
        }

        if (!s.b[1620]) {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.b[1878] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));
        s.v[1878] = if s.b[1878] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1878])) {
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

        s.b[1879] = (s.v[583] < 1e-5);
        s.v[1879] = if s.b[1879] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1879]) {
            s.store_scalar(583, 0.0);
            s.store_scalar(591, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1879])) {
            s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_mul_offset_rhs(584, 586, 596, (-1.0));
        }

        s.b[1880] = (s.v[584] < 1e-5);
        s.v[1880] = if s.b[1880] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1880]) {
            s.store_scalar(584, 0.0);
            s.store_scalar(592, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1880])) {
            s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));
        }

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_limited_exp(603, 167);
            s.store_sub_from_scalar(169, 1.0, 603);
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
            s.store_pow_ad(599, s.ad_value(167), s.ad_value(600));
            s.store_mul3_lhs(604, 602, 578, 599);
            s.store_mul(168, 167, 604);
            s.store_mul3_lhs(604, 602, 577, 599);
            s.store_mul(168, 167, 604);
            s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);
            s.store_mul3_lhs(532, 602, 578, 531);
            s.store_mul_ad_product_lhs(533, s.ad_value(532), A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(532, 602, 577, 531);
            s.store_mul_ad_product_lhs(534, s.ad_value(532), A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);
        }

        s.b[1881] = (s.v[580] < 1.0);
        s.v[1881] = if s.b[1881] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1881]) {
            s.store_scalar(580, 1.0);
        }

        s.b[1882] = (p.p554 == 1.0);
        s.v[1882] = if s.b[1882] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {
            s.store_offset_div_scaled_inputs2(167, s.ad_value(498), 1.0, s.ad_value(499), 1.0, s.ad_value(580), 1.0, 1.0);
            s.store_add(168, 583, 584);
            s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);
            s.store_scaled_add(169, 167, 170, 0.5);
            s.store_mul(167, 603, 604);
        }

        s.b[1884] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1884])) {
            s.store_mul_offset_rhs(174, 569, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(571, 567, 167);
            s.store_mul_offset_rhs(174, 570, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(572, 568, 167);
            s.store_scale(594, 573, p.p925);
        }

        s.b[1885] = ((s.v[575] - s.v[498]) < 0.001);
        s.v[1885] = if s.b[1885] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1885]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1885])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
        }

        if ((!s.b[1620]) && (!s.b[1884])) {
            s.store_scale(594, 574, p.p925);
        }

        s.b[1886] = ((s.v[576] - s.v[499]) < 0.001);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1886]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1886])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
        }

        s.b[1887] = (p.p36 == 0.0);
        s.v[1887] = if s.b[1887] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1887]) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        s.b[1888] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));
        s.v[1888] = if s.b[1888] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1887]) && s.b[1888]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {
            s.store_div_scaled_inputs3(168, s.ad_value(204), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1889] = (s.v[894] != 0.0);
        s.v[1889] = if s.b[1889] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && s.b[1889]) {
            s.store_mul_square_lhs(170, 201, 201);
            s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5, (-1e-6));
        }

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && (!s.b[1889])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        s.b[1890] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));
        s.v[1890] = if s.b[1890] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1887]) && s.b[1890]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {
            s.store_div_scaled_inputs3(168, s.ad_value(203), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1891] = (s.v[898] != 0.0);
        s.v[1891] = if s.b[1891] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && s.b[1891]) {
            s.store_mul_square_lhs(170, 202, 202);
            s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5, (-1e-6));
        }

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && (!s.b[1891])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        if ((!s.b[1620]) && (!s.b[1887])) {
            s.store_scalar(167, (s.v[200] * p.p76));
            s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);
            s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1892] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));
        s.v[1892] = if s.b[1892] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1892]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {
            s.store_div_scaled_inputs3(168, s.ad_value(207), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1893] = (s.v[903] != 0.0);
        s.v[1893] = if s.b[1893] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && s.b[1893]) {
            s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(903), s.ad_value(171)), A::div(s.ad_value(903), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5, (-1e-6));
        }

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && (!s.b[1893])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {
            s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));
        }

        s.b[1894] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));
        s.v[1894] = if s.b[1894] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1894]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {
            s.store_div_scaled_inputs3(168, s.ad_value(206), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1895] = (s.v[906] != 0.0);
        s.v[1895] = if s.b[1895] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && s.b[1895]) {
            s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(906), s.ad_value(171)), A::div(s.ad_value(906), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5, (-1e-6));
        }

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && (!s.b[1895])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {
            s.store_mul3_ad(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));
        }

        s.b[1896] = (p.p44 == 0.0);
        s.v[1896] = if s.b[1896] { 1.0 } else { 0.0 };

        s.b[1897] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));
        s.v[1897] = if s.b[1897] { 1.0 } else { 0.0 };

        s.b[1898] = (s.v[355] > (s.v[659] / 80.0));
        s.v[1898] = if s.b[1898] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1896]) && (!s.b[1897])) && s.b[1898]) {
            s.store_div_scaled_inputs(168, s.ad_value(659), -1.0, s.ad_value(355), 1.0);
        }

        s.b[1899] = (p.p44 == 1.0);
        s.v[1899] = if s.b[1899] { 1.0 } else { 0.0 };

        s.b[1900] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1900] = if s.b[1900] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {
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

        s.b[1901] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1901] = if s.b[1901] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {
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

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));
            s.store_mul_scale_offset_rhs(378, 880, 639, p.p666, (((((-1.0)) * (p.p666))) + (1.0)));
        }

    }

    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1902] = (s.v[211] > 0.0);
        s.v[1902] = if s.b[1902] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1902]) {
            s.store_sub(168, 378, 499);
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1902])) {
            s.store_sub(168, 378, 498);
        }

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_offset(169, 881, (-1.0));
        }

        s.b[1903] = (s.v[168] > 0.0);
        s.v[1903] = if s.b[1903] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1903]) {
            s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1903])) {
            s.store_scalar(170, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_limited_exp(171, 170);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);
        }

        s.b[1904] = ((p.p37 != 0.0) || (p.p38 != 0.0));
        s.v[1904] = if s.b[1904] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1904]) {
            s.store_mul_ad_rhs(469, 269, A::add_scaled_inputs4(s.ad_value(213), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), 1.0, s.ad_value(320), 1.0));
            s.store_sqrt_square_offset(168, 469, 0.0001);
            s.store_scaled_sub(471, 168, 469, 0.5);
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.b[1905] = (p.p38 != 0.0);
        s.v[1905] = if s.b[1905] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        s.b[1906] = (p.p696 != 0.0);
        s.v[1906] = if s.b[1906] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1906]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1906])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1907] = (s.v[167] < 0.01);
        s.v[1907] = if s.b[1907] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1907]) {
            s.store_scalar(167, 0.01);
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
            s.store_scalar(169, (p.p701 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        s.b[1908] = (p.p697 != 0.0);
        s.v[1908] = if s.b[1908] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1908]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1908])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1909] = (s.v[167] < 0.01);
        s.v[1909] = if s.b[1909] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1909]) {
            s.store_scalar(167, 0.01);
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
            s.store_scalar(169, (p.p699 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_offset_mul(478, 212, 269, p.p1383);
        }

        s.b[1910] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));
        s.v[1910] = if s.b[1910] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));
            s.store_sub(167, 208, 478);
            s.store_sqrt_square_offset(168, 167, 0.0001);
            s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_mul(169, 208, 209);
            s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);
            s.store_mul(171, 889, 890);
            s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p.p76), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p.p76));
            s.store_limited_exp(173, 172);
            s.store_scaled_mul(178, 178, 492, p.p1380);
        }

        s.b[1911] = (p.p37 != 0.0);
        s.v[1911] = if s.b[1911] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));
            s.store_offset_mul(169, 812, 470, 1.0);
            s.store_scaled_mul(170, 168, 169, s.v[488]);
            s.store_mul_ad(171, A::mul3(s.ad_value(253), s.ad_value(269), A::add(s.ad_value(400), s.ad_value(320))), A::limited_exp(s.ad_value(170)));
            s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_scale(168, 472, s.v[823]);
            s.store_limited_exp_neg_input(482, 168);
            s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);
            s.store_offset_square(172, 168, 0.0002);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1913] = (p.p1295 == 1.0);
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1914] = (s.v[818] < 0.01);
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) && s.b[1914]) {
            s.store_scalar(818, 0.01);
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1913])) {
            s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_offset_mul(169, 818, 228, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(229, 169, 0.0001);
        }

        s.b[1915] = (p.p1295 == 1.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1916] = (s.v[821] < 0.01);
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) && s.b[1916]) {
            s.store_scalar(821, 0.01);
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1915])) {
            s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_offset_mul(169, 821, 229, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
        }

        if (!s.b[1620]) {
            s.store_mul(502, 666, 463);
            s.store_mul(505, 667, 494);
            s.store_scale(508, 671, (s.v[189] * p.p2));
            s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));
        }

        s.b[1917] = (p.p913 == 1.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1917]) {
            s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1917])) {
            s.store_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));
        }

        if (!s.b[1620]) {
            s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));
        }

        s.b[1918] = (p.p915 == 1.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1918]) {
            s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1918])) {
            s.store_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));
        }

        if (!s.b[1620]) {
            s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));
        }

        s.b[1919] = (p.p917 == 1.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1919]) {
            s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1919])) {
            s.store_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));
        }

        s.b[1920] = (s.v[502] > 0.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1920]) {
            s.store_div(168, 498, 672);
        }

        s.b[1921] = (s.v[168] < 0.9);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1920]) && s.b[1921]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1922] = (p.p913 != 1.0);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        s.b[1923] = (p.p913 == 0.5);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p913));
        }

        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p913)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && (!s.b[1922])) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1920]) && (!s.b[1921])) {
            s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p913), (((((-1.0)) * ((5.0 * p.p913)))) + ((1.0 + p.p913)))));
            s.store_mul_ad_product_rhs(521, 672, s.ad_value(502), A::add(s.ad_value(169), s.ad_value(504)));
        }

        if ((!s.b[1620]) && (!s.b[1920])) {
            s.store_scalar(521, 0.0);
        }

        s.b[1924] = (s.v[505] > 0.0);
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1924]) {
            s.store_div(168, 498, 673);
        }

        s.b[1925] = (s.v[168] < 0.9);
        s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1924]) && s.b[1925]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1926] = (p.p915 != 1.0);
        s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };

        s.b[1927] = (p.p915 == 0.5);
        s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && s.b[1927]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p915));
        }

        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p915)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && (!s.b[1926])) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1924]) && (!s.b[1925])) {
            s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p915), (((((-1.0)) * ((5.0 * p.p915)))) + ((1.0 + p.p915)))));
            s.store_mul_ad_product_rhs(522, 673, s.ad_value(505), A::add(s.ad_value(169), s.ad_value(507)));
        }

        if ((!s.b[1620]) && (!s.b[1924])) {
            s.store_scalar(522, 0.0);
        }

        s.b[1928] = (s.v[508] > 0.0);
        s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1928]) {
            s.store_div(168, 498, 674);
        }

        s.b[1929] = (s.v[168] < 0.9);
        s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1928]) && s.b[1929]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1930] = (p.p917 != 1.0);
        s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };

        s.b[1931] = (p.p917 == 0.5);
        s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && s.b[1931]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p917));
        }

        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p917)), 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && (!s.b[1930])) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1928]) && (!s.b[1929])) {
            s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p917), (((((-1.0)) * ((5.0 * p.p917)))) + ((1.0 + p.p917)))));
            s.store_mul_ad_product_rhs(523, 674, s.ad_value(508), A::add(s.ad_value(169), s.ad_value(510)));
        }

        if ((!s.b[1620]) && (!s.b[1928])) {
            s.store_scalar(523, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(524, 533, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(520, s.ad_value(521), 1.0, s.ad_value(522), 1.0, s.ad_value(523), 1.0, s.ad_value(524), 1.0);
            s.store_mul(511, 669, 464);
            s.store_mul(514, 670, 495);
            s.store_scale(517, 668, (s.v[189] * p.p2));
            s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));
        }

        s.b[1932] = (p.p914 == 1.0);
        s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1932]) {
            s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1932])) {
            s.store_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));
        }

        if (!s.b[1620]) {
            s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));
        }

        s.b[1933] = (p.p916 == 1.0);
        s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1933]) {
            s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1933])) {
            s.store_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));
        }

        if (!s.b[1620]) {
            s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));
        }

        s.b[1934] = (p.p918 == 1.0);
        s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1934]) {
            s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1934])) {
            s.store_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));
        }

        s.b[1935] = (s.v[511] > 0.0);
        s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1935]) {
            s.store_div(168, 499, 675);
        }

        s.b[1936] = (s.v[168] < 0.9);
        s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1935]) && s.b[1936]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1937] = (p.p914 != 1.0);
        s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };

        s.b[1938] = (p.p914 == 0.5);
        s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && s.b[1938]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p914));
        }

        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && (!s.b[1937])) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1935]) && (!s.b[1936])) {
            s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));
            s.store_mul_ad_product_rhs(526, 675, s.ad_value(511), A::add(s.ad_value(169), s.ad_value(513)));
        }

        if ((!s.b[1620]) && (!s.b[1935])) {
            s.store_scalar(526, 0.0);
        }

        s.b[1939] = (s.v[514] > 0.0);
        s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1939]) {
            s.store_div(168, 499, 676);
        }

        s.b[1940] = (s.v[168] < 0.9);
        s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1939]) && s.b[1940]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1941] = (p.p916 != 1.0);
        s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };

        s.b[1942] = (p.p916 == 0.5);
        s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p916));
        }

        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && (!s.b[1941])) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1939]) && (!s.b[1940])) {
            s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));
            s.store_mul_ad_product_rhs(527, 676, s.ad_value(514), A::add(s.ad_value(169), s.ad_value(516)));
        }

        if ((!s.b[1620]) && (!s.b[1939])) {
            s.store_scalar(527, 0.0);
        }

        s.b[1943] = (s.v[517] > 0.0);
        s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1943]) {
            s.store_div(168, 499, 677);
        }

        s.b[1944] = (s.v[168] < 0.9);
        s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1943]) && s.b[1944]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1945] = (p.p918 != 1.0);
        s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };

        s.b[1946] = (p.p918 == 0.5);
        s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && s.b[1946]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) {
            s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p918));
        }

        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && (!s.b[1945])) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {
            s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));
            s.store_mul_ad_product_rhs(528, 677, s.ad_value(517), A::add(s.ad_value(169), s.ad_value(519)));
        }

        if ((!s.b[1620]) && (!s.b[1943])) {
            s.store_scalar(528, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(529, 534, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(525, s.ad_value(526), 1.0, s.ad_value(527), 1.0, s.ad_value(528), 1.0, s.ad_value(529), 1.0);
        }

        s.b[1947] = (p.p28 != 0.0);
        s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1947]) {
            s.store_powf_ad(168, A::scale(s.ad_value(706), 1.0000000000000001e-23), p.p1144);
            s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p.p1145);
            s.store_div_scaled_product_right_ad(170, 379, A::voltage(ctx, nodes, Some(10), Some(7)), p.p1143, 271, 1.0);
        }

        if (!s.b[1620]) {
            s.store_div_scaled_inputs(607, s.ad_value(746), 2.0, s.ad_value(337), 1.0);
        }

        s.b[1948] = (p.p1011 <= 0.0);
        s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1948]) {
            s.store_scalar(610, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1948])) {
            s.store_div_scaled_offset_numerator(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, s.ad_value(607), 1.0);
            s.store_mul_ln_ad_rhs(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));
        }

        s.b[1949] = (s.v[610] < 0.0);
        s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1948])) && s.b[1949]) {
            s.store_scalar(610, 0.0);
        }

        if (!s.b[1620]) {
            s.store_mul_add_scaled_inputs_rhs(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), s.ad_value(709), 1.0 / (1.602176462e-19));
            s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(320), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);
            s.store_add_scaled_product_value_ad(1006, A::scale_offset(s.ad_value(612), p.p1013, p.p1012), 1.0, 612, 612, p.p1014);
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.b[1950] = (p.p1319 == 1.0);
        s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scalar(1014, p.p1320);
        }

        s.b[1951] = (s.v[184] > s.v[1014]);
        s.v[1951] = if s.b[1951] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1951]) {
            s.store_sub_from_scalar(167, s.v[184], 1014);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1951])) {
            s.store_scalar(1014, s.v[184]);
            s.copy_ad(167, 1014);
        }

        s.b[1952] = (p.p1015 >= (s.v[167] / 2.0));
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1952]) {
            s.store_scalar(606, 0.0);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1952])) {
            s.store_scalar(606, p.p1015);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scalar(1013, s.v[184]);
            s.store_div_scaled_inputs2(980, s.ad_value(221), 1.0, s.ad_value(707), (-1.0), s.ad_value(271), 1.0);
            s.store_scaled_sqrt_ad(981, A::div_from_scalar((((2.0 * 1.602176462e-19) * s.v[180]) * p.p1322), s.ad_value(271)), 1.0 / (s.v[199]));
            s.store_ln_ad(982, A::div_from_scalar(p.p1322, s.ad_value(182)));
            s.store_scalar(168, 1.0);
            s.store_div(404, 980, 168);
            s.store_div(405, 981, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1953] = (s.v[404] < 0.0);
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1953]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(983, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1953])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(983, A::square(s.ad_value(169)), 1.0, 170);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(983), (-1.0), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(981), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(981), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(983), 1.0, s.ad_value(982), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1954] = (s.v[175] <= (-68.0));
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1955] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && s.b[1955]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1956] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && s.b[1956]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && (!s.b[1956])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(985, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(985, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scaled_add_ad(984, A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(983), (-1.0), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1950]) {
            s.store_offset_div_scaled_inputs(986, s.ad_value(981), 1.0, A::sqrt(s.ad_value(984)), 2.0, 1.0);
            s.copy_ad(987, 337);
            s.store_scale(994, 987, (s.v[199] * s.v[183]));
            s.store_scale(993, 337, (s.v[199] * s.v[183]));
            s.store_div_scaled_product_by_product(988, s.ad_value(380), s.ad_value(1014), 1.0, A::mul3_scaled_output(s.ad_value(986), s.ad_value(994), s.ad_value(271), 2.0), s.ad_value(271), 1.0);
            s.store_div_scaled_product_by_product(990, s.ad_value(380), A::sub(s.ad_value(1013), s.ad_value(1014)), 1.0, A::mul3_scaled_output(s.ad_value(253), s.ad_value(993), s.ad_value(269), 2.0), s.ad_value(269), 1.0);
            s.store_add_scaled_inputs3_offset(167, A::square(s.ad_value(985)), 4.0, s.ad_value(985), 4.0, s.ad_value(988), (-4.0), 1.0);
            s.store_offset_scaled_ad(991, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(320)), 4.0, s.ad_value(320), 4.0, s.ad_value(990), 4.0), 1.0)), 0.5, (-0.5));
        }

        s.b[1958] = (s.v[184] != s.v[1014]);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {
            s.store_mul3_affine_lhs(992, 253, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 991);
            s.store_add_scaled_inputs3(608, s.ad_value(1013), 1.0, s.ad_value(606), (-2.0), s.ad_value(1014), -1.0);
            s.store_square(609, 608);
            s.store_scale(168, 609, (10000000000.0 * s.v[199]));
            s.store_scaled_ln_ad(169, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(992), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(170, 992, 612, p.p1013);
            s.store_scaled_sub_ad(171, A::square(s.ad_value(992)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(172, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(1000, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(172), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(168)), A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(170), 1.0, s.ad_value(171), 1.0), 1.0);
            s.store_mul3_affine_lhs(173, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(1001, A::div(s.ad_value(1008), s.ad_value(173)), s.ad_value(380), 380);
            s.store_add(174, 1001, 1000);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scale(175, 271, (p.p1321 * 1.602176462e-19));
            s.store_mul3_affine_lhs(176, 1014, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(1009, A::div(s.ad_value(175), s.ad_value(176)), s.ad_value(380), 380);
            s.copy_ad(177, 1009);
        }

        s.b[1961] = (p.p1015 >= (s.v[184] / 2.0));
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1961]) {
            s.store_scalar(606, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1950])) && (!s.b[1961])) {
            s.store_scalar(606, p.p1015);
        }

        s.b[1962] = (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0));
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) {
            s.store_sub_from_scalar_scaled_input(608, s.v[184], 606, 2.0);
            s.store_square(609, 608);
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
            s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(400), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(169, 611, 612, p.p1013);
            s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(615, A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(380), 380);
            s.store_add(173, 615, 614);
        }

        if (!s.b[1620]) {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
            s.store_square(168, 167);
            s.store_offset_scaled(170, 168, (((p.p1022 * s.v[184])) * (p.p1019)), p.p1019);
            s.store_offset_scaled(171, 168, (((p.p1023 * s.v[184])) * (p.p1020)), p.p1020);
            s.store_offset_scaled(172, 168, (((p.p1298 * s.v[184])) * (p.p1297)), p.p1297);
            s.store_square(633, 172);
            s.store_square(632, 171);
        }

        s.b[1964] = (p.p39 == 0.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        s.b[1965] = (p.p39 == 1.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1964]) {
            s.store_scaled_mul(388, 271, 382, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_scaled_mul(389, 271, 385, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_mul_abs_ad_rhs(167, 337, A::add(s.ad_value(388), s.ad_value(389)));
            s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_scaled_mul(626, 253, 269, 2.0);
            s.store_mul_scale_ad_lhs(167, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 626);
            s.store_scaled_add(168, 400, 320, 0.5);
            s.store_offset(170, 168, 0.5);
            s.store_square(171, 170);
            s.store_mul(172, 171, 170);
            s.store_sub(173, 400, 320);
            s.store_square(174, 173);
            s.store_mul(175, 174, 173);
            s.store_mul_scale_offset_rhs(176, 174, 168, 6.0, 0.5);
            s.store_scale(625, 345, s.v[184]);
            s.store_scale(177, 625, 1.0 / (s.v[184]));
            s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(316), A::offset(s.ad_value(243), p.p1299), 1.0), 1.0);
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_offset_scaled(179, 179, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_scaled_add_ad_rhs(179, 179, A::sqrt(A::offset(A::mul(s.ad_value(179), s.ad_value(179)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            let assign54870_ad_e89925: A = A::mul3_scaled_output(A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), s.ad_value(632), (15.0 * 1.0 / (4.0)));
            s.store_div_scaled_inputs(622, assign54870_ad_e89925, 1.0, s.ad_value(167), ((p.p2 * s.v[183]) * 12.0));
        }

        if (!s.b[1620]) {
            s.copy_ad(217, 213);
            s.store_scalar(418, 0.0);
        }

        s.b[1970] = (p.p31 == 1.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_offset(793, 793, p.p25);
            s.store_mul(222, 221, 272);
            s.store_mul(225, 224, 272);
            s.store_mul(212, 793, 272);
            s.store_sub(217, 222, 212);
            s.store_ln_ad(432, A::max_with_scalar(A::div(s.ad_value(794), s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_ad(433, A::mul_scaled_lhs(s.ad_value(794), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(272)), 1.0 / (s.v[199]));
            s.store_div_from_scalar(295, 1.0, 433);
            s.store_div_scaled_inputs(406, s.ad_value(704), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(271), (s.v[199] * s.v[199]));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            if (s.v[704] > 0.0) {
                s.store_div_from_scalar(418, 1.0, 406);
            } else {
                s.store_scalar(418, 0.0);
            }
        }

        if ((!s.b[1620]) && s.b[1970]) {
            if (s.v[704] > 0.0) {
                s.store_div(403, 794, 704);
            } else {
                s.store_scalar(403, 0.0);
            }
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_offset(168, 403, 1.0);
            s.store_div(404, 217, 168);
            s.store_div(405, 433, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1971] = (s.v[404] < 0.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1971]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1971])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(254, A::square(s.ad_value(169)), 1.0, 170);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(433), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1972] = (s.v[175] <= (-68.0));
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1973] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && s.b[1973]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1974] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && s.b[1974]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && (!s.b[1974])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(400, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1972])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(400, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1970]) {
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_div_ad(253, s.ad_value(433), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul3_affine_lhs(306, 253, 271, 2.0, 0.0, 400);
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_scaled_product_by_product(313, s.ad_value(740), s.ad_value(271), 1.0, s.ad_value(309), s.ad_value(655), s.v[188]);
            s.store_div_scaled_product_offset_denominator(307, s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 1.0, A::mul_offset_rhs(s.ad_value(313), s.ad_value(400), 1.0), 1.0, 1.0);
            s.store_add_scaled_inputs4(321, s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(307), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(433), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_mul(322, 321, 271);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(317, 322, 0.5, 224, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(224)), A::sub(s.ad_value(322), s.ad_value(224))), ((0.25 * 0.001) * 0.001)), 0.5);
        }

        s.b[1975] = ((p.p1353 == 0.0) && (p.p1354 == 0.0));
        s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1975]) {
            s.store_scalar(1020, p.p1348);
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1975])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_div_scaled_inputs2(1020, s.ad_value(168), p.p1353, A::mul3_scaled_output(s.ad_value(168), s.ad_value(400), s.ad_value(269), p.p1354), (-1.0), A::scale_offset(s.ad_value(218), p.p1355, 1.0), 1.0, 1.0);
            s.store_scaled_add_ad(1020, A::offset(s.ad_value(1020), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1020), (-0.1), A::offset(s.ad_value(1020), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_div(317, 317, 1020);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 272);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(433), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1976] = (s.v[175] <= (-68.0));
        s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1977] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && s.b[1977]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1978] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && s.b[1978]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && (!s.b[1978])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(320, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1976])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(320, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(400), (-1.0), s.ad_value(320), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(433), A::add(s.ad_value(259), s.ad_value(169))));
            s.store_offset_product3(171, s.ad_value(403), s.ad_value(169), s.ad_value(295), 1.0, 0.5);
            s.store_sqrt_add_ad(172, A::square(s.ad_value(171)), A::mul3(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(418)));
            s.store_div_add_scaled_inputs_rhs_indices(253, 170, 171, 1.0, 172, 1.0);
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul_ad_rhs(168, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(320), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(248, 168, A::sqrt(A::offset(A::mul(s.ad_value(168), s.ad_value(168)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_scaled_add(249, 247, 248, 0.5);
            s.store_mul_ad_product_rhs(243, 253, s.ad_value(271), A::add(s.ad_value(400), s.ad_value(320)));
            s.store_mul_add_scaled_inputs_rhs(336, 335, s.ad_value(249), 1.0, s.ad_value(243), s.v[338]);
            s.store_offset(168, 403, 1.0);
            s.store_div_scaled_inputs2(404, s.ad_value(217), 1.0, s.ad_value(272), p.p139, s.ad_value(168), 1.0);
            s.store_div(405, 433, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1979] = (s.v[404] < 0.0);
        s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1979]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1979])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(254, A::square(s.ad_value(169)), 1.0, 170);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(339, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_scaled_product_by_product(314, s.ad_value(740), s.ad_value(271), 2.0, s.ad_value(339), s.ad_value(655), s.v[188]);
            s.store_sub(250, 400, 320);
            s.store_mul_ad_affine_product_rhs(168, 314, s.ad_value(250), A::mul(s.ad_value(314), s.ad_value(250)), 2.0, 0.0);
            s.store_sqrt_offset_input(342, 168, 1.0);
            s.store_scaled_offset(343, 342, 1.0, 0.5);
            s.store_div_scaled_inputs(310, s.ad_value(655), 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);
            s.store_scale(311, 310, s.v[188]);
            s.store_add(358, 317, 311);
            s.store_sub(355, 226, 315);
        }

        s.b[1980] = (s.v[786] != 0.0);
        s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1980]) {
            s.store_offset_mul_ad(364, s.ad_value(786), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(786), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1980])) {
            s.store_scalar(364, 1.0);
        }

        if (!s.b[1620]) {
            s.store_square(407, 364);
            s.store_div_from_scalar(408, 1.0, 364);
            s.store_div_from_scalar(409, 1.0, 407);
            s.store_offset(410, 364, (-1.0));
            s.store_sub(413, 217, 254);
            s.store_sub(416, 400, 320);
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_add_scaled_inputs(411, 413, 1.0, 400, 2.0);
            s.store_add_scaled_inputs(412, 413, 1.0, 320, 2.0);
            s.store_scaled_add_ad_rhs(168, 411, A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_scaled_add_ad_rhs(169, 412, A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_sqrt_offset_ad(414, A::mul(s.ad_value(168), s.ad_value(418)), 0.25);
            s.store_sqrt_offset_ad(415, A::mul(s.ad_value(169), s.ad_value(418)), 0.25);
            s.store_div_ad_rhs(168, 411, A::scale_offset(s.ad_value(414), 2.0, 1.0));
            s.store_div_ad_rhs(169, 412, A::scale_offset(s.ad_value(415), 2.0, 1.0));
            s.store_add(170, 414, 415);
            s.store_scaled_div_ad_rhs(171, 417, A::mul(A::square(s.ad_value(170)), s.ad_value(170)), 0.3333333333333333);
            s.store_div_scaled_product3_mixed_iiia(172, 1020, 343, 408, 1.0, A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)), 1.0);
            s.store_mul_scale_ad_lhs(173, A::add_scaled_square_product(s.ad_value(170), 1.0, s.ad_value(414), s.ad_value(415), 1.0), 0.8, 172);
            s.store_add_scaled_inputs(174, 173, 1.0, 418, 2.0);
            s.store_scaled_mul(175, 417, 172, 0.3333333333333333);
            s.store_div_scaled_product_mixed_iaa(402, 412, A::scale_offset(s.ad_value(415), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(415), 2.0, 1.0), 1.0);
            s.store_add_ad_lhs(401, A::add_scaled_offset_product_lhs(s.ad_value(413), 1.0, s.ad_value(253), (-1.0), s.ad_value(320), (-2.0)), 402);
            s.store_add_scaled_products_left_right_ad(381, 408, A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, A::add_scaled_products(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(253), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(175), 1.0), (-1.0)), 1.0), 1.0, 410, 401, 1.0);
            s.store_add(176, 400, 320);
            s.store_mul3_lhs(177, 417, 172, 172);
            s.store_add_ad(386, A::mul3(s.ad_value(253), s.ad_value(408), A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(417), s.ad_value(172), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(253), s.ad_value(410), s.ad_value(320), 2.0));
            s.store_mul_ad_product_rhs(383, 253, s.ad_value(409), A::add_scaled_product(s.ad_value(176), 0.5, s.ad_value(416), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), 1.0, s.ad_value(177), 0.2), (-1.0 / (6.0))));
            s.store_mul_ad_product_lhs(384, s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408)), 320);
            s.store_add(385, 383, 384);
            s.store_sub(382, 386, 385);
            s.store_add_scaled_product_value_ad(246, A::sqrt(A::offset(A::mul3(s.ad_value(271), s.ad_value(381), A::mul(s.ad_value(271), s.ad_value(381))), ((0.25 * 0.1) * 0.1))), 0.5, 271, 381, 0.5);
            s.store_mul_add_rhs(245, 271, 382, 385);
            s.store_add_scaled_inputs(167, 245, 1.0 / (p.p230), 246, (p.p231 * 1.0 / (p.p230)));
            s.store_offset_powf_ad(168, s.ad_value(167), (0.7 * p.p229), 1.0);
            s.store_div_from_scalar(427, (p.p228 * 1.9e-9), 168);
        }

    }

    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add_scaled_inputs(s.ad_value(429), (3.9 * 1.0 / (p.p110)), s.ad_value(427), 1.0 / (s.v[200])));
            s.store_mul_ad_affine_product_lhs(387, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), s.ad_value(271), (-(((p.p2 * s.v[187]) * s.v[188]) + p.p1379)), 0.0, 381);
            s.store_scaled_mul(391, 428, 271, (((p.p2 * s.v[187]) * s.v[188]) + p.p1379));
            s.store_mul_neg_lhs(389, 391, 385);
            s.store_mul_neg_lhs(388, 391, 382);
        }

        s.b[1981] = (p.p45 == 1.0);
        s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scalar(795, (p.p140 + p.p25));
            s.store_mul(231, 230, 272);
            s.store_mul(233, 232, 272);
            s.store_mul(212, 795, 272);
            s.store_sub(434, 231, 212);
            s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141), 1.0 / (s.v[199]));
            s.store_div_from_scalar(295, 1.0, 436);
            s.store_scalar(418, 0.0);
            s.store_scalar(403, 0.0);
            s.store_offset(168, 403, 1.0);
            s.store_div(404, 434, 168);
            s.store_div(405, 436, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1982] = (s.v[404] < 0.0);
        s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1982]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1982])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(254, A::square(s.ad_value(169)), 1.0, 170);
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(436), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(436), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(233), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1983] = (s.v[175] <= (-68.0));
        s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1983]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1984] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && s.b[1984]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1985] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && (!s.b[1984])) && s.b[1985]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && s.b[1983]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(400, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1983])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(400, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_div_ad(253, s.ad_value(436), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(434), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul3_affine_lhs(306, 253, 271, 2.0, 0.0, 400);
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(241), 1.0), A::pow(s.ad_value(308), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_scaled_product_by_product(313, s.ad_value(740), s.ad_value(271), 1.0, s.ad_value(309), s.ad_value(655), s.v[188]);
            s.store_div_scaled_product_offset_denominator(307, s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 1.0, A::mul_offset_rhs(s.ad_value(313), s.ad_value(400), 1.0), 1.0, 1.0);
            s.store_add_scaled_inputs4(321, s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(307), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(436), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_mul(322, 321, 271);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(317, 322, 0.5, 232, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(232)), A::sub(s.ad_value(322), s.ad_value(232))), ((0.25 * 0.001) * 0.001)), 0.5);
        }

        if ((!s.b[1620]) && s.b[1981]) {
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 232, 272);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(436), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(436), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1986] = (s.v[175] <= (-68.0));
        s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1986]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1987] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && s.b[1987]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1988] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && (!s.b[1987])) && s.b[1988]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && s.b[1986]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(320, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1986])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(320, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(400), (-1.0), s.ad_value(320), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(436), A::add(s.ad_value(259), s.ad_value(169))));
            s.store_offset_product3(171, s.ad_value(403), s.ad_value(169), s.ad_value(295), 1.0, 0.5);
            s.store_sqrt_add_ad(172, A::square(s.ad_value(171)), A::mul3(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(418)));
            s.store_div_add_scaled_inputs_rhs_indices(253, 170, 171, 1.0, 172, 1.0);
            s.store_scalar(364, 1.0);
            s.store_square(407, 364);
            s.store_div_from_scalar(408, 1.0, 364);
            s.store_div_from_scalar(409, 1.0, 407);
            s.store_offset(410, 364, (-1.0));
            s.store_sub(413, 434, 254);
            s.store_sub(416, 400, 320);
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_add_scaled_inputs(411, 413, 1.0, 400, 2.0);
            s.store_add_scaled_inputs(412, 413, 1.0, 320, 2.0);
            s.store_scaled_add_ad_rhs(168, 411, A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_scaled_add_ad_rhs(169, 412, A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_sqrt_offset_ad(414, A::mul(s.ad_value(168), s.ad_value(418)), 0.25);
            s.store_sqrt_offset_ad(415, A::mul(s.ad_value(169), s.ad_value(418)), 0.25);
            s.store_div_ad_rhs(168, 411, A::scale_offset(s.ad_value(414), 2.0, 1.0));
            s.store_div_ad_rhs(169, 412, A::scale_offset(s.ad_value(415), 2.0, 1.0));
            s.store_add(170, 414, 415);
        }

    }

    pub(super) fn stamp_reactive_block_41(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scaled_div_ad_rhs(171, 417, A::mul(A::square(s.ad_value(170)), s.ad_value(170)), 0.3333333333333333);
            s.store_scalar(343, 0.0);
            s.store_div_scaled_product_add_scaled_denominator_mixed_ai(172, 343, 408, 1.0, A::offset(s.ad_value(400), 1.0), 1.0, 320, 1.0, 1.0);
            s.store_mul_scale_ad_lhs(173, A::add_scaled_square_product(s.ad_value(170), 1.0, s.ad_value(414), s.ad_value(415), 1.0), 0.8, 172);
            s.store_add_scaled_inputs(174, 173, 1.0, 418, 2.0);
            s.store_scaled_mul(175, 417, 172, 0.3333333333333333);
            s.store_div_scaled_product_mixed_iaa(402, 412, A::scale_offset(s.ad_value(415), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(415), 2.0, 1.0), 1.0);
            s.store_add_ad_lhs(401, A::add_scaled_offset_product_lhs(s.ad_value(413), 1.0, s.ad_value(253), (-1.0), s.ad_value(320), (-2.0)), 402);
            s.store_add_scaled_products_left_right_ad(381, 408, A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, A::add_scaled_products(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(253), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(175), 1.0), (-1.0)), 1.0), 1.0, 410, 401, 1.0);
            s.store_add(176, 400, 320);
            s.store_mul3_lhs(177, 417, 172, 172);
            s.store_add_ad(386, A::mul3(s.ad_value(253), s.ad_value(408), A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(417), s.ad_value(172), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(253), s.ad_value(410), s.ad_value(320), 2.0));
            s.store_mul_ad_product_rhs(383, 253, s.ad_value(409), A::add_scaled_product(s.ad_value(176), 0.5, s.ad_value(416), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), 1.0, s.ad_value(177), 0.2), (-1.0 / (6.0))));
            s.store_mul_ad_product_lhs(384, s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408)), 320);
            s.store_add(385, 383, 384);
            s.store_mul_scale_ad_lhs(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380, 271);
            s.store_mul(440, 437, 381);
            s.store_mul(439, 437, 385);
            s.store_mul(438, 437, 386);
        }

        if ((!s.b[1620]) && (!s.b[1981])) {
            s.store_scalar(440, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(438, 0.0);
        }

        if (!s.b[1620]) {
            s.copy_ad(394, 389);
            s.copy_ad(395, 388);
            s.copy_ad(393, 387);
            s.store_neg_ad(392, A::add_scaled_inputs3(s.ad_value(393), 1.0, s.ad_value(395), 1.0, s.ad_value(394), 1.0));
            s.store_neg_ad(398, A::scale(s.ad_value(439), p.p45));
            s.store_neg_ad(399, A::sub_scaled_inputs(s.ad_value(438), p.p45, s.ad_value(439), p.p45));
            s.store_neg_ad(397, A::scale(s.ad_value(440), p.p45));
            s.store_neg_ad(396, A::add_scaled_inputs3(s.ad_value(397), 1.0, s.ad_value(399), 1.0, s.ad_value(398), 1.0));
            s.store_neg_ad(389, A::sub(A::scale(s.ad_value(439), p.p45), s.ad_value(389)));
            s.store_neg_ad(388, A::add_scaled_inputs3(s.ad_value(438), p.p45, s.ad_value(388), (-1.0), s.ad_value(439), (-p.p45)));
            s.store_neg_ad(387, A::sub(A::scale(s.ad_value(440), p.p45), s.ad_value(387)));
            s.store_neg_ad(390, A::add_scaled_inputs3(s.ad_value(387), 1.0, s.ad_value(388), 1.0, s.ad_value(389), 1.0));
        }

        s.b[1989] = (!param_given[867]);
        s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1989]) {
            s.store_scalar(788, ((((2.0 * p.p110) * 8.8541878128e-12) / 3.141592653589793) * ((((p.p871 * (1.0 + (4e-7 / p.p76)))).max(1e-38)) as f64).ln()));
        }

        if (!s.b[1620]) {
            s.store_offset(425, 788, p.p872);
            s.store_offset(426, 788, p.p873);
        }

        s.b[1990] = (p.p32 == 0.0);
        s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1990]) {
            s.store_scaled_mul(423, 425, 431, ((-s.v[187]) * p.p2));
            s.store_scaled_mul(424, 426, 430, ((-s.v[187]) * p.p2));
        }

        if ((!s.b[1620]) && (!s.b[1990])) {
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(431), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(419, s.ad_value(431), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 419, A::powf(A::offset(A::powf(A::scale(s.ad_value(419), (-1.0 / (p.p893))), p.p894), 1.0), (1.0 / p.p894)));
            s.store_sqrt_sub_from_scalar_ad(168, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(791), 1.0));
            s.store_add_scaled_products_right_right_ad(423, 425, 431, ((-s.v[187]) * p.p2), 789, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(431), 1.0, s.ad_value(219), (-1.0), s.ad_value(419), -1.0), 1.0, s.ad_value(791), s.ad_value(168), (-1.0), (-0.5)), ((-s.v[187]) * p.p2));
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(430), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(420, s.ad_value(430), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 420, A::powf(A::offset(A::powf(A::scale(s.ad_value(420), (-1.0 / (p.p891))), p.p892), 1.0), (1.0 / p.p892)));
            s.store_sqrt_sub_from_scalar_ad(169, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(792), 1.0));
            s.store_add_scaled_products_right_right_ad(424, 426, 430, ((-s.v[187]) * p.p2), 790, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(219), (-1.0), s.ad_value(420), -1.0), 1.0, s.ad_value(792), s.ad_value(169), (-1.0), (-0.5)), ((-s.v[187]) * p.p2));
        }

        if (!s.b[1620]) {
            s.store_mul_scaled_voltage(421, 379, (((-p.p2) * s.v[188]) * p.p874), ctx, nodes, Some(9), Some(10));
            s.store_neg_ad(422, A::add_scaled_inputs3(s.ad_value(423), 1.0, s.ad_value(424), 1.0, s.ad_value(421), 1.0));
            s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p.p1394));
            s.store_offset(1036, 1035, (2.0 * p.p1393));
        }

        s.b[1991] = (s.v[908] > 0.0);
        s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1991]) {
            s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if ((!s.b[1620]) && (!s.b[1991])) {
            s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product_by_product(s.ad_value(706), s.ad_value(908), -1.0, s.ad_value(182), s.ad_value(182), 1.0), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if (!s.b[1620]) {
            s.store_sub(1032, 235, 215);
            s.store_scalar(1034, (3.453133e-11 / p.p75));
            s.store_mul_ad_affine_product_rhs(1037, 909, s.ad_value(1034), A::scale_offset(s.ad_value(1036), ((s.v[187] / p.p1373) * p.p2), p.p1382), p.p1388, 0.0);
            s.store_mul_sub_rhs(1038, 1037, 1032, 1033);
            s.copy_ad(1039, 1038);
            s.store_scalar(167, (p.p1395 * ((((p.p871 * (1.0 + (p.p74 / p.p75)))).max(1e-38)) as f64).ln()));
            s.store_scalar(168, (p.p19 - p.p1));
        }

        s.b[1992] = (s.v[168] > 0.0);
        s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1992]) {
            s.store_mul(1040, 167, 168);
        }

        if ((!s.b[1620]) && (!s.b[1992])) {
            s.store_scalar(1040, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scalar(168, (p.p20 - p.p1));
        }

        s.b[1993] = (s.v[168] > 0.0);
        s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1993]) {
            s.store_mul(1041, 167, 168);
        }

        if ((!s.b[1620]) && (!s.b[1993])) {
            s.store_scalar(1041, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(1042, 1034, p.p17);
            s.store_scalar(1043, (p.p1396 * p.p17));
            s.store_scale(1044, 1034, p.p18);
            s.store_scalar(1045, (p.p1396 * p.p18));
            s.store_mul_neg_lhs(177, 379, 236);
            s.store_mul_neg_lhs(178, 379, 237);
        }

        s.b[1994] = (p.p1396 != 0.0);
        s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1994]) {
            s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p.p1399)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(178), (-p.p1399), p.p1400)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1044), s.ad_value(1045)), 0.5, 178);
            s.store_add_scaled_product_indices(1047, 170, 1.0, 168, 169, 1.0);
            s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p.p1397)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(177), (-p.p1397), p.p1398)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1042), s.ad_value(1043)), 0.5, 177);
            s.store_add_scaled_product_indices(1046, 170, 1.0, 168, 169, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1994])) {
            s.store_mul(1046, 1042, 177);
            s.store_mul(1047, 1044, 178);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_product_indices(1046, 1046, 1.0, 1040, 177, 1.0);
            s.store_add_scaled_product_indices(1047, 1047, 1.0, 1041, 178, 1.0);
        }

        s.b[1995] = (p.p27 == 1.0);
        s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
            s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19));
            s.store_mul_add_scaled_inputs_rhs(941, 835, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5);
            s.store_mul_offset_ad_rhs(940, 841, A::mul_offset_rhs(s.ad_value(848), s.ad_value(639), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_offset(273, s.ad_value(298), 0.5, s.ad_value(218), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05), A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
            s.store_mul(275, 277, 274);
            s.store_div_from_scalar(260, s.v[180], 275);
            s.store_add_scaled_inputs_products_indices(276, 836, 1.0, 941, 1.0, 838, 227, 1.0, 840, 218, (-1.0));
            s.store_offset_scaled(168, 276, 1.0 / (s.v[199]), 1.0);
            s.store_scaled_add_ad(267, A::offset(s.ad_value(168), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(168), (-1.0), A::offset(s.ad_value(168), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
            s.store_mul(269, 267, 271);
            s.store_div_from_scalar(270, 1.0, 269);
            s.store_mul(222, 221, 270);
            s.store_mul(225, 224, 270);
            s.store_mul(212, 707, 270);
            s.store_mul_neg_ad_lhs(944, A::add_scaled_product(s.ad_value(940), 1.0, s.ad_value(842), s.ad_value(218), 1.0), 227);
            s.store_mul_offset_rhs_ad(293, A::add_scaled_inputs_product(s.ad_value(843), 1.0, s.ad_value(844), 1.0 / (s.v[184]), s.ad_value(845), s.ad_value(218), 1.0), A::pow(s.ad_value(639), s.ad_value(846)), (-1.0));
            s.store_mul_scale_offset_rhs(946, 300, 218, p.p1264, 1.0);
        }

        s.b[1996] = (s.v[946] > 0.0);
        s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1996]) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.b[1997] = (s.v[167] < 40.0);
        s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1996]) && s.b[1997]) {
            s.store_div_from_scalar_offset_ad(943, (0.5 * p.p1262), A::cosh(s.ad_value(167)), (-1.0));
        }

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) {
            s.store_scaled_limited_exp_scaled_input(943, 167, -1.0, p.p1262);
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1996])) {
            s.store_scalar(943, 0.0);
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_mul_sub_rhs(945, 943, 942, 298);
            s.store_add_ad_lhs(242, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(944), 1.0, s.ad_value(293), (-1.0), s.ad_value(945), 1.0, s.ad_value(956), 1.0, p.p1151), 1.0, s.ad_value(849), s.ad_value(218), (-1.0)), 932);
            s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
            s.store_scaled_sqrt_ad(954, A::mul_scaled_lhs(s.ad_value(953), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(270)), 1.0 / (s.v[199]));
            s.store_mul_offset_rhs(954, 954, 947, 1.0);
            s.store_div(952, 951, 267);
            s.store_scalar(168, 1.0);
            s.store_div(404, 213, 168);
            s.store_div(405, 954, 168);
        }

    }

    pub(super) fn stamp_reactive_block_42(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[1620]) && s.b[1995]) {
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1998] = (s.v[404] < 0.0);
        s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1998]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1998])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(254, A::square(s.ad_value(169)), 1.0, 170);
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(954), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(954), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(952), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1999] = (s.v[175] <= (-68.0));
        s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1999]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[2000] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && s.b[2000]) {
            s.store_limited_exp(170, 171);
        }

        s.b[2001] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && (!s.b[2000])) && s.b[2001]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && (!s.b[2000])) && (!s.b[2001])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && s.b[1999]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(961, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1999])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(961, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_add_scaled_product_indices(948, 269, 2.0, 269, 961, 2.0);
            s.copy_ad(949, 948);
            s.store_add(949, 949, 224);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(950, 949, 0.5, 224, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(949), s.ad_value(224)), A::sub(s.ad_value(949), s.ad_value(224))), ((0.25 * 0.001) * 0.001)), 0.5);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(950)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(954), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(954), 1.0);
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(952), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[2002] = (s.v[175] <= (-68.0));
        s.v[2002] = if s.b[2002] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[2002]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[2003] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && s.b[2003]) {
            s.store_limited_exp(170, 171);
        }

        s.b[2004] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && (!s.b[2003])) && s.b[2004]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && (!s.b[2003])) && (!s.b[2004])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && s.b[2002]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(960, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[2002])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(960, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(961), (-1.0), s.ad_value(960), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_offset_div_ad(959, s.ad_value(954), A::add(s.ad_value(259), s.ad_value(169)), 1.0);
            s.store_mul_product3_rhs(939, 363, A::mul3_scaled_output(s.ad_value(959), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((p.p1147 * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(961), s.ad_value(960)), A::add(A::offset(s.ad_value(961), 1.0), s.ad_value(960))), 1.0);
            s.store_add(380, 939, 380);
            s.store_scalar(964, (p.p1012 * p.p1316));
            s.store_scalar(965, (p.p1013 * p.p1316));
            s.store_scalar(966, (p.p1014 * p.p1316));
            s.store_sub_from_scalar_scaled_input(962, s.v[184], 606, 2.0);
            s.store_square(963, 962);
            s.store_mul_add_scaled_inputs_rhs(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), s.ad_value(836), 1.0 / (1.602176462e-19));
            s.store_mul3_affine_lhs(612, 959, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 960);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(939)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 939, 1.602176462e-19, 0.0, 939);
            s.store_add_ad(1006, A::add_scaled_product(s.ad_value(964), 1.0, s.ad_value(965), s.ad_value(612), 1.0), A::mul3(s.ad_value(966), s.ad_value(612), s.ad_value(612)));
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scaled_mul(1008, 964, 271, 1.602176462e-19);
            s.store_mul3_affine_lhs(611, 959, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 961);
            s.store_mul_ln_ad_rhs(168, 964, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38));
            s.store_mul_sub_rhs(169, 965, 611, 612);
            s.store_mul_sub_scaled_inputs_rhs(170, 966, A::square(s.ad_value(611)), 0.5, A::square(s.ad_value(612)), 0.5);
            s.store_scale(171, 963, (10000000000.0 * (p.p1147 * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 962, 613, ((p.p1147 * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(615, A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(939), 939);
            s.store_add(173, 615, 614);
        }

        s.b[2005] = (s.v[173] > 0.0);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[2005]) {
            s.store_div_scaled_product_indices(174, 614, 615, 1.0, 173, 1.0);
            s.store_offset_scaled_ad(175, A::powf(A::sub(s.ad_value(961), s.ad_value(960)), p.p1318), p.p1317, 1.0);
        }

        if (!s.b[1620]) {
            s.store_mul_ad_rhs(1075, 379, A::add_scaled_inputs4(s.ad_value(387), 1.0, s.ad_value(421), 1.0, s.ad_value(520), 1.0, s.ad_value(525), 1.0));
        }

        s.b[2006] = (s.v[211] > 0.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[2006]) {
            s.store_mul(1050, 379, 388);
            s.store_mul(1051, 379, 395);
            s.store_mul(1052, 379, 399);
            s.store_mul(1053, 379, 389);
            s.store_mul(1054, 379, 394);
            s.store_mul(1055, 379, 398);
            s.store_mul_add_scaled_inputs3_offset_rhs(1076, 379, s.ad_value(388), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0, 0.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(1077, 379, s.ad_value(389), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[2006])) {
            s.store_mul(1050, 379, 389);
            s.store_mul(1051, 379, 394);
            s.store_mul(1052, 379, 398);
            s.store_mul(1053, 379, 388);
            s.store_mul(1054, 379, 395);
            s.store_mul(1055, 379, 399);
            s.store_mul_add_scaled_inputs3_offset_rhs(1076, 379, s.ad_value(389), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0, 0.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(1077, 379, s.ad_value(388), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0, 0.0);
        }

        if (!s.b[1620]) {
            s.store_mul_add_rhs(1078, 379, 390, 422);
            s.store_mul(1057, 379, 392);
            s.store_mul(1058, 379, 396);
        }

        s.v[1108] = s.v[183];

        s.v[1109] = s.v[184];

        s.b[2021] = ((p.p41 != 0.0) && (p.p1099 > 0.0));
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        if s.b[2021] {
            s.store_mul_voltage_ad(1017, A::mul3(s.ad_value(379), s.ad_value(211), s.ad_value(380)), ctx, nodes, Some(6), Some(7));
        }

        s.b[2022] = ((p.p33 != 2.0) && (s.v[453] > 0.0));
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[2021] && s.b[2022]) {
            s.store_add_ad_rhs(1017, 1017, A::div_scaled_product(A::voltage(ctx, nodes, Some(0), Some(6)), A::voltage(ctx, nodes, Some(0), Some(6)), 1.0, s.ad_value(455), 1.0));
        }

        s.b[2023] = ((p.p33 != 2.0) && (s.v[452] > 0.0));
        s.v[2023] = if s.b[2023] { 1.0 } else { 0.0 };

        if (s.b[2021] && s.b[2023]) {
            s.store_add_ad_rhs(1017, 1017, A::div_scaled_product(A::voltage(ctx, nodes, Some(2), Some(7)), A::voltage(ctx, nodes, Some(2), Some(7)), 1.0, s.ad_value(454), 1.0));
        }

        s.b[2024] = ((p.p40 != 0.0) && (!true));
        s.v[2024] = if s.b[2024] { 1.0 } else { 0.0 };

        s.b[2025] = true;
        s.v[2025] = if s.b[2025] { 1.0 } else { 0.0 };

        s.v[1024] = (p.p1359 * p.p1358);

        s.b[2028] = ((p.p43 == 0.0) || (!true));
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        s.b[2029] = ((p.p40 != 0.0) && (!true));
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        s.b[2030] = (p.p43 == 1.0);
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        if (((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) {
            s.store_scalar(1025, ((((((p.p1357 * p.p1356) * p.p1360) / ((2.0 * p.p1356) + (p.p1360 * s.v[1109]))) * s.v[1108]) / p.p1373) / p.p2));
        }

        s.b[2031] = (s.v[1025] < 0.001);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        s.b[2032] = (s.v[1024] <= 0.001);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        if (((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && s.b[2031]) && s.b[2032]) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && s.b[2031]) && (!s.b[2032])) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        if (((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) {
            s.store_mul_pow_ad_rhs(1027, 1028, s.ad_value(639), s.ad_value(1029));
            s.store_add_scaled_inputs4(1026, s.ad_value(387), -1.0, s.ad_value(520), -1.0, s.ad_value(525), -1.0, s.ad_value(1039), 1.0);
            s.store_sub_scaled_inputs(1031, 1030, (1.602176462e-19 * (p.p74 * (s.v[1108] * s.v[1109]))), 1026, 1.0);
            s.store_mul(167, 1027, 1031);
            s.store_scalar(168, (s.v[1108] * s.v[1108]));
            s.store_div_scaled_inputs(1023, s.ad_value(167), p.p2, s.ad_value(168), 1.0);
            s.store_div_from_scalar(1025, 1.0, 1023);
        }

        s.b[2033] = (s.v[1025] < 0.001);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        s.b[2034] = (s.v[1024] <= 0.001);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        if (((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2033]) && s.b[2034]) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2033]) && (!s.b[2034])) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        s.b[2036] = (p.p1374 < 0.001);
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        if s.b[2036] {
            s.store_scalar(167, (1.0 / 0.001));
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq6_e1526, eq6_e1526_d_n0, eq6_e1526_d_n1, eq6_e1526_d_n2, eq6_e1526_d_n3, eq6_e1526_d_n4, eq6_e1526_d_n5, eq6_e1526_d_n6, eq6_e1526_d_n7, eq6_e1526_d_n8, eq6_e1526_d_n9, eq6_e1526_d_n10, eq6_e1526_d_n11, eq6_e1526_d_n12, eq6_e1526_d_n13, eq6_e1526_d_b0, eq6_e1526_d_b1, eq6_e1526_d_b2, eq6_e1526_d_b3, eq6_e1526_d_b4, eq6_e1526_d_b5, eq6_e1526_d_b6, eq6_e1526_d_b7, eq6_e1526_d_b8, eq6_e1526_d_b9, eq6_e1526_d_b10, eq6_e1526_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq6_e1522: f64 = (-s.v[629]);
        let eq6_e1522_d_n0: f64 = (-s.dn[629][0]);
        let eq6_e1522_d_n1: f64 = (-s.dn[629][1]);
        let eq6_e1522_d_n2: f64 = (-s.dn[629][2]);
        let eq6_e1522_d_n3: f64 = (-s.dn[629][3]);
        let eq6_e1522_d_n4: f64 = (-s.dn[629][4]);
        let eq6_e1522_d_n5: f64 = (-s.dn[629][5]);
        let eq6_e1522_d_n6: f64 = (-s.dn[629][6]);
        let eq6_e1522_d_n7: f64 = (-s.dn[629][7]);
        let eq6_e1522_d_n8: f64 = (-s.dn[629][8]);
        let eq6_e1522_d_n9: f64 = (-s.dn[629][9]);
        let eq6_e1522_d_n10: f64 = (-s.dn[629][10]);
        let eq6_e1522_d_n11: f64 = (-s.dn[629][11]);
        let eq6_e1522_d_n12: f64 = (-s.dn[629][12]);
        let eq6_e1522_d_n13: f64 = (-s.dn[629][13]);
        let eq6_e1522_d_b0: f64 = (-s.db[629][0]);
        let eq6_e1522_d_b1: f64 = (-s.db[629][1]);
        let eq6_e1522_d_b2: f64 = (-s.db[629][2]);
        let eq6_e1522_d_b3: f64 = (-s.db[629][3]);
        let eq6_e1522_d_b4: f64 = (-s.db[629][4]);
        let eq6_e1522_d_b5: f64 = (-s.db[629][5]);
        let eq6_e1522_d_b6: f64 = (-s.db[629][6]);
        let eq6_e1522_d_b7: f64 = (-s.db[629][7]);
        let eq6_e1522_d_b8: f64 = (-s.db[629][8]);
        let eq6_e1522_d_b9: f64 = (-s.db[629][9]);
        let eq6_e1522_d_b10: f64 = (-s.db[629][10]);
        let eq6_e1522_d_b11: f64 = (-s.db[629][11]);
        let eq6_e1524: f64 = (eq6_e1522 * (nv13 - 0.0));
        let eq6_e1524_d_n0: f64 = (eq6_e1522_d_n0 * (nv13 - 0.0));
        let eq6_e1524_d_n1: f64 = (eq6_e1522_d_n1 * (nv13 - 0.0));
        let eq6_e1524_d_n2: f64 = (eq6_e1522_d_n2 * (nv13 - 0.0));
        let eq6_e1524_d_n3: f64 = (eq6_e1522_d_n3 * (nv13 - 0.0));
        let eq6_e1524_d_n4: f64 = (eq6_e1522_d_n4 * (nv13 - 0.0));
        let eq6_e1524_d_n5: f64 = (eq6_e1522_d_n5 * (nv13 - 0.0));
        let eq6_e1524_d_n6: f64 = (eq6_e1522_d_n6 * (nv13 - 0.0));
        let eq6_e1524_d_n7: f64 = (eq6_e1522_d_n7 * (nv13 - 0.0));
        let eq6_e1524_d_n8: f64 = (eq6_e1522_d_n8 * (nv13 - 0.0));
        let eq6_e1524_d_n9: f64 = (eq6_e1522_d_n9 * (nv13 - 0.0));
        let eq6_e1524_d_n10: f64 = (eq6_e1522_d_n10 * (nv13 - 0.0));
        let eq6_e1524_d_n11: f64 = (eq6_e1522_d_n11 * (nv13 - 0.0));
        let eq6_e1524_d_n12: f64 = (eq6_e1522_d_n12 * (nv13 - 0.0));
        let eq6_e1524_d_n13: f64 = ((eq6_e1522_d_n13 * (nv13 - 0.0)) + eq6_e1522);
        let eq6_e1524_d_b0: f64 = (eq6_e1522_d_b0 * (nv13 - 0.0));
        let eq6_e1524_d_b1: f64 = (eq6_e1522_d_b1 * (nv13 - 0.0));
        let eq6_e1524_d_b2: f64 = (eq6_e1522_d_b2 * (nv13 - 0.0));
        let eq6_e1524_d_b3: f64 = (eq6_e1522_d_b3 * (nv13 - 0.0));
        let eq6_e1524_d_b4: f64 = (eq6_e1522_d_b4 * (nv13 - 0.0));
        let eq6_e1524_d_b5: f64 = (eq6_e1522_d_b5 * (nv13 - 0.0));
        let eq6_e1524_d_b6: f64 = (eq6_e1522_d_b6 * (nv13 - 0.0));
        let eq6_e1524_d_b7: f64 = (eq6_e1522_d_b7 * (nv13 - 0.0));
        let eq6_e1524_d_b8: f64 = (eq6_e1522_d_b8 * (nv13 - 0.0));
        let eq6_e1524_d_b9: f64 = (eq6_e1522_d_b9 * (nv13 - 0.0));
        let eq6_e1524_d_b10: f64 = (eq6_e1522_d_b10 * (nv13 - 0.0));
        let eq6_e1524_d_b11: f64 = (eq6_e1522_d_b11 * (nv13 - 0.0));
        (eq6_e1524, eq6_e1524_d_n0, eq6_e1524_d_n1, eq6_e1524_d_n2, eq6_e1524_d_n3, eq6_e1524_d_n4, eq6_e1524_d_n5, eq6_e1524_d_n6, eq6_e1524_d_n7, eq6_e1524_d_n8, eq6_e1524_d_n9, eq6_e1524_d_n10, eq6_e1524_d_n11, eq6_e1524_d_n12, eq6_e1524_d_n13, eq6_e1524_d_b0, eq6_e1524_d_b1, eq6_e1524_d_b2, eq6_e1524_d_b3, eq6_e1524_d_b4, eq6_e1524_d_b5, eq6_e1524_d_b6, eq6_e1524_d_b7, eq6_e1524_d_b8, eq6_e1524_d_b9, eq6_e1524_d_b10, eq6_e1524_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1526;
        let eq6_node_derivatives: [f64; 14] = [eq6_e1526_d_n0, eq6_e1526_d_n1, eq6_e1526_d_n2, eq6_e1526_d_n3, eq6_e1526_d_n4, eq6_e1526_d_n5, eq6_e1526_d_n6, eq6_e1526_d_n7, eq6_e1526_d_n8, eq6_e1526_d_n9, eq6_e1526_d_n10, eq6_e1526_d_n11, eq6_e1526_d_n12, eq6_e1526_d_n13];
        let eq6_branch_derivatives: [f64; 12] = [eq6_e1526_d_b0, eq6_e1526_d_b1, eq6_e1526_d_b2, eq6_e1526_d_b3, eq6_e1526_d_b4, eq6_e1526_d_b5, eq6_e1526_d_b6, eq6_e1526_d_b7, eq6_e1526_d_b8, eq6_e1526_d_b9, eq6_e1526_d_b10, eq6_e1526_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1546, eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13, eq7_e1546_d_b0, eq7_e1546_d_b1, eq7_e1546_d_b2, eq7_e1546_d_b3, eq7_e1546_d_b4, eq7_e1546_d_b5, eq7_e1546_d_b6, eq7_e1546_d_b7, eq7_e1546_d_b8, eq7_e1546_d_b9, eq7_e1546_d_b10, eq7_e1546_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq7_e1535: f64 = (s.v[622] * s.v[199]);
        let eq7_e1535_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq7_e1535_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq7_e1535_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq7_e1535_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq7_e1535_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq7_e1535_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq7_e1535_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq7_e1535_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq7_e1535_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq7_e1535_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq7_e1535_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq7_e1535_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq7_e1535_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq7_e1535_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq7_e1535_d_b0: f64 = (s.db[622][0] * s.v[199]);
        let eq7_e1535_d_b1: f64 = (s.db[622][1] * s.v[199]);
        let eq7_e1535_d_b2: f64 = (s.db[622][2] * s.v[199]);
        let eq7_e1535_d_b3: f64 = (s.db[622][3] * s.v[199]);
        let eq7_e1535_d_b4: f64 = (s.db[622][4] * s.v[199]);
        let eq7_e1535_d_b5: f64 = (s.db[622][5] * s.v[199]);
        let eq7_e1535_d_b6: f64 = (s.db[622][6] * s.v[199]);
        let eq7_e1535_d_b7: f64 = (s.db[622][7] * s.v[199]);
        let eq7_e1535_d_b8: f64 = (s.db[622][8] * s.v[199]);
        let eq7_e1535_d_b9: f64 = (s.db[622][9] * s.v[199]);
        let eq7_e1535_d_b10: f64 = (s.db[622][10] * s.v[199]);
        let eq7_e1535_d_b11: f64 = (s.db[622][11] * s.v[199]);
        let eq7_e1537: f64 = (eq7_e1535 * s.v[183]);
        let eq7_e1537_d_n0: f64 = (eq7_e1535_d_n0 * s.v[183]);
        let eq7_e1537_d_n1: f64 = (eq7_e1535_d_n1 * s.v[183]);
        let eq7_e1537_d_n2: f64 = (eq7_e1535_d_n2 * s.v[183]);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * s.v[183]);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * s.v[183]);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * s.v[183]);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * s.v[183]);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * s.v[183]);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * s.v[183]);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * s.v[183]);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * s.v[183]);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * s.v[183]);
        let eq7_e1537_d_n12: f64 = (eq7_e1535_d_n12 * s.v[183]);
        let eq7_e1537_d_n13: f64 = (eq7_e1535_d_n13 * s.v[183]);
        let eq7_e1537_d_b0: f64 = (eq7_e1535_d_b0 * s.v[183]);
        let eq7_e1537_d_b1: f64 = (eq7_e1535_d_b1 * s.v[183]);
        let eq7_e1537_d_b2: f64 = (eq7_e1535_d_b2 * s.v[183]);
        let eq7_e1537_d_b3: f64 = (eq7_e1535_d_b3 * s.v[183]);
        let eq7_e1537_d_b4: f64 = (eq7_e1535_d_b4 * s.v[183]);
        let eq7_e1537_d_b5: f64 = (eq7_e1535_d_b5 * s.v[183]);
        let eq7_e1537_d_b6: f64 = (eq7_e1535_d_b6 * s.v[183]);
        let eq7_e1537_d_b7: f64 = (eq7_e1535_d_b7 * s.v[183]);
        let eq7_e1537_d_b8: f64 = (eq7_e1535_d_b8 * s.v[183]);
        let eq7_e1537_d_b9: f64 = (eq7_e1535_d_b9 * s.v[183]);
        let eq7_e1537_d_b10: f64 = (eq7_e1535_d_b10 * s.v[183]);
        let eq7_e1537_d_b11: f64 = (eq7_e1535_d_b11 * s.v[183]);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n0: f64 = (eq7_e1537_d_n0 * p.p2);
        let eq7_e1539_d_n1: f64 = (eq7_e1537_d_n1 * p.p2);
        let eq7_e1539_d_n2: f64 = (eq7_e1537_d_n2 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1539_d_n12: f64 = (eq7_e1537_d_n12 * p.p2);
        let eq7_e1539_d_n13: f64 = (eq7_e1537_d_n13 * p.p2);
        let eq7_e1539_d_b0: f64 = (eq7_e1537_d_b0 * p.p2);
        let eq7_e1539_d_b1: f64 = (eq7_e1537_d_b1 * p.p2);
        let eq7_e1539_d_b2: f64 = (eq7_e1537_d_b2 * p.p2);
        let eq7_e1539_d_b3: f64 = (eq7_e1537_d_b3 * p.p2);
        let eq7_e1539_d_b4: f64 = (eq7_e1537_d_b4 * p.p2);
        let eq7_e1539_d_b5: f64 = (eq7_e1537_d_b5 * p.p2);
        let eq7_e1539_d_b6: f64 = (eq7_e1537_d_b6 * p.p2);
        let eq7_e1539_d_b7: f64 = (eq7_e1537_d_b7 * p.p2);
        let eq7_e1539_d_b8: f64 = (eq7_e1537_d_b8 * p.p2);
        let eq7_e1539_d_b9: f64 = (eq7_e1537_d_b9 * p.p2);
        let eq7_e1539_d_b10: f64 = (eq7_e1537_d_b10 * p.p2);
        let eq7_e1539_d_b11: f64 = (eq7_e1537_d_b11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * s.v[184]);
        let eq7_e1541_d_n0: f64 = (eq7_e1539_d_n0 * s.v[184]);
        let eq7_e1541_d_n1: f64 = (eq7_e1539_d_n1 * s.v[184]);
        let eq7_e1541_d_n2: f64 = (eq7_e1539_d_n2 * s.v[184]);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * s.v[184]);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * s.v[184]);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * s.v[184]);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * s.v[184]);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * s.v[184]);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * s.v[184]);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * s.v[184]);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * s.v[184]);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * s.v[184]);
        let eq7_e1541_d_n12: f64 = (eq7_e1539_d_n12 * s.v[184]);
        let eq7_e1541_d_n13: f64 = (eq7_e1539_d_n13 * s.v[184]);
        let eq7_e1541_d_b0: f64 = (eq7_e1539_d_b0 * s.v[184]);
        let eq7_e1541_d_b1: f64 = (eq7_e1539_d_b1 * s.v[184]);
        let eq7_e1541_d_b2: f64 = (eq7_e1539_d_b2 * s.v[184]);
        let eq7_e1541_d_b3: f64 = (eq7_e1539_d_b3 * s.v[184]);
        let eq7_e1541_d_b4: f64 = (eq7_e1539_d_b4 * s.v[184]);
        let eq7_e1541_d_b5: f64 = (eq7_e1539_d_b5 * s.v[184]);
        let eq7_e1541_d_b6: f64 = (eq7_e1539_d_b6 * s.v[184]);
        let eq7_e1541_d_b7: f64 = (eq7_e1539_d_b7 * s.v[184]);
        let eq7_e1541_d_b8: f64 = (eq7_e1539_d_b8 * s.v[184]);
        let eq7_e1541_d_b9: f64 = (eq7_e1539_d_b9 * s.v[184]);
        let eq7_e1541_d_b10: f64 = (eq7_e1539_d_b10 * s.v[184]);
        let eq7_e1541_d_b11: f64 = (eq7_e1539_d_b11 * s.v[184]);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n0: f64 = (eq7_e1541_d_n0 * (nv12 - 0.0));
        let eq7_e1543_d_n1: f64 = (eq7_e1541_d_n1 * (nv12 - 0.0));
        let eq7_e1543_d_n2: f64 = (eq7_e1541_d_n2 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1543_d_n12: f64 = ((eq7_e1541_d_n12 * (nv12 - 0.0)) + eq7_e1541);
        let eq7_e1543_d_n13: f64 = (eq7_e1541_d_n13 * (nv12 - 0.0));
        let eq7_e1543_d_b0: f64 = (eq7_e1541_d_b0 * (nv12 - 0.0));
        let eq7_e1543_d_b1: f64 = (eq7_e1541_d_b1 * (nv12 - 0.0));
        let eq7_e1543_d_b2: f64 = (eq7_e1541_d_b2 * (nv12 - 0.0));
        let eq7_e1543_d_b3: f64 = (eq7_e1541_d_b3 * (nv12 - 0.0));
        let eq7_e1543_d_b4: f64 = (eq7_e1541_d_b4 * (nv12 - 0.0));
        let eq7_e1543_d_b5: f64 = (eq7_e1541_d_b5 * (nv12 - 0.0));
        let eq7_e1543_d_b6: f64 = (eq7_e1541_d_b6 * (nv12 - 0.0));
        let eq7_e1543_d_b7: f64 = (eq7_e1541_d_b7 * (nv12 - 0.0));
        let eq7_e1543_d_b8: f64 = (eq7_e1541_d_b8 * (nv12 - 0.0));
        let eq7_e1543_d_b9: f64 = (eq7_e1541_d_b9 * (nv12 - 0.0));
        let eq7_e1543_d_b10: f64 = (eq7_e1541_d_b10 * (nv12 - 0.0));
        let eq7_e1543_d_b11: f64 = (eq7_e1541_d_b11 * (nv12 - 0.0));
        let eq7_e1544: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq7_e1543);
        let eq7_e1544_d_n0: f64 = (eq7_e1543_d_n0 * ddt_scale);
        let eq7_e1544_d_n1: f64 = (eq7_e1543_d_n1 * ddt_scale);
        let eq7_e1544_d_n2: f64 = (eq7_e1543_d_n2 * ddt_scale);
        let eq7_e1544_d_n3: f64 = (eq7_e1543_d_n3 * ddt_scale);
        let eq7_e1544_d_n4: f64 = (eq7_e1543_d_n4 * ddt_scale);
        let eq7_e1544_d_n5: f64 = (eq7_e1543_d_n5 * ddt_scale);
        let eq7_e1544_d_n6: f64 = (eq7_e1543_d_n6 * ddt_scale);
        let eq7_e1544_d_n7: f64 = (eq7_e1543_d_n7 * ddt_scale);
        let eq7_e1544_d_n8: f64 = (eq7_e1543_d_n8 * ddt_scale);
        let eq7_e1544_d_n9: f64 = (eq7_e1543_d_n9 * ddt_scale);
        let eq7_e1544_d_n10: f64 = (eq7_e1543_d_n10 * ddt_scale);
        let eq7_e1544_d_n11: f64 = (eq7_e1543_d_n11 * ddt_scale);
        let eq7_e1544_d_n12: f64 = (eq7_e1543_d_n12 * ddt_scale);
        let eq7_e1544_d_n13: f64 = (eq7_e1543_d_n13 * ddt_scale);
        let eq7_e1544_d_b0: f64 = (eq7_e1543_d_b0 * ddt_scale);
        let eq7_e1544_d_b1: f64 = (eq7_e1543_d_b1 * ddt_scale);
        let eq7_e1544_d_b2: f64 = (eq7_e1543_d_b2 * ddt_scale);
        let eq7_e1544_d_b3: f64 = (eq7_e1543_d_b3 * ddt_scale);
        let eq7_e1544_d_b4: f64 = (eq7_e1543_d_b4 * ddt_scale);
        let eq7_e1544_d_b5: f64 = (eq7_e1543_d_b5 * ddt_scale);
        let eq7_e1544_d_b6: f64 = (eq7_e1543_d_b6 * ddt_scale);
        let eq7_e1544_d_b7: f64 = (eq7_e1543_d_b7 * ddt_scale);
        let eq7_e1544_d_b8: f64 = (eq7_e1543_d_b8 * ddt_scale);
        let eq7_e1544_d_b9: f64 = (eq7_e1543_d_b9 * ddt_scale);
        let eq7_e1544_d_b10: f64 = (eq7_e1543_d_b10 * ddt_scale);
        let eq7_e1544_d_b11: f64 = (eq7_e1543_d_b11 * ddt_scale);
        (eq7_e1544, eq7_e1544_d_n0, eq7_e1544_d_n1, eq7_e1544_d_n2, eq7_e1544_d_n3, eq7_e1544_d_n4, eq7_e1544_d_n5, eq7_e1544_d_n6, eq7_e1544_d_n7, eq7_e1544_d_n8, eq7_e1544_d_n9, eq7_e1544_d_n10, eq7_e1544_d_n11, eq7_e1544_d_n12, eq7_e1544_d_n13, eq7_e1544_d_b0, eq7_e1544_d_b1, eq7_e1544_d_b2, eq7_e1544_d_b3, eq7_e1544_d_b4, eq7_e1544_d_b5, eq7_e1544_d_b6, eq7_e1544_d_b7, eq7_e1544_d_b8, eq7_e1544_d_b9, eq7_e1544_d_b10, eq7_e1544_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1546;
        let eq7_node_derivatives: [f64; 14] = [eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13];
        let eq7_branch_derivatives: [f64; 12] = [eq7_e1546_d_b0, eq7_e1546_d_b1, eq7_e1546_d_b2, eq7_e1546_d_b3, eq7_e1546_d_b4, eq7_e1546_d_b5, eq7_e1546_d_b6, eq7_e1546_d_b7, eq7_e1546_d_b8, eq7_e1546_d_b9, eq7_e1546_d_b10, eq7_e1546_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq9_e1574, eq9_e1574_d_n0, eq9_e1574_d_n1, eq9_e1574_d_n2, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n12, eq9_e1574_d_n13, eq9_e1574_d_b0, eq9_e1574_d_b1, eq9_e1574_d_b2, eq9_e1574_d_b3, eq9_e1574_d_b4, eq9_e1574_d_b5, eq9_e1574_d_b6, eq9_e1574_d_b7, eq9_e1574_d_b8, eq9_e1574_d_b9, eq9_e1574_d_b10, eq9_e1574_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq9_e1572: f64 = (s.v[628] * (nv13 - 0.0));
        let eq9_e1572_d_n0: f64 = (s.dn[628][0] * (nv13 - 0.0));
        let eq9_e1572_d_n1: f64 = (s.dn[628][1] * (nv13 - 0.0));
        let eq9_e1572_d_n2: f64 = (s.dn[628][2] * (nv13 - 0.0));
        let eq9_e1572_d_n3: f64 = (s.dn[628][3] * (nv13 - 0.0));
        let eq9_e1572_d_n4: f64 = (s.dn[628][4] * (nv13 - 0.0));
        let eq9_e1572_d_n5: f64 = (s.dn[628][5] * (nv13 - 0.0));
        let eq9_e1572_d_n6: f64 = (s.dn[628][6] * (nv13 - 0.0));
        let eq9_e1572_d_n7: f64 = (s.dn[628][7] * (nv13 - 0.0));
        let eq9_e1572_d_n8: f64 = (s.dn[628][8] * (nv13 - 0.0));
        let eq9_e1572_d_n9: f64 = (s.dn[628][9] * (nv13 - 0.0));
        let eq9_e1572_d_n10: f64 = (s.dn[628][10] * (nv13 - 0.0));
        let eq9_e1572_d_n11: f64 = (s.dn[628][11] * (nv13 - 0.0));
        let eq9_e1572_d_n12: f64 = (s.dn[628][12] * (nv13 - 0.0));
        let eq9_e1572_d_n13: f64 = ((s.dn[628][13] * (nv13 - 0.0)) + s.v[628]);
        let eq9_e1572_d_b0: f64 = (s.db[628][0] * (nv13 - 0.0));
        let eq9_e1572_d_b1: f64 = (s.db[628][1] * (nv13 - 0.0));
        let eq9_e1572_d_b2: f64 = (s.db[628][2] * (nv13 - 0.0));
        let eq9_e1572_d_b3: f64 = (s.db[628][3] * (nv13 - 0.0));
        let eq9_e1572_d_b4: f64 = (s.db[628][4] * (nv13 - 0.0));
        let eq9_e1572_d_b5: f64 = (s.db[628][5] * (nv13 - 0.0));
        let eq9_e1572_d_b6: f64 = (s.db[628][6] * (nv13 - 0.0));
        let eq9_e1572_d_b7: f64 = (s.db[628][7] * (nv13 - 0.0));
        let eq9_e1572_d_b8: f64 = (s.db[628][8] * (nv13 - 0.0));
        let eq9_e1572_d_b9: f64 = (s.db[628][9] * (nv13 - 0.0));
        let eq9_e1572_d_b10: f64 = (s.db[628][10] * (nv13 - 0.0));
        let eq9_e1572_d_b11: f64 = (s.db[628][11] * (nv13 - 0.0));
        (eq9_e1572, eq9_e1572_d_n0, eq9_e1572_d_n1, eq9_e1572_d_n2, eq9_e1572_d_n3, eq9_e1572_d_n4, eq9_e1572_d_n5, eq9_e1572_d_n6, eq9_e1572_d_n7, eq9_e1572_d_n8, eq9_e1572_d_n9, eq9_e1572_d_n10, eq9_e1572_d_n11, eq9_e1572_d_n12, eq9_e1572_d_n13, eq9_e1572_d_b0, eq9_e1572_d_b1, eq9_e1572_d_b2, eq9_e1572_d_b3, eq9_e1572_d_b4, eq9_e1572_d_b5, eq9_e1572_d_b6, eq9_e1572_d_b7, eq9_e1572_d_b8, eq9_e1572_d_b9, eq9_e1572_d_b10, eq9_e1572_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e1574;
        let eq9_node_derivatives: [f64; 14] = [eq9_e1574_d_n0, eq9_e1574_d_n1, eq9_e1574_d_n2, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n12, eq9_e1574_d_n13];
        let eq9_branch_derivatives: [f64; 12] = [eq9_e1574_d_b0, eq9_e1574_d_b1, eq9_e1574_d_b2, eq9_e1574_d_b3, eq9_e1574_d_b4, eq9_e1574_d_b5, eq9_e1574_d_b6, eq9_e1574_d_b7, eq9_e1574_d_b8, eq9_e1574_d_b9, eq9_e1574_d_b10, eq9_e1574_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq10_e1600, eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13, eq10_e1600_d_b0, eq10_e1600_d_b1, eq10_e1600_d_b2, eq10_e1600_d_b3, eq10_e1600_d_b4, eq10_e1600_d_b5, eq10_e1600_d_b6, eq10_e1600_d_b7, eq10_e1600_d_b8, eq10_e1600_d_b9, eq10_e1600_d_b10, eq10_e1600_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq10_e1584: f64 = (1.0 + s.v[211]);
        let eq10_e1586: f64 = (eq10_e1584 * s.v[622]);
        let eq10_e1586_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1584 * s.dn[622][0]));
        let eq10_e1586_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1584 * s.dn[622][1]));
        let eq10_e1586_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1584 * s.dn[622][2]));
        let eq10_e1586_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1584 * s.dn[622][3]));
        let eq10_e1586_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1584 * s.dn[622][4]));
        let eq10_e1586_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1584 * s.dn[622][5]));
        let eq10_e1586_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1584 * s.dn[622][6]));
        let eq10_e1586_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1584 * s.dn[622][7]));
        let eq10_e1586_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1584 * s.dn[622][8]));
        let eq10_e1586_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1584 * s.dn[622][9]));
        let eq10_e1586_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1584 * s.dn[622][10]));
        let eq10_e1586_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1584 * s.dn[622][11]));
        let eq10_e1586_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1584 * s.dn[622][12]));
        let eq10_e1586_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1584 * s.dn[622][13]));
        let eq10_e1586_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq10_e1584 * s.db[622][0]));
        let eq10_e1586_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq10_e1584 * s.db[622][1]));
        let eq10_e1586_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq10_e1584 * s.db[622][2]));
        let eq10_e1586_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq10_e1584 * s.db[622][3]));
        let eq10_e1586_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq10_e1584 * s.db[622][4]));
        let eq10_e1586_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq10_e1584 * s.db[622][5]));
        let eq10_e1586_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq10_e1584 * s.db[622][6]));
        let eq10_e1586_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq10_e1584 * s.db[622][7]));
        let eq10_e1586_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq10_e1584 * s.db[622][8]));
        let eq10_e1586_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq10_e1584 * s.db[622][9]));
        let eq10_e1586_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq10_e1584 * s.db[622][10]));
        let eq10_e1586_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq10_e1584 * s.db[622][11]));
        let eq10_e1588: f64 = (eq10_e1586 * s.v[199]);
        let eq10_e1588_d_n0: f64 = (eq10_e1586_d_n0 * s.v[199]);
        let eq10_e1588_d_n1: f64 = (eq10_e1586_d_n1 * s.v[199]);
        let eq10_e1588_d_n2: f64 = (eq10_e1586_d_n2 * s.v[199]);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * s.v[199]);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * s.v[199]);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * s.v[199]);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * s.v[199]);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * s.v[199]);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * s.v[199]);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * s.v[199]);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * s.v[199]);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * s.v[199]);
        let eq10_e1588_d_n12: f64 = (eq10_e1586_d_n12 * s.v[199]);
        let eq10_e1588_d_n13: f64 = (eq10_e1586_d_n13 * s.v[199]);
        let eq10_e1588_d_b0: f64 = (eq10_e1586_d_b0 * s.v[199]);
        let eq10_e1588_d_b1: f64 = (eq10_e1586_d_b1 * s.v[199]);
        let eq10_e1588_d_b2: f64 = (eq10_e1586_d_b2 * s.v[199]);
        let eq10_e1588_d_b3: f64 = (eq10_e1586_d_b3 * s.v[199]);
        let eq10_e1588_d_b4: f64 = (eq10_e1586_d_b4 * s.v[199]);
        let eq10_e1588_d_b5: f64 = (eq10_e1586_d_b5 * s.v[199]);
        let eq10_e1588_d_b6: f64 = (eq10_e1586_d_b6 * s.v[199]);
        let eq10_e1588_d_b7: f64 = (eq10_e1586_d_b7 * s.v[199]);
        let eq10_e1588_d_b8: f64 = (eq10_e1586_d_b8 * s.v[199]);
        let eq10_e1588_d_b9: f64 = (eq10_e1586_d_b9 * s.v[199]);
        let eq10_e1588_d_b10: f64 = (eq10_e1586_d_b10 * s.v[199]);
        let eq10_e1588_d_b11: f64 = (eq10_e1586_d_b11 * s.v[199]);
        let eq10_e1590: f64 = (eq10_e1588 * s.v[183]);
        let eq10_e1590_d_n0: f64 = (eq10_e1588_d_n0 * s.v[183]);
        let eq10_e1590_d_n1: f64 = (eq10_e1588_d_n1 * s.v[183]);
        let eq10_e1590_d_n2: f64 = (eq10_e1588_d_n2 * s.v[183]);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * s.v[183]);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * s.v[183]);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * s.v[183]);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * s.v[183]);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * s.v[183]);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * s.v[183]);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * s.v[183]);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * s.v[183]);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * s.v[183]);
        let eq10_e1590_d_n12: f64 = (eq10_e1588_d_n12 * s.v[183]);
        let eq10_e1590_d_n13: f64 = (eq10_e1588_d_n13 * s.v[183]);
        let eq10_e1590_d_b0: f64 = (eq10_e1588_d_b0 * s.v[183]);
        let eq10_e1590_d_b1: f64 = (eq10_e1588_d_b1 * s.v[183]);
        let eq10_e1590_d_b2: f64 = (eq10_e1588_d_b2 * s.v[183]);
        let eq10_e1590_d_b3: f64 = (eq10_e1588_d_b3 * s.v[183]);
        let eq10_e1590_d_b4: f64 = (eq10_e1588_d_b4 * s.v[183]);
        let eq10_e1590_d_b5: f64 = (eq10_e1588_d_b5 * s.v[183]);
        let eq10_e1590_d_b6: f64 = (eq10_e1588_d_b6 * s.v[183]);
        let eq10_e1590_d_b7: f64 = (eq10_e1588_d_b7 * s.v[183]);
        let eq10_e1590_d_b8: f64 = (eq10_e1588_d_b8 * s.v[183]);
        let eq10_e1590_d_b9: f64 = (eq10_e1588_d_b9 * s.v[183]);
        let eq10_e1590_d_b10: f64 = (eq10_e1588_d_b10 * s.v[183]);
        let eq10_e1590_d_b11: f64 = (eq10_e1588_d_b11 * s.v[183]);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n0: f64 = (eq10_e1590_d_n0 * p.p2);
        let eq10_e1592_d_n1: f64 = (eq10_e1590_d_n1 * p.p2);
        let eq10_e1592_d_n2: f64 = (eq10_e1590_d_n2 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1592_d_n12: f64 = (eq10_e1590_d_n12 * p.p2);
        let eq10_e1592_d_n13: f64 = (eq10_e1590_d_n13 * p.p2);
        let eq10_e1592_d_b0: f64 = (eq10_e1590_d_b0 * p.p2);
        let eq10_e1592_d_b1: f64 = (eq10_e1590_d_b1 * p.p2);
        let eq10_e1592_d_b2: f64 = (eq10_e1590_d_b2 * p.p2);
        let eq10_e1592_d_b3: f64 = (eq10_e1590_d_b3 * p.p2);
        let eq10_e1592_d_b4: f64 = (eq10_e1590_d_b4 * p.p2);
        let eq10_e1592_d_b5: f64 = (eq10_e1590_d_b5 * p.p2);
        let eq10_e1592_d_b6: f64 = (eq10_e1590_d_b6 * p.p2);
        let eq10_e1592_d_b7: f64 = (eq10_e1590_d_b7 * p.p2);
        let eq10_e1592_d_b8: f64 = (eq10_e1590_d_b8 * p.p2);
        let eq10_e1592_d_b9: f64 = (eq10_e1590_d_b9 * p.p2);
        let eq10_e1592_d_b10: f64 = (eq10_e1590_d_b10 * p.p2);
        let eq10_e1592_d_b11: f64 = (eq10_e1590_d_b11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * s.v[184]);
        let eq10_e1594_d_n0: f64 = (eq10_e1592_d_n0 * s.v[184]);
        let eq10_e1594_d_n1: f64 = (eq10_e1592_d_n1 * s.v[184]);
        let eq10_e1594_d_n2: f64 = (eq10_e1592_d_n2 * s.v[184]);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * s.v[184]);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * s.v[184]);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * s.v[184]);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * s.v[184]);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * s.v[184]);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * s.v[184]);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * s.v[184]);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * s.v[184]);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * s.v[184]);
        let eq10_e1594_d_n12: f64 = (eq10_e1592_d_n12 * s.v[184]);
        let eq10_e1594_d_n13: f64 = (eq10_e1592_d_n13 * s.v[184]);
        let eq10_e1594_d_b0: f64 = (eq10_e1592_d_b0 * s.v[184]);
        let eq10_e1594_d_b1: f64 = (eq10_e1592_d_b1 * s.v[184]);
        let eq10_e1594_d_b2: f64 = (eq10_e1592_d_b2 * s.v[184]);
        let eq10_e1594_d_b3: f64 = (eq10_e1592_d_b3 * s.v[184]);
        let eq10_e1594_d_b4: f64 = (eq10_e1592_d_b4 * s.v[184]);
        let eq10_e1594_d_b5: f64 = (eq10_e1592_d_b5 * s.v[184]);
        let eq10_e1594_d_b6: f64 = (eq10_e1592_d_b6 * s.v[184]);
        let eq10_e1594_d_b7: f64 = (eq10_e1592_d_b7 * s.v[184]);
        let eq10_e1594_d_b8: f64 = (eq10_e1592_d_b8 * s.v[184]);
        let eq10_e1594_d_b9: f64 = (eq10_e1592_d_b9 * s.v[184]);
        let eq10_e1594_d_b10: f64 = (eq10_e1592_d_b10 * s.v[184]);
        let eq10_e1594_d_b11: f64 = (eq10_e1592_d_b11 * s.v[184]);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n0: f64 = (eq10_e1594_d_n0 * (nv12 - 0.0));
        let eq10_e1596_d_n1: f64 = (eq10_e1594_d_n1 * (nv12 - 0.0));
        let eq10_e1596_d_n2: f64 = (eq10_e1594_d_n2 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1596_d_n12: f64 = ((eq10_e1594_d_n12 * (nv12 - 0.0)) + eq10_e1594);
        let eq10_e1596_d_n13: f64 = (eq10_e1594_d_n13 * (nv12 - 0.0));
        let eq10_e1596_d_b0: f64 = (eq10_e1594_d_b0 * (nv12 - 0.0));
        let eq10_e1596_d_b1: f64 = (eq10_e1594_d_b1 * (nv12 - 0.0));
        let eq10_e1596_d_b2: f64 = (eq10_e1594_d_b2 * (nv12 - 0.0));
        let eq10_e1596_d_b3: f64 = (eq10_e1594_d_b3 * (nv12 - 0.0));
        let eq10_e1596_d_b4: f64 = (eq10_e1594_d_b4 * (nv12 - 0.0));
        let eq10_e1596_d_b5: f64 = (eq10_e1594_d_b5 * (nv12 - 0.0));
        let eq10_e1596_d_b6: f64 = (eq10_e1594_d_b6 * (nv12 - 0.0));
        let eq10_e1596_d_b7: f64 = (eq10_e1594_d_b7 * (nv12 - 0.0));
        let eq10_e1596_d_b8: f64 = (eq10_e1594_d_b8 * (nv12 - 0.0));
        let eq10_e1596_d_b9: f64 = (eq10_e1594_d_b9 * (nv12 - 0.0));
        let eq10_e1596_d_b10: f64 = (eq10_e1594_d_b10 * (nv12 - 0.0));
        let eq10_e1596_d_b11: f64 = (eq10_e1594_d_b11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n0: f64 = (0.5 * eq10_e1596_d_n0);
        let eq10_e1597_d_n1: f64 = (0.5 * eq10_e1596_d_n1);
        let eq10_e1597_d_n2: f64 = (0.5 * eq10_e1596_d_n2);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1596_d_n12);
        let eq10_e1597_d_n13: f64 = (0.5 * eq10_e1596_d_n13);
        let eq10_e1597_d_b0: f64 = (0.5 * eq10_e1596_d_b0);
        let eq10_e1597_d_b1: f64 = (0.5 * eq10_e1596_d_b1);
        let eq10_e1597_d_b2: f64 = (0.5 * eq10_e1596_d_b2);
        let eq10_e1597_d_b3: f64 = (0.5 * eq10_e1596_d_b3);
        let eq10_e1597_d_b4: f64 = (0.5 * eq10_e1596_d_b4);
        let eq10_e1597_d_b5: f64 = (0.5 * eq10_e1596_d_b5);
        let eq10_e1597_d_b6: f64 = (0.5 * eq10_e1596_d_b6);
        let eq10_e1597_d_b7: f64 = (0.5 * eq10_e1596_d_b7);
        let eq10_e1597_d_b8: f64 = (0.5 * eq10_e1596_d_b8);
        let eq10_e1597_d_b9: f64 = (0.5 * eq10_e1596_d_b9);
        let eq10_e1597_d_b10: f64 = (0.5 * eq10_e1596_d_b10);
        let eq10_e1597_d_b11: f64 = (0.5 * eq10_e1596_d_b11);
        let eq10_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq10_e1597);
        let eq10_e1598_d_n0: f64 = (eq10_e1597_d_n0 * ddt_scale);
        let eq10_e1598_d_n1: f64 = (eq10_e1597_d_n1 * ddt_scale);
        let eq10_e1598_d_n2: f64 = (eq10_e1597_d_n2 * ddt_scale);
        let eq10_e1598_d_n3: f64 = (eq10_e1597_d_n3 * ddt_scale);
        let eq10_e1598_d_n4: f64 = (eq10_e1597_d_n4 * ddt_scale);
        let eq10_e1598_d_n5: f64 = (eq10_e1597_d_n5 * ddt_scale);
        let eq10_e1598_d_n6: f64 = (eq10_e1597_d_n6 * ddt_scale);
        let eq10_e1598_d_n7: f64 = (eq10_e1597_d_n7 * ddt_scale);
        let eq10_e1598_d_n8: f64 = (eq10_e1597_d_n8 * ddt_scale);
        let eq10_e1598_d_n9: f64 = (eq10_e1597_d_n9 * ddt_scale);
        let eq10_e1598_d_n10: f64 = (eq10_e1597_d_n10 * ddt_scale);
        let eq10_e1598_d_n11: f64 = (eq10_e1597_d_n11 * ddt_scale);
        let eq10_e1598_d_n12: f64 = (eq10_e1597_d_n12 * ddt_scale);
        let eq10_e1598_d_n13: f64 = (eq10_e1597_d_n13 * ddt_scale);
        let eq10_e1598_d_b0: f64 = (eq10_e1597_d_b0 * ddt_scale);
        let eq10_e1598_d_b1: f64 = (eq10_e1597_d_b1 * ddt_scale);
        let eq10_e1598_d_b2: f64 = (eq10_e1597_d_b2 * ddt_scale);
        let eq10_e1598_d_b3: f64 = (eq10_e1597_d_b3 * ddt_scale);
        let eq10_e1598_d_b4: f64 = (eq10_e1597_d_b4 * ddt_scale);
        let eq10_e1598_d_b5: f64 = (eq10_e1597_d_b5 * ddt_scale);
        let eq10_e1598_d_b6: f64 = (eq10_e1597_d_b6 * ddt_scale);
        let eq10_e1598_d_b7: f64 = (eq10_e1597_d_b7 * ddt_scale);
        let eq10_e1598_d_b8: f64 = (eq10_e1597_d_b8 * ddt_scale);
        let eq10_e1598_d_b9: f64 = (eq10_e1597_d_b9 * ddt_scale);
        let eq10_e1598_d_b10: f64 = (eq10_e1597_d_b10 * ddt_scale);
        let eq10_e1598_d_b11: f64 = (eq10_e1597_d_b11 * ddt_scale);
        (eq10_e1598, eq10_e1598_d_n0, eq10_e1598_d_n1, eq10_e1598_d_n2, eq10_e1598_d_n3, eq10_e1598_d_n4, eq10_e1598_d_n5, eq10_e1598_d_n6, eq10_e1598_d_n7, eq10_e1598_d_n8, eq10_e1598_d_n9, eq10_e1598_d_n10, eq10_e1598_d_n11, eq10_e1598_d_n12, eq10_e1598_d_n13, eq10_e1598_d_b0, eq10_e1598_d_b1, eq10_e1598_d_b2, eq10_e1598_d_b3, eq10_e1598_d_b4, eq10_e1598_d_b5, eq10_e1598_d_b6, eq10_e1598_d_b7, eq10_e1598_d_b8, eq10_e1598_d_b9, eq10_e1598_d_b10, eq10_e1598_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1600;
        let eq10_node_derivatives: [f64; 14] = [eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13];
        let eq10_branch_derivatives: [f64; 12] = [eq10_e1600_d_b0, eq10_e1600_d_b1, eq10_e1600_d_b2, eq10_e1600_d_b3, eq10_e1600_d_b4, eq10_e1600_d_b5, eq10_e1600_d_b6, eq10_e1600_d_b7, eq10_e1600_d_b8, eq10_e1600_d_b9, eq10_e1600_d_b10, eq10_e1600_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1626, eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13, eq11_e1626_d_b0, eq11_e1626_d_b1, eq11_e1626_d_b2, eq11_e1626_d_b3, eq11_e1626_d_b4, eq11_e1626_d_b5, eq11_e1626_d_b6, eq11_e1626_d_b7, eq11_e1626_d_b8, eq11_e1626_d_b9, eq11_e1626_d_b10, eq11_e1626_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq11_e1610: f64 = (1.0 - s.v[211]);
        let eq11_e1610_d_n0: f64 = (-s.dn[211][0]);
        let eq11_e1610_d_n1: f64 = (-s.dn[211][1]);
        let eq11_e1610_d_n2: f64 = (-s.dn[211][2]);
        let eq11_e1610_d_n3: f64 = (-s.dn[211][3]);
        let eq11_e1610_d_n4: f64 = (-s.dn[211][4]);
        let eq11_e1610_d_n5: f64 = (-s.dn[211][5]);
        let eq11_e1610_d_n6: f64 = (-s.dn[211][6]);
        let eq11_e1610_d_n7: f64 = (-s.dn[211][7]);
        let eq11_e1610_d_n8: f64 = (-s.dn[211][8]);
        let eq11_e1610_d_n9: f64 = (-s.dn[211][9]);
        let eq11_e1610_d_n10: f64 = (-s.dn[211][10]);
        let eq11_e1610_d_n11: f64 = (-s.dn[211][11]);
        let eq11_e1610_d_n12: f64 = (-s.dn[211][12]);
        let eq11_e1610_d_n13: f64 = (-s.dn[211][13]);
        let eq11_e1610_d_b0: f64 = (-s.db[211][0]);
        let eq11_e1610_d_b1: f64 = (-s.db[211][1]);
        let eq11_e1610_d_b2: f64 = (-s.db[211][2]);
        let eq11_e1610_d_b3: f64 = (-s.db[211][3]);
        let eq11_e1610_d_b4: f64 = (-s.db[211][4]);
        let eq11_e1610_d_b5: f64 = (-s.db[211][5]);
        let eq11_e1610_d_b6: f64 = (-s.db[211][6]);
        let eq11_e1610_d_b7: f64 = (-s.db[211][7]);
        let eq11_e1610_d_b8: f64 = (-s.db[211][8]);
        let eq11_e1610_d_b9: f64 = (-s.db[211][9]);
        let eq11_e1610_d_b10: f64 = (-s.db[211][10]);
        let eq11_e1610_d_b11: f64 = (-s.db[211][11]);
        let eq11_e1612: f64 = (eq11_e1610 * s.v[622]);
        let eq11_e1612_d_n0: f64 = ((eq11_e1610_d_n0 * s.v[622]) + (eq11_e1610 * s.dn[622][0]));
        let eq11_e1612_d_n1: f64 = ((eq11_e1610_d_n1 * s.v[622]) + (eq11_e1610 * s.dn[622][1]));
        let eq11_e1612_d_n2: f64 = ((eq11_e1610_d_n2 * s.v[622]) + (eq11_e1610 * s.dn[622][2]));
        let eq11_e1612_d_n3: f64 = ((eq11_e1610_d_n3 * s.v[622]) + (eq11_e1610 * s.dn[622][3]));
        let eq11_e1612_d_n4: f64 = ((eq11_e1610_d_n4 * s.v[622]) + (eq11_e1610 * s.dn[622][4]));
        let eq11_e1612_d_n5: f64 = ((eq11_e1610_d_n5 * s.v[622]) + (eq11_e1610 * s.dn[622][5]));
        let eq11_e1612_d_n6: f64 = ((eq11_e1610_d_n6 * s.v[622]) + (eq11_e1610 * s.dn[622][6]));
        let eq11_e1612_d_n7: f64 = ((eq11_e1610_d_n7 * s.v[622]) + (eq11_e1610 * s.dn[622][7]));
        let eq11_e1612_d_n8: f64 = ((eq11_e1610_d_n8 * s.v[622]) + (eq11_e1610 * s.dn[622][8]));
        let eq11_e1612_d_n9: f64 = ((eq11_e1610_d_n9 * s.v[622]) + (eq11_e1610 * s.dn[622][9]));
        let eq11_e1612_d_n10: f64 = ((eq11_e1610_d_n10 * s.v[622]) + (eq11_e1610 * s.dn[622][10]));
        let eq11_e1612_d_n11: f64 = ((eq11_e1610_d_n11 * s.v[622]) + (eq11_e1610 * s.dn[622][11]));
        let eq11_e1612_d_n12: f64 = ((eq11_e1610_d_n12 * s.v[622]) + (eq11_e1610 * s.dn[622][12]));
        let eq11_e1612_d_n13: f64 = ((eq11_e1610_d_n13 * s.v[622]) + (eq11_e1610 * s.dn[622][13]));
        let eq11_e1612_d_b0: f64 = ((eq11_e1610_d_b0 * s.v[622]) + (eq11_e1610 * s.db[622][0]));
        let eq11_e1612_d_b1: f64 = ((eq11_e1610_d_b1 * s.v[622]) + (eq11_e1610 * s.db[622][1]));
        let eq11_e1612_d_b2: f64 = ((eq11_e1610_d_b2 * s.v[622]) + (eq11_e1610 * s.db[622][2]));
        let eq11_e1612_d_b3: f64 = ((eq11_e1610_d_b3 * s.v[622]) + (eq11_e1610 * s.db[622][3]));
        let eq11_e1612_d_b4: f64 = ((eq11_e1610_d_b4 * s.v[622]) + (eq11_e1610 * s.db[622][4]));
        let eq11_e1612_d_b5: f64 = ((eq11_e1610_d_b5 * s.v[622]) + (eq11_e1610 * s.db[622][5]));
        let eq11_e1612_d_b6: f64 = ((eq11_e1610_d_b6 * s.v[622]) + (eq11_e1610 * s.db[622][6]));
        let eq11_e1612_d_b7: f64 = ((eq11_e1610_d_b7 * s.v[622]) + (eq11_e1610 * s.db[622][7]));
        let eq11_e1612_d_b8: f64 = ((eq11_e1610_d_b8 * s.v[622]) + (eq11_e1610 * s.db[622][8]));
        let eq11_e1612_d_b9: f64 = ((eq11_e1610_d_b9 * s.v[622]) + (eq11_e1610 * s.db[622][9]));
        let eq11_e1612_d_b10: f64 = ((eq11_e1610_d_b10 * s.v[622]) + (eq11_e1610 * s.db[622][10]));
        let eq11_e1612_d_b11: f64 = ((eq11_e1610_d_b11 * s.v[622]) + (eq11_e1610 * s.db[622][11]));
        let eq11_e1614: f64 = (eq11_e1612 * s.v[199]);
        let eq11_e1614_d_n0: f64 = (eq11_e1612_d_n0 * s.v[199]);
        let eq11_e1614_d_n1: f64 = (eq11_e1612_d_n1 * s.v[199]);
        let eq11_e1614_d_n2: f64 = (eq11_e1612_d_n2 * s.v[199]);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * s.v[199]);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * s.v[199]);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * s.v[199]);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * s.v[199]);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * s.v[199]);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * s.v[199]);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * s.v[199]);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * s.v[199]);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * s.v[199]);
        let eq11_e1614_d_n12: f64 = (eq11_e1612_d_n12 * s.v[199]);
        let eq11_e1614_d_n13: f64 = (eq11_e1612_d_n13 * s.v[199]);
        let eq11_e1614_d_b0: f64 = (eq11_e1612_d_b0 * s.v[199]);
        let eq11_e1614_d_b1: f64 = (eq11_e1612_d_b1 * s.v[199]);
        let eq11_e1614_d_b2: f64 = (eq11_e1612_d_b2 * s.v[199]);
        let eq11_e1614_d_b3: f64 = (eq11_e1612_d_b3 * s.v[199]);
        let eq11_e1614_d_b4: f64 = (eq11_e1612_d_b4 * s.v[199]);
        let eq11_e1614_d_b5: f64 = (eq11_e1612_d_b5 * s.v[199]);
        let eq11_e1614_d_b6: f64 = (eq11_e1612_d_b6 * s.v[199]);
        let eq11_e1614_d_b7: f64 = (eq11_e1612_d_b7 * s.v[199]);
        let eq11_e1614_d_b8: f64 = (eq11_e1612_d_b8 * s.v[199]);
        let eq11_e1614_d_b9: f64 = (eq11_e1612_d_b9 * s.v[199]);
        let eq11_e1614_d_b10: f64 = (eq11_e1612_d_b10 * s.v[199]);
        let eq11_e1614_d_b11: f64 = (eq11_e1612_d_b11 * s.v[199]);
        let eq11_e1616: f64 = (eq11_e1614 * s.v[183]);
        let eq11_e1616_d_n0: f64 = (eq11_e1614_d_n0 * s.v[183]);
        let eq11_e1616_d_n1: f64 = (eq11_e1614_d_n1 * s.v[183]);
        let eq11_e1616_d_n2: f64 = (eq11_e1614_d_n2 * s.v[183]);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * s.v[183]);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * s.v[183]);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * s.v[183]);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * s.v[183]);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * s.v[183]);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * s.v[183]);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * s.v[183]);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * s.v[183]);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * s.v[183]);
        let eq11_e1616_d_n12: f64 = (eq11_e1614_d_n12 * s.v[183]);
        let eq11_e1616_d_n13: f64 = (eq11_e1614_d_n13 * s.v[183]);
        let eq11_e1616_d_b0: f64 = (eq11_e1614_d_b0 * s.v[183]);
        let eq11_e1616_d_b1: f64 = (eq11_e1614_d_b1 * s.v[183]);
        let eq11_e1616_d_b2: f64 = (eq11_e1614_d_b2 * s.v[183]);
        let eq11_e1616_d_b3: f64 = (eq11_e1614_d_b3 * s.v[183]);
        let eq11_e1616_d_b4: f64 = (eq11_e1614_d_b4 * s.v[183]);
        let eq11_e1616_d_b5: f64 = (eq11_e1614_d_b5 * s.v[183]);
        let eq11_e1616_d_b6: f64 = (eq11_e1614_d_b6 * s.v[183]);
        let eq11_e1616_d_b7: f64 = (eq11_e1614_d_b7 * s.v[183]);
        let eq11_e1616_d_b8: f64 = (eq11_e1614_d_b8 * s.v[183]);
        let eq11_e1616_d_b9: f64 = (eq11_e1614_d_b9 * s.v[183]);
        let eq11_e1616_d_b10: f64 = (eq11_e1614_d_b10 * s.v[183]);
        let eq11_e1616_d_b11: f64 = (eq11_e1614_d_b11 * s.v[183]);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n0: f64 = (eq11_e1616_d_n0 * p.p2);
        let eq11_e1618_d_n1: f64 = (eq11_e1616_d_n1 * p.p2);
        let eq11_e1618_d_n2: f64 = (eq11_e1616_d_n2 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1618_d_n12: f64 = (eq11_e1616_d_n12 * p.p2);
        let eq11_e1618_d_n13: f64 = (eq11_e1616_d_n13 * p.p2);
        let eq11_e1618_d_b0: f64 = (eq11_e1616_d_b0 * p.p2);
        let eq11_e1618_d_b1: f64 = (eq11_e1616_d_b1 * p.p2);
        let eq11_e1618_d_b2: f64 = (eq11_e1616_d_b2 * p.p2);
        let eq11_e1618_d_b3: f64 = (eq11_e1616_d_b3 * p.p2);
        let eq11_e1618_d_b4: f64 = (eq11_e1616_d_b4 * p.p2);
        let eq11_e1618_d_b5: f64 = (eq11_e1616_d_b5 * p.p2);
        let eq11_e1618_d_b6: f64 = (eq11_e1616_d_b6 * p.p2);
        let eq11_e1618_d_b7: f64 = (eq11_e1616_d_b7 * p.p2);
        let eq11_e1618_d_b8: f64 = (eq11_e1616_d_b8 * p.p2);
        let eq11_e1618_d_b9: f64 = (eq11_e1616_d_b9 * p.p2);
        let eq11_e1618_d_b10: f64 = (eq11_e1616_d_b10 * p.p2);
        let eq11_e1618_d_b11: f64 = (eq11_e1616_d_b11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * s.v[184]);
        let eq11_e1620_d_n0: f64 = (eq11_e1618_d_n0 * s.v[184]);
        let eq11_e1620_d_n1: f64 = (eq11_e1618_d_n1 * s.v[184]);
        let eq11_e1620_d_n2: f64 = (eq11_e1618_d_n2 * s.v[184]);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * s.v[184]);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * s.v[184]);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * s.v[184]);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * s.v[184]);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * s.v[184]);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * s.v[184]);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * s.v[184]);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * s.v[184]);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * s.v[184]);
        let eq11_e1620_d_n12: f64 = (eq11_e1618_d_n12 * s.v[184]);
        let eq11_e1620_d_n13: f64 = (eq11_e1618_d_n13 * s.v[184]);
        let eq11_e1620_d_b0: f64 = (eq11_e1618_d_b0 * s.v[184]);
        let eq11_e1620_d_b1: f64 = (eq11_e1618_d_b1 * s.v[184]);
        let eq11_e1620_d_b2: f64 = (eq11_e1618_d_b2 * s.v[184]);
        let eq11_e1620_d_b3: f64 = (eq11_e1618_d_b3 * s.v[184]);
        let eq11_e1620_d_b4: f64 = (eq11_e1618_d_b4 * s.v[184]);
        let eq11_e1620_d_b5: f64 = (eq11_e1618_d_b5 * s.v[184]);
        let eq11_e1620_d_b6: f64 = (eq11_e1618_d_b6 * s.v[184]);
        let eq11_e1620_d_b7: f64 = (eq11_e1618_d_b7 * s.v[184]);
        let eq11_e1620_d_b8: f64 = (eq11_e1618_d_b8 * s.v[184]);
        let eq11_e1620_d_b9: f64 = (eq11_e1618_d_b9 * s.v[184]);
        let eq11_e1620_d_b10: f64 = (eq11_e1618_d_b10 * s.v[184]);
        let eq11_e1620_d_b11: f64 = (eq11_e1618_d_b11 * s.v[184]);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n0: f64 = (eq11_e1620_d_n0 * (nv12 - 0.0));
        let eq11_e1622_d_n1: f64 = (eq11_e1620_d_n1 * (nv12 - 0.0));
        let eq11_e1622_d_n2: f64 = (eq11_e1620_d_n2 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1622_d_n12: f64 = ((eq11_e1620_d_n12 * (nv12 - 0.0)) + eq11_e1620);
        let eq11_e1622_d_n13: f64 = (eq11_e1620_d_n13 * (nv12 - 0.0));
        let eq11_e1622_d_b0: f64 = (eq11_e1620_d_b0 * (nv12 - 0.0));
        let eq11_e1622_d_b1: f64 = (eq11_e1620_d_b1 * (nv12 - 0.0));
        let eq11_e1622_d_b2: f64 = (eq11_e1620_d_b2 * (nv12 - 0.0));
        let eq11_e1622_d_b3: f64 = (eq11_e1620_d_b3 * (nv12 - 0.0));
        let eq11_e1622_d_b4: f64 = (eq11_e1620_d_b4 * (nv12 - 0.0));
        let eq11_e1622_d_b5: f64 = (eq11_e1620_d_b5 * (nv12 - 0.0));
        let eq11_e1622_d_b6: f64 = (eq11_e1620_d_b6 * (nv12 - 0.0));
        let eq11_e1622_d_b7: f64 = (eq11_e1620_d_b7 * (nv12 - 0.0));
        let eq11_e1622_d_b8: f64 = (eq11_e1620_d_b8 * (nv12 - 0.0));
        let eq11_e1622_d_b9: f64 = (eq11_e1620_d_b9 * (nv12 - 0.0));
        let eq11_e1622_d_b10: f64 = (eq11_e1620_d_b10 * (nv12 - 0.0));
        let eq11_e1622_d_b11: f64 = (eq11_e1620_d_b11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n0: f64 = (0.5 * eq11_e1622_d_n0);
        let eq11_e1623_d_n1: f64 = (0.5 * eq11_e1622_d_n1);
        let eq11_e1623_d_n2: f64 = (0.5 * eq11_e1622_d_n2);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1622_d_n12);
        let eq11_e1623_d_n13: f64 = (0.5 * eq11_e1622_d_n13);
        let eq11_e1623_d_b0: f64 = (0.5 * eq11_e1622_d_b0);
        let eq11_e1623_d_b1: f64 = (0.5 * eq11_e1622_d_b1);
        let eq11_e1623_d_b2: f64 = (0.5 * eq11_e1622_d_b2);
        let eq11_e1623_d_b3: f64 = (0.5 * eq11_e1622_d_b3);
        let eq11_e1623_d_b4: f64 = (0.5 * eq11_e1622_d_b4);
        let eq11_e1623_d_b5: f64 = (0.5 * eq11_e1622_d_b5);
        let eq11_e1623_d_b6: f64 = (0.5 * eq11_e1622_d_b6);
        let eq11_e1623_d_b7: f64 = (0.5 * eq11_e1622_d_b7);
        let eq11_e1623_d_b8: f64 = (0.5 * eq11_e1622_d_b8);
        let eq11_e1623_d_b9: f64 = (0.5 * eq11_e1622_d_b9);
        let eq11_e1623_d_b10: f64 = (0.5 * eq11_e1622_d_b10);
        let eq11_e1623_d_b11: f64 = (0.5 * eq11_e1622_d_b11);
        let eq11_e1624: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq11_e1623);
        let eq11_e1624_d_n0: f64 = (eq11_e1623_d_n0 * ddt_scale);
        let eq11_e1624_d_n1: f64 = (eq11_e1623_d_n1 * ddt_scale);
        let eq11_e1624_d_n2: f64 = (eq11_e1623_d_n2 * ddt_scale);
        let eq11_e1624_d_n3: f64 = (eq11_e1623_d_n3 * ddt_scale);
        let eq11_e1624_d_n4: f64 = (eq11_e1623_d_n4 * ddt_scale);
        let eq11_e1624_d_n5: f64 = (eq11_e1623_d_n5 * ddt_scale);
        let eq11_e1624_d_n6: f64 = (eq11_e1623_d_n6 * ddt_scale);
        let eq11_e1624_d_n7: f64 = (eq11_e1623_d_n7 * ddt_scale);
        let eq11_e1624_d_n8: f64 = (eq11_e1623_d_n8 * ddt_scale);
        let eq11_e1624_d_n9: f64 = (eq11_e1623_d_n9 * ddt_scale);
        let eq11_e1624_d_n10: f64 = (eq11_e1623_d_n10 * ddt_scale);
        let eq11_e1624_d_n11: f64 = (eq11_e1623_d_n11 * ddt_scale);
        let eq11_e1624_d_n12: f64 = (eq11_e1623_d_n12 * ddt_scale);
        let eq11_e1624_d_n13: f64 = (eq11_e1623_d_n13 * ddt_scale);
        let eq11_e1624_d_b0: f64 = (eq11_e1623_d_b0 * ddt_scale);
        let eq11_e1624_d_b1: f64 = (eq11_e1623_d_b1 * ddt_scale);
        let eq11_e1624_d_b2: f64 = (eq11_e1623_d_b2 * ddt_scale);
        let eq11_e1624_d_b3: f64 = (eq11_e1623_d_b3 * ddt_scale);
        let eq11_e1624_d_b4: f64 = (eq11_e1623_d_b4 * ddt_scale);
        let eq11_e1624_d_b5: f64 = (eq11_e1623_d_b5 * ddt_scale);
        let eq11_e1624_d_b6: f64 = (eq11_e1623_d_b6 * ddt_scale);
        let eq11_e1624_d_b7: f64 = (eq11_e1623_d_b7 * ddt_scale);
        let eq11_e1624_d_b8: f64 = (eq11_e1623_d_b8 * ddt_scale);
        let eq11_e1624_d_b9: f64 = (eq11_e1623_d_b9 * ddt_scale);
        let eq11_e1624_d_b10: f64 = (eq11_e1623_d_b10 * ddt_scale);
        let eq11_e1624_d_b11: f64 = (eq11_e1623_d_b11 * ddt_scale);
        (eq11_e1624, eq11_e1624_d_n0, eq11_e1624_d_n1, eq11_e1624_d_n2, eq11_e1624_d_n3, eq11_e1624_d_n4, eq11_e1624_d_n5, eq11_e1624_d_n6, eq11_e1624_d_n7, eq11_e1624_d_n8, eq11_e1624_d_n9, eq11_e1624_d_n10, eq11_e1624_d_n11, eq11_e1624_d_n12, eq11_e1624_d_n13, eq11_e1624_d_b0, eq11_e1624_d_b1, eq11_e1624_d_b2, eq11_e1624_d_b3, eq11_e1624_d_b4, eq11_e1624_d_b5, eq11_e1624_d_b6, eq11_e1624_d_b7, eq11_e1624_d_b8, eq11_e1624_d_b9, eq11_e1624_d_b10, eq11_e1624_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1626;
        let eq11_node_derivatives: [f64; 14] = [eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13];
        let eq11_branch_derivatives: [f64; 12] = [eq11_e1626_d_b0, eq11_e1626_d_b1, eq11_e1626_d_b2, eq11_e1626_d_b3, eq11_e1626_d_b4, eq11_e1626_d_b5, eq11_e1626_d_b6, eq11_e1626_d_b7, eq11_e1626_d_b8, eq11_e1626_d_b9, eq11_e1626_d_b10, eq11_e1626_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq17_e1686, eq17_e1686_d_n0, eq17_e1686_d_n1, eq17_e1686_d_n2, eq17_e1686_d_n3, eq17_e1686_d_n4, eq17_e1686_d_n5, eq17_e1686_d_n6, eq17_e1686_d_n7, eq17_e1686_d_n8, eq17_e1686_d_n9, eq17_e1686_d_n10, eq17_e1686_d_n11, eq17_e1686_d_n12, eq17_e1686_d_n13, eq17_e1686_d_b0, eq17_e1686_d_b1, eq17_e1686_d_b2, eq17_e1686_d_b3, eq17_e1686_d_b4, eq17_e1686_d_b5, eq17_e1686_d_b6, eq17_e1686_d_b7, eq17_e1686_d_b8, eq17_e1686_d_b9, eq17_e1686_d_b10, eq17_e1686_d_b11,) = {
    if ((!s.b[1620]) && s.b[1947]) {
        let eq17_e1684: f64 = (s.v[379] * s.v[974]);
        let eq17_e1684_d_n0: f64 = ((s.dn[379][0] * s.v[974]) + (s.v[379] * s.dn[974][0]));
        let eq17_e1684_d_n1: f64 = ((s.dn[379][1] * s.v[974]) + (s.v[379] * s.dn[974][1]));
        let eq17_e1684_d_n2: f64 = ((s.dn[379][2] * s.v[974]) + (s.v[379] * s.dn[974][2]));
        let eq17_e1684_d_n3: f64 = ((s.dn[379][3] * s.v[974]) + (s.v[379] * s.dn[974][3]));
        let eq17_e1684_d_n4: f64 = ((s.dn[379][4] * s.v[974]) + (s.v[379] * s.dn[974][4]));
        let eq17_e1684_d_n5: f64 = ((s.dn[379][5] * s.v[974]) + (s.v[379] * s.dn[974][5]));
        let eq17_e1684_d_n6: f64 = ((s.dn[379][6] * s.v[974]) + (s.v[379] * s.dn[974][6]));
        let eq17_e1684_d_n7: f64 = ((s.dn[379][7] * s.v[974]) + (s.v[379] * s.dn[974][7]));
        let eq17_e1684_d_n8: f64 = ((s.dn[379][8] * s.v[974]) + (s.v[379] * s.dn[974][8]));
        let eq17_e1684_d_n9: f64 = ((s.dn[379][9] * s.v[974]) + (s.v[379] * s.dn[974][9]));
        let eq17_e1684_d_n10: f64 = ((s.dn[379][10] * s.v[974]) + (s.v[379] * s.dn[974][10]));
        let eq17_e1684_d_n11: f64 = ((s.dn[379][11] * s.v[974]) + (s.v[379] * s.dn[974][11]));
        let eq17_e1684_d_n12: f64 = ((s.dn[379][12] * s.v[974]) + (s.v[379] * s.dn[974][12]));
        let eq17_e1684_d_n13: f64 = ((s.dn[379][13] * s.v[974]) + (s.v[379] * s.dn[974][13]));
        let eq17_e1684_d_b0: f64 = ((s.db[379][0] * s.v[974]) + (s.v[379] * s.db[974][0]));
        let eq17_e1684_d_b1: f64 = ((s.db[379][1] * s.v[974]) + (s.v[379] * s.db[974][1]));
        let eq17_e1684_d_b2: f64 = ((s.db[379][2] * s.v[974]) + (s.v[379] * s.db[974][2]));
        let eq17_e1684_d_b3: f64 = ((s.db[379][3] * s.v[974]) + (s.v[379] * s.db[974][3]));
        let eq17_e1684_d_b4: f64 = ((s.db[379][4] * s.v[974]) + (s.v[379] * s.db[974][4]));
        let eq17_e1684_d_b5: f64 = ((s.db[379][5] * s.v[974]) + (s.v[379] * s.db[974][5]));
        let eq17_e1684_d_b6: f64 = ((s.db[379][6] * s.v[974]) + (s.v[379] * s.db[974][6]));
        let eq17_e1684_d_b7: f64 = ((s.db[379][7] * s.v[974]) + (s.v[379] * s.db[974][7]));
        let eq17_e1684_d_b8: f64 = ((s.db[379][8] * s.v[974]) + (s.v[379] * s.db[974][8]));
        let eq17_e1684_d_b9: f64 = ((s.db[379][9] * s.v[974]) + (s.v[379] * s.db[974][9]));
        let eq17_e1684_d_b10: f64 = ((s.db[379][10] * s.v[974]) + (s.v[379] * s.db[974][10]));
        let eq17_e1684_d_b11: f64 = ((s.db[379][11] * s.v[974]) + (s.v[379] * s.db[974][11]));
        (eq17_e1684, eq17_e1684_d_n0, eq17_e1684_d_n1, eq17_e1684_d_n2, eq17_e1684_d_n3, eq17_e1684_d_n4, eq17_e1684_d_n5, eq17_e1684_d_n6, eq17_e1684_d_n7, eq17_e1684_d_n8, eq17_e1684_d_n9, eq17_e1684_d_n10, eq17_e1684_d_n11, eq17_e1684_d_n12, eq17_e1684_d_n13, eq17_e1684_d_b0, eq17_e1684_d_b1, eq17_e1684_d_b2, eq17_e1684_d_b3, eq17_e1684_d_b4, eq17_e1684_d_b5, eq17_e1684_d_b6, eq17_e1684_d_b7, eq17_e1684_d_b8, eq17_e1684_d_b9, eq17_e1684_d_b10, eq17_e1684_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1686;
        let eq17_node_derivatives: [f64; 14] = [eq17_e1686_d_n0, eq17_e1686_d_n1, eq17_e1686_d_n2, eq17_e1686_d_n3, eq17_e1686_d_n4, eq17_e1686_d_n5, eq17_e1686_d_n6, eq17_e1686_d_n7, eq17_e1686_d_n8, eq17_e1686_d_n9, eq17_e1686_d_n10, eq17_e1686_d_n11, eq17_e1686_d_n12, eq17_e1686_d_n13];
        let eq17_branch_derivatives: [f64; 12] = [eq17_e1686_d_b0, eq17_e1686_d_b1, eq17_e1686_d_b2, eq17_e1686_d_b3, eq17_e1686_d_b4, eq17_e1686_d_b5, eq17_e1686_d_b6, eq17_e1686_d_b7, eq17_e1686_d_b8, eq17_e1686_d_b9, eq17_e1686_d_b10, eq17_e1686_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1763, eq23_e1763_d_n0, eq23_e1763_d_n1, eq23_e1763_d_n2, eq23_e1763_d_n3, eq23_e1763_d_n4, eq23_e1763_d_n5, eq23_e1763_d_n6, eq23_e1763_d_n7, eq23_e1763_d_n8, eq23_e1763_d_n9, eq23_e1763_d_n10, eq23_e1763_d_n11, eq23_e1763_d_n12, eq23_e1763_d_n13, eq23_e1763_d_b0, eq23_e1763_d_b1, eq23_e1763_d_b2, eq23_e1763_d_b3, eq23_e1763_d_b4, eq23_e1763_d_b5, eq23_e1763_d_b6, eq23_e1763_d_b7, eq23_e1763_d_b8, eq23_e1763_d_b9, eq23_e1763_d_b10, eq23_e1763_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq23_e1759: f64 = (-s.v[629]);
        let eq23_e1759_d_n0: f64 = (-s.dn[629][0]);
        let eq23_e1759_d_n1: f64 = (-s.dn[629][1]);
        let eq23_e1759_d_n2: f64 = (-s.dn[629][2]);
        let eq23_e1759_d_n3: f64 = (-s.dn[629][3]);
        let eq23_e1759_d_n4: f64 = (-s.dn[629][4]);
        let eq23_e1759_d_n5: f64 = (-s.dn[629][5]);
        let eq23_e1759_d_n6: f64 = (-s.dn[629][6]);
        let eq23_e1759_d_n7: f64 = (-s.dn[629][7]);
        let eq23_e1759_d_n8: f64 = (-s.dn[629][8]);
        let eq23_e1759_d_n9: f64 = (-s.dn[629][9]);
        let eq23_e1759_d_n10: f64 = (-s.dn[629][10]);
        let eq23_e1759_d_n11: f64 = (-s.dn[629][11]);
        let eq23_e1759_d_n12: f64 = (-s.dn[629][12]);
        let eq23_e1759_d_n13: f64 = (-s.dn[629][13]);
        let eq23_e1759_d_b0: f64 = (-s.db[629][0]);
        let eq23_e1759_d_b1: f64 = (-s.db[629][1]);
        let eq23_e1759_d_b2: f64 = (-s.db[629][2]);
        let eq23_e1759_d_b3: f64 = (-s.db[629][3]);
        let eq23_e1759_d_b4: f64 = (-s.db[629][4]);
        let eq23_e1759_d_b5: f64 = (-s.db[629][5]);
        let eq23_e1759_d_b6: f64 = (-s.db[629][6]);
        let eq23_e1759_d_b7: f64 = (-s.db[629][7]);
        let eq23_e1759_d_b8: f64 = (-s.db[629][8]);
        let eq23_e1759_d_b9: f64 = (-s.db[629][9]);
        let eq23_e1759_d_b10: f64 = (-s.db[629][10]);
        let eq23_e1759_d_b11: f64 = (-s.db[629][11]);
        let eq23_e1761: f64 = (eq23_e1759 * (nv13 - 0.0));
        let eq23_e1761_d_n0: f64 = (eq23_e1759_d_n0 * (nv13 - 0.0));
        let eq23_e1761_d_n1: f64 = (eq23_e1759_d_n1 * (nv13 - 0.0));
        let eq23_e1761_d_n2: f64 = (eq23_e1759_d_n2 * (nv13 - 0.0));
        let eq23_e1761_d_n3: f64 = (eq23_e1759_d_n3 * (nv13 - 0.0));
        let eq23_e1761_d_n4: f64 = (eq23_e1759_d_n4 * (nv13 - 0.0));
        let eq23_e1761_d_n5: f64 = (eq23_e1759_d_n5 * (nv13 - 0.0));
        let eq23_e1761_d_n6: f64 = (eq23_e1759_d_n6 * (nv13 - 0.0));
        let eq23_e1761_d_n7: f64 = (eq23_e1759_d_n7 * (nv13 - 0.0));
        let eq23_e1761_d_n8: f64 = (eq23_e1759_d_n8 * (nv13 - 0.0));
        let eq23_e1761_d_n9: f64 = (eq23_e1759_d_n9 * (nv13 - 0.0));
        let eq23_e1761_d_n10: f64 = (eq23_e1759_d_n10 * (nv13 - 0.0));
        let eq23_e1761_d_n11: f64 = (eq23_e1759_d_n11 * (nv13 - 0.0));
        let eq23_e1761_d_n12: f64 = (eq23_e1759_d_n12 * (nv13 - 0.0));
        let eq23_e1761_d_n13: f64 = ((eq23_e1759_d_n13 * (nv13 - 0.0)) + eq23_e1759);
        let eq23_e1761_d_b0: f64 = (eq23_e1759_d_b0 * (nv13 - 0.0));
        let eq23_e1761_d_b1: f64 = (eq23_e1759_d_b1 * (nv13 - 0.0));
        let eq23_e1761_d_b2: f64 = (eq23_e1759_d_b2 * (nv13 - 0.0));
        let eq23_e1761_d_b3: f64 = (eq23_e1759_d_b3 * (nv13 - 0.0));
        let eq23_e1761_d_b4: f64 = (eq23_e1759_d_b4 * (nv13 - 0.0));
        let eq23_e1761_d_b5: f64 = (eq23_e1759_d_b5 * (nv13 - 0.0));
        let eq23_e1761_d_b6: f64 = (eq23_e1759_d_b6 * (nv13 - 0.0));
        let eq23_e1761_d_b7: f64 = (eq23_e1759_d_b7 * (nv13 - 0.0));
        let eq23_e1761_d_b8: f64 = (eq23_e1759_d_b8 * (nv13 - 0.0));
        let eq23_e1761_d_b9: f64 = (eq23_e1759_d_b9 * (nv13 - 0.0));
        let eq23_e1761_d_b10: f64 = (eq23_e1759_d_b10 * (nv13 - 0.0));
        let eq23_e1761_d_b11: f64 = (eq23_e1759_d_b11 * (nv13 - 0.0));
        (eq23_e1761, eq23_e1761_d_n0, eq23_e1761_d_n1, eq23_e1761_d_n2, eq23_e1761_d_n3, eq23_e1761_d_n4, eq23_e1761_d_n5, eq23_e1761_d_n6, eq23_e1761_d_n7, eq23_e1761_d_n8, eq23_e1761_d_n9, eq23_e1761_d_n10, eq23_e1761_d_n11, eq23_e1761_d_n12, eq23_e1761_d_n13, eq23_e1761_d_b0, eq23_e1761_d_b1, eq23_e1761_d_b2, eq23_e1761_d_b3, eq23_e1761_d_b4, eq23_e1761_d_b5, eq23_e1761_d_b6, eq23_e1761_d_b7, eq23_e1761_d_b8, eq23_e1761_d_b9, eq23_e1761_d_b10, eq23_e1761_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1763;
        let eq23_node_derivatives: [f64; 14] = [eq23_e1763_d_n0, eq23_e1763_d_n1, eq23_e1763_d_n2, eq23_e1763_d_n3, eq23_e1763_d_n4, eq23_e1763_d_n5, eq23_e1763_d_n6, eq23_e1763_d_n7, eq23_e1763_d_n8, eq23_e1763_d_n9, eq23_e1763_d_n10, eq23_e1763_d_n11, eq23_e1763_d_n12, eq23_e1763_d_n13];
        let eq23_branch_derivatives: [f64; 12] = [eq23_e1763_d_b0, eq23_e1763_d_b1, eq23_e1763_d_b2, eq23_e1763_d_b3, eq23_e1763_d_b4, eq23_e1763_d_b5, eq23_e1763_d_b6, eq23_e1763_d_b7, eq23_e1763_d_b8, eq23_e1763_d_b9, eq23_e1763_d_b10, eq23_e1763_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1784, eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13, eq24_e1784_d_b0, eq24_e1784_d_b1, eq24_e1784_d_b2, eq24_e1784_d_b3, eq24_e1784_d_b4, eq24_e1784_d_b5, eq24_e1784_d_b6, eq24_e1784_d_b7, eq24_e1784_d_b8, eq24_e1784_d_b9, eq24_e1784_d_b10, eq24_e1784_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq24_e1773: f64 = (s.v[622] * s.v[199]);
        let eq24_e1773_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq24_e1773_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq24_e1773_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq24_e1773_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq24_e1773_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq24_e1773_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq24_e1773_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq24_e1773_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq24_e1773_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq24_e1773_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq24_e1773_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq24_e1773_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq24_e1773_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq24_e1773_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq24_e1773_d_b0: f64 = (s.db[622][0] * s.v[199]);
        let eq24_e1773_d_b1: f64 = (s.db[622][1] * s.v[199]);
        let eq24_e1773_d_b2: f64 = (s.db[622][2] * s.v[199]);
        let eq24_e1773_d_b3: f64 = (s.db[622][3] * s.v[199]);
        let eq24_e1773_d_b4: f64 = (s.db[622][4] * s.v[199]);
        let eq24_e1773_d_b5: f64 = (s.db[622][5] * s.v[199]);
        let eq24_e1773_d_b6: f64 = (s.db[622][6] * s.v[199]);
        let eq24_e1773_d_b7: f64 = (s.db[622][7] * s.v[199]);
        let eq24_e1773_d_b8: f64 = (s.db[622][8] * s.v[199]);
        let eq24_e1773_d_b9: f64 = (s.db[622][9] * s.v[199]);
        let eq24_e1773_d_b10: f64 = (s.db[622][10] * s.v[199]);
        let eq24_e1773_d_b11: f64 = (s.db[622][11] * s.v[199]);
        let eq24_e1775: f64 = (eq24_e1773 * s.v[183]);
        let eq24_e1775_d_n0: f64 = (eq24_e1773_d_n0 * s.v[183]);
        let eq24_e1775_d_n1: f64 = (eq24_e1773_d_n1 * s.v[183]);
        let eq24_e1775_d_n2: f64 = (eq24_e1773_d_n2 * s.v[183]);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * s.v[183]);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * s.v[183]);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * s.v[183]);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * s.v[183]);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * s.v[183]);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * s.v[183]);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * s.v[183]);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * s.v[183]);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * s.v[183]);
        let eq24_e1775_d_n12: f64 = (eq24_e1773_d_n12 * s.v[183]);
        let eq24_e1775_d_n13: f64 = (eq24_e1773_d_n13 * s.v[183]);
        let eq24_e1775_d_b0: f64 = (eq24_e1773_d_b0 * s.v[183]);
        let eq24_e1775_d_b1: f64 = (eq24_e1773_d_b1 * s.v[183]);
        let eq24_e1775_d_b2: f64 = (eq24_e1773_d_b2 * s.v[183]);
        let eq24_e1775_d_b3: f64 = (eq24_e1773_d_b3 * s.v[183]);
        let eq24_e1775_d_b4: f64 = (eq24_e1773_d_b4 * s.v[183]);
        let eq24_e1775_d_b5: f64 = (eq24_e1773_d_b5 * s.v[183]);
        let eq24_e1775_d_b6: f64 = (eq24_e1773_d_b6 * s.v[183]);
        let eq24_e1775_d_b7: f64 = (eq24_e1773_d_b7 * s.v[183]);
        let eq24_e1775_d_b8: f64 = (eq24_e1773_d_b8 * s.v[183]);
        let eq24_e1775_d_b9: f64 = (eq24_e1773_d_b9 * s.v[183]);
        let eq24_e1775_d_b10: f64 = (eq24_e1773_d_b10 * s.v[183]);
        let eq24_e1775_d_b11: f64 = (eq24_e1773_d_b11 * s.v[183]);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n0: f64 = (eq24_e1775_d_n0 * p.p2);
        let eq24_e1777_d_n1: f64 = (eq24_e1775_d_n1 * p.p2);
        let eq24_e1777_d_n2: f64 = (eq24_e1775_d_n2 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1777_d_n12: f64 = (eq24_e1775_d_n12 * p.p2);
        let eq24_e1777_d_n13: f64 = (eq24_e1775_d_n13 * p.p2);
        let eq24_e1777_d_b0: f64 = (eq24_e1775_d_b0 * p.p2);
        let eq24_e1777_d_b1: f64 = (eq24_e1775_d_b1 * p.p2);
        let eq24_e1777_d_b2: f64 = (eq24_e1775_d_b2 * p.p2);
        let eq24_e1777_d_b3: f64 = (eq24_e1775_d_b3 * p.p2);
        let eq24_e1777_d_b4: f64 = (eq24_e1775_d_b4 * p.p2);
        let eq24_e1777_d_b5: f64 = (eq24_e1775_d_b5 * p.p2);
        let eq24_e1777_d_b6: f64 = (eq24_e1775_d_b6 * p.p2);
        let eq24_e1777_d_b7: f64 = (eq24_e1775_d_b7 * p.p2);
        let eq24_e1777_d_b8: f64 = (eq24_e1775_d_b8 * p.p2);
        let eq24_e1777_d_b9: f64 = (eq24_e1775_d_b9 * p.p2);
        let eq24_e1777_d_b10: f64 = (eq24_e1775_d_b10 * p.p2);
        let eq24_e1777_d_b11: f64 = (eq24_e1775_d_b11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * s.v[184]);
        let eq24_e1779_d_n0: f64 = (eq24_e1777_d_n0 * s.v[184]);
        let eq24_e1779_d_n1: f64 = (eq24_e1777_d_n1 * s.v[184]);
        let eq24_e1779_d_n2: f64 = (eq24_e1777_d_n2 * s.v[184]);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * s.v[184]);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * s.v[184]);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * s.v[184]);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * s.v[184]);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * s.v[184]);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * s.v[184]);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * s.v[184]);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * s.v[184]);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * s.v[184]);
        let eq24_e1779_d_n12: f64 = (eq24_e1777_d_n12 * s.v[184]);
        let eq24_e1779_d_n13: f64 = (eq24_e1777_d_n13 * s.v[184]);
        let eq24_e1779_d_b0: f64 = (eq24_e1777_d_b0 * s.v[184]);
        let eq24_e1779_d_b1: f64 = (eq24_e1777_d_b1 * s.v[184]);
        let eq24_e1779_d_b2: f64 = (eq24_e1777_d_b2 * s.v[184]);
        let eq24_e1779_d_b3: f64 = (eq24_e1777_d_b3 * s.v[184]);
        let eq24_e1779_d_b4: f64 = (eq24_e1777_d_b4 * s.v[184]);
        let eq24_e1779_d_b5: f64 = (eq24_e1777_d_b5 * s.v[184]);
        let eq24_e1779_d_b6: f64 = (eq24_e1777_d_b6 * s.v[184]);
        let eq24_e1779_d_b7: f64 = (eq24_e1777_d_b7 * s.v[184]);
        let eq24_e1779_d_b8: f64 = (eq24_e1777_d_b8 * s.v[184]);
        let eq24_e1779_d_b9: f64 = (eq24_e1777_d_b9 * s.v[184]);
        let eq24_e1779_d_b10: f64 = (eq24_e1777_d_b10 * s.v[184]);
        let eq24_e1779_d_b11: f64 = (eq24_e1777_d_b11 * s.v[184]);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n0: f64 = (eq24_e1779_d_n0 * (nv12 - 0.0));
        let eq24_e1781_d_n1: f64 = (eq24_e1779_d_n1 * (nv12 - 0.0));
        let eq24_e1781_d_n2: f64 = (eq24_e1779_d_n2 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1781_d_n12: f64 = ((eq24_e1779_d_n12 * (nv12 - 0.0)) + eq24_e1779);
        let eq24_e1781_d_n13: f64 = (eq24_e1779_d_n13 * (nv12 - 0.0));
        let eq24_e1781_d_b0: f64 = (eq24_e1779_d_b0 * (nv12 - 0.0));
        let eq24_e1781_d_b1: f64 = (eq24_e1779_d_b1 * (nv12 - 0.0));
        let eq24_e1781_d_b2: f64 = (eq24_e1779_d_b2 * (nv12 - 0.0));
        let eq24_e1781_d_b3: f64 = (eq24_e1779_d_b3 * (nv12 - 0.0));
        let eq24_e1781_d_b4: f64 = (eq24_e1779_d_b4 * (nv12 - 0.0));
        let eq24_e1781_d_b5: f64 = (eq24_e1779_d_b5 * (nv12 - 0.0));
        let eq24_e1781_d_b6: f64 = (eq24_e1779_d_b6 * (nv12 - 0.0));
        let eq24_e1781_d_b7: f64 = (eq24_e1779_d_b7 * (nv12 - 0.0));
        let eq24_e1781_d_b8: f64 = (eq24_e1779_d_b8 * (nv12 - 0.0));
        let eq24_e1781_d_b9: f64 = (eq24_e1779_d_b9 * (nv12 - 0.0));
        let eq24_e1781_d_b10: f64 = (eq24_e1779_d_b10 * (nv12 - 0.0));
        let eq24_e1781_d_b11: f64 = (eq24_e1779_d_b11 * (nv12 - 0.0));
        let eq24_e1782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq24_e1781);
        let eq24_e1782_d_n0: f64 = (eq24_e1781_d_n0 * ddt_scale);
        let eq24_e1782_d_n1: f64 = (eq24_e1781_d_n1 * ddt_scale);
        let eq24_e1782_d_n2: f64 = (eq24_e1781_d_n2 * ddt_scale);
        let eq24_e1782_d_n3: f64 = (eq24_e1781_d_n3 * ddt_scale);
        let eq24_e1782_d_n4: f64 = (eq24_e1781_d_n4 * ddt_scale);
        let eq24_e1782_d_n5: f64 = (eq24_e1781_d_n5 * ddt_scale);
        let eq24_e1782_d_n6: f64 = (eq24_e1781_d_n6 * ddt_scale);
        let eq24_e1782_d_n7: f64 = (eq24_e1781_d_n7 * ddt_scale);
        let eq24_e1782_d_n8: f64 = (eq24_e1781_d_n8 * ddt_scale);
        let eq24_e1782_d_n9: f64 = (eq24_e1781_d_n9 * ddt_scale);
        let eq24_e1782_d_n10: f64 = (eq24_e1781_d_n10 * ddt_scale);
        let eq24_e1782_d_n11: f64 = (eq24_e1781_d_n11 * ddt_scale);
        let eq24_e1782_d_n12: f64 = (eq24_e1781_d_n12 * ddt_scale);
        let eq24_e1782_d_n13: f64 = (eq24_e1781_d_n13 * ddt_scale);
        let eq24_e1782_d_b0: f64 = (eq24_e1781_d_b0 * ddt_scale);
        let eq24_e1782_d_b1: f64 = (eq24_e1781_d_b1 * ddt_scale);
        let eq24_e1782_d_b2: f64 = (eq24_e1781_d_b2 * ddt_scale);
        let eq24_e1782_d_b3: f64 = (eq24_e1781_d_b3 * ddt_scale);
        let eq24_e1782_d_b4: f64 = (eq24_e1781_d_b4 * ddt_scale);
        let eq24_e1782_d_b5: f64 = (eq24_e1781_d_b5 * ddt_scale);
        let eq24_e1782_d_b6: f64 = (eq24_e1781_d_b6 * ddt_scale);
        let eq24_e1782_d_b7: f64 = (eq24_e1781_d_b7 * ddt_scale);
        let eq24_e1782_d_b8: f64 = (eq24_e1781_d_b8 * ddt_scale);
        let eq24_e1782_d_b9: f64 = (eq24_e1781_d_b9 * ddt_scale);
        let eq24_e1782_d_b10: f64 = (eq24_e1781_d_b10 * ddt_scale);
        let eq24_e1782_d_b11: f64 = (eq24_e1781_d_b11 * ddt_scale);
        (eq24_e1782, eq24_e1782_d_n0, eq24_e1782_d_n1, eq24_e1782_d_n2, eq24_e1782_d_n3, eq24_e1782_d_n4, eq24_e1782_d_n5, eq24_e1782_d_n6, eq24_e1782_d_n7, eq24_e1782_d_n8, eq24_e1782_d_n9, eq24_e1782_d_n10, eq24_e1782_d_n11, eq24_e1782_d_n12, eq24_e1782_d_n13, eq24_e1782_d_b0, eq24_e1782_d_b1, eq24_e1782_d_b2, eq24_e1782_d_b3, eq24_e1782_d_b4, eq24_e1782_d_b5, eq24_e1782_d_b6, eq24_e1782_d_b7, eq24_e1782_d_b8, eq24_e1782_d_b9, eq24_e1782_d_b10, eq24_e1782_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1784;
        let eq24_node_derivatives: [f64; 14] = [eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13];
        let eq24_branch_derivatives: [f64; 12] = [eq24_e1784_d_b0, eq24_e1784_d_b1, eq24_e1784_d_b2, eq24_e1784_d_b3, eq24_e1784_d_b4, eq24_e1784_d_b5, eq24_e1784_d_b6, eq24_e1784_d_b7, eq24_e1784_d_b8, eq24_e1784_d_b9, eq24_e1784_d_b10, eq24_e1784_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1814, eq26_e1814_d_n0, eq26_e1814_d_n1, eq26_e1814_d_n2, eq26_e1814_d_n3, eq26_e1814_d_n4, eq26_e1814_d_n5, eq26_e1814_d_n6, eq26_e1814_d_n7, eq26_e1814_d_n8, eq26_e1814_d_n9, eq26_e1814_d_n10, eq26_e1814_d_n11, eq26_e1814_d_n12, eq26_e1814_d_n13, eq26_e1814_d_b0, eq26_e1814_d_b1, eq26_e1814_d_b2, eq26_e1814_d_b3, eq26_e1814_d_b4, eq26_e1814_d_b5, eq26_e1814_d_b6, eq26_e1814_d_b7, eq26_e1814_d_b8, eq26_e1814_d_b9, eq26_e1814_d_b10, eq26_e1814_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq26_e1812: f64 = (s.v[628] * (nv13 - 0.0));
        let eq26_e1812_d_n0: f64 = (s.dn[628][0] * (nv13 - 0.0));
        let eq26_e1812_d_n1: f64 = (s.dn[628][1] * (nv13 - 0.0));
        let eq26_e1812_d_n2: f64 = (s.dn[628][2] * (nv13 - 0.0));
        let eq26_e1812_d_n3: f64 = (s.dn[628][3] * (nv13 - 0.0));
        let eq26_e1812_d_n4: f64 = (s.dn[628][4] * (nv13 - 0.0));
        let eq26_e1812_d_n5: f64 = (s.dn[628][5] * (nv13 - 0.0));
        let eq26_e1812_d_n6: f64 = (s.dn[628][6] * (nv13 - 0.0));
        let eq26_e1812_d_n7: f64 = (s.dn[628][7] * (nv13 - 0.0));
        let eq26_e1812_d_n8: f64 = (s.dn[628][8] * (nv13 - 0.0));
        let eq26_e1812_d_n9: f64 = (s.dn[628][9] * (nv13 - 0.0));
        let eq26_e1812_d_n10: f64 = (s.dn[628][10] * (nv13 - 0.0));
        let eq26_e1812_d_n11: f64 = (s.dn[628][11] * (nv13 - 0.0));
        let eq26_e1812_d_n12: f64 = (s.dn[628][12] * (nv13 - 0.0));
        let eq26_e1812_d_n13: f64 = ((s.dn[628][13] * (nv13 - 0.0)) + s.v[628]);
        let eq26_e1812_d_b0: f64 = (s.db[628][0] * (nv13 - 0.0));
        let eq26_e1812_d_b1: f64 = (s.db[628][1] * (nv13 - 0.0));
        let eq26_e1812_d_b2: f64 = (s.db[628][2] * (nv13 - 0.0));
        let eq26_e1812_d_b3: f64 = (s.db[628][3] * (nv13 - 0.0));
        let eq26_e1812_d_b4: f64 = (s.db[628][4] * (nv13 - 0.0));
        let eq26_e1812_d_b5: f64 = (s.db[628][5] * (nv13 - 0.0));
        let eq26_e1812_d_b6: f64 = (s.db[628][6] * (nv13 - 0.0));
        let eq26_e1812_d_b7: f64 = (s.db[628][7] * (nv13 - 0.0));
        let eq26_e1812_d_b8: f64 = (s.db[628][8] * (nv13 - 0.0));
        let eq26_e1812_d_b9: f64 = (s.db[628][9] * (nv13 - 0.0));
        let eq26_e1812_d_b10: f64 = (s.db[628][10] * (nv13 - 0.0));
        let eq26_e1812_d_b11: f64 = (s.db[628][11] * (nv13 - 0.0));
        (eq26_e1812, eq26_e1812_d_n0, eq26_e1812_d_n1, eq26_e1812_d_n2, eq26_e1812_d_n3, eq26_e1812_d_n4, eq26_e1812_d_n5, eq26_e1812_d_n6, eq26_e1812_d_n7, eq26_e1812_d_n8, eq26_e1812_d_n9, eq26_e1812_d_n10, eq26_e1812_d_n11, eq26_e1812_d_n12, eq26_e1812_d_n13, eq26_e1812_d_b0, eq26_e1812_d_b1, eq26_e1812_d_b2, eq26_e1812_d_b3, eq26_e1812_d_b4, eq26_e1812_d_b5, eq26_e1812_d_b6, eq26_e1812_d_b7, eq26_e1812_d_b8, eq26_e1812_d_b9, eq26_e1812_d_b10, eq26_e1812_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1814;
        let eq26_node_derivatives: [f64; 14] = [eq26_e1814_d_n0, eq26_e1814_d_n1, eq26_e1814_d_n2, eq26_e1814_d_n3, eq26_e1814_d_n4, eq26_e1814_d_n5, eq26_e1814_d_n6, eq26_e1814_d_n7, eq26_e1814_d_n8, eq26_e1814_d_n9, eq26_e1814_d_n10, eq26_e1814_d_n11, eq26_e1814_d_n12, eq26_e1814_d_n13];
        let eq26_branch_derivatives: [f64; 12] = [eq26_e1814_d_b0, eq26_e1814_d_b1, eq26_e1814_d_b2, eq26_e1814_d_b3, eq26_e1814_d_b4, eq26_e1814_d_b5, eq26_e1814_d_b6, eq26_e1814_d_b7, eq26_e1814_d_b8, eq26_e1814_d_b9, eq26_e1814_d_b10, eq26_e1814_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13, eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq27_e1825: f64 = (1.0 + s.v[211]);
        let eq27_e1827: f64 = (eq27_e1825 * s.v[622]);
        let eq27_e1827_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1825 * s.dn[622][0]));
        let eq27_e1827_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1825 * s.dn[622][1]));
        let eq27_e1827_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1825 * s.dn[622][2]));
        let eq27_e1827_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1825 * s.dn[622][3]));
        let eq27_e1827_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1825 * s.dn[622][4]));
        let eq27_e1827_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1825 * s.dn[622][5]));
        let eq27_e1827_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1825 * s.dn[622][6]));
        let eq27_e1827_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1825 * s.dn[622][7]));
        let eq27_e1827_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1825 * s.dn[622][8]));
        let eq27_e1827_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1825 * s.dn[622][9]));
        let eq27_e1827_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1825 * s.dn[622][10]));
        let eq27_e1827_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1825 * s.dn[622][11]));
        let eq27_e1827_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1825 * s.dn[622][12]));
        let eq27_e1827_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1825 * s.dn[622][13]));
        let eq27_e1827_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq27_e1825 * s.db[622][0]));
        let eq27_e1827_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq27_e1825 * s.db[622][1]));
        let eq27_e1827_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq27_e1825 * s.db[622][2]));
        let eq27_e1827_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq27_e1825 * s.db[622][3]));
        let eq27_e1827_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq27_e1825 * s.db[622][4]));
        let eq27_e1827_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq27_e1825 * s.db[622][5]));
        let eq27_e1827_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq27_e1825 * s.db[622][6]));
        let eq27_e1827_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq27_e1825 * s.db[622][7]));
        let eq27_e1827_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq27_e1825 * s.db[622][8]));
        let eq27_e1827_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq27_e1825 * s.db[622][9]));
        let eq27_e1827_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq27_e1825 * s.db[622][10]));
        let eq27_e1827_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq27_e1825 * s.db[622][11]));
        let eq27_e1829: f64 = (eq27_e1827 * s.v[199]);
        let eq27_e1829_d_n0: f64 = (eq27_e1827_d_n0 * s.v[199]);
        let eq27_e1829_d_n1: f64 = (eq27_e1827_d_n1 * s.v[199]);
        let eq27_e1829_d_n2: f64 = (eq27_e1827_d_n2 * s.v[199]);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * s.v[199]);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * s.v[199]);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * s.v[199]);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * s.v[199]);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * s.v[199]);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * s.v[199]);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * s.v[199]);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * s.v[199]);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * s.v[199]);
        let eq27_e1829_d_n12: f64 = (eq27_e1827_d_n12 * s.v[199]);
        let eq27_e1829_d_n13: f64 = (eq27_e1827_d_n13 * s.v[199]);
        let eq27_e1829_d_b0: f64 = (eq27_e1827_d_b0 * s.v[199]);
        let eq27_e1829_d_b1: f64 = (eq27_e1827_d_b1 * s.v[199]);
        let eq27_e1829_d_b2: f64 = (eq27_e1827_d_b2 * s.v[199]);
        let eq27_e1829_d_b3: f64 = (eq27_e1827_d_b3 * s.v[199]);
        let eq27_e1829_d_b4: f64 = (eq27_e1827_d_b4 * s.v[199]);
        let eq27_e1829_d_b5: f64 = (eq27_e1827_d_b5 * s.v[199]);
        let eq27_e1829_d_b6: f64 = (eq27_e1827_d_b6 * s.v[199]);
        let eq27_e1829_d_b7: f64 = (eq27_e1827_d_b7 * s.v[199]);
        let eq27_e1829_d_b8: f64 = (eq27_e1827_d_b8 * s.v[199]);
        let eq27_e1829_d_b9: f64 = (eq27_e1827_d_b9 * s.v[199]);
        let eq27_e1829_d_b10: f64 = (eq27_e1827_d_b10 * s.v[199]);
        let eq27_e1829_d_b11: f64 = (eq27_e1827_d_b11 * s.v[199]);
        let eq27_e1831: f64 = (eq27_e1829 * s.v[183]);
        let eq27_e1831_d_n0: f64 = (eq27_e1829_d_n0 * s.v[183]);
        let eq27_e1831_d_n1: f64 = (eq27_e1829_d_n1 * s.v[183]);
        let eq27_e1831_d_n2: f64 = (eq27_e1829_d_n2 * s.v[183]);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * s.v[183]);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * s.v[183]);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * s.v[183]);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * s.v[183]);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * s.v[183]);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * s.v[183]);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * s.v[183]);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * s.v[183]);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * s.v[183]);
        let eq27_e1831_d_n12: f64 = (eq27_e1829_d_n12 * s.v[183]);
        let eq27_e1831_d_n13: f64 = (eq27_e1829_d_n13 * s.v[183]);
        let eq27_e1831_d_b0: f64 = (eq27_e1829_d_b0 * s.v[183]);
        let eq27_e1831_d_b1: f64 = (eq27_e1829_d_b1 * s.v[183]);
        let eq27_e1831_d_b2: f64 = (eq27_e1829_d_b2 * s.v[183]);
        let eq27_e1831_d_b3: f64 = (eq27_e1829_d_b3 * s.v[183]);
        let eq27_e1831_d_b4: f64 = (eq27_e1829_d_b4 * s.v[183]);
        let eq27_e1831_d_b5: f64 = (eq27_e1829_d_b5 * s.v[183]);
        let eq27_e1831_d_b6: f64 = (eq27_e1829_d_b6 * s.v[183]);
        let eq27_e1831_d_b7: f64 = (eq27_e1829_d_b7 * s.v[183]);
        let eq27_e1831_d_b8: f64 = (eq27_e1829_d_b8 * s.v[183]);
        let eq27_e1831_d_b9: f64 = (eq27_e1829_d_b9 * s.v[183]);
        let eq27_e1831_d_b10: f64 = (eq27_e1829_d_b10 * s.v[183]);
        let eq27_e1831_d_b11: f64 = (eq27_e1829_d_b11 * s.v[183]);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n0: f64 = (eq27_e1831_d_n0 * p.p2);
        let eq27_e1833_d_n1: f64 = (eq27_e1831_d_n1 * p.p2);
        let eq27_e1833_d_n2: f64 = (eq27_e1831_d_n2 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1833_d_n12: f64 = (eq27_e1831_d_n12 * p.p2);
        let eq27_e1833_d_n13: f64 = (eq27_e1831_d_n13 * p.p2);
        let eq27_e1833_d_b0: f64 = (eq27_e1831_d_b0 * p.p2);
        let eq27_e1833_d_b1: f64 = (eq27_e1831_d_b1 * p.p2);
        let eq27_e1833_d_b2: f64 = (eq27_e1831_d_b2 * p.p2);
        let eq27_e1833_d_b3: f64 = (eq27_e1831_d_b3 * p.p2);
        let eq27_e1833_d_b4: f64 = (eq27_e1831_d_b4 * p.p2);
        let eq27_e1833_d_b5: f64 = (eq27_e1831_d_b5 * p.p2);
        let eq27_e1833_d_b6: f64 = (eq27_e1831_d_b6 * p.p2);
        let eq27_e1833_d_b7: f64 = (eq27_e1831_d_b7 * p.p2);
        let eq27_e1833_d_b8: f64 = (eq27_e1831_d_b8 * p.p2);
        let eq27_e1833_d_b9: f64 = (eq27_e1831_d_b9 * p.p2);
        let eq27_e1833_d_b10: f64 = (eq27_e1831_d_b10 * p.p2);
        let eq27_e1833_d_b11: f64 = (eq27_e1831_d_b11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * s.v[184]);
        let eq27_e1835_d_n0: f64 = (eq27_e1833_d_n0 * s.v[184]);
        let eq27_e1835_d_n1: f64 = (eq27_e1833_d_n1 * s.v[184]);
        let eq27_e1835_d_n2: f64 = (eq27_e1833_d_n2 * s.v[184]);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * s.v[184]);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * s.v[184]);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * s.v[184]);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * s.v[184]);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * s.v[184]);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * s.v[184]);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * s.v[184]);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * s.v[184]);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * s.v[184]);
        let eq27_e1835_d_n12: f64 = (eq27_e1833_d_n12 * s.v[184]);
        let eq27_e1835_d_n13: f64 = (eq27_e1833_d_n13 * s.v[184]);
        let eq27_e1835_d_b0: f64 = (eq27_e1833_d_b0 * s.v[184]);
        let eq27_e1835_d_b1: f64 = (eq27_e1833_d_b1 * s.v[184]);
        let eq27_e1835_d_b2: f64 = (eq27_e1833_d_b2 * s.v[184]);
        let eq27_e1835_d_b3: f64 = (eq27_e1833_d_b3 * s.v[184]);
        let eq27_e1835_d_b4: f64 = (eq27_e1833_d_b4 * s.v[184]);
        let eq27_e1835_d_b5: f64 = (eq27_e1833_d_b5 * s.v[184]);
        let eq27_e1835_d_b6: f64 = (eq27_e1833_d_b6 * s.v[184]);
        let eq27_e1835_d_b7: f64 = (eq27_e1833_d_b7 * s.v[184]);
        let eq27_e1835_d_b8: f64 = (eq27_e1833_d_b8 * s.v[184]);
        let eq27_e1835_d_b9: f64 = (eq27_e1833_d_b9 * s.v[184]);
        let eq27_e1835_d_b10: f64 = (eq27_e1833_d_b10 * s.v[184]);
        let eq27_e1835_d_b11: f64 = (eq27_e1833_d_b11 * s.v[184]);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n0: f64 = (eq27_e1835_d_n0 * (nv12 - 0.0));
        let eq27_e1837_d_n1: f64 = (eq27_e1835_d_n1 * (nv12 - 0.0));
        let eq27_e1837_d_n2: f64 = (eq27_e1835_d_n2 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1837_d_n12: f64 = ((eq27_e1835_d_n12 * (nv12 - 0.0)) + eq27_e1835);
        let eq27_e1837_d_n13: f64 = (eq27_e1835_d_n13 * (nv12 - 0.0));
        let eq27_e1837_d_b0: f64 = (eq27_e1835_d_b0 * (nv12 - 0.0));
        let eq27_e1837_d_b1: f64 = (eq27_e1835_d_b1 * (nv12 - 0.0));
        let eq27_e1837_d_b2: f64 = (eq27_e1835_d_b2 * (nv12 - 0.0));
        let eq27_e1837_d_b3: f64 = (eq27_e1835_d_b3 * (nv12 - 0.0));
        let eq27_e1837_d_b4: f64 = (eq27_e1835_d_b4 * (nv12 - 0.0));
        let eq27_e1837_d_b5: f64 = (eq27_e1835_d_b5 * (nv12 - 0.0));
        let eq27_e1837_d_b6: f64 = (eq27_e1835_d_b6 * (nv12 - 0.0));
        let eq27_e1837_d_b7: f64 = (eq27_e1835_d_b7 * (nv12 - 0.0));
        let eq27_e1837_d_b8: f64 = (eq27_e1835_d_b8 * (nv12 - 0.0));
        let eq27_e1837_d_b9: f64 = (eq27_e1835_d_b9 * (nv12 - 0.0));
        let eq27_e1837_d_b10: f64 = (eq27_e1835_d_b10 * (nv12 - 0.0));
        let eq27_e1837_d_b11: f64 = (eq27_e1835_d_b11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n0: f64 = (0.5 * eq27_e1837_d_n0);
        let eq27_e1838_d_n1: f64 = (0.5 * eq27_e1837_d_n1);
        let eq27_e1838_d_n2: f64 = (0.5 * eq27_e1837_d_n2);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1837_d_n12);
        let eq27_e1838_d_n13: f64 = (0.5 * eq27_e1837_d_n13);
        let eq27_e1838_d_b0: f64 = (0.5 * eq27_e1837_d_b0);
        let eq27_e1838_d_b1: f64 = (0.5 * eq27_e1837_d_b1);
        let eq27_e1838_d_b2: f64 = (0.5 * eq27_e1837_d_b2);
        let eq27_e1838_d_b3: f64 = (0.5 * eq27_e1837_d_b3);
        let eq27_e1838_d_b4: f64 = (0.5 * eq27_e1837_d_b4);
        let eq27_e1838_d_b5: f64 = (0.5 * eq27_e1837_d_b5);
        let eq27_e1838_d_b6: f64 = (0.5 * eq27_e1837_d_b6);
        let eq27_e1838_d_b7: f64 = (0.5 * eq27_e1837_d_b7);
        let eq27_e1838_d_b8: f64 = (0.5 * eq27_e1837_d_b8);
        let eq27_e1838_d_b9: f64 = (0.5 * eq27_e1837_d_b9);
        let eq27_e1838_d_b10: f64 = (0.5 * eq27_e1837_d_b10);
        let eq27_e1838_d_b11: f64 = (0.5 * eq27_e1837_d_b11);
        let eq27_e1839: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq27_e1838);
        let eq27_e1839_d_n0: f64 = (eq27_e1838_d_n0 * ddt_scale);
        let eq27_e1839_d_n1: f64 = (eq27_e1838_d_n1 * ddt_scale);
        let eq27_e1839_d_n2: f64 = (eq27_e1838_d_n2 * ddt_scale);
        let eq27_e1839_d_n3: f64 = (eq27_e1838_d_n3 * ddt_scale);
        let eq27_e1839_d_n4: f64 = (eq27_e1838_d_n4 * ddt_scale);
        let eq27_e1839_d_n5: f64 = (eq27_e1838_d_n5 * ddt_scale);
        let eq27_e1839_d_n6: f64 = (eq27_e1838_d_n6 * ddt_scale);
        let eq27_e1839_d_n7: f64 = (eq27_e1838_d_n7 * ddt_scale);
        let eq27_e1839_d_n8: f64 = (eq27_e1838_d_n8 * ddt_scale);
        let eq27_e1839_d_n9: f64 = (eq27_e1838_d_n9 * ddt_scale);
        let eq27_e1839_d_n10: f64 = (eq27_e1838_d_n10 * ddt_scale);
        let eq27_e1839_d_n11: f64 = (eq27_e1838_d_n11 * ddt_scale);
        let eq27_e1839_d_n12: f64 = (eq27_e1838_d_n12 * ddt_scale);
        let eq27_e1839_d_n13: f64 = (eq27_e1838_d_n13 * ddt_scale);
        let eq27_e1839_d_b0: f64 = (eq27_e1838_d_b0 * ddt_scale);
        let eq27_e1839_d_b1: f64 = (eq27_e1838_d_b1 * ddt_scale);
        let eq27_e1839_d_b2: f64 = (eq27_e1838_d_b2 * ddt_scale);
        let eq27_e1839_d_b3: f64 = (eq27_e1838_d_b3 * ddt_scale);
        let eq27_e1839_d_b4: f64 = (eq27_e1838_d_b4 * ddt_scale);
        let eq27_e1839_d_b5: f64 = (eq27_e1838_d_b5 * ddt_scale);
        let eq27_e1839_d_b6: f64 = (eq27_e1838_d_b6 * ddt_scale);
        let eq27_e1839_d_b7: f64 = (eq27_e1838_d_b7 * ddt_scale);
        let eq27_e1839_d_b8: f64 = (eq27_e1838_d_b8 * ddt_scale);
        let eq27_e1839_d_b9: f64 = (eq27_e1838_d_b9 * ddt_scale);
        let eq27_e1839_d_b10: f64 = (eq27_e1838_d_b10 * ddt_scale);
        let eq27_e1839_d_b11: f64 = (eq27_e1838_d_b11 * ddt_scale);
        (eq27_e1839, eq27_e1839_d_n0, eq27_e1839_d_n1, eq27_e1839_d_n2, eq27_e1839_d_n3, eq27_e1839_d_n4, eq27_e1839_d_n5, eq27_e1839_d_n6, eq27_e1839_d_n7, eq27_e1839_d_n8, eq27_e1839_d_n9, eq27_e1839_d_n10, eq27_e1839_d_n11, eq27_e1839_d_n12, eq27_e1839_d_n13, eq27_e1839_d_b0, eq27_e1839_d_b1, eq27_e1839_d_b2, eq27_e1839_d_b3, eq27_e1839_d_b4, eq27_e1839_d_b5, eq27_e1839_d_b6, eq27_e1839_d_b7, eq27_e1839_d_b8, eq27_e1839_d_b9, eq27_e1839_d_b10, eq27_e1839_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1841;
        let eq27_node_derivatives: [f64; 14] = [eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13];
        let eq27_branch_derivatives: [f64; 12] = [eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13, eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1852_d_n0: f64 = (-s.dn[211][0]);
        let eq28_e1852_d_n1: f64 = (-s.dn[211][1]);
        let eq28_e1852_d_n2: f64 = (-s.dn[211][2]);
        let eq28_e1852_d_n3: f64 = (-s.dn[211][3]);
        let eq28_e1852_d_n4: f64 = (-s.dn[211][4]);
        let eq28_e1852_d_n5: f64 = (-s.dn[211][5]);
        let eq28_e1852_d_n6: f64 = (-s.dn[211][6]);
        let eq28_e1852_d_n7: f64 = (-s.dn[211][7]);
        let eq28_e1852_d_n8: f64 = (-s.dn[211][8]);
        let eq28_e1852_d_n9: f64 = (-s.dn[211][9]);
        let eq28_e1852_d_n10: f64 = (-s.dn[211][10]);
        let eq28_e1852_d_n11: f64 = (-s.dn[211][11]);
        let eq28_e1852_d_n12: f64 = (-s.dn[211][12]);
        let eq28_e1852_d_n13: f64 = (-s.dn[211][13]);
        let eq28_e1852_d_b0: f64 = (-s.db[211][0]);
        let eq28_e1852_d_b1: f64 = (-s.db[211][1]);
        let eq28_e1852_d_b2: f64 = (-s.db[211][2]);
        let eq28_e1852_d_b3: f64 = (-s.db[211][3]);
        let eq28_e1852_d_b4: f64 = (-s.db[211][4]);
        let eq28_e1852_d_b5: f64 = (-s.db[211][5]);
        let eq28_e1852_d_b6: f64 = (-s.db[211][6]);
        let eq28_e1852_d_b7: f64 = (-s.db[211][7]);
        let eq28_e1852_d_b8: f64 = (-s.db[211][8]);
        let eq28_e1852_d_b9: f64 = (-s.db[211][9]);
        let eq28_e1852_d_b10: f64 = (-s.db[211][10]);
        let eq28_e1852_d_b11: f64 = (-s.db[211][11]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = ((eq28_e1852_d_n0 * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = ((eq28_e1852_d_n1 * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = ((eq28_e1852_d_n2 * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = ((eq28_e1852_d_n3 * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = ((eq28_e1852_d_n4 * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = ((eq28_e1852_d_n5 * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = ((eq28_e1852_d_n6 * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = ((eq28_e1852_d_n7 * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = ((eq28_e1852_d_n8 * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = ((eq28_e1852_d_n9 * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = ((eq28_e1852_d_n10 * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = ((eq28_e1852_d_n11 * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = ((eq28_e1852_d_n12 * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = ((eq28_e1852_d_n13 * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1854_d_b0: f64 = ((eq28_e1852_d_b0 * s.v[622]) + (eq28_e1852 * s.db[622][0]));
        let eq28_e1854_d_b1: f64 = ((eq28_e1852_d_b1 * s.v[622]) + (eq28_e1852 * s.db[622][1]));
        let eq28_e1854_d_b2: f64 = ((eq28_e1852_d_b2 * s.v[622]) + (eq28_e1852 * s.db[622][2]));
        let eq28_e1854_d_b3: f64 = ((eq28_e1852_d_b3 * s.v[622]) + (eq28_e1852 * s.db[622][3]));
        let eq28_e1854_d_b4: f64 = ((eq28_e1852_d_b4 * s.v[622]) + (eq28_e1852 * s.db[622][4]));
        let eq28_e1854_d_b5: f64 = ((eq28_e1852_d_b5 * s.v[622]) + (eq28_e1852 * s.db[622][5]));
        let eq28_e1854_d_b6: f64 = ((eq28_e1852_d_b6 * s.v[622]) + (eq28_e1852 * s.db[622][6]));
        let eq28_e1854_d_b7: f64 = ((eq28_e1852_d_b7 * s.v[622]) + (eq28_e1852 * s.db[622][7]));
        let eq28_e1854_d_b8: f64 = ((eq28_e1852_d_b8 * s.v[622]) + (eq28_e1852 * s.db[622][8]));
        let eq28_e1854_d_b9: f64 = ((eq28_e1852_d_b9 * s.v[622]) + (eq28_e1852 * s.db[622][9]));
        let eq28_e1854_d_b10: f64 = ((eq28_e1852_d_b10 * s.v[622]) + (eq28_e1852 * s.db[622][10]));
        let eq28_e1854_d_b11: f64 = ((eq28_e1852_d_b11 * s.v[622]) + (eq28_e1852 * s.db[622][11]));
        let eq28_e1856: f64 = (eq28_e1854 * s.v[199]);
        let eq28_e1856_d_n0: f64 = (eq28_e1854_d_n0 * s.v[199]);
        let eq28_e1856_d_n1: f64 = (eq28_e1854_d_n1 * s.v[199]);
        let eq28_e1856_d_n2: f64 = (eq28_e1854_d_n2 * s.v[199]);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * s.v[199]);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * s.v[199]);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * s.v[199]);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * s.v[199]);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * s.v[199]);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * s.v[199]);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * s.v[199]);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * s.v[199]);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * s.v[199]);
        let eq28_e1856_d_n12: f64 = (eq28_e1854_d_n12 * s.v[199]);
        let eq28_e1856_d_n13: f64 = (eq28_e1854_d_n13 * s.v[199]);
        let eq28_e1856_d_b0: f64 = (eq28_e1854_d_b0 * s.v[199]);
        let eq28_e1856_d_b1: f64 = (eq28_e1854_d_b1 * s.v[199]);
        let eq28_e1856_d_b2: f64 = (eq28_e1854_d_b2 * s.v[199]);
        let eq28_e1856_d_b3: f64 = (eq28_e1854_d_b3 * s.v[199]);
        let eq28_e1856_d_b4: f64 = (eq28_e1854_d_b4 * s.v[199]);
        let eq28_e1856_d_b5: f64 = (eq28_e1854_d_b5 * s.v[199]);
        let eq28_e1856_d_b6: f64 = (eq28_e1854_d_b6 * s.v[199]);
        let eq28_e1856_d_b7: f64 = (eq28_e1854_d_b7 * s.v[199]);
        let eq28_e1856_d_b8: f64 = (eq28_e1854_d_b8 * s.v[199]);
        let eq28_e1856_d_b9: f64 = (eq28_e1854_d_b9 * s.v[199]);
        let eq28_e1856_d_b10: f64 = (eq28_e1854_d_b10 * s.v[199]);
        let eq28_e1856_d_b11: f64 = (eq28_e1854_d_b11 * s.v[199]);
        let eq28_e1858: f64 = (eq28_e1856 * s.v[183]);
        let eq28_e1858_d_n0: f64 = (eq28_e1856_d_n0 * s.v[183]);
        let eq28_e1858_d_n1: f64 = (eq28_e1856_d_n1 * s.v[183]);
        let eq28_e1858_d_n2: f64 = (eq28_e1856_d_n2 * s.v[183]);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * s.v[183]);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * s.v[183]);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * s.v[183]);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * s.v[183]);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * s.v[183]);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * s.v[183]);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * s.v[183]);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * s.v[183]);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * s.v[183]);
        let eq28_e1858_d_n12: f64 = (eq28_e1856_d_n12 * s.v[183]);
        let eq28_e1858_d_n13: f64 = (eq28_e1856_d_n13 * s.v[183]);
        let eq28_e1858_d_b0: f64 = (eq28_e1856_d_b0 * s.v[183]);
        let eq28_e1858_d_b1: f64 = (eq28_e1856_d_b1 * s.v[183]);
        let eq28_e1858_d_b2: f64 = (eq28_e1856_d_b2 * s.v[183]);
        let eq28_e1858_d_b3: f64 = (eq28_e1856_d_b3 * s.v[183]);
        let eq28_e1858_d_b4: f64 = (eq28_e1856_d_b4 * s.v[183]);
        let eq28_e1858_d_b5: f64 = (eq28_e1856_d_b5 * s.v[183]);
        let eq28_e1858_d_b6: f64 = (eq28_e1856_d_b6 * s.v[183]);
        let eq28_e1858_d_b7: f64 = (eq28_e1856_d_b7 * s.v[183]);
        let eq28_e1858_d_b8: f64 = (eq28_e1856_d_b8 * s.v[183]);
        let eq28_e1858_d_b9: f64 = (eq28_e1856_d_b9 * s.v[183]);
        let eq28_e1858_d_b10: f64 = (eq28_e1856_d_b10 * s.v[183]);
        let eq28_e1858_d_b11: f64 = (eq28_e1856_d_b11 * s.v[183]);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n0: f64 = (eq28_e1858_d_n0 * p.p2);
        let eq28_e1860_d_n1: f64 = (eq28_e1858_d_n1 * p.p2);
        let eq28_e1860_d_n2: f64 = (eq28_e1858_d_n2 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1860_d_n12: f64 = (eq28_e1858_d_n12 * p.p2);
        let eq28_e1860_d_n13: f64 = (eq28_e1858_d_n13 * p.p2);
        let eq28_e1860_d_b0: f64 = (eq28_e1858_d_b0 * p.p2);
        let eq28_e1860_d_b1: f64 = (eq28_e1858_d_b1 * p.p2);
        let eq28_e1860_d_b2: f64 = (eq28_e1858_d_b2 * p.p2);
        let eq28_e1860_d_b3: f64 = (eq28_e1858_d_b3 * p.p2);
        let eq28_e1860_d_b4: f64 = (eq28_e1858_d_b4 * p.p2);
        let eq28_e1860_d_b5: f64 = (eq28_e1858_d_b5 * p.p2);
        let eq28_e1860_d_b6: f64 = (eq28_e1858_d_b6 * p.p2);
        let eq28_e1860_d_b7: f64 = (eq28_e1858_d_b7 * p.p2);
        let eq28_e1860_d_b8: f64 = (eq28_e1858_d_b8 * p.p2);
        let eq28_e1860_d_b9: f64 = (eq28_e1858_d_b9 * p.p2);
        let eq28_e1860_d_b10: f64 = (eq28_e1858_d_b10 * p.p2);
        let eq28_e1860_d_b11: f64 = (eq28_e1858_d_b11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * s.v[184]);
        let eq28_e1862_d_n0: f64 = (eq28_e1860_d_n0 * s.v[184]);
        let eq28_e1862_d_n1: f64 = (eq28_e1860_d_n1 * s.v[184]);
        let eq28_e1862_d_n2: f64 = (eq28_e1860_d_n2 * s.v[184]);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * s.v[184]);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * s.v[184]);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * s.v[184]);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * s.v[184]);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * s.v[184]);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * s.v[184]);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * s.v[184]);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * s.v[184]);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * s.v[184]);
        let eq28_e1862_d_n12: f64 = (eq28_e1860_d_n12 * s.v[184]);
        let eq28_e1862_d_n13: f64 = (eq28_e1860_d_n13 * s.v[184]);
        let eq28_e1862_d_b0: f64 = (eq28_e1860_d_b0 * s.v[184]);
        let eq28_e1862_d_b1: f64 = (eq28_e1860_d_b1 * s.v[184]);
        let eq28_e1862_d_b2: f64 = (eq28_e1860_d_b2 * s.v[184]);
        let eq28_e1862_d_b3: f64 = (eq28_e1860_d_b3 * s.v[184]);
        let eq28_e1862_d_b4: f64 = (eq28_e1860_d_b4 * s.v[184]);
        let eq28_e1862_d_b5: f64 = (eq28_e1860_d_b5 * s.v[184]);
        let eq28_e1862_d_b6: f64 = (eq28_e1860_d_b6 * s.v[184]);
        let eq28_e1862_d_b7: f64 = (eq28_e1860_d_b7 * s.v[184]);
        let eq28_e1862_d_b8: f64 = (eq28_e1860_d_b8 * s.v[184]);
        let eq28_e1862_d_b9: f64 = (eq28_e1860_d_b9 * s.v[184]);
        let eq28_e1862_d_b10: f64 = (eq28_e1860_d_b10 * s.v[184]);
        let eq28_e1862_d_b11: f64 = (eq28_e1860_d_b11 * s.v[184]);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n0: f64 = (eq28_e1862_d_n0 * (nv12 - 0.0));
        let eq28_e1864_d_n1: f64 = (eq28_e1862_d_n1 * (nv12 - 0.0));
        let eq28_e1864_d_n2: f64 = (eq28_e1862_d_n2 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1864_d_n12: f64 = ((eq28_e1862_d_n12 * (nv12 - 0.0)) + eq28_e1862);
        let eq28_e1864_d_n13: f64 = (eq28_e1862_d_n13 * (nv12 - 0.0));
        let eq28_e1864_d_b0: f64 = (eq28_e1862_d_b0 * (nv12 - 0.0));
        let eq28_e1864_d_b1: f64 = (eq28_e1862_d_b1 * (nv12 - 0.0));
        let eq28_e1864_d_b2: f64 = (eq28_e1862_d_b2 * (nv12 - 0.0));
        let eq28_e1864_d_b3: f64 = (eq28_e1862_d_b3 * (nv12 - 0.0));
        let eq28_e1864_d_b4: f64 = (eq28_e1862_d_b4 * (nv12 - 0.0));
        let eq28_e1864_d_b5: f64 = (eq28_e1862_d_b5 * (nv12 - 0.0));
        let eq28_e1864_d_b6: f64 = (eq28_e1862_d_b6 * (nv12 - 0.0));
        let eq28_e1864_d_b7: f64 = (eq28_e1862_d_b7 * (nv12 - 0.0));
        let eq28_e1864_d_b8: f64 = (eq28_e1862_d_b8 * (nv12 - 0.0));
        let eq28_e1864_d_b9: f64 = (eq28_e1862_d_b9 * (nv12 - 0.0));
        let eq28_e1864_d_b10: f64 = (eq28_e1862_d_b10 * (nv12 - 0.0));
        let eq28_e1864_d_b11: f64 = (eq28_e1862_d_b11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n0: f64 = (0.5 * eq28_e1864_d_n0);
        let eq28_e1865_d_n1: f64 = (0.5 * eq28_e1864_d_n1);
        let eq28_e1865_d_n2: f64 = (0.5 * eq28_e1864_d_n2);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1864_d_n12);
        let eq28_e1865_d_n13: f64 = (0.5 * eq28_e1864_d_n13);
        let eq28_e1865_d_b0: f64 = (0.5 * eq28_e1864_d_b0);
        let eq28_e1865_d_b1: f64 = (0.5 * eq28_e1864_d_b1);
        let eq28_e1865_d_b2: f64 = (0.5 * eq28_e1864_d_b2);
        let eq28_e1865_d_b3: f64 = (0.5 * eq28_e1864_d_b3);
        let eq28_e1865_d_b4: f64 = (0.5 * eq28_e1864_d_b4);
        let eq28_e1865_d_b5: f64 = (0.5 * eq28_e1864_d_b5);
        let eq28_e1865_d_b6: f64 = (0.5 * eq28_e1864_d_b6);
        let eq28_e1865_d_b7: f64 = (0.5 * eq28_e1864_d_b7);
        let eq28_e1865_d_b8: f64 = (0.5 * eq28_e1864_d_b8);
        let eq28_e1865_d_b9: f64 = (0.5 * eq28_e1864_d_b9);
        let eq28_e1865_d_b10: f64 = (0.5 * eq28_e1864_d_b10);
        let eq28_e1865_d_b11: f64 = (0.5 * eq28_e1864_d_b11);
        let eq28_e1866: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq28_e1865);
        let eq28_e1866_d_n0: f64 = (eq28_e1865_d_n0 * ddt_scale);
        let eq28_e1866_d_n1: f64 = (eq28_e1865_d_n1 * ddt_scale);
        let eq28_e1866_d_n2: f64 = (eq28_e1865_d_n2 * ddt_scale);
        let eq28_e1866_d_n3: f64 = (eq28_e1865_d_n3 * ddt_scale);
        let eq28_e1866_d_n4: f64 = (eq28_e1865_d_n4 * ddt_scale);
        let eq28_e1866_d_n5: f64 = (eq28_e1865_d_n5 * ddt_scale);
        let eq28_e1866_d_n6: f64 = (eq28_e1865_d_n6 * ddt_scale);
        let eq28_e1866_d_n7: f64 = (eq28_e1865_d_n7 * ddt_scale);
        let eq28_e1866_d_n8: f64 = (eq28_e1865_d_n8 * ddt_scale);
        let eq28_e1866_d_n9: f64 = (eq28_e1865_d_n9 * ddt_scale);
        let eq28_e1866_d_n10: f64 = (eq28_e1865_d_n10 * ddt_scale);
        let eq28_e1866_d_n11: f64 = (eq28_e1865_d_n11 * ddt_scale);
        let eq28_e1866_d_n12: f64 = (eq28_e1865_d_n12 * ddt_scale);
        let eq28_e1866_d_n13: f64 = (eq28_e1865_d_n13 * ddt_scale);
        let eq28_e1866_d_b0: f64 = (eq28_e1865_d_b0 * ddt_scale);
        let eq28_e1866_d_b1: f64 = (eq28_e1865_d_b1 * ddt_scale);
        let eq28_e1866_d_b2: f64 = (eq28_e1865_d_b2 * ddt_scale);
        let eq28_e1866_d_b3: f64 = (eq28_e1865_d_b3 * ddt_scale);
        let eq28_e1866_d_b4: f64 = (eq28_e1865_d_b4 * ddt_scale);
        let eq28_e1866_d_b5: f64 = (eq28_e1865_d_b5 * ddt_scale);
        let eq28_e1866_d_b6: f64 = (eq28_e1865_d_b6 * ddt_scale);
        let eq28_e1866_d_b7: f64 = (eq28_e1865_d_b7 * ddt_scale);
        let eq28_e1866_d_b8: f64 = (eq28_e1865_d_b8 * ddt_scale);
        let eq28_e1866_d_b9: f64 = (eq28_e1865_d_b9 * ddt_scale);
        let eq28_e1866_d_b10: f64 = (eq28_e1865_d_b10 * ddt_scale);
        let eq28_e1866_d_b11: f64 = (eq28_e1865_d_b11 * ddt_scale);
        (eq28_e1866, eq28_e1866_d_n0, eq28_e1866_d_n1, eq28_e1866_d_n2, eq28_e1866_d_n3, eq28_e1866_d_n4, eq28_e1866_d_n5, eq28_e1866_d_n6, eq28_e1866_d_n7, eq28_e1866_d_n8, eq28_e1866_d_n9, eq28_e1866_d_n10, eq28_e1866_d_n11, eq28_e1866_d_n12, eq28_e1866_d_n13, eq28_e1866_d_b0, eq28_e1866_d_b1, eq28_e1866_d_b2, eq28_e1866_d_b3, eq28_e1866_d_b4, eq28_e1866_d_b5, eq28_e1866_d_b6, eq28_e1866_d_b7, eq28_e1866_d_b8, eq28_e1866_d_b9, eq28_e1866_d_b10, eq28_e1866_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1868;
        let eq28_node_derivatives: [f64; 14] = [eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13];
        let eq28_branch_derivatives: [f64; 12] = [eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq35_e1938: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[1057]);
        let eq35_e1938_d_n0: f64 = (s.dn[1057][0] * ddt_scale);
        let eq35_e1938_d_n1: f64 = (s.dn[1057][1] * ddt_scale);
        let eq35_e1938_d_n2: f64 = (s.dn[1057][2] * ddt_scale);
        let eq35_e1938_d_n3: f64 = (s.dn[1057][3] * ddt_scale);
        let eq35_e1938_d_n4: f64 = (s.dn[1057][4] * ddt_scale);
        let eq35_e1938_d_n5: f64 = (s.dn[1057][5] * ddt_scale);
        let eq35_e1938_d_n6: f64 = (s.dn[1057][6] * ddt_scale);
        let eq35_e1938_d_n7: f64 = (s.dn[1057][7] * ddt_scale);
        let eq35_e1938_d_n8: f64 = (s.dn[1057][8] * ddt_scale);
        let eq35_e1938_d_n9: f64 = (s.dn[1057][9] * ddt_scale);
        let eq35_e1938_d_n10: f64 = (s.dn[1057][10] * ddt_scale);
        let eq35_e1938_d_n11: f64 = (s.dn[1057][11] * ddt_scale);
        let eq35_e1938_d_n12: f64 = (s.dn[1057][12] * ddt_scale);
        let eq35_e1938_d_n13: f64 = (s.dn[1057][13] * ddt_scale);
        let eq35_e1938_d_b0: f64 = (s.db[1057][0] * ddt_scale);
        let eq35_e1938_d_b1: f64 = (s.db[1057][1] * ddt_scale);
        let eq35_e1938_d_b2: f64 = (s.db[1057][2] * ddt_scale);
        let eq35_e1938_d_b3: f64 = (s.db[1057][3] * ddt_scale);
        let eq35_e1938_d_b4: f64 = (s.db[1057][4] * ddt_scale);
        let eq35_e1938_d_b5: f64 = (s.db[1057][5] * ddt_scale);
        let eq35_e1938_d_b6: f64 = (s.db[1057][6] * ddt_scale);
        let eq35_e1938_d_b7: f64 = (s.db[1057][7] * ddt_scale);
        let eq35_e1938_d_b8: f64 = (s.db[1057][8] * ddt_scale);
        let eq35_e1938_d_b9: f64 = (s.db[1057][9] * ddt_scale);
        let eq35_e1938_d_b10: f64 = (s.db[1057][10] * ddt_scale);
        let eq35_e1938_d_b11: f64 = (s.db[1057][11] * ddt_scale);
        let eq35_value: f64 = eq35_e1938;
        let eq35_node_derivatives: [f64; 14] = [eq35_e1938_d_n0, eq35_e1938_d_n1, eq35_e1938_d_n2, eq35_e1938_d_n3, eq35_e1938_d_n4, eq35_e1938_d_n5, eq35_e1938_d_n6, eq35_e1938_d_n7, eq35_e1938_d_n8, eq35_e1938_d_n9, eq35_e1938_d_n10, eq35_e1938_d_n11, eq35_e1938_d_n12, eq35_e1938_d_n13];
        let eq35_branch_derivatives: [f64; 12] = [eq35_e1938_d_b0, eq35_e1938_d_b1, eq35_e1938_d_b2, eq35_e1938_d_b3, eq35_e1938_d_b4, eq35_e1938_d_b5, eq35_e1938_d_b6, eq35_e1938_d_b7, eq35_e1938_d_b8, eq35_e1938_d_b9, eq35_e1938_d_b10, eq35_e1938_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(10),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e1940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[1058]);
        let eq36_e1940_d_n0: f64 = (s.dn[1058][0] * ddt_scale);
        let eq36_e1940_d_n1: f64 = (s.dn[1058][1] * ddt_scale);
        let eq36_e1940_d_n2: f64 = (s.dn[1058][2] * ddt_scale);
        let eq36_e1940_d_n3: f64 = (s.dn[1058][3] * ddt_scale);
        let eq36_e1940_d_n4: f64 = (s.dn[1058][4] * ddt_scale);
        let eq36_e1940_d_n5: f64 = (s.dn[1058][5] * ddt_scale);
        let eq36_e1940_d_n6: f64 = (s.dn[1058][6] * ddt_scale);
        let eq36_e1940_d_n7: f64 = (s.dn[1058][7] * ddt_scale);
        let eq36_e1940_d_n8: f64 = (s.dn[1058][8] * ddt_scale);
        let eq36_e1940_d_n9: f64 = (s.dn[1058][9] * ddt_scale);
        let eq36_e1940_d_n10: f64 = (s.dn[1058][10] * ddt_scale);
        let eq36_e1940_d_n11: f64 = (s.dn[1058][11] * ddt_scale);
        let eq36_e1940_d_n12: f64 = (s.dn[1058][12] * ddt_scale);
        let eq36_e1940_d_n13: f64 = (s.dn[1058][13] * ddt_scale);
        let eq36_e1940_d_b0: f64 = (s.db[1058][0] * ddt_scale);
        let eq36_e1940_d_b1: f64 = (s.db[1058][1] * ddt_scale);
        let eq36_e1940_d_b2: f64 = (s.db[1058][2] * ddt_scale);
        let eq36_e1940_d_b3: f64 = (s.db[1058][3] * ddt_scale);
        let eq36_e1940_d_b4: f64 = (s.db[1058][4] * ddt_scale);
        let eq36_e1940_d_b5: f64 = (s.db[1058][5] * ddt_scale);
        let eq36_e1940_d_b6: f64 = (s.db[1058][6] * ddt_scale);
        let eq36_e1940_d_b7: f64 = (s.db[1058][7] * ddt_scale);
        let eq36_e1940_d_b8: f64 = (s.db[1058][8] * ddt_scale);
        let eq36_e1940_d_b9: f64 = (s.db[1058][9] * ddt_scale);
        let eq36_e1940_d_b10: f64 = (s.db[1058][10] * ddt_scale);
        let eq36_e1940_d_b11: f64 = (s.db[1058][11] * ddt_scale);
        let eq36_value: f64 = eq36_e1940;
        let eq36_node_derivatives: [f64; 14] = [eq36_e1940_d_n0, eq36_e1940_d_n1, eq36_e1940_d_n2, eq36_e1940_d_n3, eq36_e1940_d_n4, eq36_e1940_d_n5, eq36_e1940_d_n6, eq36_e1940_d_n7, eq36_e1940_d_n8, eq36_e1940_d_n9, eq36_e1940_d_n10, eq36_e1940_d_n11, eq36_e1940_d_n12, eq36_e1940_d_n13];
        let eq36_branch_derivatives: [f64; 12] = [eq36_e1940_d_b0, eq36_e1940_d_b1, eq36_e1940_d_b2, eq36_e1940_d_b3, eq36_e1940_d_b4, eq36_e1940_d_b5, eq36_e1940_d_b6, eq36_e1940_d_b7, eq36_e1940_d_b8, eq36_e1940_d_b9, eq36_e1940_d_b10, eq36_e1940_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(11),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_e1942: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[1051]);
        let eq37_e1942_d_n0: f64 = (s.dn[1051][0] * ddt_scale);
        let eq37_e1942_d_n1: f64 = (s.dn[1051][1] * ddt_scale);
        let eq37_e1942_d_n2: f64 = (s.dn[1051][2] * ddt_scale);
        let eq37_e1942_d_n3: f64 = (s.dn[1051][3] * ddt_scale);
        let eq37_e1942_d_n4: f64 = (s.dn[1051][4] * ddt_scale);
        let eq37_e1942_d_n5: f64 = (s.dn[1051][5] * ddt_scale);
        let eq37_e1942_d_n6: f64 = (s.dn[1051][6] * ddt_scale);
        let eq37_e1942_d_n7: f64 = (s.dn[1051][7] * ddt_scale);
        let eq37_e1942_d_n8: f64 = (s.dn[1051][8] * ddt_scale);
        let eq37_e1942_d_n9: f64 = (s.dn[1051][9] * ddt_scale);
        let eq37_e1942_d_n10: f64 = (s.dn[1051][10] * ddt_scale);
        let eq37_e1942_d_n11: f64 = (s.dn[1051][11] * ddt_scale);
        let eq37_e1942_d_n12: f64 = (s.dn[1051][12] * ddt_scale);
        let eq37_e1942_d_n13: f64 = (s.dn[1051][13] * ddt_scale);
        let eq37_e1942_d_b0: f64 = (s.db[1051][0] * ddt_scale);
        let eq37_e1942_d_b1: f64 = (s.db[1051][1] * ddt_scale);
        let eq37_e1942_d_b2: f64 = (s.db[1051][2] * ddt_scale);
        let eq37_e1942_d_b3: f64 = (s.db[1051][3] * ddt_scale);
        let eq37_e1942_d_b4: f64 = (s.db[1051][4] * ddt_scale);
        let eq37_e1942_d_b5: f64 = (s.db[1051][5] * ddt_scale);
        let eq37_e1942_d_b6: f64 = (s.db[1051][6] * ddt_scale);
        let eq37_e1942_d_b7: f64 = (s.db[1051][7] * ddt_scale);
        let eq37_e1942_d_b8: f64 = (s.db[1051][8] * ddt_scale);
        let eq37_e1942_d_b9: f64 = (s.db[1051][9] * ddt_scale);
        let eq37_e1942_d_b10: f64 = (s.db[1051][10] * ddt_scale);
        let eq37_e1942_d_b11: f64 = (s.db[1051][11] * ddt_scale);
        let eq37_value: f64 = eq37_e1942;
        let eq37_node_derivatives: [f64; 14] = [eq37_e1942_d_n0, eq37_e1942_d_n1, eq37_e1942_d_n2, eq37_e1942_d_n3, eq37_e1942_d_n4, eq37_e1942_d_n5, eq37_e1942_d_n6, eq37_e1942_d_n7, eq37_e1942_d_n8, eq37_e1942_d_n9, eq37_e1942_d_n10, eq37_e1942_d_n11, eq37_e1942_d_n12, eq37_e1942_d_n13];
        let eq37_branch_derivatives: [f64; 12] = [eq37_e1942_d_b0, eq37_e1942_d_b1, eq37_e1942_d_b2, eq37_e1942_d_b3, eq37_e1942_d_b4, eq37_e1942_d_b5, eq37_e1942_d_b6, eq37_e1942_d_b7, eq37_e1942_d_b8, eq37_e1942_d_b9, eq37_e1942_d_b10, eq37_e1942_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e1944: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, s.v[1052]);
        let eq38_e1944_d_n0: f64 = (s.dn[1052][0] * ddt_scale);
        let eq38_e1944_d_n1: f64 = (s.dn[1052][1] * ddt_scale);
        let eq38_e1944_d_n2: f64 = (s.dn[1052][2] * ddt_scale);
        let eq38_e1944_d_n3: f64 = (s.dn[1052][3] * ddt_scale);
        let eq38_e1944_d_n4: f64 = (s.dn[1052][4] * ddt_scale);
        let eq38_e1944_d_n5: f64 = (s.dn[1052][5] * ddt_scale);
        let eq38_e1944_d_n6: f64 = (s.dn[1052][6] * ddt_scale);
        let eq38_e1944_d_n7: f64 = (s.dn[1052][7] * ddt_scale);
        let eq38_e1944_d_n8: f64 = (s.dn[1052][8] * ddt_scale);
        let eq38_e1944_d_n9: f64 = (s.dn[1052][9] * ddt_scale);
        let eq38_e1944_d_n10: f64 = (s.dn[1052][10] * ddt_scale);
        let eq38_e1944_d_n11: f64 = (s.dn[1052][11] * ddt_scale);
        let eq38_e1944_d_n12: f64 = (s.dn[1052][12] * ddt_scale);
        let eq38_e1944_d_n13: f64 = (s.dn[1052][13] * ddt_scale);
        let eq38_e1944_d_b0: f64 = (s.db[1052][0] * ddt_scale);
        let eq38_e1944_d_b1: f64 = (s.db[1052][1] * ddt_scale);
        let eq38_e1944_d_b2: f64 = (s.db[1052][2] * ddt_scale);
        let eq38_e1944_d_b3: f64 = (s.db[1052][3] * ddt_scale);
        let eq38_e1944_d_b4: f64 = (s.db[1052][4] * ddt_scale);
        let eq38_e1944_d_b5: f64 = (s.db[1052][5] * ddt_scale);
        let eq38_e1944_d_b6: f64 = (s.db[1052][6] * ddt_scale);
        let eq38_e1944_d_b7: f64 = (s.db[1052][7] * ddt_scale);
        let eq38_e1944_d_b8: f64 = (s.db[1052][8] * ddt_scale);
        let eq38_e1944_d_b9: f64 = (s.db[1052][9] * ddt_scale);
        let eq38_e1944_d_b10: f64 = (s.db[1052][10] * ddt_scale);
        let eq38_e1944_d_b11: f64 = (s.db[1052][11] * ddt_scale);
        let eq38_value: f64 = eq38_e1944;
        let eq38_node_derivatives: [f64; 14] = [eq38_e1944_d_n0, eq38_e1944_d_n1, eq38_e1944_d_n2, eq38_e1944_d_n3, eq38_e1944_d_n4, eq38_e1944_d_n5, eq38_e1944_d_n6, eq38_e1944_d_n7, eq38_e1944_d_n8, eq38_e1944_d_n9, eq38_e1944_d_n10, eq38_e1944_d_n11, eq38_e1944_d_n12, eq38_e1944_d_n13];
        let eq38_branch_derivatives: [f64; 12] = [eq38_e1944_d_b0, eq38_e1944_d_b1, eq38_e1944_d_b2, eq38_e1944_d_b3, eq38_e1944_d_b4, eq38_e1944_d_b5, eq38_e1944_d_b6, eq38_e1944_d_b7, eq38_e1944_d_b8, eq38_e1944_d_b9, eq38_e1944_d_b10, eq38_e1944_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e1946: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[1054]);
        let eq39_e1946_d_n0: f64 = (s.dn[1054][0] * ddt_scale);
        let eq39_e1946_d_n1: f64 = (s.dn[1054][1] * ddt_scale);
        let eq39_e1946_d_n2: f64 = (s.dn[1054][2] * ddt_scale);
        let eq39_e1946_d_n3: f64 = (s.dn[1054][3] * ddt_scale);
        let eq39_e1946_d_n4: f64 = (s.dn[1054][4] * ddt_scale);
        let eq39_e1946_d_n5: f64 = (s.dn[1054][5] * ddt_scale);
        let eq39_e1946_d_n6: f64 = (s.dn[1054][6] * ddt_scale);
        let eq39_e1946_d_n7: f64 = (s.dn[1054][7] * ddt_scale);
        let eq39_e1946_d_n8: f64 = (s.dn[1054][8] * ddt_scale);
        let eq39_e1946_d_n9: f64 = (s.dn[1054][9] * ddt_scale);
        let eq39_e1946_d_n10: f64 = (s.dn[1054][10] * ddt_scale);
        let eq39_e1946_d_n11: f64 = (s.dn[1054][11] * ddt_scale);
        let eq39_e1946_d_n12: f64 = (s.dn[1054][12] * ddt_scale);
        let eq39_e1946_d_n13: f64 = (s.dn[1054][13] * ddt_scale);
        let eq39_e1946_d_b0: f64 = (s.db[1054][0] * ddt_scale);
        let eq39_e1946_d_b1: f64 = (s.db[1054][1] * ddt_scale);
        let eq39_e1946_d_b2: f64 = (s.db[1054][2] * ddt_scale);
        let eq39_e1946_d_b3: f64 = (s.db[1054][3] * ddt_scale);
        let eq39_e1946_d_b4: f64 = (s.db[1054][4] * ddt_scale);
        let eq39_e1946_d_b5: f64 = (s.db[1054][5] * ddt_scale);
        let eq39_e1946_d_b6: f64 = (s.db[1054][6] * ddt_scale);
        let eq39_e1946_d_b7: f64 = (s.db[1054][7] * ddt_scale);
        let eq39_e1946_d_b8: f64 = (s.db[1054][8] * ddt_scale);
        let eq39_e1946_d_b9: f64 = (s.db[1054][9] * ddt_scale);
        let eq39_e1946_d_b10: f64 = (s.db[1054][10] * ddt_scale);
        let eq39_e1946_d_b11: f64 = (s.db[1054][11] * ddt_scale);
        let eq39_value: f64 = eq39_e1946;
        let eq39_node_derivatives: [f64; 14] = [eq39_e1946_d_n0, eq39_e1946_d_n1, eq39_e1946_d_n2, eq39_e1946_d_n3, eq39_e1946_d_n4, eq39_e1946_d_n5, eq39_e1946_d_n6, eq39_e1946_d_n7, eq39_e1946_d_n8, eq39_e1946_d_n9, eq39_e1946_d_n10, eq39_e1946_d_n11, eq39_e1946_d_n12, eq39_e1946_d_n13];
        let eq39_branch_derivatives: [f64; 12] = [eq39_e1946_d_b0, eq39_e1946_d_b1, eq39_e1946_d_b2, eq39_e1946_d_b3, eq39_e1946_d_b4, eq39_e1946_d_b5, eq39_e1946_d_b6, eq39_e1946_d_b7, eq39_e1946_d_b8, eq39_e1946_d_b9, eq39_e1946_d_b10, eq39_e1946_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1948: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[1055]);
        let eq40_e1948_d_n0: f64 = (s.dn[1055][0] * ddt_scale);
        let eq40_e1948_d_n1: f64 = (s.dn[1055][1] * ddt_scale);
        let eq40_e1948_d_n2: f64 = (s.dn[1055][2] * ddt_scale);
        let eq40_e1948_d_n3: f64 = (s.dn[1055][3] * ddt_scale);
        let eq40_e1948_d_n4: f64 = (s.dn[1055][4] * ddt_scale);
        let eq40_e1948_d_n5: f64 = (s.dn[1055][5] * ddt_scale);
        let eq40_e1948_d_n6: f64 = (s.dn[1055][6] * ddt_scale);
        let eq40_e1948_d_n7: f64 = (s.dn[1055][7] * ddt_scale);
        let eq40_e1948_d_n8: f64 = (s.dn[1055][8] * ddt_scale);
        let eq40_e1948_d_n9: f64 = (s.dn[1055][9] * ddt_scale);
        let eq40_e1948_d_n10: f64 = (s.dn[1055][10] * ddt_scale);
        let eq40_e1948_d_n11: f64 = (s.dn[1055][11] * ddt_scale);
        let eq40_e1948_d_n12: f64 = (s.dn[1055][12] * ddt_scale);
        let eq40_e1948_d_n13: f64 = (s.dn[1055][13] * ddt_scale);
        let eq40_e1948_d_b0: f64 = (s.db[1055][0] * ddt_scale);
        let eq40_e1948_d_b1: f64 = (s.db[1055][1] * ddt_scale);
        let eq40_e1948_d_b2: f64 = (s.db[1055][2] * ddt_scale);
        let eq40_e1948_d_b3: f64 = (s.db[1055][3] * ddt_scale);
        let eq40_e1948_d_b4: f64 = (s.db[1055][4] * ddt_scale);
        let eq40_e1948_d_b5: f64 = (s.db[1055][5] * ddt_scale);
        let eq40_e1948_d_b6: f64 = (s.db[1055][6] * ddt_scale);
        let eq40_e1948_d_b7: f64 = (s.db[1055][7] * ddt_scale);
        let eq40_e1948_d_b8: f64 = (s.db[1055][8] * ddt_scale);
        let eq40_e1948_d_b9: f64 = (s.db[1055][9] * ddt_scale);
        let eq40_e1948_d_b10: f64 = (s.db[1055][10] * ddt_scale);
        let eq40_e1948_d_b11: f64 = (s.db[1055][11] * ddt_scale);
        let eq40_value: f64 = eq40_e1948;
        let eq40_node_derivatives: [f64; 14] = [eq40_e1948_d_n0, eq40_e1948_d_n1, eq40_e1948_d_n2, eq40_e1948_d_n3, eq40_e1948_d_n4, eq40_e1948_d_n5, eq40_e1948_d_n6, eq40_e1948_d_n7, eq40_e1948_d_n8, eq40_e1948_d_n9, eq40_e1948_d_n10, eq40_e1948_d_n11, eq40_e1948_d_n12, eq40_e1948_d_n13];
        let eq40_branch_derivatives: [f64; 12] = [eq40_e1948_d_b0, eq40_e1948_d_b1, eq40_e1948_d_b2, eq40_e1948_d_b3, eq40_e1948_d_b4, eq40_e1948_d_b5, eq40_e1948_d_b6, eq40_e1948_d_b7, eq40_e1948_d_b8, eq40_e1948_d_b9, eq40_e1948_d_b10, eq40_e1948_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1950_d_n0: f64 = (-s.dn[379][0]);
        let eq41_e1950_d_n1: f64 = (-s.dn[379][1]);
        let eq41_e1950_d_n2: f64 = (-s.dn[379][2]);
        let eq41_e1950_d_n3: f64 = (-s.dn[379][3]);
        let eq41_e1950_d_n4: f64 = (-s.dn[379][4]);
        let eq41_e1950_d_n5: f64 = (-s.dn[379][5]);
        let eq41_e1950_d_n6: f64 = (-s.dn[379][6]);
        let eq41_e1950_d_n7: f64 = (-s.dn[379][7]);
        let eq41_e1950_d_n8: f64 = (-s.dn[379][8]);
        let eq41_e1950_d_n9: f64 = (-s.dn[379][9]);
        let eq41_e1950_d_n10: f64 = (-s.dn[379][10]);
        let eq41_e1950_d_n11: f64 = (-s.dn[379][11]);
        let eq41_e1950_d_n12: f64 = (-s.dn[379][12]);
        let eq41_e1950_d_n13: f64 = (-s.dn[379][13]);
        let eq41_e1950_d_b0: f64 = (-s.db[379][0]);
        let eq41_e1950_d_b1: f64 = (-s.db[379][1]);
        let eq41_e1950_d_b2: f64 = (-s.db[379][2]);
        let eq41_e1950_d_b3: f64 = (-s.db[379][3]);
        let eq41_e1950_d_b4: f64 = (-s.db[379][4]);
        let eq41_e1950_d_b5: f64 = (-s.db[379][5]);
        let eq41_e1950_d_b6: f64 = (-s.db[379][6]);
        let eq41_e1950_d_b7: f64 = (-s.db[379][7]);
        let eq41_e1950_d_b8: f64 = (-s.db[379][8]);
        let eq41_e1950_d_b9: f64 = (-s.db[379][9]);
        let eq41_e1950_d_b10: f64 = (-s.db[379][10]);
        let eq41_e1950_d_b11: f64 = (-s.db[379][11]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = ((eq41_e1950_d_n0 * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = ((eq41_e1950_d_n1 * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = ((eq41_e1950_d_n2 * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = ((eq41_e1950_d_n3 * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = ((eq41_e1950_d_n4 * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = ((eq41_e1950_d_n5 * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = ((eq41_e1950_d_n6 * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = ((eq41_e1950_d_n7 * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = ((eq41_e1950_d_n8 * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = ((eq41_e1950_d_n9 * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = ((eq41_e1950_d_n10 * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = ((eq41_e1950_d_n11 * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = ((eq41_e1950_d_n12 * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = ((eq41_e1950_d_n13 * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1952_d_b0: f64 = ((eq41_e1950_d_b0 * s.v[423]) + (eq41_e1950 * s.db[423][0]));
        let eq41_e1952_d_b1: f64 = ((eq41_e1950_d_b1 * s.v[423]) + (eq41_e1950 * s.db[423][1]));
        let eq41_e1952_d_b2: f64 = ((eq41_e1950_d_b2 * s.v[423]) + (eq41_e1950 * s.db[423][2]));
        let eq41_e1952_d_b3: f64 = ((eq41_e1950_d_b3 * s.v[423]) + (eq41_e1950 * s.db[423][3]));
        let eq41_e1952_d_b4: f64 = ((eq41_e1950_d_b4 * s.v[423]) + (eq41_e1950 * s.db[423][4]));
        let eq41_e1952_d_b5: f64 = ((eq41_e1950_d_b5 * s.v[423]) + (eq41_e1950 * s.db[423][5]));
        let eq41_e1952_d_b6: f64 = ((eq41_e1950_d_b6 * s.v[423]) + (eq41_e1950 * s.db[423][6]));
        let eq41_e1952_d_b7: f64 = ((eq41_e1950_d_b7 * s.v[423]) + (eq41_e1950 * s.db[423][7]));
        let eq41_e1952_d_b8: f64 = ((eq41_e1950_d_b8 * s.v[423]) + (eq41_e1950 * s.db[423][8]));
        let eq41_e1952_d_b9: f64 = ((eq41_e1950_d_b9 * s.v[423]) + (eq41_e1950 * s.db[423][9]));
        let eq41_e1952_d_b10: f64 = ((eq41_e1950_d_b10 * s.v[423]) + (eq41_e1950 * s.db[423][10]));
        let eq41_e1952_d_b11: f64 = ((eq41_e1950_d_b11 * s.v[423]) + (eq41_e1950 * s.db[423][11]));
        let eq41_e1953: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq41_e1952);
        let eq41_e1953_d_n0: f64 = (eq41_e1952_d_n0 * ddt_scale);
        let eq41_e1953_d_n1: f64 = (eq41_e1952_d_n1 * ddt_scale);
        let eq41_e1953_d_n2: f64 = (eq41_e1952_d_n2 * ddt_scale);
        let eq41_e1953_d_n3: f64 = (eq41_e1952_d_n3 * ddt_scale);
        let eq41_e1953_d_n4: f64 = (eq41_e1952_d_n4 * ddt_scale);
        let eq41_e1953_d_n5: f64 = (eq41_e1952_d_n5 * ddt_scale);
        let eq41_e1953_d_n6: f64 = (eq41_e1952_d_n6 * ddt_scale);
        let eq41_e1953_d_n7: f64 = (eq41_e1952_d_n7 * ddt_scale);
        let eq41_e1953_d_n8: f64 = (eq41_e1952_d_n8 * ddt_scale);
        let eq41_e1953_d_n9: f64 = (eq41_e1952_d_n9 * ddt_scale);
        let eq41_e1953_d_n10: f64 = (eq41_e1952_d_n10 * ddt_scale);
        let eq41_e1953_d_n11: f64 = (eq41_e1952_d_n11 * ddt_scale);
        let eq41_e1953_d_n12: f64 = (eq41_e1952_d_n12 * ddt_scale);
        let eq41_e1953_d_n13: f64 = (eq41_e1952_d_n13 * ddt_scale);
        let eq41_e1953_d_b0: f64 = (eq41_e1952_d_b0 * ddt_scale);
        let eq41_e1953_d_b1: f64 = (eq41_e1952_d_b1 * ddt_scale);
        let eq41_e1953_d_b2: f64 = (eq41_e1952_d_b2 * ddt_scale);
        let eq41_e1953_d_b3: f64 = (eq41_e1952_d_b3 * ddt_scale);
        let eq41_e1953_d_b4: f64 = (eq41_e1952_d_b4 * ddt_scale);
        let eq41_e1953_d_b5: f64 = (eq41_e1952_d_b5 * ddt_scale);
        let eq41_e1953_d_b6: f64 = (eq41_e1952_d_b6 * ddt_scale);
        let eq41_e1953_d_b7: f64 = (eq41_e1952_d_b7 * ddt_scale);
        let eq41_e1953_d_b8: f64 = (eq41_e1952_d_b8 * ddt_scale);
        let eq41_e1953_d_b9: f64 = (eq41_e1952_d_b9 * ddt_scale);
        let eq41_e1953_d_b10: f64 = (eq41_e1952_d_b10 * ddt_scale);
        let eq41_e1953_d_b11: f64 = (eq41_e1952_d_b11 * ddt_scale);
        let eq41_value: f64 = eq41_e1953;
        let eq41_node_derivatives: [f64; 14] = [eq41_e1953_d_n0, eq41_e1953_d_n1, eq41_e1953_d_n2, eq41_e1953_d_n3, eq41_e1953_d_n4, eq41_e1953_d_n5, eq41_e1953_d_n6, eq41_e1953_d_n7, eq41_e1953_d_n8, eq41_e1953_d_n9, eq41_e1953_d_n10, eq41_e1953_d_n11, eq41_e1953_d_n12, eq41_e1953_d_n13];
        let eq41_branch_derivatives: [f64; 12] = [eq41_e1953_d_b0, eq41_e1953_d_b1, eq41_e1953_d_b2, eq41_e1953_d_b3, eq41_e1953_d_b4, eq41_e1953_d_b5, eq41_e1953_d_b6, eq41_e1953_d_b7, eq41_e1953_d_b8, eq41_e1953_d_b9, eq41_e1953_d_b10, eq41_e1953_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1955: f64 = (-s.v[379]);
        let eq42_e1955_d_n0: f64 = (-s.dn[379][0]);
        let eq42_e1955_d_n1: f64 = (-s.dn[379][1]);
        let eq42_e1955_d_n2: f64 = (-s.dn[379][2]);
        let eq42_e1955_d_n3: f64 = (-s.dn[379][3]);
        let eq42_e1955_d_n4: f64 = (-s.dn[379][4]);
        let eq42_e1955_d_n5: f64 = (-s.dn[379][5]);
        let eq42_e1955_d_n6: f64 = (-s.dn[379][6]);
        let eq42_e1955_d_n7: f64 = (-s.dn[379][7]);
        let eq42_e1955_d_n8: f64 = (-s.dn[379][8]);
        let eq42_e1955_d_n9: f64 = (-s.dn[379][9]);
        let eq42_e1955_d_n10: f64 = (-s.dn[379][10]);
        let eq42_e1955_d_n11: f64 = (-s.dn[379][11]);
        let eq42_e1955_d_n12: f64 = (-s.dn[379][12]);
        let eq42_e1955_d_n13: f64 = (-s.dn[379][13]);
        let eq42_e1955_d_b0: f64 = (-s.db[379][0]);
        let eq42_e1955_d_b1: f64 = (-s.db[379][1]);
        let eq42_e1955_d_b2: f64 = (-s.db[379][2]);
        let eq42_e1955_d_b3: f64 = (-s.db[379][3]);
        let eq42_e1955_d_b4: f64 = (-s.db[379][4]);
        let eq42_e1955_d_b5: f64 = (-s.db[379][5]);
        let eq42_e1955_d_b6: f64 = (-s.db[379][6]);
        let eq42_e1955_d_b7: f64 = (-s.db[379][7]);
        let eq42_e1955_d_b8: f64 = (-s.db[379][8]);
        let eq42_e1955_d_b9: f64 = (-s.db[379][9]);
        let eq42_e1955_d_b10: f64 = (-s.db[379][10]);
        let eq42_e1955_d_b11: f64 = (-s.db[379][11]);
        let eq42_e1957: f64 = (eq42_e1955 * s.v[424]);
        let eq42_e1957_d_n0: f64 = ((eq42_e1955_d_n0 * s.v[424]) + (eq42_e1955 * s.dn[424][0]));
        let eq42_e1957_d_n1: f64 = ((eq42_e1955_d_n1 * s.v[424]) + (eq42_e1955 * s.dn[424][1]));
        let eq42_e1957_d_n2: f64 = ((eq42_e1955_d_n2 * s.v[424]) + (eq42_e1955 * s.dn[424][2]));
        let eq42_e1957_d_n3: f64 = ((eq42_e1955_d_n3 * s.v[424]) + (eq42_e1955 * s.dn[424][3]));
        let eq42_e1957_d_n4: f64 = ((eq42_e1955_d_n4 * s.v[424]) + (eq42_e1955 * s.dn[424][4]));
        let eq42_e1957_d_n5: f64 = ((eq42_e1955_d_n5 * s.v[424]) + (eq42_e1955 * s.dn[424][5]));
        let eq42_e1957_d_n6: f64 = ((eq42_e1955_d_n6 * s.v[424]) + (eq42_e1955 * s.dn[424][6]));
        let eq42_e1957_d_n7: f64 = ((eq42_e1955_d_n7 * s.v[424]) + (eq42_e1955 * s.dn[424][7]));
        let eq42_e1957_d_n8: f64 = ((eq42_e1955_d_n8 * s.v[424]) + (eq42_e1955 * s.dn[424][8]));
        let eq42_e1957_d_n9: f64 = ((eq42_e1955_d_n9 * s.v[424]) + (eq42_e1955 * s.dn[424][9]));
        let eq42_e1957_d_n10: f64 = ((eq42_e1955_d_n10 * s.v[424]) + (eq42_e1955 * s.dn[424][10]));
        let eq42_e1957_d_n11: f64 = ((eq42_e1955_d_n11 * s.v[424]) + (eq42_e1955 * s.dn[424][11]));
        let eq42_e1957_d_n12: f64 = ((eq42_e1955_d_n12 * s.v[424]) + (eq42_e1955 * s.dn[424][12]));
        let eq42_e1957_d_n13: f64 = ((eq42_e1955_d_n13 * s.v[424]) + (eq42_e1955 * s.dn[424][13]));
        let eq42_e1957_d_b0: f64 = ((eq42_e1955_d_b0 * s.v[424]) + (eq42_e1955 * s.db[424][0]));
        let eq42_e1957_d_b1: f64 = ((eq42_e1955_d_b1 * s.v[424]) + (eq42_e1955 * s.db[424][1]));
        let eq42_e1957_d_b2: f64 = ((eq42_e1955_d_b2 * s.v[424]) + (eq42_e1955 * s.db[424][2]));
        let eq42_e1957_d_b3: f64 = ((eq42_e1955_d_b3 * s.v[424]) + (eq42_e1955 * s.db[424][3]));
        let eq42_e1957_d_b4: f64 = ((eq42_e1955_d_b4 * s.v[424]) + (eq42_e1955 * s.db[424][4]));
        let eq42_e1957_d_b5: f64 = ((eq42_e1955_d_b5 * s.v[424]) + (eq42_e1955 * s.db[424][5]));
        let eq42_e1957_d_b6: f64 = ((eq42_e1955_d_b6 * s.v[424]) + (eq42_e1955 * s.db[424][6]));
        let eq42_e1957_d_b7: f64 = ((eq42_e1955_d_b7 * s.v[424]) + (eq42_e1955 * s.db[424][7]));
        let eq42_e1957_d_b8: f64 = ((eq42_e1955_d_b8 * s.v[424]) + (eq42_e1955 * s.db[424][8]));
        let eq42_e1957_d_b9: f64 = ((eq42_e1955_d_b9 * s.v[424]) + (eq42_e1955 * s.db[424][9]));
        let eq42_e1957_d_b10: f64 = ((eq42_e1955_d_b10 * s.v[424]) + (eq42_e1955 * s.db[424][10]));
        let eq42_e1957_d_b11: f64 = ((eq42_e1955_d_b11 * s.v[424]) + (eq42_e1955 * s.db[424][11]));
        let eq42_e1958: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq42_e1957);
        let eq42_e1958_d_n0: f64 = (eq42_e1957_d_n0 * ddt_scale);
        let eq42_e1958_d_n1: f64 = (eq42_e1957_d_n1 * ddt_scale);
        let eq42_e1958_d_n2: f64 = (eq42_e1957_d_n2 * ddt_scale);
        let eq42_e1958_d_n3: f64 = (eq42_e1957_d_n3 * ddt_scale);
        let eq42_e1958_d_n4: f64 = (eq42_e1957_d_n4 * ddt_scale);
        let eq42_e1958_d_n5: f64 = (eq42_e1957_d_n5 * ddt_scale);
        let eq42_e1958_d_n6: f64 = (eq42_e1957_d_n6 * ddt_scale);
        let eq42_e1958_d_n7: f64 = (eq42_e1957_d_n7 * ddt_scale);
        let eq42_e1958_d_n8: f64 = (eq42_e1957_d_n8 * ddt_scale);
        let eq42_e1958_d_n9: f64 = (eq42_e1957_d_n9 * ddt_scale);
        let eq42_e1958_d_n10: f64 = (eq42_e1957_d_n10 * ddt_scale);
        let eq42_e1958_d_n11: f64 = (eq42_e1957_d_n11 * ddt_scale);
        let eq42_e1958_d_n12: f64 = (eq42_e1957_d_n12 * ddt_scale);
        let eq42_e1958_d_n13: f64 = (eq42_e1957_d_n13 * ddt_scale);
        let eq42_e1958_d_b0: f64 = (eq42_e1957_d_b0 * ddt_scale);
        let eq42_e1958_d_b1: f64 = (eq42_e1957_d_b1 * ddt_scale);
        let eq42_e1958_d_b2: f64 = (eq42_e1957_d_b2 * ddt_scale);
        let eq42_e1958_d_b3: f64 = (eq42_e1957_d_b3 * ddt_scale);
        let eq42_e1958_d_b4: f64 = (eq42_e1957_d_b4 * ddt_scale);
        let eq42_e1958_d_b5: f64 = (eq42_e1957_d_b5 * ddt_scale);
        let eq42_e1958_d_b6: f64 = (eq42_e1957_d_b6 * ddt_scale);
        let eq42_e1958_d_b7: f64 = (eq42_e1957_d_b7 * ddt_scale);
        let eq42_e1958_d_b8: f64 = (eq42_e1957_d_b8 * ddt_scale);
        let eq42_e1958_d_b9: f64 = (eq42_e1957_d_b9 * ddt_scale);
        let eq42_e1958_d_b10: f64 = (eq42_e1957_d_b10 * ddt_scale);
        let eq42_e1958_d_b11: f64 = (eq42_e1957_d_b11 * ddt_scale);
        let eq42_value: f64 = eq42_e1958;
        let eq42_node_derivatives: [f64; 14] = [eq42_e1958_d_n0, eq42_e1958_d_n1, eq42_e1958_d_n2, eq42_e1958_d_n3, eq42_e1958_d_n4, eq42_e1958_d_n5, eq42_e1958_d_n6, eq42_e1958_d_n7, eq42_e1958_d_n8, eq42_e1958_d_n9, eq42_e1958_d_n10, eq42_e1958_d_n11, eq42_e1958_d_n12, eq42_e1958_d_n13];
        let eq42_branch_derivatives: [f64; 12] = [eq42_e1958_d_b0, eq42_e1958_d_b1, eq42_e1958_d_b2, eq42_e1958_d_b3, eq42_e1958_d_b4, eq42_e1958_d_b5, eq42_e1958_d_b6, eq42_e1958_d_b7, eq42_e1958_d_b8, eq42_e1958_d_b9, eq42_e1958_d_b10, eq42_e1958_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq44_e1966: f64 = (s.v[379] * s.v[210]);
        let eq44_e1966_d_n0: f64 = ((s.dn[379][0] * s.v[210]) + (s.v[379] * s.dn[210][0]));
        let eq44_e1966_d_n1: f64 = ((s.dn[379][1] * s.v[210]) + (s.v[379] * s.dn[210][1]));
        let eq44_e1966_d_n2: f64 = ((s.dn[379][2] * s.v[210]) + (s.v[379] * s.dn[210][2]));
        let eq44_e1966_d_n3: f64 = ((s.dn[379][3] * s.v[210]) + (s.v[379] * s.dn[210][3]));
        let eq44_e1966_d_n4: f64 = ((s.dn[379][4] * s.v[210]) + (s.v[379] * s.dn[210][4]));
        let eq44_e1966_d_n5: f64 = ((s.dn[379][5] * s.v[210]) + (s.v[379] * s.dn[210][5]));
        let eq44_e1966_d_n6: f64 = ((s.dn[379][6] * s.v[210]) + (s.v[379] * s.dn[210][6]));
        let eq44_e1966_d_n7: f64 = ((s.dn[379][7] * s.v[210]) + (s.v[379] * s.dn[210][7]));
        let eq44_e1966_d_n8: f64 = ((s.dn[379][8] * s.v[210]) + (s.v[379] * s.dn[210][8]));
        let eq44_e1966_d_n9: f64 = ((s.dn[379][9] * s.v[210]) + (s.v[379] * s.dn[210][9]));
        let eq44_e1966_d_n10: f64 = ((s.dn[379][10] * s.v[210]) + (s.v[379] * s.dn[210][10]));
        let eq44_e1966_d_n11: f64 = ((s.dn[379][11] * s.v[210]) + (s.v[379] * s.dn[210][11]));
        let eq44_e1966_d_n12: f64 = ((s.dn[379][12] * s.v[210]) + (s.v[379] * s.dn[210][12]));
        let eq44_e1966_d_n13: f64 = ((s.dn[379][13] * s.v[210]) + (s.v[379] * s.dn[210][13]));
        let eq44_e1966_d_b0: f64 = ((s.db[379][0] * s.v[210]) + (s.v[379] * s.db[210][0]));
        let eq44_e1966_d_b1: f64 = ((s.db[379][1] * s.v[210]) + (s.v[379] * s.db[210][1]));
        let eq44_e1966_d_b2: f64 = ((s.db[379][2] * s.v[210]) + (s.v[379] * s.db[210][2]));
        let eq44_e1966_d_b3: f64 = ((s.db[379][3] * s.v[210]) + (s.v[379] * s.db[210][3]));
        let eq44_e1966_d_b4: f64 = ((s.db[379][4] * s.v[210]) + (s.v[379] * s.db[210][4]));
        let eq44_e1966_d_b5: f64 = ((s.db[379][5] * s.v[210]) + (s.v[379] * s.db[210][5]));
        let eq44_e1966_d_b6: f64 = ((s.db[379][6] * s.v[210]) + (s.v[379] * s.db[210][6]));
        let eq44_e1966_d_b7: f64 = ((s.db[379][7] * s.v[210]) + (s.v[379] * s.db[210][7]));
        let eq44_e1966_d_b8: f64 = ((s.db[379][8] * s.v[210]) + (s.v[379] * s.db[210][8]));
        let eq44_e1966_d_b9: f64 = ((s.db[379][9] * s.v[210]) + (s.v[379] * s.db[210][9]));
        let eq44_e1966_d_b10: f64 = ((s.db[379][10] * s.v[210]) + (s.v[379] * s.db[210][10]));
        let eq44_e1966_d_b11: f64 = ((s.db[379][11] * s.v[210]) + (s.v[379] * s.db[210][11]));
        let eq44_value: f64 = eq44_e1966;
        let eq44_node_derivatives: [f64; 14] = [eq44_e1966_d_n0, eq44_e1966_d_n1, eq44_e1966_d_n2, eq44_e1966_d_n3, eq44_e1966_d_n4, eq44_e1966_d_n5, eq44_e1966_d_n6, eq44_e1966_d_n7, eq44_e1966_d_n8, eq44_e1966_d_n9, eq44_e1966_d_n10, eq44_e1966_d_n11, eq44_e1966_d_n12, eq44_e1966_d_n13];
        let eq44_branch_derivatives: [f64; 12] = [eq44_e1966_d_b0, eq44_e1966_d_b1, eq44_e1966_d_b2, eq44_e1966_d_b3, eq44_e1966_d_b4, eq44_e1966_d_b5, eq44_e1966_d_b6, eq44_e1966_d_b7, eq44_e1966_d_b8, eq44_e1966_d_b9, eq44_e1966_d_b10, eq44_e1966_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(11),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq45_e1969: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, s.v[1039]);
        let eq45_e1969_d_n0: f64 = (s.dn[1039][0] * ddt_scale);
        let eq45_e1969_d_n1: f64 = (s.dn[1039][1] * ddt_scale);
        let eq45_e1969_d_n2: f64 = (s.dn[1039][2] * ddt_scale);
        let eq45_e1969_d_n3: f64 = (s.dn[1039][3] * ddt_scale);
        let eq45_e1969_d_n4: f64 = (s.dn[1039][4] * ddt_scale);
        let eq45_e1969_d_n5: f64 = (s.dn[1039][5] * ddt_scale);
        let eq45_e1969_d_n6: f64 = (s.dn[1039][6] * ddt_scale);
        let eq45_e1969_d_n7: f64 = (s.dn[1039][7] * ddt_scale);
        let eq45_e1969_d_n8: f64 = (s.dn[1039][8] * ddt_scale);
        let eq45_e1969_d_n9: f64 = (s.dn[1039][9] * ddt_scale);
        let eq45_e1969_d_n10: f64 = (s.dn[1039][10] * ddt_scale);
        let eq45_e1969_d_n11: f64 = (s.dn[1039][11] * ddt_scale);
        let eq45_e1969_d_n12: f64 = (s.dn[1039][12] * ddt_scale);
        let eq45_e1969_d_n13: f64 = (s.dn[1039][13] * ddt_scale);
        let eq45_e1969_d_b0: f64 = (s.db[1039][0] * ddt_scale);
        let eq45_e1969_d_b1: f64 = (s.db[1039][1] * ddt_scale);
        let eq45_e1969_d_b2: f64 = (s.db[1039][2] * ddt_scale);
        let eq45_e1969_d_b3: f64 = (s.db[1039][3] * ddt_scale);
        let eq45_e1969_d_b4: f64 = (s.db[1039][4] * ddt_scale);
        let eq45_e1969_d_b5: f64 = (s.db[1039][5] * ddt_scale);
        let eq45_e1969_d_b6: f64 = (s.db[1039][6] * ddt_scale);
        let eq45_e1969_d_b7: f64 = (s.db[1039][7] * ddt_scale);
        let eq45_e1969_d_b8: f64 = (s.db[1039][8] * ddt_scale);
        let eq45_e1969_d_b9: f64 = (s.db[1039][9] * ddt_scale);
        let eq45_e1969_d_b10: f64 = (s.db[1039][10] * ddt_scale);
        let eq45_e1969_d_b11: f64 = (s.db[1039][11] * ddt_scale);
        let eq45_e1970: f64 = (s.v[379] * eq45_e1969);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n0));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n1));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n2));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n3));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n4));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n5));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n6));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n7));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n8));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n9));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n10));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n11));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n12));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n13));
        let eq45_e1970_d_b0: f64 = ((s.db[379][0] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b0));
        let eq45_e1970_d_b1: f64 = ((s.db[379][1] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b1));
        let eq45_e1970_d_b2: f64 = ((s.db[379][2] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b2));
        let eq45_e1970_d_b3: f64 = ((s.db[379][3] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b3));
        let eq45_e1970_d_b4: f64 = ((s.db[379][4] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b4));
        let eq45_e1970_d_b5: f64 = ((s.db[379][5] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b5));
        let eq45_e1970_d_b6: f64 = ((s.db[379][6] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b6));
        let eq45_e1970_d_b7: f64 = ((s.db[379][7] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b7));
        let eq45_e1970_d_b8: f64 = ((s.db[379][8] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b8));
        let eq45_e1970_d_b9: f64 = ((s.db[379][9] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b9));
        let eq45_e1970_d_b10: f64 = ((s.db[379][10] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b10));
        let eq45_e1970_d_b11: f64 = ((s.db[379][11] * eq45_e1969) + (s.v[379] * eq45_e1969_d_b11));
        let eq45_value: f64 = eq45_e1970;
        let eq45_node_derivatives: [f64; 14] = [eq45_e1970_d_n0, eq45_e1970_d_n1, eq45_e1970_d_n2, eq45_e1970_d_n3, eq45_e1970_d_n4, eq45_e1970_d_n5, eq45_e1970_d_n6, eq45_e1970_d_n7, eq45_e1970_d_n8, eq45_e1970_d_n9, eq45_e1970_d_n10, eq45_e1970_d_n11, eq45_e1970_d_n12, eq45_e1970_d_n13];
        let eq45_branch_derivatives: [f64; 12] = [eq45_e1970_d_b0, eq45_e1970_d_b1, eq45_e1970_d_b2, eq45_e1970_d_b3, eq45_e1970_d_b4, eq45_e1970_d_b5, eq45_e1970_d_b6, eq45_e1970_d_b7, eq45_e1970_d_b8, eq45_e1970_d_b9, eq45_e1970_d_b10, eq45_e1970_d_b11];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[1047]);
        let eq46_e1972_d_n0: f64 = (s.dn[1047][0] * ddt_scale);
        let eq46_e1972_d_n1: f64 = (s.dn[1047][1] * ddt_scale);
        let eq46_e1972_d_n2: f64 = (s.dn[1047][2] * ddt_scale);
        let eq46_e1972_d_n3: f64 = (s.dn[1047][3] * ddt_scale);
        let eq46_e1972_d_n4: f64 = (s.dn[1047][4] * ddt_scale);
        let eq46_e1972_d_n5: f64 = (s.dn[1047][5] * ddt_scale);
        let eq46_e1972_d_n6: f64 = (s.dn[1047][6] * ddt_scale);
        let eq46_e1972_d_n7: f64 = (s.dn[1047][7] * ddt_scale);
        let eq46_e1972_d_n8: f64 = (s.dn[1047][8] * ddt_scale);
        let eq46_e1972_d_n9: f64 = (s.dn[1047][9] * ddt_scale);
        let eq46_e1972_d_n10: f64 = (s.dn[1047][10] * ddt_scale);
        let eq46_e1972_d_n11: f64 = (s.dn[1047][11] * ddt_scale);
        let eq46_e1972_d_n12: f64 = (s.dn[1047][12] * ddt_scale);
        let eq46_e1972_d_n13: f64 = (s.dn[1047][13] * ddt_scale);
        let eq46_e1972_d_b0: f64 = (s.db[1047][0] * ddt_scale);
        let eq46_e1972_d_b1: f64 = (s.db[1047][1] * ddt_scale);
        let eq46_e1972_d_b2: f64 = (s.db[1047][2] * ddt_scale);
        let eq46_e1972_d_b3: f64 = (s.db[1047][3] * ddt_scale);
        let eq46_e1972_d_b4: f64 = (s.db[1047][4] * ddt_scale);
        let eq46_e1972_d_b5: f64 = (s.db[1047][5] * ddt_scale);
        let eq46_e1972_d_b6: f64 = (s.db[1047][6] * ddt_scale);
        let eq46_e1972_d_b7: f64 = (s.db[1047][7] * ddt_scale);
        let eq46_e1972_d_b8: f64 = (s.db[1047][8] * ddt_scale);
        let eq46_e1972_d_b9: f64 = (s.db[1047][9] * ddt_scale);
        let eq46_e1972_d_b10: f64 = (s.db[1047][10] * ddt_scale);
        let eq46_e1972_d_b11: f64 = (s.db[1047][11] * ddt_scale);
        let eq46_value: f64 = eq46_e1972;
        let eq46_node_derivatives: [f64; 14] = [eq46_e1972_d_n0, eq46_e1972_d_n1, eq46_e1972_d_n2, eq46_e1972_d_n3, eq46_e1972_d_n4, eq46_e1972_d_n5, eq46_e1972_d_n6, eq46_e1972_d_n7, eq46_e1972_d_n8, eq46_e1972_d_n9, eq46_e1972_d_n10, eq46_e1972_d_n11, eq46_e1972_d_n12, eq46_e1972_d_n13];
        let eq46_branch_derivatives: [f64; 12] = [eq46_e1972_d_b0, eq46_e1972_d_b1, eq46_e1972_d_b2, eq46_e1972_d_b3, eq46_e1972_d_b4, eq46_e1972_d_b5, eq46_e1972_d_b6, eq46_e1972_d_b7, eq46_e1972_d_b8, eq46_e1972_d_b9, eq46_e1972_d_b10, eq46_e1972_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let eq47_e1974: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, s.v[1046]);
        let eq47_e1974_d_n0: f64 = (s.dn[1046][0] * ddt_scale);
        let eq47_e1974_d_n1: f64 = (s.dn[1046][1] * ddt_scale);
        let eq47_e1974_d_n2: f64 = (s.dn[1046][2] * ddt_scale);
        let eq47_e1974_d_n3: f64 = (s.dn[1046][3] * ddt_scale);
        let eq47_e1974_d_n4: f64 = (s.dn[1046][4] * ddt_scale);
        let eq47_e1974_d_n5: f64 = (s.dn[1046][5] * ddt_scale);
        let eq47_e1974_d_n6: f64 = (s.dn[1046][6] * ddt_scale);
        let eq47_e1974_d_n7: f64 = (s.dn[1046][7] * ddt_scale);
        let eq47_e1974_d_n8: f64 = (s.dn[1046][8] * ddt_scale);
        let eq47_e1974_d_n9: f64 = (s.dn[1046][9] * ddt_scale);
        let eq47_e1974_d_n10: f64 = (s.dn[1046][10] * ddt_scale);
        let eq47_e1974_d_n11: f64 = (s.dn[1046][11] * ddt_scale);
        let eq47_e1974_d_n12: f64 = (s.dn[1046][12] * ddt_scale);
        let eq47_e1974_d_n13: f64 = (s.dn[1046][13] * ddt_scale);
        let eq47_e1974_d_b0: f64 = (s.db[1046][0] * ddt_scale);
        let eq47_e1974_d_b1: f64 = (s.db[1046][1] * ddt_scale);
        let eq47_e1974_d_b2: f64 = (s.db[1046][2] * ddt_scale);
        let eq47_e1974_d_b3: f64 = (s.db[1046][3] * ddt_scale);
        let eq47_e1974_d_b4: f64 = (s.db[1046][4] * ddt_scale);
        let eq47_e1974_d_b5: f64 = (s.db[1046][5] * ddt_scale);
        let eq47_e1974_d_b6: f64 = (s.db[1046][6] * ddt_scale);
        let eq47_e1974_d_b7: f64 = (s.db[1046][7] * ddt_scale);
        let eq47_e1974_d_b8: f64 = (s.db[1046][8] * ddt_scale);
        let eq47_e1974_d_b9: f64 = (s.db[1046][9] * ddt_scale);
        let eq47_e1974_d_b10: f64 = (s.db[1046][10] * ddt_scale);
        let eq47_e1974_d_b11: f64 = (s.db[1046][11] * ddt_scale);
        let eq47_value: f64 = eq47_e1974;
        let eq47_node_derivatives: [f64; 14] = [eq47_e1974_d_n0, eq47_e1974_d_n1, eq47_e1974_d_n2, eq47_e1974_d_n3, eq47_e1974_d_n4, eq47_e1974_d_n5, eq47_e1974_d_n6, eq47_e1974_d_n7, eq47_e1974_d_n8, eq47_e1974_d_n9, eq47_e1974_d_n10, eq47_e1974_d_n11, eq47_e1974_d_n12, eq47_e1974_d_n13];
        let eq47_branch_derivatives: [f64; 12] = [eq47_e1974_d_b0, eq47_e1974_d_b1, eq47_e1974_d_b2, eq47_e1974_d_b3, eq47_e1974_d_b4, eq47_e1974_d_b5, eq47_e1974_d_b6, eq47_e1974_d_b7, eq47_e1974_d_b8, eq47_e1974_d_b9, eq47_e1974_d_b10, eq47_e1974_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let eq48_e1977: f64 = (s.v[379] * s.v[211]);
        let eq48_e1977_d_n0: f64 = ((s.dn[379][0] * s.v[211]) + (s.v[379] * s.dn[211][0]));
        let eq48_e1977_d_n1: f64 = ((s.dn[379][1] * s.v[211]) + (s.v[379] * s.dn[211][1]));
        let eq48_e1977_d_n2: f64 = ((s.dn[379][2] * s.v[211]) + (s.v[379] * s.dn[211][2]));
        let eq48_e1977_d_n3: f64 = ((s.dn[379][3] * s.v[211]) + (s.v[379] * s.dn[211][3]));
        let eq48_e1977_d_n4: f64 = ((s.dn[379][4] * s.v[211]) + (s.v[379] * s.dn[211][4]));
        let eq48_e1977_d_n5: f64 = ((s.dn[379][5] * s.v[211]) + (s.v[379] * s.dn[211][5]));
        let eq48_e1977_d_n6: f64 = ((s.dn[379][6] * s.v[211]) + (s.v[379] * s.dn[211][6]));
        let eq48_e1977_d_n7: f64 = ((s.dn[379][7] * s.v[211]) + (s.v[379] * s.dn[211][7]));
        let eq48_e1977_d_n8: f64 = ((s.dn[379][8] * s.v[211]) + (s.v[379] * s.dn[211][8]));
        let eq48_e1977_d_n9: f64 = ((s.dn[379][9] * s.v[211]) + (s.v[379] * s.dn[211][9]));
        let eq48_e1977_d_n10: f64 = ((s.dn[379][10] * s.v[211]) + (s.v[379] * s.dn[211][10]));
        let eq48_e1977_d_n11: f64 = ((s.dn[379][11] * s.v[211]) + (s.v[379] * s.dn[211][11]));
        let eq48_e1977_d_n12: f64 = ((s.dn[379][12] * s.v[211]) + (s.v[379] * s.dn[211][12]));
        let eq48_e1977_d_n13: f64 = ((s.dn[379][13] * s.v[211]) + (s.v[379] * s.dn[211][13]));
        let eq48_e1977_d_b0: f64 = ((s.db[379][0] * s.v[211]) + (s.v[379] * s.db[211][0]));
        let eq48_e1977_d_b1: f64 = ((s.db[379][1] * s.v[211]) + (s.v[379] * s.db[211][1]));
        let eq48_e1977_d_b2: f64 = ((s.db[379][2] * s.v[211]) + (s.v[379] * s.db[211][2]));
        let eq48_e1977_d_b3: f64 = ((s.db[379][3] * s.v[211]) + (s.v[379] * s.db[211][3]));
        let eq48_e1977_d_b4: f64 = ((s.db[379][4] * s.v[211]) + (s.v[379] * s.db[211][4]));
        let eq48_e1977_d_b5: f64 = ((s.db[379][5] * s.v[211]) + (s.v[379] * s.db[211][5]));
        let eq48_e1977_d_b6: f64 = ((s.db[379][6] * s.v[211]) + (s.v[379] * s.db[211][6]));
        let eq48_e1977_d_b7: f64 = ((s.db[379][7] * s.v[211]) + (s.v[379] * s.db[211][7]));
        let eq48_e1977_d_b8: f64 = ((s.db[379][8] * s.v[211]) + (s.v[379] * s.db[211][8]));
        let eq48_e1977_d_b9: f64 = ((s.db[379][9] * s.v[211]) + (s.v[379] * s.db[211][9]));
        let eq48_e1977_d_b10: f64 = ((s.db[379][10] * s.v[211]) + (s.v[379] * s.db[211][10]));
        let eq48_e1977_d_b11: f64 = ((s.db[379][11] * s.v[211]) + (s.v[379] * s.db[211][11]));
        let eq48_e1979: f64 = (eq48_e1977 * s.v[380]);
        let eq48_e1979_d_n0: f64 = ((eq48_e1977_d_n0 * s.v[380]) + (eq48_e1977 * s.dn[380][0]));
        let eq48_e1979_d_n1: f64 = ((eq48_e1977_d_n1 * s.v[380]) + (eq48_e1977 * s.dn[380][1]));
        let eq48_e1979_d_n2: f64 = ((eq48_e1977_d_n2 * s.v[380]) + (eq48_e1977 * s.dn[380][2]));
        let eq48_e1979_d_n3: f64 = ((eq48_e1977_d_n3 * s.v[380]) + (eq48_e1977 * s.dn[380][3]));
        let eq48_e1979_d_n4: f64 = ((eq48_e1977_d_n4 * s.v[380]) + (eq48_e1977 * s.dn[380][4]));
        let eq48_e1979_d_n5: f64 = ((eq48_e1977_d_n5 * s.v[380]) + (eq48_e1977 * s.dn[380][5]));
        let eq48_e1979_d_n6: f64 = ((eq48_e1977_d_n6 * s.v[380]) + (eq48_e1977 * s.dn[380][6]));
        let eq48_e1979_d_n7: f64 = ((eq48_e1977_d_n7 * s.v[380]) + (eq48_e1977 * s.dn[380][7]));
        let eq48_e1979_d_n8: f64 = ((eq48_e1977_d_n8 * s.v[380]) + (eq48_e1977 * s.dn[380][8]));
        let eq48_e1979_d_n9: f64 = ((eq48_e1977_d_n9 * s.v[380]) + (eq48_e1977 * s.dn[380][9]));
        let eq48_e1979_d_n10: f64 = ((eq48_e1977_d_n10 * s.v[380]) + (eq48_e1977 * s.dn[380][10]));
        let eq48_e1979_d_n11: f64 = ((eq48_e1977_d_n11 * s.v[380]) + (eq48_e1977 * s.dn[380][11]));
        let eq48_e1979_d_n12: f64 = ((eq48_e1977_d_n12 * s.v[380]) + (eq48_e1977 * s.dn[380][12]));
        let eq48_e1979_d_n13: f64 = ((eq48_e1977_d_n13 * s.v[380]) + (eq48_e1977 * s.dn[380][13]));
        let eq48_e1979_d_b0: f64 = ((eq48_e1977_d_b0 * s.v[380]) + (eq48_e1977 * s.db[380][0]));
        let eq48_e1979_d_b1: f64 = ((eq48_e1977_d_b1 * s.v[380]) + (eq48_e1977 * s.db[380][1]));
        let eq48_e1979_d_b2: f64 = ((eq48_e1977_d_b2 * s.v[380]) + (eq48_e1977 * s.db[380][2]));
        let eq48_e1979_d_b3: f64 = ((eq48_e1977_d_b3 * s.v[380]) + (eq48_e1977 * s.db[380][3]));
        let eq48_e1979_d_b4: f64 = ((eq48_e1977_d_b4 * s.v[380]) + (eq48_e1977 * s.db[380][4]));
        let eq48_e1979_d_b5: f64 = ((eq48_e1977_d_b5 * s.v[380]) + (eq48_e1977 * s.db[380][5]));
        let eq48_e1979_d_b6: f64 = ((eq48_e1977_d_b6 * s.v[380]) + (eq48_e1977 * s.db[380][6]));
        let eq48_e1979_d_b7: f64 = ((eq48_e1977_d_b7 * s.v[380]) + (eq48_e1977 * s.db[380][7]));
        let eq48_e1979_d_b8: f64 = ((eq48_e1977_d_b8 * s.v[380]) + (eq48_e1977 * s.db[380][8]));
        let eq48_e1979_d_b9: f64 = ((eq48_e1977_d_b9 * s.v[380]) + (eq48_e1977 * s.db[380][9]));
        let eq48_e1979_d_b10: f64 = ((eq48_e1977_d_b10 * s.v[380]) + (eq48_e1977 * s.db[380][10]));
        let eq48_e1979_d_b11: f64 = ((eq48_e1977_d_b11 * s.v[380]) + (eq48_e1977 * s.db[380][11]));
        let eq48_value: f64 = eq48_e1979;
        let eq48_node_derivatives: [f64; 14] = [eq48_e1979_d_n0, eq48_e1979_d_n1, eq48_e1979_d_n2, eq48_e1979_d_n3, eq48_e1979_d_n4, eq48_e1979_d_n5, eq48_e1979_d_n6, eq48_e1979_d_n7, eq48_e1979_d_n8, eq48_e1979_d_n9, eq48_e1979_d_n10, eq48_e1979_d_n11, eq48_e1979_d_n12, eq48_e1979_d_n13];
        let eq48_branch_derivatives: [f64; 12] = [eq48_e1979_d_b0, eq48_e1979_d_b1, eq48_e1979_d_b2, eq48_e1979_d_b3, eq48_e1979_d_b4, eq48_e1979_d_b5, eq48_e1979_d_b6, eq48_e1979_d_b7, eq48_e1979_d_b8, eq48_e1979_d_b9, eq48_e1979_d_b10, eq48_e1979_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e1983, eq49_e1983_d_n0, eq49_e1983_d_n1, eq49_e1983_d_n2, eq49_e1983_d_n3, eq49_e1983_d_n4, eq49_e1983_d_n5, eq49_e1983_d_n6, eq49_e1983_d_n7, eq49_e1983_d_n8, eq49_e1983_d_n9, eq49_e1983_d_n10, eq49_e1983_d_n11, eq49_e1983_d_n12, eq49_e1983_d_n13, eq49_e1983_d_b0, eq49_e1983_d_b1, eq49_e1983_d_b2, eq49_e1983_d_b3, eq49_e1983_d_b4, eq49_e1983_d_b5, eq49_e1983_d_b6, eq49_e1983_d_b7, eq49_e1983_d_b8, eq49_e1983_d_b9, eq49_e1983_d_b10, eq49_e1983_d_b11,) = {
    if s.b[2009] {
        (s.v[1102], s.dn[1102][0], s.dn[1102][1], s.dn[1102][2], s.dn[1102][3], s.dn[1102][4], s.dn[1102][5], s.dn[1102][6], s.dn[1102][7], s.dn[1102][8], s.dn[1102][9], s.dn[1102][10], s.dn[1102][11], s.dn[1102][12], s.dn[1102][13], s.db[1102][0], s.db[1102][1], s.db[1102][2], s.db[1102][3], s.db[1102][4], s.db[1102][5], s.db[1102][6], s.db[1102][7], s.db[1102][8], s.db[1102][9], s.db[1102][10], s.db[1102][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1983;
        let eq49_node_derivatives: [f64; 14] = [eq49_e1983_d_n0, eq49_e1983_d_n1, eq49_e1983_d_n2, eq49_e1983_d_n3, eq49_e1983_d_n4, eq49_e1983_d_n5, eq49_e1983_d_n6, eq49_e1983_d_n7, eq49_e1983_d_n8, eq49_e1983_d_n9, eq49_e1983_d_n10, eq49_e1983_d_n11, eq49_e1983_d_n12, eq49_e1983_d_n13];
        let eq49_branch_derivatives: [f64; 12] = [eq49_e1983_d_b0, eq49_e1983_d_b1, eq49_e1983_d_b2, eq49_e1983_d_b3, eq49_e1983_d_b4, eq49_e1983_d_b5, eq49_e1983_d_b6, eq49_e1983_d_b7, eq49_e1983_d_b8, eq49_e1983_d_b9, eq49_e1983_d_b10, eq49_e1983_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(10),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1989, eq50_e1989_d_n0, eq50_e1989_d_n1, eq50_e1989_d_n2, eq50_e1989_d_n3, eq50_e1989_d_n4, eq50_e1989_d_n5, eq50_e1989_d_n6, eq50_e1989_d_n7, eq50_e1989_d_n8, eq50_e1989_d_n9, eq50_e1989_d_n10, eq50_e1989_d_n11, eq50_e1989_d_n12, eq50_e1989_d_n13, eq50_e1989_d_b0, eq50_e1989_d_b1, eq50_e1989_d_b2, eq50_e1989_d_b3, eq50_e1989_d_b4, eq50_e1989_d_b5, eq50_e1989_d_b6, eq50_e1989_d_b7, eq50_e1989_d_b8, eq50_e1989_d_b9, eq50_e1989_d_b10, eq50_e1989_d_b11,) = {
    if s.b[2010] {
        let eq50_e1987: f64 = (s.v[1098] + s.v[1100]);
        let eq50_e1987_d_n0: f64 = (s.dn[1098][0] + s.dn[1100][0]);
        let eq50_e1987_d_n1: f64 = (s.dn[1098][1] + s.dn[1100][1]);
        let eq50_e1987_d_n2: f64 = (s.dn[1098][2] + s.dn[1100][2]);
        let eq50_e1987_d_n3: f64 = (s.dn[1098][3] + s.dn[1100][3]);
        let eq50_e1987_d_n4: f64 = (s.dn[1098][4] + s.dn[1100][4]);
        let eq50_e1987_d_n5: f64 = (s.dn[1098][5] + s.dn[1100][5]);
        let eq50_e1987_d_n6: f64 = (s.dn[1098][6] + s.dn[1100][6]);
        let eq50_e1987_d_n7: f64 = (s.dn[1098][7] + s.dn[1100][7]);
        let eq50_e1987_d_n8: f64 = (s.dn[1098][8] + s.dn[1100][8]);
        let eq50_e1987_d_n9: f64 = (s.dn[1098][9] + s.dn[1100][9]);
        let eq50_e1987_d_n10: f64 = (s.dn[1098][10] + s.dn[1100][10]);
        let eq50_e1987_d_n11: f64 = (s.dn[1098][11] + s.dn[1100][11]);
        let eq50_e1987_d_n12: f64 = (s.dn[1098][12] + s.dn[1100][12]);
        let eq50_e1987_d_n13: f64 = (s.dn[1098][13] + s.dn[1100][13]);
        let eq50_e1987_d_b0: f64 = (s.db[1098][0] + s.db[1100][0]);
        let eq50_e1987_d_b1: f64 = (s.db[1098][1] + s.db[1100][1]);
        let eq50_e1987_d_b2: f64 = (s.db[1098][2] + s.db[1100][2]);
        let eq50_e1987_d_b3: f64 = (s.db[1098][3] + s.db[1100][3]);
        let eq50_e1987_d_b4: f64 = (s.db[1098][4] + s.db[1100][4]);
        let eq50_e1987_d_b5: f64 = (s.db[1098][5] + s.db[1100][5]);
        let eq50_e1987_d_b6: f64 = (s.db[1098][6] + s.db[1100][6]);
        let eq50_e1987_d_b7: f64 = (s.db[1098][7] + s.db[1100][7]);
        let eq50_e1987_d_b8: f64 = (s.db[1098][8] + s.db[1100][8]);
        let eq50_e1987_d_b9: f64 = (s.db[1098][9] + s.db[1100][9]);
        let eq50_e1987_d_b10: f64 = (s.db[1098][10] + s.db[1100][10]);
        let eq50_e1987_d_b11: f64 = (s.db[1098][11] + s.db[1100][11]);
        (eq50_e1987, eq50_e1987_d_n0, eq50_e1987_d_n1, eq50_e1987_d_n2, eq50_e1987_d_n3, eq50_e1987_d_n4, eq50_e1987_d_n5, eq50_e1987_d_n6, eq50_e1987_d_n7, eq50_e1987_d_n8, eq50_e1987_d_n9, eq50_e1987_d_n10, eq50_e1987_d_n11, eq50_e1987_d_n12, eq50_e1987_d_n13, eq50_e1987_d_b0, eq50_e1987_d_b1, eq50_e1987_d_b2, eq50_e1987_d_b3, eq50_e1987_d_b4, eq50_e1987_d_b5, eq50_e1987_d_b6, eq50_e1987_d_b7, eq50_e1987_d_b8, eq50_e1987_d_b9, eq50_e1987_d_b10, eq50_e1987_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1989;
        let eq50_node_derivatives: [f64; 14] = [eq50_e1989_d_n0, eq50_e1989_d_n1, eq50_e1989_d_n2, eq50_e1989_d_n3, eq50_e1989_d_n4, eq50_e1989_d_n5, eq50_e1989_d_n6, eq50_e1989_d_n7, eq50_e1989_d_n8, eq50_e1989_d_n9, eq50_e1989_d_n10, eq50_e1989_d_n11, eq50_e1989_d_n12, eq50_e1989_d_n13];
        let eq50_branch_derivatives: [f64; 12] = [eq50_e1989_d_b0, eq50_e1989_d_b1, eq50_e1989_d_b2, eq50_e1989_d_b3, eq50_e1989_d_b4, eq50_e1989_d_b5, eq50_e1989_d_b6, eq50_e1989_d_b7, eq50_e1989_d_b8, eq50_e1989_d_b9, eq50_e1989_d_b10, eq50_e1989_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1995, eq51_e1995_d_n0, eq51_e1995_d_n1, eq51_e1995_d_n2, eq51_e1995_d_n3, eq51_e1995_d_n4, eq51_e1995_d_n5, eq51_e1995_d_n6, eq51_e1995_d_n7, eq51_e1995_d_n8, eq51_e1995_d_n9, eq51_e1995_d_n10, eq51_e1995_d_n11, eq51_e1995_d_n12, eq51_e1995_d_n13, eq51_e1995_d_b0, eq51_e1995_d_b1, eq51_e1995_d_b2, eq51_e1995_d_b3, eq51_e1995_d_b4, eq51_e1995_d_b5, eq51_e1995_d_b6, eq51_e1995_d_b7, eq51_e1995_d_b8, eq51_e1995_d_b9, eq51_e1995_d_b10, eq51_e1995_d_b11,) = {
    if s.b[2010] {
        let eq51_e1993: f64 = (s.v[1099] + s.v[1101]);
        let eq51_e1993_d_n0: f64 = (s.dn[1099][0] + s.dn[1101][0]);
        let eq51_e1993_d_n1: f64 = (s.dn[1099][1] + s.dn[1101][1]);
        let eq51_e1993_d_n2: f64 = (s.dn[1099][2] + s.dn[1101][2]);
        let eq51_e1993_d_n3: f64 = (s.dn[1099][3] + s.dn[1101][3]);
        let eq51_e1993_d_n4: f64 = (s.dn[1099][4] + s.dn[1101][4]);
        let eq51_e1993_d_n5: f64 = (s.dn[1099][5] + s.dn[1101][5]);
        let eq51_e1993_d_n6: f64 = (s.dn[1099][6] + s.dn[1101][6]);
        let eq51_e1993_d_n7: f64 = (s.dn[1099][7] + s.dn[1101][7]);
        let eq51_e1993_d_n8: f64 = (s.dn[1099][8] + s.dn[1101][8]);
        let eq51_e1993_d_n9: f64 = (s.dn[1099][9] + s.dn[1101][9]);
        let eq51_e1993_d_n10: f64 = (s.dn[1099][10] + s.dn[1101][10]);
        let eq51_e1993_d_n11: f64 = (s.dn[1099][11] + s.dn[1101][11]);
        let eq51_e1993_d_n12: f64 = (s.dn[1099][12] + s.dn[1101][12]);
        let eq51_e1993_d_n13: f64 = (s.dn[1099][13] + s.dn[1101][13]);
        let eq51_e1993_d_b0: f64 = (s.db[1099][0] + s.db[1101][0]);
        let eq51_e1993_d_b1: f64 = (s.db[1099][1] + s.db[1101][1]);
        let eq51_e1993_d_b2: f64 = (s.db[1099][2] + s.db[1101][2]);
        let eq51_e1993_d_b3: f64 = (s.db[1099][3] + s.db[1101][3]);
        let eq51_e1993_d_b4: f64 = (s.db[1099][4] + s.db[1101][4]);
        let eq51_e1993_d_b5: f64 = (s.db[1099][5] + s.db[1101][5]);
        let eq51_e1993_d_b6: f64 = (s.db[1099][6] + s.db[1101][6]);
        let eq51_e1993_d_b7: f64 = (s.db[1099][7] + s.db[1101][7]);
        let eq51_e1993_d_b8: f64 = (s.db[1099][8] + s.db[1101][8]);
        let eq51_e1993_d_b9: f64 = (s.db[1099][9] + s.db[1101][9]);
        let eq51_e1993_d_b10: f64 = (s.db[1099][10] + s.db[1101][10]);
        let eq51_e1993_d_b11: f64 = (s.db[1099][11] + s.db[1101][11]);
        (eq51_e1993, eq51_e1993_d_n0, eq51_e1993_d_n1, eq51_e1993_d_n2, eq51_e1993_d_n3, eq51_e1993_d_n4, eq51_e1993_d_n5, eq51_e1993_d_n6, eq51_e1993_d_n7, eq51_e1993_d_n8, eq51_e1993_d_n9, eq51_e1993_d_n10, eq51_e1993_d_n11, eq51_e1993_d_n12, eq51_e1993_d_n13, eq51_e1993_d_b0, eq51_e1993_d_b1, eq51_e1993_d_b2, eq51_e1993_d_b3, eq51_e1993_d_b4, eq51_e1993_d_b5, eq51_e1993_d_b6, eq51_e1993_d_b7, eq51_e1993_d_b8, eq51_e1993_d_b9, eq51_e1993_d_b10, eq51_e1993_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1995;
        let eq51_node_derivatives: [f64; 14] = [eq51_e1995_d_n0, eq51_e1995_d_n1, eq51_e1995_d_n2, eq51_e1995_d_n3, eq51_e1995_d_n4, eq51_e1995_d_n5, eq51_e1995_d_n6, eq51_e1995_d_n7, eq51_e1995_d_n8, eq51_e1995_d_n9, eq51_e1995_d_n10, eq51_e1995_d_n11, eq51_e1995_d_n12, eq51_e1995_d_n13];
        let eq51_branch_derivatives: [f64; 12] = [eq51_e1995_d_b0, eq51_e1995_d_b1, eq51_e1995_d_b2, eq51_e1995_d_b3, eq51_e1995_d_b4, eq51_e1995_d_b5, eq51_e1995_d_b6, eq51_e1995_d_b7, eq51_e1995_d_b8, eq51_e1995_d_b9, eq51_e1995_d_b10, eq51_e1995_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2001, eq52_e2001_d_n0, eq52_e2001_d_n1, eq52_e2001_d_n2, eq52_e2001_d_n3, eq52_e2001_d_n4, eq52_e2001_d_n5, eq52_e2001_d_n6, eq52_e2001_d_n7, eq52_e2001_d_n8, eq52_e2001_d_n9, eq52_e2001_d_n10, eq52_e2001_d_n11, eq52_e2001_d_n12, eq52_e2001_d_n13, eq52_e2001_d_b0, eq52_e2001_d_b1, eq52_e2001_d_b2, eq52_e2001_d_b3, eq52_e2001_d_b4, eq52_e2001_d_b5, eq52_e2001_d_b6, eq52_e2001_d_b7, eq52_e2001_d_b8, eq52_e2001_d_b9, eq52_e2001_d_b10, eq52_e2001_d_b11,) = {
    if s.b[2011] {
        let eq52_e1999: f64 = (s.v[1095] + s.v[1096]);
        let eq52_e1999_d_n0: f64 = (s.dn[1095][0] + s.dn[1096][0]);
        let eq52_e1999_d_n1: f64 = (s.dn[1095][1] + s.dn[1096][1]);
        let eq52_e1999_d_n2: f64 = (s.dn[1095][2] + s.dn[1096][2]);
        let eq52_e1999_d_n3: f64 = (s.dn[1095][3] + s.dn[1096][3]);
        let eq52_e1999_d_n4: f64 = (s.dn[1095][4] + s.dn[1096][4]);
        let eq52_e1999_d_n5: f64 = (s.dn[1095][5] + s.dn[1096][5]);
        let eq52_e1999_d_n6: f64 = (s.dn[1095][6] + s.dn[1096][6]);
        let eq52_e1999_d_n7: f64 = (s.dn[1095][7] + s.dn[1096][7]);
        let eq52_e1999_d_n8: f64 = (s.dn[1095][8] + s.dn[1096][8]);
        let eq52_e1999_d_n9: f64 = (s.dn[1095][9] + s.dn[1096][9]);
        let eq52_e1999_d_n10: f64 = (s.dn[1095][10] + s.dn[1096][10]);
        let eq52_e1999_d_n11: f64 = (s.dn[1095][11] + s.dn[1096][11]);
        let eq52_e1999_d_n12: f64 = (s.dn[1095][12] + s.dn[1096][12]);
        let eq52_e1999_d_n13: f64 = (s.dn[1095][13] + s.dn[1096][13]);
        let eq52_e1999_d_b0: f64 = (s.db[1095][0] + s.db[1096][0]);
        let eq52_e1999_d_b1: f64 = (s.db[1095][1] + s.db[1096][1]);
        let eq52_e1999_d_b2: f64 = (s.db[1095][2] + s.db[1096][2]);
        let eq52_e1999_d_b3: f64 = (s.db[1095][3] + s.db[1096][3]);
        let eq52_e1999_d_b4: f64 = (s.db[1095][4] + s.db[1096][4]);
        let eq52_e1999_d_b5: f64 = (s.db[1095][5] + s.db[1096][5]);
        let eq52_e1999_d_b6: f64 = (s.db[1095][6] + s.db[1096][6]);
        let eq52_e1999_d_b7: f64 = (s.db[1095][7] + s.db[1096][7]);
        let eq52_e1999_d_b8: f64 = (s.db[1095][8] + s.db[1096][8]);
        let eq52_e1999_d_b9: f64 = (s.db[1095][9] + s.db[1096][9]);
        let eq52_e1999_d_b10: f64 = (s.db[1095][10] + s.db[1096][10]);
        let eq52_e1999_d_b11: f64 = (s.db[1095][11] + s.db[1096][11]);
        (eq52_e1999, eq52_e1999_d_n0, eq52_e1999_d_n1, eq52_e1999_d_n2, eq52_e1999_d_n3, eq52_e1999_d_n4, eq52_e1999_d_n5, eq52_e1999_d_n6, eq52_e1999_d_n7, eq52_e1999_d_n8, eq52_e1999_d_n9, eq52_e1999_d_n10, eq52_e1999_d_n11, eq52_e1999_d_n12, eq52_e1999_d_n13, eq52_e1999_d_b0, eq52_e1999_d_b1, eq52_e1999_d_b2, eq52_e1999_d_b3, eq52_e1999_d_b4, eq52_e1999_d_b5, eq52_e1999_d_b6, eq52_e1999_d_b7, eq52_e1999_d_b8, eq52_e1999_d_b9, eq52_e1999_d_b10, eq52_e1999_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2001;
        let eq52_node_derivatives: [f64; 14] = [eq52_e2001_d_n0, eq52_e2001_d_n1, eq52_e2001_d_n2, eq52_e2001_d_n3, eq52_e2001_d_n4, eq52_e2001_d_n5, eq52_e2001_d_n6, eq52_e2001_d_n7, eq52_e2001_d_n8, eq52_e2001_d_n9, eq52_e2001_d_n10, eq52_e2001_d_n11, eq52_e2001_d_n12, eq52_e2001_d_n13];
        let eq52_branch_derivatives: [f64; 12] = [eq52_e2001_d_b0, eq52_e2001_d_b1, eq52_e2001_d_b2, eq52_e2001_d_b3, eq52_e2001_d_b4, eq52_e2001_d_b5, eq52_e2001_d_b6, eq52_e2001_d_b7, eq52_e2001_d_b8, eq52_e2001_d_b9, eq52_e2001_d_b10, eq52_e2001_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2005, eq53_e2005_d_n0, eq53_e2005_d_n1, eq53_e2005_d_n2, eq53_e2005_d_n3, eq53_e2005_d_n4, eq53_e2005_d_n5, eq53_e2005_d_n6, eq53_e2005_d_n7, eq53_e2005_d_n8, eq53_e2005_d_n9, eq53_e2005_d_n10, eq53_e2005_d_n11, eq53_e2005_d_n12, eq53_e2005_d_n13, eq53_e2005_d_b0, eq53_e2005_d_b1, eq53_e2005_d_b2, eq53_e2005_d_b3, eq53_e2005_d_b4, eq53_e2005_d_b5, eq53_e2005_d_b6, eq53_e2005_d_b7, eq53_e2005_d_b8, eq53_e2005_d_b9, eq53_e2005_d_b10, eq53_e2005_d_b11,) = {
    if s.b[2011] {
        (s.v[1097], s.dn[1097][0], s.dn[1097][1], s.dn[1097][2], s.dn[1097][3], s.dn[1097][4], s.dn[1097][5], s.dn[1097][6], s.dn[1097][7], s.dn[1097][8], s.dn[1097][9], s.dn[1097][10], s.dn[1097][11], s.dn[1097][12], s.dn[1097][13], s.db[1097][0], s.db[1097][1], s.db[1097][2], s.db[1097][3], s.db[1097][4], s.db[1097][5], s.db[1097][6], s.db[1097][7], s.db[1097][8], s.db[1097][9], s.db[1097][10], s.db[1097][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2005;
        let eq53_node_derivatives: [f64; 14] = [eq53_e2005_d_n0, eq53_e2005_d_n1, eq53_e2005_d_n2, eq53_e2005_d_n3, eq53_e2005_d_n4, eq53_e2005_d_n5, eq53_e2005_d_n6, eq53_e2005_d_n7, eq53_e2005_d_n8, eq53_e2005_d_n9, eq53_e2005_d_n10, eq53_e2005_d_n11, eq53_e2005_d_n12, eq53_e2005_d_n13];
        let eq53_branch_derivatives: [f64; 12] = [eq53_e2005_d_b0, eq53_e2005_d_b1, eq53_e2005_d_b2, eq53_e2005_d_b3, eq53_e2005_d_b4, eq53_e2005_d_b5, eq53_e2005_d_b6, eq53_e2005_d_b7, eq53_e2005_d_b8, eq53_e2005_d_b9, eq53_e2005_d_b10, eq53_e2005_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2010, eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13, eq54_e2010_d_b0, eq54_e2010_d_b1, eq54_e2010_d_b2, eq54_e2010_d_b3, eq54_e2010_d_b4, eq54_e2010_d_b5, eq54_e2010_d_b6, eq54_e2010_d_b7, eq54_e2010_d_b8, eq54_e2010_d_b9, eq54_e2010_d_b10, eq54_e2010_d_b11,) = {
    if (!s.b[2011]) {
        (s.v[1096], s.dn[1096][0], s.dn[1096][1], s.dn[1096][2], s.dn[1096][3], s.dn[1096][4], s.dn[1096][5], s.dn[1096][6], s.dn[1096][7], s.dn[1096][8], s.dn[1096][9], s.dn[1096][10], s.dn[1096][11], s.dn[1096][12], s.dn[1096][13], s.db[1096][0], s.db[1096][1], s.db[1096][2], s.db[1096][3], s.db[1096][4], s.db[1096][5], s.db[1096][6], s.db[1096][7], s.db[1096][8], s.db[1096][9], s.db[1096][10], s.db[1096][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2010;
        let eq54_node_derivatives: [f64; 14] = [eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13];
        let eq54_branch_derivatives: [f64; 12] = [eq54_e2010_d_b0, eq54_e2010_d_b1, eq54_e2010_d_b2, eq54_e2010_d_b3, eq54_e2010_d_b4, eq54_e2010_d_b5, eq54_e2010_d_b6, eq54_e2010_d_b7, eq54_e2010_d_b8, eq54_e2010_d_b9, eq54_e2010_d_b10, eq54_e2010_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2017, eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13, eq55_e2017_d_b0, eq55_e2017_d_b1, eq55_e2017_d_b2, eq55_e2017_d_b3, eq55_e2017_d_b4, eq55_e2017_d_b5, eq55_e2017_d_b6, eq55_e2017_d_b7, eq55_e2017_d_b8, eq55_e2017_d_b9, eq55_e2017_d_b10, eq55_e2017_d_b11,) = {
    if (!s.b[2011]) {
        let eq55_e2015: f64 = (s.v[1095] + s.v[1097]);
        let eq55_e2015_d_n0: f64 = (s.dn[1095][0] + s.dn[1097][0]);
        let eq55_e2015_d_n1: f64 = (s.dn[1095][1] + s.dn[1097][1]);
        let eq55_e2015_d_n2: f64 = (s.dn[1095][2] + s.dn[1097][2]);
        let eq55_e2015_d_n3: f64 = (s.dn[1095][3] + s.dn[1097][3]);
        let eq55_e2015_d_n4: f64 = (s.dn[1095][4] + s.dn[1097][4]);
        let eq55_e2015_d_n5: f64 = (s.dn[1095][5] + s.dn[1097][5]);
        let eq55_e2015_d_n6: f64 = (s.dn[1095][6] + s.dn[1097][6]);
        let eq55_e2015_d_n7: f64 = (s.dn[1095][7] + s.dn[1097][7]);
        let eq55_e2015_d_n8: f64 = (s.dn[1095][8] + s.dn[1097][8]);
        let eq55_e2015_d_n9: f64 = (s.dn[1095][9] + s.dn[1097][9]);
        let eq55_e2015_d_n10: f64 = (s.dn[1095][10] + s.dn[1097][10]);
        let eq55_e2015_d_n11: f64 = (s.dn[1095][11] + s.dn[1097][11]);
        let eq55_e2015_d_n12: f64 = (s.dn[1095][12] + s.dn[1097][12]);
        let eq55_e2015_d_n13: f64 = (s.dn[1095][13] + s.dn[1097][13]);
        let eq55_e2015_d_b0: f64 = (s.db[1095][0] + s.db[1097][0]);
        let eq55_e2015_d_b1: f64 = (s.db[1095][1] + s.db[1097][1]);
        let eq55_e2015_d_b2: f64 = (s.db[1095][2] + s.db[1097][2]);
        let eq55_e2015_d_b3: f64 = (s.db[1095][3] + s.db[1097][3]);
        let eq55_e2015_d_b4: f64 = (s.db[1095][4] + s.db[1097][4]);
        let eq55_e2015_d_b5: f64 = (s.db[1095][5] + s.db[1097][5]);
        let eq55_e2015_d_b6: f64 = (s.db[1095][6] + s.db[1097][6]);
        let eq55_e2015_d_b7: f64 = (s.db[1095][7] + s.db[1097][7]);
        let eq55_e2015_d_b8: f64 = (s.db[1095][8] + s.db[1097][8]);
        let eq55_e2015_d_b9: f64 = (s.db[1095][9] + s.db[1097][9]);
        let eq55_e2015_d_b10: f64 = (s.db[1095][10] + s.db[1097][10]);
        let eq55_e2015_d_b11: f64 = (s.db[1095][11] + s.db[1097][11]);
        (eq55_e2015, eq55_e2015_d_n0, eq55_e2015_d_n1, eq55_e2015_d_n2, eq55_e2015_d_n3, eq55_e2015_d_n4, eq55_e2015_d_n5, eq55_e2015_d_n6, eq55_e2015_d_n7, eq55_e2015_d_n8, eq55_e2015_d_n9, eq55_e2015_d_n10, eq55_e2015_d_n11, eq55_e2015_d_n12, eq55_e2015_d_n13, eq55_e2015_d_b0, eq55_e2015_d_b1, eq55_e2015_d_b2, eq55_e2015_d_b3, eq55_e2015_d_b4, eq55_e2015_d_b5, eq55_e2015_d_b6, eq55_e2015_d_b7, eq55_e2015_d_b8, eq55_e2015_d_b9, eq55_e2015_d_b10, eq55_e2015_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2017;
        let eq55_node_derivatives: [f64; 14] = [eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13];
        let eq55_branch_derivatives: [f64; 12] = [eq55_e2017_d_b0, eq55_e2017_d_b1, eq55_e2017_d_b2, eq55_e2017_d_b3, eq55_e2017_d_b4, eq55_e2017_d_b5, eq55_e2017_d_b6, eq55_e2017_d_b7, eq55_e2017_d_b8, eq55_e2017_d_b9, eq55_e2017_d_b10, eq55_e2017_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq57_e2028, eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13, eq57_e2028_d_b0, eq57_e2028_d_b1, eq57_e2028_d_b2, eq57_e2028_d_b3, eq57_e2028_d_b4, eq57_e2028_d_b5, eq57_e2028_d_b6, eq57_e2028_d_b7, eq57_e2028_d_b8, eq57_e2028_d_b9, eq57_e2028_d_b10, eq57_e2028_d_b11,) = {
    if (!s.b[2012]) {
        let eq57_e2026: f64 = ((nv1 - nv9) * s.v[2013]);
        let eq57_e2026_d_n0: f64 = ((nv1 - nv9) * s.dn[2013][0]);
        let eq57_e2026_d_n1: f64 = (s.v[2013] + ((nv1 - nv9) * s.dn[2013][1]));
        let eq57_e2026_d_n2: f64 = ((nv1 - nv9) * s.dn[2013][2]);
        let eq57_e2026_d_n3: f64 = ((nv1 - nv9) * s.dn[2013][3]);
        let eq57_e2026_d_n4: f64 = ((nv1 - nv9) * s.dn[2013][4]);
        let eq57_e2026_d_n5: f64 = ((nv1 - nv9) * s.dn[2013][5]);
        let eq57_e2026_d_n6: f64 = ((nv1 - nv9) * s.dn[2013][6]);
        let eq57_e2026_d_n7: f64 = ((nv1 - nv9) * s.dn[2013][7]);
        let eq57_e2026_d_n8: f64 = ((nv1 - nv9) * s.dn[2013][8]);
        let eq57_e2026_d_n9: f64 = ((-s.v[2013]) + ((nv1 - nv9) * s.dn[2013][9]));
        let eq57_e2026_d_n10: f64 = ((nv1 - nv9) * s.dn[2013][10]);
        let eq57_e2026_d_n11: f64 = ((nv1 - nv9) * s.dn[2013][11]);
        let eq57_e2026_d_n12: f64 = ((nv1 - nv9) * s.dn[2013][12]);
        let eq57_e2026_d_n13: f64 = ((nv1 - nv9) * s.dn[2013][13]);
        let eq57_e2026_d_b0: f64 = ((nv1 - nv9) * s.db[2013][0]);
        let eq57_e2026_d_b1: f64 = ((nv1 - nv9) * s.db[2013][1]);
        let eq57_e2026_d_b2: f64 = ((nv1 - nv9) * s.db[2013][2]);
        let eq57_e2026_d_b3: f64 = ((nv1 - nv9) * s.db[2013][3]);
        let eq57_e2026_d_b4: f64 = ((nv1 - nv9) * s.db[2013][4]);
        let eq57_e2026_d_b5: f64 = ((nv1 - nv9) * s.db[2013][5]);
        let eq57_e2026_d_b6: f64 = ((nv1 - nv9) * s.db[2013][6]);
        let eq57_e2026_d_b7: f64 = ((nv1 - nv9) * s.db[2013][7]);
        let eq57_e2026_d_b8: f64 = ((nv1 - nv9) * s.db[2013][8]);
        let eq57_e2026_d_b9: f64 = ((nv1 - nv9) * s.db[2013][9]);
        let eq57_e2026_d_b10: f64 = ((nv1 - nv9) * s.db[2013][10]);
        let eq57_e2026_d_b11: f64 = ((nv1 - nv9) * s.db[2013][11]);
        (eq57_e2026, eq57_e2026_d_n0, eq57_e2026_d_n1, eq57_e2026_d_n2, eq57_e2026_d_n3, eq57_e2026_d_n4, eq57_e2026_d_n5, eq57_e2026_d_n6, eq57_e2026_d_n7, eq57_e2026_d_n8, eq57_e2026_d_n9, eq57_e2026_d_n10, eq57_e2026_d_n11, eq57_e2026_d_n12, eq57_e2026_d_n13, eq57_e2026_d_b0, eq57_e2026_d_b1, eq57_e2026_d_b2, eq57_e2026_d_b3, eq57_e2026_d_b4, eq57_e2026_d_b5, eq57_e2026_d_b6, eq57_e2026_d_b7, eq57_e2026_d_b8, eq57_e2026_d_b9, eq57_e2026_d_b10, eq57_e2026_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2028;
        let eq57_node_derivatives: [f64; 14] = [eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13];
        let eq57_branch_derivatives: [f64; 12] = [eq57_e2028_d_b0, eq57_e2028_d_b1, eq57_e2028_d_b2, eq57_e2028_d_b3, eq57_e2028_d_b4, eq57_e2028_d_b5, eq57_e2028_d_b6, eq57_e2028_d_b7, eq57_e2028_d_b8, eq57_e2028_d_b9, eq57_e2028_d_b10, eq57_e2028_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
    }
}
