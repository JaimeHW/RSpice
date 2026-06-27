#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign93970_loop_guard: usize = 0;
        while {
            let assign93970_cond_e143968: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93970_cond_e143968 != 0.0
        } {
            assign93970_loop_guard += 1;
            assert!(assign93970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {
                s.store_sqrt(726, 726);
            }
            let (assign93970_body1_e144005,) = {
    if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3166]) && s.b[3167]) && s.b[3168]) && s.b[3169]) {
        let assign93970_body1_e144003: f64 = (s.v[719] + 1.0);
        (assign93970_body1_e144003,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign93970_body1_e144005;
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

        let (assign94090_e144199,) = {
    if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign94090_e144199;

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
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
        }

        let (assign94320_e144548,) = {
    if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign94320_e144548;

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
            let (assign94330_body72_e145832,) = {
    if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && s.b[3188]) {
        let assign94330_body72_e145830: f64 = (s.v[421] + 1.0);
        (assign94330_body72_e145830,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign94330_body72_e145832;
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
            let (assign94330_body79_e145945,) = {
    if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) && (!s.b[3188])) && s.b[3190]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign94330_body79_e145945;
            let (assign94330_body80_e145958,) = {
    if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3174]) {
        let assign94330_body80_e145956: f64 = (s.v[97] + 1.0);
        (assign94330_body80_e145956,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign94330_body80_e145958;
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
        }

        let (assign94480_e146121,) = {
    if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign94480_e146121;

        if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
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

    }

    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
        }

        let (assign94710_e146410,) = {
    if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign94710_e146410;

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
            let (assign94720_body56_e147259,) = {
    if (((s.b[3107] && s.b[3108]) && s.b[3192]) && s.b[3203]) {
        let assign94720_body56_e147257: f64 = (s.v[421] + 1.0);
        (assign94720_body56_e147257,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign94720_body56_e147259;
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
            let (assign94720_body63_e147359,) = {
    if ((((s.b[3107] && s.b[3108]) && s.b[3192]) && (!s.b[3203])) && s.b[3205]) {
        let assign94720_body63_e147357: f64 = (s.v[79] + 2.0);
        (assign94720_body63_e147357,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign94720_body63_e147359;
            let (assign94720_body64_e147369,) = {
    if ((s.b[3107] && s.b[3108]) && s.b[3192]) {
        let assign94720_body64_e147367: f64 = (s.v[97] + 1.0);
        (assign94720_body64_e147367,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign94720_body64_e147369;
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

        let (assign95050_e147751,) = {
    if s.b[3211] {
        (1.0,)
    } else {
        (s.v[2619],)
    }
};
        s.v[2619] = assign95050_e147751;

        if s.b[3211] {
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
            s.store_sub_offset_lhs(343, 790, 1.2, 91);
            s.store_add_scaled_products_left_left_ad(291, A::sub(s.ad_value(791), s.ad_value(790)), 339, 1.0, 338, 343, (-1.0));
        }

        s.b[3213] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] == 0.0));
        s.v[3213] = if s.b[3213] { 1.0 } else { 0.0 };

        let (assign95190_e147873,) = {
    if s.b[3213] {
        (1.0,)
    } else {
        (s.v[2622],)
    }
};
        s.v[2622] = assign95190_e147873;

        if s.b[3213] {
            s.store_scalar(289, s.v[564]);
            s.store_scalar(290, p.p276);
            s.store_scale(335, 412, s.v[635]);
        }

        s.b[3214] = (s.v[949] == 1.0);
        s.v[3214] = if s.b[3214] { 1.0 } else { 0.0 };

        if (s.b[3213] && s.b[3214]) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p63);
            s.store_sub_offset_lhs(343, 790, 1.2, 91);
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

    }

    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_add_scaled_inputs3(168, s.ad_value(790), s.v[172], s.ad_value(87), s.v[172], s.ad_value(91), (1.0 - s.v[172]));
        }

        s.b[3219] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[3219] = if s.b[3219] { 1.0 } else { 0.0 };

        if (s.b[3218] && s.b[3219]) {
            s.store_offset_add_scaled_inputs3_offset(781, s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign95580_e148184,) = {
    if (s.b[3218] && s.b[3219]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign95580_e148184;

        let (assign95590_e148190,) = {
    if (s.b[3218] && s.b[3219]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign95590_e148190;

        if (s.b[3218] && s.b[3219]) {
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

        let (assign95700_e148276,) = {
    if (((s.b[3218] && s.b[3219]) && s.b[3220]) && s.b[3221]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign95700_e148276;

        s.b[3222] = (2.0 == 2.0);
        s.v[3222] = if s.b[3222] { 1.0 } else { 0.0 };

        let (assign95720_e148292,) = {
    if ((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && s.b[3222]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign95720_e148292;

        s.b[3223] = (2.0 == 4.0);
        s.v[3223] = if s.b[3223] { 1.0 } else { 0.0 };

        let (assign95740_e148311,) = {
    if (((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && s.b[3223]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign95740_e148311;

        s.b[3224] = (2.0 == 8.0);
        s.v[3224] = if s.b[3224] { 1.0 } else { 0.0 };

        let (assign95760_e148333,) = {
    if ((((((s.b[3218] && s.b[3219]) && s.b[3220]) && (!s.b[3221])) && (!s.b[3222])) && (!s.b[3223])) && s.b[3224]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign95760_e148333;

        let (assign95770_e148341,) = {
    if ((s.b[3218] && s.b[3219]) && s.b[3220]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign95770_e148341;

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if (((s.b[3218] && s.b[3219]) && s.b[3220]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3218] && s.b[3219]) && s.b[3220]) {
                s.store_sqrt(726, 726);
            }
            let (assign95780_body1_e148369,) = {
    if ((s.b[3218] && s.b[3219]) && s.b[3220]) {
        let assign95780_body1_e148367: f64 = (s.v[719] + 1.0);
        (assign95780_body1_e148367,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign95780_body1_e148369;
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
        }

    }

    pub(super) fn stamp_transient_block_99(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[3229] && s.b[3233]) && (!s.b[3234])) {
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

    }

    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);
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
        }

    }

    pub(super) fn stamp_transient_block_101(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3296] {
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
        }

        let (assign100520_e152627,) = {
    if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign100520_e152627;

        let (assign100530_e152635,) = {
    if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign100530_e152635;

        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
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

        let (assign100680_e152779,) = {
    if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && s.b[3311]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign100680_e152779;

        s.b[3312] = (4.0 == 2.0);
        s.v[3312] = if s.b[3312] { 1.0 } else { 0.0 };

        let (assign100700_e152797,) = {
    if (((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && s.b[3312]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign100700_e152797;

        s.b[3313] = (4.0 == 4.0);
        s.v[3313] = if s.b[3313] { 1.0 } else { 0.0 };

        let (assign100720_e152818,) = {
    if ((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && s.b[3313]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign100720_e152818;

        s.b[3314] = (4.0 == 8.0);
        s.v[3314] = if s.b[3314] { 1.0 } else { 0.0 };

        let (assign100740_e152842,) = {
    if (((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && (!s.b[3313])) && s.b[3314]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign100740_e152842;

        let (assign100750_e152852,) = {
    if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign100750_e152852;

        let mut assign100760_loop_guard: usize = 0;
        while {
            let assign100760_cond_e152863: f64 = if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign100760_cond_e152863 != 0.0
        } {
            assign100760_loop_guard += 1;
            assert!(assign100760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
                s.store_sqrt(726, 726);
            }
            let (assign100760_body1_e152886,) = {
    if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
        let assign100760_body1_e152884: f64 = (s.v[719] + 1.0);
        (assign100760_body1_e152884,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign100760_body1_e152886;
        }

    }

    pub(super) fn stamp_transient_block_102(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
            s.store_mul_product3_rhs(335, 337, s.ad_value(346), s.ad_value(344), s.ad_value(337), 1.0);
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

        if s.b[3318] {
            s.store_scalar(309, s.v[522]);
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
        }

    }

    pub(super) fn stamp_transient_block_103(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[3327] {
            s.store_scale(706, 358, s.v[365]);
            s.store_scale(707, 359, s.v[365]);
            s.store_offset_sub_scaled_inputs(703, s.ad_value(299), (-s.v[365]), s.ad_value(298), s.v[365], s.v[703]);
            s.store_offset_sub_scaled_inputs(704, s.ad_value(301), (-s.v[365]), s.ad_value(297), s.v[365], s.v[704]);
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
            s.store_mul_product3_rhs(613, 611, s.ad_value(335), s.ad_value(612), s.ad_value(610), 1.0);
        }

        s.b[3340] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));
        s.v[3340] = if s.b[3340] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_sub_from_scalar(781, 1e-25, 613);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign103030_e154919,) = {
    if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign103030_e154919;

        let (assign103040_e154928,) = {
    if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign103040_e154928;

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
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

        let (assign103150_e155041,) = {
    if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && s.b[3342]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign103150_e155041;

        s.b[3343] = (2.0 == 2.0);
        s.v[3343] = if s.b[3343] { 1.0 } else { 0.0 };

        let (assign103170_e155060,) = {
    if (((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && s.b[3343]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign103170_e155060;

        s.b[3344] = (2.0 == 4.0);
        s.v[3344] = if s.b[3344] { 1.0 } else { 0.0 };

        let (assign103190_e155082,) = {
    if ((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && s.b[3344]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign103190_e155082;

        s.b[3345] = (2.0 == 8.0);
        s.v[3345] = if s.b[3345] { 1.0 } else { 0.0 };

        let (assign103210_e155107,) = {
    if (((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && (!s.b[3344])) && s.b[3345]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign103210_e155107;

        let (assign103220_e155118,) = {
    if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign103220_e155118;

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
                s.store_sqrt(726, 726);
            }
            let (assign103230_body1_e155155,) = {
    if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
        let assign103230_body1_e155153: f64 = (s.v[719] + 1.0);
        (assign103230_body1_e155153,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign103230_body1_e155155;
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

    }

    pub(super) fn stamp_transient_block_104(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
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
        }

        let (assign104340_e156529,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign104340_e156529;

        let (assign104350_e156540,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign104350_e156540;

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign104380_e156573,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign104380_e156573;

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
            }
            let (assign104390_body2_e156624,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        let assign104390_body2_e156622: f64 = (s.v[719] + 1.0);
        (assign104390_body2_e156622,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign104390_body2_e156624;
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3369] = ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0));
        s.v[3369] = if s.b[3369] { 1.0 } else { 0.0 };

        s.b[3370] = (p.p442 == 1.0);
        s.v[3370] = if s.b[3370] { 1.0 } else { 0.0 };

        let (assign104440_e156681,) = {
    if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && s.b[3370]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign104440_e156681;

        s.b[3371] = (p.p442 == 2.0);
        s.v[3371] = if s.b[3371] { 1.0 } else { 0.0 };

        let (assign104460_e156702,) = {
    if ((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && s.b[3371]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign104460_e156702;

        s.b[3372] = (p.p442 == 4.0);
        s.v[3372] = if s.b[3372] { 1.0 } else { 0.0 };

        let (assign104480_e156726,) = {
    if (((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && s.b[3372]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign104480_e156726;

        s.b[3373] = (p.p442 == 8.0);
        s.v[3373] = if s.b[3373] { 1.0 } else { 0.0 };

        let (assign104500_e156753,) = {
    if ((((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && (!s.b[3372])) && s.b[3373]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign104500_e156753;

    }

    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign104510_e156766,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign104510_e156766;

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
                s.store_sqrt(726, 726);
            }
            let (assign104520_body1_e156809,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
        let assign104520_body1_e156807: f64 = (s.v[719] + 1.0);
        (assign104520_body1_e156807,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign104520_body1_e156809;
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
            s.store_mul_product3_rhs(739, 597, s.ad_value(335), s.ad_value(596), s.ad_value(3350), 1.0);
        }

        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));
        s.v[3377] = if s.b[3377] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_sub_from_scalar(781, 1e-25, 739);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign105020_e157402,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign105020_e157402;

        let (assign105030_e157411,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105030_e157411;

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
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

        let (assign105140_e157524,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && s.b[3379]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105140_e157524;

        s.b[3380] = (2.0 == 2.0);
        s.v[3380] = if s.b[3380] { 1.0 } else { 0.0 };

        let (assign105160_e157543,) = {
    if (((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && s.b[3380]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105160_e157543;

        s.b[3381] = (2.0 == 4.0);
        s.v[3381] = if s.b[3381] { 1.0 } else { 0.0 };

        let (assign105180_e157565,) = {
    if ((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && s.b[3381]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105180_e157565;

        s.b[3382] = (2.0 == 8.0);
        s.v[3382] = if s.b[3382] { 1.0 } else { 0.0 };

        let (assign105200_e157590,) = {
    if (((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && (!s.b[3381])) && s.b[3382]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105200_e157590;

        let (assign105210_e157601,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign105210_e157601;

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
                s.store_sqrt(726, 726);
            }
            let (assign105220_body1_e157638,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
        let assign105220_body1_e157636: f64 = (s.v[719] + 1.0);
        (assign105220_body1_e157636,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign105220_body1_e157638;
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
        }

        let (assign105390_e157831,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign105390_e157831;

        let (assign105400_e157840,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105400_e157840;

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
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

        let (assign105510_e157953,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && s.b[3385]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105510_e157953;

        s.b[3386] = (2.0 == 2.0);
        s.v[3386] = if s.b[3386] { 1.0 } else { 0.0 };

        let (assign105530_e157972,) = {
    if (((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && s.b[3386]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105530_e157972;

        s.b[3387] = (2.0 == 4.0);
        s.v[3387] = if s.b[3387] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign105550_e157994,) = {
    if ((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && s.b[3387]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105550_e157994;

        s.b[3388] = (2.0 == 8.0);
        s.v[3388] = if s.b[3388] { 1.0 } else { 0.0 };

        let (assign105570_e158019,) = {
    if (((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && (!s.b[3387])) && s.b[3388]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign105570_e158019;

        let (assign105580_e158030,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign105580_e158030;

        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
                s.store_sqrt(726, 726);
            }
            let (assign105590_body1_e158067,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
        let assign105590_body1_e158065: f64 = (s.v[719] + 1.0);
        (assign105590_body1_e158065,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign105590_body1_e158067;
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
            s.store_div_scaled_inputs(4, s.ad_value(4), s.v[165], s.ad_value(385), 1.0);
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

        s.copy_ad(201, 9);

        s.copy_ad(200, 10);

        s.copy_ad(202, 11);

        s.b[3395] = (s.v[949] > 0.0);
        s.v[3395] = if s.b[3395] { 1.0 } else { 0.0 };

        if s.b[3395] {
            s.copy_ad(134, 0);
            s.copy_ad(19, 701);
            s.copy_ad(18, 700);
            s.copy_ad(741, 702);
            s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(701), 1.0, s.ad_value(702), 1.0));
            s.copy_ad(280, 709);
            s.store_scalar(736, 0.0);
            s.copy_ad(281, 710);
            s.store_scalar(737, 0.0);
            s.copy_ad(400, 699);
            s.store_scalar(738, 0.0);
            s.copy_ad(431, 430);
            s.store_scalar(432, 0.0);
            s.copy_ad(424, 422);
            s.store_scalar(425, 0.0);
            s.copy_ad(203, 7);
            s.copy_ad(204, 8);
        }

        if (s.b[3395] && (s.v[81] != 0.0)) {
            s.copy_ad(247, 708);
        }

        if (!s.b[3395]) {
            s.store_neg(134, 0);
            s.copy_ad(19, 702);
            s.copy_ad(18, 700);
            s.copy_ad(741, 701);
            s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(701), 1.0, s.ad_value(702), 1.0));
            s.store_scalar(280, 0.0);
            s.copy_ad(736, 709);
            s.store_scalar(281, 0.0);
            s.copy_ad(737, 710);
            s.store_scalar(400, 0.0);
            s.copy_ad(738, 699);
            s.store_scalar(431, 0.0);
            s.copy_ad(432, 430);
            s.store_scalar(424, 0.0);
            s.copy_ad(425, 422);
            s.copy_ad(203, 8);
            s.copy_ad(204, 7);
        }

        if ((!s.b[3395]) && (s.v[81] != 0.0)) {
            s.store_sub_from_scalar(247, 1.0, 708);
        }

        s.store_add(18, 18, 811);

        s.store_add(19, 19, 810);

        s.store_add(741, 741, 812);

        s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(18), 1.0, s.ad_value(19), 1.0, s.ad_value(741), 1.0));

        s.copy_ad(299, 703);

        s.copy_ad(301, 704);

        s.copy_ad(742, 706);

        s.copy_ad(743, 705);

        s.store_neg_ad(744, A::add_scaled_inputs3(s.ad_value(705), 1.0, s.ad_value(706), 1.0, s.ad_value(707), 1.0));

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
            s.store_add_scaled_product_right_ad(745, 729, 1.0, 683, A::sub(s.ad_value(733), s.ad_value(729)), 1.0);
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
            s.store_add_scaled_inputs3(781, s.ad_value(335), 1.0, s.ad_value(746), (-1.0), s.ad_value(740), (-p.p337));
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
            s.store_add_scaled_inputs3(336, s.ad_value(335), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.copy_ad(746, 336);
        }

        if (!s.b[3396]) {
            s.store_scalar(740, 0.0);
            s.store_scalar(746, 0.0);
        }

        s.b[3401] = (s.v[306] < 1e-15);
        s.v[3401] = if s.b[3401] { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && s.b[3401]) {
            s.store_scalar(306, 1e-15);
        }

        s.b[3402] = (s.v[307] < 1e-15);
        s.v[3402] = if s.b[3402] { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && s.b[3402]) {
            s.store_scalar(307, 1e-15);
        }

        if (s.v[81] != 0.0) {
            s.store_div_scaled_inputs2(749, s.ad_value(747), 1.0, s.ad_value(132), (-1.0), s.ad_value(306), 1.0);
            s.store_div_scaled_inputs2(750, s.ad_value(748), 1.0, s.ad_value(754), (-1.0), s.ad_value(307), 1.0);
            s.store_mul(751, 747, 247);
            s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);
            s.store_mul_sub_from_scalar_rhs(752, 747, 1.0, 247);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(749, 0.0);
            s.store_scalar(750, 0.0);
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

        s.b[3406] = (p.p24 == 1.0);
        s.v[3406] = if s.b[3406] { 1.0 } else { 0.0 };

        s.b[3409] = (p.p53 > 0.0);
        s.v[3409] = if s.b[3409] { 1.0 } else { 0.0 };

        if s.b[3409] {
            s.copy_ad(802, 746);
        }

        if (p.p28 != 0.0) {
            s.store_scalar(800, 1.0);
            s.store_scalar(801, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[623] = param_given[12];
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        s.b[769] = param_given[268];
        s.v[769] = if s.b[769] { 1.0 } else { 0.0 };

        s.b[768] = param_given[269];
        s.v[768] = if s.b[768] { 1.0 } else { 0.0 };

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[708] = 0.0;

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.v[321] = 0.0;

        s.v[78] = 0.0;

        s.v[74] = 0.0;

        s.v[347] = 0.0;

        s.v[697] = 0.0;

        s.v[698] = 0.0;

        s.v[69] = 0.8;

        s.v[70] = 0.4;

        s.v[77] = 0.0;

        s.v[79] = 0.0;

        s.v[80] = 0.0;

        s.v[81] = 0.0;

        s.v[83] = 0.0;

        s.v[84] = 0.0;

        s.v[85] = 0.0;

        s.v[86] = 0.0;

        s.v[87] = 0.0;

        s.v[88] = 0.0;

        s.v[89] = 0.0;

        s.v[90] = 0.0;

        s.v[91] = 0.0;

        s.v[92] = 0.0;

        s.v[93] = 0.0;

        s.v[94] = 0.0;

        s.v[95] = 0.0;

        s.v[96] = 0.0;

        s.v[97] = 0.0;

        s.v[98] = 0.0;

        s.v[99] = 0.0;

        s.v[100] = 0.0;

        s.v[101] = 0.0;

        s.v[102] = 0.0;

        s.v[103] = 0.0;

        s.v[104] = 0.0;

        s.v[105] = 0.0;

        s.v[106] = 0.0;

        s.v[107] = 0.0;

        s.v[108] = 0.0;

        s.v[109] = 0.0;

        s.v[110] = 0.0;

        s.v[111] = 0.0;

        s.v[112] = 0.0;

        s.v[113] = 0.0;

        s.v[114] = 0.0;

        s.v[115] = 0.0;

        s.v[116] = 0.0;

        s.v[415] = 0.0;

        s.v[117] = 0.0;

        s.v[118] = 0.0;

        s.v[119] = 0.0;

        s.v[120] = 0.0;

        s.v[121] = 0.0;

        s.v[122] = 0.0;

        s.v[123] = 0.0;

        s.v[124] = 0.0;

        s.v[125] = 0.0;

        s.v[126] = 0.0;

        s.v[127] = 0.0;

        s.v[128] = 0.0;

        s.v[129] = 0.0;

        s.v[130] = 0.0;

        s.v[20] = 0.0;

        s.v[131] = 0.0;

        s.v[132] = 0.0;

        s.v[133] = 0.0;

        s.v[19] = 0.0;

        s.v[134] = 0.0;

        s.v[135] = 0.0;

        s.v[137] = 0.0;

        s.v[138] = 0.0;

        s.v[139] = 0.0;

        s.v[140] = 0.0;

        s.v[141] = 0.0;

        s.v[142] = 0.0;

        s.v[143] = 0.0;

        s.v[144] = 0.0;

        s.v[145] = 0.0;

        s.v[146] = 0.0;

        s.v[147] = 0.0;

        s.v[148] = 0.0;

        s.v[149] = 0.0;

        s.v[150] = 0.0;

        s.v[151] = 0.0;

        s.v[152] = 0.0;

        s.v[153] = 0.0;

        s.v[154] = 0.0;

        s.v[155] = 0.0;

        s.v[156] = 0.0;

        s.v[157] = 0.0;

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[160] = 0.0;

        s.v[161] = 0.0;

        s.v[162] = 0.0;

        s.v[163] = 0.0;

        s.v[164] = 0.0;

        s.v[165] = 0.0;

        s.v[166] = 0.0;

        s.v[167] = 0.0;

        s.v[168] = 0.0;

        s.v[169] = 0.0;

        s.v[170] = 0.0;

        s.v[171] = 0.0;

        s.v[172] = 0.0;

        s.v[173] = 0.0;

        s.v[174] = 0.0;

        s.v[175] = 0.0;

        s.v[176] = 0.0;

        s.v[177] = 0.0;

        s.v[178] = 0.0;

        s.v[179] = 0.0;

        s.v[180] = 0.0;

        s.v[181] = 0.0;

        s.v[182] = 0.0;

        s.v[184] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[412] = 0.0;

        s.v[189] = 0.0;

        s.v[190] = 0.0;

        s.v[191] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[198] = 0.0;

        s.v[205] = 0.0;

        s.v[206] = 0.0;

        s.v[207] = 0.0;

        s.v[208] = 0.0;

        s.v[209] = 0.0;

        s.v[210] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[213] = 0.0;

        s.v[214] = 0.0;

        s.v[215] = 0.0;

        s.v[216] = 0.0;

        s.v[217] = 0.0;

        s.v[218] = 0.0;

        s.v[219] = 0.0;

        s.v[220] = 0.0;

        s.v[221] = 0.0;

        s.v[222] = 0.0;

        s.v[223] = 0.0;

        s.v[224] = 0.0;

        s.v[225] = 0.0;

        s.v[226] = 0.0;

        s.v[227] = 0.0;

        s.v[228] = 0.0;

        s.v[229] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[232] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[237] = 0.0;

        s.v[238] = 0.0;

        s.v[239] = 0.0;

        s.v[240] = 0.0;

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.v[243] = 0.0;

        s.v[244] = 0.0;

        s.v[245] = 0.0;

        s.v[246] = 0.0;

        s.v[247] = 0.5;

        s.v[248] = 0.0;

        s.v[249] = 0.0;

        s.v[250] = 0.0;

        s.v[251] = 0.0;

        s.v[252] = 0.0;

        s.v[253] = 0.0;

        s.v[254] = 0.0;

        s.v[255] = 0.0;

        s.v[256] = 0.0;

        s.v[258] = 0.0;

        s.v[259] = 0.0;

        s.v[260] = 0.0;

        s.v[261] = 0.0;

        s.v[262] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[268] = 0.0;

        s.v[269] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[280] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[292] = 0.0;

        s.v[293] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[300] = 0.0;

        s.v[301] = 0.0;

        s.v[302] = 0.0;

        s.v[303] = 0.0;

        s.v[304] = 0.0;

        s.v[305] = 0.0;

        s.v[313] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[316] = 0.0;

        s.v[317] = 0.0;

        s.v[318] = 0.0;

        s.v[319] = 0.0;

        s.v[320] = 0.0;

        s.v[322] = 0.0;

        s.v[323] = 0.0;

        s.v[324] = 0.0;

        s.v[328] = 0.0;

        s.v[329] = 0.0;

        s.v[330] = 0.0;

        s.v[331] = 0.0;

        s.v[332] = 0.0;

        s.v[333] = 0.0;

        s.v[334] = 0.0;

        s.v[335] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[338] = 0.0;

        s.v[339] = 0.0;

        s.v[340] = 0.0;

        s.v[341] = 0.0;

        s.v[342] = 0.0;

        s.v[343] = 0.0;

        s.v[344] = 0.0;

        s.v[345] = 0.0;

        s.v[346] = 0.0;

        s.v[348] = 0.0;

        s.v[349] = 0.0;

        s.v[350] = 0.0;

        s.v[351] = 0.0;

        s.v[352] = 0.0;

        s.v[353] = 0.0;

        s.v[354] = 0.0;

        s.v[355] = 0.0;

        s.v[356] = 0.0;

        s.v[357] = 0.0;

        s.v[358] = 0.0;

        s.v[359] = 0.0;

        s.v[364] = 0.0;

        s.v[366] = 0.0;

        s.v[367] = 0.0;

        s.v[368] = 0.0;

        s.v[369] = 0.0;

        s.v[370] = 0.0;

        s.v[371] = 0.0;

        s.v[372] = 0.0;

        s.v[373] = 0.0;

        s.v[374] = 0.0;

        s.v[375] = 0.0;

        s.v[376] = 0.0;

        s.v[377] = 0.0;

        s.v[380] = 0.0;

        s.v[381] = 0.0;

        s.v[382] = 0.0;

        s.v[383] = 0.0;

        s.v[387] = 0.0;

        s.v[388] = 0.0;

        s.v[389] = 0.0;

        s.v[390] = 0.0;

        s.v[391] = 0.0;

        s.v[392] = 0.0;

        s.v[393] = 0.0;

        s.v[394] = 0.0;

        s.v[395] = 0.0;

        s.v[396] = 0.0;

        s.v[397] = 0.0;

        s.v[398] = 0.0;

        s.v[399] = 0.0;

        s.v[400] = 0.0;

        s.v[402] = 0.0;

        s.v[403] = 0.0;

        s.v[404] = 0.0;

        s.v[405] = 0.0;

        s.v[385] = p.p334;

        s.v[386] = p.p334;

        s.v[409] = 0.0;

        s.v[410] = 0.0;

        s.v[434] = 0.0093868;

        s.v[435] = (-0.1047839);

        s.v[447] = 0.0;

        s.v[573] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[578] = 0.0;

        s.v[579] = 0.0;

        s.v[580] = 0.0;

        s.v[581] = 0.0;

        s.v[582] = 0.0;

        s.v[583] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[586] = 0.0;

        s.v[587] = 0.0;

        s.v[588] = 0.0;

        s.v[589] = 0.0;

        s.v[590] = 0.0;

        s.v[591] = 0.0;

        s.v[592] = 0.0;

        s.v[593] = 0.0;

        s.v[594] = 0.0;

        s.v[595] = 0.0;

        s.v[596] = 0.0;

        s.v[597] = 0.0;

        s.v[739] = 0.0;

        s.v[598] = 0.0;

        s.v[770] = 0.0;

        s.v[727] = 0.0;

        s.v[728] = 0.0;

        s.v[729] = 0.0;

        s.v[730] = 0.0;

        s.v[731] = 0.0;

        s.v[732] = 0.0;

        s.v[733] = 0.0;

        s.v[734] = 0.0;

        s.v[735] = 0.0;

        s.v[740] = 0.0;

        s.v[18] = 0.0;

        s.v[741] = 0.0;

        s.v[745] = 0.0;

        s.v[746] = 0.0;

        s.v[747] = 0.0;

        s.v[748] = 0.0;

        s.v[751] = 0.0;

        s.v[752] = 0.0;

        s.v[753] = 0.0;

        s.v[757] = 0.0;

        s.v[682] = 0.0;

        s.v[688] = 0.0;

        s.v[689] = 0.0;

        s.v[787] = 0.0;

        s.v[794] = 0.0;

        s.v[788] = 0.0;

        s.v[690] = 0.0;

        s.v[692] = 0.0;

        s.v[691] = 0.0;

        s.v[693] = 0.0;

        s.v[795] = 0.0;

        s.v[676] = 0.0;

        s.v[681] = 0.0;

        s.v[678] = 0.0;

        s.v[686] = 0.0;

        s.v[687] = 0.0;

        s.v[694] = 0.0;

        s.v[679] = 0.0;

        s.v[683] = 0.0;

        s.v[680] = 0.0;

        s.v[677] = 0.0;

        s.v[684] = 0.0;

        s.v[685] = 0.0;

        s.v[956] = p.p436;

        s.v[959] = p.p437;

        s.v[986] = 0.0;

        s.v[987] = 0.0;

        s.v[988] = 0.0;

        s.v[961] = 0.0;

        s.v[960] = 0.0;

        s.v[427] = p.p447;

        s.v[957] = p.p193;

        s.v[977] = 0.0;

        s.v[978] = 0.0;

        s.v[421] = 40.0;

        s.v[828] = 0.0;

        s.v[829] = 0.0;

        s.v[830] = 0.0;

        s.v[831] = 0.0;

        s.v[66] = 0.0;

        s.v[65] = 0.0;

        s.v[68] = 0.0;

        s.v[67] = 0.0;

        s.v[832] = 0.0;

        s.v[833] = 0.0;

        s.v[834] = 0.0;

        s.v[835] = 0.0;

        s.v[838] = 0.0;

        s.v[839] = 0.0;

        s.v[841] = 0.0;

        s.v[842] = 0.0;

        s.v[843] = 0.0;

        s.v[844] = 0.0;

        s.v[845] = 0.0;

        s.v[846] = 0.0;

        s.v[840] = 0.0;

        s.v[857] = 0.0;

        s.v[858] = 0.0;

        s.v[859] = 0.0;

        s.v[860] = 0.0;

        s.v[865] = 0.0;

        s.v[866] = 0.0;

        s.v[867] = 0.0;

        s.v[868] = 0.0;

        s.v[849] = 0.0;

        s.v[854] = 0.0;

        s.v[847] = 0.0;

        s.v[852] = 0.0;

        s.v[851] = 0.0;

        s.v[856] = 0.0;

        s.v[848] = 0.0;

        s.v[853] = 0.0;

        s.v[850] = 0.0;

        s.v[855] = 0.0;

        s.v[946] = 0.0;

        s.v[944] = 0.0;

        s.v[947] = 0.0;

        s.v[945] = 0.0;

        s.v[948] = 0.0;

        s.v[816] = 0.0;

        s.v[873] = 0.0;

        s.v[874] = 0.0;

        s.v[875] = 0.0;

        s.v[876] = 0.0;

        s.v[877] = 0.0;

        s.v[878] = 0.0;

        s.v[879] = 0.0;

        s.v[880] = 0.0;

        s.v[881] = 0.0;

        s.v[882] = 0.0;

        s.v[883] = 0.0;

        s.v[884] = 0.0;

        s.v[360] = 0.0;

        s.v[362] = 0.0;

        s.v[361] = 0.0;

        s.v[363] = 0.0;

        s.v[603] = 0.0;

        s.v[45] = 0.0;

        s.v[46] = 0.0;

        s.v[413] = 0.0;

        s.v[932] = 0.0;

        s.v[926] = 0.0;

        s.v[927] = 0.0;

        s.v[287] = 0.0;

        s.v[407] = 0.0;

        s.v[924] = 0.0;

        s.v[925] = 0.0;

        s.v[931] = 0.0;

        s.v[990] = 0.0;

        s.v[411] = 0.0;

        s.v[288] = 0.0;

        s.v[448] = (if (p.p40 != 0.0) { 0.0 } else { p.p17 });

        s.v[450] = p.p104;

        s.v[451] = p.p294;

        s.v[452] = p.p222;

        s.v[453] = p.p420;

        s.v[365] = 1.0;

        s.b[1004] = (s.v[452] < 0.0);
        s.v[1004] = if s.b[1004] { 1.0 } else { 0.0 };

        if s.b[1004] {
            s.store_scalar(452, 0.0);
        }

        s.b[1005] = (s.v[452] > 0.0);
        s.v[1005] = if s.b[1005] { 1.0 } else { 0.0 };

        if s.b[1005] {
            s.store_scalar(452, 0.0);
        }

        s.b[1007] = (s.v[451] < 0.0);
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        if s.b[1007] {
            s.store_scalar(451, 0.0);
        }

        s.b[1010] = (s.v[453] < 0.0);
        s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };

        if s.b[1010] {
            s.store_scalar(453, 0.0);
        }

        s.b[1011] = (s.v[453] > 1.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if s.b[1011] {
            s.store_scalar(453, 1.0);
        }

        s.v[964] = p.p340;

        s.v[965] = p.p343;

        s.v[963] = p.p42;

        s.v[967] = p.p354;

        s.v[969] = p.p355;

        s.v[966] = p.p346;

        s.v[968] = p.p349;

        s.v[970] = p.p352;

        s.v[972] = p.p360;

        s.v[973] = p.p367;

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[976] = p.p364;

        s.v[971] = p.p377;

        s.v[974] = p.p370;

        s.v[975] = p.p371;

        s.b[1106] = ((s.v[963] < 3.0) && (s.v[963] > 0.0));
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        s.b[1109] = (s.v[964] < 5000000000000000.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1109]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1110] = (s.v[964] > 1e18);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1110]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1113] = (s.v[965] < 1e-8);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1113]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1114] = (s.v[965] > 1e-6);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1114]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1117] = (s.v[966] < 1.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1117]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1118] = (s.v[966] > 100000.0);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1118]) {
            s.store_scalar(966, 100000.0);
        }

        s.b[1121] = (s.v[967] < 1.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1121]) {
            s.store_scalar(967, 1.0);
        }

        s.b[1122] = (s.v[967] > 100000.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1122]) {
            s.store_scalar(967, 100000.0);
        }

        s.b[1125] = (s.v[971] < 1.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1125]) {
            s.store_scalar(971, 1.0);
        }

        s.b[1126] = (s.v[971] > 100000.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1126]) {
            s.store_scalar(971, 100000.0);
        }

        s.b[1129] = (s.v[975] < 0.1);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1129]) {
            s.store_scalar(975, 0.1);
        }

        s.b[1130] = (s.v[975] > 4.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1130]) {
            s.store_scalar(975, 4.0);
        }

        s.b[1133] = (s.v[972] < 0.0);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1133]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1134] = (s.v[972] > 5.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if (s.b[1106] && s.b[1134]) {
            s.store_scalar(972, 5.0);
        }

        s.b[1135] = (s.v[963] == 3.0);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        s.b[1138] = (s.v[964] < 5000000000000000.0);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1138]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1139] = (s.v[964] > 1e18);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1139]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1142] = (s.v[965] < 1e-8);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1142]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1143] = (s.v[965] > 1e-6);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1143]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1146] = (s.v[966] < 1.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1146]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1147] = (s.v[966] > 10000000000.0);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1147]) {
            s.store_scalar(966, 10000000000.0);
        }

        s.b[1150] = (s.v[971] < 100.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1150]) {
            s.store_scalar(971, 100.0);
        }

        s.b[1151] = (s.v[971] > 2000000000.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1151]) {
            s.store_scalar(971, 2000000000.0);
        }

        s.b[1154] = (s.v[972] < 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1154]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1155] = (s.v[972] > 5.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (((!s.b[1106]) && s.b[1135]) && s.b[1155]) {
            s.store_scalar(972, 5.0);
        }

        s.v[543] = p.p96;

        s.b[1164] = (s.v[543] < p.p95);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_scalar(543, p.p95);
        }

        s.b[1165] = (s.v[543] > 5e-7);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.store_scalar(543, 5e-7);
        }

        s.v[545] = (p.p120 / ((100.0) as f64).powf(p.p122));

        s.v[546] = (p.p123 / ((100.0) as f64).powf(p.p129));

        s.v[547] = (p.p198 / ((100.0) as f64).powf(p.p199));

        s.v[548] = (p.p200 / ((100.0) as f64).powf(p.p201));

        s.v[549] = (p.p183 / ((100.0) as f64).powf(p.p184));

        s.v[550] = (p.p202 / ((100.0) as f64).powf(p.p203));

        s.v[551] = (p.p190 / ((100.0) as f64).powf(p.p191));

        s.v[552] = (p.p186 / 100.0);

        s.v[553] = (p.p192 / 100.0);

        s.v[554] = (p.p73 * 100.0);

        s.v[555] = (p.p311 / 100.0);

        s.v[556] = (p.p312 / 100.0);

        s.v[557] = (p.p313 / 100.0);

        s.v[558] = (p.p314 / 100.0);

        s.v[544] = (p.p336 / 1e-6);

        s.v[559] = (p.p255 * 100.0);

        s.v[560] = (p.p248 * 100.0);

        s.v[561] = (p.p249 * 100.0);

        s.v[562] = (p.p251 / 10000.0);

        s.v[563] = (p.p266 * 10000.0);

        s.v[564] = (p.p275 / 100.0);

        s.v[565] = (p.p272 / 10000.0);

        s.v[572] = (p.p273 / 10000.0);

        s.v[567] = (p.p409 / 10000.0);

        s.v[568] = (p.p412 / 100.0);

        s.v[569] = (p.p413 / 10000.0);

        s.v[570] = (p.p414 / 100.0);

        s.store_scale(964, 964, 1000000.0);

        s.v[489] = (p.p453 / 1e-6);

        s.v[764] = (p.p274 + 273.15);

        s.v[582] = (p.p0 + p.p116);

        s.v[583] = ((p.p1 / p.p7) + p.p117);

        s.v[576] = (s.v[582] * 1000000.0);

        s.v[580] = (s.v[583] * 1000000.0);

        s.v[774] = ((s.v[576]) as f64).powf(p.p553);

        s.v[775] = ((s.v[580]) as f64).powf(p.p554);

        s.v[776] = (s.v[774] * s.v[775]);

        s.v[454] = (((p.p89 + (p.p555 / s.v[774])) + (p.p643 / s.v[775])) + (p.p731 / s.v[776]));

        s.v[455] = (((p.p92 + (p.p556 / s.v[774])) + (p.p644 / s.v[775])) + (p.p732 / s.v[776]));

        s.v[456] = (((p.p93 + (p.p557 / s.v[774])) + (p.p645 / s.v[775])) + (p.p733 / s.v[776]));

        s.v[457] = (((p.p94 + (p.p558 / s.v[774])) + (p.p646 / s.v[775])) + (p.p734 / s.v[776]));

        s.v[458] = (((p.p110 + (p.p559 / s.v[774])) + (p.p647 / s.v[775])) + (p.p735 / s.v[776]));

        s.v[459] = (((p.p111 + (p.p560 / s.v[774])) + (p.p648 / s.v[775])) + (p.p736 / s.v[776]));

        s.v[460] = (((p.p112 + (p.p561 / s.v[774])) + (p.p649 / s.v[775])) + (p.p737 / s.v[776]));

        s.v[461] = (((p.p126 + (p.p562 / s.v[774])) + (p.p650 / s.v[775])) + (p.p738 / s.v[776]));

        s.v[462] = (((p.p136 + (p.p563 / s.v[774])) + (p.p651 / s.v[775])) + (p.p739 / s.v[776]));

        s.v[463] = (((p.p138 + (p.p564 / s.v[774])) + (p.p652 / s.v[775])) + (p.p740 / s.v[776]));

        s.v[464] = (((p.p141 + (p.p565 / s.v[774])) + (p.p653 / s.v[775])) + (p.p741 / s.v[776]));

        s.v[465] = (((p.p144 + (p.p566 / s.v[774])) + (p.p654 / s.v[775])) + (p.p742 / s.v[776]));

        s.v[466] = (((p.p145 + (p.p567 / s.v[774])) + (p.p655 / s.v[775])) + (p.p743 / s.v[776]));

        s.v[467] = (((p.p146 + (p.p568 / s.v[774])) + (p.p656 / s.v[775])) + (p.p744 / s.v[776]));

        s.v[468] = (((p.p147 + (p.p569 / s.v[774])) + (p.p657 / s.v[775])) + (p.p745 / s.v[776]));

        s.v[469] = (((p.p148 + (p.p570 / s.v[774])) + (p.p658 / s.v[775])) + (p.p746 / s.v[776]));

        s.v[470] = (((p.p149 + (p.p571 / s.v[774])) + (p.p659 / s.v[775])) + (p.p747 / s.v[776]));

        s.v[471] = (((p.p151 + (p.p572 / s.v[774])) + (p.p660 / s.v[775])) + (p.p748 / s.v[776]));

        s.v[472] = (((p.p154 + (p.p573 / s.v[774])) + (p.p661 / s.v[775])) + (p.p749 / s.v[776]));

        s.v[473] = (((p.p157 + (p.p574 / s.v[774])) + (p.p662 / s.v[775])) + (p.p750 / s.v[776]));

        s.v[474] = (((p.p158 + (p.p575 / s.v[774])) + (p.p663 / s.v[775])) + (p.p751 / s.v[776]));

        s.v[475] = (((p.p159 + (p.p576 / s.v[774])) + (p.p664 / s.v[775])) + (p.p752 / s.v[776]));

        s.v[476] = (((p.p161 + (p.p577 / s.v[774])) + (p.p665 / s.v[775])) + (p.p753 / s.v[776]));

        s.v[477] = (((p.p169 + (p.p578 / s.v[774])) + (p.p666 / s.v[775])) + (p.p754 / s.v[776]));

        s.v[478] = (((p.p170 + (p.p579 / s.v[774])) + (p.p667 / s.v[775])) + (p.p755 / s.v[776]));

        s.v[479] = (((p.p172 + (p.p580 / s.v[774])) + (p.p668 / s.v[775])) + (p.p756 / s.v[776]));

        s.v[480] = (((p.p177 + (p.p581 / s.v[774])) + (p.p669 / s.v[775])) + (p.p757 / s.v[776]));

        s.v[481] = (((p.p179 + (p.p582 / s.v[774])) + (p.p670 / s.v[775])) + (p.p758 / s.v[776]));

        s.v[482] = (((p.p180 + (p.p583 / s.v[774])) + (p.p671 / s.v[775])) + (p.p759 / s.v[776]));

        s.v[483] = (((p.p185 + (p.p584 / s.v[774])) + (p.p672 / s.v[775])) + (p.p760 / s.v[776]));

        s.v[484] = (((p.p182 + (p.p585 / s.v[774])) + (p.p673 / s.v[775])) + (p.p761 / s.v[776]));

        s.v[485] = (((p.p181 + (p.p586 / s.v[774])) + (p.p674 / s.v[775])) + (p.p762 / s.v[776]));

        s.v[486] = (((p.p187 + (p.p587 / s.v[774])) + (p.p675 / s.v[775])) + (p.p763 / s.v[776]));

        s.v[487] = (((p.p188 + (p.p588 / s.v[774])) + (p.p676 / s.v[775])) + (p.p764 / s.v[776]));

        s.v[488] = (((p.p189 + (p.p589 / s.v[774])) + (p.p677 / s.v[775])) + (p.p765 / s.v[776]));

        s.v[490] = (((p.p194 + (p.p590 / s.v[774])) + (p.p678 / s.v[775])) + (p.p766 / s.v[776]));

        s.v[491] = (((p.p195 + (p.p591 / s.v[774])) + (p.p679 / s.v[775])) + (p.p767 / s.v[776]));

        s.v[492] = (((p.p196 + (p.p592 / s.v[774])) + (p.p680 / s.v[775])) + (p.p768 / s.v[776]));

        s.v[493] = (((p.p197 + (p.p593 / s.v[774])) + (p.p681 / s.v[775])) + (p.p769 / s.v[776]));

        s.v[494] = (((p.p204 + (p.p594 / s.v[774])) + (p.p682 / s.v[775])) + (p.p770 / s.v[776]));

        s.v[495] = (((p.p205 + (p.p595 / s.v[774])) + (p.p683 / s.v[775])) + (p.p771 / s.v[776]));

        s.v[496] = (((p.p210 + (p.p596 / s.v[774])) + (p.p684 / s.v[775])) + (p.p772 / s.v[776]));

        s.v[497] = (((p.p211 + (p.p597 / s.v[774])) + (p.p685 / s.v[775])) + (p.p773 / s.v[776]));

        s.v[498] = (((p.p212 + (p.p598 / s.v[774])) + (p.p686 / s.v[775])) + (p.p774 / s.v[776]));

        s.v[499] = (((p.p214 + (p.p599 / s.v[774])) + (p.p687 / s.v[775])) + (p.p775 / s.v[776]));

        s.v[500] = (((p.p215 + (p.p600 / s.v[774])) + (p.p688 / s.v[775])) + (p.p776 / s.v[776]));

        s.v[501] = (((p.p216 + (p.p601 / s.v[774])) + (p.p689 / s.v[775])) + (p.p777 / s.v[776]));

        s.v[502] = (((p.p217 + (p.p602 / s.v[774])) + (p.p690 / s.v[775])) + (p.p778 / s.v[776]));

        s.v[503] = (((p.p218 + (p.p603 / s.v[774])) + (p.p691 / s.v[775])) + (p.p779 / s.v[776]));

        s.v[504] = (((p.p219 + (p.p604 / s.v[774])) + (p.p692 / s.v[775])) + (p.p780 / s.v[776]));

        s.v[505] = (((p.p269 + (p.p605 / s.v[774])) + (p.p693 / s.v[775])) + (p.p781 / s.v[776]));

        s.v[506] = (((p.p268 + (p.p606 / s.v[774])) + (p.p694 / s.v[775])) + (p.p782 / s.v[776]));

        s.v[507] = (((p.p226 + (p.p607 / s.v[774])) + (p.p695 / s.v[775])) + (p.p783 / s.v[776]));

        s.v[508] = (((p.p227 + (p.p608 / s.v[774])) + (p.p696 / s.v[775])) + (p.p784 / s.v[776]));

        s.v[509] = (((p.p228 + (p.p609 / s.v[774])) + (p.p697 / s.v[775])) + (p.p785 / s.v[776]));

        s.v[510] = (((p.p232 + (p.p610 / s.v[774])) + (p.p698 / s.v[775])) + (p.p786 / s.v[776]));

        s.v[511] = (((p.p240 + (p.p611 / s.v[774])) + (p.p699 / s.v[775])) + (p.p787 / s.v[776]));

        s.v[512] = (((p.p241 + (p.p612 / s.v[774])) + (p.p700 / s.v[775])) + (p.p788 / s.v[776]));

        s.v[513] = (((p.p245 + (p.p613 / s.v[774])) + (p.p701 / s.v[775])) + (p.p789 / s.v[776]));

        s.v[514] = (((p.p246 + (p.p614 / s.v[774])) + (p.p702 / s.v[775])) + (p.p790 / s.v[776]));

        s.v[515] = (((p.p247 + (p.p615 / s.v[774])) + (p.p703 / s.v[775])) + (p.p791 / s.v[776]));

        s.v[516] = (((p.p250 + (p.p616 / s.v[774])) + (p.p704 / s.v[775])) + (p.p792 / s.v[776]));

        s.v[517] = (((p.p253 + (p.p617 / s.v[774])) + (p.p705 / s.v[775])) + (p.p793 / s.v[776]));

        s.v[518] = (((p.p254 + (p.p618 / s.v[774])) + (p.p706 / s.v[775])) + (p.p794 / s.v[776]));

        s.v[519] = (((p.p256 + (p.p619 / s.v[774])) + (p.p707 / s.v[775])) + (p.p795 / s.v[776]));

        s.v[520] = (((p.p257 + (p.p620 / s.v[774])) + (p.p708 / s.v[775])) + (p.p796 / s.v[776]));

        s.v[522] = (((p.p265 + (p.p622 / s.v[774])) + (p.p710 / s.v[775])) + (p.p798 / s.v[776]));

        s.v[523] = (((p.p278 + (p.p623 / s.v[774])) + (p.p711 / s.v[775])) + (p.p799 / s.v[776]));

        s.v[524] = (((p.p281 + (p.p624 / s.v[774])) + (p.p712 / s.v[775])) + (p.p800 / s.v[776]));

        s.v[525] = (((p.p79 + (p.p625 / s.v[774])) + (p.p713 / s.v[775])) + (p.p801 / s.v[776]));

        s.v[526] = (((p.p86 + (p.p626 / s.v[774])) + (p.p714 / s.v[775])) + (p.p802 / s.v[776]));

        s.v[528] = (((p.p76 + (p.p628 / s.v[774])) + (p.p716 / s.v[775])) + (p.p804 / s.v[776]));

        s.v[529] = (((p.p81 + (p.p629 / s.v[774])) + (p.p717 / s.v[775])) + (p.p805 / s.v[776]));

        s.v[530] = (((p.p74 + (p.p630 / s.v[774])) + (p.p718 / s.v[775])) + (p.p806 / s.v[776]));

        s.v[531] = (((p.p298 + (p.p631 / s.v[774])) + (p.p719 / s.v[775])) + (p.p807 / s.v[776]));

        s.v[532] = (((p.p83 + (p.p632 / s.v[774])) + (p.p720 / s.v[775])) + (p.p808 / s.v[776]));

        s.v[533] = (((p.p84 + (p.p633 / s.v[774])) + (p.p721 / s.v[775])) + (p.p809 / s.v[776]));

        s.v[534] = (((p.p62 + (p.p634 / s.v[774])) + (p.p722 / s.v[775])) + (p.p810 / s.v[776]));

        s.v[535] = (((p.p59 + (p.p635 / s.v[774])) + (p.p723 / s.v[775])) + (p.p811 / s.v[776]));

        s.v[536] = (((p.p60 + (p.p636 / s.v[774])) + (p.p724 / s.v[775])) + (p.p812 / s.v[776]));

        s.v[537] = (((p.p85 + (p.p637 / s.v[774])) + (p.p725 / s.v[775])) + (p.p813 / s.v[776]));

        s.v[538] = (((p.p82 + (p.p638 / s.v[774])) + (p.p726 / s.v[775])) + (p.p814 / s.v[776]));

        s.v[539] = (((p.p61 + (p.p639 / s.v[774])) + (p.p727 / s.v[775])) + (p.p815 / s.v[776]));

        s.v[540] = (((p.p75 + (p.p640 / s.v[774])) + (p.p728 / s.v[775])) + (p.p816 / s.v[776]));

        s.v[541] = (((p.p80 + (p.p641 / s.v[774])) + (p.p729 / s.v[775])) + (p.p817 / s.v[776]));

        s.v[542] = (((p.p77 + (p.p642 / s.v[774])) + (p.p730 / s.v[775])) + (p.p818 / s.v[776]));

        s.v[818] = (((p.p493 + (p.p824 / s.v[774])) + (p.p839 / s.v[775])) + (p.p854 / s.v[776]));

        s.v[819] = (((p.p494 + (p.p825 / s.v[774])) + (p.p840 / s.v[775])) + (p.p855 / s.v[776]));

        s.v[820] = (((p.p496 + (p.p826 / s.v[774])) + (p.p841 / s.v[775])) + (p.p856 / s.v[776]));

        s.v[822] = (((p.p515 + (p.p828 / s.v[774])) + (p.p843 / s.v[775])) + (p.p858 / s.v[776]));

        s.v[823] = (((p.p516 + (p.p829 / s.v[774])) + (p.p844 / s.v[775])) + (p.p859 / s.v[776]));

        s.v[824] = (((p.p517 + (p.p830 / s.v[774])) + (p.p845 / s.v[775])) + (p.p860 / s.v[776]));

        s.v[825] = (((p.p519 + (p.p831 / s.v[774])) + (p.p846 / s.v[775])) + (p.p861 / s.v[776]));

        s.v[827] = (((p.p538 + (p.p833 / s.v[774])) + (p.p848 / s.v[775])) + (p.p863 / s.v[776]));

        s.b[1181] = (s.v[963] != 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p342));
            s.store_mul_offset_ad_rhs(964, 964, A::div_from_scalar(p.p341, s.ad_value(337)), 1.0);
        }

        s.b[1182] = (s.v[964] < 1e21);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1182]) {
            s.store_scalar(964, 1e21);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p369));
            s.store_scaled_offset_ad(973, A::div_from_scalar(p.p368, s.ad_value(337)), 1.0, s.v[973]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p362));
            s.store_scaled_offset_ad(972, A::div_from_scalar(p.p361, s.ad_value(337)), 1.0, p.p360);
        }

        s.b[1183] = (s.v[972] < 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1183]) {
            s.store_scalar(972, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p348));
            s.store_scaled_offset_ad(966, A::div_from_scalar(p.p347, s.ad_value(337)), 1.0, p.p346);
        }

        s.b[1184] = (s.v[966] < 1.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1181] && s.b[1184]) {
            s.store_scalar(966, 1.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p351));
            s.store_scaled_offset_ad(968, A::div_from_scalar(p.p350, s.ad_value(337)), 1.0, p.p349);
        }

        s.b[1185] = (s.v[968] < 0.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1185]) {
            s.store_scalar(968, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p357));
            s.store_scaled_offset_ad(967, A::div_from_scalar(p.p356, s.ad_value(337)), 1.0, p.p354);
        }

        s.b[1186] = (s.v[967] < 0.0);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1186]) {
            s.store_scalar(967, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p359));
            s.store_scaled_offset_ad(969, A::div_from_scalar(p.p358, s.ad_value(337)), 1.0, p.p355);
        }

        s.b[1187] = (s.v[969] < 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1187]) {
            s.store_scalar(969, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p373));
            s.store_scaled_offset_ad(974, A::div_from_scalar(p.p372, s.ad_value(337)), 1.0, s.v[974]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p375));
            s.store_mul_offset_ad_rhs(975, 975, A::div_from_scalar(p.p374, s.ad_value(337)), 1.0);
        }

        s.b[1188] = (s.v[975] < 0.1);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if (s.b[1181] && s.b[1188]) {
            s.store_scalar(975, 0.1);
        }

        if (!s.b[1181]) {
            s.store_scalar(964, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
        }

        s.b[1240] = ((s.v[450] * s.v[451]) > 1.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if s.b[1240] {
            s.store_div_from_scalar(450, 1.0, 451);
        }

        s.b[1242] = ((p.p40 == 1.0) && (((p.p19 > 0.0) && (s.v[459] == 0.0)) || ((p.p18 > 0.0) && (s.v[460] == 0.0))));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if s.b[1242] {
            s.store_scalar(449, 0.0);
        }

        if (!s.b[1242]) {
            s.store_scalar(449, p.p40);
        }

        s.b[1243] = (s.v[449] == 1.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if s.b[1243] {
            s.store_scalar(75, (if (p.p19 > 0.0) { 1.0 } else { 0.0 }));
        }

        if s.b[1243] {
            s.store_scalar(76, (if (p.p18 > 0.0) { 1.0 } else { 0.0 }));
        }

        s.b[1244] = ((p.p17 == 0.0) || (p.p17 == 2.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if ((!s.b[1243]) && s.b[1244]) {
            s.store_scalar(75, 0.0);
            s.store_scalar(76, 0.0);
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p130 * p.p2) * p.p7) + (((s.v[530] + s.v[538]) * (((p.p67 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p68 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(75, (if (s.v[335] > 0.0) { 1.0 } else { 0.0 }));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p131 * p.p3) * p.p7) + ((s.v[540] * (((p.p69 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p70 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(76, (if (s.v[335] > 0.0) { 1.0 } else { 0.0 }));
        }

        s.v[571] = (p.p12 / 1e-6);

        s.v[554] = (p.p73 * 100.0);

        s.v[463] = (s.v[463] / 1e-6);

        s.v[464] = (s.v[464] / 1e-6);

        s.v[494] = (s.v[494] / 1e-6);

        s.v[459] = (s.v[459] / 1e-6);

        s.v[460] = (s.v[460] / 1e-6);

        s.v[502] = (s.v[502] / 100.0);

        s.v[499] = (s.v[499] / 100.0);

        s.v[454] = (s.v[454] / 100.0);

        s.v[510] = (s.v[510] * 10000.0);

        s.v[517] = (s.v[517] / 100.0);

        s.v[518] = (s.v[518] * 100.0);

        s.v[514] = (s.v[514] * 100.0);

        s.v[520] = (s.v[520] * 100.0);

        s.v[491] = (s.v[491] * 100.0);

        s.v[511] = (s.v[511] / 10.0);

        s.v[512] = (s.v[512] * 100.0);

        s.v[522] = (s.v[522] / 100.0);

        s.v[528] = (s.v[528] / 1e-6);

        s.v[531] = (s.v[531] / 100.0);

        s.v[532] = (s.v[532] / 100.0);

        s.v[533] = (s.v[533] / 100.0);

        s.v[538] = (s.v[538] / 100.0);

        s.v[541] = (s.v[541] / 100.0);

        s.v[458] = (-s.v[458]);

        s.store_scale(973, 973, 0.01);

        s.v[81] = p.p28;

        s.b[82] = ((p.p133 != 0.0) || (p.p134 != 0.0));
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        s.b[1246] = (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if s.b[1246] {
            s.store_scalar(765, 0.0);
        }

        if (!s.b[1246]) {
            s.store_scalar(765, 1.0);
        }

        s.v[581] = (s.v[580] * s.v[576]);

        s.v[777] = (p.p289 * 1000000.0);

        s.v[616] = (s.v[457] - (s.v[764] * (9.025e-5 + (s.v[764] * 1e-7))));

        s.v[617] = (8.8541878e-12 * p.p267);

        s.copy_ad(618, 452);

        s.b[1247] = (s.v[471] == 0.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if s.b[1247] {
            s.store_scalar(615, 0.0);
            s.store_scalar(642, 0.0);
        }

        if (!s.b[1247]) {
            s.store_scalar(615, 1.0);
            s.store_scalar(642, ((((1.0 + (1.0 / s.v[576]))) as f64).powf(p.p153) * s.v[471]));
        }

        s.v[619] = (1.0 + (((s.v[576]) as f64).powf(p.p229) * p.p230));

        s.v[335] = ((1.0 / (p.p118 + (0.5 * p.p0))) + (1.0 / (p.p119 + (0.5 * p.p0))));

        s.v[589] = (2.0 / s.v[335]);

        s.b[1248] = (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0))));
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if s.b[1248] {
            s.store_scalar(335, 0.0);
            s.store_scalar(721, 0.0);
        }

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if (s.b[1248] && (s.v[721] < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1248] {
                s.store_add_scaled_inputs3(335, s.ad_value(335), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p8 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p9 + (0.5 * p.p0)))), 1.0);
                s.store_offset(721, 721, 1.0);
            }
        }

        if s.b[1248] {
            s.store_div_from_scalar(588, (2.0 * p.p7), 335);
        }

        if (!s.b[1248]) {
            s.store_scalar(588, 0.0);
        }

        s.v[773] = s.v[528];

        s.v[620] = s.v[476];

        s.v[621] = s.v[464];

        s.v[622] = s.v[463];

        s.b[1249] = ((p.p32 == 1.0) && s.b[623]);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if s.b[1249] {
            s.store_scalar(620, (s.v[620] * ((p.p282 * (((s.v[571]) as f64).ln() - ((s.v[622]) as f64).ln())) + 1.0)));
            s.store_scalar(621, ((s.v[621] + s.v[571]) - s.v[622]));
            s.store_scalar(773, ((s.v[773] + s.v[571]) - s.v[622]));
            s.store_scalar(622, s.v[571]);
        }

        s.store_scale(573, 620, ((1.0 + (p.p162 / ((s.v[580]) as f64).powf(p.p163))) * ((1.0 + (p.p164 / ((s.v[576]) as f64).powf(p.p165))) * (1.0 + (p.p167 / ((s.v[581]) as f64).powf(p.p168))))));

        s.b[1251] = (s.v[588] > 0.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if s.b[1251] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[500])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[499], s.ad_value(588)), s.v[501]);
            s.store_scalar(337, (((s.v[499] / s.v[589])) as f64).powf(s.v[501]));
            s.store_div_scaled_product_offset_denominator(573, s.ad_value(573), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        s.v[624] = ((p.p171 * (1.0 + (p.p173 / ((s.v[576]) as f64).powf(p.p176)))) * (1.0 + (p.p174 / ((s.v[580]) as f64).powf(p.p175))));

        if (s.v[573] < 1e-25) {
            s.store_scalar(573, 1e-25);
        }

        if (s.v[624] < 1e-25) {
            s.store_scalar(624, 1e-25);
        }

        s.v[335] = ((s.v[576]) as f64).powf(p.p156);

        s.v[625] = (((s.v[472] * s.v[335]) / (s.v[335] + p.p155)) / 1.034943e-10);

        s.v[626] = (s.v[473] / 1.034943e-10);

        s.v[627] = ((p.p319 * (1.0 + (p.p320 / ((s.v[576]) as f64).powf(p.p321)))) * (1.0 + (p.p322 / ((s.v[580]) as f64).powf(p.p323))));

        s.v[335] = ((1.0 + (p.p386 / ((s.v[576]) as f64).powf(p.p387))) * (1.0 + (p.p388 / ((s.v[580]) as f64).powf(p.p389))));

        s.v[633] = (p.p384 * s.v[335]);

        s.v[634] = (p.p385 * s.v[335]);

        s.v[574] = (p.p97 + (s.v[545] / (((s.v[582] + p.p121)) as f64).powf(p.p122)));

        s.store_offset(575, 451, (s.v[545] / (((s.v[582] + p.p121)) as f64).powf(p.p122)));

        s.v[577] = (p.p114 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129)));

        s.v[578] = (p.p295 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129)));

        s.v[579] = (p.p115 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129)));

        s.store_sub_from_scalar_ad(162, s.v[582], A::offset(s.ad_value(575), s.v[574]));

        s.v[628] = (s.v[582] + (p.p124 / ((s.v[581]) as f64).powf(p.p125)));

        s.v[629] = (s.v[461] / ((s.v[581]) as f64).powf(p.p127));

        s.v[335] = (1.0 + (p.p206 / (((s.v[628] * 1000000.0)) as f64).powf(p.p207)));

        s.v[336] = (1.0 + (p.p208 / ((s.v[580]) as f64).powf(p.p209)));

        s.v[495] = ((s.v[495] * s.v[335]) * s.v[336]);

        s.v[163] = (s.v[583] - (2.0 * s.v[577]));

        s.v[630] = (s.v[583] - (2.0 * s.v[578]));

        s.v[631] = (s.v[583] - (2.0 * s.v[579]));

        s.v[632] = (s.v[163] * p.p7);

        s.v[635] = (s.v[631] * p.p7);

        s.store_scale(584, 621, (1.0 + (p.p142 / ((s.v[580]) as f64).powf(p.p143))));

        s.store_scale(622, 622, (1.0 + (p.p233 / ((s.v[580]) as f64).powf(p.p234))));

        s.store_scale(335, 622, 1e-6);

        s.store_scale(336, 584, 1e-6);

        s.b[1259] = (s.v[335] < 1000000000000000.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if s.b[1259] {
            s.store_scalar(335, 1000000000000000.0);
        }

        s.store_scale(622, 335, 1000000.0);

        s.b[1261] = (s.v[336] < 1000000000000000.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if s.b[1261] {
            s.store_scalar(336, 1000000000000000.0);
        }

        s.store_scale(584, 336, 1000000.0);

        s.b[1262] = (s.v[588] > 0.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if s.b[1262] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[503])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[502], s.ad_value(588)), s.v[504]);
            s.store_scalar(337, (((s.v[502] / s.v[589])) as f64).powf(s.v[504]));
            s.store_div_scaled_product_offset_denominator(585, s.ad_value(584), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        if (!s.b[1262]) {
            s.copy_ad(585, 584);
        }

        s.b[1263] = ((s.v[582] > p.p140) || (p.p140 <= 0.0));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if s.b[1263] {
            s.store_add_scaled_inputs(586, 622, ((s.v[582] - p.p140) * 1.0 / (s.v[582])), 585, (p.p140 * 1.0 / (s.v[582])));
        }

        if (!s.b[1263]) {
            s.store_add_scaled_inputs3(586, s.ad_value(585), 1.0, s.ad_value(585), ((p.p140 - s.v[582]) * 1.0 / (p.p140)), s.ad_value(622), (-((p.p140 - s.v[582]) * 1.0 / (p.p140))));
        }

        s.v[337] = ((0.5 * s.v[582]) - p.p140);

        s.v[781] = ((s.v[337] - 1e-9) - 1e-10);

        s.v[782] = ((4.0 * 1e-9) * 1e-10);

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_offset_input(782, 782, (s.v[781] * s.v[781]));

        s.store_scaled_offset_ad(334, A::div_from_scalar(s.v[781], s.ad_value(782)), 1.0, 0.5);

        s.store_offset_scaled(337, 782, 0.5, ((((s.v[781]) * (0.5))) + (1e-9)));

        s.store_div_from_scalar_offset_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(337)), (1.0 / p.p220));

        if (0.0 >= s.v[335]) {
            s.store_scalar(336, 0.0);
        } else {
            s.copy_ad(336, 335);
        }

        s.store_add_scaled_product_right_ad(586, 586, 1.0, 336, A::sub(s.ad_value(773), s.ad_value(622)), 1.0 / (s.v[582]));

        s.store_scale(166, 586, 1.6021918e-19);

        s.store_scale(636, 166, 1.034943e-10);

        s.store_scale(637, 636, 2.0);

        s.b[1264] = ((s.v[582] <= (2.0 * p.p140)) && (p.p140 > 0.0));
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if s.b[1264] {
            s.store_add_scaled_inputs4(587, s.ad_value(585), 2.0, s.ad_value(585), (-(s.v[582] * 1.0 / (p.p140))), s.ad_value(622), (-(-(s.v[582] * 1.0 / (p.p140)))), s.ad_value(622), -1.0);
            s.store_ln_div(638, 587, 622);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[1264]) {
            s.store_scalar(638, 0.0);
        }

        s.v[639] = (((((2.0 * 1.6021918e-19) * s.v[494]) * 1.034943e-10)) as f64).sqrt();

        s.v[640] = (1.0 / (s.v[494] * s.v[494]));

        s.v[641] = ((1.0 + (s.v[542] / ((s.v[576]) as f64).powf(p.p231))) * (1.0 + (p.p238 / ((s.v[581]) as f64).powf(p.p239))));

        s.store_scaled_ln_scaled_input(158, 586, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(159, 622, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.b[1265] = (p.p51 == 1.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if s.b[1265] {
            s.store_scalar(335, (p.p5 + (s.v[163] / (3.0 * p.p4))));
            s.store_scalar(336, (s.v[582] - p.p6));
        }

        s.b[1267] = (p.p130 > 0.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if s.b[1267] {
            s.store_scalar(644, (p.p130 * p.p2));
            s.store_scalar(648, (p.p130 * p.p3));
        }

        if (!s.b[1267]) {
            s.store_scalar(644, 0.0);
            s.store_scalar(648, 0.0);
        }

        s.b[1268] = (p.p131 > 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if s.b[1268] {
            s.store_scalar(648, (p.p131 * p.p3));
        }

        if (!s.b[1268]) {
            s.store_scalar(648, 0.0);
        }

        s.b[1269] = (s.v[449] == 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        s.b[1270] = ((s.v[530] > 0.0) || (s.v[540] > 0.0));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1269] && s.b[1270]) {
            s.store_scalar(645, (1.0 + (p.p309 / ((s.v[581]) as f64).powf(p.p310))));
        }

        s.b[1271] = (s.v[538] != 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_scalar(341, (1.0 + (p.p303 / ((s.v[581]) as f64).powf(p.p304))));
            s.store_scalar(340, ((-p.p301) * ((s.v[576]) as f64).powf(p.p302)));
        }

        s.b[1272] = (s.v[340] > 60.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((s.b[1269] && s.b[1270]) && s.b[1271]) && s.b[1272]) {
            s.store_scalar(340, 60.0);
        }

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_exp(340, 340);
            s.store_mul(646, 340, 341);
        }

        if ((s.b[1269] && s.b[1270]) && (!s.b[1271])) {
            s.store_scalar(646, 0.0);
        }

        if (s.b[1269] && (!s.b[1270])) {
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
        }

        s.b[1273] = (s.v[532] != 0.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (s.b[1269] && s.b[1273]) {
            s.store_scalar(336, (1.0 + (p.p307 / ((s.v[581]) as f64).powf(p.p308))));
            s.store_scalar(335, ((-p.p305) * ((s.v[576]) as f64).powf(p.p306)));
        }

        s.b[1274] = (s.v[335] > 60.0);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if ((s.b[1269] && s.b[1273]) && s.b[1274]) {
            s.store_scalar(335, 60.0);
        }

        if (s.b[1269] && s.b[1273]) {
            s.store_exp(335, 335);
            s.store_scaled_mul(337, 336, 335, s.v[532]);
            s.store_scaled_add_sqrt_square_offset_rhs(647, 337, 337, ((((4.0 * 1e-6) / 100.0) * 1e-6) / 100.0), 0.5);
        }

        if (s.b[1269] && (!s.b[1273])) {
            s.store_scalar(647, 0.0);
        }

        if s.b[1269] {
            s.store_scalar(649, 0.0);
            s.store_scalar(614, 0.0);
            s.store_scalar(786, 0.0);
            s.store_scalar(652, 0.0);
            s.store_scalar(653, 0.0);
            s.store_scalar(654, 0.0);
        }

        if (!s.b[1269]) {
            s.store_sqrt_square_offset(649, 451, (p.p419 * p.p419));
            s.store_scalar(614, ((((p.p419 * p.p419) + (p.p97 * p.p97))) as f64).sqrt());
            s.store_scalar(786, (1.0 + (p.p424 / ((s.v[580]) as f64).powf(p.p425))));
            s.store_scalar(652, (1.0 + (p.p426 / ((s.v[576]) as f64).powf(p.p427))));
            s.store_scalar(653, (1.0 + (p.p428 / ((s.v[576]) as f64).powf(p.p429))));
            s.store_scalar(654, 1.0);
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
        }

        s.b[1275] = (s.v[459] > 0.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if s.b[1275] {
            s.store_scalar(650, ((2.0 * 1.034943e-10) / (1.6021918e-19 * s.v[459])));
            s.store_div_scaled_value_offset_denominator(651, s.ad_value(622), (((2.0 * 1.034943e-10) / 1.6021918e-19) * 1.0 / (s.v[459])), s.ad_value(622), s.v[459], 1.0);
        }

        if (!s.b[1275]) {
            s.store_scalar(650, 0.0);
            s.store_scalar(651, 0.0);
        }

        s.b[1280] = (p.p44 == 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if s.b[1280] {
            s.store_scalar(335, ((p.p108 * s.v[576]) + p.p109));
        }

        s.b[1281] = (s.v[335] < 0.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if (s.b[1280] && s.b[1281]) {
            s.store_scalar(335, 0.0);
        }

        if s.b[1280] {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), 1.0);
        }

        if (!s.b[1280]) {
            s.store_scalar(335, (p.p108 * s.v[576]));
        }

        s.b[1282] = (s.v[335] < 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((!s.b[1280]) && s.b[1282]) {
            s.store_scalar(335, 0.0);
        }

        if (!s.b[1280]) {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), ((p.p109) + (1e-25)));
        }

        s.b[1284] = (s.v[658] < 0.1);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if s.b[1284] {
            s.store_scalar(658, 0.1);
        }

        if (p.p23 != 0.0) {
            s.store_scalar(336, ((s.v[163]) as f64).powf(p.p201));
            s.store_div_scaled_value_offset_denominator(659, s.ad_value(336), (s.v[485] * (1.0 + (s.v[547] / ((s.v[582]) as f64).powf(p.p199)))), s.ad_value(336), s.v[548], 1.0);
            s.store_scalar(660, (s.v[484] * (1.0 + (s.v[549] / ((s.v[582]) as f64).powf(p.p184)))));
            s.store_scalar(661, (s.v[552] * (1.0 + (s.v[550] / ((s.v[582]) as f64).powf(p.p203)))));
            s.store_scalar(662, (s.v[481] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p.p191)))));
            s.store_scalar(663, (s.v[482] * (1.0 + (s.v[553] / s.v[582]))));
            s.copy_ad(668, 662);
            s.copy_ad(669, 663);
            s.copy_ad(665, 659);
            s.copy_ad(666, 660);
            s.copy_ad(667, 661);
        }

        if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
            s.store_scalar(668, (s.v[486] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p.p191)))));
            s.store_scalar(669, (s.v[487] * (1.0 + (s.v[553] / s.v[582]))));
        }

        if (p.p23 != 0.0) {
            s.store_scalar(664, (p.p72 * (1.0 + (p.p102 / ((s.v[576]) as f64).powf(p.p103)))));
        }

        if (p.p23 == 0.0) {
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.v[523] = (if (s.v[523] != 0.0) { (s.v[523] * (1.0 + (p.p279 / ((s.v[576]) as f64).powf(p.p280)))) } else { 0.0 });

        s.v[670] = (((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[635]) * (((1.0 + (p.p225 / p.p95))) as f64).ln());

        s.v[671] = (if (p.p134 != 0.0) { (((1000000.0 * s.v[635]) * p.p134) / ((s.v[576]) as f64).powf(p.p135)) } else { 0.0 });

        s.v[672] = (p.p283 * ((s.v[576]) as f64).powf((-p.p286)));

        s.v[673] = (p.p290 * ((s.v[576]) as f64).powf((-p.p291)));

        s.v[674] = (p.p287 * (((s.v[576] + s.v[777])) as f64).powf((-p.p288)));

        s.v[766] = (((s.v[541] / (s.v[365] * s.v[632])) * (1.0 + (p.p317 / ((s.v[576]) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((s.v[580]) as f64).powf(p.p316))));

        s.v[766] = (s.v[766] * (1.0 / ((p.p7) as f64).powf(p.p327)));

        s.v[675] = ((((1.0 / ((p.p7) as f64).powf(p.p327)) / (s.v[365] * s.v[632])) * (1.0 + (p.p317 / ((s.v[576]) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((s.v[580]) as f64).powf(p.p316))));

        s.b[1285] = ((p.p53 == 0.0) || (s.v[541] == 0.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if s.b[1285] {
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_offset(387, 387, s.v[732]);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
            s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));
            s.store_scale(676, 387, 1.0 / (s.v[764]));
            s.store_ln(590, 676);
            s.store_sub_scaled_ad_lhs(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 392, s.v[456]);
            s.store_sqrt(677, 393);
            s.store_div_from_scalar(335, 1.0, 387);
            s.store_scalar(336, (1.0 / s.v[764]));
            s.store_add_scaled_inputs4_offset(337, s.ad_value(335), p.p260, s.ad_value(336), (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));
            s.store_sqrt(192, 337);
            s.store_mul(193, 337, 192);
            s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);
            s.store_div_from_scalar(155, 1.0, 154);
            s.store_square(156, 154);
            s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));
            s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);
            s.store_exp_scaled_input(335, 590, s.v[480]);
            s.store_div(679, 335, 573);
        }

        s.b[1286] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1286]) {
            s.store_sqrt_ad(209, A::mul_scaled_lhs(s.ad_value(964), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(155)));
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

    }
}
