#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p[1137]), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1564] = (s.v[20] <= (-68.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1564]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1565] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1564]) && s.b[1565]) {s.store_limited_exp(15, 16);}
        s.b[1566] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && s.b[1566]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && (!s.b[1566])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1564]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(200, 15, 13, 1.0, 20, (-p[1137]), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p[1137]), 1.0);}
        if (s.b[1562] && (!s.b[1564])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p[1137], 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p[1137], s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p[1137], s.ad_value(95), p[1137], A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p[1137], 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p[1137], s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p[1137], s.ad_value(95), p[1137], A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1564])) {s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p[1137]);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p[1137], A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p[1137])), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1567] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1567]) {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (s.b[1562] && (!s.b[1567])) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(96, 93);s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);}
        s.b[1568] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1568]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1568])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_offset_div_ad(90, s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1569] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1569]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (s.b[1562] && (!s.b[1569])) {s.store_scaled_add_sqrt_square_offset_rhs(84, 12, 12, ((0.25 * 0.1) * 0.1), 0.5);}
        if s.b[1562] {s.store_mul3_affine_lhs(130, 90, 108, 2.0, 0.0, 200);s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 132, 407);s.store_offset(16, 15, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1570] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1570]) {s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);}
        if (s.b[1562] && (!s.b[1570])) {s.store_scaled_add_offset_sqrt_square_offset(133, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(137, 499, 108, 1.0, 133, 411, s.v[34]);s.store_div_scaled_product_offset_denominator_mixed_iaa(131, 137, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 1.0, A::mul_offset_rhs(s.ad_value(137), s.ad_value(200), 1.0), 1.0, 1.0);s.store_add_scaled_inputs4_mixed_iiia(145, 91, 1.0, 233, (-2.0), 131, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(146, 145, 108);}
        s.b[1571] = ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001)));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1571]) {s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(146), 16.0, s.ad_value(72), 16.0));}
        if (s.b[1562] && (!s.b[1571])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(141, 146, 0.5, 72, ((-1.0) * 0.5), 146, 72, ((0.25 * 0.001) * 0.001), 0.5);}
        s.b[1572] = ((p[1134] == 0.0) && (p[1135] == 0.0));s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1572]) {s.store_scalar(783, p[1129]);}
        if (s.b[1562] && (!s.b[1572])) {s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);s.store_offset_div_scaled_inputs2_mixed_iaa(783, 13, p[1134], A::mul3_scaled_output(s.ad_value(13), s.ad_value(200), s.ad_value(106), p[1135]), (-1.0), A::scale_offset(s.ad_value(61), p[1136], 1.0), 1.0, 1.0);}
        s.b[1573] = ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005)));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((s.b[1562] && (!s.b[1572])) && s.b[1573]) {s.store_div_from_scalar_scaled_input(783, ((-0.0005) * 0.0005), 783, 16.0);}
        if ((s.b[1562] && (!s.b[1572])) && (!s.b[1573])) {s.store_scaled_add_offset_sqrt_square_offset(783, 783, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        if s.b[1562] {s.store_div(141, 141, 783);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 75, 20);s.store_mul_add_lhs(142, 139, 72, 109);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 234, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 233, (-2.0), 142, -1.0);s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p[1137]), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1574] = (s.v[20] <= (-68.0));s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1574]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1575] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1574]) && s.b[1575]) {s.store_limited_exp(15, 16);}
        s.b[1576] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && s.b[1576]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && (!s.b[1576])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1574]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(144, 15, 13, 1.0, 20, (-p[1137]), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p[1137]), 1.0);}
        if (s.b[1562] && (!s.b[1574])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p[1137], 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p[1137], s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p[1137], s.ad_value(95), p[1137], A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p[1137], 13, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1574])) {s.store_add_ad(17, A::offset(A::div_from_scalar(p[1137], s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p[1137], s.ad_value(95), p[1137], A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p[1137]);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p[1137], A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p[1137])), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        if s.b[1562] {s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 200, (-1.0), 144, -1.0, (-1.0));}
        s.b[1577] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1577]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1577])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(14, 12);s.store_add_offset_lhs_mixed_ia(15, 203, 1.0, A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));s.store_offset_product3(16, s.ad_value(203), s.ad_value(14), s.ad_value(126), 1.0, 0.5);s.store_sqrt_add_ad(17, A::square(s.ad_value(16)), A::mul3(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(218)));s.store_div_add_scaled_inputs_rhs_indices(90, 15, 16, 1.0, 17, 1.0);s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1578] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1578]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1578])) {s.store_scaled_add_sqrt_square_offset_rhs(84, 12, 12, ((0.25 * 0.1) * 0.1), 0.5);}
        if s.b[1562] {s.store_mul_mixed_ia(13, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(144), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1579] = ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1)));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1579]) {s.store_div_from_scalar_scaled_input(85, ((-0.1) * 0.1), 13, 16.0);}
        if (s.b[1562] && (!s.b[1579])) {s.store_scaled_add_sqrt_square_offset_rhs(85, 13, 13, ((0.25 * 0.1) * 0.1), 0.5);}
        if s.b[1562] {s.store_scaled_add(86, 84, 85, 0.5);s.store_mul_ad_product_rhs_mixed_ia(80, 90, 108, A::add(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(156, 86, s.v[155], 80, (s.v[158] * s.v[155]));s.store_offset(13, 203, 1.0);s.store_div_scaled_inputs2_indices(204, 60, 1.0, 109, p[136], 13, 1.0);s.store_div(205, 234, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1580] = (s.v[204] < 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1580]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1562] && (!s.b[1580])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1562] {s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 156, 407);s.store_offset(16, 15, 1.0);}
        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1581]) {s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);}
        if (s.b[1562] && (!s.b[1581])) {s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(138, 499, 108, 2.0, 159, 411, s.v[34]);s.store_sub(87, 200, 144);s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);s.store_sqrt_offset_input(161, 13, 1.0);s.store_scaled_offset(162, 161, 1.0, 0.5);s.store_div_scaled_inputs_mixed_ia(134, 411, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);s.store_scale(135, 134, s.v[34]);s.store_add(170, 141, 135);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1562] {s.store_sub(167, 75, 139);}
        s.b[1582] = (s.v[542] != 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);}
        if (!s.b[1582]) {s.store_scalar(176, 1.0);}
        s.store_square(207, 176);s.store_div_from_scalar(208, 1.0, 176);s.store_div_from_scalar(209, 1.0, 207);s.store_offset(210, 176, (-1.0));s.store_sub(213, 60, 91);s.store_sub(216, 200, 144);s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if s.b[1583] {s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);}
        if (!s.b[1583]) {s.store_scaled_add_sqrt_square_offset_rhs(13, 211, 211, ((0.25 * 0.5) * 0.5), 0.5);}
        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if s.b[1584] {s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);}
        if (!s.b[1584]) {s.store_scaled_add_sqrt_square_offset_rhs(14, 212, 212, ((0.25 * 0.5) * 0.5), 0.5);}
        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);s.store_div_mixed_ia(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));s.store_div_mixed_ia(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));s.store_add(15, 214, 215);s.store_div_scaled_value_by_product_mixed_iai(16, 217, 0.3333333333333333, A::square(s.ad_value(15)), 15, 1.0);s.store_div_scaled_product3_mixed_iiia(17, 783, 162, 208, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0);s.store_mul_scale_offset_mixed_ia(18, 17, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 0.0);s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);s.store_scaled_mul(20, 217, 17, 0.3333333333333333);s.store_div_scaled_product_mixed_iaa(202, 212, A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0);s.store_add_mixed_ai(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);s.store_add_scaled_products_mixed_iaii(189, 208, A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, 210, 201, 1.0);s.store_add(21, 200, 144);s.store_mul3_lhs(22, 217, 17, 17);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_add_ad(194, A::mul3(s.ad_value(90), s.ad_value(208), A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(217), s.ad_value(17), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(90), s.ad_value(210), s.ad_value(144), 2.0));s.store_mul_ad_product_rhs_mixed_ia(191, 90, 209, A::add_scaled_product(s.ad_value(21), 0.5, s.ad_value(216), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), 1.0, s.ad_value(22), 0.2), (-1.0 / (6.0))));s.store_mul_ad_product_lhs_mixed_ia(192, 90, A::sub(s.ad_value(176), s.ad_value(208)), 144);s.store_add(193, 191, 192);s.store_sub(190, 194, 193);s.b[1585] = ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p[694])));s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if s.b[1585] {s.store_div_scalar_by_product_indices(83, ((-p[694]) * p[694]), 108, 189, 16.0);}
        if (!s.b[1585]) {s.store_add_scaled_product_mixed_aii(83, A::sqrt_square_offset(A::mul(s.ad_value(108), s.ad_value(189)), ((0.25 * p[694]) * p[694])), 0.5, 108, 189, 0.5);}
        s.store_mul_add_rhs(82, 108, 190, 193);s.store_add_scaled_inputs(12, 82, 1.0 / (p[207]), 83, (p[208] * 1.0 / (p[207])));s.store_offset_powf_ad(13, s.ad_value(12), (0.7 * p[206]), 1.0);s.store_div_from_scalar(227, (p[205] * 1.9e-9), 13);s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p[111])), s.ad_value(227), 1.0 / (s.v[47])));s.store_mul_ad_affine_product_lhs(195, A::div_from_scalar((8.85418e-12 * p[111]), s.ad_value(229)), s.ad_value(108), (((-p[2]) * s.v[33]) * s.v[34]), 0.0, 189);s.store_scaled_mul(199, 228, 108, ((p[2] * s.v[33]) * s.v[34]));s.store_mul_scale_offset_indices(196, 190, 199, -1.0, 0.0);s.store_mul_scale_offset_indices(197, 193, 199, -1.0, 0.0);s.store_add_scaled_inputs3_indices(198, 195, (-1.0), 196, (-1.0), 197, (-1.0));s.b[1586] = (!param_given[666]);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if s.b[1586] {s.store_scalar(544, ((((2.0 * p[111]) * 8.85418e-12) / 3.141592653589793) * ((((p[670] * (1.0 + (4e-7 / p[77])))).max(1e-38)) as f64).ln()));}
        s.store_primal_offset(225, 544, p[671]);s.store_primal_offset(226, 544, p[672]);s.b[1587] = (p[41] == 0.0);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if s.b[1587] {s.store_scaled_mul(223, 225, 231, ((-s.v[33]) * p[2]));s.store_scaled_mul(224, 226, 232, ((-s.v[33]) * p[2]));}
        if (!s.b[1587]) {s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(219, 231, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(18, 219, A::powf(A::offset(A::powf(A::scale(s.ad_value(219), (-1.0 / (p[692]))), p[693]), 1.0), (1.0 / p[693])));s.store_sqrt_sub_from_scalar_ad(13, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(547), 1.0));s.store_add_scaled_products_mixed_iiia(223, 225, 231, ((-s.v[33]) * p[2]), 545, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(63), (-1.0), s.ad_value(219), -1.0), 1.0, s.ad_value(547), s.ad_value(13), (-1.0), (-0.5)), ((-s.v[33]) * p[2]));s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(220, 232, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1587]) {s.store_div_mixed_ia(18, 220, A::powf(A::offset(A::powf(A::scale(s.ad_value(220), (-1.0 / (p[690]))), p[691]), 1.0), (1.0 / p[691])));s.store_sqrt_sub_from_scalar_ad(14, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(548), 1.0));s.store_add_scaled_products_mixed_iiia(224, 226, 232, ((-s.v[33]) * p[2]), 546, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(63), (-1.0), s.ad_value(220), -1.0), 1.0, s.ad_value(548), s.ad_value(14), (-1.0), (-0.5)), ((-s.v[33]) * p[2]));}
        s.store_mul_scaled_voltage(221, 187, (((-p[2]) * s.v[34]) * p[673]), ctx, nodes, Some(10), Some(11));s.b[1588] = (p[37] == 1.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if s.b[1588] {s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19));}
        if s.b[1588] {
            s.store_mul_mixed_ia(674, 612, {
                            if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1588] {s.store_mul_scale_offset_mixed_ia(673, 616, A::mul_offset_rhs(s.ad_value(623), s.ad_value(395), (-1.0)), 1.0, 1.0);}
        s.b[1589] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1589]) {s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));}
        if (s.b[1588] && (!s.b[1589])) {s.store_add_scaled_inputs3_offset_mixed_iia(110, 127, 0.5, 61, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));}
        if s.b[1588] {s.store_sqrt(111, 110);s.store_mul(112, 114, 111);s.store_div_from_scalar(97, s.v[26], 112);s.store_add_scaled_inputs_products_indices(113, 613, 1.0, 674, 1.0, 614, 76, 1.0, 615, 61, (-1.0));s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);}
        s.b[1590] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1590]) {s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);}
        if (s.b[1588] && (!s.b[1590])) {s.store_scaled_add_offset_sqrt_square_offset(104, 13, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);}
        if s.b[1588] {s.store_mul(106, 104, 108);s.store_div_from_scalar(107, 1.0, 106);s.store_mul(65, 64, 107);s.store_mul(73, 70, 107);s.store_mul(58, 482, 107);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1588] {s.store_mul_scale_offset_mixed_ia(677, 76, A::add_scaled_product(s.ad_value(673), 1.0, s.ad_value(617), s.ad_value(61), 1.0), -1.0, 0.0);s.store_mul_scale_offset(124, A::add_scaled_inputs_product(s.ad_value(618), 1.0, s.ad_value(619), 1.0 / (s.v[30]), s.ad_value(620), s.ad_value(61), 1.0), A::pow(s.ad_value(395), s.ad_value(621)), 1.0, (-1.0));s.store_mul_scale_offset_rhs(679, 129, 61, p[1016], 1.0);}
        s.b[1591] = (s.v[679] > 0.0);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1591]) {s.store_div_from_scalar(12, (p[1015] * s.v[30]), 679);}
        s.b[1592] = (s.v[12] < 40.0);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
        if ((s.b[1588] && s.b[1591]) && s.b[1592]) {s.store_div_from_scalar_offset_ad(676, (0.5 * p[1014]), A::cosh(s.ad_value(12)), (-1.0));}
        if ((s.b[1588] && s.b[1591]) && (!s.b[1592])) {s.store_scaled_limited_exp_scaled_input(676, 12, -1.0, p[1014]);}
        if (s.b[1588] && (!s.b[1591])) {s.store_scalar(676, 0.0);}
        if s.b[1588] {s.store_mul_sub_rhs(678, 676, 675, 127);s.store_add_mixed_ai(79, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(677), 1.0, s.ad_value(124), (-1.0), s.ad_value(678), 1.0, s.ad_value(688), 1.0, p[961]), 1.0, A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61), (-1.0)), 665);s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));s.store_scalar(680, (p[958] * (1.0 + (p[959] * ((s.v[30]) as f64).powf((-p[960]))))));s.store_scaled_sqrt_mul_scaled_lhs(687, 686, ((2.0 * 1.60219e-19) * s.v[26]), 107, 1.0 / (s.v[46]));s.store_mul_scale_offset_indices(687, 687, 680, 1.0, 1.0);s.store_div(685, 684, 104);s.store_scalar(13, 1.0);s.store_div(204, 59, 13);s.store_div(205, 687, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1593] = (s.v[204] < 0.0);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1593]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1588] && (!s.b[1593])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1588] {s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 687, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 685, (-2.0), 73, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1588] {s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1594] = (s.v[20] <= (-68.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1594]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1595] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((s.b[1588] && s.b[1594]) && s.b[1595]) {s.store_limited_exp(15, 16);}
        s.b[1596] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && s.b[1596]) {s.store_limited_exp(15, 20);}
        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && (!s.b[1596])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1588] && s.b[1594]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(693, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (s.b[1588] && (!s.b[1594])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1588] && (!s.b[1594])) {s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(693, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        if s.b[1588] {s.store_add_scaled_product_indices(681, 106, 2.0, 106, 693, 2.0);s.copy_ad(682, 681);s.store_add(682, 682, 70);}
        s.b[1597] = ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001)));s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1597]) {s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(682), 16.0, s.ad_value(70), 16.0));}
        if (s.b[1588] && (!s.b[1597])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(683, 682, 0.5, 70, ((-1.0) * 0.5), 682, 70, ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1588] {s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 74, 20);s.store_mul_add_lhs(142, 139, 70, 107);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 687, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 685, (-2.0), 142, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1598] = (s.v[20] <= (-68.0));s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1598]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
    ) {
        s.b[1599] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((s.b[1588] && s.b[1598]) && s.b[1599]) {s.store_limited_exp(15, 16);}
        s.b[1600] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && s.b[1600]) {s.store_limited_exp(15, 20);}
        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1588] && s.b[1598]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(692, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (s.b[1588] && (!s.b[1598])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1588] && (!s.b[1598])) {s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(692, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1601] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1601]) {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (s.b[1588] && (!s.b[1601])) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1588] {s.store_sqrt(96, 93);s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 693, (-1.0), 692, -1.0, (-1.0));}
        s.b[1602] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1602]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1588] && (!s.b[1602])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1588] {s.store_sqrt(14, 12);s.store_offset_div_ad(691, s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14)), 1.0);s.store_mul_product3_mixed_iaia(672, 175, A::mul3_scaled_output(s.ad_value(691), s.ad_value(157), s.ad_value(106), ((2.0 * p[2]) * ((p[957] * 1.0 / (s.v[30])) * s.v[46]))), 106, A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692))), 1.0);s.store_add(188, 672, 188);s.store_scalar(696, (p[785] * p[1062]));s.store_scalar(697, (p[799] * p[1062]));s.store_scalar(698, (p[800] * p[1062]));s.store_primal_sub_from_scalar_scaled_input(694, s.v[30], 359, 2.0);s.store_primal_square(695, 694);s.store_mul_add_scaled_inputs_rhs_mixed_ai(367, 108, A::offset(s.ad_value(97), s.v[46]), 1.0 / (1.60219e-19), 613, 1.0 / (1.60219e-19));s.store_mul3_affine_lhs(366, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 692);s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(672)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);s.store_mul3_affine_lhs(737, 108, 672, 1.60219e-19, 0.0, 672);s.store_add_ad(738, A::add_scaled_product(s.ad_value(696), 1.0, s.ad_value(697), s.ad_value(366), 1.0), A::mul3(s.ad_value(698), s.ad_value(366), s.ad_value(366)));s.store_square_ad(739, A::add(s.ad_value(366), s.ad_value(367)));s.store_scaled_mul(740, 696, 108, 1.60219e-19);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1588] {s.store_mul3_affine_lhs(365, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 693);s.store_mul_ln_mixed_ia(13, 696, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));s.store_mul_sub_rhs(14, 697, 365, 366);s.store_mul_sub_scaled_inputs_rhs(15, 698, A::square(s.ad_value(365)), 0.5, A::square(s.ad_value(366)), 0.5);s.store_scale(16, 695, (10000000000.0 * (p[957] * p[2])));s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);s.store_mul3_affine_lhs(17, 694, 367, ((p[957] * p[2]) * 10000000000.0), 0.0, 367);s.store_mul_ad_product_lhs_mixed_ai(369, A::div(s.ad_value(740), s.ad_value(17)), 672, 672);s.store_add(18, 369, 368);}
        s.b[1603] = (s.v[18] > 0.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1603]) {s.store_div_scaled_product_indices(19, 368, 369, 1.0, 18, 1.0);s.store_offset_scaled_ad(20, A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p[1064]), p[1063], 1.0);}
        s.b[1604] = (s.v[57] > 0.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if s.b[1604] {s.store_scaled_mul(785, 187, 196, p[29]);s.store_scaled_mul(786, 187, 197, p[29]);}
        if (!s.b[1604]) {s.store_scaled_mul(785, 187, 197, p[29]);s.store_scaled_mul(786, 187, 196, p[29]);}
        s.b[1605] = ((p[1094] == 1.0) && (p[1095] == 1.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if s.b[1605] {s.store_add(221, 221, 774);s.store_add(224, 224, 775);}
        s.b[1606] = (p[1096] == 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if (s.b[1605] && s.b[1606]) {s.store_add(221, 221, 776);s.store_add(223, 223, 777);}
        s.store_scaled_mul(787, 187, 198, p[29]);s.b[1612] = ((p[42] != 2.0) && (s.v[240] > 0.0));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if s.b[1612] {s.store_div_from_scalar(372, 1.0, 242);}
        s.b[1613] = (((p[42] == 1.0) && (p[1094] == 1.0)) && (p[1110] > 0.0));s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (s.b[1612] && s.b[1613]) {s.store_div_from_scalar(374, 1.0, 759);}
        s.b[1614] = ((p[42] != 2.0) && (s.v[239] > 0.0));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if s.b[1614] {s.store_div_from_scalar(371, 1.0, 241);}
        s.b[1615] = (((p[42] == 1.0) && (p[1094] == 1.0)) && (p[1112] > 0.0));s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (s.b[1614] && s.b[1615]) {s.store_div_from_scalar(373, 1.0, 761);}
        s.b[1621] = ((p[49] != 0.0) && (p[909] > 0.0));s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if s.b[1621] {s.store_mul_voltage_ad(749, A::mul3(s.ad_value(187), s.ad_value(57), s.ad_value(188)), ctx, nodes, Some(5), Some(7));}
        s.b[1622] = ((p[42] != 2.0) && (s.v[240] > 0.0));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });s.b[1623] = (((p[42] == 1.0) && (p[1094] == 1.0)) && (p[1110] > 0.0));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1621] && s.b[1622]) && s.b[1623]) {s.store_add_scaled_value_products_mixed_iaiai(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), 372, 1.0, A::square(A::voltage(ctx, nodes, Some(6), Some(5))), 374, 1.0);}
        if ((s.b[1621] && s.b[1622]) && (!s.b[1623])) {s.store_add_scaled_product_mixed_iai(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), 372, 1.0);}
        s.b[1624] = ((p[42] != 2.0) && (s.v[239] > 0.0));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });s.b[1625] = (((p[42] == 1.0) && (p[1094] == 1.0)) && (p[1112] > 0.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1621] && s.b[1624]) && s.b[1625]) {s.store_add_scaled_value_products_mixed_iaiai(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 371, 1.0, A::square(A::voltage(ctx, nodes, Some(8), Some(7))), 373, 1.0);}
        if ((s.b[1621] && s.b[1624]) && (!s.b[1625])) {s.store_add_scaled_product_mixed_iai(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 371, 1.0);}
        s.b[1627] = (p[8] != 0.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });s.b[1628] = (p[1097] == 0.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });s.b[1630] = ((p[8] != 0.0) && (p[1097] == 1.0));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
    }
}
