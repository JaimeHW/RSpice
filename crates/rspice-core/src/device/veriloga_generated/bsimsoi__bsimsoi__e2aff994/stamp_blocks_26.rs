#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_180(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(334), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(167, 400, 320);s.store_sub(168, 400, 320);s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);s.store_mul3_lhs(170, 832, 169, 169);s.store_offset(834, 170, 1.0);s.store_div_mixed_ia(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_limited_exp_neg_input(853, 176);s.store_mul3_lhs(340, 339, 343, 458);s.store_div(337, 740, 340);s.store_mul_ad_product_lhs_mixed_ai(380, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(253), s.ad_value(337), s.ad_value(269), ((2.0 * p[2]) * ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363), 1.0, s.ad_value(334), 1.0), 834, 853);s.store_scale(380, 380, p[26]);s.store_scalar(467, 0.0);}
        s.b[1869] = (p[7] > 1.0);s.store_scalar(1869, if s.b[1869] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1869]) {s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scale(176, 271, p[1009]);s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scaled_add(467, 167, 468, (p[1008] * p[2]));}
        s.b[1870] = (p[7] == 2.0);s.store_scalar(1870, if s.b[1870] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {s.store_primal_div_from_scalar(466, 1.0, 465);}
        s.b[1871] = (s.v[466] < p[1347]);s.store_scalar(1871, if s.b[1871] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1869]) && s.b[1870]) && s.b[1871]) {s.store_scalar(466, p[1347]);s.store_primal_div_from_scalar(465, 1.0, 466);}
        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {s.store_add(178, 465, 467);s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);}
        if (!s.b[1620]) {s.store_scalar(544, ((s.v[183] / p[1373]) + p[1377]));s.store_scalar(543, ((s.v[183] / p[1373]) + p[1378]));s.store_primal_scale(545, 543, p[74]);s.store_primal_scale(546, 544, p[74]);s.store_mul(593, 637, 590);s.store_div(167, 498, 593);s.store_limited_exp(595, 167);s.store_mul(594, 637, 590);s.store_div(167, 499, 594);s.store_limited_exp(596, 167);s.store_mul_scale_offset_mixed_ai(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, 1.0, (-1.0));}
        s.b[1872] = (s.v[550] == 0.0);s.store_scalar(1872, if s.b[1872] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1872])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);s.store_mul(548, 550, 168);s.store_mul(167, 545, 548);}
        s.b[1873] = (s.v[551] == 0.0);s.store_scalar(1873, if s.b[1873] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1873])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);s.store_mul(549, 551, 168);s.store_mul(167, 546, 549);}
        s.b[1874] = (s.v[552] == 0.0);s.store_scalar(1874, if s.b[1874] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1874])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(554, 552, 169);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_181(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1874])) {s.store_mul_scaled_offset_ad_rhs(562, 557, p[925], A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p[925], A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 498, 562);s.store_limited_exp(177, 167);}
        s.b[1875] = ((s.v[558] - s.v[498]) < 0.001);s.store_scalar(1875, if s.b[1875] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1874])) && s.b[1875]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (((!s.b[1620]) && (!s.b[1874])) && (!s.b[1875])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((!s.b[1620]) && (!s.b[1874])) {s.store_mul(170, 545, 554);}
        s.b[1876] = (s.v[553] == 0.0);s.store_scalar(1876, if s.b[1876] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1876])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(555, 553, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p[925], A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p[925], A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 499, 562);s.store_limited_exp(177, 167);}
        s.b[1877] = ((s.v[559] - s.v[499]) < 0.001);s.store_scalar(1877, if s.b[1877] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1876])) && s.b[1877]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (((!s.b[1620]) && (!s.b[1876])) && (!s.b[1877])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((!s.b[1620]) && (!s.b[1876])) {s.store_mul(170, 546, 555);}
        if (!s.b[1620]) {s.store_scalar(602, ((s.v[183] / p[1373]) * p[74]));}
        s.b[1878] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));s.store_scalar(1878, if s.b[1878] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(585, 587, 167);s.store_mul(578, 598, 167);s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(586, 588, 167);s.store_mul(577, 597, 167);s.store_mul_scale_offset_indices(583, 585, 595, 1.0, (-1.0));}
        s.b[1879] = (s.v[583] < 1e-5);s.store_scalar(1879, if s.b[1879] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1879]) {s.store_scalar(583, 0.0);s.store_scalar(591, 1.0);}
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1879])) {s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));}
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_mul_scale_offset_indices(584, 586, 596, 1.0, (-1.0));}
        s.b[1880] = (s.v[584] < 1e-5);s.store_scalar(1880, if s.b[1880] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1880]) {s.store_scalar(584, 0.0);s.store_scalar(592, 1.0);}
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1880])) {s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));}
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p[595]) / p[595]));s.store_limited_exp(603, 167);s.store_sub_from_scalar(169, 1.0, 603);s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p[595])));s.store_pow_indices(599, 167, 600);s.store_mul3_lhs(604, 602, 578, 599);s.store_mul(168, 167, 604);s.store_mul3_lhs(604, 602, 577, 599);s.store_mul(168, 167, 604);s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p[920], 1.0);s.store_mul3_lhs(532, 602, 578, 531);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_182(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_mul_ad_product_lhs_mixed_ia(533, 532, A::offset(s.ad_value(595), (-1.0)), 591);s.store_mul3_lhs(532, 602, 577, 531);s.store_mul_ad_product_lhs_mixed_ia(534, 532, A::offset(s.ad_value(596), (-1.0)), 592);s.store_primal_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);}
        s.b[1881] = (s.v[580] < 1.0);s.store_scalar(1881, if s.b[1881] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1881]) {s.store_scalar(580, 1.0);}
        s.b[1882] = (p[554] == 1.0);s.store_scalar(1882, if s.b[1882] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {s.store_offset_div_scaled_inputs2_indices(167, 498, 1.0, 499, 1.0, 580, 1.0, 1.0);s.store_add(168, 583, 584);s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);s.store_scaled_add(169, 167, 170, 0.5);s.store_mul(167, 603, 604);}
        s.b[1884] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1884])) {s.store_mul_scale_offset_indices(174, 569, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(571, 567, 167);s.store_mul_scale_offset_indices(174, 570, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(572, 568, 167);s.store_scale(594, 573, p[925]);}
        s.b[1885] = ((s.v[575] - s.v[498]) < 0.001);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1885]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1885])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if ((!s.b[1620]) && (!s.b[1884])) {s.store_scale(594, 574, p[925]);}
        s.b[1886] = ((s.v[576] - s.v[499]) < 0.001);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1886]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1886])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        s.b[1887] = (p[36] == 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1887]) {s.store_scalar(167, (s.v[200] * p[76]));}
        s.b[1888] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1887]) && s.b[1888]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {s.store_div_scaled_inputs3_indices(168, 204, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1889] = (s.v[894] != 0.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && s.b[1889]) {s.store_mul_square_lhs(170, 201, 201);s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && (!s.b[1889])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        s.b[1890] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1887]) && s.b[1890]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {s.store_div_scaled_inputs3_indices(168, 203, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_183(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1891] = (s.v[898] != 0.0);s.store_scalar(1891, if s.b[1891] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && s.b[1891]) {s.store_mul_square_lhs(170, 202, 202);s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && (!s.b[1891])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        if ((!s.b[1620]) && (!s.b[1887])) {s.store_scalar(167, (s.v[200] * p[76]));s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1892] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));s.store_scalar(1892, if s.b[1892] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1892]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {s.store_div_scaled_inputs3_indices(168, 207, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1893] = (s.v[903] != 0.0);s.store_scalar(1893, if s.b[1893] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && s.b[1893]) {s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(903), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && (!s.b[1893])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        s.b[1894] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1894]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {s.store_div_scaled_inputs3_indices(168, 206, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1895] = (s.v[906] != 0.0);s.store_scalar(1895, if s.b[1895] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && s.b[1895]) {s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);s.store_offset(171, 170, 0.0001);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_184(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && s.b[1895]) {s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(906), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && (!s.b[1895])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {s.store_mul3_ad(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        s.b[1896] = (p[44] == 0.0);s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });s.b[1897] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });s.b[1898] = (s.v[355] > (s.v[659] / 80.0));s.store_scalar(1898, if s.b[1898] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1896]) && (!s.b[1897])) && s.b[1898]) {s.store_div_scaled_inputs_indices(168, 659, -1.0, 355, 1.0);}
        s.b[1899] = (p[44] == 1.0);s.store_scalar(1899, if s.b[1899] { 1.0 } else { 0.0 });s.b[1900] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p[600], (((((-1.0)) * (p[600]))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p[643]) * p[643])), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p[644]) * p[644]), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        s.b[1901] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1901, if s.b[1901] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p[600], (((((-1.0)) * (p[600]))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p[643]) * p[643])), 0.5), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_185(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p[644]) * p[644]), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));s.store_mul_scale_offset_rhs(378, 880, 639, p[666], (((((-1.0)) * (p[666]))) + (1.0)));}
        s.b[1902] = (s.v[211] > 0.0);s.store_scalar(1902, if s.b[1902] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1902]) {s.store_sub(168, 378, 499);}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1902])) {s.store_sub(168, 378, 498);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_offset(169, 881, (-1.0));}
        s.b[1903] = (s.v[168] > 0.0);s.store_scalar(1903, if s.b[1903] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1903]) {s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1903])) {s.store_scalar(170, 0.0);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_limited_exp(171, 170);}
        if (!s.b[1620]) {s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);}
        s.b[1904] = ((p[37] != 0.0) || (p[38] != 0.0));s.store_scalar(1904, if s.b[1904] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1904]) {s.store_mul_add_scaled_inputs4_indices_rhs(469, 269, 213, 1.0, 254, (-1.0), 400, 1.0, 320, 1.0);s.store_sqrt_square_offset(168, 469, 0.0001);s.store_scaled_sub(471, 168, 469, 0.5);s.store_scaled_add(470, 469, 168, 0.5);}
        s.b[1905] = (p[38] != 0.0);s.store_scalar(1905, if s.b[1905] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 469, 1.0 / (p[671]));}
        s.b[1906] = (p[696] != 0.0);s.store_scalar(1906, if s.b[1906] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1906]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p[696]));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1906])) {s.store_scalar(167, 1.0);}
        s.b[1907] = (s.v[167] < 0.01);s.store_scalar(1907, if s.b[1907] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1907]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p[1373]) + (p[1381] / p[2])) * p[700]));s.store_scalar(169, (p[701] * p[76]));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_scaled_sub(168, 469, 809, 1.0 / (p[671]));}
        s.b[1908] = (p[697] != 0.0);s.store_scalar(1908, if s.b[1908] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1908]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p[697]));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1908])) {s.store_scalar(167, 1.0);}
        s.b[1909] = (s.v[167] < 0.01);s.store_scalar(1909, if s.b[1909] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1909]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p[1373]) + (p[1381] / p[2])) * p[698]));s.store_scalar(169, (p[699] * p[76]));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_offset_mul(478, 212, 269, p[1383]);}
        s.b[1910] = (((((p[43] != 0.0) && true) && (!((p[40] != 0.0) && (!true)))) && (p[45] == 1.0)) && (p[1380] > 0.0));s.store_scalar(1910, if s.b[1910] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));s.store_sub(167, 208, 478);s.store_sqrt_square_offset(168, 167, 0.0001);s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(178, (if (p[30] == 1.0) { p[702] } else { p[703] }));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_186(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(179, (if (p[30] == 1.0) { p[704] } else { p[705] }));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul(169, 208, 209);s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);s.store_mul(171, 889, 890);s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p[76]), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p[76]));s.store_limited_exp(173, 172);s.store_scaled_mul(178, 178, 492, p[1380]);}
        s.b[1911] = (p[37] != 0.0);s.store_scalar(1911, if s.b[1911] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));s.store_offset_mul(169, 812, 470, 1.0);s.store_scaled_mul(170, 168, 169, s.v[488]);s.store_mul_product3_mixed_aiia(171, A::limited_exp(s.ad_value(170)), 253, 269, A::add(s.ad_value(400), s.ad_value(320)), 1.0);s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_scale(168, 472, s.v[823]);s.store_limited_exp_neg_input(482, 168);s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);s.store_offset_square(172, 168, 0.0002);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1913] = (p[1295] == 1.0);s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1914] = (s.v[818] < 0.01);s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) && s.b[1914]) {s.store_scalar(818, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1913])) {s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 818, 228, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(229, 169, 0.0001);}
        s.b[1915] = (p[1295] == 1.0);s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1916] = (s.v[821] < 0.01);s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) && s.b[1916]) {s.store_scalar(821, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1915])) {s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 821, 229, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);}
        if (!s.b[1620]) {s.store_mul(502, 666, 463);s.store_mul(505, 667, 494);s.store_scale(508, 671, (s.v[189] * p[2]));s.store_scalar(503, ((0.1) as f64).powf((-p[913])));}
        s.b[1917] = (p[913] == 1.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1917]) {s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_187(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1917])) {s.store_primal_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p[913]) * (1.0 + p[913]))), (-(1.0 / (1.0 - p[913]))), (1.0 / (1.0 - p[913])));}
        if (!s.b[1620]) {s.store_scalar(506, ((0.1) as f64).powf((-p[915])));}
        s.b[1918] = (p[915] == 1.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1918]) {s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1918])) {s.store_primal_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p[915]) * (1.0 + p[915]))), (-(1.0 / (1.0 - p[915]))), (1.0 / (1.0 - p[915])));}
        if (!s.b[1620]) {s.store_scalar(509, ((0.1) as f64).powf((-p[917])));}
        s.b[1919] = (p[917] == 1.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1919]) {s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1919])) {s.store_primal_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p[917]) * (1.0 + p[917]))), (-(1.0 / (1.0 - p[917]))), (1.0 / (1.0 - p[917])));}
        s.b[1920] = (s.v[502] > 0.0);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1920]) {s.store_div(168, 498, 672);}
        s.b[1921] = (s.v[168] < 0.9);s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1920]) && s.b[1921]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1922] = (p[913] != 1.0);s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });s.b[1923] = (p[913] == 0.5);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && s.b[1923]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[913]));}
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[913])), 0.0);}
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && (!s.b[1922])) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1920]) && (!s.b[1921])) {s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[913]), (((((-1.0)) * ((5.0 * p[913])))) + ((1.0 + p[913])))));s.store_mul_ad_product_rhs_mixed_ia(521, 672, 502, A::add(s.ad_value(169), s.ad_value(504)));}
        if ((!s.b[1620]) && (!s.b[1920])) {s.store_scalar(521, 0.0);}
        s.b[1924] = (s.v[505] > 0.0);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1924]) {s.store_div(168, 498, 673);}
        s.b[1925] = (s.v[168] < 0.9);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1924]) && s.b[1925]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1926] = (p[915] != 1.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });s.b[1927] = (p[915] == 0.5);s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && s.b[1927]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[915]));}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[915])), 0.0);}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && (!s.b[1926])) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1924]) && (!s.b[1925])) {s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[915]), (((((-1.0)) * ((5.0 * p[915])))) + ((1.0 + p[915])))));s.store_mul_ad_product_rhs_mixed_ia(522, 673, 505, A::add(s.ad_value(169), s.ad_value(507)));}
        if ((!s.b[1620]) && (!s.b[1924])) {s.store_scalar(522, 0.0);}
        s.b[1928] = (s.v[508] > 0.0);s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1928]) {s.store_div(168, 498, 674);}
        s.b[1929] = (s.v[168] < 0.9);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1928]) && s.b[1929]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1930] = (p[917] != 1.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_188(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1931] = (p[917] == 0.5);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && s.b[1931]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[917]));}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[917])), 0.0);}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && (!s.b[1930])) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1928]) && (!s.b[1929])) {s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[917]), (((((-1.0)) * ((5.0 * p[917])))) + ((1.0 + p[917])))));s.store_mul_ad_product_rhs_mixed_ia(523, 674, 508, A::add(s.ad_value(169), s.ad_value(510)));}
        if ((!s.b[1620]) && (!s.b[1928])) {s.store_scalar(523, 0.0);}
        if (!s.b[1620]) {s.store_scale(524, 533, (p[919] * p[2]));s.store_add_scaled_inputs4_indices(520, 521, 1.0, 522, 1.0, 523, 1.0, 524, 1.0);s.store_mul(511, 669, 464);s.store_mul(514, 670, 495);s.store_scale(517, 668, (s.v[189] * p[2]));s.store_scalar(512, ((0.1) as f64).powf((-p[914])));}
        s.b[1932] = (p[914] == 1.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1932]) {s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1932])) {s.store_primal_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p[914]) * (1.0 + p[914]))), (-(1.0 / (1.0 - p[914]))), (1.0 / (1.0 - p[914])));}
        if (!s.b[1620]) {s.store_scalar(515, ((0.1) as f64).powf((-p[916])));}
        s.b[1933] = (p[916] == 1.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1933]) {s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1933])) {s.store_primal_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p[916]) * (1.0 + p[916]))), (-(1.0 / (1.0 - p[916]))), (1.0 / (1.0 - p[916])));}
        if (!s.b[1620]) {s.store_scalar(518, ((0.1) as f64).powf((-p[918])));}
        s.b[1934] = (p[918] == 1.0);s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1934]) {s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1934])) {s.store_primal_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p[918]) * (1.0 + p[918]))), (-(1.0 / (1.0 - p[918]))), (1.0 / (1.0 - p[918])));}
        s.b[1935] = (s.v[511] > 0.0);s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1935]) {s.store_div(168, 499, 675);}
        s.b[1936] = (s.v[168] < 0.9);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1935]) && s.b[1936]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1937] = (p[914] != 1.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });s.b[1938] = (p[914] == 0.5);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && s.b[1938]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[914]));}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[914])), 0.0);}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && (!s.b[1937])) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1935]) && (!s.b[1936])) {s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[914]), (((((-1.0)) * ((5.0 * p[914])))) + ((1.0 + p[914])))));s.store_mul_ad_product_rhs_mixed_ia(526, 675, 511, A::add(s.ad_value(169), s.ad_value(513)));}
        if ((!s.b[1620]) && (!s.b[1935])) {s.store_scalar(526, 0.0);}
        s.b[1939] = (s.v[514] > 0.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1939]) {s.store_div(168, 499, 676);}
        s.b[1940] = (s.v[168] < 0.9);s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1939]) && s.b[1940]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1941] = (p[916] != 1.0);s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_189(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1942] = (p[916] == 0.5);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && s.b[1942]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[916]));}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[916])), 0.0);}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && (!s.b[1941])) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1939]) && (!s.b[1940])) {s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[916]), (((((-1.0)) * ((5.0 * p[916])))) + ((1.0 + p[916])))));s.store_mul_ad_product_rhs_mixed_ia(527, 676, 514, A::add(s.ad_value(169), s.ad_value(516)));}
        if ((!s.b[1620]) && (!s.b[1939])) {s.store_scalar(527, 0.0);}
        s.b[1943] = (s.v[517] > 0.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1943]) {s.store_div(168, 499, 677);}
        s.b[1944] = (s.v[168] < 0.9);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1943]) && s.b[1944]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1945] = (p[918] != 1.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });s.b[1946] = (p[918] == 0.5);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && s.b[1946]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p[918]));}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p[918])), 0.0);}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && (!s.b[1945])) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p[918]), (((((-1.0)) * ((5.0 * p[918])))) + ((1.0 + p[918])))));s.store_mul_ad_product_rhs_mixed_ia(528, 677, 517, A::add(s.ad_value(169), s.ad_value(519)));}
        if ((!s.b[1620]) && (!s.b[1943])) {s.store_scalar(528, 0.0);}
        if (!s.b[1620]) {s.store_scale(529, 534, (p[919] * p[2]));s.store_add_scaled_inputs4_indices(525, 526, 1.0, 527, 1.0, 528, 1.0, 529, 1.0);}
        s.b[1947] = (p[28] != 0.0);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1947]) {s.store_powf_scaled_input(168, 706, 1.0000000000000001e-23, p[1144]);s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p[1145]);s.store_div_scaled_product_mixed_iai(170, 379, A::voltage(ctx, nodes, Some(10), Some(7)), p[1143], 271, 1.0);}
        if (!s.b[1620]) {s.store_div_scaled_inputs_indices(607, 746, 2.0, 337, 1.0);}
        s.b[1948] = (p[1011] <= 0.0);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1948]) {s.store_scalar(610, 0.0);}
        if ((!s.b[1620]) && (!s.b[1948])) {s.store_div_scaled_offset_numerator_mixed_ai(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p[1011], 607, 1.0);s.store_mul_ln_mixed_ia(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));}
        s.b[1949] = (s.v[610] < 0.0);s.store_scalar(1949, if s.b[1949] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1948])) && s.b[1949]) {s.store_scalar(610, 0.0);}
        if (!s.b[1620]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), 709, 1.0 / (1.602176462e-19));s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(320), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_190(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);s.store_add_scaled_product_mixed_aii(1006, A::scale_offset(s.ad_value(612), p[1013], p[1012]), 1.0, 612, 612, p[1014]);s.store_square_ad(1007, A::add(s.ad_value(612), s.ad_value(613)));s.store_scale(1008, 271, (p[1012] * 1.602176462e-19));}
        s.b[1950] = (p[1319] == 1.0);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1950]) {s.store_scalar(1014, p[1320]);}
        s.b[1951] = (s.v[184] > s.v[1014]);s.store_scalar(1951, if s.b[1951] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1951]) {s.store_sub_from_scalar(167, s.v[184], 1014);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1951])) {s.store_scalar(1014, s.v[184]);s.copy_ad(167, 1014);}
        s.b[1952] = (p[1015] >= (s.v[167] / 2.0));s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1952]) {s.store_scalar(606, 0.0);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1952])) {s.store_scalar(606, p[1015]);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scalar(1013, s.v[184]);s.store_div_scaled_inputs2_indices(980, 221, 1.0, 707, (-1.0), 271, 1.0);s.store_scaled_sqrt_ad(981, A::div_from_scalar((((2.0 * 1.602176462e-19) * s.v[180]) * p[1322]), s.ad_value(271)), 1.0 / (s.v[199]));s.store_ln_ad(982, A::div_from_scalar(p[1322], s.ad_value(182)));s.store_scalar(168, 1.0);s.store_div(404, 980, 168);s.store_div(405, 981, 168);s.store_sub_scaled_inputs_mixed_ia(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));}
        s.b[1953] = (s.v[404] < 0.0);s.store_scalar(1953, if s.b[1953] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1953]) {s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);s.store_neg_ad(983, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1953])) {s.store_limited_exp_neg_input(170, 169);s.store_scale(168, 405, 0.5);s.store_sub_mixed_ai(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_sub_offset_lhs_mixed_ai(983, A::square(s.ad_value(169)), 1.0, 170);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scaled_add_offset_sqrt_square_offset(175, 983, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(981), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 981, 1.0);s.store_add_scaled_inputs3_indices(168, 983, 1.0, 982, (-2.0), 225, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_191(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1620]) && s.b[1950]) {s.copy_ad(257, 259);}
        s.b[1954] = (s.v[175] <= (-68.0));s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1955] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && s.b[1955]) {s.store_limited_exp(170, 171);}
        s.b[1956] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && s.b[1956]) {s.store_limited_exp(170, 175);}
        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && (!s.b[1956])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(985, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_192(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(985, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scaled_add_offset_sqrt_square_offset(984, 983, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_offset_div_scaled_inputs_sqrt_rhs(986, 981, 1.0, 984, 2.0, 1.0);s.copy_ad(987, 337);s.store_scale(994, 987, (s.v[199] * s.v[183]));s.store_scale(993, 337, (s.v[199] * s.v[183]));s.store_div_scaled_product_by_product_mixed_iiai(988, 380, 1014, 1.0, A::mul3_scaled_output(s.ad_value(986), s.ad_value(994), s.ad_value(271), 2.0), 271, 1.0);s.store_div_scaled_product_by_product_mixed_iaai(990, 380, A::sub(s.ad_value(1013), s.ad_value(1014)), 1.0, A::mul3_scaled_output(s.ad_value(253), s.ad_value(993), s.ad_value(269), 2.0), 269, 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(167, A::square(s.ad_value(985)), 4.0, 985, 4.0, 988, (-4.0), 1.0);s.store_offset_scaled_ad(991, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(320)), 4.0, s.ad_value(320), 4.0, s.ad_value(990), 4.0), 1.0)), 0.5, (-0.5));}
        s.b[1958] = (s.v[184] != s.v[1014]);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {s.store_mul3_affine_lhs(992, 253, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 991);s.store_primal_add_scaled_inputs3_indices(608, 1013, 1.0, 606, (-2.0), 1014, -1.0);s.store_primal_square(609, 608);s.store_scale(168, 609, (10000000000.0 * s.v[199]));s.store_scaled_ln_ad(169, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(992), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p[1012]);s.store_scaled_sub(170, 992, 612, p[1013]);s.store_scaled_sub_ad(171, A::square(s.ad_value(992)), A::square(s.ad_value(612)), (0.5 * p[1014]));s.store_scale(172, 609, (10000000000.0 * (s.v[183] * p[2])));s.store_add_scaled_product(1000, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(172), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(168)), A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(170), 1.0, s.ad_value(171), 1.0), 1.0);s.store_mul3_affine_lhs(173, 608, 613, ((s.v[183] * p[2]) * 10000000000.0), 0.0, 613);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_193(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {s.store_mul_ad_product_lhs_mixed_ai(1001, A::div(s.ad_value(1008), s.ad_value(173)), 380, 380);s.store_add(174, 1001, 1000);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scale(175, 271, (p[1321] * 1.602176462e-19));s.store_mul3_affine_lhs(176, 1014, 613, ((s.v[183] * p[2]) * 10000000000.0), 0.0, 613);s.store_mul_ad_product_lhs_mixed_ai(1009, A::div(s.ad_value(175), s.ad_value(176)), 380, 380);s.copy_ad(177, 1009);}
        s.b[1961] = (p[1015] >= (s.v[184] / 2.0));s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1961]) {s.store_scalar(606, 0.0);}
        if (((!s.b[1620]) && (!s.b[1950])) && (!s.b[1961])) {s.store_scalar(606, p[1015]);}
        s.b[1962] = (((p[1012] > 0.0) || (p[1013] > 0.0)) || (p[1014] > 0.0));s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) {s.store_primal_sub_from_scalar_scaled_input(608, s.v[184], 606, 2.0);s.store_primal_square(609, 608);s.store_scale(167, 609, (10000000000.0 * s.v[199]));s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(400), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p[1012]);s.store_scaled_sub(169, 611, 612, p[1013]);s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p[1014]));s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p[2])));s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p[2]) * 10000000000.0), 0.0, 613);s.store_mul_ad_product_lhs_mixed_ai(615, A::div(s.ad_value(1008), s.ad_value(172)), 380, 380);s.store_add(173, 615, 614);}
        if (!s.b[1620]) {s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));s.store_square(168, 167);s.store_offset_scaled(170, 168, (((p[1022] * s.v[184])) * (p[1019])), p[1019]);s.store_offset_scaled(171, 168, (((p[1023] * s.v[184])) * (p[1020])), p[1020]);s.store_offset_scaled(172, 168, (((p[1298] * s.v[184])) * (p[1297])), p[1297]);s.store_square(633, 172);s.store_square(632, 171);}
        s.b[1964] = (p[39] == 0.0);s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });s.b[1965] = (p[39] == 1.0);s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1964]) {s.store_scaled_mul(388, 271, 382, ((((-p[2]) * s.v[183]) * s.v[184]) * s.v[199]));s.store_scaled_mul(389, 271, 385, ((((-p[2]) * s.v[183]) * s.v[184]) * s.v[199]));s.store_mul_abs_mixed_ia(167, 337, A::add(s.ad_value(388), s.ad_value(389)));s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_scaled_mul(626, 253, 269, 2.0);s.store_mul_scale_offset_mixed_ia(167, 626, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 0.0);s.store_scaled_add(168, 400, 320, 0.5);s.store_offset(170, 168, 0.5);s.store_square(171, 170);s.store_mul(172, 171, 170);s.store_sub(173, 400, 320);s.store_square(174, 173);s.store_mul(175, 174, 173);s.store_mul_scale_offset_rhs(176, 174, 168, 6.0, 0.5);s.store_scale(625, 345, s.v[184]);s.store_scale(177, 625, 1.0 / (s.v[184]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_194(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(316), A::offset(s.ad_value(243), p[1299]), 1.0), 1.0);}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_offset_scaled(179, 179, { let limited_exp_arg = ((-s.v[184]) / p[1296]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p[1296]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_scaled_add_sqrt_square_offset_rhs(179, 179, 179, ((0.25 * 0.1) * 0.1), 0.5);s.store_div_scaled_product3_mixed_aaii(622, A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), 632, (15.0 * 1.0 / (4.0)), 167, ((p[2] * s.v[183]) * 12.0));}
        if (!s.b[1620]) {s.copy_ad(217, 213);s.store_scalar(418, 0.0);}
        s.b[1970] = (p[31] == 1.0);s.store_scalar(1970, if s.b[1970] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1970]) {s.store_offset(793, 793, p[25]);s.store_mul(222, 221, 272);s.store_mul(225, 224, 272);s.store_mul(212, 793, 272);s.store_sub(217, 222, 212);s.store_ln_ad(432, A::max_with_scalar(A::div(s.ad_value(794), s.ad_value(182)), 1e-38));s.store_scaled_sqrt_mul_scaled_lhs(433, 794, ((2.0 * 1.602176462e-19) * s.v[180]), 272, 1.0 / (s.v[199]));s.store_div_from_scalar(295, 1.0, 433);s.store_div_scaled_inputs_indices(406, 704, ((2.0 * 1.602176462e-19) * s.v[180]), 271, (s.v[199] * s.v[199]));}
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
        if ((!s.b[1620]) && s.b[1970]) {s.store_offset(168, 403, 1.0);s.store_div(404, 217, 168);s.store_div(405, 433, 168);s.store_sub_scaled_inputs_mixed_ia(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));}
        s.b[1971] = (s.v[404] < 0.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1971]) {s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1971])) {s.store_limited_exp_neg_input(170, 169);s.store_scale(168, 405, 0.5);s.store_sub_mixed_ai(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_sub_offset_lhs_mixed_ai(254, A::square(s.ad_value(169)), 1.0, 170);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_195(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1620]) && s.b[1970]) {s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 433, 1.0);s.store_add_scaled_inputs3_indices(168, 254, 1.0, 432, (-2.0), 225, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);s.copy_ad(257, 259);}
        s.b[1972] = (s.v[175] <= (-68.0));s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1973] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && s.b[1973]) {s.store_limited_exp(170, 171);}
        s.b[1974] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && s.b[1974]) {s.store_limited_exp(170, 175);}
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && (!s.b[1974])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(400, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1972])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);}
    }
}
