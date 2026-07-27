#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_192(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) {s.store_mul(354, 335, 3115);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[3171] = (p[33] == 2.0);s.store_scalar(3171, if s.b[3171] { 1.0 } else { 0.0 });s.b[3172] = ((s.v[404] > (s.v[3160] - 0.1)) && (0.1 >= 0.0));s.store_scalar(3172, if s.b[3172] { 1.0 } else { 0.0 });
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {s.store_offset_sub(781, 404, 3160, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3173] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3173, if s.b[3173] { 1.0 } else { 0.0 });s.b[3174] = (2.0 == 1.0);s.store_scalar(3174, if s.b[3174] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && s.b[3174]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[3175] = (2.0 == 2.0);s.store_scalar(3175, if s.b[3175] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && s.b[3175]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[3176] = (2.0 == 4.0);s.store_scalar(3176, if s.b[3176] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && (!s.b[3175])) && s.b[3176]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[3177] = (2.0 == 8.0);s.store_scalar(3177, if s.b[3177] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && (!s.b[3175])) && (!s.b[3176])) && s.b[3177]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && (!s.b[3173])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 3160, (-0.1), 780);}
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
        }
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) {
        }
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) {s.store_scalar(334, 1.0);}
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && (!s.b[3171])) {
            if (s.v[404] <= s.v[3160]) {
            } else {
                s.copy_ad(404, 3160);
            }
        }
        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {s.copy_ad(3122, 404);}
        s.b[3178] = (p[33] == 1.0);s.store_scalar(3178, if s.b[3178] { 1.0 } else { 0.0 });
        let (tb,) = {
    if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, tb);
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3115)), s.ad_value(155)), 2.0);}
        s.b[3179] = (s.v[411] > 0.0);s.store_scalar(3179, if s.b[3179] { 1.0 } else { 0.0 });
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3179]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3180] = (s.v[336] < 0.0);s.store_scalar(3180, if s.b[3180] { 1.0 } else { 0.0 });
        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) && s.b[3180]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3181] = (s.v[336] < 0.0);s.store_scalar(3181, if s.b[3181] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_193(
        s: &mut Scratch,
    ) {
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3181]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3115, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);}
        let (tc,) = {
    if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, tc);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_194(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t14: usize = 0;
        while {
            let t12: f64 = (s.v[421] + 1.0);let t13: f64 = if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (s.v[97] <= t12)) { 1.0 } else { 0.0 };
            t13 != 0.0
        } {
            t14 += 1;
            if t14 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t14, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[3183] = (s.v[333] < 60.0);s.store_scalar(3183, if s.b[3183] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3183]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3183])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_mul(415, 154, 416);}
            s.b[3184] = (s.v[116] < 0.0);s.store_scalar(3184, if s.b[3184] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3184]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[3185] = (s.v[116] < 1e-6);s.store_scalar(3185, if s.b[3185] { 1.0 } else { 0.0 });
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[3186] = (s.v[338] > 0.0);s.store_scalar(3186, if s.b[3186] { 1.0 } else { 0.0 });
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) && s.b[3186]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) && (!s.b[3186])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[3187] = (s.v[338] > 0.0);s.store_scalar(3187, if s.b[3187] { 1.0 } else { 0.0 });
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) && s.b[3187]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) && (!s.b[3187])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[3188] = (s.v[116] < 0.0);s.store_scalar(3188, if s.b[3188] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3188]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[3189] = (s.v[116] < 60.0);s.store_scalar(3189, if s.b[3189] { 1.0 } else { 0.0 });s.b[3190] = (s.v[116] < 5e-5);s.store_scalar(3190, if s.b[3190] { 1.0 } else { 0.0 });
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3189]) && s.b[3190]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3189]) && (!s.b[3190])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && (!s.b[3189])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[3191] = (s.v[214] > 0.0);s.store_scalar(3191, if s.b[3191] { 1.0 } else { 0.0 });
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3191]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && (!s.b[3191])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[3192] = (s.v[79] == 1.0);s.store_scalar(3192, if s.b[3192] { 1.0 } else { 0.0 });
            let (te,) = {
    if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3192]) {
        let td: f64 = (s.v[421] + 1.0);
        (td,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, te);
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3193] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(3193, if s.b[3193] { 1.0 } else { 0.0 });
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) && s.b[3193]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {s.store_add(404, 404, 236);}
            s.b[3194] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(3194, if s.b[3194] { 1.0 } else { 0.0 });
            let (tf,) = {
    if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) && s.b[3194]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, tf);
            let (t11,) = {
    if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
        let t10: f64 = (s.v[97] + 1.0);
        (t10,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t11);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_195(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {s.store_mul(3113, 982, 223);s.store_mul(3114, 3115, 3113);s.store_offset_div(100, 3114, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[3196] = (p[33] == 4.0);s.store_scalar(3196, if s.b[3196] { 1.0 } else { 0.0 });
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 3122);}
        let (t15,) = {
    if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t15);
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3115)), s.ad_value(155)), 2.0);}
        s.b[3197] = (s.v[411] > 0.0);s.store_scalar(3197, if s.b[3197] { 1.0 } else { 0.0 });
        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3197]) {s.store_sub_from_scalar(336, p[334], 411);}
        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[3198] = (s.v[336] < 0.0);s.store_scalar(3198, if s.b[3198] { 1.0 } else { 0.0 });
        if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) && s.b[3198]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3199] = (s.v[336] < 0.0);s.store_scalar(3199, if s.b[3199] { 1.0 } else { 0.0 });
        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3199]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 3115, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);}
        let (t16,) = {
    if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t16);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_196(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t1f: usize = 0;
        while {
            let t1d: f64 = (s.v[421] + 1.0);let t1e: f64 = if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (s.v[97] <= t1d)) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[3201] = (s.v[333] < 60.0);s.store_scalar(3201, if s.b[3201] { 1.0 } else { 0.0 });
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3201]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3201])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_mul(415, 154, 416);}
            s.b[3202] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(3202, if s.b[3202] { 1.0 } else { 0.0 });
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3202]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(3123, 334, 336);s.store_mul_add_scaled_product_rhs_indices(3124, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3202])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(3123, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(3124, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[3203] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(3203, if s.b[3203] { 1.0 } else { 0.0 });
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3203]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[3204] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(3204, if s.b[3204] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3203])) && s.b[3204]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3203])) && (!s.b[3204])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[3205] = (s.v[214] > 0.0);s.store_scalar(3205, if s.b[3205] { 1.0 } else { 0.0 });
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3205]) {s.store_sqrt_add(216, 3123, 214);s.store_div_scaled_inputs2_indices(217, 3124, 0.5, 215, 0.5, 216, 1.0);}
            s.b[3206] = (s.v[3123] > 0.0);s.store_scalar(3206, if s.b[3206] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3205])) && s.b[3206]) {s.store_sqrt(216, 3123);s.store_div_scaled_inputs_indices(217, 3124, 0.5, 216, 1.0);}
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3205])) && (!s.b[3206])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[3207] = (s.v[79] > 0.0);s.store_scalar(3207, if s.b[3207] { 1.0 } else { 0.0 });
            let (t18,) = {
    if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3207]) {
        let t17: f64 = (s.v[421] + 1.0);
        (t17,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t18);
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3208] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(3208, if s.b[3208] { 1.0 } else { 0.0 });
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) && s.b[3208]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {s.store_add(404, 404, 236);}
            s.b[3209] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(3209, if s.b[3209] { 1.0 } else { 0.0 });
            let (t1a,) = {
    if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) && s.b[3209]) {
        let t19: f64 = (s.v[79] + 2.0);
        (t19,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t1a);
            let (t1c,) = {
    if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
        let t1b: f64 = (s.v[97] + 1.0);
        (t1b,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1c);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_197(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            if (s.v[3123] >= 0.0) {
                s.store_scaled_sqrt(223, 3123, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {s.store_mul(3113, 982, 223);s.store_mul(3114, 3115, 3113);s.store_offset_div(100, 3114, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.b[3111] && s.b[3112]) {s.store_sub(399, 398, 354);}
        s.b[3211] = (s.v[407] < 0.0);s.store_scalar(3211, if s.b[3211] { 1.0 } else { 0.0 });
        if ((s.b[3111] && s.b[3112]) && s.b[3211]) {s.store_neg(407, 407);}
        s.b[3212] = (p[55] == 0.0);s.store_scalar(3212, if s.b[3212] { 1.0 } else { 0.0 });s.b[3213] = (p[50] == 0.0);s.store_scalar(3213, if s.b[3213] { 1.0 } else { 0.0 });
        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && s.b[3213]) {s.store_neg(3116, 404);}
        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && (!s.b[3213])) {s.copy_ad(3116, 396);}
        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {s.store_sqrt_offset_square_offset(782, 3116, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3116), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(3116), p[137]), 782, 0.5);}
        s.b[3214] = (s.v[336] < 0.0);s.store_scalar(3214, if s.b[3214] { 1.0 } else { 0.0 });
        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && s.b[3214]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        if (s.b[3111] && s.b[3112]) {s.copy_ad(698, 354);}
        s.b[3215] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] == 0.0));s.store_scalar(3215, if s.b[3215] { 1.0 } else { 0.0 });
        let (t20,) = {
    if s.b[3215] {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t20);
        if s.b[3215] {s.store_scalar(289, s.v[564]);s.store_scalar(290, p[276]);s.store_scalar(335, (s.v[188] * s.v[635]));}
        s.b[3216] = (s.v[949] == 1.0);s.store_scalar(3216, if s.b[3216] { 1.0 } else { 0.0 });
        if (s.b[3215] && s.b[3216]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p[66]);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(291, 791, 339, 1.0, 338, 343, (-1.0));}
        if (s.b[3215] && (!s.b[3216])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p[66]);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(291, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
        s.b[3217] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] == 0.0));s.store_scalar(3217, if s.b[3217] { 1.0 } else { 0.0 });
        let (t21,) = {
    if s.b[3217] {
        (1.0,)
    } else {
        (s.v[2626],)
    }
};
        s.store_scalar(2626, t21);
        if s.b[3217] {s.store_scalar(289, s.v[564]);s.store_scalar(290, p[276]);s.store_scale(335, 412, s.v[635]);}
        s.b[3218] = (s.v[949] == 1.0);s.store_scalar(3218, if s.b[3218] { 1.0 } else { 0.0 });
        if (s.b[3217] && s.b[3218]) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));s.store_scale(339, 335, p[63]);s.store_sub_offset_lhs(343, 790, 1.2, 91);s.store_add_scaled_products_mixed_aiii(292, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_198(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3217] && (!s.b[3218])) {s.store_mul_ad_product_rhs_mixed_ia(338, 289, 335, A::add(s.ad_value(290), s.ad_value(791)));s.store_scale(339, 335, p[63]);s.store_sub_from_scalar(343, 1.2, 87);s.store_add_scaled_products_indices(292, 791, 339, 1.0, 338, 343, (-1.0));}
        if s.b[768] {s.store_scalar(295, (s.v[505] * (-s.v[635])));}
        s.b[3219] = (s.v[2623] == 0.0);s.store_scalar(3219, if s.b[3219] { 1.0 } else { 0.0 });
        if ((!s.b[768]) && s.b[3219]) {s.store_scalar(295, (((-s.v[188]) * p[66]) * s.v[635]));}
        s.store_mul_scale_offset_indices(297, 734, 295, -1.0, 0.0);
        if s.b[769] {s.store_scalar(294, (s.v[506] * (-s.v[635])));}
        s.b[3220] = (s.v[2626] == 0.0);s.store_scalar(3220, if s.b[3220] { 1.0 } else { 0.0 });
        if ((!s.b[769]) && s.b[3220]) {s.store_primal_scale(294, 412, (-(p[63] * s.v[635])));}
        s.store_mul_sub_scaled_inputs_rhs_indices(298, 294, 734, -1.0, 733, -1.0);s.b[3221] = (s.v[949] == 1.0);s.store_scalar(3221, if s.b[3221] { 1.0 } else { 0.0 });
        if s.b[3221] {s.store_scaled_sub(357, 790, 94, p[431]);s.store_mul(360, 338, 357);s.store_mul(361, 338, 357);}
        if (!s.b[3221]) {s.store_scaled_sub(357, 790, 94, (-p[431]));s.store_mul(362, 338, 357);s.store_mul(363, 338, 357);}
        s.store_scalar(296, ((-s.v[525]) * s.v[582]));s.store_scaled_sub(293, 731, 728, (-s.v[296]));s.store_scalar(172, s.v[507]);s.b[3222] = (s.v[78] != 0.0);s.store_scalar(3222, if s.b[3222] { 1.0 } else { 0.0 });
        if s.b[3222] {s.store_add_scaled_inputs3_indices(168, 790, s.v[172], 87, s.v[172], 91, (1.0 - s.v[172]));}
        s.b[3223] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(3223, if s.b[3223] { 1.0 } else { 0.0 });
        if (s.b[3222] && s.b[3223]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t22,) = {
    if (s.b[3222] && s.b[3223]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t22);
        let (t23,) = {
    if (s.b[3222] && s.b[3223]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t23);
        if (s.b[3222] && s.b[3223]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3224] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3224, if s.b[3224] { 1.0 } else { 0.0 });s.b[3225] = (2.0 == 1.0);s.store_scalar(3225, if s.b[3225] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (((s.b[3222] && s.b[3223]) && s.b[3224]) && s.b[3225]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t24);s.b[3226] = (2.0 == 2.0);s.store_scalar(3226, if s.b[3226] { 1.0 } else { 0.0 });
        let (t25,) = {
    if ((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && s.b[3226]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t25);s.b[3227] = (2.0 == 4.0);s.store_scalar(3227, if s.b[3227] { 1.0 } else { 0.0 });
        let (t26,) = {
    if (((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3227]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t26);s.b[3228] = (2.0 == 8.0);s.store_scalar(3228, if s.b[3228] { 1.0 } else { 0.0 });
        let (t27,) = {
    if ((((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3227])) && s.b[3228]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        let (t28,) = {
    if ((s.b[3222] && s.b[3223]) && s.b[3224]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t28);let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (((s.b[3222] && s.b[3223]) && s.b[3224]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;
            if t2c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[3222] && s.b[3223]) && s.b[3224]) {s.store_sqrt(726, 726);}
            let (t2a,) = {
    if ((s.b[3222] && s.b[3223]) && s.b[3224]) {
        let t29: f64 = (s.v[719] + 1.0);
        (t29,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2a);
        }
        if ((s.b[3222] && s.b[3223]) && (!s.b[3224])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (s.b[3222] && s.b[3223]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (s.b[3222] && s.b[3223]) {
        }
        if (s.b[3222] && (!s.b[3223])) {
        }
        if (s.b[3222] && (!s.b[3223])) {s.store_scalar(334, 1.0);}
        if (s.b[3222] && s.b[82]) {s.store_scalar(303, 0.0);}
        s.b[3229] = ((s.v[248] < 1e-15) || (s.v[348] < 1e-6));s.store_scalar(3229, if s.b[3229] { 1.0 } else { 0.0 });
        if (((!s.b[3222]) && s.b[82]) && s.b[3229]) {s.store_scalar(303, 0.0);}
        if (((!s.b[3222]) && s.b[82]) && (!s.b[3229])) {s.store_div_scaled_product_by_product_indices(303, 248, 155, 1.0, 238, 162, 1.0);}
        s.b[3230] = (!s.b[82]);s.store_scalar(3230, if s.b[3230] { 1.0 } else { 0.0 });
        if s.b[3230] {s.store_scalar(305, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_199(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[3230]) {s.store_scale(336, 684, ((1.034943e-10 * s.v[635]) * 1.3));}
        s.b[3231] = (p[133] != 0.0);s.store_scalar(3231, if s.b[3231] { 1.0 } else { 0.0 });
        if ((!s.b[3230]) && s.b[3231]) {s.store_add_scaled_product_indices(304, 87, 1.0, 303, 162, 1.0);s.store_add_scaled_inputs3_indices(335, 1439, s.v[172], 87, s.v[172], 304, (1.0 - s.v[172]));s.store_mul_scale_offset_mixed_ia(305, 336, A::add_scaled_inputs3(s.ad_value(87), 1.0, s.ad_value(1439), 1.0, s.ad_value(335), -1.0), (-1.0 / (p[133])), 0.0);}
        s.b[3232] = (p[134] != 0.0);s.store_scalar(3232, if s.b[3232] { 1.0 } else { 0.0 });
        if ((!s.b[3230]) && s.b[3232]) {s.store_add_scaled_inputs(305, 305, 1.0, 792, s.v[671]);}
        s.store_scalar(300, s.v[670]);s.store_scalar(302, s.v[670]);s.store_scaled_sub(299, 734, 733, s.v[300]);s.store_scale(301, 734, s.v[302]);s.b[3233] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3233, if s.b[3233] { 1.0 } else { 0.0 });
        if s.b[3233] {s.store_square(334, 676);s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[498])), p[495]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[498])), p[495]);}
        s.b[3234] = (p[48] > 0.0);s.store_scalar(3234, if s.b[3234] { 1.0 } else { 0.0 });s.b[3235] = (p[15] > s.v[632]);s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });
        if ((s.b[3233] && s.b[3234]) && s.b[3235]) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, (p[15] - s.v[632]));s.store_scale(876, 831, (p[15] - s.v[632]));s.store_scale(877, 836, s.v[632]);s.store_scale(878, 837, s.v[632]);}
        if ((s.b[3233] && s.b[3234]) && (!s.b[3235])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scale(877, 836, p[15]);s.store_scale(878, 837, p[15]);}
        if (s.b[3233] && (!s.b[3234])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, p[15]);s.store_scale(876, 831, p[15]);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);}
        if s.b[3233] {s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);}
        s.b[3236] = (s.v[847] > 0.0);s.store_scalar(3236, if s.b[3236] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3236]) {s.store_offset(336, 847, 1e-25);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_200(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3233] && s.b[3236]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p[512]);s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));s.store_exp_mul(851, 848, 850);}
        if s.b[3233] {s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[521])), p[518]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[521])), p[518]);}
        s.b[3237] = (p[48] > 0.0);s.store_scalar(3237, if s.b[3237] { 1.0 } else { 0.0 });s.b[3238] = (p[16] > s.v[632]);s.store_scalar(3238, if s.b[3238] { 1.0 } else { 0.0 });
        if ((s.b[3233] && s.b[3237]) && s.b[3238]) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, (p[16] - s.v[632]));s.store_scale(882, 831, (p[16] - s.v[632]));s.store_scale(883, 836, s.v[632]);s.store_scale(884, 837, s.v[632]);}
        if ((s.b[3233] && s.b[3237]) && (!s.b[3238])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scale(883, 836, p[16]);s.store_scale(884, 837, p[16]);}
        if (s.b[3233] && (!s.b[3237])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, p[16]);s.store_scale(882, 831, p[16]);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);}
        if s.b[3233] {s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);}
        s.b[3239] = (s.v[852] > 0.0);s.store_scalar(3239, if s.b[3239] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3239]) {s.store_offset(337, 852, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p[535]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_201(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[3233] && s.b[3239]) {s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));s.store_exp_mul(856, 853, 855);}
        if s.b[3233] {s.store_offset_scaled(832, 391, ((p[481]) * ((p[500] * p[13]))), (p[500] * p[13]));}
        s.b[3240] = (p[15] > s.v[632]);s.store_scalar(3240, if s.b[3240] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3240]) {s.store_offset_scaled(833, 391, ((p[483]) * ((p[501] * (p[15] - s.v[632])))), (p[501] * (p[15] - s.v[632])));s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * s.v[632]))), (p[502] * s.v[632]));}
        if (s.b[3233] && (!s.b[3240])) {s.store_scalar(833, 0.0);s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * p[15]))), (p[502] * p[15]));}
        s.b[3241] = (s.v[832] < 0.0);s.store_scalar(3241, if s.b[3241] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3241]) {s.store_scalar(832, 0.0);}
        s.b[3242] = (s.v[833] < 0.0);s.store_scalar(3242, if s.b[3242] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3242]) {s.store_scalar(833, 0.0);}
        s.b[3243] = (s.v[834] < 0.0);s.store_scalar(3243, if s.b[3243] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3243]) {s.store_scalar(834, 0.0);}
        if s.b[3233] {s.store_sub_from_scalar_scaled_input(841, p[506], 391, p[487]);s.store_sub_from_scalar_scaled_input(842, p[507], 391, p[489]);s.store_sub_from_scalar_scaled_input(843, p[508], 391, p[491]);}
        s.b[3244] = ((s.v[841] < 0.01) && (p[13] > 0.0));s.store_scalar(3244, if s.b[3244] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3244]) {s.store_scalar(841, 0.01);}
        s.b[3245] = ((s.v[842] < 0.01) && (p[15] > s.v[632]));s.store_scalar(3245, if s.b[3245] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3245]) {s.store_scalar(842, 0.01);}
        s.b[3246] = ((s.v[843] < 0.01) && (p[15] > 0.0));s.store_scalar(3246, if s.b[3246] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3246]) {s.store_scalar(843, 0.01);}
        if s.b[3233] {s.store_offset_scaled(835, 391, ((p[482]) * ((p[523] * p[14]))), (p[523] * p[14]));}
        s.b[3247] = (p[16] > s.v[632]);s.store_scalar(3247, if s.b[3247] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3247]) {s.store_offset_scaled(838, 391, ((p[484]) * ((p[524] * (p[16] - s.v[632])))), (p[524] * (p[16] - s.v[632])));s.store_offset_scaled(839, 391, ((p[486]) * ((p[525] * s.v[632]))), (p[525] * s.v[632]));}
        if (s.b[3233] && (!s.b[3247])) {s.store_scalar(838, 0.0);s.store_offset_scaled(839, 391, ((p[486]) * ((p[525] * p[16]))), (p[525] * p[16]));}
        s.b[3248] = (s.v[835] < 0.0);s.store_scalar(3248, if s.b[3248] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3248]) {s.store_scalar(835, 0.0);}
        s.b[3249] = (s.v[838] < 0.0);s.store_scalar(3249, if s.b[3249] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3249]) {s.store_scalar(838, 0.0);}
        s.b[3250] = (s.v[839] < 0.0);s.store_scalar(3250, if s.b[3250] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3250]) {s.store_scalar(839, 0.0);}
        if s.b[3233] {s.store_sub_from_scalar_scaled_input(844, p[529], 391, p[488]);s.store_sub_from_scalar_scaled_input(845, p[530], 391, p[490]);s.store_sub_from_scalar_scaled_input(846, p[531], 391, p[492]);}
        s.b[3251] = ((s.v[844] < 0.01) && (p[14] > 0.0));s.store_scalar(3251, if s.b[3251] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3251]) {s.store_scalar(844, 0.01);}
        s.b[3252] = ((s.v[845] < 0.01) && (p[16] > s.v[632]));s.store_scalar(3252, if s.b[3252] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3252]) {s.store_scalar(845, 0.01);}
        s.b[3253] = ((s.v[846] < 0.01) && (p[16] > 0.0));s.store_scalar(3253, if s.b[3253] { 1.0 } else { 0.0 });
        if (s.b[3233] && s.b[3253]) {s.store_scalar(846, 0.01);}
        if (!s.b[3233]) {s.store_scalar(387, (ctx_temp + p[11]));}
        s.store_scale(344, 850, p[511]);s.store_scale(343, 849, p[510]);s.b[3254] = (s.v[873] > 0.0);s.store_scalar(3254, if s.b[3254] { 1.0 } else { 0.0 });
        if s.b[3254] {s.store_mul(334, 874, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3255] = (s.v[860] < s.v[848]);s.store_scalar(3255, if s.b[3255] { 1.0 } else { 0.0 });
        if (s.b[3254] && s.b[3255]) {s.store_mul(332, 860, 850);}
        s.b[3256] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3256, if s.b[3256] { 1.0 } else { 0.0 });
        if ((s.b[3254] && s.b[3255]) && s.b[3256]) {s.store_scalar(335, 0.0);}
        if ((s.b[3254] && s.b[3255]) && (!s.b[3256])) {s.store_exp(335, 332);}
        if (s.b[3254] && s.b[3255]) {s.store_add_ad(885, A::add_scaled_products(s.ad_value(873), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (s.b[3254] && (!s.b[3255])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 873, 850, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_202(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3254] && (!s.b[3255])) {s.store_add_ad(885, A::add_scaled_products3(s.ad_value(873), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (!s.b[3254]) {s.store_scalar(885, 0.0);}
        s.store_scale(346, 874, p[514]);s.store_add_scaled_product_indices(885, 885, 1.0, 346, 860, 1.0);s.b[3257] = (s.v[875] > 0.0);s.store_scalar(3257, if s.b[3257] { 1.0 } else { 0.0 });
        if s.b[3257] {s.store_mul(334, 876, 343);s.store_mul_scale_offset_indices(332, 344, 860, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3258] = (s.v[860] < s.v[848]);s.store_scalar(3258, if s.b[3258] { 1.0 } else { 0.0 });
        if (s.b[3257] && s.b[3258]) {s.store_mul(332, 860, 850);}
        s.b[3259] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3259, if s.b[3259] { 1.0 } else { 0.0 });
        if ((s.b[3257] && s.b[3258]) && s.b[3259]) {s.store_scalar(335, 0.0);}
        if ((s.b[3257] && s.b[3258]) && (!s.b[3259])) {s.store_exp(335, 332);}
        if (s.b[3257] && s.b[3258]) {s.store_add_ad(887, A::add_scaled_products(s.ad_value(875), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (s.b[3257] && (!s.b[3258])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 875, 850, 335);s.store_add_ad(887, A::add_scaled_products3(s.ad_value(875), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (!s.b[3257]) {s.store_scalar(887, 0.0);}
        s.store_scale(346, 876, p[514]);s.store_add_scaled_product_indices(887, 887, 1.0, 346, 860, 1.0);s.b[3260] = (p[48] > 0.0);s.store_scalar(3260, if s.b[3260] { 1.0 } else { 0.0 });s.b[3261] = (s.v[877] > 0.0);s.store_scalar(3261, if s.b[3261] { 1.0 } else { 0.0 });
        if (s.b[3260] && s.b[3261]) {s.store_mul(334, 878, 343);s.store_mul_scale_offset_indices(332, 344, 868, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3262] = (s.v[868] < s.v[848]);s.store_scalar(3262, if s.b[3262] { 1.0 } else { 0.0 });
        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {s.store_mul(332, 868, 850);}
        s.b[3263] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3263, if s.b[3263] { 1.0 } else { 0.0 });
        if (((s.b[3260] && s.b[3261]) && s.b[3262]) && s.b[3263]) {s.store_scalar(335, 0.0);}
        if (((s.b[3260] && s.b[3261]) && s.b[3262]) && (!s.b[3263])) {s.store_exp(335, 332);}
        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {s.store_add_ad(889, A::add_scaled_products(s.ad_value(877), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {s.copy_ad(335, 851);s.store_mul3_lhs(338, 877, 850, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_203(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {s.store_add_ad(889, A::add_scaled_products3(s.ad_value(877), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(868), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (s.b[3260] && (!s.b[3261])) {s.store_scalar(889, 0.0);}
        if s.b[3260] {s.store_scale(346, 878, p[514]);s.store_add_scaled_product_indices(889, 889, 1.0, 346, 868, 1.0);}
        if (!s.b[3260]) {s.store_scalar(889, 0.0);}
        s.store_scale(344, 855, p[534]);s.store_scale(343, 854, p[533]);s.b[3264] = (s.v[879] > 0.0);s.store_scalar(3264, if s.b[3264] { 1.0 } else { 0.0 });
        if s.b[3264] {s.store_mul(334, 880, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3265] = (s.v[859] < s.v[853]);s.store_scalar(3265, if s.b[3265] { 1.0 } else { 0.0 });
        if (s.b[3264] && s.b[3265]) {s.store_mul(332, 859, 855);}
        s.b[3266] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3266, if s.b[3266] { 1.0 } else { 0.0 });
        if ((s.b[3264] && s.b[3265]) && s.b[3266]) {s.store_scalar(335, 0.0);}
        if ((s.b[3264] && s.b[3265]) && (!s.b[3266])) {s.store_exp(335, 332);}
        if (s.b[3264] && s.b[3265]) {s.store_add_ad(886, A::add_scaled_products(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3264] && (!s.b[3265])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 879, 855, 335);s.store_add_ad(886, A::add_scaled_products3(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (!s.b[3264]) {s.store_scalar(886, 0.0);}
        s.store_scale(346, 880, p[537]);s.store_add_scaled_product_indices(886, 886, 1.0, 346, 859, 1.0);s.b[3267] = (s.v[881] > 0.0);s.store_scalar(3267, if s.b[3267] { 1.0 } else { 0.0 });
        if s.b[3267] {s.store_mul(334, 882, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3268] = (s.v[859] < s.v[853]);s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });
        if (s.b[3267] && s.b[3268]) {s.store_mul(332, 859, 855);}
        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3269, if s.b[3269] { 1.0 } else { 0.0 });
        if ((s.b[3267] && s.b[3268]) && s.b[3269]) {s.store_scalar(335, 0.0);}
        if ((s.b[3267] && s.b[3268]) && (!s.b[3269])) {s.store_exp(335, 332);}
        if (s.b[3267] && s.b[3268]) {s.store_add_ad(888, A::add_scaled_products(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3267] && (!s.b[3268])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 881, 855, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_204(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3267] && (!s.b[3268])) {s.store_add_ad(888, A::add_scaled_products3(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (!s.b[3267]) {s.store_scalar(888, 0.0);}
        s.store_scale(346, 882, p[537]);s.store_add_scaled_product_indices(888, 888, 1.0, 346, 859, 1.0);s.b[3270] = (p[48] > 0.0);s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });s.b[3271] = (s.v[883] > 0.0);s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });
        if (s.b[3270] && s.b[3271]) {s.store_mul(334, 884, 343);s.store_mul_scale_offset_indices(332, 344, 867, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3272] = (s.v[867] < s.v[853]);s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });
        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {s.store_mul(332, 867, 855);}
        s.b[3273] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });
        if (((s.b[3270] && s.b[3271]) && s.b[3272]) && s.b[3273]) {s.store_scalar(335, 0.0);}
        if (((s.b[3270] && s.b[3271]) && s.b[3272]) && (!s.b[3273])) {s.store_exp(335, 332);}
        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {s.store_add_ad(890, A::add_scaled_products(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 883, 855, 335);s.store_add_ad(890, A::add_scaled_products3(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(867), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3270] && (!s.b[3271])) {s.store_scalar(890, 0.0);}
        if s.b[3270] {s.store_scale(346, 884, p[537]);s.store_add_scaled_product_indices(890, 890, 1.0, 346, 867, 1.0);}
        if (!s.b[3270]) {s.store_scalar(890, 0.0);}
        s.b[3274] = (s.v[832] > 0.0);s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });s.b[3275] = (s.v[860] < 0.0);s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });
        if (s.b[3274] && s.b[3275]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 841);}
        s.b[3276] = (p[503] == 0.5);s.store_scalar(3276, if s.b[3276] { 1.0 } else { 0.0 });
        if ((s.b[3274] && s.b[3275]) && s.b[3276]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3274] && s.b[3275]) && (!s.b[3276])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[503]));
            }
        }
        if (s.b[3274] && s.b[3275]) {s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[503])), 0.0);}
        if (s.b[3274] && (!s.b[3275])) {s.copy_ad(335, 832);s.store_div_scaled_inputs_indices(336, 832, p[503], 841, 1.0);s.store_mul_add_scaled_product_rhs_indices(891, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3274]) {s.store_scalar(891, 0.0);}
        s.b[3277] = (s.v[833] > 0.0);s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });s.b[3278] = (s.v[860] < 0.0);s.store_scalar(3278, if s.b[3278] { 1.0 } else { 0.0 });
        if (s.b[3277] && s.b[3278]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 842);}
        s.b[3279] = (p[504] == 0.5);s.store_scalar(3279, if s.b[3279] { 1.0 } else { 0.0 });
        if ((s.b[3277] && s.b[3278]) && s.b[3279]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_205(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3277] && s.b[3278]) && (!s.b[3279])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[504]));
            }
        }
        if (s.b[3277] && s.b[3278]) {s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[504])), 0.0);}
        if (s.b[3277] && (!s.b[3278])) {s.copy_ad(335, 833);s.store_div_scaled_inputs_indices(336, 833, p[504], 842, 1.0);s.store_mul_add_scaled_product_rhs_indices(893, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3277]) {s.store_scalar(893, 0.0);}
        s.b[3280] = (p[48] > 0.0);s.store_scalar(3280, if s.b[3280] { 1.0 } else { 0.0 });s.b[3281] = (s.v[834] > 0.0);s.store_scalar(3281, if s.b[3281] { 1.0 } else { 0.0 });s.b[3282] = (s.v[868] < 0.0);s.store_scalar(3282, if s.b[3282] { 1.0 } else { 0.0 });
        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);}
        s.b[3283] = (p[505] == 0.5);s.store_scalar(3283, if s.b[3283] { 1.0 } else { 0.0 });
        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && s.b[3283]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && (!s.b[3283])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if ((s.b[3280] && s.b[3281]) && (!s.b[3282])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 868, 335, 1.0, 868, 336, 0.5);}
        if (s.b[3280] && (!s.b[3281])) {s.store_scalar(895, 0.0);}
        s.b[3284] = (s.v[834] > 0.0);s.store_scalar(3284, if s.b[3284] { 1.0 } else { 0.0 });s.b[3285] = (s.v[860] < 0.0);s.store_scalar(3285, if s.b[3285] { 1.0 } else { 0.0 });
        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 843);}
        s.b[3286] = (p[505] == 0.5);s.store_scalar(3286, if s.b[3286] { 1.0 } else { 0.0 });
        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && s.b[3286]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && (!s.b[3286])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if (((!s.b[3280]) && s.b[3284]) && (!s.b[3285])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 860, 335, 1.0, 860, 336, 0.5);}
        if ((!s.b[3280]) && (!s.b[3284])) {s.store_scalar(895, 0.0);}
        s.b[3287] = (s.v[835] > 0.0);s.store_scalar(3287, if s.b[3287] { 1.0 } else { 0.0 });s.b[3288] = (s.v[859] < 0.0);s.store_scalar(3288, if s.b[3288] { 1.0 } else { 0.0 });
        if (s.b[3287] && s.b[3288]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 844);}
        s.b[3289] = (p[526] == 0.5);s.store_scalar(3289, if s.b[3289] { 1.0 } else { 0.0 });
        if ((s.b[3287] && s.b[3288]) && s.b[3289]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3287] && s.b[3288]) && (!s.b[3289])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[526]));
            }
        }
        if (s.b[3287] && s.b[3288]) {s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[526])), 0.0);}
        if (s.b[3287] && (!s.b[3288])) {s.copy_ad(335, 835);s.store_div_scaled_inputs_indices(336, 835, p[526], 844, 1.0);s.store_mul_add_scaled_product_rhs_indices(892, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3287]) {s.store_scalar(892, 0.0);}
        s.b[3290] = (s.v[838] > 0.0);s.store_scalar(3290, if s.b[3290] { 1.0 } else { 0.0 });s.b[3291] = (s.v[859] < 0.0);s.store_scalar(3291, if s.b[3291] { 1.0 } else { 0.0 });
        if (s.b[3290] && s.b[3291]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 845);}
        s.b[3292] = (p[527] == 0.5);s.store_scalar(3292, if s.b[3292] { 1.0 } else { 0.0 });
        if ((s.b[3290] && s.b[3291]) && s.b[3292]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_206(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3290] && s.b[3291]) && (!s.b[3292])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[527]));
            }
        }
        if (s.b[3290] && s.b[3291]) {s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[527])), 0.0);}
        if (s.b[3290] && (!s.b[3291])) {s.copy_ad(335, 838);s.store_div_scaled_inputs_indices(336, 838, p[527], 845, 1.0);s.store_mul_add_scaled_product_rhs_indices(894, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3290]) {s.store_scalar(894, 0.0);}
        s.b[3293] = (p[48] > 0.0);s.store_scalar(3293, if s.b[3293] { 1.0 } else { 0.0 });s.b[3294] = (s.v[839] > 0.0);s.store_scalar(3294, if s.b[3294] { 1.0 } else { 0.0 });s.b[3295] = (s.v[867] < 0.0);s.store_scalar(3295, if s.b[3295] { 1.0 } else { 0.0 });
        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {s.store_sub_from_scalar_div_indices(770, 1.0, 867, 846);}
        s.b[3296] = (p[528] == 0.5);s.store_scalar(3296, if s.b[3296] { 1.0 } else { 0.0 });
        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && s.b[3296]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && (!s.b[3296])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if ((s.b[3293] && s.b[3294]) && (!s.b[3295])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 867, 335, 1.0, 867, 336, 0.5);}
        if (s.b[3293] && (!s.b[3294])) {s.store_scalar(896, 0.0);}
        s.b[3297] = (s.v[839] > 0.0);s.store_scalar(3297, if s.b[3297] { 1.0 } else { 0.0 });s.b[3298] = (s.v[859] < 0.0);s.store_scalar(3298, if s.b[3298] { 1.0 } else { 0.0 });
        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 846);}
        s.b[3299] = (p[528] == 0.5);s.store_scalar(3299, if s.b[3299] { 1.0 } else { 0.0 });
        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && s.b[3299]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && (!s.b[3299])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if (((!s.b[3293]) && s.b[3297]) && (!s.b[3298])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 859, 335, 1.0, 859, 336, 0.5);}
        if ((!s.b[3293]) && (!s.b[3297])) {s.store_scalar(896, 0.0);}
        s.store_scaled_add(862, 886, 888, s.v[365]);s.store_scaled_add(861, 885, 887, s.v[365]);s.b[3300] = (p[48] > 0.0);s.store_scalar(3300, if s.b[3300] { 1.0 } else { 0.0 });
        if s.b[3300] {s.store_scale(870, 890, s.v[365]);s.store_scale(869, 889, s.v[365]);s.store_scaled_add(66, 892, 894, s.v[365]);s.store_scaled_add(65, 891, 893, s.v[365]);s.store_scale(68, 896, s.v[365]);s.store_scale(67, 895, s.v[365]);}
        if (!s.b[3300]) {s.store_scalar(870, 0.0);s.store_scalar(869, 0.0);s.store_add_scaled_inputs3_indices(66, 892, s.v[365], 894, s.v[365], 896, s.v[365]);s.store_add_scaled_inputs3_indices(65, 891, s.v[365], 893, s.v[365], 895, s.v[365]);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);}
        s.store_scalar(903, (p[540] / 1e-6));s.store_scalar(906, s.v[820]);s.store_scalar(904, (1450.0 / 10000.0));s.store_scalar(905, (500.0 / 10000.0));s.store_scalar(943, 0.001);s.store_scale_ad(908, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), 1.45e16);s.store_scaled_square(907, 908, 1.0 / (s.v[903]));s.store_powf(335, 676, (-1.5));s.store_scaled_mul(909, 335, 155, s.v[904]);s.store_scaled_mul(910, 335, 155, s.v[905]);s.store_div_scaled_product_add_scaled_denominator_indices(911, 909, 910, 2.0, 909, 1.0, 910, 1.0, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_207(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_powf(336, 676, p[547]);s.store_scale(913, 336, p[544]);s.store_sqrt_mul(912, 913, 911);s.store_mul_scaled_ln_ad_rhs(934, 155, s.v[906], A::div_from_scalar(s.v[903], s.ad_value(907)));s.store_mul_add_scaled_inputs_rhs(935, 155, A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), s.v[906], A::div_from_scalar(p[545], s.ad_value(912)), s.v[906]);s.b[3301] = (p[539] > 0.0);s.store_scalar(3301, if s.b[3301] { 1.0 } else { 0.0 });
        if s.b[3301] {s.store_scalar(936, s.v[820]);s.store_exp_mul(937, 860, 850);}
        s.b[3302] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);s.store_scalar(3302, if s.b[3302] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3302]) {s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div_scaled_inputs2(s.ad_value(935), 1.0, s.ad_value(934), (-1.0), s.ad_value(936), 1.0))));}
        if (s.b[3301] && (!s.b[3302])) {s.store_scalar(938, 1.0);}
        s.b[3303] = ((p[542] == 0.0) || (s.v[860] < s.v[934]));s.store_scalar(3303, if s.b[3303] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3303]) {s.store_scale(941, 937, p[541]);}
        if (s.b[3301] && (!s.b[3303])) {s.store_mul_scaled_exp_ad_rhs(941, 937, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3301] {
            if (s.v[941] > 1e20) {
                s.store_scalar(941, 1e20);
            } else {
            }
        }
        if s.b[3301] {s.store_mul(939, 907, 941);s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p[13]));}
        s.b[3304] = (p[543] > 0.0);s.store_scalar(3304, if s.b[3304] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3304]) {s.store_scale(922, 920, p[543]);s.store_scaled_voltage(924, ctx, nodes, Some(16), None, p[543]);s.store_scaled_sub(926, 924, 922, 1.0 / (p[543]));s.store_scale(928, 924, 1.0 / (p[543]));}
        if (s.b[3301] && (!s.b[3304])) {s.copy_ad(922, 920);s.copy_ad(928, 922);}
        s.b[3305] = ((p[542] == 0.0) || (s.v[860] < s.v[935]));s.store_scalar(3305, if s.b[3305] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3305]) {s.store_scale(942, 938, p[541]);}
        if (s.b[3301] && (!s.b[3305])) {s.store_mul_scaled_exp_ad_rhs(942, 938, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3301] {
            if (s.v[942] > 1e20) {
                s.store_scalar(942, 1e20);
            } else {
            }
        }
        if s.b[3301] {s.store_mul(940, 907, 942);s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p[13]));}
        s.b[3306] = (p[543] > 0.0);s.store_scalar(3306, if s.b[3306] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3306]) {s.store_scale(923, 921, p[543]);s.store_scaled_voltage(925, ctx, nodes, Some(17), None, p[543]);s.store_scaled_sub(927, 925, 923, 1.0 / (p[543]));s.store_scale(929, 925, 1.0 / (p[543]));}
        if (s.b[3301] && (!s.b[3306])) {s.copy_ad(923, 921);s.copy_ad(929, 923);}
        if s.b[3301] {s.store_sub_from_scalar(914, p[506], 860);s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);s.store_scaled_add(914, 914, 782, 0.5);}
        s.b[3307] = (s.v[914] < 0.0);s.store_scalar(3307, if s.b[3307] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3307]) {s.store_scalar(914, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3301] {s.store_sqrt_scaled_input(915, 914, ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903]))));}
    }
}
