#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_160(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.v[2623] != 0.0) {s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2643, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[2649] = (s.v[2644] > (s.v[2643] * 0.5));s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2649]) {s.store_scale(2644, 2643, 0.5);}
        s.b[2650] = param_given[338];s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2650]) {s.store_scalar(2643, p[338]);}
        s.b[2651] = param_given[339];s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2651]) {s.store_scalar(2644, p[339]);}
        s.b[2652] = param_given[338];s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2651])) && s.b[2652]) {s.store_scale(2644, 2643, 0.5);}
        s.b[2653] = (s.v[2644] > (s.v[2643] * 0.5));s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2653]) {s.store_scale(2644, 2643, 0.5);}
        s.b[2654] = (p[38] == 1.0);s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2654]) {s.store_neg(334, 396);}
        s.b[2655] = (s.v[334] > s.v[2644]);s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2654]) && s.b[2655]) {s.store_sub(335, 334, 2644);s.store_sub(336, 2643, 2644);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 2644, 333);}
        if (((s.v[2623] != 0.0) && s.b[2654]) && (!s.b[2655])) {s.copy_ad(344, 334);}
        if ((s.v[2623] != 0.0) && s.b[2654]) {s.store_neg(397, 344);}
        if ((s.v[2623] != 0.0) && (!s.b[2654])) {s.copy_ad(397, 396);}
        if (s.v[2623] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);}
        let (t3,) = {
    if (s.v[2623] != 0.0) {
        let t0: f64 = (-s.v[397]);let t1: f64 = (10.0 * 2.220446049250313e-16);let t2: f64 = (t0 + t1);
        (t2,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, t3);
        if (s.v[2623] != 0.0) {s.store_scalar(2638, 0.0);s.store_primal_scale(2639, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[2656] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2656]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2623] != 0.0) && (!s.b[2656])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((s.v[2623] != 0.0) && (!s.b[2656])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2623] != 0.0) && (!s.b[2656])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2623] != 0.0) && (!s.b[2656])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[2657] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t6,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_161(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (t7,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7);
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2658] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });s.b[2659] = (1.0 == 1.0);s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });
        let (t8,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && s.b[2659]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8);s.b[2660] = (1.0 == 2.0);s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && s.b[2660]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9);s.b[2661] = (1.0 == 4.0);s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });
        let (ta,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && s.b[2661]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta);s.b[2662] = (1.0 == 8.0);s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && (!s.b[2661])) && s.b[2662]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb);
        let (tc,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc);let mut t10: usize = 0;
        while {
            let tf: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;
            if t10 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t10, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {s.store_sqrt(726, 726);}
            let (te,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
        let td: f64 = (s.v[719] + 1.0);
        (td,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, te);
        }
        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && (!s.b[2658])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2657])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);}
        let (t14,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let t11: f64 = (-s.v[397]);let t12: f64 = (10.0 * 2.220446049250313e-16);let t13: f64 = (t11 + t12);
        (t13,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, t14);s.b[2663] = (s.v[402] < s.v[403]);s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2663]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2664] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2663]) && s.b[2664]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2623] != 0.0) && s.b[2663]) && (!s.b[2664])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2623] != 0.0) && s.b[2663]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2646, 404);}
        s.b[2665] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2665]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_162(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2665])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2623] != 0.0) && (!s.b[2663])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2666] = (s.v[116] >= 3.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2666]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2666])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2667] = (p[33] > 0.0);s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_163(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2668] = (p[33] == 2.0);s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2668])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {s.store_sub(444, 444, 447);s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {s.copy_ad(445, 116);}
        s.b[2669] = (p[33] == 2.0);s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });s.b[2670] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t15,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t15);
        let (t16,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t16);
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });s.b[2672] = (2.0 == 1.0);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        let (t17,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && s.b[2672]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t17);s.b[2673] = (2.0 == 2.0);s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });
        let (t18,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && s.b[2673]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t18);s.b[2674] = (2.0 == 4.0);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        let (t19,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && s.b[2674]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t19);s.b[2675] = (2.0 == 8.0);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });
        let (t1a,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && (!s.b[2674])) && s.b[2675]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1a);
        let (t1b,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1b);let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {s.store_sqrt(726, 726);}
            let (t1d,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
        let t1c: f64 = (s.v[719] + 1.0);
        (t1c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1d);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_164(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && (!s.b[2671])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && (!s.b[2670])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2669])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2676] = (p[33] == 1.0);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2677] = (s.v[411] > 0.0);s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2677]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_offset_lhs(336, 729, p[137], 782, 0.5);}
        s.b[2678] = (s.v[336] < 0.0);s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) && s.b[2678]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2679] = (s.v[336] < 0.0);s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2679]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2680] = (s.v[333] < 60.0);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2680]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2680])) {s.store_sub(416, 414, 418);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {s.store_mul(415, 154, 416);}
        s.b[2681] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });
        let (t21,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2681]) {
        let t20: f64 = (s.v[2645] + 1.0);
        (t20,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, t21);
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2681]) {s.copy_ad(116, 447);}
        if ((s.v[2623] != 0.0) && (!s.b[2663])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2682] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2682]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2682])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2623] != 0.0) && (!s.b[2663])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2683, 354, 2639);}
        s.b[2685] = (p[33] == 2.0);s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });s.b[2686] = ((s.v[2683] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {s.store_add_scaled_inputs3_indices(781, 2683, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t22,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t22);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_165(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t23,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t23);
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2687] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });s.b[2688] = (2.0 == 1.0);s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });
        let (t24,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && s.b[2688]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t24);s.b[2689] = (2.0 == 2.0);s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        let (t25,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && s.b[2689]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t25);s.b[2690] = (2.0 == 4.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });
        let (t26,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && s.b[2690]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t26);s.b[2691] = (2.0 == 8.0);s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        let (t27,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && (!s.b[2690])) && s.b[2691]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        let (t28,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t28);let mut t2c: usize = 0;
        while {
            let t2b: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;
            if t2c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {s.store_sqrt(726, 726);}
            let (t2a,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
        let t29: f64 = (s.v[719] + 1.0);
        (t29,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2a);
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && (!s.b[2686])) {s.copy_ad(335, 2683);s.store_scalar(334, 1.0);}
        s.b[2692] = (s.v[334] < 1.0);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        let (t2e,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2692]) {
        let t2d: f64 = (s.v[2645] + 2.0);
        (t2d,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, t2e);
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) {
            if (s.v[2683] <= s.v[386]) {
                s.copy_ad(335, 2683);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2693] = (s.v[2683] >= s.v[386]);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        let (t30,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) && s.b[2693]) {
        let t2f: f64 = (s.v[2645] + 2.0);
        (t2f,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, t30);s.b[2694] = (s.v[2645] >= 2.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) {s.copy_ad(2684, 404);s.store_mul(354, 335, 2639);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2695] = (p[33] == 2.0);s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });s.b[2696] = ((s.v[404] > (s.v[2684] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {s.store_offset_sub(781, 404, 2684, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t31,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t31);
        let (t32,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2697] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });s.b[2698] = (2.0 == 1.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        let (t33,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t33);s.b[2699] = (2.0 == 2.0);s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_166(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t34,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && s.b[2699]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t34);s.b[2700] = (2.0 == 4.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });
        let (t35,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && s.b[2700]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t35);s.b[2701] = (2.0 == 8.0);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        let (t36,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && (!s.b[2700])) && s.b[2701]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t36);
        let (t37,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t37);let mut t3b: usize = 0;
        while {
            let t3a: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t3a != 0.0
        } {
            t3b += 1;
            if t3b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {s.store_sqrt(726, 726);}
            let (t39,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
        let t38: f64 = (s.v[719] + 1.0);
        (t38,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t39);
        }
        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && (!s.b[2697])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2684, (-0.1), 780);}
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && (!s.b[2695])) {
            if (s.v[404] <= s.v[2684]) {
            } else {
                s.copy_ad(404, 2684);
            }
        }
        if ((s.v[2623] != 0.0) && (!s.b[2663])) {s.copy_ad(2646, 404);}
        s.b[2702] = (p[33] == 1.0);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        let (t3c,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t3c);
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);}
        s.b[2703] = (s.v[411] > 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2703]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_offset_lhs(336, 729, p[137], 782, 0.5);}
        s.b[2704] = (s.v[336] < 0.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) && s.b[2704]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2705] = (s.v[336] < 0.0);s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2705]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);}
        let (t3d,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t3d);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_167(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t45: usize = 0;
        while {
            let t43: f64 = (s.v[421] + 1.0);let t44: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (s.v[97] <= t43)) { 1.0 } else { 0.0 };
            t44 != 0.0
        } {
            t45 += 1;
            if t45 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t45, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2707] = (s.v[333] < 60.0);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2707]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2707])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_mul(415, 154, 416);}
            s.b[2708] = (s.v[116] < 0.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2708]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[2709] = (s.v[116] < 1e-6);s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[2710] = (s.v[338] > 0.0);s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[2711] = (s.v[338] > 0.0);s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[2712] = (s.v[116] < 0.0);s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2712]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[2713] = (s.v[116] < 60.0);s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });s.b[2714] = (s.v[116] < 5e-5);s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && s.b[2714]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && (!s.b[2714])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2713])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2715] = (s.v[214] > 0.0);s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2715]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2715])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2716] = (s.v[79] == 1.0);s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });
            let (t3f,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2716]) {
        let t3e: f64 = (s.v[421] + 1.0);
        (t3e,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t3f);
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2717] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2717]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {s.store_add(404, 404, 236);}
            s.b[2718] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });
            let (t40,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2718]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t40);
            let (t42,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        let t41: f64 = (s.v[97] + 1.0);
        (t41,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t42);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_168(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_mul(2637, 982, 223);s.store_mul(2638, 2639, 2637);s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[2720] = (p[33] == 4.0);s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 2646);}
        let (t46,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t46);
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);}
        s.b[2721] = (s.v[411] > 0.0);s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2721]) {s.store_sub_from_scalar(336, p[334], 411);}
        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_offset_lhs(336, 729, p[137], 782, 0.5);}
        s.b[2722] = (s.v[336] < 0.0);s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) && s.b[2722]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2723] = (s.v[336] < 0.0);s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2723]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);}
        let (t47,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t47);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_169(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t50: usize = 0;
        while {
            let t4e: f64 = (s.v[421] + 1.0);let t4f: f64 = if (((s.v[2623] != 0.0) && s.b[2720]) && (s.v[97] <= t4e)) { 1.0 } else { 0.0 };
            t4f != 0.0
        } {
            t50 += 1;
            if t50 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t50, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2725] = (s.v[333] < 60.0);s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2725]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2725])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_mul(415, 154, 416);}
            s.b[2726] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2726]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(2647, 334, 336);s.store_mul_add_scaled_product_rhs_indices(2648, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2726])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(2647, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(2648, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[2727] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2727]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[2728] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(2728, if s.b[2728] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && s.b[2728]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && (!s.b[2728])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2729] = (s.v[214] > 0.0);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2729]) {s.store_sqrt_add(216, 2647, 214);s.store_div_scaled_inputs2_indices(217, 2648, 0.5, 215, 0.5, 216, 1.0);}
            s.b[2730] = (s.v[2647] > 0.0);s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && s.b[2730]) {s.store_sqrt(216, 2647);s.store_div_scaled_inputs_indices(217, 2648, 0.5, 216, 1.0);}
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && (!s.b[2730])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2731] = (s.v[79] > 0.0);s.store_scalar(2731, if s.b[2731] { 1.0 } else { 0.0 });
            let (t49,) = {
    if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2731]) {
        let t48: f64 = (s.v[421] + 1.0);
        (t48,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t49);
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2732] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2732, if s.b[2732] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2732]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {s.store_add(404, 404, 236);}
            s.b[2733] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });
            let (t4b,) = {
    if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2733]) {
        let t4a: f64 = (s.v[79] + 2.0);
        (t4a,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t4b);
            let (t4d,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        let t4c: f64 = (s.v[97] + 1.0);
        (t4c,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t4d);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_170(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.v[2623] != 0.0) && s.b[2720]) {
            if (s.v[2647] >= 0.0) {
                s.store_scaled_sqrt(223, 2647, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_mul(2637, 982, 223);s.store_mul(2638, 2639, 2637);s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.v[2623] != 0.0) {s.store_sub(399, 398, 354);}
        s.b[2735] = (s.v[407] < 0.0);s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2735]) {s.store_neg(407, 407);}
        s.b[2736] = (p[55] == 0.0);s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });s.b[2737] = (p[50] == 0.0);s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2737]) {s.store_neg(2640, 404);}
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && (!s.b[2737])) {s.copy_ad(2640, 396);}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_sqrt_offset_square_offset(782, 2640, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2640), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_offset_lhs(336, 2640, p[137], 782, 0.5);}
        s.b[2738] = (s.v[336] < 0.0);s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2738]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        s.b[2739] = (1.0 == 1.0);s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });s.b[2740] = (1.0 == 2.0);s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });s.b[2741] = (1.0 == 3.0);s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });s.b[2742] = (1.0 == 4.0);s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });s.b[2743] = (p[55] == 1.0);s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2739]) && s.b[2743]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2623] != 0.0) && s.b[2739]) && (!s.b[2743])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2623] != 0.0) && s.b[2739]) {s.store_mul(353, 338, 398);s.store_mul(356, 338, 354);}
        if ((s.v[2623] != 0.0) && (s.b[2740] && (!s.b[2739]))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(351, 338, 398);s.store_mul(359, 338, 354);}
        s.b[2744] = (p[55] == 1.0);s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2744]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && (!s.b[2744])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {s.copy_ad(697, 404);}
        s.b[2745] = (p[430] == 0.0);s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2745]) {s.copy_ad(698, 354);}
        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {s.store_mul(352, 338, 398);s.store_mul(355, 338, 354);s.copy_ad(816, 355);}
        if ((s.v[2623] != 0.0) && (s.b[2742] && (!((s.b[2739] || s.b[2740]) || s.b[2741])))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(350, 338, 398);s.store_mul(358, 338, 354);}
        s.store_scalar(2623, 0.0);s.b[2746] = (2.0 == 1.0);s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });s.b[2747] = (2.0 == 2.0);s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });s.b[2748] = (2.0 == 3.0);s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });s.b[2749] = (2.0 == 4.0);s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });s.b[2750] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });
        let (t51,) = {
    if (s.b[2746] && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t51);
        let (t52,) = {
    if (s.b[2746] && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, t52);
        if (s.b[2746] && s.b[2750]) {s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_171(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2746] && s.b[2750]) {s.store_scalar(407, p[66]);s.store_scalar(411, 0.0);s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2751] = (((((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });
        let (t53,) = {
    if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t53);
        if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2752] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });
        let (t54,) = {
    if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t54);
        let (t55,) = {
    if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.store_scalar(2624, t55);
        if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p[63] + (p[64] * p[55])));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2753] = (((s.v[407] < 0.0) && (p[432] > 0.0)) && (p[55] == 1.0));s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });
        if (((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) {s.store_neg(407, 407);s.store_scalar(335, p[63]);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p[137]));}
        s.b[2754] = (p[113] > 0.0);s.store_scalar(2754, if s.b[2754] { 1.0 } else { 0.0 });s.b[2755] = ((s.v[396] == 0.0) || (p[113] <= 0.0));s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2755]) {
        }
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p[113], 1.0);s.store_powf(782, 781, (1.0 / p[113]));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {s.store_sqrt_offset_square_offset(782, 396, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_offset_lhs(336, 396, p[137], 782, 0.5);}
        s.b[2756] = (s.v[336] < 0.0);s.store_scalar(2756, if s.b[2756] { 1.0 } else { 0.0 });
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2756]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub(407, 407, 600);}
        s.b[2757] = (((((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });
        let (t56,) = {
    if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t56);
        if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2623] != 0.0) {s.store_scalar(2765, 0.4);}
        let (t57,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[2766],)
    }
};
        s.store_scalar(2766, t57);
        if (s.v[2623] != 0.0) {s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);}
        let (t59,) = {
    if (s.v[2623] != 0.0) {
        let t58: f64 = (-1.0);
        (t58,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t59);
        if (s.v[2623] != 0.0) {s.store_scalar(2767, 0.0);s.store_scalar(2768, 0.0);s.store_mul_scaled_ln_ad_rhs(2763, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2763), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2623] != 0.0) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2764, 781, (-0.5), 782, (-0.5), 0.8);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_172(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2770] = (s.v[2765] > (s.v[2764] * 0.5));s.store_scalar(2770, if s.b[2770] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2770]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2771] = param_given[338];s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2771]) {s.store_scalar(2764, p[338]);}
        s.b[2772] = param_given[339];s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2772]) {s.store_scalar(2765, p[339]);}
        s.b[2773] = param_given[338];s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2772])) && s.b[2773]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2774] = (s.v[2765] > (s.v[2764] * 0.5));s.store_scalar(2774, if s.b[2774] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2774]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2775] = (p[38] == 1.0);s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2775]) {s.store_neg(334, 396);}
        s.b[2776] = (s.v[334] > s.v[2765]);s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2775]) && s.b[2776]) {s.store_sub(335, 334, 2765);s.store_sub(336, 2764, 2765);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 2765, 333);}
        if (((s.v[2623] != 0.0) && s.b[2775]) && (!s.b[2776])) {s.copy_ad(344, 334);}
        if ((s.v[2623] != 0.0) && s.b[2775]) {s.store_neg(397, 344);}
        if ((s.v[2623] != 0.0) && (!s.b[2775])) {s.copy_ad(397, 396);}
        if (s.v[2623] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);}
        let (t5d,) = {
    if (s.v[2623] != 0.0) {
        let t5a: f64 = (-s.v[397]);let t5b: f64 = (10.0 * 2.220446049250313e-16);let t5c: f64 = (t5a + t5b);
        (t5c,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, t5d);
        if (s.v[2623] != 0.0) {s.store_scalar(2759, 0.0);s.store_primal_scale(2760, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[2777] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(2777, if s.b[2777] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2777]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut t5f: usize = 0;
        while {
            let t5e: f64 = if (((s.v[2623] != 0.0) && (!s.b[2777])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t5e != 0.0
        } {
            t5f += 1;
            if t5f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[2778] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(2778, if s.b[2778] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t60,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t60);
        let (t61,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t61);
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_scalar(770, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_173(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2779] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });s.b[2780] = (1.0 == 1.0);s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });
        let (t62,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && s.b[2780]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t62);s.b[2781] = (1.0 == 2.0);s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });
        let (t63,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && s.b[2781]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t63);s.b[2782] = (1.0 == 4.0);s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });
        let (t64,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && s.b[2782]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t64);s.b[2783] = (1.0 == 8.0);s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });
        let (t65,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && (!s.b[2782])) && s.b[2783]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t65);
        let (t66,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t66);let mut t6a: usize = 0;
        while {
            let t69: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t69 != 0.0
        } {
            t6a += 1;
            if t6a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t6a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {s.store_sqrt(726, 726);}
            let (t68,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
        let t67: f64 = (s.v[719] + 1.0);
        (t67,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t68);
        }
        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && (!s.b[2779])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2778])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);}
        let (t6e,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let t6b: f64 = (-s.v[397]);let t6c: f64 = (10.0 * 2.220446049250313e-16);let t6d: f64 = (t6b + t6c);
        (t6d,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, t6e);s.b[2784] = (s.v[402] < s.v[403]);s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2784]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2785] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2784]) && s.b[2785]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2623] != 0.0) && s.b[2784]) && (!s.b[2785])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2623] != 0.0) && s.b[2784]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2767, 404);}
        s.b[2786] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2786]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2786])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_174(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2786])) {s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2623] != 0.0) && (!s.b[2784])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2787] = (s.v[116] >= 3.0);s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2787]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2787])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2788] = (p[33] > 0.0);s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_175(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2789] = (p[33] == 2.0);s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2789])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_sub(444, 444, 447);s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.copy_ad(445, 116);}
        s.b[2790] = (p[33] == 2.0);s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });s.b[2791] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t6f,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6f);
        let (t70,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t70);
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2792] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });s.b[2793] = (2.0 == 1.0);s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });
        let (t71,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && s.b[2793]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t71);s.b[2794] = (2.0 == 2.0);s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });
        let (t72,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && s.b[2794]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t72);s.b[2795] = (2.0 == 4.0);s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });
        let (t73,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && s.b[2795]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t73);s.b[2796] = (2.0 == 8.0);s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });
        let (t74,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && (!s.b[2795])) && s.b[2796]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t74);
        let (t75,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t75);let mut t79: usize = 0;
        while {
            let t78: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t78 != 0.0
        } {
            t79 += 1;
            if t79 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t79, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {s.store_sqrt(726, 726);}
            let (t77,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
        let t76: f64 = (s.v[719] + 1.0);
        (t76,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t77);
        }
    }
}
