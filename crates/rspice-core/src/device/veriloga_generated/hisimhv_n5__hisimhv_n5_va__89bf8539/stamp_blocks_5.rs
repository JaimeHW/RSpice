#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign79460_loop_guard: usize = 0;
        while {
            let assign79460_cond_e121235: f64 = (s.v[421] + 1.0);
            let assign79460_cond_e121237: f64 = if (((s.v[2623] != 0.0) && s.b[2841]) && (s.v[97] <= assign79460_cond_e121235)) { 1.0 } else { 0.0 };
            assign79460_cond_e121237 != 0.0
        } {
            assign79460_loop_guard += 1;
            assert!(assign79460_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2846] = (s.v[333] < 60.0);
            s.v[2846] = if s.b[2846] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2846]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2846])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2847] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2847] = if s.b[2847] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2847]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2768, 334, 336);
                s.store_mul_add_scaled_product_rhs(2769, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2847])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2768, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2769, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2848] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2848] = if s.b[2848] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2848]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2849] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2849] = if s.b[2849] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2848])) && s.b[2849]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2848])) && (!s.b[2849])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2850] = (s.v[214] > 0.0);
            s.v[2850] = if s.b[2850] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2850]) {
                s.store_sqrt_add(216, 2768, 214);
                s.store_div_scaled_inputs2_indices(217, 2769, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2851] = (s.v[2768] > 0.0);
            s.v[2851] = if s.b[2851] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2850])) && s.b[2851]) {
                s.store_sqrt(216, 2768);
                s.store_div_scaled_inputs_indices(217, 2769, 0.5, 216, 1.0);
            }
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2850])) && (!s.b[2851])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2852] = (s.v[79] > 0.0);
            s.v[2852] = if s.b[2852] { 1.0 } else { 0.0 };
            let (assign79460_body56_e121977,) = {
    if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2852]) {
        let assign79460_body56_e121975: f64 = (s.v[421] + 1.0);
        (assign79460_body56_e121975,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign79460_body56_e121977;
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2853] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2853] = if s.b[2853] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) && s.b[2853]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) {
                s.store_add(404, 404, 236);
            }
            s.b[2854] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2854] = if s.b[2854] { 1.0 } else { 0.0 };
            let (assign79460_body63_e122067,) = {
    if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) && s.b[2854]) {
        let assign79460_body63_e122065: f64 = (s.v[79] + 2.0);
        (assign79460_body63_e122065,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign79460_body63_e122067;
            let (assign79460_body64_e122075,) = {
    if ((s.v[2623] != 0.0) && s.b[2841]) {
        let assign79460_body64_e122073: f64 = (s.v[97] + 1.0);
        (assign79460_body64_e122073,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign79460_body64_e122075;
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            if (s.v[2768] >= 0.0) {
                s.store_scaled_sqrt(223, 2768, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_mul(2758, 982, 223);
            s.store_mul(2759, 2760, 2758);
            s.store_offset_div(100, 2759, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2623] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2856] = (s.v[407] < 0.0);
        s.v[2856] = if s.b[2856] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2856]) {
            s.store_neg(407, 407);
        }

        s.b[2857] = (p.p55 == 0.0);
        s.v[2857] = if s.b[2857] { 1.0 } else { 0.0 };

        s.b[2858] = (p.p50 == 0.0);
        s.v[2858] = if s.b[2858] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) && s.b[2858]) {
            s.store_neg(2761, 404);
        }

        if ((((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) && (!s.b[2858])) {
            s.copy_ad(2761, 396);
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            s.store_sqrt_offset_square_offset(782, 2761, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2761), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2761), p.p137), 782, 0.5);
        }

        s.b[2859] = (s.v[336] < 0.0);
        s.v[2859] = if s.b[2859] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) && s.b[2859]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2860] = (2.0 == 1.0);
        s.v[2860] = if s.b[2860] { 1.0 } else { 0.0 };

        s.b[2861] = (2.0 == 2.0);
        s.v[2861] = if s.b[2861] { 1.0 } else { 0.0 };

        s.b[2862] = (2.0 == 3.0);
        s.v[2862] = if s.b[2862] { 1.0 } else { 0.0 };

        s.b[2863] = (2.0 == 4.0);
        s.v[2863] = if s.b[2863] { 1.0 } else { 0.0 };

        s.b[2864] = (p.p55 == 1.0);
        s.v[2864] = if s.b[2864] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2860]) && s.b[2864]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && s.b[2860]) && (!s.b[2864])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && s.b[2860]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2861] && (!s.b[2860]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2865] = (p.p55 == 1.0);
        s.v[2865] = if s.b[2865] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2862] && (!(s.b[2860] || s.b[2861])))) && s.b[2865]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && (s.b[2862] && (!(s.b[2860] || s.b[2861])))) && (!s.b[2865])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && (s.b[2862] && (!(s.b[2860] || s.b[2861])))) {
            s.copy_ad(697, 404);
        }

        s.b[2866] = (p.p430 == 0.0);
        s.v[2866] = if s.b[2866] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2862] && (!(s.b[2860] || s.b[2861])))) && s.b[2866]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2862] && (!(s.b[2860] || s.b[2861])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2623] != 0.0) && (s.b[2863] && (!((s.b[2860] || s.b[2861]) || s.b[2862])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2623] = 0.0;

        s.b[2867] = (3.0 == 1.0);
        s.v[2867] = if s.b[2867] { 1.0 } else { 0.0 };

        s.b[2868] = (3.0 == 2.0);
        s.v[2868] = if s.b[2868] { 1.0 } else { 0.0 };

        s.b[2869] = (3.0 == 3.0);
        s.v[2869] = if s.b[2869] { 1.0 } else { 0.0 };

        s.b[2870] = (3.0 == 4.0);
        s.v[2870] = if s.b[2870] { 1.0 } else { 0.0 };

        s.b[2871] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2871] = if s.b[2871] { 1.0 } else { 0.0 };

        let (assign80090_e122668,) = {
    if (s.b[2867] && s.b[2871]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign80090_e122668;

        let (assign80100_e122674,) = {
    if (s.b[2867] && s.b[2871]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign80100_e122674;

        if (s.b[2867] && s.b[2871]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2872] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2872] = if s.b[2872] { 1.0 } else { 0.0 };

        let (assign80190_e122747,) = {
    if ((s.b[2868] && (!s.b[2867])) && s.b[2872]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign80190_e122747;

        if ((s.b[2868] && (!s.b[2867])) && s.b[2872]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2873] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2873] = if s.b[2873] { 1.0 } else { 0.0 };

        let (assign80230_e122790,) = {
    if ((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign80230_e122790;

    }

    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let (assign80240_e122801,) = {
    if ((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.v[2624] = assign80240_e122801;

        if ((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2874] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2874] = if s.b[2874] { 1.0 } else { 0.0 };

        if (((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2875] = (p.p113 > 0.0);
        s.v[2875] = if s.b[2875] { 1.0 } else { 0.0 };

        s.b[2876] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2876] = if s.b[2876] { 1.0 } else { 0.0 };

        if (((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) && s.b[2876]) {
        }

        if (((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) && (!s.b[2876])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) && (!s.b[2876])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2877] = (s.v[336] < 0.0);
        s.v[2877] = if s.b[2877] { 1.0 } else { 0.0 };

        if (((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) && s.b[2877]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2878] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2878] = if s.b[2878] { 1.0 } else { 0.0 };

        let (assign80540_e123272,) = {
    if ((s.b[2870] && (!((s.b[2867] || s.b[2868]) || s.b[2869]))) && s.b[2878]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign80540_e123272;

        if ((s.b[2870] && (!((s.b[2867] || s.b[2868]) || s.b[2869]))) && s.b[2878]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2886, 0.4);
        }

        let (assign80590_e123314,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[2887],)
    }
};
        s.v[2887] = assign80590_e123314;

        if (s.v[2623] != 0.0) {
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
        }

        let (assign80720_e123367,) = {
    if (s.v[2623] != 0.0) {
        let assign80720_e123365: f64 = (-1.0);
        (assign80720_e123365,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign80720_e123367;

        if (s.v[2623] != 0.0) {
            s.store_scalar(2888, 0.0);
            s.store_scalar(2889, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2884, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2884), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2885, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2891] = (s.v[2886] > (s.v[2885] * 0.5));
        s.v[2891] = if s.b[2891] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2891]) {
            s.store_scale(2886, 2885, 0.5);
        }

        s.b[2892] = param_given[338];
        s.v[2892] = if s.b[2892] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2892]) {
            s.store_scalar(2885, p.p338);
        }

        s.b[2893] = param_given[339];
        s.v[2893] = if s.b[2893] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2893]) {
            s.store_scalar(2886, p.p339);
        }

        s.b[2894] = param_given[338];
        s.v[2894] = if s.b[2894] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2893])) && s.b[2894]) {
            s.store_scale(2886, 2885, 0.5);
        }

        s.b[2895] = (s.v[2886] > (s.v[2885] * 0.5));
        s.v[2895] = if s.b[2895] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2895]) {
            s.store_scale(2886, 2885, 0.5);
        }

        s.b[2896] = (p.p38 == 1.0);
        s.v[2896] = if s.b[2896] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2896]) {
            s.store_neg(334, 396);
        }

        s.b[2897] = (s.v[334] > s.v[2886]);
        s.v[2897] = if s.b[2897] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2896]) && s.b[2897]) {
            s.store_sub(335, 334, 2886);
            s.store_sub(336, 2885, 2886);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2886, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[2896]) && (!s.b[2897])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[2896]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2896])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign81130_e123708,) = {
    if (s.v[2623] != 0.0) {
        let assign81130_e123702: f64 = (-s.v[397]);
        let assign81130_e123705: f64 = (10.0 * 2.220446049250313e-16);
        let assign81130_e123706: f64 = (assign81130_e123702 + assign81130_e123705);
        (assign81130_e123706,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign81130_e123708;

        if (s.v[2623] != 0.0) {
            s.store_scalar(2880, 0.0);
            s.store_scale(2881, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2898] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2898] = if s.b[2898] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2898]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2898])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign81230_loop_guard: usize = 0;
        while {
            let assign81230_cond_e123782: f64 = if (((s.v[2623] != 0.0) && (!s.b[2898])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign81230_cond_e123782 != 0.0
        } {
            assign81230_loop_guard += 1;
            assert!(assign81230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2898])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2898])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2899] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2899] = if s.b[2899] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign81380_e123956,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign81380_e123956;

        let (assign81390_e123964,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign81390_e123964;

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
            s.store_scalar(770, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2900] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2900] = if s.b[2900] { 1.0 } else { 0.0 };

        s.b[2901] = (1.0 == 1.0);
        s.v[2901] = if s.b[2901] { 1.0 } else { 0.0 };

        let (assign81480_e124048,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && s.b[2901]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign81480_e124048;

        s.b[2902] = (1.0 == 2.0);
        s.v[2902] = if s.b[2902] { 1.0 } else { 0.0 };

        let (assign81500_e124066,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && s.b[2902]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign81500_e124066;

        s.b[2903] = (1.0 == 4.0);
        s.v[2903] = if s.b[2903] { 1.0 } else { 0.0 };

        let (assign81520_e124087,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && (!s.b[2902])) && s.b[2903]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign81520_e124087;

        s.b[2904] = (1.0 == 8.0);
        s.v[2904] = if s.b[2904] { 1.0 } else { 0.0 };

        let (assign81540_e124111,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && (!s.b[2902])) && (!s.b[2903])) && s.b[2904]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign81540_e124111;

        let (assign81550_e124121,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign81550_e124121;

        let mut assign81560_loop_guard: usize = 0;
        while {
            let assign81560_cond_e124132: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign81560_cond_e124132 != 0.0
        } {
            assign81560_loop_guard += 1;
            assert!(assign81560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) {
                s.store_sqrt(726, 726);
            }
            let (assign81560_body1_e124155,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) {
        let assign81560_body1_e124153: f64 = (s.v[719] + 1.0);
        (assign81560_body1_e124153,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign81560_body1_e124155;
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && (!s.b[2900])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2899])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign81660_e124272,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let assign81660_e124266: f64 = (-s.v[397]);
        let assign81660_e124269: f64 = (10.0 * 2.220446049250313e-16);
        let assign81660_e124270: f64 = (assign81660_e124266 + assign81660_e124269);
        (assign81660_e124270,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign81660_e124272;

        s.b[2905] = (s.v[402] < s.v[403]);
        s.v[2905] = if s.b[2905] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2905]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2906] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2906] = if s.b[2906] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2905]) && s.b[2906]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2905]) && (!s.b[2906])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2905]) {
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
            s.copy_ad(2888, 404);
        }

        s.b[2907] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2907] = if s.b[2907] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2907]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2907])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2908] = (s.v[116] >= 3.0);
        s.v[2908] = if s.b[2908] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2908]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2908])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(116, 439, 1.0, 438, 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2909] = (p.p33 > 0.0);
        s.v[2909] = if s.b[2909] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2910] = (p.p33 == 2.0);
        s.v[2910] = if s.b[2910] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && (!s.b[2910])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.copy_ad(445, 116);
        }

        s.b[2911] = (p.p33 == 2.0);
        s.v[2911] = if s.b[2911] { 1.0 } else { 0.0 };

        s.b[2912] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2912] = if s.b[2912] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign82490_e125418,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign82490_e125418;

        let (assign82500_e125431,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign82500_e125431;

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2913] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2913] = if s.b[2913] { 1.0 } else { 0.0 };

        s.b[2914] = (2.0 == 1.0);
        s.v[2914] = if s.b[2914] { 1.0 } else { 0.0 };

        let (assign82610_e125580,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && s.b[2914]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign82610_e125580;

        s.b[2915] = (2.0 == 2.0);
        s.v[2915] = if s.b[2915] { 1.0 } else { 0.0 };

        let (assign82630_e125603,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && s.b[2915]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign82630_e125603;

        s.b[2916] = (2.0 == 4.0);
        s.v[2916] = if s.b[2916] { 1.0 } else { 0.0 };

        let (assign82650_e125629,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && (!s.b[2915])) && s.b[2916]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign82650_e125629;

        s.b[2917] = (2.0 == 8.0);
        s.v[2917] = if s.b[2917] { 1.0 } else { 0.0 };

        let (assign82670_e125658,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && (!s.b[2915])) && (!s.b[2916])) && s.b[2917]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign82670_e125658;

        let (assign82680_e125673,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign82680_e125673;

        let mut assign82690_loop_guard: usize = 0;
        while {
            let assign82690_cond_e125689: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign82690_cond_e125689 != 0.0
        } {
            assign82690_loop_guard += 1;
            assert!(assign82690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) {
                s.store_sqrt(726, 726);
            }
            let (assign82690_body1_e125722,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) {
        let assign82690_body1_e125720: f64 = (s.v[719] + 1.0);
        (assign82690_body1_e125720,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign82690_body1_e125722;
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && (!s.b[2913])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && (!s.b[2912])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && (!s.b[2911])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2918] = (p.p33 == 1.0);
        s.v[2918] = if s.b[2918] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2919] = (s.v[411] > 0.0);
        s.v[2919] = if s.b[2919] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2919]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && (!s.b[2919])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2920] = (s.v[336] < 0.0);
        s.v[2920] = if s.b[2920] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && (!s.b[2919])) && s.b[2920]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && (!s.b[2919])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2921] = (s.v[336] < 0.0);
        s.v[2921] = if s.b[2921] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2921]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2881, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2922] = (s.v[333] < 60.0);
        s.v[2922] = if s.b[2922] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2922]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && (!s.b[2922])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2923] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2923] = if s.b[2923] { 1.0 } else { 0.0 };

        let (assign83120_e126311,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2923]) {
        let assign83120_e126309: f64 = (s.v[2887] + 1.0);
        (assign83120_e126309,)
    } else {
        (s.v[2887],)
    }
};
        s.v[2887] = assign83120_e126311;

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2923]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2924] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2924] = if s.b[2924] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2924]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2924])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2925, 354, 2881);
        }

        s.b[2927] = (p.p33 == 2.0);
        s.v[2927] = if s.b[2927] { 1.0 } else { 0.0 };

        s.b[2928] = ((s.v[2925] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2928] = if s.b[2928] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
            s.store_add_scaled_inputs3_indices(781, 2925, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83300_e126518,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign83300_e126518;

        let (assign83310_e126529,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83310_e126529;

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2929] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2929] = if s.b[2929] { 1.0 } else { 0.0 };

        s.b[2930] = (2.0 == 1.0);
        s.v[2930] = if s.b[2930] { 1.0 } else { 0.0 };

        let (assign83420_e126660,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && s.b[2930]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83420_e126660;

        s.b[2931] = (2.0 == 2.0);
        s.v[2931] = if s.b[2931] { 1.0 } else { 0.0 };

        let (assign83440_e126681,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && s.b[2931]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83440_e126681;

        s.b[2932] = (2.0 == 4.0);
        s.v[2932] = if s.b[2932] { 1.0 } else { 0.0 };

        let (assign83460_e126705,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && (!s.b[2931])) && s.b[2932]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83460_e126705;

        s.b[2933] = (2.0 == 8.0);
        s.v[2933] = if s.b[2933] { 1.0 } else { 0.0 };

        let (assign83480_e126732,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && (!s.b[2931])) && (!s.b[2932])) && s.b[2933]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83480_e126732;

        let (assign83490_e126745,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign83490_e126745;

        let mut assign83500_loop_guard: usize = 0;
        while {
            let assign83500_cond_e126759: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83500_cond_e126759 != 0.0
        } {
            assign83500_loop_guard += 1;
            assert!(assign83500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) {
                s.store_sqrt(726, 726);
            }
            let (assign83500_body1_e126788,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) {
        let assign83500_body1_e126786: f64 = (s.v[719] + 1.0);
        (assign83500_body1_e126786,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign83500_body1_e126788;
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && (!s.b[2929])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && (!s.b[2928])) {
            s.copy_ad(335, 2925);
        }

    }

    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && (!s.b[2928])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2934] = (s.v[334] < 1.0);
        s.v[2934] = if s.b[2934] { 1.0 } else { 0.0 };

        let (assign83600_e126930,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2934]) {
        let assign83600_e126928: f64 = (s.v[2887] + 2.0);
        (assign83600_e126928,)
    } else {
        (s.v[2887],)
    }
};
        s.v[2887] = assign83600_e126930;

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2927])) {
            if (s.v[2925] <= s.v[386]) {
                s.copy_ad(335, 2925);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2935] = (s.v[2925] >= s.v[386]);
        s.v[2935] = if s.b[2935] { 1.0 } else { 0.0 };

        let (assign83630_e126962,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2927])) && s.b[2935]) {
        let assign83630_e126960: f64 = (s.v[2887] + 2.0);
        (assign83630_e126960,)
    } else {
        (s.v[2887],)
    }
};
        s.v[2887] = assign83630_e126962;

        s.b[2936] = (s.v[2887] >= 2.0);
        s.v[2936] = if s.b[2936] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) {
            s.copy_ad(2926, 404);
            s.store_mul(354, 335, 2881);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2937] = (p.p33 == 2.0);
        s.v[2937] = if s.b[2937] { 1.0 } else { 0.0 };

        s.b[2938] = ((s.v[404] > (s.v[2926] - 0.1)) && (0.1 >= 0.0));
        s.v[2938] = if s.b[2938] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
            s.store_offset_sub(781, 404, 2926, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83750_e127096,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign83750_e127096;

        let (assign83760_e127109,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83760_e127109;

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2939] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2939] = if s.b[2939] { 1.0 } else { 0.0 };

        s.b[2940] = (2.0 == 1.0);
        s.v[2940] = if s.b[2940] { 1.0 } else { 0.0 };

        let (assign83870_e127258,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83870_e127258;

        s.b[2941] = (2.0 == 2.0);
        s.v[2941] = if s.b[2941] { 1.0 } else { 0.0 };

        let (assign83890_e127281,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && s.b[2941]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83890_e127281;

        s.b[2942] = (2.0 == 4.0);
        s.v[2942] = if s.b[2942] { 1.0 } else { 0.0 };

        let (assign83910_e127307,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && (!s.b[2941])) && s.b[2942]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83910_e127307;

        s.b[2943] = (2.0 == 8.0);
        s.v[2943] = if s.b[2943] { 1.0 } else { 0.0 };

        let (assign83930_e127336,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign83930_e127336;

        let (assign83940_e127351,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign83940_e127351;

        let mut assign83950_loop_guard: usize = 0;
        while {
            let assign83950_cond_e127367: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83950_cond_e127367 != 0.0
        } {
            assign83950_loop_guard += 1;
            assert!(assign83950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) {
                s.store_sqrt(726, 726);
            }
            let (assign83950_body1_e127400,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) {
        let assign83950_body1_e127398: f64 = (s.v[719] + 1.0);
        (assign83950_body1_e127398,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign83950_body1_e127400;
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && (!s.b[2939])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2926, (-0.1), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && (!s.b[2937])) {
            if (s.v[404] <= s.v[2926]) {
            } else {
                s.copy_ad(404, 2926);
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.copy_ad(2888, 404);
        }

        s.b[2944] = (p.p33 == 1.0);
        s.v[2944] = if s.b[2944] { 1.0 } else { 0.0 };

        let (assign84070_e127572,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign84070_e127572;

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2881)), s.ad_value(155)), 2.0);
        }

        s.b[2945] = (s.v[411] > 0.0);
        s.v[2945] = if s.b[2945] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2945]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2945])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2946] = (s.v[336] < 0.0);
        s.v[2946] = if s.b[2946] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2945])) && s.b[2946]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2945])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2947] = (s.v[336] < 0.0);
        s.v[2947] = if s.b[2947] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2947]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2881, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign84300_e127881,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign84300_e127881;

    }

    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign84310_loop_guard: usize = 0;
        while {
            let assign84310_cond_e127891: f64 = (s.v[421] + 1.0);
            let assign84310_cond_e127893: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (s.v[97] <= assign84310_cond_e127891)) { 1.0 } else { 0.0 };
            assign84310_cond_e127893 != 0.0
        } {
            assign84310_loop_guard += 1;
            assert!(assign84310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2949] = (s.v[333] < 60.0);
            s.v[2949] = if s.b[2949] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2949]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2949])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2950] = (s.v[116] < 0.0);
            s.v[2950] = if s.b[2950] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2950]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2951] = (s.v[116] < 1e-6);
            s.v[2951] = if s.b[2951] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && s.b[2951]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2952] = (s.v[338] > 0.0);
            s.v[2952] = if s.b[2952] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && s.b[2951]) && s.b[2952]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && s.b[2951]) && (!s.b[2952])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && (!s.b[2951])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2953] = (s.v[338] > 0.0);
            s.v[2953] = if s.b[2953] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && (!s.b[2951])) && s.b[2953]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && (!s.b[2951])) && (!s.b[2953])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2954] = (s.v[116] < 0.0);
            s.v[2954] = if s.b[2954] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2954]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2955] = (s.v[116] < 60.0);
            s.v[2955] = if s.b[2955] { 1.0 } else { 0.0 };
            s.b[2956] = (s.v[116] < 5e-5);
            s.v[2956] = if s.b[2956] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && s.b[2955]) && s.b[2956]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && s.b[2955]) && (!s.b[2956])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && (!s.b[2955])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2957] = (s.v[214] > 0.0);
            s.v[2957] = if s.b[2957] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && s.b[2957]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && (!s.b[2957])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2958] = (s.v[79] == 1.0);
            s.v[2958] = if s.b[2958] { 1.0 } else { 0.0 };
            let (assign84310_body72_e129039,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2958]) {
        let assign84310_body72_e129037: f64 = (s.v[421] + 1.0);
        (assign84310_body72_e129037,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign84310_body72_e129039;
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2959] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2959] = if s.b[2959] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) && s.b[2959]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) {
                s.store_add(404, 404, 236);
            }
            s.b[2960] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2960] = if s.b[2960] { 1.0 } else { 0.0 };
            let (assign84310_body79_e129142,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) && s.b[2960]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign84310_body79_e129142;
            let (assign84310_body80_e129153,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
        let assign84310_body80_e129151: f64 = (s.v[97] + 1.0);
        (assign84310_body80_e129151,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign84310_body80_e129153;
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
            s.store_mul(2879, 982, 223);
            s.store_mul(2880, 2881, 2879);
            s.store_offset_div(100, 2880, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2962] = (p.p33 == 4.0);
        s.v[2962] = if s.b[2962] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2888);
        }

        let (assign84460_e129290,) = {
    if ((s.v[2623] != 0.0) && s.b[2962]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign84460_e129290;

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2881)), s.ad_value(155)), 2.0);
        }

        s.b[2963] = (s.v[411] > 0.0);
        s.v[2963] = if s.b[2963] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2963]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2963])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2964] = (s.v[336] < 0.0);
        s.v[2964] = if s.b[2964] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2963])) && s.b[2964]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2963])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2965] = (s.v[336] < 0.0);
        s.v[2965] = if s.b[2965] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2965]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2881, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign84690_e129539,) = {
    if ((s.v[2623] != 0.0) && s.b[2962]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign84690_e129539;

    }

    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign84700_loop_guard: usize = 0;
        while {
            let assign84700_cond_e129546: f64 = (s.v[421] + 1.0);
            let assign84700_cond_e129548: f64 = if (((s.v[2623] != 0.0) && s.b[2962]) && (s.v[97] <= assign84700_cond_e129546)) { 1.0 } else { 0.0 };
            assign84700_cond_e129548 != 0.0
        } {
            assign84700_loop_guard += 1;
            assert!(assign84700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2967] = (s.v[333] < 60.0);
            s.v[2967] = if s.b[2967] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2967]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2967])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2968] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2968] = if s.b[2968] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2968]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2889, 334, 336);
                s.store_mul_add_scaled_product_rhs(2890, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2968])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2889, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2890, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2969] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2969] = if s.b[2969] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2969]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2970] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2970] = if s.b[2970] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2969])) && s.b[2970]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2969])) && (!s.b[2970])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2971] = (s.v[214] > 0.0);
            s.v[2971] = if s.b[2971] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2971]) {
                s.store_sqrt_add(216, 2889, 214);
                s.store_div_scaled_inputs2_indices(217, 2890, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2972] = (s.v[2889] > 0.0);
            s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2971])) && s.b[2972]) {
                s.store_sqrt(216, 2889);
                s.store_div_scaled_inputs_indices(217, 2890, 0.5, 216, 1.0);
            }
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2971])) && (!s.b[2972])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2973] = (s.v[79] > 0.0);
            s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };
            let (assign84700_body56_e130288,) = {
    if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2973]) {
        let assign84700_body56_e130286: f64 = (s.v[421] + 1.0);
        (assign84700_body56_e130286,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign84700_body56_e130288;
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2974] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2974] = if s.b[2974] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) && s.b[2974]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) {
                s.store_add(404, 404, 236);
            }
            s.b[2975] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2975] = if s.b[2975] { 1.0 } else { 0.0 };
            let (assign84700_body63_e130378,) = {
    if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) && s.b[2975]) {
        let assign84700_body63_e130376: f64 = (s.v[79] + 2.0);
        (assign84700_body63_e130376,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign84700_body63_e130378;
            let (assign84700_body64_e130386,) = {
    if ((s.v[2623] != 0.0) && s.b[2962]) {
        let assign84700_body64_e130384: f64 = (s.v[97] + 1.0);
        (assign84700_body64_e130384,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign84700_body64_e130386;
        }

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            if (s.v[2889] >= 0.0) {
                s.store_scaled_sqrt(223, 2889, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.store_mul(2879, 982, 223);
            s.store_mul(2880, 2881, 2879);
            s.store_offset_div(100, 2880, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2623] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2977] = (s.v[407] < 0.0);
        s.v[2977] = if s.b[2977] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2977]) {
            s.store_neg(407, 407);
        }

        s.b[2978] = (p.p55 == 0.0);
        s.v[2978] = if s.b[2978] { 1.0 } else { 0.0 };

        s.b[2979] = (p.p50 == 0.0);
        s.v[2979] = if s.b[2979] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) && s.b[2979]) {
            s.store_neg(2882, 404);
        }

        if ((((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) && (!s.b[2979])) {
            s.copy_ad(2882, 396);
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            s.store_sqrt_offset_square_offset(782, 2882, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2882), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2882), p.p137), 782, 0.5);
        }

        s.b[2980] = (s.v[336] < 0.0);
        s.v[2980] = if s.b[2980] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) && s.b[2980]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2981] = (3.0 == 1.0);
        s.v[2981] = if s.b[2981] { 1.0 } else { 0.0 };

        s.b[2982] = (3.0 == 2.0);
        s.v[2982] = if s.b[2982] { 1.0 } else { 0.0 };

        s.b[2983] = (3.0 == 3.0);
        s.v[2983] = if s.b[2983] { 1.0 } else { 0.0 };

        s.b[2984] = (3.0 == 4.0);
        s.v[2984] = if s.b[2984] { 1.0 } else { 0.0 };

        s.b[2985] = (p.p55 == 1.0);
        s.v[2985] = if s.b[2985] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2981]) && s.b[2985]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && s.b[2981]) && (!s.b[2985])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && s.b[2981]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2982] && (!s.b[2981]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2986] = (p.p55 == 1.0);
        s.v[2986] = if s.b[2986] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2983] && (!(s.b[2981] || s.b[2982])))) && s.b[2986]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && (s.b[2983] && (!(s.b[2981] || s.b[2982])))) && (!s.b[2986])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && (s.b[2983] && (!(s.b[2981] || s.b[2982])))) {
            s.copy_ad(697, 404);
        }

        s.b[2987] = (p.p430 == 0.0);
        s.v[2987] = if s.b[2987] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2983] && (!(s.b[2981] || s.b[2982])))) && s.b[2987]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2983] && (!(s.b[2981] || s.b[2982])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2623] != 0.0) && (s.b[2984] && (!((s.b[2981] || s.b[2982]) || s.b[2983])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2623] = 0.0;

        s.b[2988] = (4.0 == 1.0);
        s.v[2988] = if s.b[2988] { 1.0 } else { 0.0 };

        s.b[2989] = (4.0 == 2.0);
        s.v[2989] = if s.b[2989] { 1.0 } else { 0.0 };

        s.b[2990] = (4.0 == 3.0);
        s.v[2990] = if s.b[2990] { 1.0 } else { 0.0 };

        s.b[2991] = (4.0 == 4.0);
        s.v[2991] = if s.b[2991] { 1.0 } else { 0.0 };

        s.b[2992] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2992] = if s.b[2992] { 1.0 } else { 0.0 };

        let (assign85330_e130979,) = {
    if (s.b[2988] && s.b[2992]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign85330_e130979;

        let (assign85340_e130985,) = {
    if (s.b[2988] && s.b[2992]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign85340_e130985;

        if (s.b[2988] && s.b[2992]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2993] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2993] = if s.b[2993] { 1.0 } else { 0.0 };

        let (assign85430_e131058,) = {
    if ((s.b[2989] && (!s.b[2988])) && s.b[2993]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign85430_e131058;

        if ((s.b[2989] && (!s.b[2988])) && s.b[2993]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2994] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        let (assign85470_e131101,) = {
    if ((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign85470_e131101;

    }

    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let (assign85480_e131112,) = {
    if ((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.v[2624] = assign85480_e131112;

        if ((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2995] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2995] = if s.b[2995] { 1.0 } else { 0.0 };

        if (((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2996] = (p.p113 > 0.0);
        s.v[2996] = if s.b[2996] { 1.0 } else { 0.0 };

        s.b[2997] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2997] = if s.b[2997] { 1.0 } else { 0.0 };

        if (((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) && s.b[2997]) {
        }

        if (((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) && (!s.b[2997])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) && (!s.b[2997])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2998] = (s.v[336] < 0.0);
        s.v[2998] = if s.b[2998] { 1.0 } else { 0.0 };

        if (((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) && s.b[2998]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2999] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2999] = if s.b[2999] { 1.0 } else { 0.0 };

        let (assign85780_e131583,) = {
    if ((s.b[2991] && (!((s.b[2988] || s.b[2989]) || s.b[2990]))) && s.b[2999]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.v[2623] = assign85780_e131583;

        if ((s.b[2991] && (!((s.b[2988] || s.b[2989]) || s.b[2990]))) && s.b[2999]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(3007, 0.4);
        }

        let (assign85830_e131625,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[3008],)
    }
};
        s.v[3008] = assign85830_e131625;

        if (s.v[2623] != 0.0) {
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
        }

        let (assign85960_e131678,) = {
    if (s.v[2623] != 0.0) {
        let assign85960_e131676: f64 = (-1.0);
        (assign85960_e131676,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign85960_e131678;

        if (s.v[2623] != 0.0) {
            s.store_scalar(3009, 0.0);
            s.store_scalar(3010, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3005, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3005), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3006, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3012] = (s.v[3007] > (s.v[3006] * 0.5));
        s.v[3012] = if s.b[3012] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3012]) {
            s.store_scale(3007, 3006, 0.5);
        }

        s.b[3013] = param_given[338];
        s.v[3013] = if s.b[3013] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3013]) {
            s.store_scalar(3006, p.p338);
        }

        s.b[3014] = param_given[339];
        s.v[3014] = if s.b[3014] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3014]) {
            s.store_scalar(3007, p.p339);
        }

        s.b[3015] = param_given[338];
        s.v[3015] = if s.b[3015] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3014])) && s.b[3015]) {
            s.store_scale(3007, 3006, 0.5);
        }

        s.b[3016] = (s.v[3007] > (s.v[3006] * 0.5));
        s.v[3016] = if s.b[3016] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3016]) {
            s.store_scale(3007, 3006, 0.5);
        }

        s.b[3017] = (p.p38 == 1.0);
        s.v[3017] = if s.b[3017] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3017]) {
            s.store_neg(334, 396);
        }

        s.b[3018] = (s.v[334] > s.v[3007]);
        s.v[3018] = if s.b[3018] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[3017]) && s.b[3018]) {
            s.store_sub(335, 334, 3007);
            s.store_sub(336, 3006, 3007);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3007, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[3017]) && (!s.b[3018])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[3017]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[3017])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign86370_e132019,) = {
    if (s.v[2623] != 0.0) {
        let assign86370_e132013: f64 = (-s.v[397]);
        let assign86370_e132016: f64 = (10.0 * 2.220446049250313e-16);
        let assign86370_e132017: f64 = (assign86370_e132013 + assign86370_e132016);
        (assign86370_e132017,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign86370_e132019;

        if (s.v[2623] != 0.0) {
            s.store_scalar(3001, 0.0);
            s.store_scale(3002, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3019] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3019] = if s.b[3019] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3019]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[3019])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign86470_loop_guard: usize = 0;
        while {
            let assign86470_cond_e132093: f64 = if (((s.v[2623] != 0.0) && (!s.b[3019])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign86470_cond_e132093 != 0.0
        } {
            assign86470_loop_guard += 1;
            assert!(assign86470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[3019])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[3019])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3020] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign86620_e132267,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign86620_e132267;

        let (assign86630_e132275,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign86630_e132275;

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
            s.store_scalar(770, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3021] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

        s.b[3022] = (1.0 == 1.0);
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        let (assign86720_e132359,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && s.b[3022]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign86720_e132359;

        s.b[3023] = (1.0 == 2.0);
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        let (assign86740_e132377,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && s.b[3023]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign86740_e132377;

        s.b[3024] = (1.0 == 4.0);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        let (assign86760_e132398,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && (!s.b[3023])) && s.b[3024]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign86760_e132398;

        s.b[3025] = (1.0 == 8.0);
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        let (assign86780_e132422,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && (!s.b[3023])) && (!s.b[3024])) && s.b[3025]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign86780_e132422;

        let (assign86790_e132432,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign86790_e132432;

        let mut assign86800_loop_guard: usize = 0;
        while {
            let assign86800_cond_e132443: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86800_cond_e132443 != 0.0
        } {
            assign86800_loop_guard += 1;
            assert!(assign86800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) {
                s.store_sqrt(726, 726);
            }
            let (assign86800_body1_e132466,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) {
        let assign86800_body1_e132464: f64 = (s.v[719] + 1.0);
        (assign86800_body1_e132464,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign86800_body1_e132466;
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && (!s.b[3021])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3020])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign86900_e132583,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let assign86900_e132577: f64 = (-s.v[397]);
        let assign86900_e132580: f64 = (10.0 * 2.220446049250313e-16);
        let assign86900_e132581: f64 = (assign86900_e132577 + assign86900_e132580);
        (assign86900_e132581,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign86900_e132583;

        s.b[3026] = (s.v[402] < s.v[403]);
        s.v[3026] = if s.b[3026] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3026]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3027] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[3026]) && s.b[3027]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2623] != 0.0) && s.b[3026]) && (!s.b[3027])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[3026]) {
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
            s.copy_ad(3009, 404);
        }

        s.b[3028] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3028] = if s.b[3028] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3028]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && (!s.b[3028])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[3026])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3029] = (s.v[116] >= 3.0);
        s.v[3029] = if s.b[3029] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3029]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && (!s.b[3029])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(116, 439, 1.0, 438, 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3030] = (p.p33 > 0.0);
        s.v[3030] = if s.b[3030] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3031] = (p.p33 == 2.0);
        s.v[3031] = if s.b[3031] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3031]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3031]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3031]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && (!s.b[3031])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) {
            s.copy_ad(445, 116);
        }

        s.b[3032] = (p.p33 == 2.0);
        s.v[3032] = if s.b[3032] { 1.0 } else { 0.0 };

        s.b[3033] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3033] = if s.b[3033] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign87730_e133729,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign87730_e133729;

        let (assign87740_e133742,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign87740_e133742;

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3034] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3034] = if s.b[3034] { 1.0 } else { 0.0 };

        s.b[3035] = (2.0 == 1.0);
        s.v[3035] = if s.b[3035] { 1.0 } else { 0.0 };

        let (assign87850_e133891,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) && s.b[3035]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign87850_e133891;

        s.b[3036] = (2.0 == 2.0);
        s.v[3036] = if s.b[3036] { 1.0 } else { 0.0 };

        let (assign87870_e133914,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) && (!s.b[3035])) && s.b[3036]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign87870_e133914;

        s.b[3037] = (2.0 == 4.0);
        s.v[3037] = if s.b[3037] { 1.0 } else { 0.0 };

        let (assign87890_e133940,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) && (!s.b[3035])) && (!s.b[3036])) && s.b[3037]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign87890_e133940;

        s.b[3038] = (2.0 == 8.0);
        s.v[3038] = if s.b[3038] { 1.0 } else { 0.0 };

        let (assign87910_e133969,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) && (!s.b[3035])) && (!s.b[3036])) && (!s.b[3037])) && s.b[3038]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign87910_e133969;

        let (assign87920_e133984,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign87920_e133984;

        let mut assign87930_loop_guard: usize = 0;
        while {
            let assign87930_cond_e134000: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign87930_cond_e134000 != 0.0
        } {
            assign87930_loop_guard += 1;
            assert!(assign87930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) {
                s.store_sqrt(726, 726);
            }
            let (assign87930_body1_e134033,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && s.b[3034]) {
        let assign87930_body1_e134031: f64 = (s.v[719] + 1.0);
        (assign87930_body1_e134031,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign87930_body1_e134033;
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) && (!s.b[3034])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && s.b[3033]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && s.b[3032]) && (!s.b[3033])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3030]) && (!s.b[3032])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3039] = (p.p33 == 1.0);
        s.v[3039] = if s.b[3039] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3040] = (s.v[411] > 0.0);
        s.v[3040] = if s.b[3040] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && s.b[3040]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && (!s.b[3040])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3041] = (s.v[336] < 0.0);
        s.v[3041] = if s.b[3041] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && (!s.b[3040])) && s.b[3041]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && (!s.b[3040])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3042] = (s.v[336] < 0.0);
        s.v[3042] = if s.b[3042] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && s.b[3042]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3002, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3043] = (s.v[333] < 60.0);
        s.v[3043] = if s.b[3043] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && s.b[3043]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && (!s.b[3043])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3044] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3044] = if s.b[3044] { 1.0 } else { 0.0 };

        let (assign88360_e134622,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && s.b[3044]) {
        let assign88360_e134620: f64 = (s.v[3008] + 1.0);
        (assign88360_e134620,)
    } else {
        (s.v[3008],)
    }
};
        s.v[3008] = assign88360_e134622;

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3039]) && s.b[3044]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[3026])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3045] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3045] = if s.b[3045] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3045]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && (!s.b[3045])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[3026])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3046, 354, 3002);
        }

        s.b[3048] = (p.p33 == 2.0);
        s.v[3048] = if s.b[3048] { 1.0 } else { 0.0 };

        s.b[3049] = ((s.v[3046] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3049] = if s.b[3049] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
            s.store_add_scaled_inputs3_indices(781, 3046, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign88540_e134829,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign88540_e134829;

        let (assign88550_e134840,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign88550_e134840;

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3050] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3050] = if s.b[3050] { 1.0 } else { 0.0 };

        s.b[3051] = (2.0 == 1.0);
        s.v[3051] = if s.b[3051] { 1.0 } else { 0.0 };

        let (assign88660_e134971,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) && s.b[3051]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign88660_e134971;

        s.b[3052] = (2.0 == 2.0);
        s.v[3052] = if s.b[3052] { 1.0 } else { 0.0 };

        let (assign88680_e134992,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) && (!s.b[3051])) && s.b[3052]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign88680_e134992;

        s.b[3053] = (2.0 == 4.0);
        s.v[3053] = if s.b[3053] { 1.0 } else { 0.0 };

        let (assign88700_e135016,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) && (!s.b[3051])) && (!s.b[3052])) && s.b[3053]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign88700_e135016;

        s.b[3054] = (2.0 == 8.0);
        s.v[3054] = if s.b[3054] { 1.0 } else { 0.0 };

        let (assign88720_e135043,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) && (!s.b[3051])) && (!s.b[3052])) && (!s.b[3053])) && s.b[3054]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign88720_e135043;

        let (assign88730_e135056,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign88730_e135056;

        let mut assign88740_loop_guard: usize = 0;
        while {
            let assign88740_cond_e135070: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign88740_cond_e135070 != 0.0
        } {
            assign88740_loop_guard += 1;
            assert!(assign88740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) {
                s.store_sqrt(726, 726);
            }
            let (assign88740_body1_e135099,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && s.b[3050]) {
        let assign88740_body1_e135097: f64 = (s.v[719] + 1.0);
        (assign88740_body1_e135097,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign88740_body1_e135099;
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) && (!s.b[3050])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3049]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && (!s.b[3049])) {
            s.copy_ad(335, 3046);
        }

    }

    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && (!s.b[3049])) {
            s.store_scalar(334, 1.0);
        }

        s.b[3055] = (s.v[334] < 1.0);
        s.v[3055] = if s.b[3055] { 1.0 } else { 0.0 };

        let (assign88840_e135241,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3048]) && s.b[3055]) {
        let assign88840_e135239: f64 = (s.v[3008] + 2.0);
        (assign88840_e135239,)
    } else {
        (s.v[3008],)
    }
};
        s.v[3008] = assign88840_e135241;

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && (!s.b[3048])) {
            if (s.v[3046] <= s.v[386]) {
                s.copy_ad(335, 3046);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3056] = (s.v[3046] >= s.v[386]);
        s.v[3056] = if s.b[3056] { 1.0 } else { 0.0 };

        let (assign88870_e135273,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && (!s.b[3048])) && s.b[3056]) {
        let assign88870_e135271: f64 = (s.v[3008] + 2.0);
        (assign88870_e135271,)
    } else {
        (s.v[3008],)
    }
};
        s.v[3008] = assign88870_e135273;

        s.b[3057] = (s.v[3008] >= 2.0);
        s.v[3057] = if s.b[3057] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) {
            s.copy_ad(3047, 404);
            s.store_mul(354, 335, 3002);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[3058] = (p.p33 == 2.0);
        s.v[3058] = if s.b[3058] { 1.0 } else { 0.0 };

        s.b[3059] = ((s.v[404] > (s.v[3047] - 0.1)) && (0.1 >= 0.0));
        s.v[3059] = if s.b[3059] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
            s.store_offset_sub(781, 404, 3047, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign88990_e135407,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign88990_e135407;

        let (assign89000_e135420,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign89000_e135420;

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3060] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3060] = if s.b[3060] { 1.0 } else { 0.0 };

        s.b[3061] = (2.0 == 1.0);
        s.v[3061] = if s.b[3061] { 1.0 } else { 0.0 };

        let (assign89110_e135569,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign89110_e135569;

        s.b[3062] = (2.0 == 2.0);
        s.v[3062] = if s.b[3062] { 1.0 } else { 0.0 };

        let (assign89130_e135592,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) && s.b[3062]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign89130_e135592;

        s.b[3063] = (2.0 == 4.0);
        s.v[3063] = if s.b[3063] { 1.0 } else { 0.0 };

        let (assign89150_e135618,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) && (!s.b[3062])) && s.b[3063]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign89150_e135618;

        s.b[3064] = (2.0 == 8.0);
        s.v[3064] = if s.b[3064] { 1.0 } else { 0.0 };

        let (assign89170_e135647,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) && (!s.b[3062])) && (!s.b[3063])) && s.b[3064]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign89170_e135647;

        let (assign89180_e135662,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign89180_e135662;

        let mut assign89190_loop_guard: usize = 0;
        while {
            let assign89190_cond_e135678: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign89190_cond_e135678 != 0.0
        } {
            assign89190_loop_guard += 1;
            assert!(assign89190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) {
                s.store_sqrt(726, 726);
            }
            let (assign89190_body1_e135711,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && s.b[3060]) {
        let assign89190_body1_e135709: f64 = (s.v[719] + 1.0);
        (assign89190_body1_e135709,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign89190_body1_e135711;
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) && (!s.b[3060])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 3047, (-0.1), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3057]) && (!s.b[3058])) {
            if (s.v[404] <= s.v[3047]) {
            } else {
                s.copy_ad(404, 3047);
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[3026])) {
            s.copy_ad(3009, 404);
        }

        s.b[3065] = (p.p33 == 1.0);
        s.v[3065] = if s.b[3065] { 1.0 } else { 0.0 };

        let (assign89310_e135883,) = {
    if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign89310_e135883;

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3002)), s.ad_value(155)), 2.0);
        }

        s.b[3066] = (s.v[411] > 0.0);
        s.v[3066] = if s.b[3066] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3066]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3066])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3067] = (s.v[336] < 0.0);
        s.v[3067] = if s.b[3067] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3066])) && s.b[3067]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3066])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3068] = (s.v[336] < 0.0);
        s.v[3068] = if s.b[3068] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3068]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3002, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign89540_e136192,) = {
    if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign89540_e136192;

    }

    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign89550_loop_guard: usize = 0;
        while {
            let assign89550_cond_e136202: f64 = (s.v[421] + 1.0);
            let assign89550_cond_e136204: f64 = if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (s.v[97] <= assign89550_cond_e136202)) { 1.0 } else { 0.0 };
            assign89550_cond_e136204 != 0.0
        } {
            assign89550_loop_guard += 1;
            assert!(assign89550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3070] = (s.v[333] < 60.0);
            s.v[3070] = if s.b[3070] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3070]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3070])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3071] = (s.v[116] < 0.0);
            s.v[3071] = if s.b[3071] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3071]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3072] = (s.v[116] < 1e-6);
            s.v[3072] = if s.b[3072] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && s.b[3072]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[3073] = (s.v[338] > 0.0);
            s.v[3073] = if s.b[3073] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && s.b[3072]) && s.b[3073]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && s.b[3072]) && (!s.b[3073])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && (!s.b[3072])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[3074] = (s.v[338] > 0.0);
            s.v[3074] = if s.b[3074] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && (!s.b[3072])) && s.b[3074]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3071])) && (!s.b[3072])) && (!s.b[3074])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3075] = (s.v[116] < 0.0);
            s.v[3075] = if s.b[3075] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3075]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3076] = (s.v[116] < 60.0);
            s.v[3076] = if s.b[3076] { 1.0 } else { 0.0 };
            s.b[3077] = (s.v[116] < 5e-5);
            s.v[3077] = if s.b[3077] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3075])) && s.b[3076]) && s.b[3077]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3075])) && s.b[3076]) && (!s.b[3077])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3075])) && (!s.b[3076])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3078] = (s.v[214] > 0.0);
            s.v[3078] = if s.b[3078] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3075])) && s.b[3078]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3075])) && (!s.b[3078])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3079] = (s.v[79] == 1.0);
            s.v[3079] = if s.b[3079] { 1.0 } else { 0.0 };
            let (assign89550_body72_e137350,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && s.b[3079]) {
        let assign89550_body72_e137348: f64 = (s.v[421] + 1.0);
        (assign89550_body72_e137348,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign89550_body72_e137350;
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3079])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3079])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3080] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3080] = if s.b[3080] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3079])) && s.b[3080]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3079])) {
                s.store_add(404, 404, 236);
            }
            s.b[3081] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3081] = if s.b[3081] { 1.0 } else { 0.0 };
            let (assign89550_body79_e137453,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) && (!s.b[3079])) && s.b[3081]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign89550_body79_e137453;
            let (assign89550_body80_e137464,) = {
    if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
        let assign89550_body80_e137462: f64 = (s.v[97] + 1.0);
        (assign89550_body80_e137462,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign89550_body80_e137464;
        }

        if (((s.v[2623] != 0.0) && (!s.b[3026])) && s.b[3065]) {
            s.store_mul(3000, 982, 223);
            s.store_mul(3001, 3002, 3000);
            s.store_offset_div(100, 3001, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3083] = (p.p33 == 4.0);
        s.v[3083] = if s.b[3083] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3009);
        }

        let (assign89700_e137601,) = {
    if ((s.v[2623] != 0.0) && s.b[3083]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign89700_e137601;

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3002)), s.ad_value(155)), 2.0);
        }

        s.b[3084] = (s.v[411] > 0.0);
        s.v[3084] = if s.b[3084] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3084]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3084])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3085] = (s.v[336] < 0.0);
        s.v[3085] = if s.b[3085] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3084])) && s.b[3085]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3084])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3086] = (s.v[336] < 0.0);
        s.v[3086] = if s.b[3086] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3086]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3002, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign89930_e137850,) = {
    if ((s.v[2623] != 0.0) && s.b[3083]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign89930_e137850;

    }

    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign89940_loop_guard: usize = 0;
        while {
            let assign89940_cond_e137857: f64 = (s.v[421] + 1.0);
            let assign89940_cond_e137859: f64 = if (((s.v[2623] != 0.0) && s.b[3083]) && (s.v[97] <= assign89940_cond_e137857)) { 1.0 } else { 0.0 };
            assign89940_cond_e137859 != 0.0
        } {
            assign89940_loop_guard += 1;
            assert!(assign89940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[3083]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3088] = (s.v[333] < 60.0);
            s.v[3088] = if s.b[3088] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3088]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3088])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2623] != 0.0) && s.b[3083]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3089] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[3089] = if s.b[3089] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3089]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(3010, 334, 336);
                s.store_mul_add_scaled_product_rhs(3011, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3089])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(3010, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(3011, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[3090] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[3090] = if s.b[3090] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3090]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3091] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3090])) && s.b[3091]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3090])) && (!s.b[3091])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3092] = (s.v[214] > 0.0);
            s.v[3092] = if s.b[3092] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3092]) {
                s.store_sqrt_add(216, 3010, 214);
                s.store_div_scaled_inputs2_indices(217, 3011, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[3093] = (s.v[3010] > 0.0);
            s.v[3093] = if s.b[3093] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3092])) && s.b[3093]) {
                s.store_sqrt(216, 3010);
                s.store_div_scaled_inputs_indices(217, 3011, 0.5, 216, 1.0);
            }
            if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3092])) && (!s.b[3093])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2623] != 0.0) && s.b[3083]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[3083]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[3083]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3094] = (s.v[79] > 0.0);
            s.v[3094] = if s.b[3094] { 1.0 } else { 0.0 };
            let (assign89940_body56_e138599,) = {
    if (((s.v[2623] != 0.0) && s.b[3083]) && s.b[3094]) {
        let assign89940_body56_e138597: f64 = (s.v[421] + 1.0);
        (assign89940_body56_e138597,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign89940_body56_e138599;
            if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3094])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3094])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3095] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3095] = if s.b[3095] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3094])) && s.b[3095]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3094])) {
                s.store_add(404, 404, 236);
            }
            s.b[3096] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3096] = if s.b[3096] { 1.0 } else { 0.0 };
            let (assign89940_body63_e138689,) = {
    if ((((s.v[2623] != 0.0) && s.b[3083]) && (!s.b[3094])) && s.b[3096]) {
        let assign89940_body63_e138687: f64 = (s.v[79] + 2.0);
        (assign89940_body63_e138687,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign89940_body63_e138689;
            let (assign89940_body64_e138697,) = {
    if ((s.v[2623] != 0.0) && s.b[3083]) {
        let assign89940_body64_e138695: f64 = (s.v[97] + 1.0);
        (assign89940_body64_e138695,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign89940_body64_e138697;
        }

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            if (s.v[3010] >= 0.0) {
                s.store_scaled_sqrt(223, 3010, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[3083]) {
            s.store_mul(3000, 982, 223);
            s.store_mul(3001, 3002, 3000);
            s.store_offset_div(100, 3001, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2623] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[3098] = (s.v[407] < 0.0);
        s.v[3098] = if s.b[3098] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[3098]) {
            s.store_neg(407, 407);
        }

        s.b[3099] = (p.p55 == 0.0);
        s.v[3099] = if s.b[3099] { 1.0 } else { 0.0 };

        s.b[3100] = (p.p50 == 0.0);
        s.v[3100] = if s.b[3100] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) && s.b[3100]) {
            s.store_neg(3003, 404);
        }

        if ((((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) && (!s.b[3100])) {
            s.copy_ad(3003, 396);
        }

        if (((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) {
            s.store_sqrt_offset_square_offset(782, 3003, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3003), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3003), p.p137), 782, 0.5);
        }

        s.b[3101] = (s.v[336] < 0.0);
        s.v[3101] = if s.b[3101] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) && s.b[3101]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2623] != 0.0) && s.b[3098]) && s.b[3099]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[3102] = (4.0 == 1.0);
        s.v[3102] = if s.b[3102] { 1.0 } else { 0.0 };

        s.b[3103] = (4.0 == 2.0);
        s.v[3103] = if s.b[3103] { 1.0 } else { 0.0 };

        s.b[3104] = (4.0 == 3.0);
        s.v[3104] = if s.b[3104] { 1.0 } else { 0.0 };

        s.b[3105] = (4.0 == 4.0);
        s.v[3105] = if s.b[3105] { 1.0 } else { 0.0 };

        s.b[3106] = (p.p55 == 1.0);
        s.v[3106] = if s.b[3106] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[3102]) && s.b[3106]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && s.b[3102]) && (!s.b[3106])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && s.b[3102]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[3103] && (!s.b[3102]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[3107] = (p.p55 == 1.0);
        s.v[3107] = if s.b[3107] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[3104] && (!(s.b[3102] || s.b[3103])))) && s.b[3107]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && (s.b[3104] && (!(s.b[3102] || s.b[3103])))) && (!s.b[3107])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && (s.b[3104] && (!(s.b[3102] || s.b[3103])))) {
            s.copy_ad(697, 404);
        }

        s.b[3108] = (p.p430 == 0.0);
        s.v[3108] = if s.b[3108] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[3104] && (!(s.b[3102] || s.b[3103])))) && s.b[3108]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[3104] && (!(s.b[3102] || s.b[3103])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2623] != 0.0) && (s.b[3105] && (!((s.b[3102] || s.b[3103]) || s.b[3104])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.b[3109] = (p.p430 > 0.0);
        s.v[3109] = if s.b[3109] { 1.0 } else { 0.0 };

        let (assign90520_e139267,) = {
    if s.b[3109] {
        (1.0,)
    } else {
        (s.v[406],)
    }
};
        s.v[406] = assign90520_e139267;

        s.b[3110] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[3110] = if s.b[3110] { 1.0 } else { 0.0 };

        if (s.b[3109] && s.b[3110]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, 0.0);
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_scalar(3118, 0.4);
        }

        let (assign90630_e139342,) = {
    if (s.b[3109] && s.b[3110]) {
        (0.0,)
    } else {
        (s.v[3119],)
    }
};
        s.v[3119] = assign90630_e139342;

        if (s.b[3109] && s.b[3110]) {
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
        }

    }

    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[3109] && s.b[3110]) {
            s.store_scalar(446, 0.0);
        }

        let (assign90760_e139421,) = {
    if (s.b[3109] && s.b[3110]) {
        let assign90760_e139419: f64 = (-1.0);
        (assign90760_e139419,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign90760_e139421;

        if (s.b[3109] && s.b[3110]) {
            s.store_scalar(3120, 0.0);
            s.store_scalar(3121, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3116, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3116), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.b[3109] && s.b[3110]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3109] && s.b[3110]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3117, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3123] = (s.v[3118] > (s.v[3117] * 0.5));
        s.v[3123] = if s.b[3123] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3123]) {
            s.store_scale(3118, 3117, 0.5);
        }

        s.b[3124] = param_given[338];
        s.v[3124] = if s.b[3124] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3124]) {
            s.store_scalar(3117, p.p338);
        }

        s.b[3125] = param_given[339];
        s.v[3125] = if s.b[3125] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3125]) {
            s.store_scalar(3118, p.p339);
        }

        s.b[3126] = param_given[338];
        s.v[3126] = if s.b[3126] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3125])) && s.b[3126]) {
            s.store_scale(3118, 3117, 0.5);
        }

        s.b[3127] = (s.v[3118] > (s.v[3117] * 0.5));
        s.v[3127] = if s.b[3127] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3127]) {
            s.store_scale(3118, 3117, 0.5);
        }

        s.b[3128] = (p.p38 == 1.0);
        s.v[3128] = if s.b[3128] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3128]) {
            s.store_neg(334, 396);
        }

        s.b[3129] = (s.v[334] > s.v[3118]);
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && s.b[3128]) && s.b[3129]) {
            s.store_sub(335, 334, 3118);
            s.store_sub(336, 3117, 3118);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3118, 333);
        }

        if (((s.b[3109] && s.b[3110]) && s.b[3128]) && (!s.b[3129])) {
            s.copy_ad(344, 334);
        }

        if ((s.b[3109] && s.b[3110]) && s.b[3128]) {
            s.store_neg(397, 344);
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3128])) {
            s.copy_ad(397, 396);
        }

        if (s.b[3109] && s.b[3110]) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign91170_e139830,) = {
    if (s.b[3109] && s.b[3110]) {
        let assign91170_e139824: f64 = (-s.v[397]);
        let assign91170_e139827: f64 = (10.0 * 2.220446049250313e-16);
        let assign91170_e139828: f64 = (assign91170_e139824 + assign91170_e139827);
        (assign91170_e139828,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign91170_e139830;

        if (s.b[3109] && s.b[3110]) {
            s.store_scalar(3112, 0.0);
            s.store_scale(3113, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3130] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3130] = if s.b[3130] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3130]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3130])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign91270_loop_guard: usize = 0;
        while {
            let assign91270_cond_e139922: f64 = if (((s.b[3109] && s.b[3110]) && (!s.b[3130])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91270_cond_e139922 != 0.0
        } {
            assign91270_loop_guard += 1;
            assert!(assign91270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3109] && s.b[3110]) && (!s.b[3130])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3130])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3131] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3131] = if s.b[3131] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign91420_e140128,) = {
    if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign91420_e140128;

        let (assign91430_e140138,) = {
    if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign91430_e140138;

        if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3132] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3132] = if s.b[3132] { 1.0 } else { 0.0 };

        s.b[3133] = (1.0 == 1.0);
        s.v[3133] = if s.b[3133] { 1.0 } else { 0.0 };

        let (assign91520_e140236,) = {
    if (((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) && s.b[3133]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign91520_e140236;

        s.b[3134] = (1.0 == 2.0);
        s.v[3134] = if s.b[3134] { 1.0 } else { 0.0 };

        let (assign91540_e140256,) = {
    if ((((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) && (!s.b[3133])) && s.b[3134]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign91540_e140256;

        s.b[3135] = (1.0 == 4.0);
        s.v[3135] = if s.b[3135] { 1.0 } else { 0.0 };

        let (assign91560_e140279,) = {
    if (((((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) && (!s.b[3133])) && (!s.b[3134])) && s.b[3135]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign91560_e140279;

        s.b[3136] = (1.0 == 8.0);
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        let (assign91580_e140305,) = {
    if ((((((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) && (!s.b[3133])) && (!s.b[3134])) && (!s.b[3135])) && s.b[3136]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign91580_e140305;

        let (assign91590_e140317,) = {
    if ((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign91590_e140317;

        let mut assign91600_loop_guard: usize = 0;
        while {
            let assign91600_cond_e140330: f64 = if (((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91600_cond_e140330 != 0.0
        } {
            assign91600_loop_guard += 1;
            assert!(assign91600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) {
                s.store_sqrt(726, 726);
            }
            let (assign91600_body1_e140357,) = {
    if ((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && s.b[3132]) {
        let assign91600_body1_e140355: f64 = (s.v[719] + 1.0);
        (assign91600_body1_e140355,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign91600_body1_e140357;
        }

        if ((((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) && (!s.b[3132])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && s.b[3131]) {
        }

        if (((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) && (!s.b[3131])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign91700_e140494,) = {
    if ((s.b[3109] && s.b[3110]) && (s.v[406] != 0.0)) {
        let assign91700_e140488: f64 = (-s.v[397]);
        let assign91700_e140491: f64 = (10.0 * 2.220446049250313e-16);
        let assign91700_e140492: f64 = (assign91700_e140488 + assign91700_e140491);
        (assign91700_e140492,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign91700_e140494;

        s.b[3137] = (s.v[402] < s.v[403]);
        s.v[3137] = if s.b[3137] { 1.0 } else { 0.0 };

        if ((s.b[3109] && s.b[3110]) && s.b[3137]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3138] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3109] && s.b[3110]) && s.b[3137]) && s.b[3138]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.b[3109] && s.b[3110]) && s.b[3137]) && (!s.b[3138])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.b[3109] && s.b[3110]) && s.b[3137]) {
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
            s.copy_ad(3120, 404);
        }

        s.b[3139] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3139] = if s.b[3139] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3139]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && (!s.b[3139])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3137])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3140] = (s.v[116] >= 3.0);
        s.v[3140] = if s.b[3140] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3140]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && (!s.b[3140])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(116, 439, 1.0, 438, 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3141] = (p.p33 > 0.0);
        s.v[3141] = if s.b[3141] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3142] = (p.p33 == 2.0);
        s.v[3142] = if s.b[3142] { 1.0 } else { 0.0 };

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3142]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3142]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3142]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && (!s.b[3142])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) {
            s.copy_ad(445, 116);
        }

        s.b[3143] = (p.p33 == 2.0);
        s.v[3143] = if s.b[3143] { 1.0 } else { 0.0 };

        s.b[3144] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3144] = if s.b[3144] { 1.0 } else { 0.0 };

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign92530_e141790,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign92530_e141790;

        let (assign92540_e141805,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign92540_e141805;

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3145] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3145] = if s.b[3145] { 1.0 } else { 0.0 };

        s.b[3146] = (2.0 == 1.0);
        s.v[3146] = if s.b[3146] { 1.0 } else { 0.0 };

        let (assign92650_e141972,) = {
    if (((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) && s.b[3146]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign92650_e141972;

        s.b[3147] = (2.0 == 2.0);
        s.v[3147] = if s.b[3147] { 1.0 } else { 0.0 };

        let (assign92670_e141997,) = {
    if ((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) && (!s.b[3146])) && s.b[3147]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign92670_e141997;

        s.b[3148] = (2.0 == 4.0);
        s.v[3148] = if s.b[3148] { 1.0 } else { 0.0 };

        let (assign92690_e142025,) = {
    if (((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) && (!s.b[3146])) && (!s.b[3147])) && s.b[3148]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign92690_e142025;

        s.b[3149] = (2.0 == 8.0);
        s.v[3149] = if s.b[3149] { 1.0 } else { 0.0 };

        let (assign92710_e142056,) = {
    if ((((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) && (!s.b[3146])) && (!s.b[3147])) && (!s.b[3148])) && s.b[3149]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign92710_e142056;

        let (assign92720_e142073,) = {
    if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign92720_e142073;

        let mut assign92730_loop_guard: usize = 0;
        while {
            let assign92730_cond_e142091: f64 = if (((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92730_cond_e142091 != 0.0
        } {
            assign92730_loop_guard += 1;
            assert!(assign92730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) {
                s.store_sqrt(726, 726);
            }
            let (assign92730_body1_e142128,) = {
    if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && s.b[3145]) {
        let assign92730_body1_e142126: f64 = (s.v[719] + 1.0);
        (assign92730_body1_e142126,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign92730_body1_e142128;
        }

        if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) && (!s.b[3145])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && s.b[3144]) {
        }

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && s.b[3143]) && (!s.b[3144])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3141]) && (!s.b[3143])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3150] = (p.p33 == 1.0);
        s.v[3150] = if s.b[3150] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3151] = (s.v[411] > 0.0);
        s.v[3151] = if s.b[3151] { 1.0 } else { 0.0 };

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && s.b[3151]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && (!s.b[3151])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3152] = (s.v[336] < 0.0);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && (!s.b[3151])) && s.b[3152]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && (!s.b[3151])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
        }

    }

    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && (!s.b[3151])) {
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3153] = (s.v[336] < 0.0);
        s.v[3153] = if s.b[3153] { 1.0 } else { 0.0 };

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && s.b[3153]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3113, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3154] = (s.v[333] < 60.0);
        s.v[3154] = if s.b[3154] { 1.0 } else { 0.0 };

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && s.b[3154]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && (!s.b[3154])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3155] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3155] = if s.b[3155] { 1.0 } else { 0.0 };

        let (assign93160_e142791,) = {
    if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && s.b[3155]) {
        let assign93160_e142789: f64 = (s.v[3119] + 1.0);
        (assign93160_e142789,)
    } else {
        (s.v[3119],)
    }
};
        s.v[3119] = assign93160_e142791;

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3150]) && s.b[3155]) {
            s.copy_ad(116, 447);
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3137])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3156] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3156] = if s.b[3156] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3156]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && (!s.b[3156])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.b[3109] && s.b[3110]) && (!s.b[3137])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3157, 354, 3113);
        }

        s.b[3159] = (p.p33 == 2.0);
        s.v[3159] = if s.b[3159] { 1.0 } else { 0.0 };

        s.b[3160] = ((s.v[3157] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3160] = if s.b[3160] { 1.0 } else { 0.0 };

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
            s.store_add_scaled_inputs3_indices(781, 3157, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign93340_e143028,) = {
    if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign93340_e143028;

        let (assign93350_e143041,) = {
    if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93350_e143041;

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3161] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3161] = if s.b[3161] { 1.0 } else { 0.0 };

        s.b[3162] = (2.0 == 1.0);
        s.v[3162] = if s.b[3162] { 1.0 } else { 0.0 };

        let (assign93460_e143190,) = {
    if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) && s.b[3162]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93460_e143190;

        s.b[3163] = (2.0 == 2.0);
        s.v[3163] = if s.b[3163] { 1.0 } else { 0.0 };

        let (assign93480_e143213,) = {
    if (((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) && (!s.b[3162])) && s.b[3163]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93480_e143213;

        s.b[3164] = (2.0 == 4.0);
        s.v[3164] = if s.b[3164] { 1.0 } else { 0.0 };

        let (assign93500_e143239,) = {
    if ((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) && (!s.b[3162])) && (!s.b[3163])) && s.b[3164]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93500_e143239;

        s.b[3165] = (2.0 == 8.0);
        s.v[3165] = if s.b[3165] { 1.0 } else { 0.0 };

        let (assign93520_e143268,) = {
    if (((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) && (!s.b[3162])) && (!s.b[3163])) && (!s.b[3164])) && s.b[3165]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93520_e143268;

        let (assign93530_e143283,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign93530_e143283;

        let mut assign93540_loop_guard: usize = 0;
        while {
            let assign93540_cond_e143299: f64 = if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93540_cond_e143299 != 0.0
        } {
            assign93540_loop_guard += 1;
            assert!(assign93540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) {
                s.store_sqrt(726, 726);
            }
            let (assign93540_body1_e143332,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && s.b[3161]) {
        let assign93540_body1_e143330: f64 = (s.v[719] + 1.0);
        (assign93540_body1_e143330,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign93540_body1_e143332;
        }

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) && (!s.b[3161])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3160]) {
        }

        if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && (!s.b[3160])) {
            s.copy_ad(335, 3157);
            s.store_scalar(334, 1.0);
        }

        s.b[3166] = (s.v[334] < 1.0);
        s.v[3166] = if s.b[3166] { 1.0 } else { 0.0 };

        let (assign93640_e143492,) = {
    if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3159]) && s.b[3166]) {
        let assign93640_e143490: f64 = (s.v[3119] + 2.0);
        (assign93640_e143490,)
    } else {
        (s.v[3119],)
    }
};
        s.v[3119] = assign93640_e143492;

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && (!s.b[3159])) {
            if (s.v[3157] <= s.v[386]) {
                s.copy_ad(335, 3157);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3167] = (s.v[3157] >= s.v[386]);
        s.v[3167] = if s.b[3167] { 1.0 } else { 0.0 };

        let (assign93670_e143528,) = {
    if ((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && (!s.b[3159])) && s.b[3167]) {
        let assign93670_e143526: f64 = (s.v[3119] + 2.0);
        (assign93670_e143526,)
    } else {
        (s.v[3119],)
    }
};
        s.v[3119] = assign93670_e143528;

        s.b[3168] = (s.v[3119] >= 2.0);
        s.v[3168] = if s.b[3168] { 1.0 } else { 0.0 };

        if (((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) {
            s.copy_ad(3158, 404);
            s.store_mul(354, 335, 3113);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[3169] = (p.p33 == 2.0);
        s.v[3169] = if s.b[3169] { 1.0 } else { 0.0 };

        s.b[3170] = ((s.v[404] > (s.v[3158] - 0.1)) && (0.1 >= 0.0));
        s.v[3170] = if s.b[3170] { 1.0 } else { 0.0 };

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) {
            s.store_offset_sub(781, 404, 3158, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign93790_e143680,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign93790_e143680;

        let (assign93800_e143695,) = {
    if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93800_e143695;

        if (((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3171] = if s.b[3171] { 1.0 } else { 0.0 };

        s.b[3172] = (2.0 == 1.0);
        s.v[3172] = if s.b[3172] { 1.0 } else { 0.0 };

        let (assign93910_e143862,) = {
    if (((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93910_e143862;

        s.b[3173] = (2.0 == 2.0);
        s.v[3173] = if s.b[3173] { 1.0 } else { 0.0 };

        let (assign93930_e143887,) = {
    if ((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) && s.b[3173]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93930_e143887;

        s.b[3174] = (2.0 == 4.0);
        s.v[3174] = if s.b[3174] { 1.0 } else { 0.0 };

        let (assign93950_e143915,) = {
    if (((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) && (!s.b[3173])) && s.b[3174]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93950_e143915;

        s.b[3175] = (2.0 == 8.0);
        s.v[3175] = if s.b[3175] { 1.0 } else { 0.0 };

        let (assign93970_e143946,) = {
    if ((((((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) && (!s.b[3173])) && (!s.b[3174])) && s.b[3175]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign93970_e143946;

        let (assign93980_e143963,) = {
    if ((((((s.b[3109] && s.b[3110]) && (!s.b[3137])) && s.b[3168]) && s.b[3169]) && s.b[3170]) && s.b[3171]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign93980_e143963;

    }
}
