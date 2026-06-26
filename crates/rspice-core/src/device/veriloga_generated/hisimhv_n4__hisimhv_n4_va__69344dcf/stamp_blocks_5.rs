#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3018])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[3024] = (s.v[402] < s.v[403]);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3024]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[3025] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3024]) && s.b[3025]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3024]) && (!s.b[3025])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[3024]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3007, 404);
        }

        s.b[3026] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3026] = if s.b[3026] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3026]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3026])) {
            s.store_offset_ad(332, A::div_scaled_offset_numerator(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3027] = (s.v[116] >= 3.0);
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3027]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_ad(332, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_ad(332, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3027])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2(437, s.ad_value(402), -1.0, s.ad_value(397), -1.0, s.ad_value(212), 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3(116, s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3028] = (p.p33 > 0.0);
        s.v[3028] = if s.b[3028] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3029] = (p.p33 == 2.0);
        s.v[3029] = if s.b[3029] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(447, s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && (!s.b[3029])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.copy_ad(445, 116);
        }

        s.b[3030] = (p.p33 == 2.0);
        s.v[3030] = if s.b[3030] { 1.0 } else { 0.0 };

        s.b[3031] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3031] = if s.b[3031] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_add_scaled_inputs3(781, s.ad_value(445), 1.0, s.ad_value(446), (-1.0), s.ad_value(446), 0.2);
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

        s.b[3032] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3032] = if s.b[3032] { 1.0 } else { 0.0 };

        s.b[3033] = (2.0 == 1.0);
        s.v[3033] = if s.b[3033] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && s.b[3033]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3034] = (2.0 == 2.0);
        s.v[3034] = if s.b[3034] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && s.b[3034]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3035] = (2.0 == 4.0);
        s.v[3035] = if s.b[3035] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && (!s.b[3034])) && s.b[3035]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3036] = (2.0 == 8.0);
        s.v[3036] = if s.b[3036] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && (!s.b[3034])) && (!s.b[3035])) && s.b[3036]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign87910_loop_guard: usize = 0;
        while {
            let assign87910_cond_e133987: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign87910_cond_e133987 != 0.0
        } {
            assign87910_loop_guard += 1;
            assert!(assign87910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && (!s.b[3032])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3(116, s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && (!s.b[3031])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && (!s.b[3030])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3037] = (p.p33 == 1.0);
        s.v[3037] = if s.b[3037] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3038] = (s.v[411] > 0.0);
        s.v[3038] = if s.b[3038] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3038]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3039] = (s.v[336] < 0.0);
        s.v[3039] = if s.b[3039] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) && s.b[3039]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3040] = (s.v[336] < 0.0);
        s.v[3040] = if s.b[3040] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3040]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3041] = (s.v[333] < 60.0);
        s.v[3041] = if s.b[3041] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3041]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3041])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3042] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3042] = if s.b[3042] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3042]) {
            s.store_offset(3006, 3006, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3043] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3043] = if s.b[3043] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3043]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3043])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3044, 354, 3000);
        }

        s.b[3046] = (p.p33 == 2.0);
        s.v[3046] = if s.b[3046] { 1.0 } else { 0.0 };

        s.b[3047] = ((s.v[3044] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3047] = if s.b[3047] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
            s.store_add_scaled_inputs3(781, s.ad_value(3044), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1);
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

        s.b[3048] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3048] = if s.b[3048] { 1.0 } else { 0.0 };

        s.b[3049] = (2.0 == 1.0);
        s.v[3049] = if s.b[3049] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && s.b[3049]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3050] = (2.0 == 2.0);
        s.v[3050] = if s.b[3050] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && s.b[3050]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3051] = (2.0 == 4.0);
        s.v[3051] = if s.b[3051] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && (!s.b[3050])) && s.b[3051]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3052] = (2.0 == 8.0);
        s.v[3052] = if s.b[3052] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && (!s.b[3050])) && (!s.b[3051])) && s.b[3052]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign88720_loop_guard: usize = 0;
        while {
            let assign88720_cond_e135057: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign88720_cond_e135057 != 0.0
        } {
            assign88720_loop_guard += 1;
            assert!(assign88720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && (!s.b[3048])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3(335, s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && (!s.b[3047])) {
            s.copy_ad(335, 3044);
            s.store_scalar(334, 1.0);
        }

        s.b[3053] = (s.v[334] < 1.0);
        s.v[3053] = if s.b[3053] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3053]) {
            s.store_offset(3006, 3006, 2.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3046])) {
            if (s.v[3044] <= s.v[386]) {
                s.copy_ad(335, 3044);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3054] = (s.v[3044] >= s.v[386]);
        s.v[3054] = if s.b[3054] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3046])) && s.b[3054]) {
            s.store_offset(3006, 3006, 2.0);
        }

        s.b[3055] = (s.v[3006] >= 2.0);
        s.v[3055] = if s.b[3055] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) {
            s.copy_ad(3045, 404);
            s.store_mul(354, 335, 3000);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.b[3056] = (p.p33 == 2.0);
        s.v[3056] = if s.b[3056] { 1.0 } else { 0.0 };

        s.b[3057] = ((s.v[404] > (s.v[3045] - 0.1)) && (0.1 >= 0.0));
        s.v[3057] = if s.b[3057] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
            s.store_offset_sub(781, 404, 3045, 0.1);
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

        s.b[3058] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3058] = if s.b[3058] { 1.0 } else { 0.0 };

        s.b[3059] = (2.0 == 1.0);
        s.v[3059] = if s.b[3059] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3060] = (2.0 == 2.0);
        s.v[3060] = if s.b[3060] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && s.b[3060]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3061] = (2.0 == 4.0);
        s.v[3061] = if s.b[3061] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && (!s.b[3060])) && s.b[3061]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3062] = (2.0 == 8.0);
        s.v[3062] = if s.b[3062] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && (!s.b[3060])) && (!s.b[3061])) && s.b[3062]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign89170_loop_guard: usize = 0;
        while {
            let assign89170_cond_e135665: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign89170_cond_e135665 != 0.0
        } {
            assign89170_loop_guard += 1;
            assert!(assign89170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && (!s.b[3058])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_ad_lhs(404, A::offset(s.ad_value(3045), (-0.1)), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && (!s.b[3057])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && (!s.b[3057])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && (!s.b[3056])) {
            if (s.v[404] <= s.v[3045]) {
            } else {
                s.copy_ad(404, 3045);
            }
        }

    }

    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.copy_ad(3007, 404);
        }

        s.b[3063] = (p.p33 == 1.0);
        s.v[3063] = if s.b[3063] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3000)), s.ad_value(155)), 2.0);
        }

        s.b[3064] = (s.v[411] > 0.0);
        s.v[3064] = if s.b[3064] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3064]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3065] = (s.v[336] < 0.0);
        s.v[3065] = if s.b[3065] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) && s.b[3065]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3066] = (s.v[336] < 0.0);
        s.v[3066] = if s.b[3066] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3066]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign89530_loop_guard: usize = 0;
        while {
            let assign89530_cond_e136189: f64 = (s.v[421] + 1.0);
            let assign89530_cond_e136191: f64 = if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (s.v[97] <= assign89530_cond_e136189)) { 1.0 } else { 0.0 };
            assign89530_cond_e136191 != 0.0
        } {
            assign89530_loop_guard += 1;
            assert!(assign89530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3068] = (s.v[333] < 60.0);
            s.v[3068] = if s.b[3068] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3068]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3068])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3069] = (s.v[116] < 0.0);
            s.v[3069] = if s.b[3069] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3069]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3070] = (s.v[116] < 1e-6);
            s.v[3070] = if s.b[3070] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[3071] = (s.v[338] > 0.0);
            s.v[3071] = if s.b[3071] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4(338, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
            }
            s.b[3072] = (s.v[338] > 0.0);
            s.v[3072] = if s.b[3072] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3072]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3072])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3073] = (s.v[116] < 0.0);
            s.v[3073] = if s.b[3073] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3073]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3074] = (s.v[116] < 60.0);
            s.v[3074] = if s.b[3074] { 1.0 } else { 0.0 };
            s.b[3075] = (s.v[116] < 5e-5);
            s.v[3075] = if s.b[3075] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3074]) && s.b[3075]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3074]) && (!s.b[3075])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && (!s.b[3074])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3076] = (s.v[214] > 0.0);
            s.v[3076] = if s.b[3076] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3076]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && (!s.b[3076])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3077] = (s.v[79] == 1.0);
            s.v[3077] = if s.b[3077] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3077]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3078] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3078] = if s.b[3078] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) && s.b[3078]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_add(404, 404, 236);
            }
            s.b[3079] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3079] = if s.b[3079] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) && s.b[3079]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_mul(2998, 982, 223);
            s.store_mul(2999, 3000, 2998);
            s.store_offset_div(100, 2999, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3081] = (p.p33 == 4.0);
        s.v[3081] = if s.b[3081] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3007);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3000)), s.ad_value(155)), 2.0);
        }

        s.b[3082] = (s.v[411] > 0.0);
        s.v[3082] = if s.b[3082] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3082]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3083] = (s.v[336] < 0.0);
        s.v[3083] = if s.b[3083] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) && s.b[3083]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3084] = (s.v[336] < 0.0);
        s.v[3084] = if s.b[3084] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3084]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign89920_loop_guard: usize = 0;
        while {
            let assign89920_cond_e137844: f64 = (s.v[421] + 1.0);
            let assign89920_cond_e137846: f64 = if (((s.v[2621] != 0.0) && s.b[3081]) && (s.v[97] <= assign89920_cond_e137844)) { 1.0 } else { 0.0 };
            assign89920_cond_e137846 != 0.0
        } {
            assign89920_loop_guard += 1;
            assert!(assign89920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3086] = (s.v[333] < 60.0);
            s.v[3086] = if s.b[3086] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3086]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3086])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3087] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[3087] = if s.b[3087] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3087]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(3008, 334, 336);
                s.store_mul_add_scaled_product_rhs(3009, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3087])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4(3008, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
                s.store_mul_sub_ad_rhs(3009, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[3088] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[3088] = if s.b[3088] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3088]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3089] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[3089] = if s.b[3089] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3088])) && s.b[3089]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3088])) && (!s.b[3089])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3090] = (s.v[214] > 0.0);
            s.v[3090] = if s.b[3090] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3090]) {
                s.store_sqrt_add(216, 3008, 214);
                s.store_div_scaled_inputs2(217, s.ad_value(3009), 0.5, s.ad_value(215), 0.5, s.ad_value(216), 1.0);
            }
            s.b[3091] = (s.v[3008] > 0.0);
            s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3090])) && s.b[3091]) {
                s.store_sqrt(216, 3008);
                s.store_div_scaled_inputs(217, s.ad_value(3009), 0.5, s.ad_value(216), 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3090])) && (!s.b[3091])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3092] = (s.v[79] > 0.0);
            s.v[3092] = if s.b[3092] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3092]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3093] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3093] = if s.b[3093] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) && s.b[3093]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_add(404, 404, 236);
            }
            s.b[3094] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3094] = if s.b[3094] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) && s.b[3094]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            if (s.v[3008] >= 0.0) {
                s.store_scaled_sqrt(223, 3008, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_mul(2998, 982, 223);
            s.store_mul(2999, 3000, 2998);
            s.store_offset_div(100, 2999, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[3096] = (s.v[407] < 0.0);
        s.v[3096] = if s.b[3096] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3096]) {
            s.store_neg(407, 407);
        }

        s.b[3097] = (p.p55 == 0.0);
        s.v[3097] = if s.b[3097] { 1.0 } else { 0.0 };

        s.b[3098] = (p.p50 == 0.0);
        s.v[3098] = if s.b[3098] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && s.b[3098]) {
            s.store_neg(3001, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && (!s.b[3098])) {
            s.copy_ad(3001, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(3001), p.p137, A::offset(s.ad_value(3001), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3001), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3001), p.p137), 782, 0.5);
        }

        s.b[3099] = (s.v[336] < 0.0);
        s.v[3099] = if s.b[3099] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && s.b[3099]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3(781, s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(603, s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[3100] = (4.0 == 1.0);
        s.v[3100] = if s.b[3100] { 1.0 } else { 0.0 };

        s.b[3101] = (4.0 == 2.0);
        s.v[3101] = if s.b[3101] { 1.0 } else { 0.0 };

        s.b[3102] = (4.0 == 3.0);
        s.v[3102] = if s.b[3102] { 1.0 } else { 0.0 };

        s.b[3103] = (4.0 == 4.0);
        s.v[3103] = if s.b[3103] { 1.0 } else { 0.0 };

        s.b[3104] = (p.p55 == 1.0);
        s.v[3104] = if s.b[3104] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3100]) && s.b[3104]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && s.b[3100]) && (!s.b[3104])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[3100]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[3101] && (!s.b[3100]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[3105] = (p.p55 == 1.0);
        s.v[3105] = if s.b[3105] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && s.b[3105]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && (!s.b[3105])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) {
            s.copy_ad(697, 404);
        }

        s.b[3106] = (p.p430 == 0.0);
        s.v[3106] = if s.b[3106] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && s.b[3106]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[3103] && (!((s.b[3100] || s.b[3101]) || s.b[3102])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.b[3107] = (p.p430 > 0.0);
        s.v[3107] = if s.b[3107] { 1.0 } else { 0.0 };

        if s.b[3107] {
            s.store_scalar(406, 1.0);
        }

        s.b[3108] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[3108] = if s.b[3108] { 1.0 } else { 0.0 };

        if (s.b[3107] && s.b[3108]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, 0.0);
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_scalar(3116, 0.4);
            s.store_scalar(3117, 0.0);
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
            s.store_scalar(79, (-1.0));
            s.store_scalar(3118, 0.0);
            s.store_scalar(3119, 0.0);
            s.store_mul_scaled_ad_rhs(3114, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3114), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

    }

    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[3107] && s.b[3108]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3107] && s.b[3108]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(3115, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[3121] = (s.v[3116] > (s.v[3115] * 0.5));
        s.v[3121] = if s.b[3121] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3121]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3122] = param_given[338];
        s.v[3122] = if s.b[3122] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3122]) {
            s.store_scalar(3115, p.p338);
        }

        s.b[3123] = param_given[339];
        s.v[3123] = if s.b[3123] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3123]) {
            s.store_scalar(3116, p.p339);
        }

        s.b[3124] = param_given[338];
        s.v[3124] = if s.b[3124] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3123])) && s.b[3124]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3125] = (s.v[3116] > (s.v[3115] * 0.5));
        s.v[3125] = if s.b[3125] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3125]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3126] = (p.p38 == 1.0);
        s.v[3126] = if s.b[3126] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {
            s.store_neg(334, 396);
        }

        s.b[3127] = (s.v[334] > s.v[3116]);
        s.v[3127] = if s.b[3127] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && s.b[3127]) {
            s.store_sub(335, 334, 3116);
            s.store_sub(336, 3115, 3116);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3116, 333);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && (!s.b[3127])) {
            s.copy_ad(344, 334);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {
            s.store_neg(397, 344);
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3126])) {
            s.copy_ad(397, 396);
        }

        if (s.b[3107] && s.b[3108]) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(3110, 0.0);
            s.store_scale(3111, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3128] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3128] = if s.b[3128] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3128]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign91250_loop_guard: usize = 0;
        while {
            let assign91250_cond_e139909: f64 = if (((s.b[3107] && s.b[3108]) && (!s.b[3128])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91250_cond_e139909 != 0.0
        } {
            assign91250_loop_guard += 1;
            assert!(assign91250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs(335, s.ad_value(781), 0.5, s.ad_value(782), 0.5, 0.5);
        }

        s.b[3129] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_add_scaled_inputs3(781, s.ad_value(402), 1.0, s.ad_value(397), 1.0, s.ad_value(335), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3130] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3130] = if s.b[3130] { 1.0 } else { 0.0 };

        s.b[3131] = (1.0 == 1.0);
        s.v[3131] = if s.b[3131] { 1.0 } else { 0.0 };

        if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && s.b[3131]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3132] = (1.0 == 2.0);
        s.v[3132] = if s.b[3132] { 1.0 } else { 0.0 };

        if ((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && s.b[3132]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3133] = (1.0 == 4.0);
        s.v[3133] = if s.b[3133] { 1.0 } else { 0.0 };

        if (((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && s.b[3133]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3134] = (1.0 == 8.0);
        s.v[3134] = if s.b[3134] { 1.0 } else { 0.0 };

        if ((((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && (!s.b[3133])) && s.b[3134]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign91580_loop_guard: usize = 0;
        while {
            let assign91580_cond_e140317: f64 = if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91580_cond_e140317 != 0.0
        } {
            assign91580_loop_guard += 1;
            assert!(assign91580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && (!s.b[3130])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && (!s.b[3129])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[3135] = (s.v[402] < s.v[403]);
        s.v[3135] = if s.b[3135] { 1.0 } else { 0.0 };

        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[3136] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && s.b[3136]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && (!s.b[3136])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3118, 404);
        }

        s.b[3137] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3137] = if s.b[3137] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3137]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3137])) {
            s.store_offset_ad(332, A::div_scaled_offset_numerator(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3138] = (s.v[116] >= 3.0);
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_ad(332, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

    }

    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_ad(332, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3138])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2(437, s.ad_value(402), -1.0, s.ad_value(397), -1.0, s.ad_value(212), 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3(116, s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
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
            s.store_add_scaled_inputs3(447, s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
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
            s.store_add_scaled_inputs3(781, s.ad_value(445), 1.0, s.ad_value(446), (-1.0), s.ad_value(446), 0.2);
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
            s.store_add_scaled_inputs3(116, s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0);
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
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
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
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
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3152] = (s.v[333] < 60.0);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3152]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
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
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3154])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
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
            s.store_add_scaled_inputs3(781, s.ad_value(3155), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1);
        }

    }

    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
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
            s.store_add_scaled_inputs3(335, s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0);
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
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
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
            s.store_add_ad_lhs(404, A::offset(s.ad_value(3156), (-0.1)), 780);
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
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
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
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
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
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
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
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
                s.store_add_scaled_inputs4(338, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
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
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
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
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
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
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
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
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
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
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
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
                s.store_add_scaled_inputs4(3119, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
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
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3201] = (s.v[214] > 0.0);
            s.v[3201] = if s.b[3201] { 1.0 } else { 0.0 };
            if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3201]) {
                s.store_sqrt_add(216, 3119, 214);
                s.store_div_scaled_inputs2(217, s.ad_value(3120), 0.5, s.ad_value(215), 0.5, s.ad_value(216), 1.0);
            }
            s.b[3202] = (s.v[3119] > 0.0);
            s.v[3202] = if s.b[3202] { 1.0 } else { 0.0 };
            if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3201])) && s.b[3202]) {
                s.store_sqrt(216, 3119);
                s.store_div_scaled_inputs(217, s.ad_value(3120), 0.5, s.ad_value(216), 1.0);
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
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(3112), p.p137, A::offset(s.ad_value(3112), p.p137)), ((4.0 * 0.1) * 0.1));
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
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3(781, s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1));
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
            s.store_add_scaled_inputs3(603, s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
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
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add(s.ad_value(290), s.ad_value(791)));
            s.store_scale(339, 335, p.p66);
            s.store_sub_from_scalar(343, 1.2, 87);
            s.store_add_scaled_products_indices(291, 791, 339, 1.0, 338, 343, (-1.0));
        }

        if (s.b[3211] && (!s.b[3212])) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p66);
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
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
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p63);
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
            s.store_add_scaled_products_left_left_ad(292, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));
        }

        if (s.b[3213] && (!s.b[3214])) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add(s.ad_value(290), s.ad_value(791)));
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

        s.store_mul_scaled_ad_rhs(298, 294, -1.0, A::sub(s.ad_value(734), s.ad_value(733)));

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
            s.store_add_scaled_inputs3(168, s.ad_value(790), s.v[172], s.ad_value(87), s.v[172], s.ad_value(91), (1.0 - s.v[172]));
        }

        s.b[3219] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[3219] = if s.b[3219] { 1.0 } else { 0.0 };

        if (s.b[3218] && s.b[3219]) {
            s.store_offset_ad(781, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
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
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
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
            s.store_add_scaled_inputs3(335, s.ad_value(1435), s.v[172], s.ad_value(87), s.v[172], s.ad_value(304), (1.0 - s.v[172]));
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
            s.store_add_scaled_inputs3(847, s.ad_value(873), 1.0, s.ad_value(875), 1.0, s.ad_value(877), 1.0);
        }

        s.b[3232] = (s.v[847] > 0.0);
        s.v[3232] = if s.b[3232] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3232]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_ad(848, A::div_from_scalar(s.v[820], s.ad_value(154)), A::ln(A::offset(A::div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0), 1.0)));
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
            s.store_add_scaled_inputs3(852, s.ad_value(879), 1.0, s.ad_value(881), 1.0, s.ad_value(883), 1.0);
        }

        s.b[3235] = (s.v[852] > 0.0);
        s.v[3235] = if s.b[3235] { 1.0 } else { 0.0 };

        if (s.b[3229] && s.b[3235]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_ad(853, A::div_from_scalar(s.v[825], s.ad_value(154)), A::ln(A::offset(A::div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0), 1.0)));
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

    pub(super) fn stamp_transient_block_90(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
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

        if (s.b[3250] && s.b[3251]) {
            s.store_add_ad(885, A::add_scaled_products(s.ad_value(873), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if (s.b[3250] && (!s.b[3251])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 873, 850, 335);
            s.store_add_ad(885, A::add_scaled_products3(s.ad_value(873), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if (!s.b[3250]) {
            s.store_scalar(885, 0.0);
        }

        s.store_scale(346, 874, p.p514);

        s.store_add_scaled_product_indices(885, 885, 1.0, 346, 860, 1.0);

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

        if (s.b[3253] && s.b[3254]) {
            s.store_add_ad(887, A::add_scaled_products(s.ad_value(875), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if (s.b[3253] && (!s.b[3254])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 875, 850, 335);
            s.store_add_ad(887, A::add_scaled_products3(s.ad_value(875), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if (!s.b[3253]) {
            s.store_scalar(887, 0.0);
        }

        s.store_scale(346, 876, p.p514);

        s.store_add_scaled_product_indices(887, 887, 1.0, 346, 860, 1.0);

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

        if ((s.b[3256] && s.b[3257]) && s.b[3258]) {
            s.store_add_ad(889, A::add_scaled_products(s.ad_value(877), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if ((s.b[3256] && s.b[3257]) && (!s.b[3258])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 877, 850, 335);
            s.store_add_ad(889, A::add_scaled_products3(s.ad_value(877), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(868), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));
        }

        if (s.b[3256] && (!s.b[3257])) {
            s.store_scalar(889, 0.0);
        }

        if s.b[3256] {
            s.store_scale(346, 878, p.p514);
            s.store_add_scaled_product_indices(889, 889, 1.0, 346, 868, 1.0);
        }

        if (!s.b[3256]) {
            s.store_scalar(889, 0.0);
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

        if (s.b[3260] && s.b[3261]) {
            s.store_add_ad(886, A::add_scaled_products(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if (s.b[3260] && (!s.b[3261])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 879, 855, 335);
            s.store_add_ad(886, A::add_scaled_products3(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if (!s.b[3260]) {
            s.store_scalar(886, 0.0);
        }

        s.store_scale(346, 880, p.p537);

        s.store_add_scaled_product_indices(886, 886, 1.0, 346, 859, 1.0);

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

        if (s.b[3263] && s.b[3264]) {
            s.store_add_ad(888, A::add_scaled_products(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if (s.b[3263] && (!s.b[3264])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 881, 855, 335);
            s.store_add_ad(888, A::add_scaled_products3(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if (!s.b[3263]) {
            s.store_scalar(888, 0.0);
        }

        s.store_scale(346, 882, p.p537);

        s.store_add_scaled_product_indices(888, 888, 1.0, 346, 859, 1.0);

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

        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {
            s.store_add_ad(890, A::add_scaled_products(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if ((s.b[3266] && s.b[3267]) && (!s.b[3268])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 883, 855, 335);
            s.store_add_ad(890, A::add_scaled_products3(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(867), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));
        }

        if (s.b[3266] && (!s.b[3267])) {
            s.store_scalar(890, 0.0);
        }

        if s.b[3266] {
            s.store_scale(346, 884, p.p537);
            s.store_add_scaled_product_indices(890, 890, 1.0, 346, 867, 1.0);
        }

        if (!s.b[3266]) {
            s.store_scalar(890, 0.0);
        }

        s.b[3270] = (s.v[832] > 0.0);
        s.v[3270] = if s.b[3270] { 1.0 } else { 0.0 };

        s.b[3271] = (s.v[860] < 0.0);
        s.v[3271] = if s.b[3271] { 1.0 } else { 0.0 };

        if (s.b[3270] && s.b[3271]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(841)));
        }

        s.b[3272] = (p.p503 == 0.5);
        s.v[3272] = if s.b[3272] { 1.0 } else { 0.0 };

        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

    }

    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_div_scaled_inputs(336, s.ad_value(832), p.p503, s.ad_value(841), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(842)));
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
            s.store_div_scaled_inputs(336, s.ad_value(833), p.p504, s.ad_value(842), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(868), s.ad_value(843)));
        }

        s.b[3279] = (p.p505 == 0.5);
        s.v[3279] = if s.b[3279] { 1.0 } else { 0.0 };

        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && s.b[3279]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

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
            s.store_div_scaled_inputs(336, s.ad_value(834), p.p505, s.ad_value(843), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(843)));
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
            s.store_div_scaled_inputs(336, s.ad_value(834), p.p505, s.ad_value(843), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(844)));
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
            s.store_div_scaled_inputs(336, s.ad_value(835), p.p526, s.ad_value(844), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(845)));
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
            s.store_div_scaled_inputs(336, s.ad_value(838), p.p527, s.ad_value(845), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(867), s.ad_value(846)));
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
            s.store_div_scaled_inputs(336, s.ad_value(839), p.p528, s.ad_value(846), 1.0);
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
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(846)));
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
            s.store_div_scaled_inputs(336, s.ad_value(839), p.p528, s.ad_value(846), 1.0);
            s.store_mul_add_scaled_product_rhs(896, 859, s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5);
        }

        if ((!s.b[3289]) && (!s.b[3293])) {
            s.store_scalar(896, 0.0);
        }

        s.store_scaled_add(862, 886, 888, s.v[365]);

        s.store_scaled_add(861, 885, 887, s.v[365]);

        s.b[3296] = (p.p48 > 0.0);
        s.v[3296] = if s.b[3296] { 1.0 } else { 0.0 };

        if s.b[3296] {
            s.store_scale(870, 890, s.v[365]);
            s.store_scale(869, 889, s.v[365]);
            s.store_scaled_add(66, 892, 894, s.v[365]);
            s.store_scaled_add(65, 891, 893, s.v[365]);
            s.store_scale(68, 896, s.v[365]);
            s.store_scale(67, 895, s.v[365]);
        }

        if (!s.b[3296]) {
            s.store_scalar(870, 0.0);
            s.store_scalar(869, 0.0);
            s.store_add_scaled_inputs3(66, s.ad_value(892), s.v[365], s.ad_value(894), s.v[365], s.ad_value(896), s.v[365]);
            s.store_add_scaled_inputs3(65, s.ad_value(891), s.v[365], s.ad_value(893), s.v[365], s.ad_value(895), s.v[365]);
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

        s.store_div_scaled_product_denominator_ad(911, 909, 910, 2.0, A::add(s.ad_value(909), s.ad_value(910)), 1.0);

        s.store_powf(336, 676, p.p547);

        s.store_scale(913, 336, p.p544);

        s.store_sqrt_mul(912, 913, 911);

        s.store_mul_scaled_ad_rhs(934, 155, s.v[906], A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))));

        s.store_mul_scaled_ad_rhs(935, 155, s.v[906], A::add(A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), A::div_from_scalar(p.p545, s.ad_value(912))));

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
            s.store_mul_scaled_ad_rhs(941, 937, p.p541, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542))));
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
        }

    }

    pub(super) fn stamp_transient_block_92(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[3297] && s.b[3300]) {
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
            s.store_mul_scaled_ad_rhs(942, 938, p.p541, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542))));
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

        if s.b[3297] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[3297] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(915, p.p545, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
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
            s.store_mul_ad_product_rhs(917, 912, s.ad_value(928), A::sub(A::exp(A::div_from_scalar((-p.p545), s.ad_value(912))), A::exp(A::div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0))));
            s.store_mul_ad_product_rhs(918, 912, s.ad_value(929), A::offset(A::exp(A::div_scaled_inputs(A::sub_from_scalar(p.p545, s.ad_value(933)), -1.0, s.ad_value(912), 1.0)), (-1.0)));
            s.store_neg_ad(919, A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(917), 1.0, s.ad_value(918), 1.0));
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
            s.store_exp_ad(336, A::div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
            s.store_mul3_lhs(428, 339, 283, 336);
        }

        s.b[3316] = (p.p48 > 0.0);
        s.v[3316] = if s.b[3316] { 1.0 } else { 0.0 };

        if ((s.b[3307] && s.b[3308]) && s.b[3316]) {
            s.store_mul_offset_lhs(429, 428, 1.0, 870);
        }

        if ((s.b[3307] && s.b[3308]) && (!s.b[3316])) {
            s.store_mul_offset_lhs(429, 428, 1.0, 862);
        }

        s.b[3317] = (s.v[78] == 0.0);
        s.v[3317] = if s.b[3317] { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && s.b[3317]) {
            s.store_scalar(346, p.p270);
            s.store_scalar(344, p.p271);
            s.copy_ad(337, 170);
            s.store_mul_ad_lhs(335, A::mul3(s.ad_value(346), s.ad_value(344), s.ad_value(337)), 337);
            s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);
            s.store_div(306, 335, 336);
        }

        if ((s.v[81] != 0.0) && (!s.b[3317])) {
            s.store_scalar(306, p.p270);
        }

        if (s.v[81] != 0.0) {
            s.store_scalar(336, s.v[565]);
            s.store_mul(307, 336, 185);
        }

        s.b[3318] = ((p.p26 != 0.0) && (s.v[78] == 0.0));
        s.v[3318] = if s.b[3318] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_93(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3318] {
            s.store_scalar(309, s.v[522]);
            s.store_scalar(310, s.v[521]);
            s.store_scalar(311, s.v[563]);
            s.store_scale(335, 238, 6.241449993689894e18);
            s.store_sqrt_offset_ad(782, A::mul(A::sub(s.ad_value(87), s.ad_value(1431)), A::sub(s.ad_value(87), s.ad_value(1431))), ((4.0 * 0.001) * 0.001));
            s.store_scaled_offset_ad(334, A::div_scaled_inputs2(s.ad_value(87), 1.0, s.ad_value(1431), (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3(339, s.ad_value(87), 0.5, s.ad_value(1431), ((-1.0) * 0.5), s.ad_value(782), 0.5);
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
            let assign101290_ad_e153400: A = A::add_scaled_product(A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0);
            s.store_add_scaled_product_mixed_aai(338, assign101290_ad_e153400, 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);
        }

        if (s.b[3318] && (!s.b[3320])) {
            s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);
        }

        if s.b[3318] {
            s.store_mul_ad_lhs(312, A::div_scaled_product_by_product(A::square(s.ad_value(134)), s.ad_value(310), 1.0, s.ad_value(170), s.ad_value(154), s.v[632]), 338);
        }

        if (!s.b[3318]) {
            s.store_scalar(312, 0.0);
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
        }

        if s.b[3321] {
            let assign101490_ad_e153678: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0);
            s.store_div_scaled_product_by_product(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), assign101490_ad_e153678, 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), s.ad_value(314), 1.0);
        }

        if (!s.b[3321]) {
            s.store_scalar(315, 0.0);
        }

        s.b[3324] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3324] = if s.b[3324] { 1.0 } else { 0.0 };

        if s.b[3324] {
            s.store_sqrt(322, 319);
            s.store_add(336, 127, 322);
            s.store_square(337, 317);
            s.store_square(338, 319);
            s.store_scaled_mul(339, 317, 319, 42.0);
            s.store_add_scaled_inputs3(339, s.ad_value(339), 1.0, s.ad_value(337), 4.0, s.ad_value(338), 4.0);
            s.store_add_ad_rhs(339, 339, A::mul3_scaled_output(s.ad_value(322), s.ad_value(127), A::add(s.ad_value(317), s.ad_value(319)), 20.0));
            s.store_square(344, 336);
            s.store_square(344, 344);
            s.store_div_ad_rhs(323, 339, A::mul(s.ad_value(344), s.ad_value(336)));
            s.store_mul_ad_product_lhs(324, A::div_from_scalar(s.v[632], s.ad_value(170)), s.ad_value(253), 185);
            s.store_mul(325, 324, 127);
            s.store_div(326, 315, 325);
            s.store_add_ad_lhs(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);
            s.store_div_scaled_product_by_product(327, s.ad_value(320), s.ad_value(341), 3.872983346207417, s.ad_value(336), A::sqrt(A::mul(A::mul3(s.ad_value(326), s.ad_value(336), s.ad_value(127)), s.ad_value(339))), 6.0);
        }

        s.store_scale(0, 134, s.v[365]);

        s.store_scale(699, 400, s.v[365]);

        s.copy_ad(430, 429);

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
            s.store_scale(754, 20, s.v[365]);
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
            s.store_scale(308, 572, s.v[188]);
            s.store_voltage(817, ctx, nodes, Some(13), None);
            s.store_div_scaled_inputs2(815, s.ad_value(817), 1.0, s.ad_value(816), (-1.0), s.ad_value(308), 1.0);
            s.store_add_scaled_inputs3(352, s.ad_value(352), 1.0, s.ad_value(816), -1.0, s.ad_value(817), 1.0);
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
            s.store_add_scaled_inputs4(700, s.ad_value(700), 1.0, s.ad_value(305), s.v[365], s.ad_value(360), ((-1.0) * s.v[365]), s.ad_value(362), (-s.v[365]));
            s.store_add_scaled_inputs3(701, s.ad_value(701), 1.0, s.ad_value(361), s.v[365], s.ad_value(305), (-s.v[365]));
            s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);
            s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);
            s.store_scale(706, 358, s.v[365]);
            s.store_scale(707, 359, s.v[365]);
            s.store_offset_ad(703, A::sub_scaled_inputs(s.ad_value(299), (-s.v[365]), s.ad_value(298), s.v[365]), s.v[703]);
            s.store_offset_ad(704, A::sub_scaled_inputs(s.ad_value(301), (-s.v[365]), s.ad_value(297), s.v[365]), s.v[704]);
        }

        s.store_scaled_add(709, 280, 287, s.v[365]);

        s.store_scale(710, 281, s.v[365]);

        s.store_scale(11, 202, (-s.v[365]));

        s.b[3328] = (s.v[949] == 1.0);
        s.v[3328] = if s.b[3328] { 1.0 } else { 0.0 };

        if s.b[3328] {
            s.store_sub_scaled_inputs(9, 199, (p.p252 * s.v[365]), 201, s.v[365]);
        }

        if (!s.b[3328]) {
            s.store_sub_scaled_inputs(9, 199, ((1.0 - p.p252) * s.v[365]), 200, s.v[365]);
        }

        s.b[3329] = (s.v[949] == 1.0);
        s.v[3329] = if s.b[3329] { 1.0 } else { 0.0 };

        if s.b[3329] {
            s.store_sub_scaled_inputs(10, 199, ((1.0 - p.p252) * s.v[365]), 200, s.v[365]);
        }

        if (!s.b[3329]) {
            s.store_sub_scaled_inputs(10, 199, (p.p252 * s.v[365]), 201, s.v[365]);
        }

        s.store_scale(7, 203, s.v[365]);

        s.store_scale(8, 204, s.v[365]);

        s.store_scale(807, 387, (4.0 * 1.3806226e-23));

        s.store_scale(711, 312, s.v[365]);

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

        s.v[714] = 0.0;

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
            s.store_div_scaled_inputs2(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, s.ad_value(329), (-1.0), s.ad_value(790), 1.0);
            s.store_add_ad_rhs(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));
        }

        if (s.b[3330] && (!s.b[3331])) {
            s.store_div(331, 254, 316);
        }

        if s.b[3330] {
            s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);
            s.copy_ad(714, 327);
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

        if s.b[3330] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(714, 0.0);
            }
        }

        s.store_mul(952, 807, 712);

        s.copy_ad(951, 714);

    }

    pub(super) fn stamp_transient_block_94(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
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
            s.store_offset_ad(607, A::mul3(s.ad_value(795), s.ad_value(786), s.ad_value(652)), 1e-25);
            s.store_div(608, 804, 604);
            s.store_mul(609, 606, 608);
        }

        s.b[3335] = (s.v[804] >= 0.0);
        s.v[3335] = if s.b[3335] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3335]) {
            s.store_div(335, 609, 607);
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3335])) {
            s.store_div_scaled_inputs(335, s.ad_value(609), -1.0, s.ad_value(607), 1.0);
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
            s.store_pow_ad(337, s.ad_value(335), A::offset(s.ad_value(959), (-1.0)));
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
            s.store_mul_ad_lhs(613, A::mul3(s.ad_value(335), s.ad_value(612), s.ad_value(610)), 611);
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

        s.b[3346] = ((s.v[5] > p.p444) && (p.p30 != 0.0));
        s.v[3346] = if s.b[3346] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3346]) {
            s.store_div_from_scalar(696, s.v[365], 5);
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3346])) {
            s.store_scalar(696, 0.0);
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
            s.store_offset_ad(3351, A::mul3(s.ad_value(788), s.ad_value(786), s.ad_value(652)), 1e-25);
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
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[3360] = (s.v[108] < 1e-12);
        s.v[3360] = if s.b[3360] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3359]) && s.b[3360]) {
            s.store_scalar(108, 1e-12);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {
            s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);
        }

    }

    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {
            s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
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
            s.store_div_scaled_inputs(335, s.ad_value(592), -1.0, s.ad_value(3351), 1.0);
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
            s.store_pow_ad(337, s.ad_value(335), A::offset(s.ad_value(956), (-1.0)));
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
            s.store_add_scaled_inputs3(781, s.ad_value(717), 1.0, s.ad_value(718), (-1.0), s.ad_value(717), (-0.001));
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
            s.store_add_scaled_inputs3(718, s.ad_value(717), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.store_sub(597, 717, 718);
        }

        s.b[3367] = ((p.p441 > 0.0) && (p.p440 > 1.0));
        s.v[3367] = if s.b[3367] { 1.0 } else { 0.0 };

        s.b[3368] = ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0));
        s.v[3368] = if s.b[3368] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_add_scaled_inputs3(781, s.ad_value(597), 1.0, s.ad_value(408), (-p.p440), s.ad_value(408), p.p441);
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
            s.store_add_scaled_inputs3(336, s.ad_value(408), p.p440, s.ad_value(408), (-p.p441), s.ad_value(780), 1.0);
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
            s.store_add_scaled_inputs3(781, s.ad_value(789), 1.0, s.ad_value(600), (-1.0), s.ad_value(789), (-0.01));
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
            s.store_add_scaled_inputs3(602, s.ad_value(789), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
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
            s.store_mul_ad_lhs(739, A::mul3(s.ad_value(335), s.ad_value(596), s.ad_value(3350)), 597);
        }

        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));
        s.v[3377] = if s.b[3377] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_sub_from_scalar(781, 1e-25, 739);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
        }

    }
}
