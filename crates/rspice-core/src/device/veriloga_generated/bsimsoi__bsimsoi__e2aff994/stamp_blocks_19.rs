#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1717] = (p.p914 == 1.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1717]) {s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1717])) {s.store_primal_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));}
        if s.b[1620] {s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));}
        s.b[1718] = (p.p916 == 1.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1718]) {s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1718])) {s.store_primal_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));}
        if s.b[1620] {s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));}
        s.b[1719] = (p.p918 == 1.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1719]) {s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1719])) {s.store_primal_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));}
        s.b[1720] = (s.v[511] > 0.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1720]) {s.store_div(168, 499, 675);}
        s.b[1721] = (s.v[168] < 0.9);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1720]) && s.b[1721]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1722] = (p.p914 != 1.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });s.b[1723] = (p.p914 == 0.5);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && s.b[1723]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) && (!s.b[1723])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p914));}
        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && s.b[1722]) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);}
        if (((s.b[1620] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1720]) && (!s.b[1721])) {s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));s.store_mul_ad_product_rhs_mixed_ia(526, 675, 511, A::add(s.ad_value(169), s.ad_value(513)));}
        if (s.b[1620] && (!s.b[1720])) {s.store_scalar(526, 0.0);}
        s.b[1724] = (s.v[514] > 0.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1724]) {s.store_div(168, 499, 676);}
        s.b[1725] = (s.v[168] < 0.9);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1724]) && s.b[1725]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1726] = (p.p916 != 1.0);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });s.b[1727] = (p.p916 == 0.5);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) && (!s.b[1727])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p916));}
        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && s.b[1726]) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);}
        if (((s.b[1620] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1724]) && (!s.b[1725])) {s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));s.store_mul_ad_product_rhs_mixed_ia(527, 676, 514, A::add(s.ad_value(169), s.ad_value(516)));}
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
        s.b[1730] = (p.p918 != 1.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });s.b[1731] = (p.p918 == 0.5);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && s.b[1731]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) && (!s.b[1731])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p918));}
        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && s.b[1730]) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);}
        if (((s.b[1620] && s.b[1728]) && s.b[1729]) && (!s.b[1730])) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1728]) && (!s.b[1729])) {s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));s.store_mul_ad_product_rhs_mixed_ia(528, 677, 517, A::add(s.ad_value(169), s.ad_value(519)));}
        if (s.b[1620] && (!s.b[1728])) {s.store_scalar(528, 0.0);}
        if s.b[1620] {s.store_scale(529, 534, (p.p919 * p.p2));s.store_add_scaled_inputs4_indices(525, 526, 1.0, 527, 1.0, 528, 1.0, 529, 1.0);}
        s.b[1732] = (s.v[22] <= 0.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1732]) {s.copy_ad(1078, 52);s.store_scalar(1077, 0.0);s.copy_ad(1075, 1078);s.store_scalar(1076, 0.0);}
        if (s.b[1620] && (!s.b[1732])) {s.store_scaled_div(26, 250, 84, 0.5);s.store_square(27, 26);s.store_mul_scale_offset_mixed_ai(366, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(74), s.ad_value(250), (-0.5)), 354, -1.0, 1.0);s.store_add_product3_rhs_mixed_iia(1078, 52, 87, 250, A::add(A::offset(A::mul_scaled_output(s.ad_value(26), s.ad_value(354), 0.3333333333333333), (-1.0)), s.ad_value(354)), 0.5);s.store_scaled_mul(54, 74, 250, 0.16666666666666666);s.store_add_scaled_product_mixed_iia(25, 366, 1.0, 354, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), s.ad_value(26), 1.0), 1.0);s.store_add_scaled_products_mixed_aaia(1077, A::square(s.ad_value(354)), A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(54), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), (-1.0)), 0.5, 366, A::offset(s.ad_value(354), 1.0), 0.5);s.store_sub(1075, 1078, 25);s.store_add_scaled_inputs3_indices(1076, 1078, 1.0, 1075, (-1.0), 1077, -1.0);}
        if s.b[1620] {s.store_scaled_add_mixed_ia(246, 1075, A::sqrt_square_offset(s.ad_value(1075), ((0.25 * 0.1) * 0.1)), 0.5);s.store_add(245, 1076, 1077);s.store_add_scaled_inputs(167, 245, 1.0 / (p.p230), 246, (p.p231 * 1.0 / (p.p230)));s.store_scaled_add_mixed_ia(167, 167, A::sqrt_square_offset(s.ad_value(167), ((4.0 * 0.001) * 0.001)), 0.5);s.store_offset_powf_ad(168, s.ad_value(167), (0.7 * p.p229), 1.0);s.store_div_from_scalar(427, (p.p228 * 1.9e-9), 168);s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add_scaled_inputs(s.ad_value(429), (3.9 * 1.0 / (p.p110)), s.ad_value(427), 1.0 / (s.v[200])));s.store_mul_scale_offset_mixed_ia(387, 1075, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), (-(((p.p2 * s.v[187]) * s.v[188]) + p.p1379)), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1620] {s.store_scale(391, 428, (((p.p2 * s.v[187]) * s.v[188]) + p.p1379));}
        s.b[1733] = (s.v[211] > 0.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1733]) {s.store_mul_scale_offset_indices(388, 1076, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(389, 1077, 391, -1.0, 0.0);}
        if (s.b[1620] && (!s.b[1733])) {s.store_mul_scale_offset_indices(388, 1077, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(389, 1076, 391, -1.0, 0.0);}
        if s.b[1620] {s.store_add_scaled_inputs3_indices(390, 387, (-1.0), 388, (-1.0), 389, (-1.0));}
        s.b[1734] = (!param_given[867]);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1734]) {s.store_scalar(788, ((((2.0 * p.p110) * 8.8541878128e-12) / 3.141592653589793) * ((((p.p871 * (1.0 + (4e-7 / p.p76)))).max(1e-38)) as f64).ln()));}
        if s.b[1620] {s.store_primal_offset(425, 788, p.p872);s.store_primal_offset(426, 788, p.p873);s.store_scalar(561, ((s.v[187] / p.p1373) + p.p1378));s.store_scalar(560, ((s.v[187] / p.p1373) + p.p1377));}
        s.b[1735] = (p.p32 == 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1735]) {s.store_mul3_affine_lhs(423, 561, 425, (-p.p2), 0.0, 431);s.store_mul3_affine_lhs(424, 560, 426, (-p.p2), 0.0, 430);}
        if (s.b[1620] && (!s.b[1735])) {s.store_sqrt_offset_ad(167, A::square(A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(419, 431, 0.5, 219, ((-1.0) * 0.5), 167, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(173, 419, A::powf(A::offset(A::powf(A::scale(s.ad_value(419), (-1.0 / (p.p893))), p.p894), 1.0), (1.0 / p.p894)));s.store_sqrt_sub_from_scalar_ad(168, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(791), 1.0));s.store_mul_add_scaled_products_rhs_mixed_iiia(423, 561, 425, 431, (-p.p2), 789, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(431), 1.0, s.ad_value(219), (-1.0), s.ad_value(419), -1.0), 1.0, s.ad_value(791), s.ad_value(168), (-1.0), (-0.5)), (-p.p2));s.store_sqrt_offset_ad(167, A::square(A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(420, 430, 0.5, 219, ((-1.0) * 0.5), 167, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(173, 420, A::powf(A::offset(A::powf(A::scale(s.ad_value(420), (-1.0 / (p.p891))), p.p892), 1.0), (1.0 / p.p892)));s.store_sqrt_sub_from_scalar_ad(169, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(792), 1.0));s.store_mul_add_scaled_products_rhs_mixed_iiia(424, 560, 426, 430, (-p.p2), 790, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(219), (-1.0), s.ad_value(420), -1.0), 1.0, s.ad_value(792), s.ad_value(169), (-1.0), (-0.5)), (-p.p2));}
        if s.b[1620] {s.store_mul_scaled_voltage(421, 379, (((-p.p2) * s.v[188]) * p.p874), ctx, nodes, Some(9), Some(10));s.store_add_scaled_inputs3_indices(422, 423, (-1.0), 424, (-1.0), 421, (-1.0));s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p.p1394));s.store_primal_offset(1036, 1035, (2.0 * p.p1393));}
        s.b[1736] = (s.v[908] > 0.0);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1736]) {s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);}
        if (s.b[1620] && (!s.b[1736])) {s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product_by_product(s.ad_value(706), s.ad_value(908), -1.0, s.ad_value(182), s.ad_value(182), 1.0), 1e-38));s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);}
        if s.b[1620] {s.store_sub(1032, 235, 215);s.store_scalar(1034, (3.453133e-11 / p.p75));s.store_primal_mul_ad_affine_product_rhs(1037, 909, s.ad_value(1034), A::scale_offset(s.ad_value(1036), ((s.v[187] / p.p1373) * p.p2), p.p1382), p.p1388, 0.0);s.store_mul_sub_rhs(1038, 1037, 1032, 1033);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1620] {s.copy_ad(1039, 1038);}
        s.b[1737] = (p.p47 != 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1737]) {s.store_scalar(167, (p.p1395 * ((((p.p871 * (1.0 + (p.p74 / p.p75)))).max(1e-38)) as f64).ln()));s.store_scalar(168, (p.p19 - p.p1));}
        s.b[1738] = (s.v[168] > 0.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1738]) {s.store_mul(1040, 167, 168);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1738])) {s.store_scalar(1040, 0.0);}
        if (s.b[1620] && s.b[1737]) {s.store_scalar(168, (p.p20 - p.p1));}
        s.b[1739] = (s.v[168] > 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1739]) {s.store_mul(1041, 167, 168);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1739])) {s.store_scalar(1041, 0.0);}
        if (s.b[1620] && s.b[1737]) {s.store_primal_scale(1042, 1034, p.p17);s.store_scalar(1043, (p.p1396 * p.p17));s.store_primal_scale(1044, 1034, p.p18);s.store_scalar(1045, (p.p1396 * p.p18));s.store_mul_scale_offset_indices(177, 236, 379, -1.0, 0.0);s.store_mul_scale_offset_indices(178, 237, 379, -1.0, 0.0);}
        s.b[1740] = (p.p1396 != 0.0);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1737]) && s.b[1740]) {s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p.p1399)));s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(178), (-p.p1399), p.p1400)), 1e-38));s.store_mul_scale_offset_mixed_ia(170, 178, A::add(s.ad_value(1044), s.ad_value(1045)), 0.5, 0.0);s.store_add_scaled_product_indices(1047, 170, 1.0, 168, 169, 1.0);s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p.p1397)));s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(177), (-p.p1397), p.p1398)), 1e-38));s.store_mul_scale_offset_mixed_ia(170, 177, A::add(s.ad_value(1042), s.ad_value(1043)), 0.5, 0.0);s.store_add_scaled_product_indices(1046, 170, 1.0, 168, 169, 1.0);}
        if ((s.b[1620] && s.b[1737]) && (!s.b[1740])) {s.store_mul(1046, 1042, 177);s.store_mul(1047, 1044, 178);}
        if (s.b[1620] && s.b[1737]) {s.store_add_scaled_product_indices(1046, 1046, 1.0, 1040, 177, 1.0);s.store_add_scaled_product_indices(1047, 1047, 1.0, 1041, 178, 1.0);}
        if (s.b[1620] && (!s.b[1737])) {s.store_scalar(1046, 0.0);s.store_scalar(1047, 0.0);}
        s.b[1741] = (p.p45 == 1.0);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1741]) {s.store_scalar(795, (p.p140 + p.p25));s.store_mul(231, 230, 272);s.store_mul(233, 232, 272);s.store_mul(212, 795, 272);s.store_mul(240, 239, 272);s.store_sub(434, 231, 212);s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141), 1.0 / (s.v[199]));s.copy_ad(294, 436);s.copy_ad(214, 434);s.store_mul(215, 708, 272);s.store_sub(216, 240, 215);s.store_div_from_scalar(295, 1.0, 294);s.store_square(296, 294);s.store_div_from_scalar(297, 1.0, 296);s.copy_ad(251, 435);s.store_scalar(706, p.p141);s.store_div(124, 294, 2);s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);s.store_scale(126, 125, 1e-7);s.store_scalar(127, (5.0 / 4.0));s.store_div_from_scalar(128, 1.0, 124);s.store_square(129, 124);s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));}
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
        if (s.b[1620] && s.b[1741]) {s.store_mul(46, 46, 271);s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);s.store_div_from_scalar(96, 1.0, 95);s.store_add_mixed_ai(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 233);s.store_limited_exp_neg_input(99, 97);s.store_scale(101, 95, 0.001);s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(271)), 1.0, 3, 216, (-1.0));s.store_add_scaled_product_mixed_iia(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);}
        s.b[1745] = (s.v[4] < s.v[97]);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });s.b[1746] = (s.v[214] < s.v[104]);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });s.b[1747] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && s.b[1747]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && s.b[1747]) {s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1748] = (s.v[214] < (-s.v[101]));s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 99, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 97, 3.0);s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) {s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 97);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {s.copy_ad(47, 2);s.store_primal_square(48, 47);s.store_add_scaled_product_indices(8, 4, 1.0, 46, 272, (-1.0));s.store_add_scaled_product_mixed_iia(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));s.store_offset(43, 97, 3.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));s.store_add_scaled_inputs_product_mixed_iiia(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));s.store_square(109, 108);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {s.store_primal_sub_from_scalar(110, 1.0, 48);}
        s.b[1749] = (s.v[107] < 0.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) && s.b[1749]) {s.store_scalar(107, 0.0);}
        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {s.store_add_scaled_inputs3_mixed_iia(49, 97, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);s.store_add(111, 107, 108);s.store_square(112, 111);s.store_add_scaled_inputs_product_mixed_aiii(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));s.store_div_scaled_product_mixed_iia(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);s.store_add(117, 106, 116);s.store_limited_exp_sub(118, 117, 97);s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));s.store_mul_add_scaled_sub_value_product_rhs_mixed_aii(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, 296, 118, (((-1.0)) * (2.0)));s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);s.store_add(9, 117, 122);}
        s.b[1750] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1741]) && (!s.b[1745])) && s.b[1750]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1751] = (s.v[214] < (-s.v[101]));s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && s.b[1751]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && s.b[1751]) {s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 99, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && (!s.b[1751])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 97, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && (!s.b[1751])) {s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 97);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && (!s.b[1751])) {s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (s.b[1620] && s.b[1741]) {s.copy_ad(123, 9);s.store_scalar(102, 1e-7);s.store_scalar(103, 2.0);s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);s.store_add_scaled_value_products_mixed_iaiai(6, 24, 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), 727, (-1.0), A::offset(s.ad_value(3), 1.0), 46, 1.0);}
        s.b[1752] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1752]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
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
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
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
                                }
                            }
                        });
        }
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 97);s.store_limited_exp_sub(177, 123, 97);s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && s.b[1741]) {s.copy_ad(123, 22);}
        s.b[1753] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1753]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
}
