#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_sub_scaled_inputs_mixed_ia(123, 123, 0.5, A::sqrt_square_offset(s.ad_value(123), ((0.25 * 0.005) * 0.005)), 0.5, (0.25 * 0.005));s.store_mul_scale_offset(124, A::add_scaled_product(A::offset(s.ad_value(454), (p.p869 / s.v[30])), 1.0, s.ad_value(455), s.ad_value(61), 1.0), A::powf(s.ad_value(395), p.p868), 1.0, (-1.0));s.b[1356] = (s.v[116] > 0.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if s.b[1356] {s.store_mul_scale_offset_indices(12, 76, 117, -1.0, 0.0);}
        s.b[1357] = (s.v[12] < (-80.0));s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if (s.b[1356] && s.b[1357]) {s.store_scalar(14, 1.804851387e-35);}
        if (s.b[1356] && (!s.b[1357])) {s.store_limited_exp(14, 12);}
        if s.b[1356] {s.store_offset_mul_offset_rhs(15, 116, 14, 1.0, s.v[30]);s.store_mul_scaled_ln_ad_rhs(115, 106, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[30], s.ad_value(15)), 1e-38));}
        if (!s.b[1356]) {s.store_scalar(115, 0.0);}
        s.store_add_div_rhs_mixed_ia(16, 121, 118, A::pow_from_scalar(s.v[30], s.ad_value(119)));s.store_add_scaled_product_mixed_iia(115, 115, 1.0, 16, A::tanh(A::mul(s.ad_value(120), s.ad_value(76))), (-1.0));s.store_offset(482, 482, p.p35);s.store_mul(65, 64, 107);s.store_mul(73, 70, 107);s.store_mul(58, 482, 107);s.store_add_scaled_products_mixed_iaii(122, 495, A::sub(s.ad_value(111), s.ad_value(128)), 1.0, 494, 61, (-1.0));s.store_add_mixed_ai(79, A::add(A::add_scaled_inputs4(s.ad_value(123), 1.0, s.ad_value(115), 1.0, s.ad_value(122), 1.0, s.ad_value(124), -1.0), s.ad_value(659)), 663);s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));s.store_scaled_sqrt_mul_scaled_lhs(125, 481, ((2.0 * 1.60219e-19) * s.v[26]), 109, 1.0 / (s.v[46]));s.store_scalar(710, 0.5);
        if (!(((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_sqrt_square_offset_ad(12, A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), ((4.0 * 0.001) * 0.001), 0.5);
        } else {
            if (((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(12, ((-0.001) * 0.001), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0));
            } else {
                s.store_scalar(12, 0.0);
            }
        }
        s.store_offset_div_scaled_inputs_sqrt_rhs(90, 125, 1.0, 12, 2.0, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
    ) {
        if (!((((((s.v[70] * s.v[109]) + (2.0 * s.v[88])) + (((s.v[710]).max(1e-38)) as f64).ln()) + (2.0 * s.v[710])) + ((((((2.0 * s.v[90]) / s.v[125]) * ((((2.0 * s.v[710]) * s.v[90]) / s.v[125]) + (2.0 * ((s.v[12]) as f64).sqrt())))).max(1e-38)) as f64).ln()) < ((-10000.0) * 0.001))) {
            let t0: A = A::sqrt(s.ad_value(12));let t1: A = A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, t0, 2.0)), 1e-38));s.store_add_scaled_inputs3_offset(711, A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), 0.5, t1, 0.5, A::sqrt_square_offset(A::add(A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), t1), ((4.0 * 0.001) * 0.001)), 0.5, ((((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710]))) * 0.5));
        } else {
            if ((((((s.v[70] * s.v[109]) + (2.0 * s.v[88])) + (((s.v[710]).max(1e-38)) as f64).ln()) + (2.0 * s.v[710])) + ((((((2.0 * s.v[90]) / s.v[125]) * ((((2.0 * s.v[710]) * s.v[90]) / s.v[125]) + (2.0 * ((s.v[12]) as f64).sqrt())))).max(1e-38)) as f64).ln()) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_add_ad(711, ((-0.001) * 0.001), A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, A::sqrt(s.ad_value(12)), 2.0)), 1e-38)));
            } else {
                s.store_scalar(711, 0.0);
            }
        }
        s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aai(857, 187, A::add_scaled_product(s.ad_value(482), 1.0, A::add_scaled_product(s.ad_value(711), 1.0, s.ad_value(70), s.ad_value(109), (-1.0)), s.ad_value(108), 1.0), 1.0, A::mul3(s.ad_value(108), s.ad_value(125), A::sqrt(s.ad_value(711))), 1.0, 79, 1.0, 0.0);s.store_scaled_sqrt_mul_scaled_lhs(125, 481, ((2.0 * 1.60219e-19) * s.v[26]), 107, 1.0 / (s.v[46]));s.store_div_from_scalar(126, 1.0, 125);s.store_div(89, 88, 104);s.store_scalar(13, 1.0);s.store_scale(204, 59, 1.0 / (s.v[13]));s.store_scale(205, 125, 1.0 / (s.v[13]));s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));s.b[1358] = (s.v[204] < 0.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if s.b[1358] {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (!s.b[1358]) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if (!s.b[1358]) {s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 125, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 89, (-2.0), 73, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);s.b[1359] = (s.v[20] <= (-68.0));s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if s.b[1359] {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1360] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (s.b[1359] && s.b[1360]) {s.store_limited_exp(15, 16);}
        s.b[1361] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if ((s.b[1359] && (!s.b[1360])) && s.b[1361]) {s.store_limited_exp(15, 20);}
        if ((s.b[1359] && (!s.b[1360])) && (!s.b[1361])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if s.b[1359] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(200, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (!s.b[1359]) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1359]) {s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1362] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if s.b[1362] {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (!s.b[1362]) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        s.store_sqrt(96, 93);s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);s.b[1363] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if s.b[1363] {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (!s.b[1363]) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);s.store_scalar(155, (1e-8 / (s.v[47] * p.p77)));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(12, 106, A::add_scaled_inputs_product(s.ad_value(59), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));s.b[1364] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (!s.b[1364]) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        s.store_mul3_affine_lhs(130, 90, 106, 2.0, 0.0, 200);s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(130), s.ad_value(84)), 1.0, 0.5), s.ad_value(513));s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(132), s.ad_value(407)), 1.0);s.store_offset(16, 15, 1.0);s.b[1365] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if s.b[1365] {s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);}
        if (!s.b[1365]) {s.store_scaled_add_offset_sqrt_square_offset(133, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        s.store_primal_div_from_scalar_scaled_ad(235, 1.0, A::pow_from_scalar((s.v[29] * 1000000.0), s.ad_value(527)), p.p2);s.b[1366] = (p.p42 == 1.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(243, 0.0);}
        if (!s.b[1366]) {s.store_offset_mul(12, 526, 130, 1.0);s.store_mul_sub_rhs(13, 543, 111, 128);s.store_add_mixed_ai(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);s.store_add_mixed_ia(15, 14, A::sqrt_square_offset(s.ad_value(14), 0.01));}
        s.b[1367] = (p.p42 == 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1367]) {s.store_mul_ad_affine_product_lhs(243, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2, 0.0, 408);}
        if ((!s.b[1366]) && (!s.b[1367])) {s.store_mul_add_mixed_iai(243, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), 240);}
        s.store_pow_ad(12, s.ad_value(133), A::div_from_scalar(1.0, s.ad_value(166)));s.store_mul(23, 453, 61);s.store_sqrt_square_offset(24, 23, 0.1);s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_product_offset_denominator_mixed_iia(14, 200, 13, (10.0 * p.p433), A::mul(s.ad_value(200), s.ad_value(13)), (10.0 * p.p433), 1.0);s.b[1368] = (s.v[536] < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if s.b[1368] {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);}
        if (!s.b[1368]) {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);}
        s.b[1369] = (s.v[243] > 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if s.b[1369] {s.store_mul3_affine_lhs(23, 90, 106, ((s.v[29] * 2.0) * s.v[46]), 0.0, 502);s.store_div_scaled_product3_indices(24, 23, 138, 243, 1.0, 106, 2.0);s.store_div_scaled_product_offset_denominator_mixed_iaa(12, 138, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1370] = (s.v[13] != 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1370]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if (s.b[1369] && (!s.b[1370])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if s.b[1369] {s.store_add_scaled_value_products_mixed_aiiia(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1371] = (s.v[13] != 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1371]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if (s.b[1369] && (!s.b[1371])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if s.b[1369] {s.store_add_scaled_value_products3_mixed_iiiiaia(18, 15, 1.0, 12, 17, 1.0, 24, A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(12, 12, 16, 18);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1372] = (s.v[13] != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1372]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if (s.b[1369] && (!s.b[1372])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
    ) {
        if s.b[1369] {s.store_add_scaled_value_products_mixed_aiiia(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1373] = (s.v[13] != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1373]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if (s.b[1369] && (!s.b[1373])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if s.b[1369] {s.store_add_scaled_value_products3_mixed_iiiiaia(18, 15, 1.0, 12, 17, 1.0, 24, A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(131, 12, 16, 18);}
        if (!s.b[1369]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(12, 138, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1374] = (s.v[13] != 0.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1374]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if ((!s.b[1369]) && (!s.b[1374])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if (!s.b[1369]) {s.store_add_scaled_products_mixed_iiia(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1375] = (s.v[13] != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1375]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if ((!s.b[1369]) && (!s.b[1375])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if (!s.b[1369]) {s.store_add_scaled_value_products_mixed_iiiia(18, 15, 1.0, 12, 17, 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(12, 12, 16, 18);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1376] = (s.v[13] != 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1376]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if ((!s.b[1369]) && (!s.b[1376])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if (!s.b[1369]) {s.store_add_scaled_products_mixed_iiia(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1377] = (s.v[13] != 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1369]) && s.b[1377]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if ((!s.b[1369]) && (!s.b[1377])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if (!s.b[1369]) {s.store_add_scaled_value_products_mixed_iiiia(18, 15, 1.0, 12, 17, 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(131, 12, 16, 18);}
        s.store_add_scaled_inputs4_mixed_iiia(143, 91, 1.0, 89, (-2.0), 131, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(125), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(136, 143, 106);s.b[1378] = ((p.p1130 == 0.0) && (p.p1131 == 0.0));s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if s.b[1378] {s.store_scalar(782, 1.0);}
        if (!s.b[1378]) {s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);s.store_offset_div_scaled_inputs2_mixed_iaa(782, 13, p.p1130, A::mul3_scaled_output(s.ad_value(13), A::powf(s.ad_value(200), p.p1132), s.ad_value(106), p.p1131), (-1.0), A::scale_offset(s.ad_value(61), p.p1133, 1.0), 1.0, 1.0);}
        s.b[1379] = ((0.1 == 0.0) && (s.v[782] < ((-2500.0) * 0.0005)));s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((!s.b[1378]) && s.b[1379]) {s.store_div_from_scalar_scaled_input(782, ((-0.0005) * 0.0005), 782, 16.0);}
        if ((!s.b[1378]) && (!s.b[1379])) {s.store_scaled_add_offset_sqrt_square_offset(782, 782, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        s.b[1380] = ((0.0 == 0.0) && ((s.v[136] - s.v[70]) < ((-2500.0) * 0.001)));s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if s.b[1380] {s.store_div_from_scalar_ad(140, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(136), 16.0, s.ad_value(70), 16.0));}
        if (!s.b[1380]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(140, 136, 0.5, 70, ((-1.0) * 0.5), 136, 70, ((0.25 * 0.001) * 0.001), 0.5);}
        s.store_div(140, 140, 782);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(140)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 74, 20);s.store_mul_add_lhs(142, 139, 70, 107);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 125, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 89, (-2.0), 142, -1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
    ) {
        s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);s.b[1381] = (s.v[20] <= (-68.0));s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if s.b[1381] {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1382] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (s.b[1381] && s.b[1382]) {s.store_limited_exp(15, 16);}
        s.b[1383] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((s.b[1381] && (!s.b[1382])) && s.b[1383]) {s.store_limited_exp(15, 20);}
        if ((s.b[1381] && (!s.b[1382])) && (!s.b[1383])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if s.b[1381] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(144, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (!s.b[1381]) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if (!s.b[1381]) {s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 200, (-1.0), 144, -1.0, (-1.0));s.b[1384] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if s.b[1384] {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (!s.b[1384]) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        s.store_sqrt(14, 12);s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), s.ad_value(14)), 1.0);s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));s.store_div_from_scalar_add_ad(12, 1.0, A::offset(s.ad_value(200), 1.0), s.ad_value(144));s.store_mul(13, 217, 12);s.store_add_scaled_inputs_product_mixed_iiaa(189, 59, 1.0, 91, (-1.0), A::offset(s.ad_value(90), (-1.0)), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(13), 0.3333333333333333), (-1.0));s.store_scale(14, 90, 0.3333333333333333);s.store_mul(15, 13, 12);s.store_mul_mixed_ia(190, 14, A::add_scaled_inputs_product(s.ad_value(200), 2.0, s.ad_value(144), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 0.8, 1.0), 1.0, s.ad_value(144), 1.2), s.ad_value(15), 0.5));s.store_mul_mixed_ia(193, 14, A::add_scaled_inputs_product(s.ad_value(200), 1.0, s.ad_value(144), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 1.2, 1.0), 1.0, s.ad_value(144), 0.8), s.ad_value(15), 0.5));s.b[1385] = ((0.0 == 0.0) && ((s.v[106] * s.v[189]) < ((-2500.0) * 0.1)));s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if s.b[1385] {s.store_div_scalar_by_product_indices(81, ((-0.1) * 0.1), 106, 189, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1385]) {s.store_add_scaled_product_mixed_aii(81, A::sqrt_square_offset(A::mul(s.ad_value(106), s.ad_value(189)), ((0.25 * 0.1) * 0.1)), 0.5, 106, 189, 0.5);}
        s.store_mul_add_rhs(80, 106, 190, 193);s.store_add_scaled_inputs(156, 81, s.v[155], 80, (s.v[158] * s.v[155]));s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(80), s.ad_value(81)), 1.0, 0.5), s.ad_value(513));s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(156), s.ad_value(407)), 1.0);s.store_offset(16, 15, 1.0);s.b[1386] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if s.b[1386] {s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);}
        if (!s.b[1386]) {s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        s.store_div_scaled_inputs_mixed_ia(134, 502, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);s.store_scale(135, 134, s.v[30]);s.b[1387] = (s.v[537] > 0.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if s.b[1387] {s.store_offset_div_scaled_product_indices(172, 537, 80, 1.0, 135, 1.0, 1.0);}
        if (!s.b[1387]) {s.store_div_from_scalar_sub_from_scalar_ad(172, 1.0, 1.0, A::div_scaled_product(s.ad_value(537), s.ad_value(80), 1.0, s.ad_value(135), 1.0));}
        s.copy_ad(171, 519);s.store_sub(167, 74, 139);s.store_add_scaled_inputs(174, 80, 1.0, 106, 2.0);s.b[1388] = (s.v[171] > 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if s.b[1388] {s.store_div_add_scaled_inputs_rhs_indices(15, 174, 140, 1.0, 174, 1.0);}
        if s.b[1388] {
            if (!((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(16, A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_product(16, ((-0.001) * 0.001), 520, 61, 1.0);
                } else {
                    s.store_scalar(16, 0.0);
                }
            }
        }
        if s.b[1388] {s.store_div_from_scalar(17, 1.0, 16);s.store_mul_product3_mixed_iaii(173, 17, A::div(s.ad_value(174), s.ad_value(171)), 15, 172, 1.0);s.store_offset_div(175, 167, 173, 1.0);}
        if (!s.b[1388]) {s.store_scalar(175, 1.0);}
        s.b[1389] = (s.v[525] <= 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if s.b[1389] {s.store_scalar(105, 1.0);}
        if (!s.b[1389]) {s.store_div_scaled_inputs_indices(21, 525, ((s.v[30]) as f64).sqrt(), 174, 1.0);s.store_div_from_scalar_offset_input(105, 1.0, 21, 1.0);}
        s.store_add(170, 140, 135);s.b[1390] = (s.v[541] > 0.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (p.p350 < 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1390] && s.b[1391]) {s.store_div_scaled_value_by_product_mixed_iai(13, 541, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), 105, 1.0);}
        if (s.b[1390] && (!s.b[1391])) {s.store_div_scaled_product_offset_rhs_mixed_iai(13, 541, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, 105, 1.0);}
        if s.b[1390] {s.store_offset_mul_ad(176, s.ad_value(13), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(13), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);}
        s.b[1392] = (p.p350 < 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if ((!s.b[1390]) && s.b[1392]) {s.store_div_scaled_value_by_product_mixed_iai(13, 541, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), 105, 1.0);}
        if ((!s.b[1390]) && (!s.b[1392])) {s.store_div_scaled_product_offset_rhs_mixed_iai(13, 541, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, 105, 1.0);}
        if (!s.b[1390]) {s.store_offset(176, 13, 1.0);}
        s.store_mul(175, 175, 176);s.store_limited_exp_mul(13, 524, 74);s.b[1393] = (s.v[523] > 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if s.b[1393] {s.store_scalar(14, (1.0 + (p.p369 * s.v[30])));s.store_div_scaled_offset_numerator_mixed_ai(168, A::mul(s.ad_value(14), s.ad_value(13)), 1.0, 1.0, 523, 1.0);s.store_mul(168, 168, 105);}
        if (!s.b[1393]) {s.store_scalar(168, 5.540622384e34);}
        s.store_div(16, 167, 168);s.store_offset(12, 16, 1.0);s.store_mul(175, 175, 12);s.b[1394] = (s.v[522] > 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });s.b[1395] = (s.v[167] > ((s.v[521] * s.v[129]) / 80.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (s.b[1394] && s.b[1395]) {s.store_div_scaled_product_indices(12, 521, 129, 1.0, 167, 1.0);s.store_div_scaled_inputs_limited_exp_lhs(169, 12, s.v[30], 522, 1.0);}
        if (s.b[1394] && (!s.b[1395])) {s.store_div_from_scalar(169, (5.540622384e34 * s.v[30]), 522);}
        if (!s.b[1394]) {s.store_scalar(169, 5.540622384e34);}
        s.store_offset_div(177, 167, 169, 1.0);s.store_mul(175, 175, 177);s.store_pow_ad(12, s.ad_value(159), A::div_from_scalar(1.0, s.ad_value(166)));s.store_mul(23, 453, 61);s.store_sqrt_square_offset(24, 23, 0.1);s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);s.store_div_scaled_product_offset_denominator_mixed_iia(14, 80, 13, (10.0 * p.p433), A::mul(s.ad_value(80), s.ad_value(13)), (10.0 * p.p433), 1.0);s.b[1396] = (s.v[536] < 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if s.b[1396] {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);}
        if (!s.b[1396]) {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 144, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);s.b[1397] = (s.v[13] != 0.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_add_scaled_product_mixed_iaa(162, 14, 0.5, A::div_from_scalar(1.0, s.ad_value(13)), A::asinh(s.ad_value(13)), 0.5);}
        if (!s.b[1397]) {s.store_scaled_add_mixed_ia(162, 14, A::div_from_scalar(1.0, s.ad_value(14)), 0.5);}
        s.copy_ad(163, 162);s.store_scalar(241, 0.0);s.store_scalar(242, 0.0);s.b[1398] = (p.p42 == 1.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if s.b[1398] {s.store_scalar(244, 0.0);s.store_scalar(245, 1.0);s.store_mul_voltage_ad(71, s.ad_value(187), ctx, nodes, Some(8), Some(11));s.store_sub(53, 64, 71);s.store_sub(14, 53, 63);s.store_sqrt_square_offset(15, 14, 0.01);s.store_scaled_add(77, 14, 15, 0.5);s.store_offset_mul(17, 526, 77, 1.0);s.copy_ad(51, 71);s.store_add_scaled_product_mixed_aii(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 51, 1.0);s.store_scaled_add_mixed_ia(16, 18, A::sqrt_square_offset(s.ad_value(18), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(241, 408, 239, 1.0, A::add_scaled_product(s.ad_value(529), 1.0, s.ad_value(531), s.ad_value(16), 1.0), 235, 1.0);s.store_mul_voltage_ad(67, s.ad_value(187), ctx, nodes, Some(6), Some(11));s.store_sub(55, 64, 67);s.store_sub(14, 55, 63);s.store_sqrt_square_offset(15, 14, 0.01);s.store_scaled_add(78, 14, 15, 0.5);s.store_offset_mul(17, 526, 78, 1.0);s.copy_ad(49, 67);s.store_add_scaled_product_mixed_aii(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 49, 1.0);s.store_scaled_add_mixed_ia(16, 18, A::sqrt_square_offset(s.ad_value(18), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(242, 408, 240, 1.0, A::add_scaled_product(s.ad_value(528), 1.0, s.ad_value(530), s.ad_value(16), 1.0), 235, 1.0);}
        if (!s.b[1398]) {s.store_offset_mul(12, 526, 80, 1.0);s.store_mul_sub_rhs(13, 543, 111, 128);s.store_add_mixed_ai(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);s.store_scaled_add_mixed_ia(15, 14, A::sqrt_square_offset(s.ad_value(14), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(244, s.ad_value(408), A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), p.p2, 0.0, 235);s.copy_ad(242, 240);s.copy_ad(241, 239);s.store_offset_product3(245, A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30])), 1.0);}
        s.b[1399] = (p.p42 == 2.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1398]) && s.b[1399]) {s.store_mul_add_mixed_iai(244, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), 240);s.store_scalar(242, 0.0);s.store_scalar(241, 0.0);s.store_offset_product3(245, A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30])), 1.0);}
        s.store_add_div_rhs_mixed_ia(12, 150, 153, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(104), s.ad_value(393), 2.0));s.store_sub(216, 200, 144);s.store_mul3_lhs(13, 12, 216, 216);s.store_offset(14, 13, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(15, 14, 0.5, A::sqrt_square_offset(s.ad_value(14), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(154, A::sqrt(A::offset(s.ad_value(15), 1.0)), 1.0, 0.5);s.store_offset_sub_scaled_inputs(154, A::offset(s.ad_value(154), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(154), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(12, 200, 144);s.store_sub(13, 200, 144);s.store_div_add_scaled_inputs_rhs_indices(14, 13, 12, 1.0, 610, 1.0);s.store_mul3_lhs(15, 609, 14, 14);s.store_offset(611, 15, 1.0);s.store_div_mixed_ia(21, 633, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(636), A::mul3(s.ad_value(639), s.ad_value(13), s.ad_value(13)))), s.ad_value(12), 1.0, s.ad_value(104), s.ad_value(393), 2.0));s.store_limited_exp_neg_input(628, 21);s.store_mul3_lhs(160, 159, 162, 245);s.store_div(157, 499, 160);s.store_mul_ad_product_lhs_mixed_ai(188, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(200), s.ad_value(144)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)))), s.ad_value(175), 1.0, s.ad_value(154), 1.0), 611, 628);s.store_scale(188, 188, p.p36);s.b[1400] = ((p.p42 == 1.0) && (p.p1094 == 1.0));s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if s.b[1400] {s.store_mul_ln_mixed_ia(753, 108, A::div_scaled_inputs(s.ad_value(481), p.p1117, A::square(s.ad_value(28)), 1.0));}
        s.b[1401] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1401]) {s.store_mul_sqrt_mixed_ia(753, 108, A::offset(A::square(s.ad_value(753)), 1e-6));}
        if s.b[1400] {s.store_sub_from_scalar_scaled_input(16, 1.0, 50, p.p1113);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1402] = ((0.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.001)));s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1402]) {s.store_div_from_scalar_scaled_input(16, ((-0.001) * 0.001), 16, 16.0);}
        if (s.b[1400] && (!s.b[1402])) {s.store_scaled_add_mixed_ia(16, 16, A::sqrt_square_offset(s.ad_value(16), ((0.25 * 0.001) * 0.001)), 0.5);}
        if s.b[1400] {s.store_offset(13, 200, (-p.p1102));}
        s.b[1403] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1403]) {s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);}
        if (s.b[1400] && (!s.b[1403])) {s.store_scaled_add_offset_sqrt_square_offset(13, 13, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1400] {s.store_div_scaled_value_offset_denominator(14, s.ad_value(13), (10.0 * p.p1103), s.ad_value(13), (10.0 * p.p1103), 1.0);s.store_mul_scale_offset_rhs(754, 763, 14, p.p1101, 1.0);s.store_scale(23, 754, ((p.p2 * s.v[29]) * 1.60219e-19));}
        s.b[1404] = (p.p1110 != 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1404]) {s.store_abs_voltage(757, ctx, nodes, Some(6), Some(5));}
        s.b[1405] = (p.p1127 == 0.0);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1405]) {s.store_scalar(21, 1.0);}
        s.b[1406] = ((0.0 == 0.0) && ((s.v[757] - p.p1126) < ((-2500.0) * 0.5)));s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && s.b[1406]) {s.store_div_from_scalar_offset_scaled_input(22, ((-0.5) * 0.5), 757, 16.0, ((-p.p1126) * 16.0));}
        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && (!s.b[1406])) {s.store_scaled_add_sqrt_square_offset_ad(22, A::offset(s.ad_value(757), (-p.p1126)), ((0.25 * 0.5) * 0.5), 0.5);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1405])) {s.store_offset_scaled(21, 22, p.p1127, 1.0);}
        s.b[1408] = ((p.p1098 != 0.0) && (p.p514 > 0.0));s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1408]) {s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p514) as f64).ln())) / p.p515)));s.store_mul_ad_affine_product_rhs(750, 23, s.ad_value(21), A::scale_offset(A::powf(s.ad_value(760), p.p515), p.p514, 1.0), p.p1099, 0.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1408])) {s.store_scaled_mul(750, 23, 21, p.p1099);}
        if (s.b[1400] && s.b[1404]) {s.store_offset_div(14, 50, 753, 1.0);}
        s.b[1409] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1409]) {s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1409])) {s.store_scaled_add_mixed_ia(14, 14, A::sqrt_square_offset(s.ad_value(14), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1404]) {s.store_sub_scaled_inputs_mixed_ai(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 1.0, 50, p.p1125);}
        s.b[1410] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1410]) {s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1410])) {s.store_scaled_add_mixed_ia(18, 18, A::sqrt_square_offset(s.ad_value(18), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1404]) {s.store_mul(750, 18, 750);s.store_mul3_affine_lhs(19, 762, 235, p.p1110, 0.0, 16);s.store_mul(755, 750, 19);let t2: A = A::powf(s.ad_value(757), (4.0 - p.p1107));s.store_div_ad(752, t2, A::add_scaled_inputs(t2, 1.0, A::powf(s.ad_value(755), (4.0 - p.p1107)), p.p1122));s.store_powf(17, 752, (1.0 / p.p1107));s.store_div_scaled_product_indices(20, 17, 757, 1.0, 755, 1.0);}
        s.b[1411] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1411]) {s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1411])) {s.store_scaled_add_mixed_ia(20, 20, A::sqrt_square_offset(s.ad_value(20), ((0.25 * 0.001) * 0.001)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1400] && s.b[1404]) {s.store_mul_powf_mixed_ia(759, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));}
        s.b[1412] = (p.p1112 != 0.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1412]) {s.store_abs_voltage(758, ctx, nodes, Some(7), Some(8));}
        s.b[1414] = ((p.p1098 != 0.0) && (p.p516 > 0.0));s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1414]) {s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p516) as f64).ln())) / p.p517)));s.store_mul_scaled_powf_scale_offset_rhs(751, 23, p.p1109, 760, p.p517, p.p516, 1.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1414])) {s.store_scale(751, 23, p.p1109);}
        if (s.b[1400] && s.b[1412]) {s.store_offset_div(14, 50, 753, 1.0);}
        s.b[1415] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1415]) {s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1415])) {s.store_scaled_add_mixed_ia(14, 14, A::sqrt_square_offset(s.ad_value(14), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_sub_scaled_inputs_mixed_ai(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 1.0, 50, p.p1125);}
        s.b[1416] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1416]) {s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1416])) {s.store_scaled_add_mixed_ia(18, 18, A::sqrt_square_offset(s.ad_value(18), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_mul(751, 18, 751);s.store_mul3_affine_lhs(19, 762, 235, p.p1112, 0.0, 16);s.store_mul(756, 751, 19);let t3: A = A::powf(s.ad_value(758), (4.0 - p.p1107));s.store_div_ad(752, t3, A::add_scaled_inputs(t3, 1.0, A::powf(s.ad_value(756), (4.0 - p.p1107)), p.p1122));s.store_powf(17, 752, (1.0 / p.p1107));s.store_div_scaled_product_indices(20, 17, 758, 1.0, 756, 1.0);}
        s.b[1417] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1417]) {s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1417])) {s.store_scaled_add_mixed_ia(20, 20, A::sqrt_square_offset(s.ad_value(20), ((0.25 * 0.001) * 0.001)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_mul_powf_mixed_ia(761, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));}
        s.b[1418] = ((p.p1110 != 0.0) && (p.p1112 != 0.0));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1418]) {s.store_div_scaled_product_mixed_iia(17, 57, 188, 1.0, A::min(s.ad_value(750), s.ad_value(751)), 1.0);s.store_offset_sub_scaled_inputs(17, A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(17), (-1.0)), ((0.25 * p.p1108) * p.p1108)), 0.5, (0.25 * p.p1108));s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));}
        s.b[1419] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1418]) && s.b[1419]) {s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);}
        if ((s.b[1400] && s.b[1418]) && (!s.b[1419])) {s.store_scaled_add_offset_sqrt_square_offset(17, 17, (-1.0), (-(-1.0)), ((0.25 * p.p1108) * p.p1108), 0.5);}
        if (s.b[1400] && s.b[1418]) {s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));s.store_mul_ad_product_lhs_mixed_ia(188, 57, A::min(s.ad_value(750), s.ad_value(751)), 17);}
        s.b[1420] = (p.p1110 != 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {s.store_div_scaled_product_indices(17, 57, 188, 1.0, 750, 1.0);}
    }
}
