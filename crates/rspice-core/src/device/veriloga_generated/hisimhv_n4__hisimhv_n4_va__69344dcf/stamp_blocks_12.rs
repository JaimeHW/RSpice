#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3138])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3139] = (p.p33 > 0.0);
        s.v[3139] = if s.b[3139] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3140] = (p.p33 == 2.0);
        s.v[3140] = if s.b[3140] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

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

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.copy_ad(445, 116);
        }

        s.b[3141] = (p.p33 == 2.0);
        s.v[3141] = if s.b[3141] { 1.0 } else { 0.0 };

        s.b[3142] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3142] = if s.b[3142] { 1.0 } else { 0.0 };

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3143] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3143] = if s.b[3143] { 1.0 } else { 0.0 };

        s.b[3144] = (2.0 == 1.0);
        s.v[3144] = if s.b[3144] { 1.0 } else { 0.0 };

        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && s.b[3144]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3145] = (2.0 == 2.0);
        s.v[3145] = if s.b[3145] { 1.0 } else { 0.0 };

        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && s.b[3145]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3146] = (2.0 == 4.0);
        s.v[3146] = if s.b[3146] { 1.0 } else { 0.0 };

        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3146]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3147] = (2.0 == 8.0);
        s.v[3147] = if s.b[3147] { 1.0 } else { 0.0 };

        if ((((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3146])) && s.b[3147]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign92710_loop_guard: usize = 0;
        while {
            let assign92710_cond_e142078: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92710_cond_e142078 != 0.0
        } {
            assign92710_loop_guard += 1;
            assert!(assign92710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && (!s.b[3143])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && (!s.b[3142])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && (!s.b[3141])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3148] = (p.p33 == 1.0);
        s.v[3148] = if s.b[3148] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3149] = (s.v[411] > 0.0);
        s.v[3149] = if s.b[3149] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3149]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3150] = (s.v[336] < 0.0);
        s.v[3150] = if s.b[3150] { 1.0 } else { 0.0 };

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) && s.b[3150]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3151] = (s.v[336] < 0.0);
        s.v[3151] = if s.b[3151] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3151]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3152] = (s.v[333] < 60.0);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3152]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3152])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3153] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3153] = if s.b[3153] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3153]) {
            s.store_offset(3117, 3117, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3154] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3154] = if s.b[3154] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3154]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3154])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3155, 354, 3111);
        }

        s.b[3157] = (p.p33 == 2.0);
        s.v[3157] = if s.b[3157] { 1.0 } else { 0.0 };

        s.b[3158] = ((s.v[3155] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3158] = if s.b[3158] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {
            s.store_add_scaled_inputs3_indices(781, 3155, 1.0, 386, (-1.0), 386, 0.1);
        }

    }

    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3159] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3159] = if s.b[3159] { 1.0 } else { 0.0 };

        s.b[3160] = (2.0 == 1.0);
        s.v[3160] = if s.b[3160] { 1.0 } else { 0.0 };

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && s.b[3160]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3161] = (2.0 == 2.0);
        s.v[3161] = if s.b[3161] { 1.0 } else { 0.0 };

        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && s.b[3161]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3162] = (2.0 == 4.0);
        s.v[3162] = if s.b[3162] { 1.0 } else { 0.0 };

        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && (!s.b[3161])) && s.b[3162]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3163] = (2.0 == 8.0);
        s.v[3163] = if s.b[3163] { 1.0 } else { 0.0 };

        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (!s.b[3160])) && (!s.b[3161])) && (!s.b[3162])) && s.b[3163]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign93520_loop_guard: usize = 0;
        while {
            let assign93520_cond_e143286: f64 = if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93520_cond_e143286 != 0.0
        } {
            assign93520_loop_guard += 1;
            assert!(assign93520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && s.b[3159]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) && (!s.b[3159])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3158]) {
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && (!s.b[3158])) {
            s.copy_ad(335, 3155);
            s.store_scalar(334, 1.0);
        }

        s.b[3164] = (s.v[334] < 1.0);
        s.v[3164] = if s.b[3164] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3157]) && s.b[3164]) {
            s.store_offset(3117, 3117, 2.0);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3157])) {
            if (s.v[3155] <= s.v[386]) {
                s.copy_ad(335, 3155);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3165] = (s.v[3155] >= s.v[386]);
        s.v[3165] = if s.b[3165] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3157])) && s.b[3165]) {
            s.store_offset(3117, 3117, 2.0);
        }

        s.b[3166] = (s.v[3117] >= 2.0);
        s.v[3166] = if s.b[3166] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) {
            s.copy_ad(3156, 404);
            s.store_mul(354, 335, 3111);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[3167] = (p.p33 == 2.0);
        s.v[3167] = if s.b[3167] { 1.0 } else { 0.0 };

        s.b[3168] = ((s.v[404] > (s.v[3156] - 0.1)) && (0.1 >= 0.0));
        s.v[3168] = if s.b[3168] { 1.0 } else { 0.0 };

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {
            s.store_offset_sub(781, 404, 3156, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3169] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3169] = if s.b[3169] { 1.0 } else { 0.0 };

        s.b[3170] = (2.0 == 1.0);
        s.v[3170] = if s.b[3170] { 1.0 } else { 0.0 };

        if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && s.b[3170]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3171] = (2.0 == 2.0);
        s.v[3171] = if s.b[3171] { 1.0 } else { 0.0 };

        if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && s.b[3171]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3172] = (2.0 == 4.0);
        s.v[3172] = if s.b[3172] { 1.0 } else { 0.0 };

        if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && (!s.b[3171])) && s.b[3172]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3173] = (2.0 == 8.0);
        s.v[3173] = if s.b[3173] { 1.0 } else { 0.0 };

        if ((((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (!s.b[3170])) && (!s.b[3171])) && (!s.b[3172])) && s.b[3173]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign93970_loop_guard: usize = 0;
        while {
            let assign93970_cond_e143968: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93970_cond_e143968 != 0.0
        } {
            assign93970_loop_guard += 1;
            assert!(assign93970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && (!s.b[3169])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 3156, (-0.1), 780);
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) {
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && (!s.b[3168])) {
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && (!s.b[3168])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && (!s.b[3167])) {
            if (s.v[404] <= s.v[3156]) {
            } else {
                s.copy_ad(404, 3156);
            }
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.copy_ad(3118, 404);
        }

        s.b[3174] = (p.p33 == 1.0);
        s.v[3174] = if s.b[3174] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);
        }

        s.b[3175] = (s.v[411] > 0.0);
        s.v[3175] = if s.b[3175] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3175]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3176] = (s.v[336] < 0.0);
        s.v[3176] = if s.b[3176] { 1.0 } else { 0.0 };

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) && s.b[3176]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3175])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3177] = (s.v[336] < 0.0);
        s.v[3177] = if s.b[3177] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3177]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign94330_loop_guard: usize = 0;
        while {
            let assign94330_cond_e144560: f64 = (s.v[421] + 1.0);
            let assign94330_cond_e144562: f64 = if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (s.v[97] <= assign94330_cond_e144560)) { 1.0 } else { 0.0 };
            assign94330_cond_e144562 != 0.0
        } {
            assign94330_loop_guard += 1;
            assert!(assign94330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3179] = (s.v[333] < 60.0);
            s.v[3179] = if s.b[3179] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3179]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3179])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3180] = (s.v[116] < 0.0);
            s.v[3180] = if s.b[3180] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3180]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3181] = (s.v[116] < 1e-6);
            s.v[3181] = if s.b[3181] { 1.0 } else { 0.0 };
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[3182] = (s.v[338] > 0.0);
            s.v[3182] = if s.b[3182] { 1.0 } else { 0.0 };
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) && s.b[3182]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && s.b[3181]) && (!s.b[3182])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[3183] = (s.v[338] > 0.0);
            s.v[3183] = if s.b[3183] { 1.0 } else { 0.0 };
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) && s.b[3183]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3180])) && (!s.b[3181])) && (!s.b[3183])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3184] = (s.v[116] < 0.0);
            s.v[3184] = if s.b[3184] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3184]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3185] = (s.v[116] < 60.0);
            s.v[3185] = if s.b[3185] { 1.0 } else { 0.0 };
            s.b[3186] = (s.v[116] < 5e-5);
            s.v[3186] = if s.b[3186] { 1.0 } else { 0.0 };
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3185]) && s.b[3186]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3185]) && (!s.b[3186])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && (!s.b[3185])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3187] = (s.v[214] > 0.0);
            s.v[3187] = if s.b[3187] { 1.0 } else { 0.0 };
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && s.b[3187]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3184])) && (!s.b[3187])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3188] = (s.v[79] == 1.0);
            s.v[3188] = if s.b[3188] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3188]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3189] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3189] = if s.b[3189] { 1.0 } else { 0.0 };
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) && s.b[3189]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) {
                s.store_add(404, 404, 236);
            }
            s.b[3190] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3190] = if s.b[3190] { 1.0 } else { 0.0 };
            if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) && s.b[3190]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
            s.store_mul(3109, 982, 223);
            s.store_mul(3110, 3111, 3109);
            s.store_offset_div(100, 3110, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3192] = (p.p33 == 4.0);
        s.v[3192] = if s.b[3192] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3118);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0);
        }

        s.b[3193] = (s.v[411] > 0.0);
        s.v[3193] = if s.b[3193] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3193]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3194] = (s.v[336] < 0.0);
        s.v[3194] = if s.b[3194] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) && s.b[3194]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3193])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3195] = (s.v[336] < 0.0);
        s.v[3195] = if s.b[3195] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3195]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3111, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign94720_loop_guard: usize = 0;
        while {
            let assign94720_cond_e146419: f64 = (s.v[421] + 1.0);
            let assign94720_cond_e146421: f64 = if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (s.v[97] <= assign94720_cond_e146419)) { 1.0 } else { 0.0 };
            assign94720_cond_e146421 != 0.0
        } {
            assign94720_loop_guard += 1;
            assert!(assign94720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3197] = (s.v[333] < 60.0);
            s.v[3197] = if s.b[3197] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3197]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3197])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3198] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[3198] = if s.b[3198] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3198]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(3119, 334, 336);
                s.store_mul_add_scaled_product_rhs(3120, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3198])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(3119, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(3120, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[3199] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[3199] = if s.b[3199] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3199]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3200] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[3200] = if s.b[3200] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3199])) && s.b[3200]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3199])) && (!s.b[3200])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3201] = (s.v[214] > 0.0);
            s.v[3201] = if s.b[3201] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3201]) {
                s.store_sqrt_add(216, 3119, 214);
                s.store_div_scaled_inputs2_indices(217, 3120, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[3202] = (s.v[3119] > 0.0);
            s.v[3202] = if s.b[3202] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3201])) && s.b[3202]) {
                s.store_sqrt(216, 3119);
                s.store_div_scaled_inputs_indices(217, 3120, 0.5, 216, 1.0);
            }
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3201])) && (!s.b[3202])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3203] = (s.v[79] > 0.0);
            s.v[3203] = if s.b[3203] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3203]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3204] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3204] = if s.b[3204] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) && s.b[3204]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) {
                s.store_add(404, 404, 236);
            }
            s.b[3205] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3205] = if s.b[3205] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) && s.b[3205]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            if (s.v[3119] >= 0.0) {
                s.store_scaled_sqrt(223, 3119, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
            s.store_mul(3109, 982, 223);
            s.store_mul(3110, 3111, 3109);
            s.store_offset_div(100, 3110, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.b[3107] && s.b[3108]) {
            s.store_sub(399, 398, 354);
        }

        s.b[3207] = (s.v[407] < 0.0);
        s.v[3207] = if s.b[3207] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3207]) {
            s.store_neg(407, 407);
        }

        s.b[3208] = (p.p55 == 0.0);
        s.v[3208] = if s.b[3208] { 1.0 } else { 0.0 };

        s.b[3209] = (p.p50 == 0.0);
        s.v[3209] = if s.b[3209] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3209]) {
            s.store_neg(3112, 404);
        }

        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && (!s.b[3209])) {
            s.copy_ad(3112, 396);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            s.store_sqrt_offset_square_offset(782, 3112, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3112), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3112), p.p137), 782, 0.5);
        }

        s.b[3210] = (s.v[336] < 0.0);
        s.v[3210] = if s.b[3210] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) && s.b[3210]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3207]) && s.b[3208]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        if (s.b[3107] && s.b[3108]) {
            s.copy_ad(698, 354);
        }

        s.b[3211] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] == 0.0));
        s.v[3211] = if s.b[3211] { 1.0 } else { 0.0 };

        if s.b[3211] {
            s.store_scalar(2619, 1.0);
            s.store_scalar(289, s.v[564]);
            s.store_scalar(290, p.p276);
            s.store_scalar(335, (s.v[188] * s.v[635]));
        }

        s.b[3212] = (s.v[949] == 1.0);
        s.v[3212] = if s.b[3212] { 1.0 } else { 0.0 };

        if (s.b[3211] && s.b[3212]) {
            s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));
            s.store_scale(339, 335, p.p66);
            s.store_sub_from_scalar(343, 1.2, 87);
            s.store_add_scaled_products_indices(291, 791, 339, 1.0, 338, 343, (-1.0));
        }

        if (s.b[3211] && (!s.b[3212])) {
            s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p66);
            s.store_sub_offset_lhs(343, 790, 1.2, 91);
            s.store_add_scaled_products_left_left_ad(291, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));
        }

        s.b[3213] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] == 0.0));
        s.v[3213] = if s.b[3213] { 1.0 } else { 0.0 };

        if s.b[3213] {
            s.store_scalar(2622, 1.0);
            s.store_scalar(289, s.v[564]);
            s.store_scalar(290, p.p276);
            s.store_scale(335, 412, s.v[635]);
        }

        s.b[3214] = (s.v[949] == 1.0);
        s.v[3214] = if s.b[3214] { 1.0 } else { 0.0 };

        if (s.b[3213] && s.b[3214]) {
            s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p63);
            s.store_sub_offset_lhs(343, 790, 1.2, 91);
            s.store_add_scaled_products_left_left_ad(292, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));
        }

        if (s.b[3213] && (!s.b[3214])) {
            s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));
            s.store_scale(339, 335, p.p63);
            s.store_sub_from_scalar(343, 1.2, 87);
            s.store_add_scaled_products_indices(292, 791, 339, 1.0, 338, 343, (-1.0));
        }

        if s.b[768] {
            s.store_scalar(295, (s.v[505] * (-s.v[635])));
        }

        s.b[3215] = (s.v[2619] == 0.0);
        s.v[3215] = if s.b[3215] { 1.0 } else { 0.0 };

        if ((!s.b[768]) && s.b[3215]) {
            s.store_scalar(295, (((-s.v[188]) * p.p66) * s.v[635]));
        }

        s.store_mul_neg_lhs(297, 295, 734);

        if s.b[769] {
            s.store_scalar(294, (s.v[506] * (-s.v[635])));
        }

        s.b[3216] = (s.v[2622] == 0.0);
        s.v[3216] = if s.b[3216] { 1.0 } else { 0.0 };

        if ((!s.b[769]) && s.b[3216]) {
            s.store_scale(294, 412, (-(p.p63 * s.v[635])));
        }

        s.store_mul_sub_scaled_inputs_rhs(298, 294, s.ad_value(734), -1.0, s.ad_value(733), -1.0);

        s.b[3217] = (s.v[949] == 1.0);
        s.v[3217] = if s.b[3217] { 1.0 } else { 0.0 };

        if s.b[3217] {
            s.store_scaled_sub(357, 790, 94, p.p431);
            s.store_mul(360, 338, 357);
            s.store_mul(361, 338, 357);
        }

        if (!s.b[3217]) {
            s.store_scaled_sub(357, 790, 94, (-p.p431));
            s.store_mul(362, 338, 357);
            s.store_mul(363, 338, 357);
        }

        s.v[296] = ((-s.v[525]) * s.v[582]);

        s.store_scaled_sub(293, 731, 728, (-s.v[296]));

        s.v[172] = s.v[507];

        s.b[3218] = (s.v[78] != 0.0);
        s.v[3218] = if s.b[3218] { 1.0 } else { 0.0 };

        if s.b[3218] {
            s.store_add_scaled_inputs3_indices(168, 790, s.v[172], 87, s.v[172], 91, (1.0 - s.v[172]));
        }

        s.b[3219] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[3219] = if s.b[3219] { 1.0 } else { 0.0 };

        if (s.b[3218] && s.b[3219]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3218] && s.b[3219]) {
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3220] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3220] = if s.b[3220] { 1.0 } else { 0.0 };

        s.b[3221] = (2.0 == 1.0);
        s.v[3221] = if s.b[3221] { 1.0 } else { 0.0 };

        if (((s.b[3218] && s.b[3219]) && s.b[3220]) && s.b[3221]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3222] = (2.0 == 2.0);
        s.v[3222] = if s.b[3222] { 1.0 } else { 0.0 };

        if ((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && s.b[3222]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3223] = (2.0 == 4.0);
        s.v[3223] = if s.b[3223] { 1.0 } else { 0.0 };

        if (((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && s.b[3223]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3224] = (2.0 == 8.0);
        s.v[3224] = if s.b[3224] { 1.0 } else { 0.0 };

        if ((((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && (!s.b[3223])) && s.b[3224]) {
            s.store_scalar(720, 4.0);
        }

        if ((s.b[3218] && s.b[3219]) && s.b[3220]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if (((s.b[3218] && s.b[3219]) && s.b[3220]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3218] && s.b[3219]) && s.b[3220]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((s.b[3218] && s.b[3219]) && (!s.b[3220])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (s.b[3218] && s.b[3219]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (s.b[3218] && s.b[3219]) {
        }

        if (s.b[3218] && (!s.b[3219])) {
        }

        if (s.b[3218] && (!s.b[3219])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3218] && s.b[82]) {
            s.store_scalar(303, 0.0);
        }

        s.b[3225] = ((s.v[248] < 1e-15) || (s.v[348] < 1e-6));
        s.v[3225] = if s.b[3225] { 1.0 } else { 0.0 };

        if (((!s.b[3218]) && s.b[82]) && s.b[3225]) {
            s.store_scalar(303, 0.0);
        }

        if (((!s.b[3218]) && s.b[82]) && (!s.b[3225])) {
            s.store_div_scaled_product_by_product(303, s.ad_value(248), s.ad_value(155), 1.0, s.ad_value(238), s.ad_value(162), 1.0);
        }

        s.b[3226] = (!s.b[82]);
        s.v[3226] = if s.b[3226] { 1.0 } else { 0.0 };

        if s.b[3226] {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[3226]) {
            s.store_scale(336, 684, ((1.034943e-10 * s.v[635]) * 1.3));
        }

        s.b[3227] = (p.p133 != 0.0);
        s.v[3227] = if s.b[3227] { 1.0 } else { 0.0 };

        if ((!s.b[3226]) && s.b[3227]) {
            s.store_add_scaled_product_indices(304, 87, 1.0, 303, 162, 1.0);
            s.store_add_scaled_inputs3_indices(335, 1435, s.v[172], 87, s.v[172], 304, (1.0 - s.v[172]));
            s.store_mul_scale_ad_lhs(305, A::add_scaled_inputs3(s.ad_value(87), 1.0, s.ad_value(1435), 1.0, s.ad_value(335), -1.0), (-1.0 / (p.p133)), 336);
        }

        s.b[3228] = (p.p134 != 0.0);
        s.v[3228] = if s.b[3228] { 1.0 } else { 0.0 };

        if ((!s.b[3226]) && s.b[3228]) {
            s.store_add_scaled_inputs(305, 305, 1.0, 792, s.v[671]);
        }

        s.v[300] = s.v[670];

        s.v[302] = s.v[670];

        s.store_scaled_sub(299, 734, 733, s.v[300]);

        s.store_scale(301, 734, s.v[302]);

        s.b[3229] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3229] = if s.b[3229] { 1.0 } else { 0.0 };

        if s.b[3229] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[3230] = (p.p48 > 0.0);
        s.v[3230] = if s.b[3230] { 1.0 } else { 0.0 };

        s.b[3231] = (p.p15 > s.v[632]);
        s.v[3231] = if s.b[3231] { 1.0 } else { 0.0 };

        if ((s.b[3229] && s.b[3230]) && s.b[3231]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[3229] && s.b[3230]) && (!s.b[3231])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scalar(875, 0.0);
            s.store_scalar(876, 0.0);
            s.store_scale(877, 836, p.p15);
            s.store_scale(878, 837, p.p15);
        }

        if (s.b[3229] && (!s.b[3230])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[3229] {
            s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);
        }

        s.b[3232] = (s.v[847] > 0.0);
        s.v[3232] = if s.b[3232] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3232]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));
            s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[3229] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[3233] = (p.p48 > 0.0);
        s.v[3233] = if s.b[3233] { 1.0 } else { 0.0 };

        s.b[3234] = (p.p16 > s.v[632]);
        s.v[3234] = if s.b[3234] { 1.0 } else { 0.0 };

        if ((s.b[3229] && s.b[3233]) && s.b[3234]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

        if ((s.b[3229] && s.b[3233]) && (!s.b[3234])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[3229] && (!s.b[3233])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[3229] {
            s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);
        }

        s.b[3235] = (s.v[852] > 0.0);
        s.v[3235] = if s.b[3235] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3235]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));
            s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[3229] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[3236] = (p.p15 > s.v[632]);
        s.v[3236] = if s.b[3236] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3236]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[3229] && (!s.b[3236])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[3237] = (s.v[832] < 0.0);
        s.v[3237] = if s.b[3237] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3237]) {
            s.store_scalar(832, 0.0);
        }

        s.b[3238] = (s.v[833] < 0.0);
        s.v[3238] = if s.b[3238] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3238]) {
            s.store_scalar(833, 0.0);
        }

        s.b[3239] = (s.v[834] < 0.0);
        s.v[3239] = if s.b[3239] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3239]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[3229] {
            s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);
            s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);
            s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);
        }

        s.b[3240] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.v[3240] = if s.b[3240] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3240]) {
            s.store_scalar(841, 0.01);
        }

        s.b[3241] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.v[3241] = if s.b[3241] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_90(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[3229] && s.b[3241]) {
            s.store_scalar(842, 0.01);
        }

        s.b[3242] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.v[3242] = if s.b[3242] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3242]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[3229] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[3243] = (p.p16 > s.v[632]);
        s.v[3243] = if s.b[3243] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3243]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[3229] && (!s.b[3243])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[3244] = (s.v[835] < 0.0);
        s.v[3244] = if s.b[3244] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3244]) {
            s.store_scalar(835, 0.0);
        }

        s.b[3245] = (s.v[838] < 0.0);
        s.v[3245] = if s.b[3245] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3245]) {
            s.store_scalar(838, 0.0);
        }

        s.b[3246] = (s.v[839] < 0.0);
        s.v[3246] = if s.b[3246] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3246]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[3229] {
            s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);
            s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);
            s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);
        }

        s.b[3247] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.v[3247] = if s.b[3247] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3247]) {
            s.store_scalar(844, 0.01);
        }

        s.b[3248] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.v[3248] = if s.b[3248] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3248]) {
            s.store_scalar(845, 0.01);
        }

        s.b[3249] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.v[3249] = if s.b[3249] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3249]) {
            s.store_scalar(846, 0.01);
        }

        if (!s.b[3229]) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        s.store_scale(344, 850, p.p511);

        s.store_scale(343, 849, p.p510);

        s.b[3250] = (s.v[873] > 0.0);
        s.v[3250] = if s.b[3250] { 1.0 } else { 0.0 };

        if s.b[3250] {
            s.store_mul(334, 874, 343);
            s.store_mul_neg_lhs(332, 860, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3251] = (s.v[860] < s.v[848]);
        s.v[3251] = if s.b[3251] { 1.0 } else { 0.0 };

        if (s.b[3250] && s.b[3251]) {
            s.store_mul(332, 860, 850);
        }

        s.b[3252] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3252] = if s.b[3252] { 1.0 } else { 0.0 };

        if ((s.b[3250] && s.b[3251]) && s.b[3252]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3250] && s.b[3251]) && (!s.b[3252])) {
            s.store_exp(335, 332);
        }

        if (s.b[3250] && (!s.b[3251])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 873, 850, 335);
        }

        s.store_scale(346, 874, p.p514);

        s.b[3253] = (s.v[875] > 0.0);
        s.v[3253] = if s.b[3253] { 1.0 } else { 0.0 };

        if s.b[3253] {
            s.store_mul(334, 876, 343);
            s.store_mul_neg_lhs(332, 860, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3254] = (s.v[860] < s.v[848]);
        s.v[3254] = if s.b[3254] { 1.0 } else { 0.0 };

        if (s.b[3253] && s.b[3254]) {
            s.store_mul(332, 860, 850);
        }

        s.b[3255] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3255] = if s.b[3255] { 1.0 } else { 0.0 };

        if ((s.b[3253] && s.b[3254]) && s.b[3255]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3253] && s.b[3254]) && (!s.b[3255])) {
            s.store_exp(335, 332);
        }

        if (s.b[3253] && (!s.b[3254])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 875, 850, 335);
        }

        s.store_scale(346, 876, p.p514);

        s.b[3256] = (p.p48 > 0.0);
        s.v[3256] = if s.b[3256] { 1.0 } else { 0.0 };

        s.b[3257] = (s.v[877] > 0.0);
        s.v[3257] = if s.b[3257] { 1.0 } else { 0.0 };

        if (s.b[3256] && s.b[3257]) {
            s.store_mul(334, 878, 343);
            s.store_mul_neg_lhs(332, 868, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3258] = (s.v[868] < s.v[848]);
        s.v[3258] = if s.b[3258] { 1.0 } else { 0.0 };

        if ((s.b[3256] && s.b[3257]) && s.b[3258]) {
            s.store_mul(332, 868, 850);
        }

        s.b[3259] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3259] = if s.b[3259] { 1.0 } else { 0.0 };

        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && s.b[3259]) {
            s.store_scalar(335, 0.0);
        }

        if (((s.b[3256] && s.b[3257]) && s.b[3258]) && (!s.b[3259])) {
            s.store_exp(335, 332);
        }

        if ((s.b[3256] && s.b[3257]) && (!s.b[3258])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 877, 850, 335);
        }

        if s.b[3256] {
            s.store_scale(346, 878, p.p514);
        }

        s.store_scale(344, 855, p.p534);

        s.store_scale(343, 854, p.p533);

        s.b[3260] = (s.v[879] > 0.0);
        s.v[3260] = if s.b[3260] { 1.0 } else { 0.0 };

        if s.b[3260] {
            s.store_mul(334, 880, 343);
            s.store_mul_neg_lhs(332, 859, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3261] = (s.v[859] < s.v[853]);
        s.v[3261] = if s.b[3261] { 1.0 } else { 0.0 };

        if (s.b[3260] && s.b[3261]) {
            s.store_mul(332, 859, 855);
        }

        s.b[3262] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3262] = if s.b[3262] { 1.0 } else { 0.0 };

        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {
            s.store_exp(335, 332);
        }

        if (s.b[3260] && (!s.b[3261])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 879, 855, 335);
        }

        s.store_scale(346, 880, p.p537);

        s.b[3263] = (s.v[881] > 0.0);
        s.v[3263] = if s.b[3263] { 1.0 } else { 0.0 };

        if s.b[3263] {
            s.store_mul(334, 882, 343);
            s.store_mul_neg_lhs(332, 859, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3264] = (s.v[859] < s.v[853]);
        s.v[3264] = if s.b[3264] { 1.0 } else { 0.0 };

        if (s.b[3263] && s.b[3264]) {
            s.store_mul(332, 859, 855);
        }

        s.b[3265] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3265] = if s.b[3265] { 1.0 } else { 0.0 };

        if ((s.b[3263] && s.b[3264]) && s.b[3265]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3263] && s.b[3264]) && (!s.b[3265])) {
            s.store_exp(335, 332);
        }

        if (s.b[3263] && (!s.b[3264])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 881, 855, 335);
        }

        s.store_scale(346, 882, p.p537);

        s.b[3266] = (p.p48 > 0.0);
        s.v[3266] = if s.b[3266] { 1.0 } else { 0.0 };

        s.b[3267] = (s.v[883] > 0.0);
        s.v[3267] = if s.b[3267] { 1.0 } else { 0.0 };

        if (s.b[3266] && s.b[3267]) {
            s.store_mul(334, 884, 343);
            s.store_mul_neg_lhs(332, 867, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3268] = (s.v[867] < s.v[853]);
        s.v[3268] = if s.b[3268] { 1.0 } else { 0.0 };

        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {
            s.store_mul(332, 867, 855);
        }

        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3269] = if s.b[3269] { 1.0 } else { 0.0 };

        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && s.b[3269]) {
            s.store_scalar(335, 0.0);
        }

        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && (!s.b[3269])) {
            s.store_exp(335, 332);
        }

        if ((s.b[3266] && s.b[3267]) && (!s.b[3268])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 883, 855, 335);
        }

        if s.b[3266] {
            s.store_scale(346, 884, p.p537);
        }

        s.b[3270] = (s.v[832] > 0.0);
        s.v[3270] = if s.b[3270] { 1.0 } else { 0.0 };

        s.b[3271] = (s.v[860] < 0.0);
        s.v[3271] = if s.b[3271] { 1.0 } else { 0.0 };

        if (s.b[3270] && s.b[3271]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 860, 841);
        }

        s.b[3272] = (p.p503 == 0.5);
        s.v[3272] = if s.b[3272] { 1.0 } else { 0.0 };

        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p503));
            }
        }

        if (s.b[3270] && s.b[3271]) {
            s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p503)), 0.0);
        }

        if (s.b[3270] && (!s.b[3271])) {
            s.copy_ad(335, 832);
            s.store_div_scaled_inputs_indices(336, 832, p.p503, 841, 1.0);
            s.store_mul_add_scaled_product_rhs(891, 860, s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5);
        }

        if (!s.b[3270]) {
            s.store_scalar(891, 0.0);
        }

        s.b[3273] = (s.v[833] > 0.0);
        s.v[3273] = if s.b[3273] { 1.0 } else { 0.0 };

        s.b[3274] = (s.v[860] < 0.0);
        s.v[3274] = if s.b[3274] { 1.0 } else { 0.0 };

        if (s.b[3273] && s.b[3274]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 860, 842);
        }

        s.b[3275] = (p.p504 == 0.5);
        s.v[3275] = if s.b[3275] { 1.0 } else { 0.0 };

        if ((s.b[3273] && s.b[3274]) && s.b[3275]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3273] && s.b[3274]) && (!s.b[3275])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p504));
            }
        }

        if (s.b[3273] && s.b[3274]) {
            s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p504)), 0.0);
        }

        if (s.b[3273] && (!s.b[3274])) {
            s.copy_ad(335, 833);
            s.store_div_scaled_inputs_indices(336, 833, p.p504, 842, 1.0);
            s.store_mul_add_scaled_product_rhs(893, 860, s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5);
        }

        if (!s.b[3273]) {
            s.store_scalar(893, 0.0);
        }

        s.b[3276] = (p.p48 > 0.0);
        s.v[3276] = if s.b[3276] { 1.0 } else { 0.0 };

        s.b[3277] = (s.v[834] > 0.0);
        s.v[3277] = if s.b[3277] { 1.0 } else { 0.0 };

        s.b[3278] = (s.v[868] < 0.0);
        s.v[3278] = if s.b[3278] { 1.0 } else { 0.0 };

        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);
        }

        s.b[3279] = (p.p505 == 0.5);
        s.v[3279] = if s.b[3279] { 1.0 } else { 0.0 };

        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && s.b[3279]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

    }

    pub(super) fn stamp_reactive_block_91(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && (!s.b[3279])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p505));
            }
        }

        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {
            s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);
        }

        if ((s.b[3276] && s.b[3277]) && (!s.b[3278])) {
            s.copy_ad(335, 834);
            s.store_div_scaled_inputs_indices(336, 834, p.p505, 843, 1.0);
            s.store_mul_add_scaled_product_rhs(895, 868, s.ad_value(335), 1.0, s.ad_value(868), s.ad_value(336), 0.5);
        }

        if (s.b[3276] && (!s.b[3277])) {
            s.store_scalar(895, 0.0);
        }

        s.b[3280] = (s.v[834] > 0.0);
        s.v[3280] = if s.b[3280] { 1.0 } else { 0.0 };

        s.b[3281] = (s.v[860] < 0.0);
        s.v[3281] = if s.b[3281] { 1.0 } else { 0.0 };

        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 860, 843);
        }

        s.b[3282] = (p.p505 == 0.5);
        s.v[3282] = if s.b[3282] { 1.0 } else { 0.0 };

        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && s.b[3282]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && (!s.b[3282])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p505));
            }
        }

        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {
            s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);
        }

        if (((!s.b[3276]) && s.b[3280]) && (!s.b[3281])) {
            s.copy_ad(335, 834);
            s.store_div_scaled_inputs_indices(336, 834, p.p505, 843, 1.0);
            s.store_mul_add_scaled_product_rhs(895, 860, s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5);
        }

        if ((!s.b[3276]) && (!s.b[3280])) {
            s.store_scalar(895, 0.0);
        }

        s.b[3283] = (s.v[835] > 0.0);
        s.v[3283] = if s.b[3283] { 1.0 } else { 0.0 };

        s.b[3284] = (s.v[859] < 0.0);
        s.v[3284] = if s.b[3284] { 1.0 } else { 0.0 };

        if (s.b[3283] && s.b[3284]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 859, 844);
        }

        s.b[3285] = (p.p526 == 0.5);
        s.v[3285] = if s.b[3285] { 1.0 } else { 0.0 };

        if ((s.b[3283] && s.b[3284]) && s.b[3285]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3283] && s.b[3284]) && (!s.b[3285])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p526));
            }
        }

        if (s.b[3283] && s.b[3284]) {
            s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p526)), 0.0);
        }

        if (s.b[3283] && (!s.b[3284])) {
            s.copy_ad(335, 835);
            s.store_div_scaled_inputs_indices(336, 835, p.p526, 844, 1.0);
            s.store_mul_add_scaled_product_rhs(892, 859, s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5);
        }

        if (!s.b[3283]) {
            s.store_scalar(892, 0.0);
        }

        s.b[3286] = (s.v[838] > 0.0);
        s.v[3286] = if s.b[3286] { 1.0 } else { 0.0 };

        s.b[3287] = (s.v[859] < 0.0);
        s.v[3287] = if s.b[3287] { 1.0 } else { 0.0 };

        if (s.b[3286] && s.b[3287]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 859, 845);
        }

        s.b[3288] = (p.p527 == 0.5);
        s.v[3288] = if s.b[3288] { 1.0 } else { 0.0 };

        if ((s.b[3286] && s.b[3287]) && s.b[3288]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3286] && s.b[3287]) && (!s.b[3288])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p527));
            }
        }

        if (s.b[3286] && s.b[3287]) {
            s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p527)), 0.0);
        }

        if (s.b[3286] && (!s.b[3287])) {
            s.copy_ad(335, 838);
            s.store_div_scaled_inputs_indices(336, 838, p.p527, 845, 1.0);
            s.store_mul_add_scaled_product_rhs(894, 859, s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5);
        }

        if (!s.b[3286]) {
            s.store_scalar(894, 0.0);
        }

        s.b[3289] = (p.p48 > 0.0);
        s.v[3289] = if s.b[3289] { 1.0 } else { 0.0 };

        s.b[3290] = (s.v[839] > 0.0);
        s.v[3290] = if s.b[3290] { 1.0 } else { 0.0 };

        s.b[3291] = (s.v[867] < 0.0);
        s.v[3291] = if s.b[3291] { 1.0 } else { 0.0 };

        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 867, 846);
        }

        s.b[3292] = (p.p528 == 0.5);
        s.v[3292] = if s.b[3292] { 1.0 } else { 0.0 };

        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && s.b[3292]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && (!s.b[3292])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p528));
            }
        }

        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {
            s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);
        }

        if ((s.b[3289] && s.b[3290]) && (!s.b[3291])) {
            s.copy_ad(335, 839);
            s.store_div_scaled_inputs_indices(336, 839, p.p528, 846, 1.0);
            s.store_mul_add_scaled_product_rhs(896, 867, s.ad_value(335), 1.0, s.ad_value(867), s.ad_value(336), 0.5);
        }

        if (s.b[3289] && (!s.b[3290])) {
            s.store_scalar(896, 0.0);
        }

        s.b[3293] = (s.v[839] > 0.0);
        s.v[3293] = if s.b[3293] { 1.0 } else { 0.0 };

        s.b[3294] = (s.v[859] < 0.0);
        s.v[3294] = if s.b[3294] { 1.0 } else { 0.0 };

        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {
            s.store_sub_from_scalar_div_indices(770, 1.0, 859, 846);
        }

        s.b[3295] = (p.p528 == 0.5);
        s.v[3295] = if s.b[3295] { 1.0 } else { 0.0 };

        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && s.b[3295]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && (!s.b[3295])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p.p528));
            }
        }

        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {
            s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);
        }

        if (((!s.b[3289]) && s.b[3293]) && (!s.b[3294])) {
            s.copy_ad(335, 839);
            s.store_div_scaled_inputs_indices(336, 839, p.p528, 846, 1.0);
            s.store_mul_add_scaled_product_rhs(896, 859, s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5);
        }

        if ((!s.b[3289]) && (!s.b[3293])) {
            s.store_scalar(896, 0.0);
        }

        s.b[3296] = (p.p48 > 0.0);
        s.v[3296] = if s.b[3296] { 1.0 } else { 0.0 };

        if s.b[3296] {
            s.store_scaled_add(66, 892, 894, s.v[365]);
            s.store_scaled_add(65, 891, 893, s.v[365]);
            s.store_scale(68, 896, s.v[365]);
            s.store_scale(67, 895, s.v[365]);
        }

        if (!s.b[3296]) {
            s.store_add_scaled_inputs3_indices(66, 892, s.v[365], 894, s.v[365], 896, s.v[365]);
            s.store_add_scaled_inputs3_indices(65, 891, s.v[365], 893, s.v[365], 895, s.v[365]);
            s.store_scalar(68, 0.0);
            s.store_scalar(67, 0.0);
        }

        s.v[903] = (p.p540 / 1e-6);

        s.v[906] = s.v[820];

        s.v[904] = (1450.0 / 10000.0);

        s.v[905] = (500.0 / 10000.0);

        s.v[943] = 0.001;

        s.store_scale_ad(908, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), 1.45e16);

        s.store_scaled_square(907, 908, 1.0 / (s.v[903]));

        s.store_powf(335, 676, (-1.5));

        s.store_scaled_mul(909, 335, 155, s.v[904]);

        s.store_scaled_mul(910, 335, 155, s.v[905]);

        s.store_div_scaled_product_add_scaled_denominator_indices(911, 909, 910, 2.0, 909, 1.0, 910, 1.0, 1.0);

        s.store_powf(336, 676, p.p547);

        s.store_scale(913, 336, p.p544);

        s.store_sqrt_mul(912, 913, 911);

        s.store_mul_scaled_ln_ad_rhs(934, 155, s.v[906], A::div_from_scalar(s.v[903], s.ad_value(907)));

        s.store_mul_add_scaled_inputs_rhs(935, 155, A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), s.v[906], A::div_from_scalar(p.p545, s.ad_value(912)), s.v[906]);

        s.b[3297] = (p.p539 > 0.0);
        s.v[3297] = if s.b[3297] { 1.0 } else { 0.0 };

        if s.b[3297] {
            s.store_scalar(936, s.v[820]);
            s.store_exp_mul(937, 860, 850);
        }

        s.b[3298] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);
        s.v[3298] = if s.b[3298] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3298]) {
            s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div_scaled_inputs2(s.ad_value(935), 1.0, s.ad_value(934), (-1.0), s.ad_value(936), 1.0))));
        }

        if (s.b[3297] && (!s.b[3298])) {
            s.store_scalar(938, 1.0);
        }

        s.b[3299] = ((p.p542 == 0.0) || (s.v[860] < s.v[934]));
        s.v[3299] = if s.b[3299] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3299]) {
            s.store_scale(941, 937, p.p541);
        }

        if (s.b[3297] && (!s.b[3299])) {
            s.store_mul_scaled_exp_ad_rhs(941, 937, p.p541, A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542)));
        }

        if s.b[3297] {
            if (s.v[941] > 1e20) {
                s.store_scalar(941, 1e20);
            } else {
            }
        }

        if s.b[3297] {
            s.store_mul(939, 907, 941);
            s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p.p13));
        }

        s.b[3300] = (p.p543 > 0.0);
        s.v[3300] = if s.b[3300] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3300]) {
            s.store_scale(922, 920, p.p543);
            s.store_scaled_voltage(924, ctx, nodes, Some(15), None, p.p543);
            s.store_scaled_sub(926, 924, 922, 1.0 / (p.p543));
            s.store_scale(928, 924, 1.0 / (p.p543));
        }

        if (s.b[3297] && (!s.b[3300])) {
            s.copy_ad(922, 920);
            s.copy_ad(928, 922);
        }

        s.b[3301] = ((p.p542 == 0.0) || (s.v[860] < s.v[935]));
        s.v[3301] = if s.b[3301] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3301]) {
            s.store_scale(942, 938, p.p541);
        }

        if (s.b[3297] && (!s.b[3301])) {
            s.store_mul_scaled_exp_ad_rhs(942, 938, p.p541, A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542)));
        }

        if s.b[3297] {
            if (s.v[942] > 1e20) {
                s.store_scalar(942, 1e20);
            } else {
            }
        }

        if s.b[3297] {
            s.store_mul(940, 907, 942);
            s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p.p13));
        }

        s.b[3302] = (p.p543 > 0.0);
        s.v[3302] = if s.b[3302] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3302]) {
            s.store_scale(923, 921, p.p543);
            s.store_scaled_voltage(925, ctx, nodes, Some(16), None, p.p543);
            s.store_scaled_sub(927, 925, 923, 1.0 / (p.p543));
            s.store_scale(929, 925, 1.0 / (p.p543));
        }

        if (s.b[3297] && (!s.b[3302])) {
            s.copy_ad(923, 921);
            s.copy_ad(929, 923);
        }

        if s.b[3297] {
            s.store_sub_from_scalar(914, p.p506, 860);
            s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));
            s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);
            s.store_scaled_add(914, 914, 782, 0.5);
        }

        s.b[3303] = (s.v[914] < 0.0);
        s.v[3303] = if s.b[3303] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3303]) {
            s.store_scalar(914, 0.0);
            s.store_scalar(334, 0.0);
        }

        if s.b[3297] {
            s.store_sqrt_scaled_input(915, 914, ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903]))));
            s.store_offset_sub_from_scalar_ad(781, p.p545, s.ad_value(915), (-1e-7));
            s.store_scalar(782, ((4.0 * p.p545) * 1e-7));
        }

    }

    pub(super) fn stamp_reactive_block_92(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3297] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[3297] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(915, 781, (-0.5), 782, (-0.5), p.p545);
        }

        s.b[3304] = (p.p546 > 0.0);
        s.v[3304] = if s.b[3304] { 1.0 } else { 0.0 };

        if (s.b[3297] && s.b[3304]) {
            s.store_scale(930, 915, p.p546);
            s.store_scaled_voltage(931, ctx, nodes, Some(17), None, p.p546);
            s.store_scaled_sub(932, 931, 930, 1.0 / (p.p546));
            s.store_scale(933, 931, 1.0 / (p.p546));
        }

        if (s.b[3297] && (!s.b[3304])) {
            s.copy_ad(930, 915);
            s.copy_ad(933, 930);
        }

        if s.b[3297] {
            s.store_scalar(916, ((-((s.v[903] * p.p13) * 1.6021918e-19)) * p.p545));
            s.store_mul_ad_product_rhs_mixed_ia(917, 912, 928, A::sub(A::exp(A::div_from_scalar((-p.p545), s.ad_value(912))), A::exp_div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0)));
            s.store_mul_ad_product_rhs_mixed_ia(918, 912, 929, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p.p545, s.ad_value(933)), -1.0, s.ad_value(912), 1.0), (-1.0)));
            s.store_add_scaled_inputs3_indices(919, 916, (-1.0), 917, (-1.0), 918, (-1.0));
            s.store_add_scaled_inputs(65, 65, 1.0, 919, s.v[365]);
        }

        s.b[3305] = ((p.p539 > 0.0) && (p.p543 > 0.0));
        s.v[3305] = if s.b[3305] { 1.0 } else { 0.0 };

        s.b[3306] = ((p.p539 > 0.0) && (p.p546 > 0.0));
        s.v[3306] = if s.b[3306] { 1.0 } else { 0.0 };

        s.b[3307] = (p.p46 == 1.0);
        s.v[3307] = if s.b[3307] { 1.0 } else { 0.0 };

        s.b[3308] = ((s.v[486] > 0.0) && (s.v[454] > 0.0));
        s.v[3308] = if s.b[3308] { 1.0 } else { 0.0 };

        if (s.b[3307] && s.b[3308]) {
            s.store_mul(335, 665, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 666, 1434, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[3309] = ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0));
        s.v[3309] = if s.b[3309] { 1.0 } else { 0.0 };

        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
            s.store_sub(781, 341, 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3310] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[3310] = if s.b[3310] { 1.0 } else { 0.0 };

        s.b[3311] = (4.0 == 1.0);
        s.v[3311] = if s.b[3311] { 1.0 } else { 0.0 };

        if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && s.b[3311]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3312] = (4.0 == 2.0);
        s.v[3312] = if s.b[3312] { 1.0 } else { 0.0 };

        if (((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && s.b[3312]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3313] = (4.0 == 4.0);
        s.v[3313] = if s.b[3313] { 1.0 } else { 0.0 };

        if ((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && s.b[3313]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3314] = (4.0 == 8.0);
        s.v[3314] = if s.b[3314] { 1.0 } else { 0.0 };

        if (((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && (!s.b[3313])) && s.b[3314]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign100760_loop_guard: usize = 0;
        while {
            let assign100760_cond_e152863: f64 = if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign100760_cond_e152863 != 0.0
        } {
            assign100760_loop_guard += 1;
            assert!(assign100760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3307] && s.b[3308]) && s.b[3309]) && (!s.b[3310])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub(340, 341, 780);
        }

        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        }

        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {
        }

        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3307] && s.b[3308]) {
            s.store_sqrt(340, 340);
            s.store_add_ad_rhs(282, 335, A::mul_sub_from_scalar_rhs(s.ad_value(337), 1.0, s.ad_value(340)));
            s.store_div_from_scalar_offset_input(336, s.v[582], 667, s.v[582]);
            s.store_add_scaled_inputs_product_indices(283, 1435, s.v[488], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[3315] = (s.v[283] < 0.0);
        s.v[3315] = if s.b[3315] { 1.0 } else { 0.0 };

        if ((s.b[3307] && s.b[3308]) && s.b[3315]) {
            s.store_scalar(283, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3307] && s.b[3308]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);
        }

        if (s.b[3307] && s.b[3308]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }

        if (s.b[3307] && s.b[3308]) {
            s.store_div(339, 668, 958);
            s.store_mul(340, 669, 958);
            s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
        }

        s.b[3317] = (s.v[78] == 0.0);
        s.v[3317] = if s.b[3317] { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && s.b[3317]) {
            s.store_scalar(346, p.p270);
            s.store_scalar(344, p.p271);
            s.copy_ad(337, 170);
            s.store_mul_product3_indices(335, 337, 346, 344, 337, 1.0);
            s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);
        }

        if (s.v[81] != 0.0) {
            s.store_scalar(336, s.v[565]);
        }

        s.b[3318] = ((p.p26 != 0.0) && (s.v[78] == 0.0));
        s.v[3318] = if s.b[3318] { 1.0 } else { 0.0 };

        if s.b[3318] {
            s.store_scalar(309, s.v[522]);
            s.store_scalar(311, s.v[563]);
            s.store_scale(335, 238, 6.241449993689894e18);
            s.store_sqrt_offset_ad(782, A::square(A::sub(s.ad_value(87), s.ad_value(1431))), ((4.0 * 0.001) * 0.001));
            s.store_scaled_offset_ad(334, A::div_scaled_inputs2(s.ad_value(87), 1.0, s.ad_value(1431), (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3_indices(339, 87, 0.5, 1431, ((-1.0) * 0.5), 782, 0.5);
        }

        s.b[3319] = (s.v[339] < 0.0);
        s.v[3319] = if s.b[3319] { 1.0 } else { 0.0 };

        if (s.b[3318] && s.b[3319]) {
            s.store_scalar(339, 0.0);
            s.store_scalar(334, 0.0);
        }

        if s.b[3318] {
            s.store_mul_scaled_ad_lhs(336, A::add_scaled_inputs3(s.ad_value(185), 1.0, A::div(s.ad_value(238), s.ad_value(339)), 1.0, s.ad_value(311), 1.0), 155, 6.241449993689894e18);
            s.store_sub_ad_lhs(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);
        }

        s.b[3320] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[3320] = if s.b[3320] { 1.0 } else { 0.0 };

        if (s.b[3318] && s.b[3320]) {
            s.store_add_scaled_value_products(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), s.ad_value(253), 1.0);
        }

        if (s.b[3318] && (!s.b[3320])) {
            s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);
        }

        s.b[3321] = (((p.p30 != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3321] = if s.b[3321] { 1.0 } else { 0.0 };

        if s.b[3321] {
            s.store_div_scaled_offset_numerator(313, A::sub(s.ad_value(168), s.ad_value(87)), 1.0, (10.0 * 2.220446049250313e-16), s.ad_value(170), 1.0);
        }

        if s.b[3321] {
            if (s.v[313] >= 0.0) {
            } else {
                s.store_scalar(313, 0.0);
            }
        }

        if s.b[3321] {
            s.store_scaled_mul(346, 254, 313, 1e-7);
        }

        s.b[3322] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3322] = if s.b[3322] { 1.0 } else { 0.0 };

        if (s.b[3321] && s.b[3322]) {
            s.store_scalar(341, 1.0);
        }

        s.b[3323] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3323] = if s.b[3323] { 1.0 } else { 0.0 };

        if ((s.b[3321] && (!s.b[3322])) && s.b[3323]) {
            s.copy_ad(341, 346);
        }

        if ((s.b[3321] && (!s.b[3322])) && (!s.b[3323])) {
            if (s.v[313] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_powf(341, 313, (p.p178 - 1.0));
            }
        }

        if s.b[3321] {
            s.store_mul(342, 346, 341);
            s.store_offset(343, 342, 1.0);
        }

        if s.b[3321] {
            if (s.v[343] == 0.0) {
                s.store_scalar(344, 0.0);
            } else {
                s.store_powf(344, 343, (((-1.0) / p.p178) - 1.0));
            }
        }

        if s.b[3321] {
            s.store_mul(345, 343, 344);
            s.store_mul(316, 254, 345);
            s.store_scaled_add(314, 253, 316, 0.5);
            s.store_square(334, 125);
            s.store_div_scaled_product_by_product(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), s.ad_value(314), 1.0);
        }

        if (!s.b[3321]) {
            s.store_scalar(315, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_93(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[3324] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3324] = if s.b[3324] { 1.0 } else { 0.0 };

        if s.b[3324] {
            s.store_sqrt(322, 319);
            s.store_add(336, 127, 322);
            s.store_square(337, 317);
            s.store_square(338, 319);
            s.store_scaled_mul(339, 317, 319, 42.0);
            s.store_add_scaled_inputs3_indices(339, 339, 1.0, 337, 4.0, 338, 4.0);
            s.store_add_product3_rhs_mixed_iia(339, 339, 322, 127, A::add(s.ad_value(317), s.ad_value(319)), 20.0);
            s.store_square(344, 336);
            s.store_square(344, 344);
            s.store_div_ad_rhs(323, 339, A::mul(s.ad_value(344), s.ad_value(336)));
            s.store_mul_ad_product_lhs_mixed_ai(324, A::div_from_scalar(s.v[632], s.ad_value(170)), 253, 185);
            s.store_add_ad_lhs(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);
        }

        s.store_scale(0, 134, s.v[365]);

        s.store_scale(699, 400, s.v[365]);

        s.v[705] = 0.0;

        s.v[706] = 0.0;

        s.v[707] = 0.0;

        s.v[811] = 0.0;

        s.v[810] = 0.0;

        s.v[812] = 0.0;

        s.v[703] = 0.0;

        s.v[704] = 0.0;

        s.b[3325] = ((s.v[81] != 0.0) || (p.p22 == 2.0));
        s.v[3325] = if s.b[3325] { 1.0 } else { 0.0 };

        if s.b[3325] {
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.copy_ad(708, 247);
            s.store_scale(132, 132, s.v[365]);
        }

        if (!s.b[3325]) {
            s.store_scaled_add(700, 20, 132, (-s.v[365]));
            s.store_scale(701, 19, s.v[365]);
            s.store_scaled_sub(702, 132, 19, s.v[365]);
        }

        if (p.p29 != 0.0) {
            s.store_scale(572, 91, s.v[572]);
            s.store_sqrt_square_offset(782, 572, ((4.0 * 1e-12) * 1e-12));
            s.store_offset_scaled_div(334, 572, 782, 0.5, 0.5);
            s.store_scaled_add(572, 572, 782, 0.5);
        }

        s.b[3326] = (s.v[572] < 0.0);
        s.v[3326] = if s.b[3326] { 1.0 } else { 0.0 };

        if ((p.p29 != 0.0) && s.b[3326]) {
            s.store_scalar(572, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (p.p29 != 0.0) {
            s.store_voltage(817, ctx, nodes, Some(13), None);
            s.store_add_scaled_inputs3_indices(352, 352, 1.0, 816, -1.0, 817, 1.0);
            s.copy_ad(355, 817);
        }

        if (p.p29 == 0.0) {
            s.copy_ad(817, 816);
        }

        s.b[3327] = (p.p22 > 0.0);
        s.v[3327] = if s.b[3327] { 1.0 } else { 0.0 };

        if s.b[3327] {
            s.store_scaled_add_ad_lhs(811, A::add_scaled_inputs4(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0, s.ad_value(291), 1.0), 292, s.v[365]);
            s.store_scaled_sub(810, 355, 292, s.v[365]);
            s.store_scaled_sub(812, 356, 291, s.v[365]);
            s.store_add_scaled_inputs4_indices(700, 700, 1.0, 305, s.v[365], 360, ((-1.0) * s.v[365]), 362, (-s.v[365]));
            s.store_add_scaled_inputs3_indices(701, 701, 1.0, 361, s.v[365], 305, (-s.v[365]));
            s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);
            s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);
            s.store_scale(706, 358, s.v[365]);
            s.store_scale(707, 359, s.v[365]);
            s.store_offset_sub_scaled_inputs_indices(703, 299, (-s.v[365]), 298, s.v[365], s.v[703]);
            s.store_offset_sub_scaled_inputs_indices(704, 301, (-s.v[365]), 297, s.v[365], s.v[704]);
        }

        s.store_scaled_add(709, 280, 287, s.v[365]);

        s.store_scale(710, 281, s.v[365]);

        s.store_scale(807, 387, (4.0 * 1.3806226e-23));

        s.store_scale(712, 315, s.v[365]);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(5), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(7), None));

        s.store_scale(23, 23, p.p87);

        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }

        s.v[713] = 0.0;

        s.b[3330] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3330] = if s.b[3330] { 1.0 } else { 0.0 };

        if s.b[3330] {
            s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));
            s.store_scale(344, 757, 1.0 / (s.v[365]));
            s.store_div_scaled_product3_indices(328, 155, 344, 344, (0.1185185185185185 * 1.6021918e-19), 324, 1.0);
        }

        s.b[3331] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));
        s.v[3331] = if s.b[3331] { 1.0 } else { 0.0 };

        if (s.b[3330] && s.b[3331]) {
            s.store_div(329, 254, 253);
            s.store_div_scaled_inputs2_mixed_aii(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, 329, (-1.0), 790, 1.0);
            s.store_add_ad_rhs(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));
        }

        if (s.b[3330] && (!s.b[3331])) {
            s.store_div(331, 254, 316);
        }

        if s.b[3330] {
            s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);
        }

        if s.b[3330] {
            if (s.v[713] < 0.0) {
                s.store_scalar(713, 0.0);
            } else {
            }
        }

        if s.b[3330] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(713, 0.0);
            }
        }

        s.store_mul(952, 807, 712);

        if ((s.v[952] > 0.0) && (s.v[713] > 0.0)) {
            s.store_sqrt_div(953, 713, 952);
        } else {
            s.store_scalar(953, 0.0);
        }

        if (s.v[949] > 0.0) {
            s.store_mul_sub_from_scalar_rhs(954, 953, 1.0, 247);
        } else {
            s.store_mul(954, 953, 247);
        }

        if (s.v[949] > 0.0) {
            s.store_mul(955, 953, 247);
        } else {
            s.store_mul_sub_from_scalar_rhs(955, 953, 1.0, 247);
        }

        s.v[716] = 0.0;

        s.v[715] = 0.0;

        s.b[3332] = (s.v[449] == 1.0);
        s.v[3332] = if s.b[3332] { 1.0 } else { 0.0 };

        s.b[3333] = (s.v[76] == 0.0);
        s.v[3333] = if s.b[3333] { 1.0 } else { 0.0 };

        s.b[3334] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3334] = if s.b[3334] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }

        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_add_ad_rhs(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p439));
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3334])) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_scalar(164, (s.v[630] * p.p7));
            s.store_scalar(604, p.p71);
            s.store_scalar(605, s.v[460]);
            s.store_mul(606, 794, 653);
            s.store_offset_product3(607, s.ad_value(795), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);
            s.store_div(608, 804, 604);
            s.store_mul(609, 606, 608);
        }

        s.b[3335] = (s.v[804] >= 0.0);
        s.v[3335] = if s.b[3335] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3335]) {
            s.store_div(335, 609, 607);
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3335])) {
            s.store_div_scaled_inputs_indices(335, 609, -1.0, 607, 1.0);
        }

        s.b[3336] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3336] = if s.b[3336] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3336]) {
            s.store_scalar(337, 1.0);
        }

        s.b[3337] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3337] = if s.b[3337] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && s.b[3337]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && (!s.b[3337])) {
            s.store_pow_offset_rhs(337, 335, 959, (-1.0));
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[3338] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3338] = if s.b[3338] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3338]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[3339] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3339] = if s.b[3339] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && s.b[3339]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)));
            }
        }

        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_mul(610, 606, 339);
            s.copy_ad(611, 605);
            s.copy_ad(612, 614);
            s.store_div_from_scalar(335, 1.6021918e-19, 604);
            s.store_mul_product3_indices(613, 611, 335, 612, 610, 1.0);
        }

        s.b[3340] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));
        s.v[3340] = if s.b[3340] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_sub_from_scalar(781, 1e-25, 613);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_reactive_block_94(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3341] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3341] = if s.b[3341] { 1.0 } else { 0.0 };

        s.b[3342] = (2.0 == 1.0);
        s.v[3342] = if s.b[3342] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && s.b[3342]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3343] = (2.0 == 2.0);
        s.v[3343] = if s.b[3343] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && s.b[3343]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3344] = (2.0 == 4.0);
        s.v[3344] = if s.b[3344] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && s.b[3344]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3345] = (2.0 == 8.0);
        s.v[3345] = if s.b[3345] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && (!s.b[3344])) && s.b[3345]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && (!s.b[3341])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-25);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);
            s.store_sub_from_scalar(613, 1e-25, 780);
        }

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_div_from_scalar(5, 1.0, 613);
            s.store_div(5, 5, 164);
            s.store_add(5, 5, 648);
        }

        s.b[3347] = (s.v[5] < p.p444);
        s.v[3347] = if s.b[3347] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3347]) {
            s.store_scalar(5, p.p444);
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        s.b[3352] = (s.v[75] == 0.0);
        s.v[3352] = if s.b[3352] { 1.0 } else { 0.0 };

        if (s.b[3332] && (!s.b[3352])) {
            s.copy_ad(3348, 729);
            s.copy_ad(3349, 728);
        }

        s.b[3353] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3353] = if s.b[3353] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_add_ad_rhs(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p438));
        }

        s.b[3355] = (s.v[956] < 0.1);
        s.v[3355] = if s.b[3355] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3353]) && s.b[3355]) {
            s.store_scalar(956, 0.1);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3353])) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_scalar(164, (s.v[630] * p.p7));
            s.store_scalar(785, (p.p67 + p.p68));
            s.store_offset(789, 451, 1e-12);
            s.store_scalar(408, s.v[459]);
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3349), p.p410, A::scale(s.ad_value(3349), p.p411)), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(654, 335, 782, 0.5);
        }

        s.b[3356] = (s.v[654] < 0.0);
        s.v[3356] = if s.b[3356] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3356]) {
            s.store_scalar(654, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul3_lhs(593, 787, 653, 654);
            s.store_offset_product3(3351, s.ad_value(788), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);
            s.copy_ad(594, 453);
            s.store_scalar(595, p.p421);
            s.store_scale(335, 593, 10000.0);
            s.store_scale(336, 3351, 100.0);
        }

        s.b[3359] = (s.v[799] < 0.0);
        s.v[3359] = if s.b[3359] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {
            s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[3360] = (s.v[108] < 1e-12);
        s.v[3360] = if s.b[3360] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3359]) && s.b[3360]) {
            s.store_scalar(108, 1e-12);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {
            s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {
            s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[3361] = (s.v[108] < 1e-12);
        s.v[3361] = if s.b[3361] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) && s.b[3361]) {
            s.store_scalar(108, 1e-12);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {
            s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div(591, 598, 785);
            s.store_mul(592, 593, 591);
        }

        s.b[3362] = (s.v[799] >= 0.0);
        s.v[3362] = if s.b[3362] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3362]) {
            s.store_div(335, 592, 3351);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3362])) {
            s.store_div_scaled_inputs_indices(335, 592, -1.0, 3351, 1.0);
        }

        s.b[3363] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3363] = if s.b[3363] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3363]) {
            s.store_scalar(337, 1.0);
        }

        s.b[3364] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3364] = if s.b[3364] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && s.b[3364]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && (!s.b[3364])) {
            s.store_pow_offset_rhs(337, 335, 956, (-1.0));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[3365] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3365] = if s.b[3365] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3365]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[3366] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3366] = if s.b[3366] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && s.b[3366]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul(3350, 593, 339);
            s.store_offset(338, 335, 1.0);
            s.store_div_from_scalar(339, 1.0, 338);
            s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p.p423), 1.0), 1.0);
            s.store_offset(781, 338, (-0.001));
            s.store_scalar(782, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(339, 781, 782, 0.5);
            s.store_mul(717, 408, 339);
            s.store_scale(718, 698, (6.241449993689894e18 * p.p430));
            s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));
            s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));
        }

        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(597, 717, 718);
        }

        s.b[3367] = ((p.p441 > 0.0) && (p.p440 > 1.0));
        s.v[3367] = if s.b[3367] { 1.0 } else { 0.0 };

        s.b[3368] = ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0));
        s.v[3368] = if s.b[3368] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p.p440), 408, p.p441);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 408, 408, (p.p441 * p.p441));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (s.v[719] < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3369] = ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0));
        s.v[3369] = if s.b[3369] { 1.0 } else { 0.0 };

        s.b[3370] = (p.p442 == 1.0);
        s.v[3370] = if s.b[3370] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && s.b[3370]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3371] = (p.p442 == 2.0);
        s.v[3371] = if s.b[3371] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && s.b[3371]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3372] = (p.p442 == 4.0);
        s.v[3372] = if s.b[3372] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && s.b[3372]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3373] = (p.p442 == 8.0);
        s.v[3373] = if s.b[3373] { 1.0 } else { 0.0 };

        if ((((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && (!s.b[3372])) && s.b[3373]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (!s.b[3369])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p.p442)));
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 408, p.p441, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 408, 725, 726, p.p441, 770, 1.0);
            s.store_add_scaled_inputs3_indices(336, 408, p.p440, 408, (-p.p441), 780, 1.0);
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && (!s.b[3368])) {
            s.copy_ad(336, 597);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3367]) {
            s.copy_ad(597, 336);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_neg(334, 697);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[3374] = (s.v[334] < 0.0);
        s.v[3374] = if s.b[3374] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3374]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));
            s.store_sqrt_mul(599, 650, 334);
            s.store_offset_sub(336, 3348, 3349, p.p137);
            s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3375] = (s.v[336] < 0.0);
        s.v[3375] = if s.b[3375] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3375]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));
            s.store_sqrt_mul(600, 651, 336);
            s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));
            s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));
        }

        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_scalar(601, (p.p419 + 1e-25));
            s.store_mul_sub_from_scalar_ad_rhs(596, 649, 1.0, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))));
            s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));
            s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);
            s.store_scaled_add(596, 596, 782, 0.5);
        }

        s.b[3376] = (s.v[596] < 0.0);
        s.v[3376] = if s.b[3376] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3376]) {
            s.store_scalar(596, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p.p422);
            s.store_mul_product3_indices(739, 597, 335, 596, 3350, 1.0);
        }

        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));
        s.v[3377] = if s.b[3377] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_sub_from_scalar(781, 1e-25, 739);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3378] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3378] = if s.b[3378] { 1.0 } else { 0.0 };

        s.b[3379] = (2.0 == 1.0);
        s.v[3379] = if s.b[3379] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && s.b[3379]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3380] = (2.0 == 2.0);
        s.v[3380] = if s.b[3380] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && s.b[3380]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3381] = (2.0 == 4.0);
        s.v[3381] = if s.b[3381] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && s.b[3381]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3382] = (2.0 == 8.0);
        s.v[3382] = if s.b[3382] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && (!s.b[3381])) && s.b[3382]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && (!s.b[3378])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-25);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);
            s.store_sub_from_scalar(739, 1e-25, 780);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div_from_scalar(4, 1.0, 739);
            s.store_div(4, 4, 164);
        }

        s.b[3383] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));
        s.v[3383] = if s.b[3383] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
            s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));
            s.store_square(722, 781);
            s.store_scalar(723, (1000.0 * 1000.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3384] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3384] = if s.b[3384] { 1.0 } else { 0.0 };

        s.b[3385] = (2.0 == 1.0);
        s.v[3385] = if s.b[3385] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && s.b[3385]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3386] = (2.0 == 2.0);
        s.v[3386] = if s.b[3386] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && s.b[3386]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3387] = (2.0 == 4.0);
        s.v[3387] = if s.b[3387] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && s.b[3387]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3388] = (2.0 == 8.0);
        s.v[3388] = if s.b[3388] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && (!s.b[3387])) && s.b[3388]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && (!s.b[3384])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1000.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1000.0, 770, 1.0);
            s.store_offset(4, 780, (1000000.0 - 1000.0));
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {
            s.store_scalar(334, 1.0);
        }

        s.b[3389] = ((p.p54 == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.v[3389] = if s.b[3389] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3389]) {
            s.store_sub_from_scalar(385, p.p334, 384);
            s.store_div_scaled_inputs_indices(4, 4, s.v[165], 385, 1.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_add(4, 4, 644);
        }

        s.b[3391] = (s.v[4] < p.p444);
        s.v[3391] = if s.b[3391] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3391]) {
            s.store_scalar(4, p.p444);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
        }

        s.b[3392] = (s.v[4] < p.p444);
        s.v[3392] = if s.b[3392] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3392]) {
            s.store_scalar(4, p.p444);
        }

        s.b[3393] = (s.v[5] < p.p444);
        s.v[3393] = if s.b[3393] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3393]) {
            s.store_scalar(5, p.p444);
        }

        s.b[3394] = (s.v[370] > 0.0);
        s.v[3394] = if s.b[3394] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3394]) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        if ((!s.b[3332]) && (!s.b[3394])) {
            s.store_scale(715, 5, 1.0 / (s.v[365]));
            s.store_scale(716, 4, 1.0 / (s.v[365]));
        }

        s.copy_ad(4, 715);

        s.copy_ad(5, 716);

        s.b[3395] = (s.v[949] > 0.0);
        s.v[3395] = if s.b[3395] { 1.0 } else { 0.0 };

        if s.b[3395] {
            s.copy_ad(134, 0);
            s.copy_ad(19, 701);
            s.copy_ad(18, 700);
            s.copy_ad(741, 702);
            s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));
            s.copy_ad(280, 709);
            s.copy_ad(281, 710);
            s.copy_ad(400, 699);
        }

        if (s.b[3395] && (s.v[81] != 0.0)) {
            s.copy_ad(247, 708);
        }

        if (!s.b[3395]) {
            s.store_neg(134, 0);
            s.copy_ad(19, 702);
            s.copy_ad(18, 700);
            s.copy_ad(741, 701);
            s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
            s.store_scalar(400, 0.0);
        }

        if ((!s.b[3395]) && (s.v[81] != 0.0)) {
            s.store_sub_from_scalar(247, 1.0, 708);
        }

        s.store_add(18, 18, 811);

        s.store_add(19, 19, 810);

        s.store_add(741, 741, 812);

        s.store_add_scaled_inputs3_indices(20, 18, (-1.0), 19, (-1.0), 741, (-1.0));

        s.copy_ad(299, 703);

        s.copy_ad(301, 704);

        s.copy_ad(742, 706);

        s.copy_ad(743, 705);

        s.store_add_scaled_inputs3_indices(744, 705, (-1.0), 706, (-1.0), 707, (-1.0));

        s.b[3396] = (p.p53 > 0.0);
        s.v[3396] = if s.b[3396] { 1.0 } else { 0.0 };

        s.b[3397] = (s.v[766] > 0.0001);
        s.v[3397] = if s.b[3397] { 1.0 } else { 0.0 };

        if (s.b[3396] && s.b[3397]) {
            s.store_div_from_scalar(740, 1.0, 766);
        }

        if (s.b[3396] && (!s.b[3397])) {
            s.store_scalar(740, (1.0 / 0.0001));
        }

        s.b[3398] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);
        s.v[3398] = if s.b[3398] { 1.0 } else { 0.0 };

        s.b[3399] = (s.v[529] == 1.0);
        s.v[3399] = if s.b[3399] { 1.0 } else { 0.0 };

        if ((s.b[3396] && s.b[3398]) && s.b[3399]) {
            s.copy_ad(745, 733);
        }

        if ((s.b[3396] && s.b[3398]) && (!s.b[3399])) {
            s.store_add_scaled_product_right_sub(745, 729, 1.0, 683, 733, 729, 1.0);
        }

        if (s.b[3396] && (!s.b[3398])) {
            s.copy_ad(745, 729);
        }

        if s.b[3396] {
            s.store_mul(746, 134, 745);
        }

        s.b[3400] = (p.p53 == 1.0);
        s.v[3400] = if s.b[3400] { 1.0 } else { 0.0 };

        if (s.b[3396] && s.b[3400]) {
            s.store_scale(335, 740, p.p433);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 746, (-1.0), 740, (-p.p337));
            s.store_scaled_mul(782, 335, 740, (4.0 * p.p337));
        }

        if (s.b[3396] && s.b[3400]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3396] && s.b[3400]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 335, 1.0, 781, (-0.5), 782, (-0.5));
            s.copy_ad(746, 336);
        }

        if (!s.b[3396]) {
            s.store_scalar(740, 0.0);
            s.store_scalar(746, 0.0);
        }

        if (s.v[81] != 0.0) {
            s.store_mul(751, 747, 247);
            s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);
            s.store_mul_sub_from_scalar_rhs(752, 747, 1.0, 247);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(751, 0.0);
            s.store_scalar(753, 0.0);
            s.store_scalar(752, 0.0);
        }

        s.store_scaled_mul(0, 949, 134, p.p87);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(5), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(7), None));

        s.store_scale(23, 23, p.p87);

        s.b[3403] = (s.v[949] == 1.0);
        s.v[3403] = if s.b[3403] { 1.0 } else { 0.0 };

        if s.b[3403] {
            s.copy_ad(757, 23);
        }

        if (!s.b[3403]) {
            s.copy_ad(757, 22);
        }

        s.b[3405] = (p.p48 > 0.0);
        s.v[3405] = if s.b[3405] { 1.0 } else { 0.0 };

        if (p.p28 != 0.0) {
            s.store_scalar(800, 1.0);
            s.store_scalar(801, 1.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11,) = {
    if s.b[3305] {
        let eq0_e1015: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[924]);
        let eq0_e1016: f64 = (s.v[926] + eq0_e1015);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + (s.dn[924][0] * ddt_scale));
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + (s.dn[924][1] * ddt_scale));
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + (s.dn[924][2] * ddt_scale));
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + (s.dn[924][3] * ddt_scale));
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + (s.dn[924][4] * ddt_scale));
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + (s.dn[924][5] * ddt_scale));
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + (s.dn[924][6] * ddt_scale));
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + (s.dn[924][7] * ddt_scale));
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + (s.dn[924][8] * ddt_scale));
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + (s.dn[924][9] * ddt_scale));
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + (s.dn[924][10] * ddt_scale));
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + (s.dn[924][11] * ddt_scale));
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + (s.dn[924][12] * ddt_scale));
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + (s.dn[924][13] * ddt_scale));
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + (s.dn[924][14] * ddt_scale));
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + (s.dn[924][15] * ddt_scale));
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + (s.dn[924][16] * ddt_scale));
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + (s.dn[924][17] * ddt_scale));
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + (s.db[924][0] * ddt_scale));
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + (s.db[924][1] * ddt_scale));
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + (s.db[924][2] * ddt_scale));
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + (s.db[924][3] * ddt_scale));
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + (s.db[924][4] * ddt_scale));
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + (s.db[924][5] * ddt_scale));
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + (s.db[924][6] * ddt_scale));
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + (s.db[924][7] * ddt_scale));
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + (s.db[924][8] * ddt_scale));
        let eq0_e1016_d_b9: f64 = (s.db[926][9] + (s.db[924][9] * ddt_scale));
        let eq0_e1016_d_b10: f64 = (s.db[926][10] + (s.db[924][10] * ddt_scale));
        let eq0_e1016_d_b11: f64 = (s.db[926][11] + (s.db[924][11] * ddt_scale));
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_d_b9, eq0_e1016_d_b10, eq0_e1016_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1018;
        let eq0_node_derivatives: [f64; 18] = [eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17];
        let eq0_branch_derivatives: [f64; 12] = [eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11,) = {
    if s.b[3305] {
        let eq1_e1022: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[925]);
        let eq1_e1023: f64 = (s.v[927] + eq1_e1022);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + (s.dn[925][0] * ddt_scale));
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + (s.dn[925][1] * ddt_scale));
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + (s.dn[925][2] * ddt_scale));
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + (s.dn[925][3] * ddt_scale));
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + (s.dn[925][4] * ddt_scale));
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + (s.dn[925][5] * ddt_scale));
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + (s.dn[925][6] * ddt_scale));
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + (s.dn[925][7] * ddt_scale));
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + (s.dn[925][8] * ddt_scale));
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + (s.dn[925][9] * ddt_scale));
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + (s.dn[925][10] * ddt_scale));
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + (s.dn[925][11] * ddt_scale));
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + (s.dn[925][12] * ddt_scale));
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + (s.dn[925][13] * ddt_scale));
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + (s.dn[925][14] * ddt_scale));
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + (s.dn[925][15] * ddt_scale));
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + (s.dn[925][16] * ddt_scale));
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + (s.dn[925][17] * ddt_scale));
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + (s.db[925][0] * ddt_scale));
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + (s.db[925][1] * ddt_scale));
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + (s.db[925][2] * ddt_scale));
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + (s.db[925][3] * ddt_scale));
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + (s.db[925][4] * ddt_scale));
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + (s.db[925][5] * ddt_scale));
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + (s.db[925][6] * ddt_scale));
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + (s.db[925][7] * ddt_scale));
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + (s.db[925][8] * ddt_scale));
        let eq1_e1023_d_b9: f64 = (s.db[927][9] + (s.db[925][9] * ddt_scale));
        let eq1_e1023_d_b10: f64 = (s.db[927][10] + (s.db[925][10] * ddt_scale));
        let eq1_e1023_d_b11: f64 = (s.db[927][11] + (s.db[925][11] * ddt_scale));
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_d_b9, eq1_e1023_d_b10, eq1_e1023_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1025;
        let eq1_node_derivatives: [f64; 18] = [eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17];
        let eq1_branch_derivatives: [f64; 12] = [eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11,) = {
    if s.b[3306] {
        let eq4_e1039: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[931]);
        let eq4_e1040: f64 = (s.v[932] + eq4_e1039);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + (s.dn[931][0] * ddt_scale));
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + (s.dn[931][1] * ddt_scale));
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + (s.dn[931][2] * ddt_scale));
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + (s.dn[931][3] * ddt_scale));
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + (s.dn[931][4] * ddt_scale));
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + (s.dn[931][5] * ddt_scale));
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + (s.dn[931][6] * ddt_scale));
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + (s.dn[931][7] * ddt_scale));
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + (s.dn[931][8] * ddt_scale));
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + (s.dn[931][9] * ddt_scale));
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + (s.dn[931][10] * ddt_scale));
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + (s.dn[931][11] * ddt_scale));
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + (s.dn[931][12] * ddt_scale));
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + (s.dn[931][13] * ddt_scale));
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + (s.dn[931][14] * ddt_scale));
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + (s.dn[931][15] * ddt_scale));
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + (s.dn[931][16] * ddt_scale));
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + (s.dn[931][17] * ddt_scale));
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + (s.db[931][0] * ddt_scale));
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + (s.db[931][1] * ddt_scale));
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + (s.db[931][2] * ddt_scale));
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + (s.db[931][3] * ddt_scale));
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + (s.db[931][4] * ddt_scale));
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + (s.db[931][5] * ddt_scale));
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + (s.db[931][6] * ddt_scale));
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + (s.db[931][7] * ddt_scale));
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + (s.db[931][8] * ddt_scale));
        let eq4_e1040_d_b9: f64 = (s.db[932][9] + (s.db[931][9] * ddt_scale));
        let eq4_e1040_d_b10: f64 = (s.db[932][10] + (s.db[931][10] * ddt_scale));
        let eq4_e1040_d_b11: f64 = (s.db[932][11] + (s.db[931][11] * ddt_scale));
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_d_b9, eq4_e1040_d_b10, eq4_e1040_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1042;
        let eq4_node_derivatives: [f64; 18] = [eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17];
        let eq4_branch_derivatives: [f64; 12] = [eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq6_e1051: f64 = (s.v[134] + s.v[400]);
        let eq6_e1051_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);
        let eq6_e1051_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);
        let eq6_e1051_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);
        let eq6_e1051_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);
        let eq6_e1051_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);
        let eq6_e1051_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);
        let eq6_e1051_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);
        let eq6_e1051_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);
        let eq6_e1051_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);
        let eq6_e1051_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);
        let eq6_e1051_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);
        let eq6_e1051_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);
        let eq6_e1051_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);
        let eq6_e1051_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);
        let eq6_e1051_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);
        let eq6_e1051_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);
        let eq6_e1051_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);
        let eq6_e1051_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);
        let eq6_e1051_d_b0: f64 = (s.db[134][0] + s.db[400][0]);
        let eq6_e1051_d_b1: f64 = (s.db[134][1] + s.db[400][1]);
        let eq6_e1051_d_b2: f64 = (s.db[134][2] + s.db[400][2]);
        let eq6_e1051_d_b3: f64 = (s.db[134][3] + s.db[400][3]);
        let eq6_e1051_d_b4: f64 = (s.db[134][4] + s.db[400][4]);
        let eq6_e1051_d_b5: f64 = (s.db[134][5] + s.db[400][5]);
        let eq6_e1051_d_b6: f64 = (s.db[134][6] + s.db[400][6]);
        let eq6_e1051_d_b7: f64 = (s.db[134][7] + s.db[400][7]);
        let eq6_e1051_d_b8: f64 = (s.db[134][8] + s.db[400][8]);
        let eq6_e1051_d_b9: f64 = (s.db[134][9] + s.db[400][9]);
        let eq6_e1051_d_b10: f64 = (s.db[134][10] + s.db[400][10]);
        let eq6_e1051_d_b11: f64 = (s.db[134][11] + s.db[400][11]);
        let eq6_e1053: f64 = (eq6_e1051 - s.v[738]);
        let eq6_e1053_d_n0: f64 = (eq6_e1051_d_n0 - s.dn[738][0]);
        let eq6_e1053_d_n1: f64 = (eq6_e1051_d_n1 - s.dn[738][1]);
        let eq6_e1053_d_n2: f64 = (eq6_e1051_d_n2 - s.dn[738][2]);
        let eq6_e1053_d_n3: f64 = (eq6_e1051_d_n3 - s.dn[738][3]);
        let eq6_e1053_d_n4: f64 = (eq6_e1051_d_n4 - s.dn[738][4]);
        let eq6_e1053_d_n5: f64 = (eq6_e1051_d_n5 - s.dn[738][5]);
        let eq6_e1053_d_n6: f64 = (eq6_e1051_d_n6 - s.dn[738][6]);
        let eq6_e1053_d_n7: f64 = (eq6_e1051_d_n7 - s.dn[738][7]);
        let eq6_e1053_d_n8: f64 = (eq6_e1051_d_n8 - s.dn[738][8]);
        let eq6_e1053_d_n9: f64 = (eq6_e1051_d_n9 - s.dn[738][9]);
        let eq6_e1053_d_n10: f64 = (eq6_e1051_d_n10 - s.dn[738][10]);
        let eq6_e1053_d_n11: f64 = (eq6_e1051_d_n11 - s.dn[738][11]);
        let eq6_e1053_d_n12: f64 = (eq6_e1051_d_n12 - s.dn[738][12]);
        let eq6_e1053_d_n13: f64 = (eq6_e1051_d_n13 - s.dn[738][13]);
        let eq6_e1053_d_n14: f64 = (eq6_e1051_d_n14 - s.dn[738][14]);
        let eq6_e1053_d_n15: f64 = (eq6_e1051_d_n15 - s.dn[738][15]);
        let eq6_e1053_d_n16: f64 = (eq6_e1051_d_n16 - s.dn[738][16]);
        let eq6_e1053_d_n17: f64 = (eq6_e1051_d_n17 - s.dn[738][17]);
        let eq6_e1053_d_b0: f64 = (eq6_e1051_d_b0 - s.db[738][0]);
        let eq6_e1053_d_b1: f64 = (eq6_e1051_d_b1 - s.db[738][1]);
        let eq6_e1053_d_b2: f64 = (eq6_e1051_d_b2 - s.db[738][2]);
        let eq6_e1053_d_b3: f64 = (eq6_e1051_d_b3 - s.db[738][3]);
        let eq6_e1053_d_b4: f64 = (eq6_e1051_d_b4 - s.db[738][4]);
        let eq6_e1053_d_b5: f64 = (eq6_e1051_d_b5 - s.db[738][5]);
        let eq6_e1053_d_b6: f64 = (eq6_e1051_d_b6 - s.db[738][6]);
        let eq6_e1053_d_b7: f64 = (eq6_e1051_d_b7 - s.db[738][7]);
        let eq6_e1053_d_b8: f64 = (eq6_e1051_d_b8 - s.db[738][8]);
        let eq6_e1053_d_b9: f64 = (eq6_e1051_d_b9 - s.db[738][9]);
        let eq6_e1053_d_b10: f64 = (eq6_e1051_d_b10 - s.db[738][10]);
        let eq6_e1053_d_b11: f64 = (eq6_e1051_d_b11 - s.db[738][11]);
        let eq6_e1054: f64 = (p.p87 * eq6_e1053);
        let eq6_e1054_d_n0: f64 = (p.p87 * eq6_e1053_d_n0);
        let eq6_e1054_d_n1: f64 = (p.p87 * eq6_e1053_d_n1);
        let eq6_e1054_d_n2: f64 = (p.p87 * eq6_e1053_d_n2);
        let eq6_e1054_d_n3: f64 = (p.p87 * eq6_e1053_d_n3);
        let eq6_e1054_d_n4: f64 = (p.p87 * eq6_e1053_d_n4);
        let eq6_e1054_d_n5: f64 = (p.p87 * eq6_e1053_d_n5);
        let eq6_e1054_d_n6: f64 = (p.p87 * eq6_e1053_d_n6);
        let eq6_e1054_d_n7: f64 = (p.p87 * eq6_e1053_d_n7);
        let eq6_e1054_d_n8: f64 = (p.p87 * eq6_e1053_d_n8);
        let eq6_e1054_d_n9: f64 = (p.p87 * eq6_e1053_d_n9);
        let eq6_e1054_d_n10: f64 = (p.p87 * eq6_e1053_d_n10);
        let eq6_e1054_d_n11: f64 = (p.p87 * eq6_e1053_d_n11);
        let eq6_e1054_d_n12: f64 = (p.p87 * eq6_e1053_d_n12);
        let eq6_e1054_d_n13: f64 = (p.p87 * eq6_e1053_d_n13);
        let eq6_e1054_d_n14: f64 = (p.p87 * eq6_e1053_d_n14);
        let eq6_e1054_d_n15: f64 = (p.p87 * eq6_e1053_d_n15);
        let eq6_e1054_d_n16: f64 = (p.p87 * eq6_e1053_d_n16);
        let eq6_e1054_d_n17: f64 = (p.p87 * eq6_e1053_d_n17);
        let eq6_e1054_d_b0: f64 = (p.p87 * eq6_e1053_d_b0);
        let eq6_e1054_d_b1: f64 = (p.p87 * eq6_e1053_d_b1);
        let eq6_e1054_d_b2: f64 = (p.p87 * eq6_e1053_d_b2);
        let eq6_e1054_d_b3: f64 = (p.p87 * eq6_e1053_d_b3);
        let eq6_e1054_d_b4: f64 = (p.p87 * eq6_e1053_d_b4);
        let eq6_e1054_d_b5: f64 = (p.p87 * eq6_e1053_d_b5);
        let eq6_e1054_d_b6: f64 = (p.p87 * eq6_e1053_d_b6);
        let eq6_e1054_d_b7: f64 = (p.p87 * eq6_e1053_d_b7);
        let eq6_e1054_d_b8: f64 = (p.p87 * eq6_e1053_d_b8);
        let eq6_e1054_d_b9: f64 = (p.p87 * eq6_e1053_d_b9);
        let eq6_e1054_d_b10: f64 = (p.p87 * eq6_e1053_d_b10);
        let eq6_e1054_d_b11: f64 = (p.p87 * eq6_e1053_d_b11);
        let eq6_value: f64 = eq6_e1054;
        let eq6_node_derivatives: [f64; 18] = [eq6_e1054_d_n0, eq6_e1054_d_n1, eq6_e1054_d_n2, eq6_e1054_d_n3, eq6_e1054_d_n4, eq6_e1054_d_n5, eq6_e1054_d_n6, eq6_e1054_d_n7, eq6_e1054_d_n8, eq6_e1054_d_n9, eq6_e1054_d_n10, eq6_e1054_d_n11, eq6_e1054_d_n12, eq6_e1054_d_n13, eq6_e1054_d_n14, eq6_e1054_d_n15, eq6_e1054_d_n16, eq6_e1054_d_n17];
        let eq6_branch_derivatives: [f64; 12] = [eq6_e1054_d_b0, eq6_e1054_d_b1, eq6_e1054_d_b2, eq6_e1054_d_b3, eq6_e1054_d_b4, eq6_e1054_d_b5, eq6_e1054_d_b6, eq6_e1054_d_b7, eq6_e1054_d_b8, eq6_e1054_d_b9, eq6_e1054_d_b10, eq6_e1054_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e1058: f64 = (s.v[424] - s.v[425]);
        let eq7_e1058_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);
        let eq7_e1058_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);
        let eq7_e1058_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);
        let eq7_e1058_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);
        let eq7_e1058_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);
        let eq7_e1058_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);
        let eq7_e1058_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);
        let eq7_e1058_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);
        let eq7_e1058_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);
        let eq7_e1058_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);
        let eq7_e1058_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);
        let eq7_e1058_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);
        let eq7_e1058_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);
        let eq7_e1058_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);
        let eq7_e1058_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);
        let eq7_e1058_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);
        let eq7_e1058_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);
        let eq7_e1058_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);
        let eq7_e1058_d_b0: f64 = (s.db[424][0] - s.db[425][0]);
        let eq7_e1058_d_b1: f64 = (s.db[424][1] - s.db[425][1]);
        let eq7_e1058_d_b2: f64 = (s.db[424][2] - s.db[425][2]);
        let eq7_e1058_d_b3: f64 = (s.db[424][3] - s.db[425][3]);
        let eq7_e1058_d_b4: f64 = (s.db[424][4] - s.db[425][4]);
        let eq7_e1058_d_b5: f64 = (s.db[424][5] - s.db[425][5]);
        let eq7_e1058_d_b6: f64 = (s.db[424][6] - s.db[425][6]);
        let eq7_e1058_d_b7: f64 = (s.db[424][7] - s.db[425][7]);
        let eq7_e1058_d_b8: f64 = (s.db[424][8] - s.db[425][8]);
        let eq7_e1058_d_b9: f64 = (s.db[424][9] - s.db[425][9]);
        let eq7_e1058_d_b10: f64 = (s.db[424][10] - s.db[425][10]);
        let eq7_e1058_d_b11: f64 = (s.db[424][11] - s.db[425][11]);
        let eq7_e1059: f64 = (p.p87 * eq7_e1058);
        let eq7_e1059_d_n0: f64 = (p.p87 * eq7_e1058_d_n0);
        let eq7_e1059_d_n1: f64 = (p.p87 * eq7_e1058_d_n1);
        let eq7_e1059_d_n2: f64 = (p.p87 * eq7_e1058_d_n2);
        let eq7_e1059_d_n3: f64 = (p.p87 * eq7_e1058_d_n3);
        let eq7_e1059_d_n4: f64 = (p.p87 * eq7_e1058_d_n4);
        let eq7_e1059_d_n5: f64 = (p.p87 * eq7_e1058_d_n5);
        let eq7_e1059_d_n6: f64 = (p.p87 * eq7_e1058_d_n6);
        let eq7_e1059_d_n7: f64 = (p.p87 * eq7_e1058_d_n7);
        let eq7_e1059_d_n8: f64 = (p.p87 * eq7_e1058_d_n8);
        let eq7_e1059_d_n9: f64 = (p.p87 * eq7_e1058_d_n9);
        let eq7_e1059_d_n10: f64 = (p.p87 * eq7_e1058_d_n10);
        let eq7_e1059_d_n11: f64 = (p.p87 * eq7_e1058_d_n11);
        let eq7_e1059_d_n12: f64 = (p.p87 * eq7_e1058_d_n12);
        let eq7_e1059_d_n13: f64 = (p.p87 * eq7_e1058_d_n13);
        let eq7_e1059_d_n14: f64 = (p.p87 * eq7_e1058_d_n14);
        let eq7_e1059_d_n15: f64 = (p.p87 * eq7_e1058_d_n15);
        let eq7_e1059_d_n16: f64 = (p.p87 * eq7_e1058_d_n16);
        let eq7_e1059_d_n17: f64 = (p.p87 * eq7_e1058_d_n17);
        let eq7_e1059_d_b0: f64 = (p.p87 * eq7_e1058_d_b0);
        let eq7_e1059_d_b1: f64 = (p.p87 * eq7_e1058_d_b1);
        let eq7_e1059_d_b2: f64 = (p.p87 * eq7_e1058_d_b2);
        let eq7_e1059_d_b3: f64 = (p.p87 * eq7_e1058_d_b3);
        let eq7_e1059_d_b4: f64 = (p.p87 * eq7_e1058_d_b4);
        let eq7_e1059_d_b5: f64 = (p.p87 * eq7_e1058_d_b5);
        let eq7_e1059_d_b6: f64 = (p.p87 * eq7_e1058_d_b6);
        let eq7_e1059_d_b7: f64 = (p.p87 * eq7_e1058_d_b7);
        let eq7_e1059_d_b8: f64 = (p.p87 * eq7_e1058_d_b8);
        let eq7_e1059_d_b9: f64 = (p.p87 * eq7_e1058_d_b9);
        let eq7_e1059_d_b10: f64 = (p.p87 * eq7_e1058_d_b10);
        let eq7_e1059_d_b11: f64 = (p.p87 * eq7_e1058_d_b11);
        let eq7_value: f64 = eq7_e1059;
        let eq7_node_derivatives: [f64; 18] = [eq7_e1059_d_n0, eq7_e1059_d_n1, eq7_e1059_d_n2, eq7_e1059_d_n3, eq7_e1059_d_n4, eq7_e1059_d_n5, eq7_e1059_d_n6, eq7_e1059_d_n7, eq7_e1059_d_n8, eq7_e1059_d_n9, eq7_e1059_d_n10, eq7_e1059_d_n11, eq7_e1059_d_n12, eq7_e1059_d_n13, eq7_e1059_d_n14, eq7_e1059_d_n15, eq7_e1059_d_n16, eq7_e1059_d_n17];
        let eq7_branch_derivatives: [f64; 12] = [eq7_e1059_d_b0, eq7_e1059_d_b1, eq7_e1059_d_b2, eq7_e1059_d_b3, eq7_e1059_d_b4, eq7_e1059_d_b5, eq7_e1059_d_b6, eq7_e1059_d_b7, eq7_e1059_d_b8, eq7_e1059_d_b9, eq7_e1059_d_b10, eq7_e1059_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1063: f64 = (s.v[203] + s.v[280]);
        let eq8_e1063_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);
        let eq8_e1063_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);
        let eq8_e1063_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);
        let eq8_e1063_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);
        let eq8_e1063_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);
        let eq8_e1063_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);
        let eq8_e1063_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);
        let eq8_e1063_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);
        let eq8_e1063_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);
        let eq8_e1063_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);
        let eq8_e1063_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);
        let eq8_e1063_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);
        let eq8_e1063_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);
        let eq8_e1063_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);
        let eq8_e1063_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);
        let eq8_e1063_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);
        let eq8_e1063_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);
        let eq8_e1063_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);
        let eq8_e1063_d_b0: f64 = (s.db[203][0] + s.db[280][0]);
        let eq8_e1063_d_b1: f64 = (s.db[203][1] + s.db[280][1]);
        let eq8_e1063_d_b2: f64 = (s.db[203][2] + s.db[280][2]);
        let eq8_e1063_d_b3: f64 = (s.db[203][3] + s.db[280][3]);
        let eq8_e1063_d_b4: f64 = (s.db[203][4] + s.db[280][4]);
        let eq8_e1063_d_b5: f64 = (s.db[203][5] + s.db[280][5]);
        let eq8_e1063_d_b6: f64 = (s.db[203][6] + s.db[280][6]);
        let eq8_e1063_d_b7: f64 = (s.db[203][7] + s.db[280][7]);
        let eq8_e1063_d_b8: f64 = (s.db[203][8] + s.db[280][8]);
        let eq8_e1063_d_b9: f64 = (s.db[203][9] + s.db[280][9]);
        let eq8_e1063_d_b10: f64 = (s.db[203][10] + s.db[280][10]);
        let eq8_e1063_d_b11: f64 = (s.db[203][11] + s.db[280][11]);
        let eq8_e1065: f64 = (eq8_e1063 + s.v[431]);
        let eq8_e1065_d_n0: f64 = (eq8_e1063_d_n0 + s.dn[431][0]);
        let eq8_e1065_d_n1: f64 = (eq8_e1063_d_n1 + s.dn[431][1]);
        let eq8_e1065_d_n2: f64 = (eq8_e1063_d_n2 + s.dn[431][2]);
        let eq8_e1065_d_n3: f64 = (eq8_e1063_d_n3 + s.dn[431][3]);
        let eq8_e1065_d_n4: f64 = (eq8_e1063_d_n4 + s.dn[431][4]);
        let eq8_e1065_d_n5: f64 = (eq8_e1063_d_n5 + s.dn[431][5]);
        let eq8_e1065_d_n6: f64 = (eq8_e1063_d_n6 + s.dn[431][6]);
        let eq8_e1065_d_n7: f64 = (eq8_e1063_d_n7 + s.dn[431][7]);
        let eq8_e1065_d_n8: f64 = (eq8_e1063_d_n8 + s.dn[431][8]);
        let eq8_e1065_d_n9: f64 = (eq8_e1063_d_n9 + s.dn[431][9]);
        let eq8_e1065_d_n10: f64 = (eq8_e1063_d_n10 + s.dn[431][10]);
        let eq8_e1065_d_n11: f64 = (eq8_e1063_d_n11 + s.dn[431][11]);
        let eq8_e1065_d_n12: f64 = (eq8_e1063_d_n12 + s.dn[431][12]);
        let eq8_e1065_d_n13: f64 = (eq8_e1063_d_n13 + s.dn[431][13]);
        let eq8_e1065_d_n14: f64 = (eq8_e1063_d_n14 + s.dn[431][14]);
        let eq8_e1065_d_n15: f64 = (eq8_e1063_d_n15 + s.dn[431][15]);
        let eq8_e1065_d_n16: f64 = (eq8_e1063_d_n16 + s.dn[431][16]);
        let eq8_e1065_d_n17: f64 = (eq8_e1063_d_n17 + s.dn[431][17]);
        let eq8_e1065_d_b0: f64 = (eq8_e1063_d_b0 + s.db[431][0]);
        let eq8_e1065_d_b1: f64 = (eq8_e1063_d_b1 + s.db[431][1]);
        let eq8_e1065_d_b2: f64 = (eq8_e1063_d_b2 + s.db[431][2]);
        let eq8_e1065_d_b3: f64 = (eq8_e1063_d_b3 + s.db[431][3]);
        let eq8_e1065_d_b4: f64 = (eq8_e1063_d_b4 + s.db[431][4]);
        let eq8_e1065_d_b5: f64 = (eq8_e1063_d_b5 + s.db[431][5]);
        let eq8_e1065_d_b6: f64 = (eq8_e1063_d_b6 + s.db[431][6]);
        let eq8_e1065_d_b7: f64 = (eq8_e1063_d_b7 + s.db[431][7]);
        let eq8_e1065_d_b8: f64 = (eq8_e1063_d_b8 + s.db[431][8]);
        let eq8_e1065_d_b9: f64 = (eq8_e1063_d_b9 + s.db[431][9]);
        let eq8_e1065_d_b10: f64 = (eq8_e1063_d_b10 + s.db[431][10]);
        let eq8_e1065_d_b11: f64 = (eq8_e1063_d_b11 + s.db[431][11]);
        let eq8_e1066: f64 = (p.p87 * eq8_e1065);
        let eq8_e1066_d_n0: f64 = (p.p87 * eq8_e1065_d_n0);
        let eq8_e1066_d_n1: f64 = (p.p87 * eq8_e1065_d_n1);
        let eq8_e1066_d_n2: f64 = (p.p87 * eq8_e1065_d_n2);
        let eq8_e1066_d_n3: f64 = (p.p87 * eq8_e1065_d_n3);
        let eq8_e1066_d_n4: f64 = (p.p87 * eq8_e1065_d_n4);
        let eq8_e1066_d_n5: f64 = (p.p87 * eq8_e1065_d_n5);
        let eq8_e1066_d_n6: f64 = (p.p87 * eq8_e1065_d_n6);
        let eq8_e1066_d_n7: f64 = (p.p87 * eq8_e1065_d_n7);
        let eq8_e1066_d_n8: f64 = (p.p87 * eq8_e1065_d_n8);
        let eq8_e1066_d_n9: f64 = (p.p87 * eq8_e1065_d_n9);
        let eq8_e1066_d_n10: f64 = (p.p87 * eq8_e1065_d_n10);
        let eq8_e1066_d_n11: f64 = (p.p87 * eq8_e1065_d_n11);
        let eq8_e1066_d_n12: f64 = (p.p87 * eq8_e1065_d_n12);
        let eq8_e1066_d_n13: f64 = (p.p87 * eq8_e1065_d_n13);
        let eq8_e1066_d_n14: f64 = (p.p87 * eq8_e1065_d_n14);
        let eq8_e1066_d_n15: f64 = (p.p87 * eq8_e1065_d_n15);
        let eq8_e1066_d_n16: f64 = (p.p87 * eq8_e1065_d_n16);
        let eq8_e1066_d_n17: f64 = (p.p87 * eq8_e1065_d_n17);
        let eq8_e1066_d_b0: f64 = (p.p87 * eq8_e1065_d_b0);
        let eq8_e1066_d_b1: f64 = (p.p87 * eq8_e1065_d_b1);
        let eq8_e1066_d_b2: f64 = (p.p87 * eq8_e1065_d_b2);
        let eq8_e1066_d_b3: f64 = (p.p87 * eq8_e1065_d_b3);
        let eq8_e1066_d_b4: f64 = (p.p87 * eq8_e1065_d_b4);
        let eq8_e1066_d_b5: f64 = (p.p87 * eq8_e1065_d_b5);
        let eq8_e1066_d_b6: f64 = (p.p87 * eq8_e1065_d_b6);
        let eq8_e1066_d_b7: f64 = (p.p87 * eq8_e1065_d_b7);
        let eq8_e1066_d_b8: f64 = (p.p87 * eq8_e1065_d_b8);
        let eq8_e1066_d_b9: f64 = (p.p87 * eq8_e1065_d_b9);
        let eq8_e1066_d_b10: f64 = (p.p87 * eq8_e1065_d_b10);
        let eq8_e1066_d_b11: f64 = (p.p87 * eq8_e1065_d_b11);
        let eq8_value: f64 = eq8_e1066;
        let eq8_node_derivatives: [f64; 18] = [eq8_e1066_d_n0, eq8_e1066_d_n1, eq8_e1066_d_n2, eq8_e1066_d_n3, eq8_e1066_d_n4, eq8_e1066_d_n5, eq8_e1066_d_n6, eq8_e1066_d_n7, eq8_e1066_d_n8, eq8_e1066_d_n9, eq8_e1066_d_n10, eq8_e1066_d_n11, eq8_e1066_d_n12, eq8_e1066_d_n13, eq8_e1066_d_n14, eq8_e1066_d_n15, eq8_e1066_d_n16, eq8_e1066_d_n17];
        let eq8_branch_derivatives: [f64; 12] = [eq8_e1066_d_b0, eq8_e1066_d_b1, eq8_e1066_d_b2, eq8_e1066_d_b3, eq8_e1066_d_b4, eq8_e1066_d_b5, eq8_e1066_d_b6, eq8_e1066_d_b7, eq8_e1066_d_b8, eq8_e1066_d_b9, eq8_e1066_d_b10, eq8_e1066_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq9_e1070: f64 = (s.v[204] + s.v[736]);
        let eq9_e1070_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);
        let eq9_e1070_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);
        let eq9_e1070_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);
        let eq9_e1070_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);
        let eq9_e1070_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);
        let eq9_e1070_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);
        let eq9_e1070_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);
        let eq9_e1070_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);
        let eq9_e1070_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);
        let eq9_e1070_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);
        let eq9_e1070_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);
        let eq9_e1070_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);
        let eq9_e1070_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);
        let eq9_e1070_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);
        let eq9_e1070_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);
        let eq9_e1070_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);
        let eq9_e1070_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);
        let eq9_e1070_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);
        let eq9_e1070_d_b0: f64 = (s.db[204][0] + s.db[736][0]);
        let eq9_e1070_d_b1: f64 = (s.db[204][1] + s.db[736][1]);
        let eq9_e1070_d_b2: f64 = (s.db[204][2] + s.db[736][2]);
        let eq9_e1070_d_b3: f64 = (s.db[204][3] + s.db[736][3]);
        let eq9_e1070_d_b4: f64 = (s.db[204][4] + s.db[736][4]);
        let eq9_e1070_d_b5: f64 = (s.db[204][5] + s.db[736][5]);
        let eq9_e1070_d_b6: f64 = (s.db[204][6] + s.db[736][6]);
        let eq9_e1070_d_b7: f64 = (s.db[204][7] + s.db[736][7]);
        let eq9_e1070_d_b8: f64 = (s.db[204][8] + s.db[736][8]);
        let eq9_e1070_d_b9: f64 = (s.db[204][9] + s.db[736][9]);
        let eq9_e1070_d_b10: f64 = (s.db[204][10] + s.db[736][10]);
        let eq9_e1070_d_b11: f64 = (s.db[204][11] + s.db[736][11]);
        let eq9_e1072: f64 = (eq9_e1070 + s.v[432]);
        let eq9_e1072_d_n0: f64 = (eq9_e1070_d_n0 + s.dn[432][0]);
        let eq9_e1072_d_n1: f64 = (eq9_e1070_d_n1 + s.dn[432][1]);
        let eq9_e1072_d_n2: f64 = (eq9_e1070_d_n2 + s.dn[432][2]);
        let eq9_e1072_d_n3: f64 = (eq9_e1070_d_n3 + s.dn[432][3]);
        let eq9_e1072_d_n4: f64 = (eq9_e1070_d_n4 + s.dn[432][4]);
        let eq9_e1072_d_n5: f64 = (eq9_e1070_d_n5 + s.dn[432][5]);
        let eq9_e1072_d_n6: f64 = (eq9_e1070_d_n6 + s.dn[432][6]);
        let eq9_e1072_d_n7: f64 = (eq9_e1070_d_n7 + s.dn[432][7]);
        let eq9_e1072_d_n8: f64 = (eq9_e1070_d_n8 + s.dn[432][8]);
        let eq9_e1072_d_n9: f64 = (eq9_e1070_d_n9 + s.dn[432][9]);
        let eq9_e1072_d_n10: f64 = (eq9_e1070_d_n10 + s.dn[432][10]);
        let eq9_e1072_d_n11: f64 = (eq9_e1070_d_n11 + s.dn[432][11]);
        let eq9_e1072_d_n12: f64 = (eq9_e1070_d_n12 + s.dn[432][12]);
        let eq9_e1072_d_n13: f64 = (eq9_e1070_d_n13 + s.dn[432][13]);
        let eq9_e1072_d_n14: f64 = (eq9_e1070_d_n14 + s.dn[432][14]);
        let eq9_e1072_d_n15: f64 = (eq9_e1070_d_n15 + s.dn[432][15]);
        let eq9_e1072_d_n16: f64 = (eq9_e1070_d_n16 + s.dn[432][16]);
        let eq9_e1072_d_n17: f64 = (eq9_e1070_d_n17 + s.dn[432][17]);
        let eq9_e1072_d_b0: f64 = (eq9_e1070_d_b0 + s.db[432][0]);
        let eq9_e1072_d_b1: f64 = (eq9_e1070_d_b1 + s.db[432][1]);
        let eq9_e1072_d_b2: f64 = (eq9_e1070_d_b2 + s.db[432][2]);
        let eq9_e1072_d_b3: f64 = (eq9_e1070_d_b3 + s.db[432][3]);
        let eq9_e1072_d_b4: f64 = (eq9_e1070_d_b4 + s.db[432][4]);
        let eq9_e1072_d_b5: f64 = (eq9_e1070_d_b5 + s.db[432][5]);
        let eq9_e1072_d_b6: f64 = (eq9_e1070_d_b6 + s.db[432][6]);
        let eq9_e1072_d_b7: f64 = (eq9_e1070_d_b7 + s.db[432][7]);
        let eq9_e1072_d_b8: f64 = (eq9_e1070_d_b8 + s.db[432][8]);
        let eq9_e1072_d_b9: f64 = (eq9_e1070_d_b9 + s.db[432][9]);
        let eq9_e1072_d_b10: f64 = (eq9_e1070_d_b10 + s.db[432][10]);
        let eq9_e1072_d_b11: f64 = (eq9_e1070_d_b11 + s.db[432][11]);
        let eq9_e1073: f64 = (p.p87 * eq9_e1072);
        let eq9_e1073_d_n0: f64 = (p.p87 * eq9_e1072_d_n0);
        let eq9_e1073_d_n1: f64 = (p.p87 * eq9_e1072_d_n1);
        let eq9_e1073_d_n2: f64 = (p.p87 * eq9_e1072_d_n2);
        let eq9_e1073_d_n3: f64 = (p.p87 * eq9_e1072_d_n3);
        let eq9_e1073_d_n4: f64 = (p.p87 * eq9_e1072_d_n4);
        let eq9_e1073_d_n5: f64 = (p.p87 * eq9_e1072_d_n5);
        let eq9_e1073_d_n6: f64 = (p.p87 * eq9_e1072_d_n6);
        let eq9_e1073_d_n7: f64 = (p.p87 * eq9_e1072_d_n7);
        let eq9_e1073_d_n8: f64 = (p.p87 * eq9_e1072_d_n8);
        let eq9_e1073_d_n9: f64 = (p.p87 * eq9_e1072_d_n9);
        let eq9_e1073_d_n10: f64 = (p.p87 * eq9_e1072_d_n10);
        let eq9_e1073_d_n11: f64 = (p.p87 * eq9_e1072_d_n11);
        let eq9_e1073_d_n12: f64 = (p.p87 * eq9_e1072_d_n12);
        let eq9_e1073_d_n13: f64 = (p.p87 * eq9_e1072_d_n13);
        let eq9_e1073_d_n14: f64 = (p.p87 * eq9_e1072_d_n14);
        let eq9_e1073_d_n15: f64 = (p.p87 * eq9_e1072_d_n15);
        let eq9_e1073_d_n16: f64 = (p.p87 * eq9_e1072_d_n16);
        let eq9_e1073_d_n17: f64 = (p.p87 * eq9_e1072_d_n17);
        let eq9_e1073_d_b0: f64 = (p.p87 * eq9_e1072_d_b0);
        let eq9_e1073_d_b1: f64 = (p.p87 * eq9_e1072_d_b1);
        let eq9_e1073_d_b2: f64 = (p.p87 * eq9_e1072_d_b2);
        let eq9_e1073_d_b3: f64 = (p.p87 * eq9_e1072_d_b3);
        let eq9_e1073_d_b4: f64 = (p.p87 * eq9_e1072_d_b4);
        let eq9_e1073_d_b5: f64 = (p.p87 * eq9_e1072_d_b5);
        let eq9_e1073_d_b6: f64 = (p.p87 * eq9_e1072_d_b6);
        let eq9_e1073_d_b7: f64 = (p.p87 * eq9_e1072_d_b7);
        let eq9_e1073_d_b8: f64 = (p.p87 * eq9_e1072_d_b8);
        let eq9_e1073_d_b9: f64 = (p.p87 * eq9_e1072_d_b9);
        let eq9_e1073_d_b10: f64 = (p.p87 * eq9_e1072_d_b10);
        let eq9_e1073_d_b11: f64 = (p.p87 * eq9_e1072_d_b11);
        let eq9_value: f64 = eq9_e1073;
        let eq9_node_derivatives: [f64; 18] = [eq9_e1073_d_n0, eq9_e1073_d_n1, eq9_e1073_d_n2, eq9_e1073_d_n3, eq9_e1073_d_n4, eq9_e1073_d_n5, eq9_e1073_d_n6, eq9_e1073_d_n7, eq9_e1073_d_n8, eq9_e1073_d_n9, eq9_e1073_d_n10, eq9_e1073_d_n11, eq9_e1073_d_n12, eq9_e1073_d_n13, eq9_e1073_d_n14, eq9_e1073_d_n15, eq9_e1073_d_n16, eq9_e1073_d_n17];
        let eq9_branch_derivatives: [f64; 12] = [eq9_e1073_d_b0, eq9_e1073_d_b1, eq9_e1073_d_b2, eq9_e1073_d_b3, eq9_e1073_d_b4, eq9_e1073_d_b5, eq9_e1073_d_b6, eq9_e1073_d_b7, eq9_e1073_d_b8, eq9_e1073_d_b9, eq9_e1073_d_b10, eq9_e1073_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1076: f64 = (p.p87 * s.v[281]);
        let eq10_e1076_d_n0: f64 = (p.p87 * s.dn[281][0]);
        let eq10_e1076_d_n1: f64 = (p.p87 * s.dn[281][1]);
        let eq10_e1076_d_n2: f64 = (p.p87 * s.dn[281][2]);
        let eq10_e1076_d_n3: f64 = (p.p87 * s.dn[281][3]);
        let eq10_e1076_d_n4: f64 = (p.p87 * s.dn[281][4]);
        let eq10_e1076_d_n5: f64 = (p.p87 * s.dn[281][5]);
        let eq10_e1076_d_n6: f64 = (p.p87 * s.dn[281][6]);
        let eq10_e1076_d_n7: f64 = (p.p87 * s.dn[281][7]);
        let eq10_e1076_d_n8: f64 = (p.p87 * s.dn[281][8]);
        let eq10_e1076_d_n9: f64 = (p.p87 * s.dn[281][9]);
        let eq10_e1076_d_n10: f64 = (p.p87 * s.dn[281][10]);
        let eq10_e1076_d_n11: f64 = (p.p87 * s.dn[281][11]);
        let eq10_e1076_d_n12: f64 = (p.p87 * s.dn[281][12]);
        let eq10_e1076_d_n13: f64 = (p.p87 * s.dn[281][13]);
        let eq10_e1076_d_n14: f64 = (p.p87 * s.dn[281][14]);
        let eq10_e1076_d_n15: f64 = (p.p87 * s.dn[281][15]);
        let eq10_e1076_d_n16: f64 = (p.p87 * s.dn[281][16]);
        let eq10_e1076_d_n17: f64 = (p.p87 * s.dn[281][17]);
        let eq10_e1076_d_b0: f64 = (p.p87 * s.db[281][0]);
        let eq10_e1076_d_b1: f64 = (p.p87 * s.db[281][1]);
        let eq10_e1076_d_b2: f64 = (p.p87 * s.db[281][2]);
        let eq10_e1076_d_b3: f64 = (p.p87 * s.db[281][3]);
        let eq10_e1076_d_b4: f64 = (p.p87 * s.db[281][4]);
        let eq10_e1076_d_b5: f64 = (p.p87 * s.db[281][5]);
        let eq10_e1076_d_b6: f64 = (p.p87 * s.db[281][6]);
        let eq10_e1076_d_b7: f64 = (p.p87 * s.db[281][7]);
        let eq10_e1076_d_b8: f64 = (p.p87 * s.db[281][8]);
        let eq10_e1076_d_b9: f64 = (p.p87 * s.db[281][9]);
        let eq10_e1076_d_b10: f64 = (p.p87 * s.db[281][10]);
        let eq10_e1076_d_b11: f64 = (p.p87 * s.db[281][11]);
        let eq10_value: f64 = eq10_e1076;
        let eq10_node_derivatives: [f64; 18] = [eq10_e1076_d_n0, eq10_e1076_d_n1, eq10_e1076_d_n2, eq10_e1076_d_n3, eq10_e1076_d_n4, eq10_e1076_d_n5, eq10_e1076_d_n6, eq10_e1076_d_n7, eq10_e1076_d_n8, eq10_e1076_d_n9, eq10_e1076_d_n10, eq10_e1076_d_n11, eq10_e1076_d_n12, eq10_e1076_d_n13, eq10_e1076_d_n14, eq10_e1076_d_n15, eq10_e1076_d_n16, eq10_e1076_d_n17];
        let eq10_branch_derivatives: [f64; 12] = [eq10_e1076_d_b0, eq10_e1076_d_b1, eq10_e1076_d_b2, eq10_e1076_d_b3, eq10_e1076_d_b4, eq10_e1076_d_b5, eq10_e1076_d_b6, eq10_e1076_d_b7, eq10_e1076_d_b8, eq10_e1076_d_b9, eq10_e1076_d_b10, eq10_e1076_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1079: f64 = (p.p87 * s.v[737]);
        let eq11_e1079_d_n0: f64 = (p.p87 * s.dn[737][0]);
        let eq11_e1079_d_n1: f64 = (p.p87 * s.dn[737][1]);
        let eq11_e1079_d_n2: f64 = (p.p87 * s.dn[737][2]);
        let eq11_e1079_d_n3: f64 = (p.p87 * s.dn[737][3]);
        let eq11_e1079_d_n4: f64 = (p.p87 * s.dn[737][4]);
        let eq11_e1079_d_n5: f64 = (p.p87 * s.dn[737][5]);
        let eq11_e1079_d_n6: f64 = (p.p87 * s.dn[737][6]);
        let eq11_e1079_d_n7: f64 = (p.p87 * s.dn[737][7]);
        let eq11_e1079_d_n8: f64 = (p.p87 * s.dn[737][8]);
        let eq11_e1079_d_n9: f64 = (p.p87 * s.dn[737][9]);
        let eq11_e1079_d_n10: f64 = (p.p87 * s.dn[737][10]);
        let eq11_e1079_d_n11: f64 = (p.p87 * s.dn[737][11]);
        let eq11_e1079_d_n12: f64 = (p.p87 * s.dn[737][12]);
        let eq11_e1079_d_n13: f64 = (p.p87 * s.dn[737][13]);
        let eq11_e1079_d_n14: f64 = (p.p87 * s.dn[737][14]);
        let eq11_e1079_d_n15: f64 = (p.p87 * s.dn[737][15]);
        let eq11_e1079_d_n16: f64 = (p.p87 * s.dn[737][16]);
        let eq11_e1079_d_n17: f64 = (p.p87 * s.dn[737][17]);
        let eq11_e1079_d_b0: f64 = (p.p87 * s.db[737][0]);
        let eq11_e1079_d_b1: f64 = (p.p87 * s.db[737][1]);
        let eq11_e1079_d_b2: f64 = (p.p87 * s.db[737][2]);
        let eq11_e1079_d_b3: f64 = (p.p87 * s.db[737][3]);
        let eq11_e1079_d_b4: f64 = (p.p87 * s.db[737][4]);
        let eq11_e1079_d_b5: f64 = (p.p87 * s.db[737][5]);
        let eq11_e1079_d_b6: f64 = (p.p87 * s.db[737][6]);
        let eq11_e1079_d_b7: f64 = (p.p87 * s.db[737][7]);
        let eq11_e1079_d_b8: f64 = (p.p87 * s.db[737][8]);
        let eq11_e1079_d_b9: f64 = (p.p87 * s.db[737][9]);
        let eq11_e1079_d_b10: f64 = (p.p87 * s.db[737][10]);
        let eq11_e1079_d_b11: f64 = (p.p87 * s.db[737][11]);
        let eq11_value: f64 = eq11_e1079;
        let eq11_node_derivatives: [f64; 18] = [eq11_e1079_d_n0, eq11_e1079_d_n1, eq11_e1079_d_n2, eq11_e1079_d_n3, eq11_e1079_d_n4, eq11_e1079_d_n5, eq11_e1079_d_n6, eq11_e1079_d_n7, eq11_e1079_d_n8, eq11_e1079_d_n9, eq11_e1079_d_n10, eq11_e1079_d_n11, eq11_e1079_d_n12, eq11_e1079_d_n13, eq11_e1079_d_n14, eq11_e1079_d_n15, eq11_e1079_d_n16, eq11_e1079_d_n17];
        let eq11_branch_derivatives: [f64; 12] = [eq11_e1079_d_b0, eq11_e1079_d_b1, eq11_e1079_d_b2, eq11_e1079_d_b3, eq11_e1079_d_b4, eq11_e1079_d_b5, eq11_e1079_d_b6, eq11_e1079_d_b7, eq11_e1079_d_b8, eq11_e1079_d_b9, eq11_e1079_d_b10, eq11_e1079_d_b11];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1082: f64 = (p.p87 * s.v[862]);
        let eq12_e1082_d_n0: f64 = (p.p87 * s.dn[862][0]);
        let eq12_e1082_d_n1: f64 = (p.p87 * s.dn[862][1]);
        let eq12_e1082_d_n2: f64 = (p.p87 * s.dn[862][2]);
        let eq12_e1082_d_n3: f64 = (p.p87 * s.dn[862][3]);
        let eq12_e1082_d_n4: f64 = (p.p87 * s.dn[862][4]);
        let eq12_e1082_d_n5: f64 = (p.p87 * s.dn[862][5]);
        let eq12_e1082_d_n6: f64 = (p.p87 * s.dn[862][6]);
        let eq12_e1082_d_n7: f64 = (p.p87 * s.dn[862][7]);
        let eq12_e1082_d_n8: f64 = (p.p87 * s.dn[862][8]);
        let eq12_e1082_d_n9: f64 = (p.p87 * s.dn[862][9]);
        let eq12_e1082_d_n10: f64 = (p.p87 * s.dn[862][10]);
        let eq12_e1082_d_n11: f64 = (p.p87 * s.dn[862][11]);
        let eq12_e1082_d_n12: f64 = (p.p87 * s.dn[862][12]);
        let eq12_e1082_d_n13: f64 = (p.p87 * s.dn[862][13]);
        let eq12_e1082_d_n14: f64 = (p.p87 * s.dn[862][14]);
        let eq12_e1082_d_n15: f64 = (p.p87 * s.dn[862][15]);
        let eq12_e1082_d_n16: f64 = (p.p87 * s.dn[862][16]);
        let eq12_e1082_d_n17: f64 = (p.p87 * s.dn[862][17]);
        let eq12_e1082_d_b0: f64 = (p.p87 * s.db[862][0]);
        let eq12_e1082_d_b1: f64 = (p.p87 * s.db[862][1]);
        let eq12_e1082_d_b2: f64 = (p.p87 * s.db[862][2]);
        let eq12_e1082_d_b3: f64 = (p.p87 * s.db[862][3]);
        let eq12_e1082_d_b4: f64 = (p.p87 * s.db[862][4]);
        let eq12_e1082_d_b5: f64 = (p.p87 * s.db[862][5]);
        let eq12_e1082_d_b6: f64 = (p.p87 * s.db[862][6]);
        let eq12_e1082_d_b7: f64 = (p.p87 * s.db[862][7]);
        let eq12_e1082_d_b8: f64 = (p.p87 * s.db[862][8]);
        let eq12_e1082_d_b9: f64 = (p.p87 * s.db[862][9]);
        let eq12_e1082_d_b10: f64 = (p.p87 * s.db[862][10]);
        let eq12_e1082_d_b11: f64 = (p.p87 * s.db[862][11]);
        let eq12_value: f64 = eq12_e1082;
        let eq12_node_derivatives: [f64; 18] = [eq12_e1082_d_n0, eq12_e1082_d_n1, eq12_e1082_d_n2, eq12_e1082_d_n3, eq12_e1082_d_n4, eq12_e1082_d_n5, eq12_e1082_d_n6, eq12_e1082_d_n7, eq12_e1082_d_n8, eq12_e1082_d_n9, eq12_e1082_d_n10, eq12_e1082_d_n11, eq12_e1082_d_n12, eq12_e1082_d_n13, eq12_e1082_d_n14, eq12_e1082_d_n15, eq12_e1082_d_n16, eq12_e1082_d_n17];
        let eq12_branch_derivatives: [f64; 12] = [eq12_e1082_d_b0, eq12_e1082_d_b1, eq12_e1082_d_b2, eq12_e1082_d_b3, eq12_e1082_d_b4, eq12_e1082_d_b5, eq12_e1082_d_b6, eq12_e1082_d_b7, eq12_e1082_d_b8, eq12_e1082_d_b9, eq12_e1082_d_b10, eq12_e1082_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1085: f64 = (p.p87 * s.v[861]);
        let eq13_e1085_d_n0: f64 = (p.p87 * s.dn[861][0]);
        let eq13_e1085_d_n1: f64 = (p.p87 * s.dn[861][1]);
        let eq13_e1085_d_n2: f64 = (p.p87 * s.dn[861][2]);
        let eq13_e1085_d_n3: f64 = (p.p87 * s.dn[861][3]);
        let eq13_e1085_d_n4: f64 = (p.p87 * s.dn[861][4]);
        let eq13_e1085_d_n5: f64 = (p.p87 * s.dn[861][5]);
        let eq13_e1085_d_n6: f64 = (p.p87 * s.dn[861][6]);
        let eq13_e1085_d_n7: f64 = (p.p87 * s.dn[861][7]);
        let eq13_e1085_d_n8: f64 = (p.p87 * s.dn[861][8]);
        let eq13_e1085_d_n9: f64 = (p.p87 * s.dn[861][9]);
        let eq13_e1085_d_n10: f64 = (p.p87 * s.dn[861][10]);
        let eq13_e1085_d_n11: f64 = (p.p87 * s.dn[861][11]);
        let eq13_e1085_d_n12: f64 = (p.p87 * s.dn[861][12]);
        let eq13_e1085_d_n13: f64 = (p.p87 * s.dn[861][13]);
        let eq13_e1085_d_n14: f64 = (p.p87 * s.dn[861][14]);
        let eq13_e1085_d_n15: f64 = (p.p87 * s.dn[861][15]);
        let eq13_e1085_d_n16: f64 = (p.p87 * s.dn[861][16]);
        let eq13_e1085_d_n17: f64 = (p.p87 * s.dn[861][17]);
        let eq13_e1085_d_b0: f64 = (p.p87 * s.db[861][0]);
        let eq13_e1085_d_b1: f64 = (p.p87 * s.db[861][1]);
        let eq13_e1085_d_b2: f64 = (p.p87 * s.db[861][2]);
        let eq13_e1085_d_b3: f64 = (p.p87 * s.db[861][3]);
        let eq13_e1085_d_b4: f64 = (p.p87 * s.db[861][4]);
        let eq13_e1085_d_b5: f64 = (p.p87 * s.db[861][5]);
        let eq13_e1085_d_b6: f64 = (p.p87 * s.db[861][6]);
        let eq13_e1085_d_b7: f64 = (p.p87 * s.db[861][7]);
        let eq13_e1085_d_b8: f64 = (p.p87 * s.db[861][8]);
        let eq13_e1085_d_b9: f64 = (p.p87 * s.db[861][9]);
        let eq13_e1085_d_b10: f64 = (p.p87 * s.db[861][10]);
        let eq13_e1085_d_b11: f64 = (p.p87 * s.db[861][11]);
        let eq13_value: f64 = eq13_e1085;
        let eq13_node_derivatives: [f64; 18] = [eq13_e1085_d_n0, eq13_e1085_d_n1, eq13_e1085_d_n2, eq13_e1085_d_n3, eq13_e1085_d_n4, eq13_e1085_d_n5, eq13_e1085_d_n6, eq13_e1085_d_n7, eq13_e1085_d_n8, eq13_e1085_d_n9, eq13_e1085_d_n10, eq13_e1085_d_n11, eq13_e1085_d_n12, eq13_e1085_d_n13, eq13_e1085_d_n14, eq13_e1085_d_n15, eq13_e1085_d_n16, eq13_e1085_d_n17];
        let eq13_branch_derivatives: [f64; 12] = [eq13_e1085_d_b0, eq13_e1085_d_b1, eq13_e1085_d_b2, eq13_e1085_d_b3, eq13_e1085_d_b4, eq13_e1085_d_b5, eq13_e1085_d_b6, eq13_e1085_d_b7, eq13_e1085_d_b8, eq13_e1085_d_b9, eq13_e1085_d_b10, eq13_e1085_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1088: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[66]);
        let eq14_e1089: f64 = (p.p87 * eq14_e1088);
        let eq14_e1089_d_n0: f64 = (p.p87 * (s.dn[66][0] * ddt_scale));
        let eq14_e1089_d_n1: f64 = (p.p87 * (s.dn[66][1] * ddt_scale));
        let eq14_e1089_d_n2: f64 = (p.p87 * (s.dn[66][2] * ddt_scale));
        let eq14_e1089_d_n3: f64 = (p.p87 * (s.dn[66][3] * ddt_scale));
        let eq14_e1089_d_n4: f64 = (p.p87 * (s.dn[66][4] * ddt_scale));
        let eq14_e1089_d_n5: f64 = (p.p87 * (s.dn[66][5] * ddt_scale));
        let eq14_e1089_d_n6: f64 = (p.p87 * (s.dn[66][6] * ddt_scale));
        let eq14_e1089_d_n7: f64 = (p.p87 * (s.dn[66][7] * ddt_scale));
        let eq14_e1089_d_n8: f64 = (p.p87 * (s.dn[66][8] * ddt_scale));
        let eq14_e1089_d_n9: f64 = (p.p87 * (s.dn[66][9] * ddt_scale));
        let eq14_e1089_d_n10: f64 = (p.p87 * (s.dn[66][10] * ddt_scale));
        let eq14_e1089_d_n11: f64 = (p.p87 * (s.dn[66][11] * ddt_scale));
        let eq14_e1089_d_n12: f64 = (p.p87 * (s.dn[66][12] * ddt_scale));
        let eq14_e1089_d_n13: f64 = (p.p87 * (s.dn[66][13] * ddt_scale));
        let eq14_e1089_d_n14: f64 = (p.p87 * (s.dn[66][14] * ddt_scale));
        let eq14_e1089_d_n15: f64 = (p.p87 * (s.dn[66][15] * ddt_scale));
        let eq14_e1089_d_n16: f64 = (p.p87 * (s.dn[66][16] * ddt_scale));
        let eq14_e1089_d_n17: f64 = (p.p87 * (s.dn[66][17] * ddt_scale));
        let eq14_e1089_d_b0: f64 = (p.p87 * (s.db[66][0] * ddt_scale));
        let eq14_e1089_d_b1: f64 = (p.p87 * (s.db[66][1] * ddt_scale));
        let eq14_e1089_d_b2: f64 = (p.p87 * (s.db[66][2] * ddt_scale));
        let eq14_e1089_d_b3: f64 = (p.p87 * (s.db[66][3] * ddt_scale));
        let eq14_e1089_d_b4: f64 = (p.p87 * (s.db[66][4] * ddt_scale));
        let eq14_e1089_d_b5: f64 = (p.p87 * (s.db[66][5] * ddt_scale));
        let eq14_e1089_d_b6: f64 = (p.p87 * (s.db[66][6] * ddt_scale));
        let eq14_e1089_d_b7: f64 = (p.p87 * (s.db[66][7] * ddt_scale));
        let eq14_e1089_d_b8: f64 = (p.p87 * (s.db[66][8] * ddt_scale));
        let eq14_e1089_d_b9: f64 = (p.p87 * (s.db[66][9] * ddt_scale));
        let eq14_e1089_d_b10: f64 = (p.p87 * (s.db[66][10] * ddt_scale));
        let eq14_e1089_d_b11: f64 = (p.p87 * (s.db[66][11] * ddt_scale));
        let eq14_value: f64 = eq14_e1089;
        let eq14_node_derivatives: [f64; 18] = [eq14_e1089_d_n0, eq14_e1089_d_n1, eq14_e1089_d_n2, eq14_e1089_d_n3, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n12, eq14_e1089_d_n13, eq14_e1089_d_n14, eq14_e1089_d_n15, eq14_e1089_d_n16, eq14_e1089_d_n17];
        let eq14_branch_derivatives: [f64; 12] = [eq14_e1089_d_b0, eq14_e1089_d_b1, eq14_e1089_d_b2, eq14_e1089_d_b3, eq14_e1089_d_b4, eq14_e1089_d_b5, eq14_e1089_d_b6, eq14_e1089_d_b7, eq14_e1089_d_b8, eq14_e1089_d_b9, eq14_e1089_d_b10, eq14_e1089_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[65]);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * (s.dn[65][0] * ddt_scale));
        let eq15_e1093_d_n1: f64 = (p.p87 * (s.dn[65][1] * ddt_scale));
        let eq15_e1093_d_n2: f64 = (p.p87 * (s.dn[65][2] * ddt_scale));
        let eq15_e1093_d_n3: f64 = (p.p87 * (s.dn[65][3] * ddt_scale));
        let eq15_e1093_d_n4: f64 = (p.p87 * (s.dn[65][4] * ddt_scale));
        let eq15_e1093_d_n5: f64 = (p.p87 * (s.dn[65][5] * ddt_scale));
        let eq15_e1093_d_n6: f64 = (p.p87 * (s.dn[65][6] * ddt_scale));
        let eq15_e1093_d_n7: f64 = (p.p87 * (s.dn[65][7] * ddt_scale));
        let eq15_e1093_d_n8: f64 = (p.p87 * (s.dn[65][8] * ddt_scale));
        let eq15_e1093_d_n9: f64 = (p.p87 * (s.dn[65][9] * ddt_scale));
        let eq15_e1093_d_n10: f64 = (p.p87 * (s.dn[65][10] * ddt_scale));
        let eq15_e1093_d_n11: f64 = (p.p87 * (s.dn[65][11] * ddt_scale));
        let eq15_e1093_d_n12: f64 = (p.p87 * (s.dn[65][12] * ddt_scale));
        let eq15_e1093_d_n13: f64 = (p.p87 * (s.dn[65][13] * ddt_scale));
        let eq15_e1093_d_n14: f64 = (p.p87 * (s.dn[65][14] * ddt_scale));
        let eq15_e1093_d_n15: f64 = (p.p87 * (s.dn[65][15] * ddt_scale));
        let eq15_e1093_d_n16: f64 = (p.p87 * (s.dn[65][16] * ddt_scale));
        let eq15_e1093_d_n17: f64 = (p.p87 * (s.dn[65][17] * ddt_scale));
        let eq15_e1093_d_b0: f64 = (p.p87 * (s.db[65][0] * ddt_scale));
        let eq15_e1093_d_b1: f64 = (p.p87 * (s.db[65][1] * ddt_scale));
        let eq15_e1093_d_b2: f64 = (p.p87 * (s.db[65][2] * ddt_scale));
        let eq15_e1093_d_b3: f64 = (p.p87 * (s.db[65][3] * ddt_scale));
        let eq15_e1093_d_b4: f64 = (p.p87 * (s.db[65][4] * ddt_scale));
        let eq15_e1093_d_b5: f64 = (p.p87 * (s.db[65][5] * ddt_scale));
        let eq15_e1093_d_b6: f64 = (p.p87 * (s.db[65][6] * ddt_scale));
        let eq15_e1093_d_b7: f64 = (p.p87 * (s.db[65][7] * ddt_scale));
        let eq15_e1093_d_b8: f64 = (p.p87 * (s.db[65][8] * ddt_scale));
        let eq15_e1093_d_b9: f64 = (p.p87 * (s.db[65][9] * ddt_scale));
        let eq15_e1093_d_b10: f64 = (p.p87 * (s.db[65][10] * ddt_scale));
        let eq15_e1093_d_b11: f64 = (p.p87 * (s.db[65][11] * ddt_scale));
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivatives: [f64; 18] = [eq15_e1093_d_n0, eq15_e1093_d_n1, eq15_e1093_d_n2, eq15_e1093_d_n3, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n12, eq15_e1093_d_n13, eq15_e1093_d_n14, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_branch_derivatives: [f64; 12] = [eq15_e1093_d_b0, eq15_e1093_d_b1, eq15_e1093_d_b2, eq15_e1093_d_b3, eq15_e1093_d_b4, eq15_e1093_d_b5, eq15_e1093_d_b6, eq15_e1093_d_b7, eq15_e1093_d_b8, eq15_e1093_d_b9, eq15_e1093_d_b10, eq15_e1093_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1099, eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17, eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8, eq16_e1099_d_b9, eq16_e1099_d_b10, eq16_e1099_d_b11,) = {
    if s.b[3405] {
        let eq16_e1097: f64 = (p.p87 * s.v[870]);
        let eq16_e1097_d_n0: f64 = (p.p87 * s.dn[870][0]);
        let eq16_e1097_d_n1: f64 = (p.p87 * s.dn[870][1]);
        let eq16_e1097_d_n2: f64 = (p.p87 * s.dn[870][2]);
        let eq16_e1097_d_n3: f64 = (p.p87 * s.dn[870][3]);
        let eq16_e1097_d_n4: f64 = (p.p87 * s.dn[870][4]);
        let eq16_e1097_d_n5: f64 = (p.p87 * s.dn[870][5]);
        let eq16_e1097_d_n6: f64 = (p.p87 * s.dn[870][6]);
        let eq16_e1097_d_n7: f64 = (p.p87 * s.dn[870][7]);
        let eq16_e1097_d_n8: f64 = (p.p87 * s.dn[870][8]);
        let eq16_e1097_d_n9: f64 = (p.p87 * s.dn[870][9]);
        let eq16_e1097_d_n10: f64 = (p.p87 * s.dn[870][10]);
        let eq16_e1097_d_n11: f64 = (p.p87 * s.dn[870][11]);
        let eq16_e1097_d_n12: f64 = (p.p87 * s.dn[870][12]);
        let eq16_e1097_d_n13: f64 = (p.p87 * s.dn[870][13]);
        let eq16_e1097_d_n14: f64 = (p.p87 * s.dn[870][14]);
        let eq16_e1097_d_n15: f64 = (p.p87 * s.dn[870][15]);
        let eq16_e1097_d_n16: f64 = (p.p87 * s.dn[870][16]);
        let eq16_e1097_d_n17: f64 = (p.p87 * s.dn[870][17]);
        let eq16_e1097_d_b0: f64 = (p.p87 * s.db[870][0]);
        let eq16_e1097_d_b1: f64 = (p.p87 * s.db[870][1]);
        let eq16_e1097_d_b2: f64 = (p.p87 * s.db[870][2]);
        let eq16_e1097_d_b3: f64 = (p.p87 * s.db[870][3]);
        let eq16_e1097_d_b4: f64 = (p.p87 * s.db[870][4]);
        let eq16_e1097_d_b5: f64 = (p.p87 * s.db[870][5]);
        let eq16_e1097_d_b6: f64 = (p.p87 * s.db[870][6]);
        let eq16_e1097_d_b7: f64 = (p.p87 * s.db[870][7]);
        let eq16_e1097_d_b8: f64 = (p.p87 * s.db[870][8]);
        let eq16_e1097_d_b9: f64 = (p.p87 * s.db[870][9]);
        let eq16_e1097_d_b10: f64 = (p.p87 * s.db[870][10]);
        let eq16_e1097_d_b11: f64 = (p.p87 * s.db[870][11]);
        (eq16_e1097, eq16_e1097_d_n0, eq16_e1097_d_n1, eq16_e1097_d_n2, eq16_e1097_d_n3, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n12, eq16_e1097_d_n13, eq16_e1097_d_n14, eq16_e1097_d_n15, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_b0, eq16_e1097_d_b1, eq16_e1097_d_b2, eq16_e1097_d_b3, eq16_e1097_d_b4, eq16_e1097_d_b5, eq16_e1097_d_b6, eq16_e1097_d_b7, eq16_e1097_d_b8, eq16_e1097_d_b9, eq16_e1097_d_b10, eq16_e1097_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1099;
        let eq16_node_derivatives: [f64; 18] = [eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17];
        let eq16_branch_derivatives: [f64; 12] = [eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8, eq16_e1099_d_b9, eq16_e1099_d_b10, eq16_e1099_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1105, eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17, eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8, eq17_e1105_d_b9, eq17_e1105_d_b10, eq17_e1105_d_b11,) = {
    if s.b[3405] {
        let eq17_e1103: f64 = (p.p87 * s.v[869]);
        let eq17_e1103_d_n0: f64 = (p.p87 * s.dn[869][0]);
        let eq17_e1103_d_n1: f64 = (p.p87 * s.dn[869][1]);
        let eq17_e1103_d_n2: f64 = (p.p87 * s.dn[869][2]);
        let eq17_e1103_d_n3: f64 = (p.p87 * s.dn[869][3]);
        let eq17_e1103_d_n4: f64 = (p.p87 * s.dn[869][4]);
        let eq17_e1103_d_n5: f64 = (p.p87 * s.dn[869][5]);
        let eq17_e1103_d_n6: f64 = (p.p87 * s.dn[869][6]);
        let eq17_e1103_d_n7: f64 = (p.p87 * s.dn[869][7]);
        let eq17_e1103_d_n8: f64 = (p.p87 * s.dn[869][8]);
        let eq17_e1103_d_n9: f64 = (p.p87 * s.dn[869][9]);
        let eq17_e1103_d_n10: f64 = (p.p87 * s.dn[869][10]);
        let eq17_e1103_d_n11: f64 = (p.p87 * s.dn[869][11]);
        let eq17_e1103_d_n12: f64 = (p.p87 * s.dn[869][12]);
        let eq17_e1103_d_n13: f64 = (p.p87 * s.dn[869][13]);
        let eq17_e1103_d_n14: f64 = (p.p87 * s.dn[869][14]);
        let eq17_e1103_d_n15: f64 = (p.p87 * s.dn[869][15]);
        let eq17_e1103_d_n16: f64 = (p.p87 * s.dn[869][16]);
        let eq17_e1103_d_n17: f64 = (p.p87 * s.dn[869][17]);
        let eq17_e1103_d_b0: f64 = (p.p87 * s.db[869][0]);
        let eq17_e1103_d_b1: f64 = (p.p87 * s.db[869][1]);
        let eq17_e1103_d_b2: f64 = (p.p87 * s.db[869][2]);
        let eq17_e1103_d_b3: f64 = (p.p87 * s.db[869][3]);
        let eq17_e1103_d_b4: f64 = (p.p87 * s.db[869][4]);
        let eq17_e1103_d_b5: f64 = (p.p87 * s.db[869][5]);
        let eq17_e1103_d_b6: f64 = (p.p87 * s.db[869][6]);
        let eq17_e1103_d_b7: f64 = (p.p87 * s.db[869][7]);
        let eq17_e1103_d_b8: f64 = (p.p87 * s.db[869][8]);
        let eq17_e1103_d_b9: f64 = (p.p87 * s.db[869][9]);
        let eq17_e1103_d_b10: f64 = (p.p87 * s.db[869][10]);
        let eq17_e1103_d_b11: f64 = (p.p87 * s.db[869][11]);
        (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n1, eq17_e1103_d_n2, eq17_e1103_d_n3, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n12, eq17_e1103_d_n13, eq17_e1103_d_n14, eq17_e1103_d_n15, eq17_e1103_d_n16, eq17_e1103_d_n17, eq17_e1103_d_b0, eq17_e1103_d_b1, eq17_e1103_d_b2, eq17_e1103_d_b3, eq17_e1103_d_b4, eq17_e1103_d_b5, eq17_e1103_d_b6, eq17_e1103_d_b7, eq17_e1103_d_b8, eq17_e1103_d_b9, eq17_e1103_d_b10, eq17_e1103_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1105;
        let eq17_node_derivatives: [f64; 18] = [eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17];
        let eq17_branch_derivatives: [f64; 12] = [eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8, eq17_e1105_d_b9, eq17_e1105_d_b10, eq17_e1105_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11,) = {
    if s.b[3405] {
        let eq18_e1109: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[68]);
        let eq18_e1110: f64 = (p.p87 * eq18_e1109);
        let eq18_e1110_d_n0: f64 = (p.p87 * (s.dn[68][0] * ddt_scale));
        let eq18_e1110_d_n1: f64 = (p.p87 * (s.dn[68][1] * ddt_scale));
        let eq18_e1110_d_n2: f64 = (p.p87 * (s.dn[68][2] * ddt_scale));
        let eq18_e1110_d_n3: f64 = (p.p87 * (s.dn[68][3] * ddt_scale));
        let eq18_e1110_d_n4: f64 = (p.p87 * (s.dn[68][4] * ddt_scale));
        let eq18_e1110_d_n5: f64 = (p.p87 * (s.dn[68][5] * ddt_scale));
        let eq18_e1110_d_n6: f64 = (p.p87 * (s.dn[68][6] * ddt_scale));
        let eq18_e1110_d_n7: f64 = (p.p87 * (s.dn[68][7] * ddt_scale));
        let eq18_e1110_d_n8: f64 = (p.p87 * (s.dn[68][8] * ddt_scale));
        let eq18_e1110_d_n9: f64 = (p.p87 * (s.dn[68][9] * ddt_scale));
        let eq18_e1110_d_n10: f64 = (p.p87 * (s.dn[68][10] * ddt_scale));
        let eq18_e1110_d_n11: f64 = (p.p87 * (s.dn[68][11] * ddt_scale));
        let eq18_e1110_d_n12: f64 = (p.p87 * (s.dn[68][12] * ddt_scale));
        let eq18_e1110_d_n13: f64 = (p.p87 * (s.dn[68][13] * ddt_scale));
        let eq18_e1110_d_n14: f64 = (p.p87 * (s.dn[68][14] * ddt_scale));
        let eq18_e1110_d_n15: f64 = (p.p87 * (s.dn[68][15] * ddt_scale));
        let eq18_e1110_d_n16: f64 = (p.p87 * (s.dn[68][16] * ddt_scale));
        let eq18_e1110_d_n17: f64 = (p.p87 * (s.dn[68][17] * ddt_scale));
        let eq18_e1110_d_b0: f64 = (p.p87 * (s.db[68][0] * ddt_scale));
        let eq18_e1110_d_b1: f64 = (p.p87 * (s.db[68][1] * ddt_scale));
        let eq18_e1110_d_b2: f64 = (p.p87 * (s.db[68][2] * ddt_scale));
        let eq18_e1110_d_b3: f64 = (p.p87 * (s.db[68][3] * ddt_scale));
        let eq18_e1110_d_b4: f64 = (p.p87 * (s.db[68][4] * ddt_scale));
        let eq18_e1110_d_b5: f64 = (p.p87 * (s.db[68][5] * ddt_scale));
        let eq18_e1110_d_b6: f64 = (p.p87 * (s.db[68][6] * ddt_scale));
        let eq18_e1110_d_b7: f64 = (p.p87 * (s.db[68][7] * ddt_scale));
        let eq18_e1110_d_b8: f64 = (p.p87 * (s.db[68][8] * ddt_scale));
        let eq18_e1110_d_b9: f64 = (p.p87 * (s.db[68][9] * ddt_scale));
        let eq18_e1110_d_b10: f64 = (p.p87 * (s.db[68][10] * ddt_scale));
        let eq18_e1110_d_b11: f64 = (p.p87 * (s.db[68][11] * ddt_scale));
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8, eq18_e1110_d_b9, eq18_e1110_d_b10, eq18_e1110_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1112;
        let eq18_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17];
        let eq18_branch_derivatives: [f64; 12] = [eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11,) = {
    if s.b[3405] {
        let eq19_e1116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[67]);
        let eq19_e1117: f64 = (p.p87 * eq19_e1116);
        let eq19_e1117_d_n0: f64 = (p.p87 * (s.dn[67][0] * ddt_scale));
        let eq19_e1117_d_n1: f64 = (p.p87 * (s.dn[67][1] * ddt_scale));
        let eq19_e1117_d_n2: f64 = (p.p87 * (s.dn[67][2] * ddt_scale));
        let eq19_e1117_d_n3: f64 = (p.p87 * (s.dn[67][3] * ddt_scale));
        let eq19_e1117_d_n4: f64 = (p.p87 * (s.dn[67][4] * ddt_scale));
        let eq19_e1117_d_n5: f64 = (p.p87 * (s.dn[67][5] * ddt_scale));
        let eq19_e1117_d_n6: f64 = (p.p87 * (s.dn[67][6] * ddt_scale));
        let eq19_e1117_d_n7: f64 = (p.p87 * (s.dn[67][7] * ddt_scale));
        let eq19_e1117_d_n8: f64 = (p.p87 * (s.dn[67][8] * ddt_scale));
        let eq19_e1117_d_n9: f64 = (p.p87 * (s.dn[67][9] * ddt_scale));
        let eq19_e1117_d_n10: f64 = (p.p87 * (s.dn[67][10] * ddt_scale));
        let eq19_e1117_d_n11: f64 = (p.p87 * (s.dn[67][11] * ddt_scale));
        let eq19_e1117_d_n12: f64 = (p.p87 * (s.dn[67][12] * ddt_scale));
        let eq19_e1117_d_n13: f64 = (p.p87 * (s.dn[67][13] * ddt_scale));
        let eq19_e1117_d_n14: f64 = (p.p87 * (s.dn[67][14] * ddt_scale));
        let eq19_e1117_d_n15: f64 = (p.p87 * (s.dn[67][15] * ddt_scale));
        let eq19_e1117_d_n16: f64 = (p.p87 * (s.dn[67][16] * ddt_scale));
        let eq19_e1117_d_n17: f64 = (p.p87 * (s.dn[67][17] * ddt_scale));
        let eq19_e1117_d_b0: f64 = (p.p87 * (s.db[67][0] * ddt_scale));
        let eq19_e1117_d_b1: f64 = (p.p87 * (s.db[67][1] * ddt_scale));
        let eq19_e1117_d_b2: f64 = (p.p87 * (s.db[67][2] * ddt_scale));
        let eq19_e1117_d_b3: f64 = (p.p87 * (s.db[67][3] * ddt_scale));
        let eq19_e1117_d_b4: f64 = (p.p87 * (s.db[67][4] * ddt_scale));
        let eq19_e1117_d_b5: f64 = (p.p87 * (s.db[67][5] * ddt_scale));
        let eq19_e1117_d_b6: f64 = (p.p87 * (s.db[67][6] * ddt_scale));
        let eq19_e1117_d_b7: f64 = (p.p87 * (s.db[67][7] * ddt_scale));
        let eq19_e1117_d_b8: f64 = (p.p87 * (s.db[67][8] * ddt_scale));
        let eq19_e1117_d_b9: f64 = (p.p87 * (s.db[67][9] * ddt_scale));
        let eq19_e1117_d_b10: f64 = (p.p87 * (s.db[67][10] * ddt_scale));
        let eq19_e1117_d_b11: f64 = (p.p87 * (s.db[67][11] * ddt_scale));
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8, eq19_e1117_d_b9, eq19_e1117_d_b10, eq19_e1117_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1119;
        let eq19_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17];
        let eq19_branch_derivatives: [f64; 12] = [eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1125, eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17, eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8, eq20_e1125_d_b9, eq20_e1125_d_b10, eq20_e1125_d_b11,) = {
    if s.b[3406] {
        let eq20_e1123: f64 = (p.p87 * s.v[200]);
        let eq20_e1123_d_n0: f64 = (p.p87 * s.dn[200][0]);
        let eq20_e1123_d_n1: f64 = (p.p87 * s.dn[200][1]);
        let eq20_e1123_d_n2: f64 = (p.p87 * s.dn[200][2]);
        let eq20_e1123_d_n3: f64 = (p.p87 * s.dn[200][3]);
        let eq20_e1123_d_n4: f64 = (p.p87 * s.dn[200][4]);
        let eq20_e1123_d_n5: f64 = (p.p87 * s.dn[200][5]);
        let eq20_e1123_d_n6: f64 = (p.p87 * s.dn[200][6]);
        let eq20_e1123_d_n7: f64 = (p.p87 * s.dn[200][7]);
        let eq20_e1123_d_n8: f64 = (p.p87 * s.dn[200][8]);
        let eq20_e1123_d_n9: f64 = (p.p87 * s.dn[200][9]);
        let eq20_e1123_d_n10: f64 = (p.p87 * s.dn[200][10]);
        let eq20_e1123_d_n11: f64 = (p.p87 * s.dn[200][11]);
        let eq20_e1123_d_n12: f64 = (p.p87 * s.dn[200][12]);
        let eq20_e1123_d_n13: f64 = (p.p87 * s.dn[200][13]);
        let eq20_e1123_d_n14: f64 = (p.p87 * s.dn[200][14]);
        let eq20_e1123_d_n15: f64 = (p.p87 * s.dn[200][15]);
        let eq20_e1123_d_n16: f64 = (p.p87 * s.dn[200][16]);
        let eq20_e1123_d_n17: f64 = (p.p87 * s.dn[200][17]);
        let eq20_e1123_d_b0: f64 = (p.p87 * s.db[200][0]);
        let eq20_e1123_d_b1: f64 = (p.p87 * s.db[200][1]);
        let eq20_e1123_d_b2: f64 = (p.p87 * s.db[200][2]);
        let eq20_e1123_d_b3: f64 = (p.p87 * s.db[200][3]);
        let eq20_e1123_d_b4: f64 = (p.p87 * s.db[200][4]);
        let eq20_e1123_d_b5: f64 = (p.p87 * s.db[200][5]);
        let eq20_e1123_d_b6: f64 = (p.p87 * s.db[200][6]);
        let eq20_e1123_d_b7: f64 = (p.p87 * s.db[200][7]);
        let eq20_e1123_d_b8: f64 = (p.p87 * s.db[200][8]);
        let eq20_e1123_d_b9: f64 = (p.p87 * s.db[200][9]);
        let eq20_e1123_d_b10: f64 = (p.p87 * s.db[200][10]);
        let eq20_e1123_d_b11: f64 = (p.p87 * s.db[200][11]);
        (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n1, eq20_e1123_d_n2, eq20_e1123_d_n3, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n12, eq20_e1123_d_n13, eq20_e1123_d_n14, eq20_e1123_d_n15, eq20_e1123_d_n16, eq20_e1123_d_n17, eq20_e1123_d_b0, eq20_e1123_d_b1, eq20_e1123_d_b2, eq20_e1123_d_b3, eq20_e1123_d_b4, eq20_e1123_d_b5, eq20_e1123_d_b6, eq20_e1123_d_b7, eq20_e1123_d_b8, eq20_e1123_d_b9, eq20_e1123_d_b10, eq20_e1123_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        let eq20_node_derivatives: [f64; 18] = [eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17];
        let eq20_branch_derivatives: [f64; 12] = [eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8, eq20_e1125_d_b9, eq20_e1125_d_b10, eq20_e1125_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1131, eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17, eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8, eq21_e1131_d_b9, eq21_e1131_d_b10, eq21_e1131_d_b11,) = {
    if s.b[3406] {
        let eq21_e1129: f64 = (p.p87 * s.v[201]);
        let eq21_e1129_d_n0: f64 = (p.p87 * s.dn[201][0]);
        let eq21_e1129_d_n1: f64 = (p.p87 * s.dn[201][1]);
        let eq21_e1129_d_n2: f64 = (p.p87 * s.dn[201][2]);
        let eq21_e1129_d_n3: f64 = (p.p87 * s.dn[201][3]);
        let eq21_e1129_d_n4: f64 = (p.p87 * s.dn[201][4]);
        let eq21_e1129_d_n5: f64 = (p.p87 * s.dn[201][5]);
        let eq21_e1129_d_n6: f64 = (p.p87 * s.dn[201][6]);
        let eq21_e1129_d_n7: f64 = (p.p87 * s.dn[201][7]);
        let eq21_e1129_d_n8: f64 = (p.p87 * s.dn[201][8]);
        let eq21_e1129_d_n9: f64 = (p.p87 * s.dn[201][9]);
        let eq21_e1129_d_n10: f64 = (p.p87 * s.dn[201][10]);
        let eq21_e1129_d_n11: f64 = (p.p87 * s.dn[201][11]);
        let eq21_e1129_d_n12: f64 = (p.p87 * s.dn[201][12]);
        let eq21_e1129_d_n13: f64 = (p.p87 * s.dn[201][13]);
        let eq21_e1129_d_n14: f64 = (p.p87 * s.dn[201][14]);
        let eq21_e1129_d_n15: f64 = (p.p87 * s.dn[201][15]);
        let eq21_e1129_d_n16: f64 = (p.p87 * s.dn[201][16]);
        let eq21_e1129_d_n17: f64 = (p.p87 * s.dn[201][17]);
        let eq21_e1129_d_b0: f64 = (p.p87 * s.db[201][0]);
        let eq21_e1129_d_b1: f64 = (p.p87 * s.db[201][1]);
        let eq21_e1129_d_b2: f64 = (p.p87 * s.db[201][2]);
        let eq21_e1129_d_b3: f64 = (p.p87 * s.db[201][3]);
        let eq21_e1129_d_b4: f64 = (p.p87 * s.db[201][4]);
        let eq21_e1129_d_b5: f64 = (p.p87 * s.db[201][5]);
        let eq21_e1129_d_b6: f64 = (p.p87 * s.db[201][6]);
        let eq21_e1129_d_b7: f64 = (p.p87 * s.db[201][7]);
        let eq21_e1129_d_b8: f64 = (p.p87 * s.db[201][8]);
        let eq21_e1129_d_b9: f64 = (p.p87 * s.db[201][9]);
        let eq21_e1129_d_b10: f64 = (p.p87 * s.db[201][10]);
        let eq21_e1129_d_b11: f64 = (p.p87 * s.db[201][11]);
        (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n1, eq21_e1129_d_n2, eq21_e1129_d_n3, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n12, eq21_e1129_d_n13, eq21_e1129_d_n14, eq21_e1129_d_n15, eq21_e1129_d_n16, eq21_e1129_d_n17, eq21_e1129_d_b0, eq21_e1129_d_b1, eq21_e1129_d_b2, eq21_e1129_d_b3, eq21_e1129_d_b4, eq21_e1129_d_b5, eq21_e1129_d_b6, eq21_e1129_d_b7, eq21_e1129_d_b8, eq21_e1129_d_b9, eq21_e1129_d_b10, eq21_e1129_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1131;
        let eq21_node_derivatives: [f64; 18] = [eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17];
        let eq21_branch_derivatives: [f64; 12] = [eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8, eq21_e1131_d_b9, eq21_e1131_d_b10, eq21_e1131_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1137, eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17, eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8, eq22_e1137_d_b9, eq22_e1137_d_b10, eq22_e1137_d_b11,) = {
    if s.b[3406] {
        let eq22_e1135: f64 = (p.p87 * s.v[202]);
        let eq22_e1135_d_n0: f64 = (p.p87 * s.dn[202][0]);
        let eq22_e1135_d_n1: f64 = (p.p87 * s.dn[202][1]);
        let eq22_e1135_d_n2: f64 = (p.p87 * s.dn[202][2]);
        let eq22_e1135_d_n3: f64 = (p.p87 * s.dn[202][3]);
        let eq22_e1135_d_n4: f64 = (p.p87 * s.dn[202][4]);
        let eq22_e1135_d_n5: f64 = (p.p87 * s.dn[202][5]);
        let eq22_e1135_d_n6: f64 = (p.p87 * s.dn[202][6]);
        let eq22_e1135_d_n7: f64 = (p.p87 * s.dn[202][7]);
        let eq22_e1135_d_n8: f64 = (p.p87 * s.dn[202][8]);
        let eq22_e1135_d_n9: f64 = (p.p87 * s.dn[202][9]);
        let eq22_e1135_d_n10: f64 = (p.p87 * s.dn[202][10]);
        let eq22_e1135_d_n11: f64 = (p.p87 * s.dn[202][11]);
        let eq22_e1135_d_n12: f64 = (p.p87 * s.dn[202][12]);
        let eq22_e1135_d_n13: f64 = (p.p87 * s.dn[202][13]);
        let eq22_e1135_d_n14: f64 = (p.p87 * s.dn[202][14]);
        let eq22_e1135_d_n15: f64 = (p.p87 * s.dn[202][15]);
        let eq22_e1135_d_n16: f64 = (p.p87 * s.dn[202][16]);
        let eq22_e1135_d_n17: f64 = (p.p87 * s.dn[202][17]);
        let eq22_e1135_d_b0: f64 = (p.p87 * s.db[202][0]);
        let eq22_e1135_d_b1: f64 = (p.p87 * s.db[202][1]);
        let eq22_e1135_d_b2: f64 = (p.p87 * s.db[202][2]);
        let eq22_e1135_d_b3: f64 = (p.p87 * s.db[202][3]);
        let eq22_e1135_d_b4: f64 = (p.p87 * s.db[202][4]);
        let eq22_e1135_d_b5: f64 = (p.p87 * s.db[202][5]);
        let eq22_e1135_d_b6: f64 = (p.p87 * s.db[202][6]);
        let eq22_e1135_d_b7: f64 = (p.p87 * s.db[202][7]);
        let eq22_e1135_d_b8: f64 = (p.p87 * s.db[202][8]);
        let eq22_e1135_d_b9: f64 = (p.p87 * s.db[202][9]);
        let eq22_e1135_d_b10: f64 = (p.p87 * s.db[202][10]);
        let eq22_e1135_d_b11: f64 = (p.p87 * s.db[202][11]);
        (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n1, eq22_e1135_d_n2, eq22_e1135_d_n3, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n12, eq22_e1135_d_n13, eq22_e1135_d_n14, eq22_e1135_d_n15, eq22_e1135_d_n16, eq22_e1135_d_n17, eq22_e1135_d_b0, eq22_e1135_d_b1, eq22_e1135_d_b2, eq22_e1135_d_b3, eq22_e1135_d_b4, eq22_e1135_d_b5, eq22_e1135_d_b6, eq22_e1135_d_b7, eq22_e1135_d_b8, eq22_e1135_d_b9, eq22_e1135_d_b10, eq22_e1135_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1137;
        let eq22_node_derivatives: [f64; 18] = [eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17];
        let eq22_branch_derivatives: [f64; 12] = [eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8, eq22_e1137_d_b9, eq22_e1137_d_b10, eq22_e1137_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1143, eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17, eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8, eq23_e1143_d_b9, eq23_e1143_d_b10, eq23_e1143_d_b11,) = {
    if (s.v[75] != 0.0) {
        let eq23_e1141: f64 = ((nv0 - nv5) / s.v[4]);
        let eq23_e1141_d_n0: f64 = ((s.v[4] - ((nv0 - nv5) * s.dn[4][0])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n1: f64 = (-(((nv0 - nv5) * s.dn[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n2: f64 = (-(((nv0 - nv5) * s.dn[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n3: f64 = (-(((nv0 - nv5) * s.dn[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n4: f64 = (-(((nv0 - nv5) * s.dn[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n5: f64 = (((-s.v[4]) - ((nv0 - nv5) * s.dn[4][5])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n6: f64 = (-(((nv0 - nv5) * s.dn[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n7: f64 = (-(((nv0 - nv5) * s.dn[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n8: f64 = (-(((nv0 - nv5) * s.dn[4][8]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n9: f64 = (-(((nv0 - nv5) * s.dn[4][9]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n10: f64 = (-(((nv0 - nv5) * s.dn[4][10]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n11: f64 = (-(((nv0 - nv5) * s.dn[4][11]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n12: f64 = (-(((nv0 - nv5) * s.dn[4][12]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n13: f64 = (-(((nv0 - nv5) * s.dn[4][13]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n14: f64 = (-(((nv0 - nv5) * s.dn[4][14]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n15: f64 = (-(((nv0 - nv5) * s.dn[4][15]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n16: f64 = (-(((nv0 - nv5) * s.dn[4][16]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n17: f64 = (-(((nv0 - nv5) * s.dn[4][17]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b0: f64 = (-(((nv0 - nv5) * s.db[4][0]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b1: f64 = (-(((nv0 - nv5) * s.db[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b2: f64 = (-(((nv0 - nv5) * s.db[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b3: f64 = (-(((nv0 - nv5) * s.db[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b4: f64 = (-(((nv0 - nv5) * s.db[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b5: f64 = (-(((nv0 - nv5) * s.db[4][5]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b6: f64 = (-(((nv0 - nv5) * s.db[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b7: f64 = (-(((nv0 - nv5) * s.db[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b8: f64 = (-(((nv0 - nv5) * s.db[4][8]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b9: f64 = (-(((nv0 - nv5) * s.db[4][9]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b10: f64 = (-(((nv0 - nv5) * s.db[4][10]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b11: f64 = (-(((nv0 - nv5) * s.db[4][11]) / (s.v[4] * s.v[4])));
        (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n1, eq23_e1141_d_n2, eq23_e1141_d_n3, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n12, eq23_e1141_d_n13, eq23_e1141_d_n14, eq23_e1141_d_n15, eq23_e1141_d_n16, eq23_e1141_d_n17, eq23_e1141_d_b0, eq23_e1141_d_b1, eq23_e1141_d_b2, eq23_e1141_d_b3, eq23_e1141_d_b4, eq23_e1141_d_b5, eq23_e1141_d_b6, eq23_e1141_d_b7, eq23_e1141_d_b8, eq23_e1141_d_b9, eq23_e1141_d_b10, eq23_e1141_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1143;
        let eq23_node_derivatives: [f64; 18] = [eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17];
        let eq23_branch_derivatives: [f64; 12] = [eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8, eq23_e1143_d_b9, eq23_e1143_d_b10, eq23_e1143_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1154, eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17, eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8, eq25_e1154_d_b9, eq25_e1154_d_b10, eq25_e1154_d_b11,) = {
    if (s.v[76] != 0.0) {
        let eq25_e1152: f64 = ((nv7 - nv2) / s.v[5]);
        let eq25_e1152_d_n0: f64 = (-(((nv7 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n1: f64 = (-(((nv7 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n2: f64 = (((-s.v[5]) - ((nv7 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n3: f64 = (-(((nv7 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n4: f64 = (-(((nv7 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n5: f64 = (-(((nv7 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n6: f64 = (-(((nv7 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n7: f64 = ((s.v[5] - ((nv7 - nv2) * s.dn[5][7])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n8: f64 = (-(((nv7 - nv2) * s.dn[5][8]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n9: f64 = (-(((nv7 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n10: f64 = (-(((nv7 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n11: f64 = (-(((nv7 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n12: f64 = (-(((nv7 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n13: f64 = (-(((nv7 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n14: f64 = (-(((nv7 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n15: f64 = (-(((nv7 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n16: f64 = (-(((nv7 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n17: f64 = (-(((nv7 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b0: f64 = (-(((nv7 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b1: f64 = (-(((nv7 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b2: f64 = (-(((nv7 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b3: f64 = (-(((nv7 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b4: f64 = (-(((nv7 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b5: f64 = (-(((nv7 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b6: f64 = (-(((nv7 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b7: f64 = (-(((nv7 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b8: f64 = (-(((nv7 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b9: f64 = (-(((nv7 - nv2) * s.db[5][9]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b10: f64 = (-(((nv7 - nv2) * s.db[5][10]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b11: f64 = (-(((nv7 - nv2) * s.db[5][11]) / (s.v[5] * s.v[5])));
        (eq25_e1152, eq25_e1152_d_n0, eq25_e1152_d_n1, eq25_e1152_d_n2, eq25_e1152_d_n3, eq25_e1152_d_n4, eq25_e1152_d_n5, eq25_e1152_d_n6, eq25_e1152_d_n7, eq25_e1152_d_n8, eq25_e1152_d_n9, eq25_e1152_d_n10, eq25_e1152_d_n11, eq25_e1152_d_n12, eq25_e1152_d_n13, eq25_e1152_d_n14, eq25_e1152_d_n15, eq25_e1152_d_n16, eq25_e1152_d_n17, eq25_e1152_d_b0, eq25_e1152_d_b1, eq25_e1152_d_b2, eq25_e1152_d_b3, eq25_e1152_d_b4, eq25_e1152_d_b5, eq25_e1152_d_b6, eq25_e1152_d_b7, eq25_e1152_d_b8, eq25_e1152_d_b9, eq25_e1152_d_b10, eq25_e1152_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1154;
        let eq25_node_derivatives: [f64; 18] = [eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17];
        let eq25_branch_derivatives: [f64; 12] = [eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8, eq25_e1154_d_b9, eq25_e1154_d_b10, eq25_e1154_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1163_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq27_e1163_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq27_e1163_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq27_e1164: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq27_e1163);
        let eq27_e1165: f64 = (p.p87 * eq27_e1164);
        let eq27_e1165_d_n0: f64 = (p.p87 * (eq27_e1163_d_n0 * ddt_scale));
        let eq27_e1165_d_n1: f64 = (p.p87 * (eq27_e1163_d_n1 * ddt_scale));
        let eq27_e1165_d_n2: f64 = (p.p87 * (eq27_e1163_d_n2 * ddt_scale));
        let eq27_e1165_d_n3: f64 = (p.p87 * (eq27_e1163_d_n3 * ddt_scale));
        let eq27_e1165_d_n4: f64 = (p.p87 * (eq27_e1163_d_n4 * ddt_scale));
        let eq27_e1165_d_n5: f64 = (p.p87 * (eq27_e1163_d_n5 * ddt_scale));
        let eq27_e1165_d_n6: f64 = (p.p87 * (eq27_e1163_d_n6 * ddt_scale));
        let eq27_e1165_d_n7: f64 = (p.p87 * (eq27_e1163_d_n7 * ddt_scale));
        let eq27_e1165_d_n8: f64 = (p.p87 * (eq27_e1163_d_n8 * ddt_scale));
        let eq27_e1165_d_n9: f64 = (p.p87 * (eq27_e1163_d_n9 * ddt_scale));
        let eq27_e1165_d_n10: f64 = (p.p87 * (eq27_e1163_d_n10 * ddt_scale));
        let eq27_e1165_d_n11: f64 = (p.p87 * (eq27_e1163_d_n11 * ddt_scale));
        let eq27_e1165_d_n12: f64 = (p.p87 * (eq27_e1163_d_n12 * ddt_scale));
        let eq27_e1165_d_n13: f64 = (p.p87 * (eq27_e1163_d_n13 * ddt_scale));
        let eq27_e1165_d_n14: f64 = (p.p87 * (eq27_e1163_d_n14 * ddt_scale));
        let eq27_e1165_d_n15: f64 = (p.p87 * (eq27_e1163_d_n15 * ddt_scale));
        let eq27_e1165_d_n16: f64 = (p.p87 * (eq27_e1163_d_n16 * ddt_scale));
        let eq27_e1165_d_n17: f64 = (p.p87 * (eq27_e1163_d_n17 * ddt_scale));
        let eq27_e1165_d_b0: f64 = (p.p87 * (eq27_e1163_d_b0 * ddt_scale));
        let eq27_e1165_d_b1: f64 = (p.p87 * (eq27_e1163_d_b1 * ddt_scale));
        let eq27_e1165_d_b2: f64 = (p.p87 * (eq27_e1163_d_b2 * ddt_scale));
        let eq27_e1165_d_b3: f64 = (p.p87 * (eq27_e1163_d_b3 * ddt_scale));
        let eq27_e1165_d_b4: f64 = (p.p87 * (eq27_e1163_d_b4 * ddt_scale));
        let eq27_e1165_d_b5: f64 = (p.p87 * (eq27_e1163_d_b5 * ddt_scale));
        let eq27_e1165_d_b6: f64 = (p.p87 * (eq27_e1163_d_b6 * ddt_scale));
        let eq27_e1165_d_b7: f64 = (p.p87 * (eq27_e1163_d_b7 * ddt_scale));
        let eq27_e1165_d_b8: f64 = (p.p87 * (eq27_e1163_d_b8 * ddt_scale));
        let eq27_e1165_d_b9: f64 = (p.p87 * (eq27_e1163_d_b9 * ddt_scale));
        let eq27_e1165_d_b10: f64 = (p.p87 * (eq27_e1163_d_b10 * ddt_scale));
        let eq27_e1165_d_b11: f64 = (p.p87 * (eq27_e1163_d_b11 * ddt_scale));
        let eq27_value: f64 = eq27_e1165;
        let eq27_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, eq27_e1165_d_n1, eq27_e1165_d_n2, eq27_e1165_d_n3, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, eq27_e1165_d_n14, eq27_e1165_d_n15, eq27_e1165_d_n16, eq27_e1165_d_n17];
        let eq27_branch_derivatives: [f64; 12] = [eq27_e1165_d_b0, eq27_e1165_d_b1, eq27_e1165_d_b2, eq27_e1165_d_b3, eq27_e1165_d_b4, eq27_e1165_d_b5, eq27_e1165_d_b6, eq27_e1165_d_b7, eq27_e1165_d_b8, eq27_e1165_d_b9, eq27_e1165_d_b10, eq27_e1165_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1169_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq28_e1169_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq28_e1169_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq28_e1170: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq28_e1169);
        let eq28_e1171: f64 = (p.p87 * eq28_e1170);
        let eq28_e1171_d_n0: f64 = (p.p87 * (eq28_e1169_d_n0 * ddt_scale));
        let eq28_e1171_d_n1: f64 = (p.p87 * (eq28_e1169_d_n1 * ddt_scale));
        let eq28_e1171_d_n2: f64 = (p.p87 * (eq28_e1169_d_n2 * ddt_scale));
        let eq28_e1171_d_n3: f64 = (p.p87 * (eq28_e1169_d_n3 * ddt_scale));
        let eq28_e1171_d_n4: f64 = (p.p87 * (eq28_e1169_d_n4 * ddt_scale));
        let eq28_e1171_d_n5: f64 = (p.p87 * (eq28_e1169_d_n5 * ddt_scale));
        let eq28_e1171_d_n6: f64 = (p.p87 * (eq28_e1169_d_n6 * ddt_scale));
        let eq28_e1171_d_n7: f64 = (p.p87 * (eq28_e1169_d_n7 * ddt_scale));
        let eq28_e1171_d_n8: f64 = (p.p87 * (eq28_e1169_d_n8 * ddt_scale));
        let eq28_e1171_d_n9: f64 = (p.p87 * (eq28_e1169_d_n9 * ddt_scale));
        let eq28_e1171_d_n10: f64 = (p.p87 * (eq28_e1169_d_n10 * ddt_scale));
        let eq28_e1171_d_n11: f64 = (p.p87 * (eq28_e1169_d_n11 * ddt_scale));
        let eq28_e1171_d_n12: f64 = (p.p87 * (eq28_e1169_d_n12 * ddt_scale));
        let eq28_e1171_d_n13: f64 = (p.p87 * (eq28_e1169_d_n13 * ddt_scale));
        let eq28_e1171_d_n14: f64 = (p.p87 * (eq28_e1169_d_n14 * ddt_scale));
        let eq28_e1171_d_n15: f64 = (p.p87 * (eq28_e1169_d_n15 * ddt_scale));
        let eq28_e1171_d_n16: f64 = (p.p87 * (eq28_e1169_d_n16 * ddt_scale));
        let eq28_e1171_d_n17: f64 = (p.p87 * (eq28_e1169_d_n17 * ddt_scale));
        let eq28_e1171_d_b0: f64 = (p.p87 * (eq28_e1169_d_b0 * ddt_scale));
        let eq28_e1171_d_b1: f64 = (p.p87 * (eq28_e1169_d_b1 * ddt_scale));
        let eq28_e1171_d_b2: f64 = (p.p87 * (eq28_e1169_d_b2 * ddt_scale));
        let eq28_e1171_d_b3: f64 = (p.p87 * (eq28_e1169_d_b3 * ddt_scale));
        let eq28_e1171_d_b4: f64 = (p.p87 * (eq28_e1169_d_b4 * ddt_scale));
        let eq28_e1171_d_b5: f64 = (p.p87 * (eq28_e1169_d_b5 * ddt_scale));
        let eq28_e1171_d_b6: f64 = (p.p87 * (eq28_e1169_d_b6 * ddt_scale));
        let eq28_e1171_d_b7: f64 = (p.p87 * (eq28_e1169_d_b7 * ddt_scale));
        let eq28_e1171_d_b8: f64 = (p.p87 * (eq28_e1169_d_b8 * ddt_scale));
        let eq28_e1171_d_b9: f64 = (p.p87 * (eq28_e1169_d_b9 * ddt_scale));
        let eq28_e1171_d_b10: f64 = (p.p87 * (eq28_e1169_d_b10 * ddt_scale));
        let eq28_e1171_d_b11: f64 = (p.p87 * (eq28_e1169_d_b11 * ddt_scale));
        let eq28_value: f64 = eq28_e1171;
        let eq28_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, eq28_e1171_d_n1, eq28_e1171_d_n2, eq28_e1171_d_n3, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n12, eq28_e1171_d_n13, eq28_e1171_d_n14, eq28_e1171_d_n15, eq28_e1171_d_n16, eq28_e1171_d_n17];
        let eq28_branch_derivatives: [f64; 12] = [eq28_e1171_d_b0, eq28_e1171_d_b1, eq28_e1171_d_b2, eq28_e1171_d_b3, eq28_e1171_d_b4, eq28_e1171_d_b5, eq28_e1171_d_b6, eq28_e1171_d_b7, eq28_e1171_d_b8, eq28_e1171_d_b9, eq28_e1171_d_b10, eq28_e1171_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1176_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq29_e1176_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq29_e1176_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1178_d_b9: f64 = (eq29_e1176_d_b9 + s.db[752][9]);
        let eq29_e1178_d_b10: f64 = (eq29_e1176_d_b10 + s.db[752][10]);
        let eq29_e1178_d_b11: f64 = (eq29_e1176_d_b11 + s.db[752][11]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1179_d_b9: f64 = (s.db[20][9] - eq29_e1178_d_b9);
        let eq29_e1179_d_b10: f64 = (s.db[20][10] - eq29_e1178_d_b10);
        let eq29_e1179_d_b11: f64 = (s.db[20][11] - eq29_e1178_d_b11);
        let eq29_e1180: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq29_e1179);
        let eq29_e1181: f64 = (p.p87 * eq29_e1180);
        let eq29_e1181_d_n0: f64 = (p.p87 * (eq29_e1179_d_n0 * ddt_scale));
        let eq29_e1181_d_n1: f64 = (p.p87 * (eq29_e1179_d_n1 * ddt_scale));
        let eq29_e1181_d_n2: f64 = (p.p87 * (eq29_e1179_d_n2 * ddt_scale));
        let eq29_e1181_d_n3: f64 = (p.p87 * (eq29_e1179_d_n3 * ddt_scale));
        let eq29_e1181_d_n4: f64 = (p.p87 * (eq29_e1179_d_n4 * ddt_scale));
        let eq29_e1181_d_n5: f64 = (p.p87 * (eq29_e1179_d_n5 * ddt_scale));
        let eq29_e1181_d_n6: f64 = (p.p87 * (eq29_e1179_d_n6 * ddt_scale));
        let eq29_e1181_d_n7: f64 = (p.p87 * (eq29_e1179_d_n7 * ddt_scale));
        let eq29_e1181_d_n8: f64 = (p.p87 * (eq29_e1179_d_n8 * ddt_scale));
        let eq29_e1181_d_n9: f64 = (p.p87 * (eq29_e1179_d_n9 * ddt_scale));
        let eq29_e1181_d_n10: f64 = (p.p87 * (eq29_e1179_d_n10 * ddt_scale));
        let eq29_e1181_d_n11: f64 = (p.p87 * (eq29_e1179_d_n11 * ddt_scale));
        let eq29_e1181_d_n12: f64 = (p.p87 * (eq29_e1179_d_n12 * ddt_scale));
        let eq29_e1181_d_n13: f64 = (p.p87 * (eq29_e1179_d_n13 * ddt_scale));
        let eq29_e1181_d_n14: f64 = (p.p87 * (eq29_e1179_d_n14 * ddt_scale));
        let eq29_e1181_d_n15: f64 = (p.p87 * (eq29_e1179_d_n15 * ddt_scale));
        let eq29_e1181_d_n16: f64 = (p.p87 * (eq29_e1179_d_n16 * ddt_scale));
        let eq29_e1181_d_n17: f64 = (p.p87 * (eq29_e1179_d_n17 * ddt_scale));
        let eq29_e1181_d_b0: f64 = (p.p87 * (eq29_e1179_d_b0 * ddt_scale));
        let eq29_e1181_d_b1: f64 = (p.p87 * (eq29_e1179_d_b1 * ddt_scale));
        let eq29_e1181_d_b2: f64 = (p.p87 * (eq29_e1179_d_b2 * ddt_scale));
        let eq29_e1181_d_b3: f64 = (p.p87 * (eq29_e1179_d_b3 * ddt_scale));
        let eq29_e1181_d_b4: f64 = (p.p87 * (eq29_e1179_d_b4 * ddt_scale));
        let eq29_e1181_d_b5: f64 = (p.p87 * (eq29_e1179_d_b5 * ddt_scale));
        let eq29_e1181_d_b6: f64 = (p.p87 * (eq29_e1179_d_b6 * ddt_scale));
        let eq29_e1181_d_b7: f64 = (p.p87 * (eq29_e1179_d_b7 * ddt_scale));
        let eq29_e1181_d_b8: f64 = (p.p87 * (eq29_e1179_d_b8 * ddt_scale));
        let eq29_e1181_d_b9: f64 = (p.p87 * (eq29_e1179_d_b9 * ddt_scale));
        let eq29_e1181_d_b10: f64 = (p.p87 * (eq29_e1179_d_b10 * ddt_scale));
        let eq29_e1181_d_b11: f64 = (p.p87 * (eq29_e1179_d_b11 * ddt_scale));
        let eq29_value: f64 = eq29_e1181;
        let eq29_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, eq29_e1181_d_n1, eq29_e1181_d_n2, eq29_e1181_d_n3, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, eq29_e1181_d_n14, eq29_e1181_d_n15, eq29_e1181_d_n16, eq29_e1181_d_n17];
        let eq29_branch_derivatives: [f64; 12] = [eq29_e1181_d_b0, eq29_e1181_d_b1, eq29_e1181_d_b2, eq29_e1181_d_b3, eq29_e1181_d_b4, eq29_e1181_d_b5, eq29_e1181_d_b6, eq29_e1181_d_b7, eq29_e1181_d_b8, eq29_e1181_d_b9, eq29_e1181_d_b10, eq29_e1181_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[743]);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * (s.dn[743][0] * ddt_scale));
        let eq30_e1185_d_n1: f64 = (p.p87 * (s.dn[743][1] * ddt_scale));
        let eq30_e1185_d_n2: f64 = (p.p87 * (s.dn[743][2] * ddt_scale));
        let eq30_e1185_d_n3: f64 = (p.p87 * (s.dn[743][3] * ddt_scale));
        let eq30_e1185_d_n4: f64 = (p.p87 * (s.dn[743][4] * ddt_scale));
        let eq30_e1185_d_n5: f64 = (p.p87 * (s.dn[743][5] * ddt_scale));
        let eq30_e1185_d_n6: f64 = (p.p87 * (s.dn[743][6] * ddt_scale));
        let eq30_e1185_d_n7: f64 = (p.p87 * (s.dn[743][7] * ddt_scale));
        let eq30_e1185_d_n8: f64 = (p.p87 * (s.dn[743][8] * ddt_scale));
        let eq30_e1185_d_n9: f64 = (p.p87 * (s.dn[743][9] * ddt_scale));
        let eq30_e1185_d_n10: f64 = (p.p87 * (s.dn[743][10] * ddt_scale));
        let eq30_e1185_d_n11: f64 = (p.p87 * (s.dn[743][11] * ddt_scale));
        let eq30_e1185_d_n12: f64 = (p.p87 * (s.dn[743][12] * ddt_scale));
        let eq30_e1185_d_n13: f64 = (p.p87 * (s.dn[743][13] * ddt_scale));
        let eq30_e1185_d_n14: f64 = (p.p87 * (s.dn[743][14] * ddt_scale));
        let eq30_e1185_d_n15: f64 = (p.p87 * (s.dn[743][15] * ddt_scale));
        let eq30_e1185_d_n16: f64 = (p.p87 * (s.dn[743][16] * ddt_scale));
        let eq30_e1185_d_n17: f64 = (p.p87 * (s.dn[743][17] * ddt_scale));
        let eq30_e1185_d_b0: f64 = (p.p87 * (s.db[743][0] * ddt_scale));
        let eq30_e1185_d_b1: f64 = (p.p87 * (s.db[743][1] * ddt_scale));
        let eq30_e1185_d_b2: f64 = (p.p87 * (s.db[743][2] * ddt_scale));
        let eq30_e1185_d_b3: f64 = (p.p87 * (s.db[743][3] * ddt_scale));
        let eq30_e1185_d_b4: f64 = (p.p87 * (s.db[743][4] * ddt_scale));
        let eq30_e1185_d_b5: f64 = (p.p87 * (s.db[743][5] * ddt_scale));
        let eq30_e1185_d_b6: f64 = (p.p87 * (s.db[743][6] * ddt_scale));
        let eq30_e1185_d_b7: f64 = (p.p87 * (s.db[743][7] * ddt_scale));
        let eq30_e1185_d_b8: f64 = (p.p87 * (s.db[743][8] * ddt_scale));
        let eq30_e1185_d_b9: f64 = (p.p87 * (s.db[743][9] * ddt_scale));
        let eq30_e1185_d_b10: f64 = (p.p87 * (s.db[743][10] * ddt_scale));
        let eq30_e1185_d_b11: f64 = (p.p87 * (s.db[743][11] * ddt_scale));
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, eq30_e1185_d_n1, eq30_e1185_d_n2, eq30_e1185_d_n3, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, eq30_e1185_d_n15, eq30_e1185_d_n16, eq30_e1185_d_n17];
        let eq30_branch_derivatives: [f64; 12] = [eq30_e1185_d_b0, eq30_e1185_d_b1, eq30_e1185_d_b2, eq30_e1185_d_b3, eq30_e1185_d_b4, eq30_e1185_d_b5, eq30_e1185_d_b6, eq30_e1185_d_b7, eq30_e1185_d_b8, eq30_e1185_d_b9, eq30_e1185_d_b10, eq30_e1185_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[742]);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * (s.dn[742][0] * ddt_scale));
        let eq31_e1189_d_n1: f64 = (p.p87 * (s.dn[742][1] * ddt_scale));
        let eq31_e1189_d_n2: f64 = (p.p87 * (s.dn[742][2] * ddt_scale));
        let eq31_e1189_d_n3: f64 = (p.p87 * (s.dn[742][3] * ddt_scale));
        let eq31_e1189_d_n4: f64 = (p.p87 * (s.dn[742][4] * ddt_scale));
        let eq31_e1189_d_n5: f64 = (p.p87 * (s.dn[742][5] * ddt_scale));
        let eq31_e1189_d_n6: f64 = (p.p87 * (s.dn[742][6] * ddt_scale));
        let eq31_e1189_d_n7: f64 = (p.p87 * (s.dn[742][7] * ddt_scale));
        let eq31_e1189_d_n8: f64 = (p.p87 * (s.dn[742][8] * ddt_scale));
        let eq31_e1189_d_n9: f64 = (p.p87 * (s.dn[742][9] * ddt_scale));
        let eq31_e1189_d_n10: f64 = (p.p87 * (s.dn[742][10] * ddt_scale));
        let eq31_e1189_d_n11: f64 = (p.p87 * (s.dn[742][11] * ddt_scale));
        let eq31_e1189_d_n12: f64 = (p.p87 * (s.dn[742][12] * ddt_scale));
        let eq31_e1189_d_n13: f64 = (p.p87 * (s.dn[742][13] * ddt_scale));
        let eq31_e1189_d_n14: f64 = (p.p87 * (s.dn[742][14] * ddt_scale));
        let eq31_e1189_d_n15: f64 = (p.p87 * (s.dn[742][15] * ddt_scale));
        let eq31_e1189_d_n16: f64 = (p.p87 * (s.dn[742][16] * ddt_scale));
        let eq31_e1189_d_n17: f64 = (p.p87 * (s.dn[742][17] * ddt_scale));
        let eq31_e1189_d_b0: f64 = (p.p87 * (s.db[742][0] * ddt_scale));
        let eq31_e1189_d_b1: f64 = (p.p87 * (s.db[742][1] * ddt_scale));
        let eq31_e1189_d_b2: f64 = (p.p87 * (s.db[742][2] * ddt_scale));
        let eq31_e1189_d_b3: f64 = (p.p87 * (s.db[742][3] * ddt_scale));
        let eq31_e1189_d_b4: f64 = (p.p87 * (s.db[742][4] * ddt_scale));
        let eq31_e1189_d_b5: f64 = (p.p87 * (s.db[742][5] * ddt_scale));
        let eq31_e1189_d_b6: f64 = (p.p87 * (s.db[742][6] * ddt_scale));
        let eq31_e1189_d_b7: f64 = (p.p87 * (s.db[742][7] * ddt_scale));
        let eq31_e1189_d_b8: f64 = (p.p87 * (s.db[742][8] * ddt_scale));
        let eq31_e1189_d_b9: f64 = (p.p87 * (s.db[742][9] * ddt_scale));
        let eq31_e1189_d_b10: f64 = (p.p87 * (s.db[742][10] * ddt_scale));
        let eq31_e1189_d_b11: f64 = (p.p87 * (s.db[742][11] * ddt_scale));
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17];
        let eq31_branch_derivatives: [f64; 12] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8, eq31_e1189_d_b9, eq31_e1189_d_b10, eq31_e1189_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[744]);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * (s.dn[744][0] * ddt_scale));
        let eq32_e1193_d_n1: f64 = (p.p87 * (s.dn[744][1] * ddt_scale));
        let eq32_e1193_d_n2: f64 = (p.p87 * (s.dn[744][2] * ddt_scale));
        let eq32_e1193_d_n3: f64 = (p.p87 * (s.dn[744][3] * ddt_scale));
        let eq32_e1193_d_n4: f64 = (p.p87 * (s.dn[744][4] * ddt_scale));
        let eq32_e1193_d_n5: f64 = (p.p87 * (s.dn[744][5] * ddt_scale));
        let eq32_e1193_d_n6: f64 = (p.p87 * (s.dn[744][6] * ddt_scale));
        let eq32_e1193_d_n7: f64 = (p.p87 * (s.dn[744][7] * ddt_scale));
        let eq32_e1193_d_n8: f64 = (p.p87 * (s.dn[744][8] * ddt_scale));
        let eq32_e1193_d_n9: f64 = (p.p87 * (s.dn[744][9] * ddt_scale));
        let eq32_e1193_d_n10: f64 = (p.p87 * (s.dn[744][10] * ddt_scale));
        let eq32_e1193_d_n11: f64 = (p.p87 * (s.dn[744][11] * ddt_scale));
        let eq32_e1193_d_n12: f64 = (p.p87 * (s.dn[744][12] * ddt_scale));
        let eq32_e1193_d_n13: f64 = (p.p87 * (s.dn[744][13] * ddt_scale));
        let eq32_e1193_d_n14: f64 = (p.p87 * (s.dn[744][14] * ddt_scale));
        let eq32_e1193_d_n15: f64 = (p.p87 * (s.dn[744][15] * ddt_scale));
        let eq32_e1193_d_n16: f64 = (p.p87 * (s.dn[744][16] * ddt_scale));
        let eq32_e1193_d_n17: f64 = (p.p87 * (s.dn[744][17] * ddt_scale));
        let eq32_e1193_d_b0: f64 = (p.p87 * (s.db[744][0] * ddt_scale));
        let eq32_e1193_d_b1: f64 = (p.p87 * (s.db[744][1] * ddt_scale));
        let eq32_e1193_d_b2: f64 = (p.p87 * (s.db[744][2] * ddt_scale));
        let eq32_e1193_d_b3: f64 = (p.p87 * (s.db[744][3] * ddt_scale));
        let eq32_e1193_d_b4: f64 = (p.p87 * (s.db[744][4] * ddt_scale));
        let eq32_e1193_d_b5: f64 = (p.p87 * (s.db[744][5] * ddt_scale));
        let eq32_e1193_d_b6: f64 = (p.p87 * (s.db[744][6] * ddt_scale));
        let eq32_e1193_d_b7: f64 = (p.p87 * (s.db[744][7] * ddt_scale));
        let eq32_e1193_d_b8: f64 = (p.p87 * (s.db[744][8] * ddt_scale));
        let eq32_e1193_d_b9: f64 = (p.p87 * (s.db[744][9] * ddt_scale));
        let eq32_e1193_d_b10: f64 = (p.p87 * (s.db[744][10] * ddt_scale));
        let eq32_e1193_d_b11: f64 = (p.p87 * (s.db[744][11] * ddt_scale));
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17];
        let eq32_branch_derivatives: [f64; 12] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8, eq32_e1193_d_b9, eq32_e1193_d_b10, eq32_e1193_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[299]);
        let eq33_e1198: f64 = (eq33_e1195 * eq33_e1197);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * (s.dn[299][0] * ddt_scale));
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * (s.dn[299][1] * ddt_scale));
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * (s.dn[299][2] * ddt_scale));
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * (s.dn[299][3] * ddt_scale));
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * (s.dn[299][4] * ddt_scale));
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * (s.dn[299][5] * ddt_scale));
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * (s.dn[299][6] * ddt_scale));
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * (s.dn[299][7] * ddt_scale));
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * (s.dn[299][8] * ddt_scale));
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * (s.dn[299][9] * ddt_scale));
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * (s.dn[299][10] * ddt_scale));
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * (s.dn[299][11] * ddt_scale));
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * (s.dn[299][12] * ddt_scale));
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * (s.dn[299][13] * ddt_scale));
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * (s.dn[299][14] * ddt_scale));
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * (s.dn[299][15] * ddt_scale));
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * (s.dn[299][16] * ddt_scale));
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * (s.dn[299][17] * ddt_scale));
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * (s.db[299][0] * ddt_scale));
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * (s.db[299][1] * ddt_scale));
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * (s.db[299][2] * ddt_scale));
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * (s.db[299][3] * ddt_scale));
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * (s.db[299][4] * ddt_scale));
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * (s.db[299][5] * ddt_scale));
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * (s.db[299][6] * ddt_scale));
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * (s.db[299][7] * ddt_scale));
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * (s.db[299][8] * ddt_scale));
        let eq33_e1198_d_b9: f64 = (eq33_e1195 * (s.db[299][9] * ddt_scale));
        let eq33_e1198_d_b10: f64 = (eq33_e1195 * (s.db[299][10] * ddt_scale));
        let eq33_e1198_d_b11: f64 = (eq33_e1195 * (s.db[299][11] * ddt_scale));
        let eq33_value: f64 = eq33_e1198;
        let eq33_node_derivatives: [f64; 18] = [eq33_e1198_d_n0, eq33_e1198_d_n1, eq33_e1198_d_n2, eq33_e1198_d_n3, eq33_e1198_d_n4, eq33_e1198_d_n5, eq33_e1198_d_n6, eq33_e1198_d_n7, eq33_e1198_d_n8, eq33_e1198_d_n9, eq33_e1198_d_n10, eq33_e1198_d_n11, eq33_e1198_d_n12, eq33_e1198_d_n13, eq33_e1198_d_n14, eq33_e1198_d_n15, eq33_e1198_d_n16, eq33_e1198_d_n17];
        let eq33_branch_derivatives: [f64; 12] = [eq33_e1198_d_b0, eq33_e1198_d_b1, eq33_e1198_d_b2, eq33_e1198_d_b3, eq33_e1198_d_b4, eq33_e1198_d_b5, eq33_e1198_d_b6, eq33_e1198_d_b7, eq33_e1198_d_b8, eq33_e1198_d_b9, eq33_e1198_d_b10, eq33_e1198_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(0),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[301]);
        let eq34_e1203: f64 = (eq34_e1200 * eq34_e1202);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * (s.dn[301][0] * ddt_scale));
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * (s.dn[301][1] * ddt_scale));
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * (s.dn[301][2] * ddt_scale));
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * (s.dn[301][3] * ddt_scale));
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * (s.dn[301][4] * ddt_scale));
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * (s.dn[301][5] * ddt_scale));
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * (s.dn[301][6] * ddt_scale));
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * (s.dn[301][7] * ddt_scale));
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * (s.dn[301][8] * ddt_scale));
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * (s.dn[301][9] * ddt_scale));
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * (s.dn[301][10] * ddt_scale));
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * (s.dn[301][11] * ddt_scale));
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * (s.dn[301][12] * ddt_scale));
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * (s.dn[301][13] * ddt_scale));
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * (s.dn[301][14] * ddt_scale));
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * (s.dn[301][15] * ddt_scale));
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * (s.dn[301][16] * ddt_scale));
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * (s.dn[301][17] * ddt_scale));
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * (s.db[301][0] * ddt_scale));
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * (s.db[301][1] * ddt_scale));
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * (s.db[301][2] * ddt_scale));
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * (s.db[301][3] * ddt_scale));
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * (s.db[301][4] * ddt_scale));
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * (s.db[301][5] * ddt_scale));
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * (s.db[301][6] * ddt_scale));
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * (s.db[301][7] * ddt_scale));
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * (s.db[301][8] * ddt_scale));
        let eq34_e1203_d_b9: f64 = (eq34_e1200 * (s.db[301][9] * ddt_scale));
        let eq34_e1203_d_b10: f64 = (eq34_e1200 * (s.db[301][10] * ddt_scale));
        let eq34_e1203_d_b11: f64 = (eq34_e1200 * (s.db[301][11] * ddt_scale));
        let eq34_value: f64 = eq34_e1203;
        let eq34_node_derivatives: [f64; 18] = [eq34_e1203_d_n0, eq34_e1203_d_n1, eq34_e1203_d_n2, eq34_e1203_d_n3, eq34_e1203_d_n4, eq34_e1203_d_n5, eq34_e1203_d_n6, eq34_e1203_d_n7, eq34_e1203_d_n8, eq34_e1203_d_n9, eq34_e1203_d_n10, eq34_e1203_d_n11, eq34_e1203_d_n12, eq34_e1203_d_n13, eq34_e1203_d_n14, eq34_e1203_d_n15, eq34_e1203_d_n16, eq34_e1203_d_n17];
        let eq34_branch_derivatives: [f64; 12] = [eq34_e1203_d_b0, eq34_e1203_d_b1, eq34_e1203_d_b2, eq34_e1203_d_b3, eq34_e1203_d_b4, eq34_e1203_d_b5, eq34_e1203_d_b6, eq34_e1203_d_b7, eq34_e1203_d_b8, eq34_e1203_d_b9, eq34_e1203_d_b10, eq34_e1203_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq39_e1229: f64 = (s.v[951] * (nv14 - 0.0));
        let eq39_e1229_d_n0: f64 = (s.dn[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_n1: f64 = (s.dn[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_n2: f64 = (s.dn[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_n3: f64 = (s.dn[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_n4: f64 = (s.dn[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_n5: f64 = (s.dn[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_n6: f64 = (s.dn[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_n7: f64 = (s.dn[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_n8: f64 = (s.dn[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_n9: f64 = (s.dn[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_n10: f64 = (s.dn[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_n11: f64 = (s.dn[951][11] * (nv14 - 0.0));
        let eq39_e1229_d_n12: f64 = (s.dn[951][12] * (nv14 - 0.0));
        let eq39_e1229_d_n13: f64 = (s.dn[951][13] * (nv14 - 0.0));
        let eq39_e1229_d_n14: f64 = ((s.dn[951][14] * (nv14 - 0.0)) + s.v[951]);
        let eq39_e1229_d_n15: f64 = (s.dn[951][15] * (nv14 - 0.0));
        let eq39_e1229_d_n16: f64 = (s.dn[951][16] * (nv14 - 0.0));
        let eq39_e1229_d_n17: f64 = (s.dn[951][17] * (nv14 - 0.0));
        let eq39_e1229_d_b0: f64 = (s.db[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_b1: f64 = (s.db[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_b2: f64 = (s.db[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_b3: f64 = (s.db[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_b4: f64 = (s.db[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_b5: f64 = (s.db[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_b6: f64 = (s.db[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_b7: f64 = (s.db[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_b8: f64 = (s.db[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_b9: f64 = (s.db[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_b10: f64 = (s.db[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_b11: f64 = (s.db[951][11] * (nv14 - 0.0));
        let eq39_value: f64 = eq39_e1229;
        let eq39_node_derivatives: [f64; 18] = [eq39_e1229_d_n0, eq39_e1229_d_n1, eq39_e1229_d_n2, eq39_e1229_d_n3, eq39_e1229_d_n4, eq39_e1229_d_n5, eq39_e1229_d_n6, eq39_e1229_d_n7, eq39_e1229_d_n8, eq39_e1229_d_n9, eq39_e1229_d_n10, eq39_e1229_d_n11, eq39_e1229_d_n12, eq39_e1229_d_n13, eq39_e1229_d_n14, eq39_e1229_d_n15, eq39_e1229_d_n16, eq39_e1229_d_n17];
        let eq39_branch_derivatives: [f64; 12] = [eq39_e1229_d_b0, eq39_e1229_d_b1, eq39_e1229_d_b2, eq39_e1229_d_b3, eq39_e1229_d_b4, eq39_e1229_d_b5, eq39_e1229_d_b6, eq39_e1229_d_b7, eq39_e1229_d_b8, eq39_e1229_d_b9, eq39_e1229_d_b10, eq39_e1229_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq40_e1232);
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 18] = [(eq40_e1232_d_n0 * ddt_scale), (eq40_e1232_d_n1 * ddt_scale), (eq40_e1232_d_n2 * ddt_scale), (eq40_e1232_d_n3 * ddt_scale), (eq40_e1232_d_n4 * ddt_scale), (eq40_e1232_d_n5 * ddt_scale), (eq40_e1232_d_n6 * ddt_scale), (eq40_e1232_d_n7 * ddt_scale), (eq40_e1232_d_n8 * ddt_scale), (eq40_e1232_d_n9 * ddt_scale), (eq40_e1232_d_n10 * ddt_scale), (eq40_e1232_d_n11 * ddt_scale), (eq40_e1232_d_n12 * ddt_scale), (eq40_e1232_d_n13 * ddt_scale), (eq40_e1232_d_n14 * ddt_scale), (eq40_e1232_d_n15 * ddt_scale), (eq40_e1232_d_n16 * ddt_scale), (eq40_e1232_d_n17 * ddt_scale)];
        let eq40_branch_derivatives: [f64; 12] = [(eq40_e1232_d_b0 * ddt_scale), (eq40_e1232_d_b1 * ddt_scale), (eq40_e1232_d_b2 * ddt_scale), (eq40_e1232_d_b3 * ddt_scale), (eq40_e1232_d_b4 * ddt_scale), (eq40_e1232_d_b5 * ddt_scale), (eq40_e1232_d_b6 * ddt_scale), (eq40_e1232_d_b7 * ddt_scale), (eq40_e1232_d_b8 * ddt_scale), (eq40_e1232_d_b9 * ddt_scale), (eq40_e1232_d_b10 * ddt_scale), (eq40_e1232_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq41_e1236);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 18] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n1 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n3 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n11 * ddt_scale), (eq41_e1236_d_n12 * ddt_scale), (eq41_e1236_d_n13 * ddt_scale), (eq41_e1236_d_n14 * ddt_scale), (eq41_e1236_d_n15 * ddt_scale), (eq41_e1236_d_n16 * ddt_scale), (eq41_e1236_d_n17 * ddt_scale)];
        let eq41_branch_derivatives: [f64; 12] = [(eq41_e1236_d_b0 * ddt_scale), (eq41_e1236_d_b1 * ddt_scale), (eq41_e1236_d_b2 * ddt_scale), (eq41_e1236_d_b3 * ddt_scale), (eq41_e1236_d_b4 * ddt_scale), (eq41_e1236_d_b5 * ddt_scale), (eq41_e1236_d_b6 * ddt_scale), (eq41_e1236_d_b7 * ddt_scale), (eq41_e1236_d_b8 * ddt_scale), (eq41_e1236_d_b9 * ddt_scale), (eq41_e1236_d_b10 * ddt_scale), (eq41_e1236_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1332, eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17, eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11,) = {
    if s.b[3409] {
        let eq56_e1330: f64 = (-s.v[802]);
        (eq56_e1330, (-s.dn[802][0]), (-s.dn[802][1]), (-s.dn[802][2]), (-s.dn[802][3]), (-s.dn[802][4]), (-s.dn[802][5]), (-s.dn[802][6]), (-s.dn[802][7]), (-s.dn[802][8]), (-s.dn[802][9]), (-s.dn[802][10]), (-s.dn[802][11]), (-s.dn[802][12]), (-s.dn[802][13]), (-s.dn[802][14]), (-s.dn[802][15]), (-s.dn[802][16]), (-s.dn[802][17]), (-s.db[802][0]), (-s.db[802][1]), (-s.db[802][2]), (-s.db[802][3]), (-s.db[802][4]), (-s.db[802][5]), (-s.db[802][6]), (-s.db[802][7]), (-s.db[802][8]), (-s.db[802][9]), (-s.db[802][10]), (-s.db[802][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1332;
        let eq56_node_derivatives: [f64; 18] = [eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17];
        let eq56_branch_derivatives: [f64; 12] = [eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
    }
}
