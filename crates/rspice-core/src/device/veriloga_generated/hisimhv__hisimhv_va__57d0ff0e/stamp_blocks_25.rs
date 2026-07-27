#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_157(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && s.b[2905]) {s.store_scalar(720, 3.0);}
        s.b[2906] = (1.0 == 8.0);s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {s.store_scalar(720, 4.0);}
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && (!s.b[2902])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2901])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[2907] = (s.v[402] < s.v[403]);s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2907]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2908] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2907]) && s.b[2908]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2625] != 0.0) && s.b[2907]) && (!s.b[2908])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2625] != 0.0) && s.b[2907]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2890, 404);}
        s.b[2909] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2909]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2909])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2910] = (s.v[116] >= 3.0);s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2910]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_158(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2910]) {s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2910])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2911] = (p[33] > 0.0);s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2912] = (p[33] == 2.0);s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2912])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_sub(444, 444, 447);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_159(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.copy_ad(445, 116);}
        s.b[2913] = (p[33] == 2.0);s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });s.b[2914] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2915] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });s.b[2916] = (2.0 == 1.0);s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && s.b[2916]) {s.store_scalar(720, 1.0);}
        s.b[2917] = (2.0 == 2.0);s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && s.b[2917]) {s.store_scalar(720, 2.0);}
        s.b[2918] = (2.0 == 4.0);s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && s.b[2918]) {s.store_scalar(720, 3.0);}
        s.b[2919] = (2.0 == 8.0);s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && (!s.b[2918])) && s.b[2919]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && (!s.b[2915])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && (!s.b[2914])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2913])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2920] = (p[33] == 1.0);s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2921] = (s.v[411] > 0.0);s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2921]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2922] = (s.v[336] < 0.0);s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) && s.b[2922]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2923] = (s.v[336] < 0.0);s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2923]) {s.store_scalar(336, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_160(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2923]) {s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2924] = (s.v[333] < 60.0);s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2924]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2924])) {s.store_sub(416, 414, 418);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_mul(415, 154, 416);}
        s.b[2925] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2925]) {s.store_primal_offset(2889, 2889, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2926] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2926]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2926])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2927, 354, 2883);}
        s.b[2929] = (p[33] == 2.0);s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });s.b[2930] = ((s.v[2927] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {s.store_add_scaled_inputs3_indices(781, 2927, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2931] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });s.b[2932] = (2.0 == 1.0);s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && s.b[2932]) {s.store_scalar(720, 1.0);}
        s.b[2933] = (2.0 == 2.0);s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && s.b[2933]) {s.store_scalar(720, 2.0);}
        s.b[2934] = (2.0 == 4.0);s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && s.b[2934]) {s.store_scalar(720, 3.0);}
        s.b[2935] = (2.0 == 8.0);s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && (!s.b[2934])) && s.b[2935]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && (!s.b[2931])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && (!s.b[2930])) {s.copy_ad(335, 2927);s.store_scalar(334, 1.0);}
        s.b[2936] = (s.v[334] < 1.0);s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_161(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2936]) {s.store_primal_offset(2889, 2889, 2.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) {
            if (s.v[2927] <= s.v[386]) {
                s.copy_ad(335, 2927);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2937] = (s.v[2927] >= s.v[386]);s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) && s.b[2937]) {s.store_primal_offset(2889, 2889, 2.0);}
        s.b[2938] = (s.v[2889] >= 2.0);s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) {s.copy_ad(2928, 404);s.store_mul(354, 335, 2883);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2939] = (p[33] == 2.0);s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });s.b[2940] = ((s.v[404] > (s.v[2928] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {s.store_offset_sub(781, 404, 2928, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });s.b[2942] = (2.0 == 1.0);s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && s.b[2942]) {s.store_scalar(720, 1.0);}
        s.b[2943] = (2.0 == 2.0);s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && s.b[2943]) {s.store_scalar(720, 2.0);}
        s.b[2944] = (2.0 == 4.0);s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {s.store_scalar(720, 3.0);}
        s.b[2945] = (2.0 == 8.0);s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) && s.b[2945]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && (!s.b[2941])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2928, (-0.1), 780);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && (!s.b[2939])) {
            if (s.v[404] <= s.v[2928]) {
            } else {
                s.copy_ad(404, 2928);
            }
        }
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.copy_ad(2890, 404);}
        s.b[2946] = (p[33] == 1.0);s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2883)), s.ad_value(155)), 2.0);}
        s.b[2947] = (s.v[411] > 0.0);s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2947]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2948] = (s.v[336] < 0.0);s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2949] = (s.v[336] < 0.0);s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_162(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2949]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_163(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut ta: usize = 0;
        while {
            let t8: f64 = (s.v[421] + 1.0);let t9: f64 = if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (s.v[97] <= t8)) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2951] = (s.v[333] < 60.0);s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2951]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2951])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_mul(415, 154, 416);}
            s.b[2952] = (s.v[116] < 0.0);s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2952]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[2953] = (s.v[116] < 1e-6);s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[2954] = (s.v[338] > 0.0);s.store_scalar(2954, if s.b[2954] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) && s.b[2954]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) && (!s.b[2954])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[2955] = (s.v[338] > 0.0);s.store_scalar(2955, if s.b[2955] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) && s.b[2955]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) && (!s.b[2955])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[2956] = (s.v[116] < 0.0);s.store_scalar(2956, if s.b[2956] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2956]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[2957] = (s.v[116] < 60.0);s.store_scalar(2957, if s.b[2957] { 1.0 } else { 0.0 });s.b[2958] = (s.v[116] < 5e-5);s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2957]) && s.b[2958]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2957]) && (!s.b[2958])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && (!s.b[2957])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2959] = (s.v[214] > 0.0);s.store_scalar(2959, if s.b[2959] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2959]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && (!s.b[2959])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2960] = (s.v[79] == 1.0);s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2960]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2961] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) && s.b[2961]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {s.store_add(404, 404, 236);}
            s.b[2962] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) && s.b[2962]) {s.store_scalar(79, 1.0);}
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_164(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_mul(2881, 982, 223);s.store_mul(2882, 2883, 2881);s.store_offset_div(100, 2882, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[2964] = (p[33] == 4.0);s.store_scalar(2964, if s.b[2964] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 2890);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2883)), s.ad_value(155)), 2.0);}
        s.b[2965] = (s.v[411] > 0.0);s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2965]) {s.store_sub_from_scalar(336, p[334], 411);}
        if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2966] = (s.v[336] < 0.0);s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) && s.b[2966]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2967] = (s.v[336] < 0.0);s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2967]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.v[2625] != 0.0) && s.b[2964]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_165(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut td: usize = 0;
        while {
            let tb: f64 = (s.v[421] + 1.0);let tc: f64 = if (((s.v[2625] != 0.0) && s.b[2964]) && (s.v[97] <= tb)) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2969] = (s.v[333] < 60.0);s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2969]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2969])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_mul(415, 154, 416);}
            s.b[2970] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(2970, if s.b[2970] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2970]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(2891, 334, 336);s.store_mul_add_scaled_product_rhs_indices(2892, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2970])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(2891, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(2892, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[2971] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(2971, if s.b[2971] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2971]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[2972] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(2972, if s.b[2972] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && s.b[2972]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && (!s.b[2972])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2973] = (s.v[214] > 0.0);s.store_scalar(2973, if s.b[2973] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2973]) {s.store_sqrt_add(216, 2891, 214);s.store_div_scaled_inputs2_indices(217, 2892, 0.5, 215, 0.5, 216, 1.0);}
            s.b[2974] = (s.v[2891] > 0.0);s.store_scalar(2974, if s.b[2974] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && s.b[2974]) {s.store_sqrt(216, 2891);s.store_div_scaled_inputs_indices(217, 2892, 0.5, 216, 1.0);}
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && (!s.b[2974])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2975] = (s.v[79] > 0.0);s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2975]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2976] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2976, if s.b[2976] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2976]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {s.store_add(404, 404, 236);}
            s.b[2977] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2977, if s.b[2977] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2977]) {s.store_primal_offset(79, 79, 2.0);}
            if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_166(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2625] != 0.0) && s.b[2964]) {
            if (s.v[2891] >= 0.0) {
                s.store_scaled_sqrt(223, 2891, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.v[2625] != 0.0) && s.b[2964]) {s.store_mul(2881, 982, 223);s.store_mul(2882, 2883, 2881);s.store_offset_div(100, 2882, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.v[2625] != 0.0) {s.store_sub(399, 398, 354);}
        s.b[2979] = (s.v[407] < 0.0);s.store_scalar(2979, if s.b[2979] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2979]) {s.store_neg(407, 407);}
        s.b[2980] = (p[55] == 0.0);s.store_scalar(2980, if s.b[2980] { 1.0 } else { 0.0 });s.b[2981] = (p[50] == 0.0);s.store_scalar(2981, if s.b[2981] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2981]) {s.store_neg(2884, 404);}
        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && (!s.b[2981])) {s.copy_ad(2884, 396);}
        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {s.store_sqrt_offset_square_offset(782, 2884, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2884), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(2884), p[137]), 782, 0.5);}
        s.b[2982] = (s.v[336] < 0.0);s.store_scalar(2982, if s.b[2982] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2982]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        s.b[2983] = (3.0 == 1.0);s.store_scalar(2983, if s.b[2983] { 1.0 } else { 0.0 });s.b[2984] = (3.0 == 2.0);s.store_scalar(2984, if s.b[2984] { 1.0 } else { 0.0 });s.b[2985] = (3.0 == 3.0);s.store_scalar(2985, if s.b[2985] { 1.0 } else { 0.0 });s.b[2986] = (3.0 == 4.0);s.store_scalar(2986, if s.b[2986] { 1.0 } else { 0.0 });s.b[2987] = (p[55] == 1.0);s.store_scalar(2987, if s.b[2987] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2983]) && s.b[2987]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2625] != 0.0) && s.b[2983]) && (!s.b[2987])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2625] != 0.0) && s.b[2983]) {s.store_mul(353, 338, 398);s.store_mul(356, 338, 354);}
        if ((s.v[2625] != 0.0) && (s.b[2984] && (!s.b[2983]))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(351, 338, 398);s.store_mul(359, 338, 354);}
        s.b[2988] = (p[55] == 1.0);s.store_scalar(2988, if s.b[2988] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2988]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && (!s.b[2988])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {s.copy_ad(697, 404);}
        s.b[2989] = (p[430] == 0.0);s.store_scalar(2989, if s.b[2989] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2989]) {s.copy_ad(698, 354);}
        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {s.store_mul(352, 338, 398);s.store_mul(355, 338, 354);s.copy_ad(816, 355);}
        if ((s.v[2625] != 0.0) && (s.b[2986] && (!((s.b[2983] || s.b[2984]) || s.b[2985])))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(350, 338, 398);s.store_mul(358, 338, 354);}
        s.store_scalar(2625, 0.0);s.b[2990] = (4.0 == 1.0);s.store_scalar(2990, if s.b[2990] { 1.0 } else { 0.0 });s.b[2991] = (4.0 == 2.0);s.store_scalar(2991, if s.b[2991] { 1.0 } else { 0.0 });s.b[2992] = (4.0 == 3.0);s.store_scalar(2992, if s.b[2992] { 1.0 } else { 0.0 });s.b[2993] = (4.0 == 4.0);s.store_scalar(2993, if s.b[2993] { 1.0 } else { 0.0 });s.b[2994] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2994, if s.b[2994] { 1.0 } else { 0.0 });
        if (s.b[2990] && s.b[2994]) {s.store_scalar(2625, 1.0);s.store_scalar(2623, 1.0);s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);s.store_scalar(407, p[66]);s.store_scalar(411, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_167(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[2990] && s.b[2994]) {s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2995] = (((((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2995, if s.b[2995] { 1.0 } else { 0.0 });
        if ((s.b[2991] && (!s.b[2990])) && s.b[2995]) {s.store_scalar(2625, 1.0);s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2996] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2996, if s.b[2996] { 1.0 } else { 0.0 });
        if ((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) {s.store_scalar(2625, 1.0);s.store_scalar(2626, 1.0);s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p[63] + (p[64] * p[55])));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2997] = (((s.v[407] < 0.0) && (p[432] > 0.0)) && (p[55] == 1.0));s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });
        if (((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) {s.store_neg(407, 407);s.store_scalar(335, p[63]);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p[137]));}
        s.b[2998] = (p[113] > 0.0);s.store_scalar(2998, if s.b[2998] { 1.0 } else { 0.0 });s.b[2999] = ((s.v[396] == 0.0) || (p[113] <= 0.0));s.store_scalar(2999, if s.b[2999] { 1.0 } else { 0.0 });
        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && s.b[2999]) {
        }
        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p[113], 1.0);s.store_powf(782, 781, (1.0 / p[113]));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) {s.store_sqrt_offset_square_offset(782, 396, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(396), p[137]), 782, 0.5);}
        s.b[3000] = (s.v[336] < 0.0);s.store_scalar(3000, if s.b[3000] { 1.0 } else { 0.0 });
        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && s.b[3000]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub(407, 407, 600);}
        s.b[3001] = (((((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(3001, if s.b[3001] { 1.0 } else { 0.0 });
        if ((s.b[2993] && (!((s.b[2990] || s.b[2991]) || s.b[2992]))) && s.b[3001]) {s.store_scalar(2625, 1.0);s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2625] != 0.0) {s.store_scalar(3009, 0.4);s.store_scalar(3010, 0.0);s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(79, (-1.0));s.store_scalar(3011, 0.0);s.store_scalar(3012, 0.0);s.store_mul_scaled_ln_ad_rhs(3007, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3007), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2625] != 0.0) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(3008, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[3014] = (s.v[3009] > (s.v[3008] * 0.5));s.store_scalar(3014, if s.b[3014] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3014]) {s.store_scale(3009, 3008, 0.5);}
        s.b[3015] = param_given[338];s.store_scalar(3015, if s.b[3015] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3015]) {s.store_scalar(3008, p[338]);}
        s.b[3016] = param_given[339];s.store_scalar(3016, if s.b[3016] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3016]) {s.store_scalar(3009, p[339]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_168(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[3017] = param_given[338];s.store_scalar(3017, if s.b[3017] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3016])) && s.b[3017]) {s.store_scale(3009, 3008, 0.5);}
        s.b[3018] = (s.v[3009] > (s.v[3008] * 0.5));s.store_scalar(3018, if s.b[3018] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3018]) {s.store_scale(3009, 3008, 0.5);}
        s.b[3019] = (p[38] == 1.0);s.store_scalar(3019, if s.b[3019] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3019]) {s.store_neg(334, 396);}
        s.b[3020] = (s.v[334] > s.v[3009]);s.store_scalar(3020, if s.b[3020] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[3019]) && s.b[3020]) {s.store_sub(335, 334, 3009);s.store_sub(336, 3008, 3009);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 3009, 333);}
        if (((s.v[2625] != 0.0) && s.b[3019]) && (!s.b[3020])) {s.copy_ad(344, 334);}
        if ((s.v[2625] != 0.0) && s.b[3019]) {s.store_neg(397, 344);}
        if ((s.v[2625] != 0.0) && (!s.b[3019])) {s.copy_ad(397, 396);}
        if (s.v[2625] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);s.store_scalar(3003, 0.0);s.store_primal_scale(3004, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[3021] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(3021, if s.b[3021] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3021]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2625] != 0.0) && (!s.b[3021])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((s.v[2625] != 0.0) && (!s.b[3021])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2625] != 0.0) && (!s.b[3021])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2625] != 0.0) && (!s.b[3021])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[3022] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(3022, if s.b[3022] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3023] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(3023, if s.b[3023] { 1.0 } else { 0.0 });s.b[3024] = (1.0 == 1.0);s.store_scalar(3024, if s.b[3024] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && s.b[3024]) {s.store_scalar(720, 1.0);}
        s.b[3025] = (1.0 == 2.0);s.store_scalar(3025, if s.b[3025] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && s.b[3025]) {s.store_scalar(720, 2.0);}
        s.b[3026] = (1.0 == 4.0);s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_169(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && s.b[3026]) {s.store_scalar(720, 3.0);}
        s.b[3027] = (1.0 == 8.0);s.store_scalar(3027, if s.b[3027] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3026])) && s.b[3027]) {s.store_scalar(720, 4.0);}
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && (!s.b[3023])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3022])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[3028] = (s.v[402] < s.v[403]);s.store_scalar(3028, if s.b[3028] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[3028]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[3029] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(3029, if s.b[3029] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[3028]) && s.b[3029]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2625] != 0.0) && s.b[3028]) && (!s.b[3029])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2625] != 0.0) && s.b[3028]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(3011, 404);}
        s.b[3030] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(3030, if s.b[3030] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3030]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3030])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2625] != 0.0) && (!s.b[3028])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[3031] = (s.v[116] >= 3.0);s.store_scalar(3031, if s.b[3031] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3031]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_170(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3031]) {s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3031])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[3032] = (p[33] > 0.0);s.store_scalar(3032, if s.b[3032] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[3033] = (p[33] == 2.0);s.store_scalar(3033, if s.b[3033] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3033])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {s.store_sub(444, 444, 447);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_171(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {s.copy_ad(445, 116);}
        s.b[3034] = (p[33] == 2.0);s.store_scalar(3034, if s.b[3034] { 1.0 } else { 0.0 });s.b[3035] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(3035, if s.b[3035] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3036] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3036, if s.b[3036] { 1.0 } else { 0.0 });s.b[3037] = (2.0 == 1.0);s.store_scalar(3037, if s.b[3037] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && s.b[3037]) {s.store_scalar(720, 1.0);}
        s.b[3038] = (2.0 == 2.0);s.store_scalar(3038, if s.b[3038] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && s.b[3038]) {s.store_scalar(720, 2.0);}
        s.b[3039] = (2.0 == 4.0);s.store_scalar(3039, if s.b[3039] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && s.b[3039]) {s.store_scalar(720, 3.0);}
        s.b[3040] = (2.0 == 8.0);s.store_scalar(3040, if s.b[3040] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && (!s.b[3039])) && s.b[3040]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && (!s.b[3036])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && (!s.b[3035])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3034])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[3041] = (p[33] == 1.0);s.store_scalar(3041, if s.b[3041] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[3042] = (s.v[411] > 0.0);s.store_scalar(3042, if s.b[3042] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3042]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3043] = (s.v[336] < 0.0);s.store_scalar(3043, if s.b[3043] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) && s.b[3043]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3044] = (s.v[336] < 0.0);s.store_scalar(3044, if s.b[3044] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3044]) {s.store_scalar(336, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_172(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3044]) {s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[3045] = (s.v[333] < 60.0);s.store_scalar(3045, if s.b[3045] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3045]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3045])) {s.store_sub(416, 414, 418);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {s.store_mul(415, 154, 416);}
        s.b[3046] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(3046, if s.b[3046] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3046]) {s.store_primal_offset(3010, 3010, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2625] != 0.0) && (!s.b[3028])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[3047] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(3047, if s.b[3047] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3047]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3047])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2625] != 0.0) && (!s.b[3028])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(3048, 354, 3004);}
        s.b[3050] = (p[33] == 2.0);s.store_scalar(3050, if s.b[3050] { 1.0 } else { 0.0 });s.b[3051] = ((s.v[3048] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(3051, if s.b[3051] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {s.store_add_scaled_inputs3_indices(781, 3048, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3052, if s.b[3052] { 1.0 } else { 0.0 });s.b[3053] = (2.0 == 1.0);s.store_scalar(3053, if s.b[3053] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && s.b[3053]) {s.store_scalar(720, 1.0);}
        s.b[3054] = (2.0 == 2.0);s.store_scalar(3054, if s.b[3054] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && s.b[3054]) {s.store_scalar(720, 2.0);}
        s.b[3055] = (2.0 == 4.0);s.store_scalar(3055, if s.b[3055] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && s.b[3055]) {s.store_scalar(720, 3.0);}
        s.b[3056] = (2.0 == 8.0);s.store_scalar(3056, if s.b[3056] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && (!s.b[3055])) && s.b[3056]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && (!s.b[3052])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
        }
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && (!s.b[3051])) {s.copy_ad(335, 3048);s.store_scalar(334, 1.0);}
        s.b[3057] = (s.v[334] < 1.0);s.store_scalar(3057, if s.b[3057] { 1.0 } else { 0.0 });
    }
}
