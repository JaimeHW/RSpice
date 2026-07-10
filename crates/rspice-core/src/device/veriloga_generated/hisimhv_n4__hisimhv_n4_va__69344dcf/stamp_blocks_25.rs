#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_181(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);}
        s.b[3175] = (s.v[411] > 0.0);s.store_scalar(3175, if s.b[3175] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3175]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[3176] = (s.v[336] < 0.0);s.store_scalar(3176, if s.b[3176] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) && s.b[3176]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3177] = (s.v[336] < 0.0);s.store_scalar(3177, if s.b[3177] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3177]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_182(
        s: &mut ReactiveScratch,
    ) {
        let mut t4: usize = 0;
        while {
            let t2: f64 = (s.v[421] + 1.0);let t3: f64 = if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (s.v[97] <= t2)) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;assert!(t4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[3179] = (s.v[333] < 60.0);s.store_scalar(3179, if s.b[3179] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3179]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3179])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_mul(415, 154, 416);}
            s.b[3180] = (s.v[116] < 0.0);s.store_scalar(3180, if s.b[3180] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3180]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[3181] = (s.v[116] < 1e-6);s.store_scalar(3181, if s.b[3181] { 1.0 } else { 0.0 });
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[3182] = (s.v[338] > 0.0);s.store_scalar(3182, if s.b[3182] { 1.0 } else { 0.0 });
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) && s.b[3182]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) && (!s.b[3182])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[3183] = (s.v[338] > 0.0);s.store_scalar(3183, if s.b[3183] { 1.0 } else { 0.0 });
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) && s.b[3183]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) && (!s.b[3183])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[3184] = (s.v[116] < 0.0);s.store_scalar(3184, if s.b[3184] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3184]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[3185] = (s.v[116] < 60.0);s.store_scalar(3185, if s.b[3185] { 1.0 } else { 0.0 });s.b[3186] = (s.v[116] < 5e-5);s.store_scalar(3186, if s.b[3186] { 1.0 } else { 0.0 });
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3185]) && s.b[3186]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3185]) && (!s.b[3186])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && (!s.b[3185])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[3187] = (s.v[214] > 0.0);s.store_scalar(3187, if s.b[3187] { 1.0 } else { 0.0 });
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3187]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && (!s.b[3187])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[3188] = (s.v[79] == 1.0);s.store_scalar(3188, if s.b[3188] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3188]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3189] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(3189, if s.b[3189] { 1.0 } else { 0.0 });
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) && s.b[3189]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {s.store_add(404, 404, 236);}
            s.b[3190] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(3190, if s.b[3190] { 1.0 } else { 0.0 });
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) && s.b[3190]) {s.store_scalar(79, 1.0);}
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_183(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_mul(3109, 982, 223);s.store_mul(3110, 3111, 3109);s.store_offset_div(100, 3110, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[3192] = (p.p33 == 4.0);s.store_scalar(3192, if s.b[3192] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 3118);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);}
        s.b[3193] = (s.v[411] > 0.0);s.store_scalar(3193, if s.b[3193] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3193]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[3194] = (s.v[336] < 0.0);s.store_scalar(3194, if s.b[3194] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) && s.b[3194]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3195] = (s.v[336] < 0.0);s.store_scalar(3195, if s.b[3195] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3195]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_184(
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t5: f64 = (s.v[421] + 1.0);let t6: f64 = if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (s.v[97] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[3197] = (s.v[333] < 60.0);s.store_scalar(3197, if s.b[3197] { 1.0 } else { 0.0 });
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3197]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3197])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_mul(415, 154, 416);}
            s.b[3198] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(3198, if s.b[3198] { 1.0 } else { 0.0 });
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3198]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(3119, 334, 336);s.store_mul_add_scaled_product_rhs_indices(3120, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3198])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(3119, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(3120, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[3199] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(3199, if s.b[3199] { 1.0 } else { 0.0 });
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3199]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[3200] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(3200, if s.b[3200] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3199])) && s.b[3200]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3199])) && (!s.b[3200])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[3201] = (s.v[214] > 0.0);s.store_scalar(3201, if s.b[3201] { 1.0 } else { 0.0 });
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3201]) {s.store_sqrt_add(216, 3119, 214);s.store_div_scaled_inputs2_indices(217, 3120, 0.5, 215, 0.5, 216, 1.0);}
            s.b[3202] = (s.v[3119] > 0.0);s.store_scalar(3202, if s.b[3202] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3201])) && s.b[3202]) {s.store_sqrt(216, 3119);s.store_div_scaled_inputs_indices(217, 3120, 0.5, 216, 1.0);}
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3201])) && (!s.b[3202])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[3203] = (s.v[79] > 0.0);s.store_scalar(3203, if s.b[3203] { 1.0 } else { 0.0 });
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3203]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3204] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(3204, if s.b[3204] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) && s.b[3204]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {s.store_add(404, 404, 236);}
            s.b[3205] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(3205, if s.b[3205] { 1.0 } else { 0.0 });
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) && s.b[3205]) {s.store_primal_offset(79, 79, 2.0);}
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_185(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            if (s.v[3119] >= 0.0) {
                s.store_scaled_sqrt(223, 3119, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_mul(3109, 982, 223);s.store_mul(3110, 3111, 3109);s.store_offset_div(100, 3110, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.b[3107] && s.b[3108]) {s.store_sub(399, 398, 354);}
        s.b[3207] = (s.v[407] < 0.0);s.store_scalar(3207, if s.b[3207] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3207]) {s.store_neg(407, 407);}
        s.b[3208] = (p.p55 == 0.0);s.store_scalar(3208, if s.b[3208] { 1.0 } else { 0.0 });s.b[3209] = (p.p50 == 0.0);s.store_scalar(3209, if s.b[3209] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3209]) {s.store_neg(3112, 404);}
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && (!s.b[3209])) {s.copy_ad(3112, 396);}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_sqrt_offset_square_offset(782, 3112, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3112), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(3112), p.p137), 782, 0.5);}
        s.b[3210] = (s.v[336] < 0.0);s.store_scalar(3210, if s.b[3210] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3210]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        if (s.b[3107] && s.b[3108]) {s.copy_ad(698, 354);}
        s.b[3211] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] == 0.0));s.store_scalar(3211, if s.b[3211] { 1.0 } else { 0.0 });
        if s.b[3211] {s.store_scalar(2619, 1.0);s.store_scalar(289, s.v[564]);s.store_scalar(290, p.p276);s.store_scalar(335, (s.v[188] * s.v[635]));}
        s.b[3212] = (s.v[949] == 1.0);s.store_scalar(3212, if s.b[3212] { 1.0 } else { 0.0 });
        if (s.b[3211] && s.b[3212]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p.p66);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(291, 791, 339, 1.0, 338, 343, (-1.0));}
        if (s.b[3211] && (!s.b[3212])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p.p66);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(291, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
        s.b[3213] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] == 0.0));s.store_scalar(3213, if s.b[3213] { 1.0 } else { 0.0 });
        if s.b[3213] {s.store_scalar(2622, 1.0);s.store_scalar(289, s.v[564]);s.store_scalar(290, p.p276);s.store_scale(335, 412, s.v[635]);}
        s.b[3214] = (s.v[949] == 1.0);s.store_scalar(3214, if s.b[3214] { 1.0 } else { 0.0 });
        if (s.b[3213] && s.b[3214]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p.p63);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(292, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_186(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3213] && (!s.b[3214])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p.p63);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(292, 791, 339, 1.0, 338, 343, (-1.0));}
        if s.b[768] {s.store_scalar(295, (s.v[505] * (-s.v[635])));}
        s.b[3215] = (s.v[2619] == 0.0);s.store_scalar(3215, if s.b[3215] { 1.0 } else { 0.0 });
        if ((!s.b[768]) && s.b[3215]) {s.store_scalar(295, (((-s.v[188]) * p.p66) * s.v[635]));}
        s.store_mul_scale_offset_indices(297, 734, 295, -1.0, 0.0);
        if s.b[769] {s.store_scalar(294, (s.v[506] * (-s.v[635])));}
        s.b[3216] = (s.v[2622] == 0.0);s.store_scalar(3216, if s.b[3216] { 1.0 } else { 0.0 });
        if ((!s.b[769]) && s.b[3216]) {s.store_primal_scale(294, 412, (-(p.p63 * s.v[635])));}
        s.store_mul_sub_scaled_inputs_rhs_indices(298, 294, 734, -1.0, 733, -1.0);s.b[3217] = (s.v[949] == 1.0);s.store_scalar(3217, if s.b[3217] { 1.0 } else { 0.0 });
        if s.b[3217] {s.store_scaled_sub(357, 790, 94, p.p431);s.store_mul(360, 338, 357);s.store_mul(361, 338, 357);}
        if (!s.b[3217]) {s.store_scaled_sub(357, 790, 94, (-p.p431));s.store_mul(362, 338, 357);s.store_mul(363, 338, 357);}
        s.store_scalar(296, ((-s.v[525]) * s.v[582]));s.store_scaled_sub(293, 731, 728, (-s.v[296]));s.store_scalar(172, s.v[507]);s.b[3218] = (s.v[78] != 0.0);s.store_scalar(3218, if s.b[3218] { 1.0 } else { 0.0 });
        if s.b[3218] {s.store_add_scaled_inputs3_indices(168, 790, s.v[172], 87, s.v[172], 91, (1.0 - s.v[172]));}
        s.b[3219] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(3219, if s.b[3219] { 1.0 } else { 0.0 });
        if (s.b[3218] && s.b[3219]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3220] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3220, if s.b[3220] { 1.0 } else { 0.0 });s.b[3221] = (2.0 == 1.0);s.store_scalar(3221, if s.b[3221] { 1.0 } else { 0.0 });
        if (((s.b[3218] && s.b[3219]) && s.b[3220]) && s.b[3221]) {s.store_scalar(720, 1.0);}
        s.b[3222] = (2.0 == 2.0);s.store_scalar(3222, if s.b[3222] { 1.0 } else { 0.0 });
        if ((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && s.b[3222]) {s.store_scalar(720, 2.0);}
        s.b[3223] = (2.0 == 4.0);s.store_scalar(3223, if s.b[3223] { 1.0 } else { 0.0 });
        if (((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && s.b[3223]) {s.store_scalar(720, 3.0);}
        s.b[3224] = (2.0 == 8.0);s.store_scalar(3224, if s.b[3224] { 1.0 } else { 0.0 });
        if ((((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && (!s.b[3223])) && s.b[3224]) {s.store_scalar(720, 4.0);}
        if ((s.b[3218] && s.b[3219]) && s.b[3220]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((s.b[3218] && s.b[3219]) && s.b[3220]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3218] && s.b[3219]) && s.b[3220]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((s.b[3218] && s.b[3219]) && (!s.b[3220])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (s.b[3218] && s.b[3219]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (s.b[3218] && s.b[3219]) {
        }
        if (s.b[3218] && (!s.b[3219])) {
        }
        if (s.b[3218] && (!s.b[3219])) {s.store_scalar(334, 1.0);}
        if (s.b[3218] && s.b[82]) {s.store_scalar(303, 0.0);}
        s.b[3225] = ((s.v[248] < 1e-15) || (s.v[348] < 1e-6));s.store_scalar(3225, if s.b[3225] { 1.0 } else { 0.0 });
        if (((!s.b[3218]) && s.b[82]) && s.b[3225]) {s.store_scalar(303, 0.0);}
        if (((!s.b[3218]) && s.b[82]) && (!s.b[3225])) {s.store_div_scaled_product_by_product_indices(303, 248, 155, 1.0, 238, 162, 1.0);}
        s.b[3226] = (!s.b[82]);s.store_scalar(3226, if s.b[3226] { 1.0 } else { 0.0 });
        if s.b[3226] {s.store_scalar(305, 0.0);}
        if (!s.b[3226]) {s.store_scale(336, 684, ((1.034943e-10 * s.v[635]) * 1.3));}
        s.b[3227] = (p.p133 != 0.0);s.store_scalar(3227, if s.b[3227] { 1.0 } else { 0.0 });
        if ((!s.b[3226]) && s.b[3227]) {s.store_add_scaled_product_indices(304, 87, 1.0, 303, 162, 1.0);s.store_add_scaled_inputs3_indices(335, 1435, s.v[172], 87, s.v[172], 304, (1.0 - s.v[172]));s.store_mul_scale_offset_mixed_ia(305, 336, A::add_scaled_inputs3(s.ad_value(87), 1.0, s.ad_value(1435), 1.0, s.ad_value(335), -1.0), (-1.0 / (p.p133)), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_187(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[3228] = (p.p134 != 0.0);s.store_scalar(3228, if s.b[3228] { 1.0 } else { 0.0 });
        if ((!s.b[3226]) && s.b[3228]) {s.store_add_scaled_inputs(305, 305, 1.0, 792, s.v[671]);}
        s.store_scalar(300, s.v[670]);s.store_scalar(302, s.v[670]);s.store_scaled_sub(299, 734, 733, s.v[300]);s.store_scale(301, 734, s.v[302]);s.b[3229] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(3229, if s.b[3229] { 1.0 } else { 0.0 });
        if s.b[3229] {s.store_square(334, 676);s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);}
        s.b[3230] = (p.p48 > 0.0);s.store_scalar(3230, if s.b[3230] { 1.0 } else { 0.0 });s.b[3231] = (p.p15 > s.v[632]);s.store_scalar(3231, if s.b[3231] { 1.0 } else { 0.0 });
        if ((s.b[3229] && s.b[3230]) && s.b[3231]) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scale(875, 829, (p.p15 - s.v[632]));s.store_scale(876, 831, (p.p15 - s.v[632]));s.store_scale(877, 836, s.v[632]);s.store_scale(878, 837, s.v[632]);}
        if ((s.b[3229] && s.b[3230]) && (!s.b[3231])) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scale(877, 836, p.p15);s.store_scale(878, 837, p.p15);}
        if (s.b[3229] && (!s.b[3230])) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scale(875, 829, p.p15);s.store_scale(876, 831, p.p15);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);}
        if s.b[3229] {s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);}
        s.b[3232] = (s.v[847] > 0.0);s.store_scalar(3232, if s.b[3232] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3232]) {s.store_offset(336, 847, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));s.store_exp_mul(851, 848, 850);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_188(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[3229] {s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);}
        s.b[3233] = (p.p48 > 0.0);s.store_scalar(3233, if s.b[3233] { 1.0 } else { 0.0 });s.b[3234] = (p.p16 > s.v[632]);s.store_scalar(3234, if s.b[3234] { 1.0 } else { 0.0 });
        if ((s.b[3229] && s.b[3233]) && s.b[3234]) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scale(881, 829, (p.p16 - s.v[632]));s.store_scale(882, 831, (p.p16 - s.v[632]));s.store_scale(883, 836, s.v[632]);s.store_scale(884, 837, s.v[632]);}
        if ((s.b[3229] && s.b[3233]) && (!s.b[3234])) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scale(883, 836, p.p16);s.store_scale(884, 837, p.p16);}
        if (s.b[3229] && (!s.b[3233])) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scale(881, 829, p.p16);s.store_scale(882, 831, p.p16);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);}
        if s.b[3229] {s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);}
        s.b[3235] = (s.v[852] > 0.0);s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3235]) {s.store_offset(337, 852, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));s.store_exp_mul(856, 853, 855);}
        if s.b[3229] {s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));}
        s.b[3236] = (p.p15 > s.v[632]);s.store_scalar(3236, if s.b[3236] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3236]) {s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));}
        if (s.b[3229] && (!s.b[3236])) {s.store_scalar(833, 0.0);s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));}
        s.b[3237] = (s.v[832] < 0.0);s.store_scalar(3237, if s.b[3237] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3237]) {s.store_scalar(832, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_189(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.b[3238] = (s.v[833] < 0.0);s.store_scalar(3238, if s.b[3238] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3238]) {s.store_scalar(833, 0.0);}
        s.b[3239] = (s.v[834] < 0.0);s.store_scalar(3239, if s.b[3239] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3239]) {s.store_scalar(834, 0.0);}
        if s.b[3229] {s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);}
        s.b[3240] = ((s.v[841] < 0.01) && (p.p13 > 0.0));s.store_scalar(3240, if s.b[3240] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3240]) {s.store_scalar(841, 0.01);}
        s.b[3241] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));s.store_scalar(3241, if s.b[3241] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3241]) {s.store_scalar(842, 0.01);}
        s.b[3242] = ((s.v[843] < 0.01) && (p.p15 > 0.0));s.store_scalar(3242, if s.b[3242] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3242]) {s.store_scalar(843, 0.01);}
        if s.b[3229] {s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));}
        s.b[3243] = (p.p16 > s.v[632]);s.store_scalar(3243, if s.b[3243] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3243]) {s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));}
        if (s.b[3229] && (!s.b[3243])) {s.store_scalar(838, 0.0);s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));}
        s.b[3244] = (s.v[835] < 0.0);s.store_scalar(3244, if s.b[3244] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3244]) {s.store_scalar(835, 0.0);}
        s.b[3245] = (s.v[838] < 0.0);s.store_scalar(3245, if s.b[3245] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3245]) {s.store_scalar(838, 0.0);}
        s.b[3246] = (s.v[839] < 0.0);s.store_scalar(3246, if s.b[3246] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3246]) {s.store_scalar(839, 0.0);}
        if s.b[3229] {s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);}
        s.b[3247] = ((s.v[844] < 0.01) && (p.p14 > 0.0));s.store_scalar(3247, if s.b[3247] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3247]) {s.store_scalar(844, 0.01);}
        s.b[3248] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));s.store_scalar(3248, if s.b[3248] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3248]) {s.store_scalar(845, 0.01);}
        s.b[3249] = ((s.v[846] < 0.01) && (p.p16 > 0.0));s.store_scalar(3249, if s.b[3249] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3249]) {s.store_scalar(846, 0.01);}
        if (!s.b[3229]) {s.store_scalar(387, (ctx_temp + p.p11));}
        s.store_scale(344, 850, p.p511);s.store_scale(343, 849, p.p510);s.b[3250] = (s.v[873] > 0.0);s.store_scalar(3250, if s.b[3250] { 1.0 } else { 0.0 });
        if s.b[3250] {s.store_mul(334, 874, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3251] = (s.v[860] < s.v[848]);s.store_scalar(3251, if s.b[3251] { 1.0 } else { 0.0 });
        if (s.b[3250] && s.b[3251]) {s.store_mul(332, 860, 850);}
        s.b[3252] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3252, if s.b[3252] { 1.0 } else { 0.0 });
        if ((s.b[3250] && s.b[3251]) && s.b[3252]) {s.store_scalar(335, 0.0);}
        if ((s.b[3250] && s.b[3251]) && (!s.b[3252])) {s.store_exp(335, 332);}
        if (s.b[3250] && (!s.b[3251])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 873, 850, 335);}
        s.store_scale(346, 874, p.p514);s.b[3253] = (s.v[875] > 0.0);s.store_scalar(3253, if s.b[3253] { 1.0 } else { 0.0 });
        if s.b[3253] {s.store_mul(334, 876, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3254] = (s.v[860] < s.v[848]);s.store_scalar(3254, if s.b[3254] { 1.0 } else { 0.0 });
        if (s.b[3253] && s.b[3254]) {s.store_mul(332, 860, 850);}
        s.b[3255] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3255, if s.b[3255] { 1.0 } else { 0.0 });
        if ((s.b[3253] && s.b[3254]) && s.b[3255]) {s.store_scalar(335, 0.0);}
        if ((s.b[3253] && s.b[3254]) && (!s.b[3255])) {s.store_exp(335, 332);}
        if (s.b[3253] && (!s.b[3254])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 875, 850, 335);}
        s.store_scale(346, 876, p.p514);s.b[3256] = (p.p48 > 0.0);s.store_scalar(3256, if s.b[3256] { 1.0 } else { 0.0 });s.b[3257] = (s.v[877] > 0.0);s.store_scalar(3257, if s.b[3257] { 1.0 } else { 0.0 });
        if (s.b[3256] && s.b[3257]) {s.store_mul(334, 878, 343);s.store_mul_scale_offset_indices(332, 344, 868, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3258] = (s.v[868] < s.v[848]);s.store_scalar(3258, if s.b[3258] { 1.0 } else { 0.0 });
        if ((s.b[3256] && s.b[3257]) && s.b[3258]) {s.store_mul(332, 868, 850);}
        s.b[3259] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3259, if s.b[3259] { 1.0 } else { 0.0 });
        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && s.b[3259]) {s.store_scalar(335, 0.0);}
        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && (!s.b[3259])) {s.store_exp(335, 332);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_190(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3256] && s.b[3257]) && (!s.b[3258])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 877, 850, 335);}
        if s.b[3256] {s.store_scale(346, 878, p.p514);}
        s.store_scale(344, 855, p.p534);s.store_scale(343, 854, p.p533);s.b[3260] = (s.v[879] > 0.0);s.store_scalar(3260, if s.b[3260] { 1.0 } else { 0.0 });
        if s.b[3260] {s.store_mul(334, 880, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3261] = (s.v[859] < s.v[853]);s.store_scalar(3261, if s.b[3261] { 1.0 } else { 0.0 });
        if (s.b[3260] && s.b[3261]) {s.store_mul(332, 859, 855);}
        s.b[3262] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3262, if s.b[3262] { 1.0 } else { 0.0 });
        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {s.store_scalar(335, 0.0);}
        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {s.store_exp(335, 332);}
        if (s.b[3260] && (!s.b[3261])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 879, 855, 335);}
        s.store_scale(346, 880, p.p537);s.b[3263] = (s.v[881] > 0.0);s.store_scalar(3263, if s.b[3263] { 1.0 } else { 0.0 });
        if s.b[3263] {s.store_mul(334, 882, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3264] = (s.v[859] < s.v[853]);s.store_scalar(3264, if s.b[3264] { 1.0 } else { 0.0 });
        if (s.b[3263] && s.b[3264]) {s.store_mul(332, 859, 855);}
        s.b[3265] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3265, if s.b[3265] { 1.0 } else { 0.0 });
        if ((s.b[3263] && s.b[3264]) && s.b[3265]) {s.store_scalar(335, 0.0);}
        if ((s.b[3263] && s.b[3264]) && (!s.b[3265])) {s.store_exp(335, 332);}
        if (s.b[3263] && (!s.b[3264])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 881, 855, 335);}
        s.store_scale(346, 882, p.p537);s.b[3266] = (p.p48 > 0.0);s.store_scalar(3266, if s.b[3266] { 1.0 } else { 0.0 });s.b[3267] = (s.v[883] > 0.0);s.store_scalar(3267, if s.b[3267] { 1.0 } else { 0.0 });
        if (s.b[3266] && s.b[3267]) {s.store_mul(334, 884, 343);s.store_mul_scale_offset_indices(332, 344, 867, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3268] = (s.v[867] < s.v[853]);s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });
        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {s.store_mul(332, 867, 855);}
        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3269, if s.b[3269] { 1.0 } else { 0.0 });
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && s.b[3269]) {s.store_scalar(335, 0.0);}
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && (!s.b[3269])) {s.store_exp(335, 332);}
        if ((s.b[3266] && s.b[3267]) && (!s.b[3268])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 883, 855, 335);}
        if s.b[3266] {s.store_scale(346, 884, p.p537);}
        s.b[3270] = (s.v[832] > 0.0);s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });s.b[3271] = (s.v[860] < 0.0);s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });
        if (s.b[3270] && s.b[3271]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 841);}
        s.b[3272] = (p.p503 == 0.5);s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });
        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p503));
            }
        }
        if (s.b[3270] && s.b[3271]) {s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p503)), 0.0);}
        if (s.b[3270] && (!s.b[3271])) {s.copy_ad(335, 832);s.store_div_scaled_inputs_indices(336, 832, p.p503, 841, 1.0);s.store_mul_add_scaled_product_rhs_indices(891, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3270]) {s.store_scalar(891, 0.0);}
        s.b[3273] = (s.v[833] > 0.0);s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });s.b[3274] = (s.v[860] < 0.0);s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });
        if (s.b[3273] && s.b[3274]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 842);}
        s.b[3275] = (p.p504 == 0.5);s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });
        if ((s.b[3273] && s.b[3274]) && s.b[3275]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3273] && s.b[3274]) && (!s.b[3275])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p504));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_191(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3273] && s.b[3274]) {s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p504)), 0.0);}
        if (s.b[3273] && (!s.b[3274])) {s.copy_ad(335, 833);s.store_div_scaled_inputs_indices(336, 833, p.p504, 842, 1.0);s.store_mul_add_scaled_product_rhs_indices(893, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3273]) {s.store_scalar(893, 0.0);}
        s.b[3276] = (p.p48 > 0.0);s.store_scalar(3276, if s.b[3276] { 1.0 } else { 0.0 });s.b[3277] = (s.v[834] > 0.0);s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });s.b[3278] = (s.v[868] < 0.0);s.store_scalar(3278, if s.b[3278] { 1.0 } else { 0.0 });
        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);}
        s.b[3279] = (p.p505 == 0.5);s.store_scalar(3279, if s.b[3279] { 1.0 } else { 0.0 });
        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && s.b[3279]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && (!s.b[3279])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p505));
            }
        }
        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);}
        if ((s.b[3276] && s.b[3277]) && (!s.b[3278])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p.p505, 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 868, 335, 1.0, 868, 336, 0.5);}
        if (s.b[3276] && (!s.b[3277])) {s.store_scalar(895, 0.0);}
        s.b[3280] = (s.v[834] > 0.0);s.store_scalar(3280, if s.b[3280] { 1.0 } else { 0.0 });s.b[3281] = (s.v[860] < 0.0);s.store_scalar(3281, if s.b[3281] { 1.0 } else { 0.0 });
        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 843);}
        s.b[3282] = (p.p505 == 0.5);s.store_scalar(3282, if s.b[3282] { 1.0 } else { 0.0 });
        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && s.b[3282]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && (!s.b[3282])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p505));
            }
        }
        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);}
        if (((!s.b[3276]) && s.b[3280]) && (!s.b[3281])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p.p505, 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 860, 335, 1.0, 860, 336, 0.5);}
        if ((!s.b[3276]) && (!s.b[3280])) {s.store_scalar(895, 0.0);}
        s.b[3283] = (s.v[835] > 0.0);s.store_scalar(3283, if s.b[3283] { 1.0 } else { 0.0 });s.b[3284] = (s.v[859] < 0.0);s.store_scalar(3284, if s.b[3284] { 1.0 } else { 0.0 });
        if (s.b[3283] && s.b[3284]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 844);}
        s.b[3285] = (p.p526 == 0.5);s.store_scalar(3285, if s.b[3285] { 1.0 } else { 0.0 });
        if ((s.b[3283] && s.b[3284]) && s.b[3285]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3283] && s.b[3284]) && (!s.b[3285])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p526));
            }
        }
        if (s.b[3283] && s.b[3284]) {s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p526)), 0.0);}
        if (s.b[3283] && (!s.b[3284])) {s.copy_ad(335, 835);s.store_div_scaled_inputs_indices(336, 835, p.p526, 844, 1.0);s.store_mul_add_scaled_product_rhs_indices(892, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3283]) {s.store_scalar(892, 0.0);}
        s.b[3286] = (s.v[838] > 0.0);s.store_scalar(3286, if s.b[3286] { 1.0 } else { 0.0 });s.b[3287] = (s.v[859] < 0.0);s.store_scalar(3287, if s.b[3287] { 1.0 } else { 0.0 });
        if (s.b[3286] && s.b[3287]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 845);}
        s.b[3288] = (p.p527 == 0.5);s.store_scalar(3288, if s.b[3288] { 1.0 } else { 0.0 });
        if ((s.b[3286] && s.b[3287]) && s.b[3288]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3286] && s.b[3287]) && (!s.b[3288])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p527));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_192(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3286] && s.b[3287]) {s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p527)), 0.0);}
        if (s.b[3286] && (!s.b[3287])) {s.copy_ad(335, 838);s.store_div_scaled_inputs_indices(336, 838, p.p527, 845, 1.0);s.store_mul_add_scaled_product_rhs_indices(894, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3286]) {s.store_scalar(894, 0.0);}
        s.b[3289] = (p.p48 > 0.0);s.store_scalar(3289, if s.b[3289] { 1.0 } else { 0.0 });s.b[3290] = (s.v[839] > 0.0);s.store_scalar(3290, if s.b[3290] { 1.0 } else { 0.0 });s.b[3291] = (s.v[867] < 0.0);s.store_scalar(3291, if s.b[3291] { 1.0 } else { 0.0 });
        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {s.store_sub_from_scalar_div_indices(770, 1.0, 867, 846);}
        s.b[3292] = (p.p528 == 0.5);s.store_scalar(3292, if s.b[3292] { 1.0 } else { 0.0 });
        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && s.b[3292]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && (!s.b[3292])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p528));
            }
        }
        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);}
        if ((s.b[3289] && s.b[3290]) && (!s.b[3291])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p.p528, 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 867, 335, 1.0, 867, 336, 0.5);}
        if (s.b[3289] && (!s.b[3290])) {s.store_scalar(896, 0.0);}
        s.b[3293] = (s.v[839] > 0.0);s.store_scalar(3293, if s.b[3293] { 1.0 } else { 0.0 });s.b[3294] = (s.v[859] < 0.0);s.store_scalar(3294, if s.b[3294] { 1.0 } else { 0.0 });
        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 846);}
        s.b[3295] = (p.p528 == 0.5);s.store_scalar(3295, if s.b[3295] { 1.0 } else { 0.0 });
        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && s.b[3295]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && (!s.b[3295])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p528));
            }
        }
        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);}
        if (((!s.b[3289]) && s.b[3293]) && (!s.b[3294])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p.p528, 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 859, 335, 1.0, 859, 336, 0.5);}
        if ((!s.b[3289]) && (!s.b[3293])) {s.store_scalar(896, 0.0);}
        s.b[3296] = (p.p48 > 0.0);s.store_scalar(3296, if s.b[3296] { 1.0 } else { 0.0 });
        if s.b[3296] {s.store_scaled_add(66, 892, 894, s.v[365]);s.store_scaled_add(65, 891, 893, s.v[365]);s.store_scale(68, 896, s.v[365]);s.store_scale(67, 895, s.v[365]);}
        if (!s.b[3296]) {s.store_add_scaled_inputs3_indices(66, 892, s.v[365], 894, s.v[365], 896, s.v[365]);s.store_add_scaled_inputs3_indices(65, 891, s.v[365], 893, s.v[365], 895, s.v[365]);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);}
        s.store_scalar(903, (p.p540 / 1e-6));s.store_scalar(906, s.v[820]);s.store_scalar(904, (1450.0 / 10000.0));s.store_scalar(905, (500.0 / 10000.0));s.store_scalar(943, 0.001);s.store_scale_ad(908, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), 1.45e16);s.store_scaled_square(907, 908, 1.0 / (s.v[903]));s.store_powf(335, 676, (-1.5));s.store_scaled_mul(909, 335, 155, s.v[904]);s.store_scaled_mul(910, 335, 155, s.v[905]);s.store_div_scaled_product_add_scaled_denominator_indices(911, 909, 910, 2.0, 909, 1.0, 910, 1.0, 1.0);s.store_powf(336, 676, p.p547);s.store_scale(913, 336, p.p544);s.store_sqrt_mul(912, 913, 911);s.store_mul_scaled_ln_ad_rhs(934, 155, s.v[906], A::div_from_scalar(s.v[903], s.ad_value(907)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_193(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_mul_add_scaled_inputs_rhs(935, 155, A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), s.v[906], A::div_from_scalar(p.p545, s.ad_value(912)), s.v[906]);s.b[3297] = (p.p539 > 0.0);s.store_scalar(3297, if s.b[3297] { 1.0 } else { 0.0 });
        if s.b[3297] {s.store_scalar(936, s.v[820]);s.store_exp_mul(937, 860, 850);}
        s.b[3298] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);s.store_scalar(3298, if s.b[3298] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3298]) {s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div_scaled_inputs2(s.ad_value(935), 1.0, s.ad_value(934), (-1.0), s.ad_value(936), 1.0))));}
        if (s.b[3297] && (!s.b[3298])) {s.store_scalar(938, 1.0);}
        s.b[3299] = ((p.p542 == 0.0) || (s.v[860] < s.v[934]));s.store_scalar(3299, if s.b[3299] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3299]) {s.store_scale(941, 937, p.p541);}
        if (s.b[3297] && (!s.b[3299])) {s.store_mul_scaled_exp_ad_rhs(941, 937, p.p541, A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542)));}
        if s.b[3297] {
            if (s.v[941] > 1e20) {
                s.store_scalar(941, 1e20);
            } else {
            }
        }
        if s.b[3297] {s.store_mul(939, 907, 941);s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p.p13));}
        s.b[3300] = (p.p543 > 0.0);s.store_scalar(3300, if s.b[3300] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3300]) {s.store_scale(922, 920, p.p543);s.store_scaled_voltage(924, ctx, nodes, Some(15), None, p.p543);s.store_scaled_sub(926, 924, 922, 1.0 / (p.p543));s.store_scale(928, 924, 1.0 / (p.p543));}
        if (s.b[3297] && (!s.b[3300])) {s.copy_ad(922, 920);s.copy_ad(928, 922);}
        s.b[3301] = ((p.p542 == 0.0) || (s.v[860] < s.v[935]));s.store_scalar(3301, if s.b[3301] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3301]) {s.store_scale(942, 938, p.p541);}
        if (s.b[3297] && (!s.b[3301])) {s.store_mul_scaled_exp_ad_rhs(942, 938, p.p541, A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542)));}
        if s.b[3297] {
            if (s.v[942] > 1e20) {
                s.store_scalar(942, 1e20);
            } else {
            }
        }
        if s.b[3297] {s.store_mul(940, 907, 942);s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p.p13));}
        s.b[3302] = (p.p543 > 0.0);s.store_scalar(3302, if s.b[3302] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3302]) {s.store_scale(923, 921, p.p543);s.store_scaled_voltage(925, ctx, nodes, Some(16), None, p.p543);s.store_scaled_sub(927, 925, 923, 1.0 / (p.p543));s.store_scale(929, 925, 1.0 / (p.p543));}
        if (s.b[3297] && (!s.b[3302])) {s.copy_ad(923, 921);s.copy_ad(929, 923);}
        if s.b[3297] {s.store_sub_from_scalar(914, p.p506, 860);s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);s.store_scaled_add(914, 914, 782, 0.5);}
        s.b[3303] = (s.v[914] < 0.0);s.store_scalar(3303, if s.b[3303] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3303]) {s.store_scalar(914, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3297] {s.store_sqrt_scaled_input(915, 914, ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903]))));s.store_offset_sub_from_scalar_ad(781, p.p545, s.ad_value(915), (-1e-7));s.store_scalar(782, ((4.0 * p.p545) * 1e-7));}
        if s.b[3297] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_194(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3297] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(915, 781, (-0.5), 782, (-0.5), p.p545);}
        s.b[3304] = (p.p546 > 0.0);s.store_scalar(3304, if s.b[3304] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3304]) {s.store_scale(930, 915, p.p546);s.store_scaled_voltage(931, ctx, nodes, Some(17), None, p.p546);s.store_scaled_sub(932, 931, 930, 1.0 / (p.p546));s.store_scale(933, 931, 1.0 / (p.p546));}
        if (s.b[3297] && (!s.b[3304])) {s.copy_ad(930, 915);s.copy_ad(933, 930);}
        if s.b[3297] {s.store_scalar(916, ((-((s.v[903] * p.p13) * 1.6021918e-19)) * p.p545));s.store_mul_ad_product_rhs_mixed_ia(917, 912, 928, A::sub(A::exp(A::div_from_scalar((-p.p545), s.ad_value(912))), A::exp_div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0)));s.store_mul_ad_product_rhs_mixed_ia(918, 912, 929, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p.p545, s.ad_value(933)), -1.0, s.ad_value(912), 1.0), (-1.0)));s.store_add_scaled_inputs3_indices(919, 916, (-1.0), 917, (-1.0), 918, (-1.0));s.store_add_scaled_inputs(65, 65, 1.0, 919, s.v[365]);}
        s.b[3305] = ((p.p539 > 0.0) && (p.p543 > 0.0));s.store_scalar(3305, if s.b[3305] { 1.0 } else { 0.0 });s.b[3306] = ((p.p539 > 0.0) && (p.p546 > 0.0));s.store_scalar(3306, if s.b[3306] { 1.0 } else { 0.0 });s.b[3307] = (p.p46 == 1.0);s.store_scalar(3307, if s.b[3307] { 1.0 } else { 0.0 });s.b[3308] = ((s.v[486] > 0.0) && (s.v[454] > 0.0));s.store_scalar(3308, if s.b[3308] { 1.0 } else { 0.0 });
        if (s.b[3307] && s.b[3308]) {s.store_mul(335, 665, 85);s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 666, 1434, (-1.0));s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[3309] = ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0));s.store_scalar(3309, if s.b[3309] { 1.0 } else { 0.0 });
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {s.store_sub(781, 341, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3310] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(3310, if s.b[3310] { 1.0 } else { 0.0 });s.b[3311] = (4.0 == 1.0);s.store_scalar(3311, if s.b[3311] { 1.0 } else { 0.0 });
        if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && s.b[3311]) {s.store_scalar(720, 1.0);}
        s.b[3312] = (4.0 == 2.0);s.store_scalar(3312, if s.b[3312] { 1.0 } else { 0.0 });
        if (((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && s.b[3312]) {s.store_scalar(720, 2.0);}
        s.b[3313] = (4.0 == 4.0);s.store_scalar(3313, if s.b[3313] { 1.0 } else { 0.0 });
        if ((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && s.b[3313]) {s.store_scalar(720, 3.0);}
        s.b[3314] = (4.0 == 8.0);s.store_scalar(3314, if s.b[3314] { 1.0 } else { 0.0 });
        if (((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && (!s.b[3313])) && s.b[3314]) {s.store_scalar(720, 4.0);}
        if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3307] && s.b[3308]) && s.b[3309]) && (!s.b[3310])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub(340, 341, 780);}
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        }
        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {
        }
        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {s.store_scalar(334, 1.0);}
        if (s.b[3307] && s.b[3308]) {s.store_sqrt(340, 340);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_195(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3307] && s.b[3308]) {s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 667, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1435, s.v[488], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[3315] = (s.v[283] < 0.0);s.store_scalar(3315, if s.b[3315] { 1.0 } else { 0.0 });
        if ((s.b[3307] && s.b[3308]) && s.b[3315]) {s.store_scalar(283, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3307] && s.b[3308]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if (s.b[3307] && s.b[3308]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if (s.b[3307] && s.b[3308]) {s.store_div(339, 668, 958);s.store_mul(340, 669, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));}
        s.b[3317] = (s.v[78] == 0.0);s.store_scalar(3317, if s.b[3317] { 1.0 } else { 0.0 });
        if ((s.v[81] != 0.0) && s.b[3317]) {s.store_scalar(346, p.p270);s.store_scalar(344, p.p271);s.copy_ad(337, 170);s.store_mul_product3_indices(335, 337, 346, 344, 337, 1.0);s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);}
        if (s.v[81] != 0.0) {s.store_scalar(336, s.v[565]);}
        s.b[3318] = ((p.p26 != 0.0) && (s.v[78] == 0.0));s.store_scalar(3318, if s.b[3318] { 1.0 } else { 0.0 });
        if s.b[3318] {s.store_scalar(309, s.v[522]);s.store_scalar(311, s.v[563]);s.store_scale(335, 238, 6.241449993689894e18);s.store_sqrt_offset_ad(782, A::square(A::sub(s.ad_value(87), s.ad_value(1431))), ((4.0 * 0.001) * 0.001));s.store_scaled_offset_ad(334, A::div_scaled_inputs2(s.ad_value(87), 1.0, s.ad_value(1431), (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(339, 87, 0.5, 1431, ((-1.0) * 0.5), 782, 0.5);}
        s.b[3319] = (s.v[339] < 0.0);s.store_scalar(3319, if s.b[3319] { 1.0 } else { 0.0 });
        if (s.b[3318] && s.b[3319]) {s.store_scalar(339, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3318] {s.store_mul_scale_offset_mixed_ai(336, A::add_scaled_inputs3(s.ad_value(185), 1.0, A::div(s.ad_value(238), s.ad_value(339)), 1.0, s.ad_value(311), 1.0), 155, 6.241449993689894e18, 0.0);s.store_sub_mixed_ai(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);}
        s.b[3320] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(3320, if s.b[3320] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_196(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3318] && s.b[3320]) {s.store_add_scaled_value_products_mixed_aaaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        if (s.b[3318] && (!s.b[3320])) {s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        s.b[3321] = (((p.p30 != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3321, if s.b[3321] { 1.0 } else { 0.0 });
        if s.b[3321] {s.store_div_scaled_offset_numerator_mixed_ai(313, A::sub(s.ad_value(168), s.ad_value(87)), 1.0, (10.0 * 2.220446049250313e-16), 170, 1.0);}
        if s.b[3321] {
            if (s.v[313] >= 0.0) {
            } else {
                s.store_scalar(313, 0.0);
            }
        }
        if s.b[3321] {s.store_scaled_mul(346, 254, 313, 1e-7);}
        s.b[3322] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3322, if s.b[3322] { 1.0 } else { 0.0 });
        if (s.b[3321] && s.b[3322]) {s.store_scalar(341, 1.0);}
        s.b[3323] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3323, if s.b[3323] { 1.0 } else { 0.0 });
        if ((s.b[3321] && (!s.b[3322])) && s.b[3323]) {s.copy_ad(341, 346);}
        if ((s.b[3321] && (!s.b[3322])) && (!s.b[3323])) {
            if (s.v[313] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_powf(341, 313, (p.p178 - 1.0));
            }
        }
        if s.b[3321] {s.store_mul(342, 346, 341);s.store_offset(343, 342, 1.0);}
        if s.b[3321] {
            if (s.v[343] == 0.0) {
                s.store_scalar(344, 0.0);
            } else {
                s.store_powf(344, 343, (((-1.0) / p.p178) - 1.0));
            }
        }
        if s.b[3321] {s.store_mul(345, 343, 344);s.store_mul(316, 254, 345);s.store_scaled_add(314, 253, 316, 0.5);s.store_square(334, 125);}
    }
}
