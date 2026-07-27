#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_179(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[3107] && s.b[3108]) {s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(79, (-1.0));s.store_scalar(3118, 0.0);s.store_scalar(3119, 0.0);s.store_mul_scaled_ln_ad_rhs(3114, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3114), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.b[3107] && s.b[3108]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3107] && s.b[3108]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(3115, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[3121] = (s.v[3116] > (s.v[3115] * 0.5));s.store_scalar(3121, if s.b[3121] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3121]) {s.store_scale(3116, 3115, 0.5);}
        s.b[3122] = param_given[338];s.store_scalar(3122, if s.b[3122] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3122]) {s.store_scalar(3115, p[338]);}
        s.b[3123] = param_given[339];s.store_scalar(3123, if s.b[3123] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3123]) {s.store_scalar(3116, p[339]);}
        s.b[3124] = param_given[338];s.store_scalar(3124, if s.b[3124] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3123])) && s.b[3124]) {s.store_scale(3116, 3115, 0.5);}
        s.b[3125] = (s.v[3116] > (s.v[3115] * 0.5));s.store_scalar(3125, if s.b[3125] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3125]) {s.store_scale(3116, 3115, 0.5);}
        s.b[3126] = (p[38] == 1.0);s.store_scalar(3126, if s.b[3126] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {s.store_neg(334, 396);}
        s.b[3127] = (s.v[334] > s.v[3116]);s.store_scalar(3127, if s.b[3127] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && s.b[3127]) {s.store_sub(335, 334, 3116);s.store_sub(336, 3115, 3116);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 3116, 333);}
        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && (!s.b[3127])) {s.copy_ad(344, 334);}
        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {s.store_neg(397, 344);}
        if ((s.b[3107] && s.b[3108]) && (!s.b[3126])) {s.copy_ad(397, 396);}
        if (s.b[3107] && s.b[3108]) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);s.store_scalar(3110, 0.0);s.store_primal_scale(3111, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[3128] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(3128, if s.b[3128] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3128]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((s.b[3107] && s.b[3108]) && (!s.b[3128])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_180(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[3129] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(3129, if s.b[3129] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3130] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(3130, if s.b[3130] { 1.0 } else { 0.0 });s.b[3131] = (1.0 == 1.0);s.store_scalar(3131, if s.b[3131] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && s.b[3131]) {s.store_scalar(720, 1.0);}
        s.b[3132] = (1.0 == 2.0);s.store_scalar(3132, if s.b[3132] { 1.0 } else { 0.0 });
        if ((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && s.b[3132]) {s.store_scalar(720, 2.0);}
        s.b[3133] = (1.0 == 4.0);s.store_scalar(3133, if s.b[3133] { 1.0 } else { 0.0 });
        if (((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && s.b[3133]) {s.store_scalar(720, 3.0);}
        s.b[3134] = (1.0 == 8.0);s.store_scalar(3134, if s.b[3134] { 1.0 } else { 0.0 });
        if ((((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && (!s.b[3133])) && s.b[3134]) {s.store_scalar(720, 4.0);}
        if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && (!s.b[3130])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
        }
        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && (!s.b[3129])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[3135] = (s.v[402] < s.v[403]);s.store_scalar(3135, if s.b[3135] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[3136] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(3136, if s.b[3136] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && s.b[3136]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && (!s.b[3136])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(3118, 404);}
        s.b[3137] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(3137, if s.b[3137] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3137]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_181(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3137])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[3138] = (s.v[116] >= 3.0);s.store_scalar(3138, if s.b[3138] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3138])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[3139] = (p[33] > 0.0);s.store_scalar(3139, if s.b[3139] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_182(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[3140] = (p[33] == 2.0);s.store_scalar(3140, if s.b[3140] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && (!s.b[3140])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {s.store_sub(444, 444, 447);s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {s.copy_ad(445, 116);}
        s.b[3141] = (p[33] == 2.0);s.store_scalar(3141, if s.b[3141] { 1.0 } else { 0.0 });s.b[3142] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(3142, if s.b[3142] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3143] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3143, if s.b[3143] { 1.0 } else { 0.0 });s.b[3144] = (2.0 == 1.0);s.store_scalar(3144, if s.b[3144] { 1.0 } else { 0.0 });
        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && s.b[3144]) {s.store_scalar(720, 1.0);}
        s.b[3145] = (2.0 == 2.0);s.store_scalar(3145, if s.b[3145] { 1.0 } else { 0.0 });
        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && s.b[3145]) {s.store_scalar(720, 2.0);}
        s.b[3146] = (2.0 == 4.0);s.store_scalar(3146, if s.b[3146] { 1.0 } else { 0.0 });
        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3146]) {s.store_scalar(720, 3.0);}
        s.b[3147] = (2.0 == 8.0);s.store_scalar(3147, if s.b[3147] { 1.0 } else { 0.0 });
        if ((((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3146])) && s.b[3147]) {s.store_scalar(720, 4.0);}
        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && (!s.b[3143])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && (!s.b[3142])) {s.copy_ad(116, 445);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_183(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && (!s.b[3142])) {s.store_scalar(335, 1.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && (!s.b[3141])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[3148] = (p[33] == 1.0);s.store_scalar(3148, if s.b[3148] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[3149] = (s.v[411] > 0.0);s.store_scalar(3149, if s.b[3149] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3149]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3150] = (s.v[336] < 0.0);s.store_scalar(3150, if s.b[3150] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) && s.b[3150]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3151] = (s.v[336] < 0.0);s.store_scalar(3151, if s.b[3151] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3151]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[3152] = (s.v[333] < 60.0);s.store_scalar(3152, if s.b[3152] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3152]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3152])) {s.store_sub(416, 414, 418);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {s.store_mul(415, 154, 416);}
        s.b[3153] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(3153, if s.b[3153] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3153]) {s.store_primal_offset(3117, 3117, 1.0);s.copy_ad(116, 447);}
        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[3154] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(3154, if s.b[3154] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3154]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3154])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(3155, 354, 3111);}
        s.b[3157] = (p[33] == 2.0);s.store_scalar(3157, if s.b[3157] { 1.0 } else { 0.0 });s.b[3158] = ((s.v[3155] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(3158, if s.b[3158] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {s.store_add_scaled_inputs3_indices(781, 3155, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3159] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3159, if s.b[3159] { 1.0 } else { 0.0 });s.b[3160] = (2.0 == 1.0);s.store_scalar(3160, if s.b[3160] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_184(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && s.b[3160]) {s.store_scalar(720, 1.0);}
        s.b[3161] = (2.0 == 2.0);s.store_scalar(3161, if s.b[3161] { 1.0 } else { 0.0 });
        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && s.b[3161]) {s.store_scalar(720, 2.0);}
        s.b[3162] = (2.0 == 4.0);s.store_scalar(3162, if s.b[3162] { 1.0 } else { 0.0 });
        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && (!s.b[3161])) && s.b[3162]) {s.store_scalar(720, 3.0);}
        s.b[3163] = (2.0 == 8.0);s.store_scalar(3163, if s.b[3163] { 1.0 } else { 0.0 });
        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && (!s.b[3161])) && (!s.b[3162])) && s.b[3163]) {s.store_scalar(720, 4.0);}
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && (!s.b[3159])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {
        }
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && (!s.b[3158])) {s.copy_ad(335, 3155);s.store_scalar(334, 1.0);}
        s.b[3164] = (s.v[334] < 1.0);s.store_scalar(3164, if s.b[3164] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3164]) {s.store_primal_offset(3117, 3117, 2.0);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3157])) {
            if (s.v[3155] <= s.v[386]) {
                s.copy_ad(335, 3155);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[3165] = (s.v[3155] >= s.v[386]);s.store_scalar(3165, if s.b[3165] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3157])) && s.b[3165]) {s.store_primal_offset(3117, 3117, 2.0);}
        s.b[3166] = (s.v[3117] >= 2.0);s.store_scalar(3166, if s.b[3166] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) {s.copy_ad(3156, 404);s.store_mul(354, 335, 3111);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[3167] = (p[33] == 2.0);s.store_scalar(3167, if s.b[3167] { 1.0 } else { 0.0 });s.b[3168] = ((s.v[404] > (s.v[3156] - 0.1)) && (0.1 >= 0.0));s.store_scalar(3168, if s.b[3168] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {s.store_offset_sub(781, 404, 3156, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3169] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3169, if s.b[3169] { 1.0 } else { 0.0 });s.b[3170] = (2.0 == 1.0);s.store_scalar(3170, if s.b[3170] { 1.0 } else { 0.0 });
        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && s.b[3170]) {s.store_scalar(720, 1.0);}
        s.b[3171] = (2.0 == 2.0);s.store_scalar(3171, if s.b[3171] { 1.0 } else { 0.0 });
        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && s.b[3171]) {s.store_scalar(720, 2.0);}
        s.b[3172] = (2.0 == 4.0);s.store_scalar(3172, if s.b[3172] { 1.0 } else { 0.0 });
        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && (!s.b[3171])) && s.b[3172]) {s.store_scalar(720, 3.0);}
        s.b[3173] = (2.0 == 8.0);s.store_scalar(3173, if s.b[3173] { 1.0 } else { 0.0 });
        if ((((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && (!s.b[3171])) && (!s.b[3172])) && s.b[3173]) {s.store_scalar(720, 4.0);}
        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && (!s.b[3169])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 3156, (-0.1), 780);}
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && (!s.b[3168])) {
        }
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && (!s.b[3168])) {s.store_scalar(334, 1.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && (!s.b[3167])) {
            if (s.v[404] <= s.v[3156]) {
            } else {
                s.copy_ad(404, 3156);
            }
        }
        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {s.copy_ad(3118, 404);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_185(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[3174] = (p[33] == 1.0);s.store_scalar(3174, if s.b[3174] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);}
        s.b[3175] = (s.v[411] > 0.0);s.store_scalar(3175, if s.b[3175] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3175]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3176] = (s.v[336] < 0.0);s.store_scalar(3176, if s.b[3176] { 1.0 } else { 0.0 });
        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) && s.b[3176]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3177] = (s.v[336] < 0.0);s.store_scalar(3177, if s.b[3177] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3177]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_186(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut tc: usize = 0;
        while {
            let ta: f64 = (s.v[421] + 1.0);let tb: f64 = if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (s.v[97] <= ta)) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
    pub(super) fn stamp_reactive_block_187(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {s.store_mul(3109, 982, 223);s.store_mul(3110, 3111, 3109);s.store_offset_div(100, 3110, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[3192] = (p[33] == 4.0);s.store_scalar(3192, if s.b[3192] { 1.0 } else { 0.0 });
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 3118);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);}
        s.b[3193] = (s.v[411] > 0.0);s.store_scalar(3193, if s.b[3193] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3193]) {s.store_sub_from_scalar(336, p[334], 411);}
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3194] = (s.v[336] < 0.0);s.store_scalar(3194, if s.b[3194] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) && s.b[3194]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3195] = (s.v[336] < 0.0);s.store_scalar(3195, if s.b[3195] { 1.0 } else { 0.0 });
        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3195]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_188(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut tf: usize = 0;
        while {
            let td: f64 = (s.v[421] + 1.0);let te: f64 = if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (s.v[97] <= td)) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
    pub(super) fn stamp_reactive_block_189(
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
        s.b[3208] = (p[55] == 0.0);s.store_scalar(3208, if s.b[3208] { 1.0 } else { 0.0 });s.b[3209] = (p[50] == 0.0);s.store_scalar(3209, if s.b[3209] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3209]) {s.store_neg(3112, 404);}
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && (!s.b[3209])) {s.copy_ad(3112, 396);}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_sqrt_offset_square_offset(782, 3112, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3112), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(3112), p[137]), 782, 0.5);}
        s.b[3210] = (s.v[336] < 0.0);s.store_scalar(3210, if s.b[3210] { 1.0 } else { 0.0 });
        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3210]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        if (s.b[3107] && s.b[3108]) {s.copy_ad(698, 354);}
        s.b[3211] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] == 0.0));s.store_scalar(3211, if s.b[3211] { 1.0 } else { 0.0 });
        if s.b[3211] {s.store_scalar(2619, 1.0);s.store_scalar(289, s.v[564]);s.store_scalar(290, p[276]);s.store_scalar(335, (s.v[188] * s.v[635]));}
        s.b[3212] = (s.v[949] == 1.0);s.store_scalar(3212, if s.b[3212] { 1.0 } else { 0.0 });
        if (s.b[3211] && s.b[3212]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p[66]);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(291, 791, 339, 1.0, 338, 343, (-1.0));}
        if (s.b[3211] && (!s.b[3212])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p[66]);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(291, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
        s.b[3213] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] == 0.0));s.store_scalar(3213, if s.b[3213] { 1.0 } else { 0.0 });
        if s.b[3213] {s.store_scalar(2622, 1.0);s.store_scalar(289, s.v[564]);s.store_scalar(290, p[276]);s.store_scale(335, 412, s.v[635]);}
        s.b[3214] = (s.v[949] == 1.0);s.store_scalar(3214, if s.b[3214] { 1.0 } else { 0.0 });
        if (s.b[3213] && s.b[3214]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p[63]);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(292, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_190(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3213] && (!s.b[3214])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p[63]);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(292, 791, 339, 1.0, 338, 343, (-1.0));}
        if s.b[768] {s.store_scalar(295, (s.v[505] * (-s.v[635])));}
        s.b[3215] = (s.v[2619] == 0.0);s.store_scalar(3215, if s.b[3215] { 1.0 } else { 0.0 });
        if ((!s.b[768]) && s.b[3215]) {s.store_scalar(295, (((-s.v[188]) * p[66]) * s.v[635]));}
        s.store_mul_scale_offset_indices(297, 734, 295, -1.0, 0.0);
        if s.b[769] {s.store_scalar(294, (s.v[506] * (-s.v[635])));}
        s.b[3216] = (s.v[2622] == 0.0);s.store_scalar(3216, if s.b[3216] { 1.0 } else { 0.0 });
        if ((!s.b[769]) && s.b[3216]) {s.store_primal_scale(294, 412, (-(p[63] * s.v[635])));}
        s.store_mul_sub_scaled_inputs_rhs_indices(298, 294, 734, -1.0, 733, -1.0);s.b[3217] = (s.v[949] == 1.0);s.store_scalar(3217, if s.b[3217] { 1.0 } else { 0.0 });
        if s.b[3217] {s.store_scaled_sub(357, 790, 94, p[431]);s.store_mul(360, 338, 357);s.store_mul(361, 338, 357);}
        if (!s.b[3217]) {s.store_scaled_sub(357, 790, 94, (-p[431]));s.store_mul(362, 338, 357);s.store_mul(363, 338, 357);}
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
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((s.b[3218] && s.b[3219]) && s.b[3220]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
        s.b[3227] = (p[133] != 0.0);s.store_scalar(3227, if s.b[3227] { 1.0 } else { 0.0 });
        if ((!s.b[3226]) && s.b[3227]) {s.store_add_scaled_product_indices(304, 87, 1.0, 303, 162, 1.0);s.store_add_scaled_inputs3_indices(335, 1435, s.v[172], 87, s.v[172], 304, (1.0 - s.v[172]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_191(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[3226]) && s.b[3227]) {s.store_mul_scale_offset_mixed_ia(305, 336, A::add_scaled_inputs3(s.ad_value(87), 1.0, s.ad_value(1435), 1.0, s.ad_value(335), -1.0), (-1.0 / (p[133])), 0.0);}
        s.b[3228] = (p[134] != 0.0);s.store_scalar(3228, if s.b[3228] { 1.0 } else { 0.0 });
        if ((!s.b[3226]) && s.b[3228]) {s.store_add_scaled_inputs(305, 305, 1.0, 792, s.v[671]);}
        s.store_scalar(300, s.v[670]);s.store_scalar(302, s.v[670]);s.store_scaled_sub(299, 734, 733, s.v[300]);s.store_scale(301, 734, s.v[302]);s.b[3229] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3229, if s.b[3229] { 1.0 } else { 0.0 });
        if s.b[3229] {s.store_square(334, 676);s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[498])), p[495]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[498])), p[495]);}
        s.b[3230] = (p[48] > 0.0);s.store_scalar(3230, if s.b[3230] { 1.0 } else { 0.0 });s.b[3231] = (p[15] > s.v[632]);s.store_scalar(3231, if s.b[3231] { 1.0 } else { 0.0 });
        if ((s.b[3229] && s.b[3230]) && s.b[3231]) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, (p[15] - s.v[632]));s.store_scale(876, 831, (p[15] - s.v[632]));s.store_scale(877, 836, s.v[632]);s.store_scale(878, 837, s.v[632]);}
        if ((s.b[3229] && s.b[3230]) && (!s.b[3231])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scale(877, 836, p[15]);s.store_scale(878, 837, p[15]);}
        if (s.b[3229] && (!s.b[3230])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, p[15]);s.store_scale(876, 831, p[15]);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);}
        if s.b[3229] {s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);}
        s.b[3232] = (s.v[847] > 0.0);s.store_scalar(3232, if s.b[3232] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3232]) {s.store_offset(336, 847, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_192(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3229] && s.b[3232]) {s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p[512]);s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));s.store_exp_mul(851, 848, 850);}
        if s.b[3229] {s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[521])), p[518]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[521])), p[518]);}
        s.b[3233] = (p[48] > 0.0);s.store_scalar(3233, if s.b[3233] { 1.0 } else { 0.0 });s.b[3234] = (p[16] > s.v[632]);s.store_scalar(3234, if s.b[3234] { 1.0 } else { 0.0 });
        if ((s.b[3229] && s.b[3233]) && s.b[3234]) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, (p[16] - s.v[632]));s.store_scale(882, 831, (p[16] - s.v[632]));s.store_scale(883, 836, s.v[632]);s.store_scale(884, 837, s.v[632]);}
        if ((s.b[3229] && s.b[3233]) && (!s.b[3234])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scale(883, 836, p[16]);s.store_scale(884, 837, p[16]);}
        if (s.b[3229] && (!s.b[3233])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, p[16]);s.store_scale(882, 831, p[16]);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);}
        if s.b[3229] {s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);}
        s.b[3235] = (s.v[852] > 0.0);s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3235]) {s.store_offset(337, 852, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p[535]);s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));s.store_exp_mul(856, 853, 855);}
        if s.b[3229] {s.store_offset_scaled(832, 391, ((p[481]) * ((p[500] * p[13]))), (p[500] * p[13]));}
        s.b[3236] = (p[15] > s.v[632]);s.store_scalar(3236, if s.b[3236] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3236]) {s.store_offset_scaled(833, 391, ((p[483]) * ((p[501] * (p[15] - s.v[632])))), (p[501] * (p[15] - s.v[632])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_193(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[3229] && s.b[3236]) {s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * s.v[632]))), (p[502] * s.v[632]));}
        if (s.b[3229] && (!s.b[3236])) {s.store_scalar(833, 0.0);s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * p[15]))), (p[502] * p[15]));}
        s.b[3237] = (s.v[832] < 0.0);s.store_scalar(3237, if s.b[3237] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3237]) {s.store_scalar(832, 0.0);}
        s.b[3238] = (s.v[833] < 0.0);s.store_scalar(3238, if s.b[3238] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3238]) {s.store_scalar(833, 0.0);}
        s.b[3239] = (s.v[834] < 0.0);s.store_scalar(3239, if s.b[3239] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3239]) {s.store_scalar(834, 0.0);}
        if s.b[3229] {s.store_sub_from_scalar_scaled_input(841, p[506], 391, p[487]);s.store_sub_from_scalar_scaled_input(842, p[507], 391, p[489]);s.store_sub_from_scalar_scaled_input(843, p[508], 391, p[491]);}
        s.b[3240] = ((s.v[841] < 0.01) && (p[13] > 0.0));s.store_scalar(3240, if s.b[3240] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3240]) {s.store_scalar(841, 0.01);}
        s.b[3241] = ((s.v[842] < 0.01) && (p[15] > s.v[632]));s.store_scalar(3241, if s.b[3241] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3241]) {s.store_scalar(842, 0.01);}
        s.b[3242] = ((s.v[843] < 0.01) && (p[15] > 0.0));s.store_scalar(3242, if s.b[3242] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3242]) {s.store_scalar(843, 0.01);}
        if s.b[3229] {s.store_offset_scaled(835, 391, ((p[482]) * ((p[523] * p[14]))), (p[523] * p[14]));}
        s.b[3243] = (p[16] > s.v[632]);s.store_scalar(3243, if s.b[3243] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3243]) {s.store_offset_scaled(838, 391, ((p[484]) * ((p[524] * (p[16] - s.v[632])))), (p[524] * (p[16] - s.v[632])));s.store_offset_scaled(839, 391, ((p[486]) * ((p[525] * s.v[632]))), (p[525] * s.v[632]));}
        if (s.b[3229] && (!s.b[3243])) {s.store_scalar(838, 0.0);s.store_offset_scaled(839, 391, ((p[486]) * ((p[525] * p[16]))), (p[525] * p[16]));}
        s.b[3244] = (s.v[835] < 0.0);s.store_scalar(3244, if s.b[3244] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3244]) {s.store_scalar(835, 0.0);}
        s.b[3245] = (s.v[838] < 0.0);s.store_scalar(3245, if s.b[3245] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3245]) {s.store_scalar(838, 0.0);}
        s.b[3246] = (s.v[839] < 0.0);s.store_scalar(3246, if s.b[3246] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3246]) {s.store_scalar(839, 0.0);}
        if s.b[3229] {s.store_sub_from_scalar_scaled_input(844, p[529], 391, p[488]);s.store_sub_from_scalar_scaled_input(845, p[530], 391, p[490]);s.store_sub_from_scalar_scaled_input(846, p[531], 391, p[492]);}
        s.b[3247] = ((s.v[844] < 0.01) && (p[14] > 0.0));s.store_scalar(3247, if s.b[3247] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3247]) {s.store_scalar(844, 0.01);}
        s.b[3248] = ((s.v[845] < 0.01) && (p[16] > s.v[632]));s.store_scalar(3248, if s.b[3248] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3248]) {s.store_scalar(845, 0.01);}
        s.b[3249] = ((s.v[846] < 0.01) && (p[16] > 0.0));s.store_scalar(3249, if s.b[3249] { 1.0 } else { 0.0 });
        if (s.b[3229] && s.b[3249]) {s.store_scalar(846, 0.01);}
        if (!s.b[3229]) {s.store_scalar(387, (ctx_temp + p[11]));}
        s.store_scale(344, 850, p[511]);s.store_scale(343, 849, p[510]);s.b[3250] = (s.v[873] > 0.0);s.store_scalar(3250, if s.b[3250] { 1.0 } else { 0.0 });
        if s.b[3250] {s.store_mul(334, 874, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3251] = (s.v[860] < s.v[848]);s.store_scalar(3251, if s.b[3251] { 1.0 } else { 0.0 });
        if (s.b[3250] && s.b[3251]) {s.store_mul(332, 860, 850);}
        s.b[3252] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3252, if s.b[3252] { 1.0 } else { 0.0 });
        if ((s.b[3250] && s.b[3251]) && s.b[3252]) {s.store_scalar(335, 0.0);}
        if ((s.b[3250] && s.b[3251]) && (!s.b[3252])) {s.store_exp(335, 332);}
        if (s.b[3250] && (!s.b[3251])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 873, 850, 335);}
        s.store_scale(346, 874, p[514]);s.b[3253] = (s.v[875] > 0.0);s.store_scalar(3253, if s.b[3253] { 1.0 } else { 0.0 });
        if s.b[3253] {s.store_mul(334, 876, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3254] = (s.v[860] < s.v[848]);s.store_scalar(3254, if s.b[3254] { 1.0 } else { 0.0 });
        if (s.b[3253] && s.b[3254]) {s.store_mul(332, 860, 850);}
        s.b[3255] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3255, if s.b[3255] { 1.0 } else { 0.0 });
        if ((s.b[3253] && s.b[3254]) && s.b[3255]) {s.store_scalar(335, 0.0);}
        if ((s.b[3253] && s.b[3254]) && (!s.b[3255])) {s.store_exp(335, 332);}
        if (s.b[3253] && (!s.b[3254])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 875, 850, 335);}
        s.store_scale(346, 876, p[514]);s.b[3256] = (p[48] > 0.0);s.store_scalar(3256, if s.b[3256] { 1.0 } else { 0.0 });s.b[3257] = (s.v[877] > 0.0);s.store_scalar(3257, if s.b[3257] { 1.0 } else { 0.0 });
        if (s.b[3256] && s.b[3257]) {s.store_mul(334, 878, 343);s.store_mul_scale_offset_indices(332, 344, 868, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_194(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[3258] = (s.v[868] < s.v[848]);s.store_scalar(3258, if s.b[3258] { 1.0 } else { 0.0 });
        if ((s.b[3256] && s.b[3257]) && s.b[3258]) {s.store_mul(332, 868, 850);}
        s.b[3259] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3259, if s.b[3259] { 1.0 } else { 0.0 });
        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && s.b[3259]) {s.store_scalar(335, 0.0);}
        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && (!s.b[3259])) {s.store_exp(335, 332);}
        if ((s.b[3256] && s.b[3257]) && (!s.b[3258])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 877, 850, 335);}
        if s.b[3256] {s.store_scale(346, 878, p[514]);}
        s.store_scale(344, 855, p[534]);s.store_scale(343, 854, p[533]);s.b[3260] = (s.v[879] > 0.0);s.store_scalar(3260, if s.b[3260] { 1.0 } else { 0.0 });
        if s.b[3260] {s.store_mul(334, 880, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3261] = (s.v[859] < s.v[853]);s.store_scalar(3261, if s.b[3261] { 1.0 } else { 0.0 });
        if (s.b[3260] && s.b[3261]) {s.store_mul(332, 859, 855);}
        s.b[3262] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3262, if s.b[3262] { 1.0 } else { 0.0 });
        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {s.store_scalar(335, 0.0);}
        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {s.store_exp(335, 332);}
        if (s.b[3260] && (!s.b[3261])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 879, 855, 335);}
        s.store_scale(346, 880, p[537]);s.b[3263] = (s.v[881] > 0.0);s.store_scalar(3263, if s.b[3263] { 1.0 } else { 0.0 });
        if s.b[3263] {s.store_mul(334, 882, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3264] = (s.v[859] < s.v[853]);s.store_scalar(3264, if s.b[3264] { 1.0 } else { 0.0 });
        if (s.b[3263] && s.b[3264]) {s.store_mul(332, 859, 855);}
        s.b[3265] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3265, if s.b[3265] { 1.0 } else { 0.0 });
        if ((s.b[3263] && s.b[3264]) && s.b[3265]) {s.store_scalar(335, 0.0);}
        if ((s.b[3263] && s.b[3264]) && (!s.b[3265])) {s.store_exp(335, 332);}
        if (s.b[3263] && (!s.b[3264])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 881, 855, 335);}
        s.store_scale(346, 882, p[537]);s.b[3266] = (p[48] > 0.0);s.store_scalar(3266, if s.b[3266] { 1.0 } else { 0.0 });s.b[3267] = (s.v[883] > 0.0);s.store_scalar(3267, if s.b[3267] { 1.0 } else { 0.0 });
        if (s.b[3266] && s.b[3267]) {s.store_mul(334, 884, 343);s.store_mul_scale_offset_indices(332, 344, 867, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3268] = (s.v[867] < s.v[853]);s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });
        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {s.store_mul(332, 867, 855);}
        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3269, if s.b[3269] { 1.0 } else { 0.0 });
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && s.b[3269]) {s.store_scalar(335, 0.0);}
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && (!s.b[3269])) {s.store_exp(335, 332);}
        if ((s.b[3266] && s.b[3267]) && (!s.b[3268])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 883, 855, 335);}
        if s.b[3266] {s.store_scale(346, 884, p[537]);}
        s.b[3270] = (s.v[832] > 0.0);s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });s.b[3271] = (s.v[860] < 0.0);s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });
        if (s.b[3270] && s.b[3271]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 841);}
        s.b[3272] = (p[503] == 0.5);s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });
        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[503]));
            }
        }
        if (s.b[3270] && s.b[3271]) {s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[503])), 0.0);}
        if (s.b[3270] && (!s.b[3271])) {s.copy_ad(335, 832);s.store_div_scaled_inputs_indices(336, 832, p[503], 841, 1.0);s.store_mul_add_scaled_product_rhs_indices(891, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3270]) {s.store_scalar(891, 0.0);}
        s.b[3273] = (s.v[833] > 0.0);s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });s.b[3274] = (s.v[860] < 0.0);s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });
        if (s.b[3273] && s.b[3274]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 842);}
        s.b[3275] = (p[504] == 0.5);s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });
        if ((s.b[3273] && s.b[3274]) && s.b[3275]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
}
