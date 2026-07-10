#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_174(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1851]) {s.store_div_scaled_product_mixed_iaa(172, 314, A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);}
        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1851])) {s.store_mul_div_scaled_inputs_indices(172, 314, 168, (-2.0), 169, 1.0);}
        if ((!s.b[1620]) && (!s.b[1843])) {s.store_add_scaled_value_products_mixed_iiiia(173, 170, 1.0, 167, 172, 1.0, 314, A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(307, 167, 171, 173);}
        if (!s.b[1620]) {s.store_add_scaled_inputs4_mixed_iiia(319, 254, 1.0, 252, (-2.0), 307, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(294), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(312, 319, 269);}
        s.b[1852] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));s.store_scalar(1852, if s.b[1852] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1852]) {s.store_scalar(1019, 1.0);}
        if ((!s.b[1620]) && (!s.b[1852])) {s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(400), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0, 1.0);s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        if (!s.b[1620]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(316, 312, 0.5, 224, ((-1.0) * 0.5), 312, 224, ((0.25 * 0.001) * 0.001), 0.5);s.store_div(316, 316, 1019);s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(316)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));s.store_mul(315, 226, 175);s.store_mul_add_lhs(318, 315, 224, 270);s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 294, 1.0);s.store_add_scaled_inputs3_indices(168, 254, 1.0, 252, (-2.0), 318, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_175(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1620]) {s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);s.copy_ad(257, 259);}
        s.b[1853] = (s.v[175] <= (-68.0));s.store_scalar(1853, if s.b[1853] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1853]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1854] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1854, if s.b[1854] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1853]) && s.b[1854]) {s.store_limited_exp(170, 171);}
        s.b[1855] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1855, if s.b[1855] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1853]) && (!s.b[1854])) && s.b[1855]) {s.store_limited_exp(170, 175);}
        if ((((!s.b[1620]) && s.b[1853]) && (!s.b[1854])) && (!s.b[1855])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if ((!s.b[1620]) && s.b[1853]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(320, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if ((!s.b[1620]) && (!s.b[1853])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_176(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1620]) && (!s.b[1853])) {s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(320, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));}
        if (!s.b[1620]) {s.store_add_scaled_inputs3_offset_indices(255, 254, 1.0, 400, (-1.0), 320, -1.0, (-1.0));s.store_scaled_add_offset_sqrt_square_offset(167, 255, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(169, 167);s.store_offset_div_ad(253, s.ad_value(294), A::add(s.ad_value(259), s.ad_value(169)), 1.0);s.store_square_ad(417, A::sub(s.ad_value(400), s.ad_value(320)));s.store_div_from_scalar_add_ad(167, 1.0, A::offset(s.ad_value(400), 1.0), s.ad_value(320));s.store_mul(168, 417, 167);s.store_add_scaled_inputs_product_mixed_iiaa(381, 213, 1.0, 254, (-1.0), A::offset(s.ad_value(253), (-1.0)), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(168), 0.3333333333333333), (-1.0));s.store_scale(169, 253, 0.3333333333333333);s.store_mul(170, 168, 167);s.store_mul_mixed_ia(382, 169, A::add_scaled_inputs_product(s.ad_value(400), 2.0, s.ad_value(320), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 0.8, 1.0), 1.0, s.ad_value(320), 1.2), s.ad_value(170), 0.5));s.store_mul_mixed_ia(385, 169, A::add_scaled_inputs_product(s.ad_value(400), 1.0, s.ad_value(320), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 1.2, 1.0), 1.0, s.ad_value(320), 0.8), s.ad_value(170), 0.5));s.store_add_scaled_product_mixed_aii(244, A::sqrt_square_offset(A::mul(s.ad_value(269), s.ad_value(381)), ((0.25 * 0.1) * 0.1)), 0.5, 269, 381, 0.5);s.store_mul_add_rhs(243, 269, 382, 385);s.store_mul_add_scaled_inputs_rhs_indices(336, 335, 244, 1.0, 243, s.v[338]);s.store_pow_ad(169, A::scaled_offset(A::div(s.ad_value(243), s.ad_value(244)), 1.0, 0.5), s.ad_value(757));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_177(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);s.store_offset(171, 170, 1.0);s.store_scaled_add_offset_sqrt_square_offset(339, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);s.store_div_scaled_inputs_mixed_ia(310, 746, 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);s.store_scale(311, 310, s.v[184]);}
        s.b[1856] = (s.v[781] > 0.0);s.store_scalar(1856, if s.b[1856] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1856]) {s.store_offset_div_scaled_product_indices(360, 781, 243, 1.0, 311, 1.0, 1.0);}
        if ((!s.b[1620]) && (!s.b[1856])) {s.store_div_from_scalar_sub_from_scalar_ad(360, 1.0, 1.0, A::div_scaled_product(s.ad_value(781), s.ad_value(243), 1.0, s.ad_value(311), 1.0));}
        if (!s.b[1620]) {s.copy_ad(359, 763);s.store_sub(355, 226, 315);s.store_add_scaled_inputs(362, 243, 1.0, 269, 2.0);}
        s.b[1857] = (s.v[359] > 0.0);s.store_scalar(1857, if s.b[1857] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1857]) {s.store_div_add_scaled_inputs_rhs_indices(170, 362, 316, 1.0, 362, 1.0);s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);s.store_div_from_scalar(172, 1.0, 171);s.store_mul_product3_mixed_iaii(361, 172, A::div(s.ad_value(362), s.ad_value(359)), 170, 360, 1.0);s.store_offset_div(363, 355, 361, 1.0);}
        if ((!s.b[1620]) && (!s.b[1857])) {s.store_scalar(363, 1.0);}
        s.b[1858] = (s.v[769] <= 0.0);s.store_scalar(1858, if s.b[1858] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1858]) {s.store_scalar(268, 1.0);}
        if ((!s.b[1620]) && (!s.b[1858])) {s.store_div_scaled_inputs_indices(176, 769, ((s.v[184]) as f64).sqrt(), 362, 1.0);s.store_div_from_scalar_offset_input(268, 1.0, 176, 1.0);}
        if (!s.b[1620]) {s.store_add(358, 316, 311);}
        s.b[1859] = (s.v[785] > 0.0);s.store_scalar(1859, if s.b[1859] { 1.0 } else { 0.0 });s.b[1860] = (p.p414 < 0.0);s.store_scalar(1860, if s.b[1860] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1859]) && s.b[1860]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), 268, 1.0);}
        if (((!s.b[1620]) && s.b[1859]) && (!s.b[1860])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((!s.b[1620]) && s.b[1859]) {s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);}
        s.b[1861] = (p.p414 < 0.0);s.store_scalar(1861, if s.b[1861] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1859])) && s.b[1861]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), 268, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_178(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && (!s.b[1859])) && (!s.b[1861])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((!s.b[1620]) && (!s.b[1859])) {s.store_offset(364, 168, 1.0);}
        if (!s.b[1620]) {s.store_mul(363, 363, 364);s.store_limited_exp_mul(168, 768, 226);}
        s.b[1862] = (s.v[767] > 0.0);s.store_scalar(1862, if s.b[1862] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1862]) {s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));s.store_div_scaled_offset_numerator_mixed_ai(356, A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, 767, 1.0);s.store_mul(356, 356, 268);}
        if ((!s.b[1620]) && (!s.b[1862])) {s.store_scalar(356, 5.540622384e34);}
        if (!s.b[1620]) {s.store_div(171, 355, 356);s.store_offset(167, 171, 1.0);s.store_mul(363, 363, 167);}
        s.b[1863] = (s.v[766] > 0.0);s.store_scalar(1863, if s.b[1863] { 1.0 } else { 0.0 });s.b[1864] = (s.v[355] > ((s.v[765] * s.v[300]) / 80.0));s.store_scalar(1864, if s.b[1864] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1863]) && s.b[1864]) {s.store_div_scaled_product_indices(167, 765, 300, 1.0, 355, 1.0);s.store_div_scaled_inputs_limited_exp_lhs(357, 167, s.v[184], 766, 1.0);}
        if (((!s.b[1620]) && s.b[1863]) && (!s.b[1864])) {s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);}
        if ((!s.b[1620]) && (!s.b[1863])) {s.store_scalar(357, 5.540622384e34);}
        if (!s.b[1620]) {s.store_offset_div(365, 355, 357, 1.0);s.store_mul(363, 363, 365);s.store_pow_ad(167, s.ad_value(339), A::div_from_scalar(1.0, s.ad_value(348)));s.store_mul(178, 678, 218);s.store_sqrt_square_offset(179, 178, 0.1);s.store_scaled_add_ad(168, A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179))), 0.5);s.store_div_scaled_product_offset_denominator_mixed_iia(169, 243, 168, (10.0 * p.p497), A::mul(s.ad_value(243), s.ad_value(168)), (10.0 * p.p497), 1.0);}
        s.b[1865] = (s.v[780] < 0.0);s.store_scalar(1865, if s.b[1865] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1865]) {s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))), 2.0);}
        if ((!s.b[1620]) && (!s.b[1865])) {s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0), 2.0);}
        if (!s.b[1620]) {s.store_mul_sub_scaled_inputs_rhs_indices(168, 314, 400, 2.0, 320, 2.0);s.store_sqrt_square_offset(169, 168, 1.0);}
        s.b[1866] = (s.v[168] != 0.0);s.store_scalar(1866, if s.b[1866] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1866]) {s.store_add_scaled_product_mixed_iaa(343, 169, 0.5, A::div_from_scalar(1.0, s.ad_value(168)), A::asinh(s.ad_value(168)), 0.5);}
        if ((!s.b[1620]) && (!s.b[1866])) {s.store_scaled_add_mixed_ia(343, 169, A::div_from_scalar(1.0, s.ad_value(169)), 0.5);}
        if (!s.b[1620]) {s.copy_ad(345, 343);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);}
        s.b[1867] = (p.p33 == 1.0);s.store_scalar(1867, if s.b[1867] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1867]) {s.store_scalar(457, 0.0);s.store_scalar(458, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(228, 169, 170, 0.5);s.store_offset_mul(172, 770, 228, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_179(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1867]) {s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 202, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(454, 652, 452, 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), 448, 1.0);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(229, 169, 170, 0.5);s.store_offset_mul(172, 770, 229, 1.0);s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(455, 652, 453, 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), 448, 1.0);}
        if ((!s.b[1620]) && (!s.b[1867])) {s.store_offset_mul(167, 770, 243, 1.0);s.store_mul_sub_rhs(168, 787, 274, 299);s.store_add_mixed_ai(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);s.store_scaled_add_mixed_ia(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);s.copy_ad(455, 453);s.copy_ad(454, 452);s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        s.b[1868] = (p.p33 == 2.0);s.store_scalar(1868, if s.b[1868] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1867])) && s.b[1868]) {s.store_mul_add_mixed_iai(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), 453);s.store_scalar(455, 0.0);s.store_scalar(454, 0.0);s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        if (!s.b[1620]) {s.store_add_div_rhs_mixed_ia(167, 330, 333, A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_sub(416, 400, 320);s.store_mul3_lhs(168, 167, 416, 416);s.store_offset(169, 168, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt_square_offset(s.ad_value(169), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_180(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(334), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(167, 400, 320);s.store_sub(168, 400, 320);s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);s.store_mul3_lhs(170, 832, 169, 169);s.store_offset(834, 170, 1.0);s.store_div_mixed_ia(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_limited_exp_neg_input(853, 176);s.store_mul3_lhs(340, 339, 343, 458);s.store_div(337, 740, 340);s.store_mul_ad_product_lhs_mixed_ai(380, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(253), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363), 1.0, s.ad_value(334), 1.0), 834, 853);s.store_scale(380, 380, p.p26);s.store_scalar(467, 0.0);}
        s.b[1869] = (p.p7 > 1.0);s.store_scalar(1869, if s.b[1869] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1869]) {s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scale(176, 271, p.p1009);s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));}
        s.b[1870] = (p.p7 == 2.0);s.store_scalar(1870, if s.b[1870] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {s.store_primal_div_from_scalar(466, 1.0, 465);}
        s.b[1871] = (s.v[466] < p.p1347);s.store_scalar(1871, if s.b[1871] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1869]) && s.b[1870]) && s.b[1871]) {s.store_scalar(466, p.p1347);s.store_primal_div_from_scalar(465, 1.0, 466);}
        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {s.store_add(178, 465, 467);s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);}
        if (!s.b[1620]) {s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));s.store_primal_scale(545, 543, p.p74);s.store_primal_scale(546, 544, p.p74);s.store_mul(593, 637, 590);s.store_div(167, 498, 593);s.store_limited_exp(595, 167);s.store_mul(594, 637, 590);s.store_div(167, 499, 594);s.store_limited_exp(596, 167);s.store_mul_scale_offset_mixed_ai(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, 1.0, (-1.0));}
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
        if ((!s.b[1620]) && (!s.b[1874])) {s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 498, 562);s.store_limited_exp(177, 167);}
        s.b[1875] = ((s.v[558] - s.v[498]) < 0.001);s.store_scalar(1875, if s.b[1875] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1874])) && s.b[1875]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (((!s.b[1620]) && (!s.b[1874])) && (!s.b[1875])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((!s.b[1620]) && (!s.b[1874])) {s.store_mul(170, 545, 554);}
        s.b[1876] = (s.v[553] == 0.0);s.store_scalar(1876, if s.b[1876] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1876])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(555, 553, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 499, 562);s.store_limited_exp(177, 167);}
        s.b[1877] = ((s.v[559] - s.v[499]) < 0.001);s.store_scalar(1877, if s.b[1877] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1876])) && s.b[1877]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (((!s.b[1620]) && (!s.b[1876])) && (!s.b[1877])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((!s.b[1620]) && (!s.b[1876])) {s.store_mul(170, 546, 555);}
        if (!s.b[1620]) {s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));}
        s.b[1878] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));s.store_scalar(1878, if s.b[1878] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(585, 587, 167);s.store_mul(578, 598, 167);s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(586, 588, 167);s.store_mul(577, 597, 167);s.store_mul_scale_offset_indices(583, 585, 595, 1.0, (-1.0));}
        s.b[1879] = (s.v[583] < 1e-5);s.store_scalar(1879, if s.b[1879] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1879]) {s.store_scalar(583, 0.0);s.store_scalar(591, 1.0);}
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1879])) {s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));}
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_mul_scale_offset_indices(584, 586, 596, 1.0, (-1.0));}
        s.b[1880] = (s.v[584] < 1e-5);s.store_scalar(1880, if s.b[1880] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1880]) {s.store_scalar(584, 0.0);s.store_scalar(592, 1.0);}
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1880])) {s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));}
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));s.store_limited_exp(603, 167);s.store_sub_from_scalar(169, 1.0, 603);s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));s.store_pow_indices(599, 167, 600);s.store_mul3_lhs(604, 602, 578, 599);s.store_mul(168, 167, 604);s.store_mul3_lhs(604, 602, 577, 599);s.store_mul(168, 167, 604);s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);s.store_mul3_lhs(532, 602, 578, 531);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_182(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1878])) {s.store_mul_ad_product_lhs_mixed_ia(533, 532, A::offset(s.ad_value(595), (-1.0)), 591);s.store_mul3_lhs(532, 602, 577, 531);s.store_mul_ad_product_lhs_mixed_ia(534, 532, A::offset(s.ad_value(596), (-1.0)), 592);s.store_primal_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);}
        s.b[1881] = (s.v[580] < 1.0);s.store_scalar(1881, if s.b[1881] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1881]) {s.store_scalar(580, 1.0);}
        s.b[1882] = (p.p554 == 1.0);s.store_scalar(1882, if s.b[1882] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {s.store_offset_div_scaled_inputs2_indices(167, 498, 1.0, 499, 1.0, 580, 1.0, 1.0);s.store_add(168, 583, 584);s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);s.store_scaled_add(169, 167, 170, 0.5);s.store_mul(167, 603, 604);}
        s.b[1884] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && (!s.b[1884])) {s.store_mul_scale_offset_indices(174, 569, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(571, 567, 167);s.store_mul_scale_offset_indices(174, 570, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(572, 568, 167);s.store_scale(594, 573, p.p925);}
        s.b[1885] = ((s.v[575] - s.v[498]) < 0.001);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1885]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1885])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);}
        if ((!s.b[1620]) && (!s.b[1884])) {s.store_scale(594, 574, p.p925);}
        s.b[1886] = ((s.v[576] - s.v[499]) < 0.001);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1886]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1886])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);}
        s.b[1887] = (p.p36 == 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1887]) {s.store_scalar(167, (s.v[200] * p.p76));}
        s.b[1888] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1887]) && s.b[1888]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {s.store_div_scaled_inputs3_indices(168, 204, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
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
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1891] = (s.v[898] != 0.0);s.store_scalar(1891, if s.b[1891] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && s.b[1891]) {s.store_mul_square_lhs(170, 202, 202);s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && (!s.b[1891])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        if ((!s.b[1620]) && (!s.b[1887])) {s.store_scalar(167, (s.v[200] * p.p76));s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1892] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));s.store_scalar(1892, if s.b[1892] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1892]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {s.store_div_scaled_inputs3_indices(168, 207, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1893] = (s.v[903] != 0.0);s.store_scalar(1893, if s.b[1893] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && s.b[1893]) {s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(903), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && (!s.b[1893])) {s.store_scalar(172, 1.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        s.b[1894] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1894]) {s.store_scalar(173, 0.0);}
        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {s.store_div_scaled_inputs3_indices(168, 206, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
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
        s.b[1896] = (p.p44 == 0.0);s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });s.b[1897] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });s.b[1898] = (s.v[355] > (s.v[659] / 80.0));s.store_scalar(1898, if s.b[1898] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1896]) && (!s.b[1897])) && s.b[1898]) {s.store_div_scaled_inputs_indices(168, 659, -1.0, 355, 1.0);}
        s.b[1899] = (p.p44 == 1.0);s.store_scalar(1899, if s.b[1899] { 1.0 } else { 0.0 });s.b[1900] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p.p643) * p.p643)), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p.p644) * p.p644), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        s.b[1901] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1901, if s.b[1901] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p.p643) * p.p643)), 0.5), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_185(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p.p644) * p.p644), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));s.store_mul_scale_offset_rhs(378, 880, 639, p.p666, (((((-1.0)) * (p.p666))) + (1.0)));}
        s.b[1902] = (s.v[211] > 0.0);s.store_scalar(1902, if s.b[1902] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1902]) {s.store_sub(168, 378, 499);}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1902])) {s.store_sub(168, 378, 498);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_offset(169, 881, (-1.0));}
        s.b[1903] = (s.v[168] > 0.0);s.store_scalar(1903, if s.b[1903] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1903]) {s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1903])) {s.store_scalar(170, 0.0);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_limited_exp(171, 170);}
        if (!s.b[1620]) {s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);}
        s.b[1904] = ((p.p37 != 0.0) || (p.p38 != 0.0));s.store_scalar(1904, if s.b[1904] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1904]) {s.store_mul_add_scaled_inputs4_indices_rhs(469, 269, 213, 1.0, 254, (-1.0), 400, 1.0, 320, 1.0);s.store_sqrt_square_offset(168, 469, 0.0001);s.store_scaled_sub(471, 168, 469, 0.5);s.store_scaled_add(470, 469, 168, 0.5);}
        s.b[1905] = (p.p38 != 0.0);s.store_scalar(1905, if s.b[1905] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 469, 1.0 / (p.p671));}
        s.b[1906] = (p.p696 != 0.0);s.store_scalar(1906, if s.b[1906] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1906]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1906])) {s.store_scalar(167, 1.0);}
        s.b[1907] = (s.v[167] < 0.01);s.store_scalar(1907, if s.b[1907] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1907]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));s.store_scalar(169, (p.p701 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));}
        s.b[1908] = (p.p697 != 0.0);s.store_scalar(1908, if s.b[1908] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1908]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1908])) {s.store_scalar(167, 1.0);}
        s.b[1909] = (s.v[167] < 0.01);s.store_scalar(1909, if s.b[1909] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1909]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));s.store_scalar(169, (p.p699 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_offset_mul(478, 212, 269, p.p1383);}
        s.b[1910] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));s.store_scalar(1910, if s.b[1910] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));s.store_sub(167, 208, 478);s.store_sqrt_square_offset(168, 167, 0.0001);s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_186(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul(169, 208, 209);s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);s.store_mul(171, 889, 890);s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p.p76), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p.p76));s.store_limited_exp(173, 172);s.store_scaled_mul(178, 178, 492, p.p1380);}
        s.b[1911] = (p.p37 != 0.0);s.store_scalar(1911, if s.b[1911] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));s.store_offset_mul(169, 812, 470, 1.0);s.store_scaled_mul(170, 168, 169, s.v[488]);s.store_mul_product3_mixed_aiia(171, A::limited_exp(s.ad_value(170)), 253, 269, A::add(s.ad_value(400), s.ad_value(320)), 1.0);s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_scale(168, 472, s.v[823]);s.store_limited_exp_neg_input(482, 168);s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);s.store_offset_square(172, 168, 0.0002);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1913] = (p.p1295 == 1.0);s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1914] = (s.v[818] < 0.01);s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) && s.b[1914]) {s.store_scalar(818, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1913])) {s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 818, 228, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(229, 169, 0.0001);}
        s.b[1915] = (p.p1295 == 1.0);s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1916] = (s.v[821] < 0.01);s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) && s.b[1916]) {s.store_scalar(821, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1915])) {s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 821, 229, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);}
        if (!s.b[1620]) {s.store_mul(502, 666, 463);s.store_mul(505, 667, 494);s.store_scale(508, 671, (s.v[189] * p.p2));s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));}
        s.b[1917] = (p.p913 == 1.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1917]) {s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_187(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1917])) {s.store_primal_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));}
        if (!s.b[1620]) {s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));}
        s.b[1918] = (p.p915 == 1.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1918]) {s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1918])) {s.store_primal_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));}
        if (!s.b[1620]) {s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));}
        s.b[1919] = (p.p917 == 1.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1919]) {s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1919])) {s.store_primal_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));}
        s.b[1920] = (s.v[502] > 0.0);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1920]) {s.store_div(168, 498, 672);}
        s.b[1921] = (s.v[168] < 0.9);s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1920]) && s.b[1921]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1922] = (p.p913 != 1.0);s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });s.b[1923] = (p.p913 == 0.5);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && s.b[1923]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p913));}
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p913)), 0.0);}
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && (!s.b[1922])) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1920]) && (!s.b[1921])) {s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p913), (((((-1.0)) * ((5.0 * p.p913)))) + ((1.0 + p.p913)))));s.store_mul_ad_product_rhs_mixed_ia(521, 672, 502, A::add(s.ad_value(169), s.ad_value(504)));}
        if ((!s.b[1620]) && (!s.b[1920])) {s.store_scalar(521, 0.0);}
        s.b[1924] = (s.v[505] > 0.0);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1924]) {s.store_div(168, 498, 673);}
        s.b[1925] = (s.v[168] < 0.9);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1924]) && s.b[1925]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1926] = (p.p915 != 1.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });s.b[1927] = (p.p915 == 0.5);s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && s.b[1927]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p915));}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p915)), 0.0);}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && (!s.b[1926])) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1924]) && (!s.b[1925])) {s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p915), (((((-1.0)) * ((5.0 * p.p915)))) + ((1.0 + p.p915)))));s.store_mul_ad_product_rhs_mixed_ia(522, 673, 505, A::add(s.ad_value(169), s.ad_value(507)));}
        if ((!s.b[1620]) && (!s.b[1924])) {s.store_scalar(522, 0.0);}
        s.b[1928] = (s.v[508] > 0.0);s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1928]) {s.store_div(168, 498, 674);}
        s.b[1929] = (s.v[168] < 0.9);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1928]) && s.b[1929]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1930] = (p.p917 != 1.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_188(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1931] = (p.p917 == 0.5);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && s.b[1931]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p917));}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p917)), 0.0);}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && (!s.b[1930])) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1928]) && (!s.b[1929])) {s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p917), (((((-1.0)) * ((5.0 * p.p917)))) + ((1.0 + p.p917)))));s.store_mul_ad_product_rhs_mixed_ia(523, 674, 508, A::add(s.ad_value(169), s.ad_value(510)));}
        if ((!s.b[1620]) && (!s.b[1928])) {s.store_scalar(523, 0.0);}
        if (!s.b[1620]) {s.store_scale(524, 533, (p.p919 * p.p2));s.store_add_scaled_inputs4_indices(520, 521, 1.0, 522, 1.0, 523, 1.0, 524, 1.0);s.store_mul(511, 669, 464);s.store_mul(514, 670, 495);s.store_scale(517, 668, (s.v[189] * p.p2));s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));}
        s.b[1932] = (p.p914 == 1.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1932]) {s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1932])) {s.store_primal_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));}
        if (!s.b[1620]) {s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));}
        s.b[1933] = (p.p916 == 1.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1933]) {s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1933])) {s.store_primal_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));}
        if (!s.b[1620]) {s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));}
        s.b[1934] = (p.p918 == 1.0);s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1934]) {s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1934])) {s.store_primal_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));}
        s.b[1935] = (s.v[511] > 0.0);s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1935]) {s.store_div(168, 499, 675);}
        s.b[1936] = (s.v[168] < 0.9);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1935]) && s.b[1936]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1937] = (p.p914 != 1.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });s.b[1938] = (p.p914 == 0.5);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && s.b[1938]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p914));}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && (!s.b[1937])) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1935]) && (!s.b[1936])) {s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));s.store_mul_ad_product_rhs_mixed_ia(526, 675, 511, A::add(s.ad_value(169), s.ad_value(513)));}
        if ((!s.b[1620]) && (!s.b[1935])) {s.store_scalar(526, 0.0);}
        s.b[1939] = (s.v[514] > 0.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1939]) {s.store_div(168, 499, 676);}
        s.b[1940] = (s.v[168] < 0.9);s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1939]) && s.b[1940]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1941] = (p.p916 != 1.0);s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_189(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1942] = (p.p916 == 0.5);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && s.b[1942]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p916));}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && (!s.b[1941])) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1939]) && (!s.b[1940])) {s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));s.store_mul_ad_product_rhs_mixed_ia(527, 676, 514, A::add(s.ad_value(169), s.ad_value(516)));}
        if ((!s.b[1620]) && (!s.b[1939])) {s.store_scalar(527, 0.0);}
        s.b[1943] = (s.v[517] > 0.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1943]) {s.store_div(168, 499, 677);}
        s.b[1944] = (s.v[168] < 0.9);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1943]) && s.b[1944]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1945] = (p.p918 != 1.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });s.b[1946] = (p.p918 == 0.5);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && s.b[1946]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p918));}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && (!s.b[1945])) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));s.store_mul_ad_product_rhs_mixed_ia(528, 677, 517, A::add(s.ad_value(169), s.ad_value(519)));}
        if ((!s.b[1620]) && (!s.b[1943])) {s.store_scalar(528, 0.0);}
        if (!s.b[1620]) {s.store_scale(529, 534, (p.p919 * p.p2));s.store_add_scaled_inputs4_indices(525, 526, 1.0, 527, 1.0, 528, 1.0, 529, 1.0);}
        s.b[1947] = (p.p28 != 0.0);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1947]) {s.store_powf_scaled_input(168, 706, 1.0000000000000001e-23, p.p1144);s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p.p1145);s.store_div_scaled_product_mixed_iai(170, 379, A::voltage(ctx, nodes, Some(10), Some(7)), p.p1143, 271, 1.0);}
        if (!s.b[1620]) {s.store_div_scaled_inputs_indices(607, 746, 2.0, 337, 1.0);}
        s.b[1948] = (p.p1011 <= 0.0);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1948]) {s.store_scalar(610, 0.0);}
        if ((!s.b[1620]) && (!s.b[1948])) {s.store_div_scaled_offset_numerator_mixed_ai(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, 607, 1.0);s.store_mul_ln_mixed_ia(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));}
        s.b[1949] = (s.v[610] < 0.0);s.store_scalar(1949, if s.b[1949] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1948])) && s.b[1949]) {s.store_scalar(610, 0.0);}
        if (!s.b[1620]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), 709, 1.0 / (1.602176462e-19));s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(320), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);}
    }
}
