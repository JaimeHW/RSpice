#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
    ) {
        s.b[780] = (2.0 == 1.0);s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[778]) && s.b[779]) && s.b[780]) {s.store_scalar(55, 1.0);}
        s.b[781] = (2.0 == 2.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        if ((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && s.b[781]) {s.store_scalar(55, 2.0);}
        s.b[782] = (2.0 == 4.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && s.b[782]) {s.store_scalar(55, 3.0);}
        s.b[783] = (2.0 == 8.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && (!s.b[782])) && s.b[783]) {s.store_scalar(55, 4.0);}
        if ((s.b[735] && s.b[778]) && s.b[779]) {s.store_scalar(54, 0.0);}
        let mut t14: usize = 0;
        while {
            let t13: f64 = if (((s.b[735] && s.b[778]) && s.b[779]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t13 != 0.0
        } {
            t14 += 1;assert!(t14 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[735] && s.b[778]) && s.b[779]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[735] && s.b[778]) && (!s.b[779])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[735] && s.b[778]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 738, 53);s.store_add_scaled_inputs3_indices(405, 403, 1.0, 738, (-1.0), 43, 1.0);}
        if (s.b[735] && (!s.b[778])) {
        }
        if s.b[735] {s.store_mul_scale_offset_indices(369, 229, 405, -1.0, 0.0);s.store_add_scaled_product_indices(384, 227, 1.0, 341, 738, ((-0.5) * 9662367879.197212));s.store_add_scaled_product_indices(385, 384, 1.0, 386, 738, (-9662367879.197212));}
        s.b[784] = (s.v[144] >= 1.0);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[784]) {s.store_scalar(349, s.v[619]);s.store_scalar(350, s.v[620]);s.store_scalar(351, s.v[621]);}
        if (s.b[735] && s.b[784]) {s.store_scalar(339, (if (s.v[349] < s.v[385]) { 1.0 } else { 2.0 }));}
        if (s.b[735] && (!s.b[784])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if (s.b[735] && (!s.b[784])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (s.b[735] && (!s.b[784])) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul(181, 225, 376);}
        s.b[785] = (s.v[181] < 3.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[785]) {s.store_mul_sub_rhs(337, 225, 178, 156);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[786] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && s.b[786]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && s.b[786]) {s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[735] && (!s.b[784])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_div_scaled_inputs(401, 378, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
            } else {
                s.store_scalar(401, 0.0);
            }
        }
        s.b[787] = (s.v[401] < s.v[738]);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[787]) {s.store_scalar(339, 1.0);}
        if ((s.b[735] && (!s.b[784])) && (!s.b[787])) {s.store_scalar(339, 2.0);}
        s.b[788] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[788]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if ((s.b[735] && (!s.b[784])) && (!s.b[788])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        s.b[789] = ((s.v[178] - s.v[383]) > 0.0);s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) {s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);}
        s.b[790] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });s.b[792] = (2.0 == 1.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && s.b[792]) {s.store_scalar(55, 1.0);}
        s.b[793] = (2.0 == 2.0);s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && s.b[793]) {s.store_scalar(55, 2.0);}
        s.b[794] = (2.0 == 4.0);s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });
        if ((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && s.b[794]) {s.store_scalar(55, 3.0);}
        s.b[795] = (2.0 == 8.0);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        if (((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && (!s.b[794])) && s.b[795]) {s.store_scalar(55, 4.0);}
        if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {s.store_scalar(54, 0.0);}
        let mut t16: usize = 0;
        while {
            let t15: f64 = if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t15 != 0.0
        } {
            t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && (!s.b[791])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && (!s.b[790])) {s.copy_ad(378, 376);}
        if (s.b[735] && (!s.b[784])) {s.copy_ad(349, 378);s.copy_ad(163, 376);s.store_sub_mixed_ai(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(737), 0.5), 475);}
        s.b[796] = (s.v[328] < 0.0);s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {s.store_mul_scale_offset_indices(329, 474, 737, 1.0, s.v[94]);s.store_square(329, 329);s.store_offset_scaled(332, 328, (-1.6), 0.6);s.store_scalar(331, 0.5);s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));}
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(330, 329, 331, 226);s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));}
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(738), (-(1.0 / (2.0) * 9662367879.197212))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));}
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_sqrt(329, 329);s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);s.store_scaled_sub(380, 330, 329, 0.5);s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));}
        s.b[797] = (s.v[380] < s.v[382]);s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && s.b[797]) {s.copy_ad(351, 380);}
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {s.store_offset_sub(44, 381, 380, (-0.0008));s.store_scale(45, 381, (4.0 * 0.0008));}
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[735] && (!s.b[784])) {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t18: usize = 0;
        while {
            let t17: f64 = if ((s.b[735] && (!s.b[784])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t17 != 0.0
        } {
            t18 += 1;assert!(t18 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[735] && (!s.b[784])) {s.copy_ad(328, 474);s.store_mul(329, 225, 351);s.store_exp_neg_input(330, 329);}
            s.b[798] = (s.v[351] > 1e-9);s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[798]) {s.store_exp_mul(327, 225, 351);s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));}
            s.b[799] = (s.v[351] < (-1e-9));s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[784])) && (!s.b[798])) && s.b[799]) {s.store_mul_sqrt_mixed_ia(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));s.store_mul_scale_offset_mixed_ai(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 330, -1.0, 1.0);}
            if (((s.b[735] && (!s.b[784])) && (!s.b[798])) && (!s.b[799])) {s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);}
            if (s.b[735] && (!s.b[784])) {s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 741, 741, 4.0);s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 741, 1e-10);}
            s.b[800] = (s.v[333] < 0.0);s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[800]) {s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);}
            if (s.b[735] && (!s.b[784])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 742, -1.0);s.store_scaled_mul(45, 341, 742, (-4.0));}
            if (s.b[735] && (!s.b[784])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[735] && (!s.b[784])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(334, 334, 332, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);s.store_sub_mixed_ia(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(738), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(738), 9662367879.197212), s.ad_value(389)), 1.0));s.copy_ad(334, 167);}
            s.b[801] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[801]) {s.store_scalar(167, s.v[57]);}
            if (s.b[735] && (!s.b[784])) {s.copy_ad(351, 333);s.copy_ad(357, 331);s.store_primal_offset(167, 167, 1.0);}
        }
        if (s.b[735] && (!s.b[784])) {s.store_add(351, 475, 351);s.store_add_scaled_product_mixed_iia(350, 349, 1.0, 737, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
        s.b[802] = ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2)));s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[735] && s.b[802]) {s.store_scalar(446, s.v[136]);s.store_add_scaled_inputs4_indices(445, 174, 1.0, 446, (-1.0), 185, 1.0, 320, -1.0);s.store_scalar(143, p.p137);s.copy_ad(207, 445);s.store_sqrt_div_scaled_inputs(208, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10), 225, 1.0);s.store_div_scaled_product_by_product_indices(209, 230, 230, 1.0, 544, 544, 1.0);s.store_div_scaled_product_by_product_indices(210, 208, 208, 1.0, 323, 323, 1.0);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_scalar_by_product_indices(223, 1.0, 209, 210, 1.0);s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));s.store_add_scaled_inputs3_indices(217, 216, 1.0, 215, (-1.0), 143, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(218, 216, 1.0, 217, (-0.5), A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0), (-0.5));s.store_exp_mul(224, 225, 218);s.store_add_scaled_product_mixed_aii(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);s.store_offset_mul(220, 225, 218, (-1.0));}
        s.b[803] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
        if ((s.b[735] && s.b[802]) && s.b[803]) {s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_div_scaled_inputs_indices(214, 105, 2.0, 225, 1.0);s.store_scalar(250, (300.0 * 0.0001));s.store_scalar(316, 0.0);s.store_neg_ad(328, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(173))), (-1.0)));s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));s.store_mul_ad_product_lhs_mixed_ai(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 328, 329);s.copy_ad(394, 222);s.copy_ad(395, 218);s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        s.b[804] = (s.v[336] < (10.0 * 2.220446049250313e-16));s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[804]) {s.store_scalar(336, (10.0 * 2.220446049250313e-16));}
        if ((s.b[735] && s.b[802]) && s.b[803]) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.copy_ad(163, 376);s.store_sub(166, 376, 395);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[805] = (s.v[166] < 0.0);s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[805]) {s.store_scalar(166, 0.0);}
        if ((s.b[735] && s.b[802]) && s.b[803]) {s.store_scale(332, 166, (1.0 + 0.3));s.store_offset_sub(333, 332, 173, (-0.03));s.store_sqrt_add_scaled_square_input(334, 333, 1.0, 332, (4.0 * 0.03));s.store_add_scaled_inputs3_indices(165, 332, 1.0, 333, (-0.5), 334, (-0.5));}
        s.b[806] = (s.v[165] > s.v[166]);s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[806]) {s.copy_ad(165, 166);}
        if ((s.b[735] && s.b[802]) && s.b[803]) {s.copy_ad(449, 165);s.store_scalar(824, (s.v[88] * 100.0));s.store_primal_scale(825, 107, 100.0);s.store_scalar(826, (s.v[97] * 100.0));}
        s.b[827] = (p.p36 == 0.0);s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[827]) {s.store_scalar(447, 0.0);}
        if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.store_scalar(448, 4.12);s.store_primal_scaled_mul(807, 825, 826, (p.p142 * 1.6021918e-19));s.store_div(808, 807, 302);s.store_div_scaled_inputs_mixed_ai(809, A::offset(A::add_scaled_inputs4(s.ad_value(514), p.p145, s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(237), 1.0), p.p144), -1.0, 824, 1.0);s.store_scalar(562, 0.0);}
        let mut t2: usize = 0;
        while {
            let t0: f64 = (100.0 - 1.0);let t1: f64 = if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && (s.v[562] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.copy_ad(810, 562);s.store_scalar(811, 100.0);s.store_primal_div(812, 810, 811);s.store_add_scaled_inputs3_mixed_iia(813, 159, 1.0, 175, 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(812), 1.0), -1.0);s.store_sub_from_scalar_div_indices(814, 1.0, 813, 448);s.store_add_div_rhs_indices(817, 809, 813, 824);s.store_square(815, 817);s.store_sqrt_square_offset(44, 814, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(814, 814, 0.5, 44, 0.5, (1e-10 * 0.001));}
            s.b[828] = (s.v[814] < 0.0);s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && s.b[828]) {s.store_scalar(814, 0.0);}
            if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.store_offset_scaled_ad(816, A::mul(A::sqrt(s.ad_value(814)), s.ad_value(814)), (-p.p143), p.p143);s.store_div_scaled_inputs_indices(818, 816, -1.0, 817, 1.0);}
            s.b[829] = (s.v[818] < (-34.0));s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && s.b[829]) {s.store_scalar(820, 0.0);}
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && (!s.b[829])) {s.store_exp(820, 818);}
            if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.copy_ad(821, 808);s.store_mul3_affine_lhs(822, 821, 816, (0.25 * 7.38905609893065), 0.0, 816);}
            s.b[830] = (((2.0 * s.v[817]) + s.v[816]) < 0.0);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && s.b[830]) {s.copy_ad(450, 822);}
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && (!s.b[830])) {s.copy_ad(819, 807);s.store_mul3_lhs(823, 819, 815, 820);}
            s.b[831] = ((s.v[823] < s.v[822]) || (s.v[817] < 0.0));s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });
            if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && (!s.b[830])) && s.b[831]) {s.copy_ad(450, 822);}
            if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && (!s.b[830])) && (!s.b[831])) {s.copy_ad(450, 823);}
            if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.store_add(447, 447, 450);}
            s.b[832] = (s.v[450] < 1e-9);s.store_scalar(832, if s.b[832] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) && s.b[832]) {s.store_scalar(562, 100.0);s.store_scalar(167, s.v[57]);}
            if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[827])) {s.store_primal_offset(562, 562, 1.0);}
        }
        s.b[845] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[845]) {s.store_scalar(263, 0.0);}
        s.b[846] = (p.p44 <= 0.0);s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && s.b[846]) {s.copy_ad(833, 445);s.store_square(840, 323);s.copy_ad(841, 545);s.store_div(835, 841, 840);s.store_div_from_scalar(842, 2.0, 841);s.store_mul(836, 842, 840);s.store_add_scaled_inputs_product_indices(837, 833, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(839, 836, 837, 1.0);s.store_sqrt_square_offset(44, 839, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(838, 839, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[847] = (s.v[838] < 0.0);s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && s.b[846]) && s.b[847]) {s.store_scalar(838, 0.0);}
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && s.b[846]) {s.store_offset(838, 838, 1e-50);s.store_sqrt(838, 838);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && s.b[846]) {s.store_add_scaled_product_mixed_aii(843, A::mul_sub_from_scalar_rhs(s.ad_value(835), 1.0, s.ad_value(838)), 1.0, 833, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(844, 173, p.p122, 395, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(843)), -1.0);s.store_sqrt_square_offset(44, 844, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(844, 844, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[848] = (s.v[844] < 0.0);s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && s.b[846]) && s.b[848]) {s.store_scalar(844, 0.0);}
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) {s.store_mul(833, 134, 445);s.store_div_square_rhs(835, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(836, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(837, 833, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(838, 836, 837, 1.0);s.store_scaled_offset(840, 836, 1.0, 2.0);}
        s.b[849] = ((s.v[838] < (1e-50 + s.v[840])) && (s.v[840] >= 0.0));s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) {s.store_sub_offset_lhs(44, 840, 1e-50, 838);s.store_square(49, 44);s.store_square(50, 840);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[850] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });s.b[851] = (4.0 == 1.0);s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) && s.b[851]) {s.store_scalar(55, 1.0);}
        s.b[852] = (4.0 == 2.0);s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
        if ((((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) && (!s.b[851])) && s.b[852]) {s.store_scalar(55, 2.0);}
        s.b[853] = (4.0 == 4.0);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
        if (((((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) && (!s.b[851])) && (!s.b[852])) && s.b[853]) {s.store_scalar(55, 3.0);}
        s.b[854] = (4.0 == 8.0);s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });
        if ((((((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) && (!s.b[851])) && (!s.b[852])) && (!s.b[853])) && s.b[854]) {s.store_scalar(55, 4.0);}
        if ((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) {s.store_scalar(54, 0.0);}
        let mut t4: usize = 0;
        while {
            let t3: f64 = if (((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;assert!(t4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && s.b[850]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) && (!s.b[850])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[849]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 840, 53);s.store_sub_offset_lhs(838, 840, 1e-50, 43);}
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && (!s.b[849])) {
        }
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) {
            if (s.v[838] <= 0.0) {
                s.store_scalar(838, 0.0);
            } else {
                s.store_sqrt(838, 838);
            }
        }
        if ((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) {s.store_add_mul_sub_from_scalar_rhs_indices(843, 833, 835, 1.0, 838);s.store_div_from_scalar_offset_input(834, s.v[100], 131, s.v[100]);s.store_add_scaled_product_mixed_aii(844, A::scale_offset(s.ad_value(173), p.p122, s.v[176]), 1.0, 834, 843, (-1.0));s.store_sqrt_square_offset(44, 844, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(844, 844, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[855] = (s.v[844] < 0.0);s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) && (!s.b[846])) && s.b[855]) {s.store_scalar(844, 0.0);}
        if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[845])) {s.store_offset(844, 844, 1e-50);s.store_ad_value(834, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(844), 1.0));s.store_mul_product3_indices(263, 834, 132, 844, 394, 1.0);}
        s.b[863] = (p.p26 == 1.0);s.store_scalar(863, if s.b[863] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[735] && s.b[802]) && s.b[803]) && s.b[863]) {s.store_mul_ad_affine_product_rhs(856, 738, s.ad_value(107), A::exp_scaled_input(s.ad_value(225), (-p.p141)), 1.6021918e-19, 0.0);s.store_offset_scaled(857, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(858, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 856, 857, 1.0);s.store_mul_add_lhs(567, 263, 447, 858);s.store_mul_scaled_ln_offset_rhs(859, 227, p.p140, 567, 1.0);s.store_sqrt_mul_scaled_lhs(860, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_sqrt_ad(861, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(395), s.ad_value(859)))), (-1.0)), 1.0, s.ad_value(225), A::sub(s.ad_value(395), s.ad_value(859)), 1.0));s.store_sqrt_ad(862, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(395))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(395), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(393, 860, 861, -1.0, 862, -1.0);}
        if ((((s.b[735] && s.b[802]) && s.b[803]) && s.b[863]) && (p.p37 != 0.0)) {s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);}
        if (((s.b[735] && s.b[802]) && s.b[803]) && (!s.b[863])) {s.store_scalar(393, 0.0);}
        if ((s.b[735] && s.b[802]) && (!s.b[803])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if (s.b[735] && (!s.b[802])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if s.b[735] {s.copy_ad(343, 349);s.copy_ad(344, 350);s.copy_ad(345, 351);s.store_scalar(430, 0.0);s.store_scalar(611, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
    ) {
        let mut tc: usize = 0;
        while {
            let tb: f64 = if (s.b[735] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;assert!(tc <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[735] {s.store_sub(865, 351, 475);s.store_mul(864, 225, 865);s.store_exp_neg_input(327, 864);}
            s.b[899] = (s.v[865] < (-1e-9));s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[899]) {s.store_mul_sqrt_mixed_ia(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(864)), (-1.0)));s.store_div_scaled_offset_numerator_indices(871, 327, (-s.v[122]), s.v[122], 357, 1.0);}
            s.b[900] = (s.v[865] > 1e-9);s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[899])) && s.b[900]) {s.store_exp(866, 864);s.store_mul_scaled_sqrt_ad_rhs(357, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(864)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(866), s.ad_value(864)), (-1.0), 1.0));s.store_div_mixed_ai(871, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(866), 1.0), s.v[122]), 357);}
            if ((s.b[735] && (!s.b[899])) && (!s.b[900])) {s.store_mul_scale_offset_indices(357, 864, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(871, 225, 474, -1.0, 0.0);}
            if s.b[735] {s.copy_ad(361, 369);s.store_mul(864, 225, 349);s.store_exp_mul(869, 225, 349);s.store_scalar(867, 1.0);s.store_sqrt_ad(868, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(869), 1.0, s.ad_value(864), 1.0, s.ad_value(867), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(898, 225, 379, A::offset(s.ad_value(869), 1.0), 2.0, 868, 2.0);s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 868, -1.0);s.store_mul_scale_offset_indices(870, 898, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(865, 350, 1.0, 349, (-1.0), 740, 1.0);s.store_mul(864, 225, 865);}
            s.b[901] = ((-s.v[864]) >= 500.0);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[901]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(864)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if (s.b[735] && (!s.b[901])) {s.store_neg(44, 864);s.store_scalar(327, 1.0);}
            let mut t8: usize = 0;
            while {
                let t7: f64 = if ((s.b[735] && (!s.b[901])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t7 != 0.0
            } {
                t8 += 1;assert!(t8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (s.b[735] && (!s.b[901])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if (s.b[735] && (!s.b[901])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if s.b[735] {s.store_exp_neg_input(327, 864);s.store_sqrt_offset_ad(866, A::add(s.ad_value(327), s.ad_value(864)), (-1.0));}
            s.b[902] = (s.v[865] < (-1e-9));s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[902]) {s.store_mul(363, 238, 866);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 866, 740, 2.0);s.store_neg(365, 364);}
            s.b[903] = (s.v[865] > 1e-9);s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[902])) && s.b[903]) {s.store_mul_scale_offset_indices(363, 866, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 866, 740, 2.0);s.store_neg(365, 364);}
            if ((s.b[735] && (!s.b[902])) && (!s.b[903])) {s.store_scaled_mul(363, 238, 864, (-0.7071067811865476));s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));s.store_neg(365, 364);}
            s.b[904] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[904]) {s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[905] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });s.b[906] = (2.0 == 1.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
            if (((s.b[735] && s.b[904]) && s.b[905]) && s.b[906]) {s.store_scalar(55, 1.0);}
            s.b[907] = (2.0 == 2.0);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[904]) && s.b[905]) && (!s.b[906])) && s.b[907]) {s.store_scalar(55, 2.0);}
            s.b[908] = (2.0 == 4.0);s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
            if (((((s.b[735] && s.b[904]) && s.b[905]) && (!s.b[906])) && (!s.b[907])) && s.b[908]) {s.store_scalar(55, 3.0);}
            s.b[909] = (2.0 == 8.0);s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && s.b[904]) && s.b[905]) && (!s.b[906])) && (!s.b[907])) && (!s.b[908])) && s.b[909]) {s.store_scalar(55, 4.0);}
            if ((s.b[735] && s.b[904]) && s.b[905]) {s.store_scalar(54, 0.0);}
            let mut ta: usize = 0;
            while {
                let t9: f64 = if (((s.b[735] && s.b[904]) && s.b[905]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t9 != 0.0
            } {
                ta += 1;assert!(ta <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[735] && s.b[904]) && s.b[905]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
            }
            if ((s.b[735] && s.b[904]) && (!s.b[905])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[735] && s.b[904]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(897, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(363, A::neg(s.ad_value(406)), -1.0, 897, 1.0);}
            if (s.b[735] && s.b[904]) {
            }
            if (s.b[735] && (!s.b[904])) {
            }
            if (s.b[735] && (!s.b[904])) {s.store_scalar(327, 1.0);}
            if s.b[735] {s.store_mul(364, 364, 327);s.store_mul(365, 365, 327);}
            s.b[910] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[910]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 363);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[911] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });s.b[912] = (2.0 == 1.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
            if (((s.b[735] && s.b[910]) && s.b[911]) && s.b[912]) {s.store_scalar(55, 1.0);}
            s.b[913] = (2.0 == 2.0);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
            if ((((s.b[735] && s.b[910]) && s.b[911]) && (!s.b[912])) && s.b[913]) {s.store_scalar(55, 2.0);}
            s.b[914] = (2.0 == 4.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
            if (((((s.b[735] && s.b[910]) && s.b[911]) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {s.store_scalar(55, 3.0);}
            s.b[915] = (2.0 == 8.0);s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && s.b[910]) && s.b[911]) && (!s.b[912])) && (!s.b[913])) && (!s.b[914])) && s.b[915]) {s.store_scalar(55, 4.0);}
            if ((s.b[735] && s.b[910]) && s.b[911]) {s.store_scalar(54, 0.0);}
            let mut t6: usize = 0;
            while {
                let t5: f64 = if (((s.b[735] && s.b[910]) && s.b[911]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t5 != 0.0
            } {
                t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[735] && s.b[910]) && s.b[911]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
            }
            if ((s.b[735] && s.b[910]) && (!s.b[911])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[735] && s.b[910]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(897, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(363, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 897);}
            if (s.b[735] && s.b[910]) {
            }
            if (s.b[735] && (!s.b[910])) {
            }
            if (s.b[735] && (!s.b[910])) {s.store_scalar(327, 1.0);}
            if s.b[735] {s.store_mul(365, 365, 327);s.store_mul(364, 364, 327);s.store_add(356, 361, 363);}
            s.b[916] = (s.v[430] == 1.0);s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[916]) {s.copy_ad(611, 167);s.store_scalar(167, s.v[57]);}
            if (s.b[735] && (!s.b[916])) {s.store_add_scaled_inputs_product_mixed_iiia(875, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(876, 1.0, 324, A::add(s.ad_value(870), s.ad_value(365)), 1.0);s.store_mul_scale_offset_indices(877, 364, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(878, 871, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(865, 349, 1.0, 737, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);s.store_mul(867, 737, 871);s.store_sub(879, 350, 865);s.store_scalar(880, (-1.0));s.store_scalar(881, 1.0);s.store_neg(882, 867);s.store_add_scaled_inputs3_indices(883, 351, 1.0, 350, (-1.0), 357, (-s.v[94]));s.store_scalar(884, (-1.0));s.store_sub_from_scalar_scaled_input(885, 1.0, 871, s.v[94]);s.store_add_scaled_inputs4(886, A::mul3(s.ad_value(876), s.ad_value(881), s.ad_value(885)), 1.0, A::mul3(s.ad_value(876), s.ad_value(882), s.ad_value(884)), (-1.0), A::mul3(s.ad_value(877), s.ad_value(880), s.ad_value(885)), -1.0, A::mul3(s.ad_value(878), s.ad_value(880), s.ad_value(884)), 1.0);s.store_div_from_scalar_offset_input(887, 1.0, 886, 1e-50);s.store_add_scaled_products_indices(888, 881, 885, 1.0, 882, 884, (-1.0));s.store_add_scaled_products_indices(889, 878, 884, 1.0, 877, 885, (-1.0));s.store_add_scaled_products_indices(890, 877, 882, 1.0, 878, 881, (-1.0));s.store_mul_scale_offset_indices(891, 885, 880, -1.0, 0.0);s.store_mul(892, 876, 885);s.store_add_scaled_products_indices(893, 878, 880, 1.0, 876, 882, (-1.0));s.store_primal_mul(894, 880, 884);s.store_mul_scale_offset_indices(895, 884, 876, -1.0, 0.0);s.store_add_scaled_products_indices(896, 876, 881, 1.0, 877, 880, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(872, 887, 888, 875, -1.0, 889, 879, -1.0, 890, 883, -1.0);s.store_mul_add_scaled_products3_indices_rhs(873, 887, 891, 875, -1.0, 892, 879, -1.0, 893, 883, -1.0);s.store_mul_add_scaled_products3_indices_rhs(874, 887, 894, 875, -1.0, 895, 879, -1.0, 896, 883, -1.0);s.store_abs(865, 872);}
            s.b[917] = (s.v[865] < ((s.v[873]) as f64).abs());s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[916])) && s.b[917]) {s.store_abs(865, 873);}
            s.b[918] = (s.v[865] < ((s.v[874]) as f64).abs());s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[916])) && s.b[918]) {s.store_abs(865, 874);}
            if (s.b[735] && (!s.b[916])) {s.store_scalar(407, 1.0);}
            s.b[919] = (s.v[167] > 80.0);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[916])) && s.b[919]) {s.store_scalar(407, 125.0);}
            s.b[920] = (s.v[167] > 40.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[916])) && (!s.b[919])) && s.b[920]) {s.store_scalar(407, 125.0);}
            s.b[921] = (s.v[167] > 20.0);s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
            if ((((s.b[735] && (!s.b[916])) && (!s.b[919])) && (!s.b[920])) && s.b[921]) {s.store_scalar(407, 25.0);}
            s.b[922] = (s.v[167] > 10.0);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[916])) && (!s.b[919])) && (!s.b[920])) && (!s.b[921])) && s.b[922]) {s.store_scalar(407, 5.0);}
            s.b[923] = (s.v[865] > (0.1 / s.v[407]));s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[916])) && s.b[923]) {s.store_mul_mixed_ia(872, 872, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(865), 1.0));s.store_mul_mixed_ia(873, 873, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(865), 1.0));s.store_mul_mixed_ia(874, 874, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(865), 1.0));}
            if (s.b[735] && (!s.b[916])) {s.store_add(349, 349, 872);s.store_add(350, 350, 873);s.store_add(351, 351, 874);s.store_primal_scale(408, 407, 5e-12);}
            s.b[924] = (s.v[865] < s.v[408]);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[916])) && s.b[924]) {s.store_scalar(430, 1.0);}
            if s.b[735] {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if s.b[735] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }
        s.b[925] = (s.v[430] == 0.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[925]) {s.copy_ad(349, 343);s.copy_ad(350, 344);s.copy_ad(351, 345);}
        if s.b[735] {s.copy_ad(161, 349);s.store_neg(244, 355);}
        s.b[926] = (s.v[244] <= 1e-50);s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[926]) {s.store_scalar(244, 1e-50);}
        if s.b[735] {s.store_mul(192, 244, 324);}
        s.b[927] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[927]) {s.store_scale(327, 108, (-s.v[98]));s.copy_ad(362, 369);s.copy_ad(366, 363);s.store_add(359, 362, 366);s.store_scaled_add(437, 359, 356, (-0.5));s.store_mul(196, 327, 437);s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_scalar(197, 0.0);s.store_scaled_mul(392, 357, 108, s.v[98]);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);s.store_scalar(145, 1.0);s.copy_ad(352, 349);s.copy_ad(353, 350);s.copy_ad(354, 351);s.copy_ad(360, 357);s.copy_ad(162, 161);s.copy_ad(314, 162);}
        if (s.b[735] && (!s.b[927])) {s.copy_ad(453, 157);s.store_scalar(934, 1e-50);s.store_div_square_rhs(929, 545, 323);s.store_offset_mul_ad(931, A::div_from_scalar(2.0, s.ad_value(929)), A::sub(s.ad_value(159), s.ad_value(934)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(929), 1.0);}
        s.b[935] = ((s.v[931] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[935]) {s.store_sub(44, 332, 931);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[936] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });s.b[937] = (4.0 == 1.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) && s.b[937]) {s.store_scalar(55, 1.0);}
        s.b[938] = (4.0 == 2.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) && (!s.b[937])) && s.b[938]) {s.store_scalar(55, 2.0);}
        s.b[939] = (4.0 == 4.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) && (!s.b[937])) && (!s.b[938])) && s.b[939]) {s.store_scalar(55, 3.0);}
        s.b[940] = (4.0 == 8.0);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) && (!s.b[937])) && (!s.b[938])) && (!s.b[939])) && s.b[940]) {s.store_scalar(55, 4.0);}
        if (((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) {s.store_scalar(54, 0.0);}
        let mut te: usize = 0;
        while {
            let td: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;assert!(te <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[935]) && s.b[936]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((s.b[735] && (!s.b[927])) && s.b[935]) && (!s.b[936])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((s.b[735] && (!s.b[927])) && s.b[935]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);s.store_sub(931, 332, 43);}
        if ((s.b[735] && (!s.b[927])) && (!s.b[935])) {
        }
        if (s.b[735] && (!s.b[927])) {s.store_sqrt(930, 931);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[735] && (!s.b[927])) {s.store_add_mul_sub_from_scalar_rhs_indices(934, 159, 929, 1.0, 930);s.store_sqrt_square_offset(44, 934, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(934, 934, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[941] = (s.v[934] < 0.0);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[941]) {s.store_scalar(934, 0.0);}
        if (s.b[735] && (!s.b[927])) {s.store_div(928, 157, 934);s.store_pow_offset_rhs(929, 928, 138, (-1.0));s.store_mul(933, 929, 928);s.store_offset(930, 933, 1.0);s.store_pow_ad(931, s.ad_value(930), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(932, 931, 930);s.store_div(452, 157, 932);s.copy_ad(157, 452);}
        s.b[942] = (s.v[157] < 0.0);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[942]) {s.copy_ad(162, 161);s.store_sub(164, 162, 161);s.copy_ad(352, 162);s.copy_ad(353, 350);s.copy_ad(354, 351);s.store_scalar(430, 1.0);}
        s.b[943] = (s.v[144] >= 1.0);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && s.b[943]) {s.store_scalar(352, s.v[622]);s.store_scalar(353, s.v[623]);s.store_scalar(354, s.v[624]);}
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[944] = (s.v[165] < 0.0);s.store_scalar(944, if s.b[944] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[944]) {s.store_scalar(165, 0.0);}
        s.b[945] = (s.v[165] > s.v[157]);s.store_scalar(945, if s.b[945] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[944])) && s.b[945]) {s.copy_ad(165, 157);}
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {s.copy_ad(164, 165);s.store_add(162, 349, 164);s.copy_ad(352, 162);s.copy_ad(388, 390);s.store_scaled_square(946, 474, (s.v[95] * s.v[95]));}
        s.b[952] = (s.v[352] < s.v[385]);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) {s.store_neg(947, 475);s.store_add_scaled_inputs3_mixed_aai(948, A::square(A::add_scaled_product(s.ad_value(947), 2.0, s.ad_value(946), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(947)), (-4.0), 946, (-4.0));}
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) {
            if (s.v[948] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(948, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) {s.store_sqrt(948, 948);s.store_add_scaled_product_indices(949, 947, 2.0, 946, 225, 1.0);s.store_scaled_sub(950, 949, 948, 0.5);s.store_div_ad(951, A::ln(A::div_scaled_product_by_product(s.ad_value(947), s.ad_value(947), 1.0, s.ad_value(946), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(947))));}
        s.b[953] = (s.v[950] < s.v[382]);s.store_scalar(953, if s.b[953] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) && s.b[953]) {s.copy_ad(354, 950);}
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) && (!s.b[953])) {s.store_offset_sub(44, 951, 950, (-0.0008));s.store_scale(45, 951, (4.0 * 0.0008));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) && (!s.b[953])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[952]) && (!s.b[953])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 951, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) {s.store_neg_ad(947, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(738), (-(1.0 / (2.0) * 9662367879.197212))));s.store_add_scaled_inputs3_mixed_aai(948, A::square(A::add_scaled_product(s.ad_value(947), 2.0, s.ad_value(946), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(947)), (-4.0), 946, (-4.0));}
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) {
            if (s.v[948] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(948, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) {s.store_sqrt(948, 948);s.store_add_scaled_product_indices(949, 947, 2.0, 946, 225, 1.0);s.store_scaled_sub(950, 949, 948, 0.5);s.store_div_ad(951, A::ln(A::div_scaled_product_by_product(s.ad_value(947), s.ad_value(947), 1.0, s.ad_value(946), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(947))));}
        s.b[954] = (s.v[950] < s.v[382]);s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) && s.b[954]) {s.copy_ad(354, 950);}
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) && (!s.b[954])) {s.store_offset_sub(44, 951, 950, (-0.0008));s.store_scale(45, 951, (4.0 * 0.0008));}
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) && (!s.b[954])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[952])) && (!s.b[954])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 951, 1.0, 44, (-0.5), 45, (-0.5));}
        if (((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) {s.store_div_scaled_inputs_indices(955, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[963] = (s.v[955] > 0.0);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[963]) {s.store_sqrt_div_scaled_inputs(401, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[963])) {s.store_scalar(401, 0.0);}
        s.b[964] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.store_scalar(168, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        let mut t10: usize = 0;
        while {
            let tf: f64 = if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.copy_ad(956, 474);s.store_mul(957, 225, 354);s.store_exp_neg_input(958, 957);}
            s.b[965] = (s.v[354] > 1e-9);s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && s.b[965]) {s.store_exp_mul(955, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(959, 956, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(955), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(960, s.v[122], 959, A::add_scaled_sub_value_product(1.0, s.ad_value(958), 1.0, s.ad_value(239), s.ad_value(955), 1.0));}
            s.b[966] = (s.v[354] < (-1e-9));s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && (!s.b[965])) && s.b[966]) {s.store_mul_sqrt_mixed_ia(959, 956, A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)));s.store_mul_scale_offset_mixed_ai(960, A::div_from_scalar(s.v[122], s.ad_value(959)), 958, -1.0, 1.0);}
            if ((((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && (!s.b[965])) && (!s.b[966])) {s.store_mul_ad_affine_product_lhs(959, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(960, 225, s.v[122], -1.0);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.store_sqrt_add_scaled_square_product(45, 959, 1.0, 741, 741, 4.0);s.store_offset_scaled_div(962, 959, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(961, 959, 0.5, 45, 0.5, 741, 1e-10);}
            s.b[967] = (s.v[961] < 0.0);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && s.b[967]) {s.store_scalar(961, 0.0);s.store_scalar(962, 0.0);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 961, (-1.0), 742, -1.0);s.store_scaled_mul(45, 341, 742, (-4.0));}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(961, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(962, 962, 960, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(961)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 962, 2.0, 961, 1.0);s.store_sub_mixed_ia(961, 354, A::div_scaled_inputs4(s.ad_value(959), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(960), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));}
            s.b[968] = ((((s.v[961] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) && s.b[968]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.copy_ad(354, 961);s.copy_ad(360, 959);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[964]) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.store_scalar(168, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        let mut t12: usize = 0;
        while {
            let t11: f64 = if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;assert!(t12 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.copy_ad(956, 474);s.store_mul(957, 225, 354);s.store_exp_neg_input(958, 957);}
            s.b[969] = (s.v[354] > 1e-9);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && s.b[969]) {s.store_exp_mul(955, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(959, 956, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(955), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(960, s.v[122], 959, A::add_scaled_sub_value_product(1.0, s.ad_value(958), 1.0, s.ad_value(239), s.ad_value(955), 1.0));}
            s.b[970] = (s.v[354] < (-1e-9));s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && (!s.b[969])) && s.b[970]) {s.store_mul_sqrt_mixed_ia(959, 956, A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)));s.store_mul_scale_offset_mixed_ai(960, A::div_from_scalar(s.v[122], s.ad_value(959)), 958, -1.0, 1.0);}
            if ((((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && (!s.b[969])) && (!s.b[970])) {s.store_mul_ad_affine_product_lhs(959, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(960, 225, s.v[122], -1.0);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.store_sqrt_add_scaled_square_product(45, 959, 1.0, 741, 741, 4.0);s.store_offset_scaled_div(962, 959, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(961, 959, 0.5, 45, 0.5, 741, 1e-10);}
            s.b[971] = (s.v[961] < 0.0);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && s.b[971]) {s.store_scalar(961, 0.0);s.store_scalar(962, 0.0);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 961, (-1.0), 742, -1.0);s.store_scaled_mul(45, 341, 742, (-4.0));}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(961, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(962, 962, 960, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(961)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 962, 2.0, 961, 1.0);s.store_sub_mixed_ia(961, 354, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(959), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(959), 1.0, s.ad_value(341), 0.5), s.ad_value(738), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(960), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(960), s.ad_value(738), 9662367879.197212), s.ad_value(389)), 1.0));}
            s.b[972] = ((((s.v[961] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) && s.b[972]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.copy_ad(354, 961);s.copy_ad(360, 959);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && (!s.b[964])) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        s.b[973] = (s.v[353] < 0.0);s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && (!s.b[942])) && (!s.b[943])) && s.b[973]) {s.store_scalar(353, 0.0);}
        s.b[1009] = (s.v[349] < 0.0);s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1009]) {s.copy_ad(352, 349);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        s.b[1010] = (s.v[353] < 0.01);s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1010]) {s.store_add_scaled_product_mixed_iia(353, 352, 1.0, 737, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
        if (s.b[735] && (!s.b[927])) {s.copy_ad(346, 352);s.copy_ad(347, 353);s.copy_ad(348, 354);s.store_scalar(430, 0.0);s.store_scalar(611, 0.0);s.store_scalar(168, 1.0);}
    }
}
