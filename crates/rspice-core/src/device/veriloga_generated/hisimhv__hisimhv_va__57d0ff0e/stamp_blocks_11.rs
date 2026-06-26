#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign84720_loop_guard: usize = 0;
        while {
            let assign84720_cond_e129553: f64 = (s.v[421] + 1.0);
            let assign84720_cond_e129555: f64 = if (((s.v[2625] != 0.0) && s.b[2964]) && (s.v[97] <= assign84720_cond_e129553)) { 1.0 } else { 0.0 };
            assign84720_cond_e129555 != 0.0
        } {
            assign84720_loop_guard += 1;
            assert!(assign84720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2969] = (s.v[333] < 60.0);
            s.v[2969] = if s.b[2969] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2969]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2969])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2970] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2970] = if s.b[2970] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2970]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25))))));
                s.store_scaled_mul_ad(336, A::square(s.ad_value(415)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25))))));
                s.store_sub(2891, 334, 336);
                s.store_mul_ad_rhs(2892, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2970])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_ad(2891, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
                s.store_mul_ad_rhs(2892, 154, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(334)), 1.0, s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)), (-1.0)));
            }
            s.b[2971] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2971] = if s.b[2971] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2971]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2972] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && s.b[2972]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && (!s.b[2972])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(229), A::offset(s.ad_value(116), 1.0), (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2973] = (s.v[214] > 0.0);
            s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2973]) {
                s.store_sqrt_add(216, 2891, 214);
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(2892), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[2974] = (s.v[2891] > 0.0);
            s.v[2974] = if s.b[2974] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && s.b[2974]) {
                s.store_sqrt(216, 2891);
                s.store_scaled_div(217, 2892, 216, 0.5);
            }
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && (!s.b[2974])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_ad_value(232, A::add_scaled_product(A::sub(s.ad_value(404), s.ad_value(402)), 1.0, s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2975] = (s.v[79] > 0.0);
            s.v[2975] = if s.b[2975] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2975]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2976] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2976] = if s.b[2976] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2976]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_add(404, 404, 236);
            }
            s.b[2977] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2977] = if s.b[2977] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2977]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_ad_value(223, {
                if (s.v[2891] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(2891)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_mul(2881, 982, 223);
            s.store_mul(2882, 2883, 2881);
            s.store_offset_div(100, 2882, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2979] = (s.v[407] < 0.0);
        s.v[2979] = if s.b[2979] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[2979]) {
            s.store_neg(407, 407);
        }

        s.b[2980] = (p.p55 == 0.0);
        s.v[2980] = if s.b[2980] { 1.0 } else { 0.0 };

        s.b[2981] = (p.p50 == 0.0);
        s.v[2981] = if s.b[2981] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2981]) {
            s.store_neg(2884, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && (!s.b[2981])) {
            s.copy_ad(2884, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(2884), p.p137), A::offset(s.ad_value(2884), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(2884), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2884), p.p137), 782, 0.5);
        }

        s.b[2982] = (s.v[336] < 0.0);
        s.v[2982] = if s.b[2982] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2982]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_sub(407, 407, 603);
        }

        s.b[2983] = (3.0 == 1.0);
        s.v[2983] = if s.b[2983] { 1.0 } else { 0.0 };

        s.b[2984] = (3.0 == 2.0);
        s.v[2984] = if s.b[2984] { 1.0 } else { 0.0 };

        s.b[2985] = (3.0 == 3.0);
        s.v[2985] = if s.b[2985] { 1.0 } else { 0.0 };

        s.b[2986] = (3.0 == 4.0);
        s.v[2986] = if s.b[2986] { 1.0 } else { 0.0 };

        s.b[2987] = (p.p55 == 1.0);
        s.v[2987] = if s.b[2987] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[2983]) && s.b[2987]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2625] != 0.0) && s.b[2983]) && (!s.b[2987])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[2983]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2984] && (!s.b[2983]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2988] = (p.p55 == 1.0);
        s.v[2988] = if s.b[2988] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2988]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && (!s.b[2988])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {
            s.copy_ad(697, 404);
        }

        s.b[2989] = (p.p430 == 0.0);
        s.v[2989] = if s.b[2989] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2989]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[2986] && (!((s.b[2983] || s.b[2984]) || s.b[2985])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2625] = 0.0;

        s.b[2990] = (4.0 == 1.0);
        s.v[2990] = if s.b[2990] { 1.0 } else { 0.0 };

        s.b[2991] = (4.0 == 2.0);
        s.v[2991] = if s.b[2991] { 1.0 } else { 0.0 };

        s.b[2992] = (4.0 == 3.0);
        s.v[2992] = if s.b[2992] { 1.0 } else { 0.0 };

        s.b[2993] = (4.0 == 4.0);
        s.v[2993] = if s.b[2993] { 1.0 } else { 0.0 };

        s.b[2994] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        if (s.b[2990] && s.b[2994]) {
            s.store_scalar(2625, 1.0);
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2995] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2995] = if s.b[2995] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[2990])) && s.b[2995]) {
            s.store_scalar(2625, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2996] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2996] = if s.b[2996] { 1.0 } else { 0.0 };

        if ((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) {
            s.store_scalar(2625, 1.0);
            s.store_scalar(2626, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2997] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2997] = if s.b[2997] { 1.0 } else { 0.0 };

        if (((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) {
            s.store_neg(407, 407);
        }

    }

    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) {
            s.store_scalar(335, p.p63);
            s.store_offset_ad(996, A::div_scaled_product(s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0), (-p.p137));
        }

        s.b[2998] = (p.p113 > 0.0);
        s.v[2998] = if s.b[2998] { 1.0 } else { 0.0 };

        s.b[2999] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2999] = if s.b[2999] { 1.0 } else { 0.0 };

        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && s.b[2999]) {
        }

        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_ad_value(396, A::div_scaled_product(s.ad_value(783), s.ad_value(784), 1.0, s.ad_value(782), 1.0));
        }

        if ((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(396), p.p137), A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(396), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[3000] = (s.v[336] < 0.0);
        s.v[3000] = if s.b[3000] { 1.0 } else { 0.0 };

        if (((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) && s.b[3000]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2992] && (!(s.b[2990] || s.b[2991]))) && s.b[2996]) && s.b[2997]) && s.b[2998]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[3001] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[3001] = if s.b[3001] { 1.0 } else { 0.0 };

        if ((s.b[2993] && (!((s.b[2990] || s.b[2991]) || s.b[2992]))) && s.b[3001]) {
            s.store_scalar(2625, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2625] != 0.0) {
            s.store_scalar(3009, 0.4);
            s.store_scalar(3010, 0.0);
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
            s.store_scalar(3011, 0.0);
            s.store_scalar(3012, 0.0);
            s.store_mul_scaled_ad_rhs(3007, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3007), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2625] != 0.0) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[2625] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(3008, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[3014] = (s.v[3009] > (s.v[3008] * 0.5));
        s.v[3014] = if s.b[3014] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3014]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3015] = param_given[338];
        s.v[3015] = if s.b[3015] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3015]) {
            s.store_scalar(3008, p.p338);
        }

        s.b[3016] = param_given[339];
        s.v[3016] = if s.b[3016] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3016]) {
            s.store_scalar(3009, p.p339);
        }

        s.b[3017] = param_given[338];
        s.v[3017] = if s.b[3017] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3016])) && s.b[3017]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3018] = (s.v[3009] > (s.v[3008] * 0.5));
        s.v[3018] = if s.b[3018] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3018]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3019] = (p.p38 == 1.0);
        s.v[3019] = if s.b[3019] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3019]) {
            s.store_neg(334, 396);
        }

        s.b[3020] = (s.v[334] > s.v[3009]);
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[3019]) && s.b[3020]) {
            s.store_sub(335, 334, 3009);
            s.store_sub(336, 3008, 3009);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_add_ad(780, 1.0, A::add_scaled_inputs3(A::offset(s.ad_value(781), 1.0), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0), s.ad_value(784));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3(A::scale_offset(s.ad_value(781), 2.0, 1.0), 1.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_ad_value(334, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(780)), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
            s.store_neg(345, 345);
            s.store_add(344, 3009, 333);
        }

        if (((s.v[2625] != 0.0) && s.b[3019]) && (!s.b[3020])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2625] != 0.0) && s.b[3019]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3019])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2625] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(3003, 0.0);
            s.store_scale(3004, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3021] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3021]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3021])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign86490_loop_guard: usize = 0;
        while {
            let assign86490_cond_e132100: f64 = if (((s.v[2625] != 0.0) && (!s.b[3021])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign86490_cond_e132100 != 0.0
        } {
            assign86490_loop_guard += 1;
            assert!(assign86490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && (!s.b[3021])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[3021])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[3022] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(402), 1.0, s.ad_value(397), 1.0, s.ad_value(335), 1.0));
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

        s.b[3023] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        s.b[3024] = (1.0 == 1.0);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && s.b[3024]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3025] = (1.0 == 2.0);
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && s.b[3025]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3026] = (1.0 == 4.0);
        s.v[3026] = if s.b[3026] { 1.0 } else { 0.0 };

        if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && s.b[3026]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3027] = (1.0 == 8.0);
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3026])) && s.b[3027]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign86820_loop_guard: usize = 0;
        while {
            let assign86820_cond_e132450: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86820_cond_e132450 != 0.0
        } {
            assign86820_loop_guard += 1;
            assert!(assign86820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && (!s.b[3023])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3022])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[3028] = (s.v[402] < s.v[403]);
        s.v[3028] = if s.b[3028] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3028]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[3029] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3029] = if s.b[3029] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[3028]) && s.b[3029]) {
            s.store_scaled_div(274, 277, 278, 0.5);
        }

        if (((s.v[2625] != 0.0) && s.b[3028]) && (!s.b[3029])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2625] != 0.0) && s.b[3028]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_ad_value(272, A::add_scaled_product(A::add_scaled_inputs(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, s.ad_value(273), 2.0), 1.0, s.ad_value(273), s.ad_value(273), 1.414213562373095));
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3011, 404);
        }

        s.b[3030] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3030] = if s.b[3030] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3030]) {
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(402), 1.0, s.ad_value(213), s.ad_value(154), 0.5));
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3030])) {
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3031] = (s.v[116] >= 3.0);
        s.v[3031] = if s.b[3031] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3031]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3031])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_ad_value(437, A::div_scaled_inputs(A::add(s.ad_value(402), s.ad_value(397)), -1.0, s.ad_value(212), 1.0));
            s.store_ad_value(441, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product(s.ad_value(435), s.ad_value(436), 1.0, A::mul_scaled_lhs(s.ad_value(434), 6.0, s.ad_value(434)), 1.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0));
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0));
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3032] = (p.p33 > 0.0);
        s.v[3032] = if s.b[3032] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_ad_value(447, A::add_scaled_product(A::sub(A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334)))), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        s.b[3033] = (p.p33 == 2.0);
        s.v[3033] = if s.b[3033] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_ad_value(447, A::add_scaled_inputs3(s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3033])) {
            s.store_ad_value(447, {
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_ad_value(447, {
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_ad_value(446, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_ad_value(446, {
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.copy_ad(445, 116);
        }

        s.b[3034] = (p.p33 == 2.0);
        s.v[3034] = if s.b[3034] { 1.0 } else { 0.0 };

        s.b[3035] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3035] = if s.b[3035] { 1.0 } else { 0.0 };

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(445), 1.0, s.ad_value(446), (-1.0), s.ad_value(446), 0.2));
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

        s.b[3036] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3036] = if s.b[3036] { 1.0 } else { 0.0 };

        s.b[3037] = (2.0 == 1.0);
        s.v[3037] = if s.b[3037] { 1.0 } else { 0.0 };

        if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && s.b[3037]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3038] = (2.0 == 2.0);
        s.v[3038] = if s.b[3038] { 1.0 } else { 0.0 };

        if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && s.b[3038]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3039] = (2.0 == 4.0);
        s.v[3039] = if s.b[3039] { 1.0 } else { 0.0 };

        if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && s.b[3039]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3040] = (2.0 == 8.0);
        s.v[3040] = if s.b[3040] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && (!s.b[3039])) && s.b[3040]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign87950_loop_guard: usize = 0;
        while {
            let assign87950_cond_e134007: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign87950_cond_e134007 != 0.0
        } {
            assign87950_loop_guard += 1;
            assert!(assign87950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && (!s.b[3036])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_ad_value(335, A::div_scaled_product3(s.ad_value(446), s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0));
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && (!s.b[3035])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3034])) {
            s.store_ad_value(116, {
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.b[3041] = (p.p33 == 1.0);
        s.v[3041] = if s.b[3041] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3042] = (s.v[411] > 0.0);
        s.v[3042] = if s.b[3042] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3042]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3043] = (s.v[336] < 0.0);
        s.v[3043] = if s.b[3043] { 1.0 } else { 0.0 };

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) && s.b[3043]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3044] = (s.v[336] < 0.0);
        s.v[3044] = if s.b[3044] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3044]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3045] = (s.v[333] < 60.0);
        s.v[3045] = if s.b[3045] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3045]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3045])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3046] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3046] = if s.b[3046] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3046]) {
            s.store_offset(3010, 3010, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3047] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3047] = if s.b[3047] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3047]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3047])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3048, 354, 3004);
        }

        s.b[3050] = (p.p33 == 2.0);
        s.v[3050] = if s.b[3050] { 1.0 } else { 0.0 };

        s.b[3051] = ((s.v[3048] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3051] = if s.b[3051] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(3048), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1));
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

        s.b[3052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3052] = if s.b[3052] { 1.0 } else { 0.0 };

        s.b[3053] = (2.0 == 1.0);
        s.v[3053] = if s.b[3053] { 1.0 } else { 0.0 };

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && s.b[3053]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3054] = (2.0 == 2.0);
        s.v[3054] = if s.b[3054] { 1.0 } else { 0.0 };

        if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && s.b[3054]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3055] = (2.0 == 4.0);
        s.v[3055] = if s.b[3055] { 1.0 } else { 0.0 };

        if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && s.b[3055]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3056] = (2.0 == 8.0);
        s.v[3056] = if s.b[3056] { 1.0 } else { 0.0 };

        if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && (!s.b[3055])) && s.b[3056]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign88760_loop_guard: usize = 0;
        while {
            let assign88760_cond_e135077: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign88760_cond_e135077 != 0.0
        } {
            assign88760_loop_guard += 1;
            assert!(assign88760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && (!s.b[3052])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(386), s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && (!s.b[3051])) {
            s.copy_ad(335, 3048);
            s.store_scalar(334, 1.0);
        }

        s.b[3057] = (s.v[334] < 1.0);
        s.v[3057] = if s.b[3057] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3057]) {
            s.store_offset(3010, 3010, 2.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3050])) {
            s.store_ad_value(335, {
                if (s.v[3048] <= s.v[386]) {
                    s.ad_value(3048)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.b[3058] = (s.v[3048] >= s.v[386]);
        s.v[3058] = if s.b[3058] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3050])) && s.b[3058]) {
            s.store_offset(3010, 3010, 2.0);
        }

        s.b[3059] = (s.v[3010] >= 2.0);
        s.v[3059] = if s.b[3059] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) {
            s.copy_ad(3049, 404);
            s.store_mul(354, 335, 3004);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.b[3060] = (p.p33 == 2.0);
        s.v[3060] = if s.b[3060] { 1.0 } else { 0.0 };

        s.b[3061] = ((s.v[404] > (s.v[3049] - 0.1)) && (0.1 >= 0.0));
        s.v[3061] = if s.b[3061] { 1.0 } else { 0.0 };

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
            s.store_offset_sub(781, 404, 3049, 0.1);
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

        s.b[3062] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3062] = if s.b[3062] { 1.0 } else { 0.0 };

        s.b[3063] = (2.0 == 1.0);
        s.v[3063] = if s.b[3063] { 1.0 } else { 0.0 };

        if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && s.b[3063]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3064] = (2.0 == 2.0);
        s.v[3064] = if s.b[3064] { 1.0 } else { 0.0 };

        if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && s.b[3064]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3065] = (2.0 == 4.0);
        s.v[3065] = if s.b[3065] { 1.0 } else { 0.0 };

        if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && (!s.b[3064])) && s.b[3065]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3066] = (2.0 == 8.0);
        s.v[3066] = if s.b[3066] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && (!s.b[3064])) && (!s.b[3065])) && s.b[3066]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign89210_loop_guard: usize = 0;
        while {
            let assign89210_cond_e135685: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign89210_cond_e135685 != 0.0
        } {
            assign89210_loop_guard += 1;
            assert!(assign89210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && (!s.b[3062])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_add_ad_lhs(404, A::offset(s.ad_value(3049), (-0.1)), 780);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && (!s.b[3060])) {
            s.store_ad_value(404, {
                if (s.v[404] <= s.v[3049]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(3049)
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.copy_ad(3011, 404);
        }

        s.b[3067] = (p.p33 == 1.0);
        s.v[3067] = if s.b[3067] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3004)), s.ad_value(155)), 2.0);
        }

        s.b[3068] = (s.v[411] > 0.0);
        s.v[3068] = if s.b[3068] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3068]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3069] = (s.v[336] < 0.0);
        s.v[3069] = if s.b[3069] { 1.0 } else { 0.0 };

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) && s.b[3069]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3070] = (s.v[336] < 0.0);
        s.v[3070] = if s.b[3070] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3070]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign89570_loop_guard: usize = 0;
        while {
            let assign89570_cond_e136209: f64 = (s.v[421] + 1.0);
            let assign89570_cond_e136211: f64 = if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (s.v[97] <= assign89570_cond_e136209)) { 1.0 } else { 0.0 };
            assign89570_cond_e136211 != 0.0
        } {
            assign89570_loop_guard += 1;
            assert!(assign89570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3072] = (s.v[333] < 60.0);
            s.v[3072] = if s.b[3072] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3072]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3072])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3073] = (s.v[116] < 0.0);
            s.v[3073] = if s.b[3073] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3073]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3074] = (s.v[116] < 1e-6);
            s.v[3074] = if s.b[3074] { 1.0 } else { 0.0 };
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25))))));
                s.store_scaled_mul_ad(336, A::square(s.ad_value(415)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25))))));
                s.store_sub(338, 334, 336);
            }
            s.b[3075] = (s.v[338] > 0.0);
            s.v[3075] = if s.b[3075] { 1.0 } else { 0.0 };
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) && s.b[3075]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) && (!s.b[3075])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_ad(338, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            s.b[3076] = (s.v[338] > 0.0);
            s.v[3076] = if s.b[3076] { 1.0 } else { 0.0 };
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) && s.b[3076]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(334)), 1.0, s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) && (!s.b[3076])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3077] = (s.v[116] < 0.0);
            s.v[3077] = if s.b[3077] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3077]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3078] = (s.v[116] < 60.0);
            s.v[3078] = if s.b[3078] { 1.0 } else { 0.0 };
            s.b[3079] = (s.v[116] < 5e-5);
            s.v[3079] = if s.b[3079] { 1.0 } else { 0.0 };
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3078]) && s.b[3079]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3078]) && (!s.b[3079])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && (!s.b[3078])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(229), A::offset(s.ad_value(116), 1.0), (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3080] = (s.v[214] > 0.0);
            s.v[3080] = if s.b[3080] { 1.0 } else { 0.0 };
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3080]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_ad_lhs(217, A::add_scaled_product(s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5)), 216);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && (!s.b[3080])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_ad_value(232, A::add_scaled_product(A::sub(s.ad_value(404), s.ad_value(402)), 1.0, s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3081] = (s.v[79] == 1.0);
            s.v[3081] = if s.b[3081] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3081]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3082] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3082] = if s.b[3082] { 1.0 } else { 0.0 };
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) && s.b[3082]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_add(404, 404, 236);
            }
            s.b[3083] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3083] = if s.b[3083] { 1.0 } else { 0.0 };
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) && s.b[3083]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_mul(3002, 982, 223);
            s.store_mul(3003, 3004, 3002);
            s.store_offset_div(100, 3003, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3085] = (p.p33 == 4.0);
        s.v[3085] = if s.b[3085] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3011);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3004)), s.ad_value(155)), 2.0);
        }

        s.b[3086] = (s.v[411] > 0.0);
        s.v[3086] = if s.b[3086] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3086]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3087] = (s.v[336] < 0.0);
        s.v[3087] = if s.b[3087] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) && s.b[3087]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3088] = (s.v[336] < 0.0);
        s.v[3088] = if s.b[3088] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3088]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_83(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign89960_loop_guard: usize = 0;
        while {
            let assign89960_cond_e137864: f64 = (s.v[421] + 1.0);
            let assign89960_cond_e137866: f64 = if (((s.v[2625] != 0.0) && s.b[3085]) && (s.v[97] <= assign89960_cond_e137864)) { 1.0 } else { 0.0 };
            assign89960_cond_e137866 != 0.0
        } {
            assign89960_loop_guard += 1;
            assert!(assign89960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3090] = (s.v[333] < 60.0);
            s.v[3090] = if s.b[3090] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3090]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3090])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3091] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3091]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25))))));
                s.store_scaled_mul_ad(336, A::square(s.ad_value(415)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25))))));
                s.store_sub(3012, 334, 336);
                s.store_mul_ad_rhs(3013, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3091])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_ad(3012, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
                s.store_mul_ad_rhs(3013, 154, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(334)), 1.0, s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)), (-1.0)));
            }
            s.b[3092] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[3092] = if s.b[3092] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3092]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3093] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[3093] = if s.b[3093] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3092])) && s.b[3093]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3092])) && (!s.b[3093])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(229), A::offset(s.ad_value(116), 1.0), (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3094] = (s.v[214] > 0.0);
            s.v[3094] = if s.b[3094] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3094]) {
                s.store_sqrt_add(216, 3012, 214);
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(3013), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[3095] = (s.v[3012] > 0.0);
            s.v[3095] = if s.b[3095] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3094])) && s.b[3095]) {
                s.store_sqrt(216, 3012);
                s.store_scaled_div(217, 3013, 216, 0.5);
            }
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3094])) && (!s.b[3095])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_ad_value(232, A::add_scaled_product(A::sub(s.ad_value(404), s.ad_value(402)), 1.0, s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3096] = (s.v[79] > 0.0);
            s.v[3096] = if s.b[3096] { 1.0 } else { 0.0 };
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3096]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3097] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3097] = if s.b[3097] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) && s.b[3097]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_add(404, 404, 236);
            }
            s.b[3098] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3098] = if s.b[3098] { 1.0 } else { 0.0 };
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) && s.b[3098]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_ad_value(223, {
                if (s.v[3012] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(3012)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_mul(3002, 982, 223);
            s.store_mul(3003, 3004, 3002);
            s.store_offset_div(100, 3003, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[3100] = (s.v[407] < 0.0);
        s.v[3100] = if s.b[3100] { 1.0 } else { 0.0 };

        if ((s.v[2625] != 0.0) && s.b[3100]) {
            s.store_neg(407, 407);
        }

        s.b[3101] = (p.p55 == 0.0);
        s.v[3101] = if s.b[3101] { 1.0 } else { 0.0 };

        s.b[3102] = (p.p50 == 0.0);
        s.v[3102] = if s.b[3102] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && s.b[3102]) {
            s.store_neg(3005, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && (!s.b[3102])) {
            s.copy_ad(3005, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(3005), p.p137), A::offset(s.ad_value(3005), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(3005), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3005), p.p137), 782, 0.5);
        }

        s.b[3103] = (s.v[336] < 0.0);
        s.v[3103] = if s.b[3103] { 1.0 } else { 0.0 };

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && s.b[3103]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_sub(407, 407, 603);
        }

        s.b[3104] = (4.0 == 1.0);
        s.v[3104] = if s.b[3104] { 1.0 } else { 0.0 };

        s.b[3105] = (4.0 == 2.0);
        s.v[3105] = if s.b[3105] { 1.0 } else { 0.0 };

        s.b[3106] = (4.0 == 3.0);
        s.v[3106] = if s.b[3106] { 1.0 } else { 0.0 };

        s.b[3107] = (4.0 == 4.0);
        s.v[3107] = if s.b[3107] { 1.0 } else { 0.0 };

        s.b[3108] = (p.p55 == 1.0);
        s.v[3108] = if s.b[3108] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && s.b[3104]) && s.b[3108]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2625] != 0.0) && s.b[3104]) && (!s.b[3108])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[3104]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[3105] && (!s.b[3104]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[3109] = (p.p55 == 1.0);
        s.v[3109] = if s.b[3109] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && s.b[3109]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && (!s.b[3109])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) {
            s.copy_ad(697, 404);
        }

        s.b[3110] = (p.p430 == 0.0);
        s.v[3110] = if s.b[3110] { 1.0 } else { 0.0 };

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && s.b[3110]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[3107] && (!((s.b[3104] || s.b[3105]) || s.b[3106])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.b[3111] = (p.p430 > 0.0);
        s.v[3111] = if s.b[3111] { 1.0 } else { 0.0 };

        if s.b[3111] {
            s.store_scalar(406, 1.0);
        }

        s.b[3112] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[3112] = if s.b[3112] { 1.0 } else { 0.0 };

        if (s.b[3111] && s.b[3112]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, 0.0);
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_scalar(3120, 0.4);
            s.store_scalar(3121, 0.0);
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
            s.store_scalar(3122, 0.0);
            s.store_scalar(3123, 0.0);
            s.store_mul_scaled_ad_rhs(3118, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3118), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

    }

    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[3111] && s.b[3112]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[3111] && s.b[3112]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(3119, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[3125] = (s.v[3120] > (s.v[3119] * 0.5));
        s.v[3125] = if s.b[3125] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3125]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3126] = param_given[338];
        s.v[3126] = if s.b[3126] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3126]) {
            s.store_scalar(3119, p.p338);
        }

        s.b[3127] = param_given[339];
        s.v[3127] = if s.b[3127] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3127]) {
            s.store_scalar(3120, p.p339);
        }

        s.b[3128] = param_given[338];
        s.v[3128] = if s.b[3128] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3127])) && s.b[3128]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3129] = (s.v[3120] > (s.v[3119] * 0.5));
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3129]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3130] = (p.p38 == 1.0);
        s.v[3130] = if s.b[3130] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3130]) {
            s.store_neg(334, 396);
        }

        s.b[3131] = (s.v[334] > s.v[3120]);
        s.v[3131] = if s.b[3131] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && s.b[3130]) && s.b[3131]) {
            s.store_sub(335, 334, 3120);
            s.store_sub(336, 3119, 3120);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_add_ad(780, 1.0, A::add_scaled_inputs3(A::offset(s.ad_value(781), 1.0), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0), s.ad_value(784));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3(A::scale_offset(s.ad_value(781), 2.0, 1.0), 1.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_ad_value(334, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(780)), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
            s.store_neg(345, 345);
            s.store_add(344, 3120, 333);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3130]) && (!s.b[3131])) {
            s.copy_ad(344, 334);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3130]) {
            s.store_neg(397, 344);
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3130])) {
            s.copy_ad(397, 396);
        }

        if (s.b[3111] && s.b[3112]) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(3114, 0.0);
            s.store_scale(3115, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3132] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3132] = if s.b[3132] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3132]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign91290_loop_guard: usize = 0;
        while {
            let assign91290_cond_e139929: f64 = if (((s.b[3111] && s.b[3112]) && (!s.b[3132])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91290_cond_e139929 != 0.0
        } {
            assign91290_loop_guard += 1;
            assert!(assign91290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[3133] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3133] = if s.b[3133] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(402), 1.0, s.ad_value(397), 1.0, s.ad_value(335), 1.0));
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

        s.b[3134] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3134] = if s.b[3134] { 1.0 } else { 0.0 };

        s.b[3135] = (1.0 == 1.0);
        s.v[3135] = if s.b[3135] { 1.0 } else { 0.0 };

        if (((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && s.b[3135]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3136] = (1.0 == 2.0);
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        if ((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && s.b[3136]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3137] = (1.0 == 4.0);
        s.v[3137] = if s.b[3137] { 1.0 } else { 0.0 };

        if (((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && (!s.b[3136])) && s.b[3137]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3138] = (1.0 == 8.0);
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

        if ((((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && (!s.b[3136])) && (!s.b[3137])) && s.b[3138]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign91620_loop_guard: usize = 0;
        while {
            let assign91620_cond_e140337: f64 = if (((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91620_cond_e140337 != 0.0
        } {
            assign91620_loop_guard += 1;
            assert!(assign91620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && (!s.b[3134])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && (!s.b[3133])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[3139] = (s.v[402] < s.v[403]);
        s.v[3139] = if s.b[3139] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3139]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[3140] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3140] = if s.b[3140] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && s.b[3139]) && s.b[3140]) {
            s.store_scaled_div(274, 277, 278, 0.5);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3139]) && (!s.b[3140])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3139]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_ad_value(272, A::add_scaled_product(A::add_scaled_inputs(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, s.ad_value(273), 2.0), 1.0, s.ad_value(273), s.ad_value(273), 1.414213562373095));
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3122, 404);
        }

        s.b[3141] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[3141] = if s.b[3141] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3141]) {
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(402), 1.0, s.ad_value(213), s.ad_value(154), 0.5));
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3141])) {
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3142] = (s.v[116] >= 3.0);
        s.v[3142] = if s.b[3142] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3142]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

    }

    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3142]) {
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3142])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_ad_value(437, A::div_scaled_inputs(A::add(s.ad_value(402), s.ad_value(397)), -1.0, s.ad_value(212), 1.0));
            s.store_ad_value(441, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product(s.ad_value(435), s.ad_value(436), 1.0, A::mul_scaled_lhs(s.ad_value(434), 6.0, s.ad_value(434)), 1.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0));
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0));
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3143] = (p.p33 > 0.0);
        s.v[3143] = if s.b[3143] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_ad_value(447, A::add_scaled_product(A::sub(A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334)))), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        s.b[3144] = (p.p33 == 2.0);
        s.v[3144] = if s.b[3144] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_ad_value(447, A::add_scaled_inputs3(s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && (!s.b[3144])) {
            s.store_ad_value(447, {
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_ad_value(447, {
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_ad_value(446, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_ad_value(446, {
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.copy_ad(445, 116);
        }

        s.b[3145] = (p.p33 == 2.0);
        s.v[3145] = if s.b[3145] { 1.0 } else { 0.0 };

        s.b[3146] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[3146] = if s.b[3146] { 1.0 } else { 0.0 };

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(445), 1.0, s.ad_value(446), (-1.0), s.ad_value(446), 0.2));
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

        s.b[3147] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3147] = if s.b[3147] { 1.0 } else { 0.0 };

        s.b[3148] = (2.0 == 1.0);
        s.v[3148] = if s.b[3148] { 1.0 } else { 0.0 };

        if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && s.b[3148]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3149] = (2.0 == 2.0);
        s.v[3149] = if s.b[3149] { 1.0 } else { 0.0 };

        if ((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && s.b[3149]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3150] = (2.0 == 4.0);
        s.v[3150] = if s.b[3150] { 1.0 } else { 0.0 };

        if (((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && (!s.b[3149])) && s.b[3150]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3151] = (2.0 == 8.0);
        s.v[3151] = if s.b[3151] { 1.0 } else { 0.0 };

        if ((((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && (!s.b[3149])) && (!s.b[3150])) && s.b[3151]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign92750_loop_guard: usize = 0;
        while {
            let assign92750_cond_e142098: f64 = if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92750_cond_e142098 != 0.0
        } {
            assign92750_loop_guard += 1;
            assert!(assign92750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && (!s.b[3147])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_ad_value(335, A::div_scaled_product3(s.ad_value(446), s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0));
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && (!s.b[3146])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && (!s.b[3145])) {
            s.store_ad_value(116, {
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.b[3152] = (p.p33 == 1.0);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3153] = (s.v[411] > 0.0);
        s.v[3153] = if s.b[3153] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && s.b[3153]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3154] = (s.v[336] < 0.0);
        s.v[3154] = if s.b[3154] { 1.0 } else { 0.0 };

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) && s.b[3154]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3155] = (s.v[336] < 0.0);
        s.v[3155] = if s.b[3155] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && s.b[3155]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3115, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3156] = (s.v[333] < 60.0);
        s.v[3156] = if s.b[3156] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && s.b[3156]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3156])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3157] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[3157] = if s.b[3157] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && s.b[3157]) {
            s.store_offset(3121, 3121, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[3158] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[3158] = if s.b[3158] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3158]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3158])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3159, 354, 3115);
        }

        s.b[3161] = (p.p33 == 2.0);
        s.v[3161] = if s.b[3161] { 1.0 } else { 0.0 };

        s.b[3162] = ((s.v[3159] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[3162] = if s.b[3162] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(3159), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1));
        }

    }

    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) {
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

        s.b[3163] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3163] = if s.b[3163] { 1.0 } else { 0.0 };

        s.b[3164] = (2.0 == 1.0);
        s.v[3164] = if s.b[3164] { 1.0 } else { 0.0 };

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) && s.b[3164]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3165] = (2.0 == 2.0);
        s.v[3165] = if s.b[3165] { 1.0 } else { 0.0 };

        if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) && (!s.b[3164])) && s.b[3165]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3166] = (2.0 == 4.0);
        s.v[3166] = if s.b[3166] { 1.0 } else { 0.0 };

        if ((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) && (!s.b[3164])) && (!s.b[3165])) && s.b[3166]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3167] = (2.0 == 8.0);
        s.v[3167] = if s.b[3167] { 1.0 } else { 0.0 };

        if (((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) && (!s.b[3164])) && (!s.b[3165])) && (!s.b[3166])) && s.b[3167]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign93560_loop_guard: usize = 0;
        while {
            let assign93560_cond_e143306: f64 = if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93560_cond_e143306 != 0.0
        } {
            assign93560_loop_guard += 1;
            assert!(assign93560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && s.b[3163]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) && (!s.b[3163])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(386), s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0));
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3162]) {
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && (!s.b[3162])) {
            s.copy_ad(335, 3159);
            s.store_scalar(334, 1.0);
        }

        s.b[3168] = (s.v[334] < 1.0);
        s.v[3168] = if s.b[3168] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3161]) && s.b[3168]) {
            s.store_offset(3121, 3121, 2.0);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3161])) {
            s.store_ad_value(335, {
                if (s.v[3159] <= s.v[386]) {
                    s.ad_value(3159)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.b[3169] = (s.v[3159] >= s.v[386]);
        s.v[3169] = if s.b[3169] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3161])) && s.b[3169]) {
            s.store_offset(3121, 3121, 2.0);
        }

        s.b[3170] = (s.v[3121] >= 2.0);
        s.v[3170] = if s.b[3170] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) {
            s.copy_ad(3160, 404);
            s.store_mul(354, 335, 3115);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.b[3171] = (p.p33 == 2.0);
        s.v[3171] = if s.b[3171] { 1.0 } else { 0.0 };

        s.b[3172] = ((s.v[404] > (s.v[3160] - 0.1)) && (0.1 >= 0.0));
        s.v[3172] = if s.b[3172] { 1.0 } else { 0.0 };

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
            s.store_offset_sub(781, 404, 3160, 0.1);
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

        s.b[3173] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3173] = if s.b[3173] { 1.0 } else { 0.0 };

        s.b[3174] = (2.0 == 1.0);
        s.v[3174] = if s.b[3174] { 1.0 } else { 0.0 };

        if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && s.b[3174]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3175] = (2.0 == 2.0);
        s.v[3175] = if s.b[3175] { 1.0 } else { 0.0 };

        if ((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && s.b[3175]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3176] = (2.0 == 4.0);
        s.v[3176] = if s.b[3176] { 1.0 } else { 0.0 };

        if (((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && (!s.b[3175])) && s.b[3176]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3177] = (2.0 == 8.0);
        s.v[3177] = if s.b[3177] { 1.0 } else { 0.0 };

        if ((((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (!s.b[3174])) && (!s.b[3175])) && (!s.b[3176])) && s.b[3177]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign94010_loop_guard: usize = 0;
        while {
            let assign94010_cond_e143988: f64 = if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign94010_cond_e143988 != 0.0
        } {
            assign94010_loop_guard += 1;
            assert!(assign94010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && s.b[3173]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) && (!s.b[3173])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_add_ad_lhs(404, A::offset(s.ad_value(3160), (-0.1)), 780);
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && s.b[3172]) {
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) {
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && s.b[3171]) && (!s.b[3172])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3170]) && (!s.b[3171])) {
            s.store_ad_value(404, {
                if (s.v[404] <= s.v[3160]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(3160)
                }
            });
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {
            s.copy_ad(3122, 404);
        }

        s.b[3178] = (p.p33 == 1.0);
        s.v[3178] = if s.b[3178] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3115)), s.ad_value(155)), 2.0);
        }

        s.b[3179] = (s.v[411] > 0.0);
        s.v[3179] = if s.b[3179] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3179]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3180] = (s.v[336] < 0.0);
        s.v[3180] = if s.b[3180] { 1.0 } else { 0.0 };

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) && s.b[3180]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3179])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3181] = (s.v[336] < 0.0);
        s.v[3181] = if s.b[3181] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3181]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3115, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign94370_loop_guard: usize = 0;
        while {
            let assign94370_cond_e144580: f64 = (s.v[421] + 1.0);
            let assign94370_cond_e144582: f64 = if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (s.v[97] <= assign94370_cond_e144580)) { 1.0 } else { 0.0 };
            assign94370_cond_e144582 != 0.0
        } {
            assign94370_loop_guard += 1;
            assert!(assign94370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3183] = (s.v[333] < 60.0);
            s.v[3183] = if s.b[3183] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3183]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3183])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3184] = (s.v[116] < 0.0);
            s.v[3184] = if s.b[3184] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3184]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3185] = (s.v[116] < 1e-6);
            s.v[3185] = if s.b[3185] { 1.0 } else { 0.0 };
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25))))));
                s.store_scaled_mul_ad(336, A::square(s.ad_value(415)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25))))));
                s.store_sub(338, 334, 336);
            }
            s.b[3186] = (s.v[338] > 0.0);
            s.v[3186] = if s.b[3186] { 1.0 } else { 0.0 };
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) && s.b[3186]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && s.b[3185]) && (!s.b[3186])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_ad(338, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            s.b[3187] = (s.v[338] > 0.0);
            s.v[3187] = if s.b[3187] { 1.0 } else { 0.0 };
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) && s.b[3187]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(334)), 1.0, s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3184])) && (!s.b[3185])) && (!s.b[3187])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3188] = (s.v[116] < 0.0);
            s.v[3188] = if s.b[3188] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3188]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3189] = (s.v[116] < 60.0);
            s.v[3189] = if s.b[3189] { 1.0 } else { 0.0 };
            s.b[3190] = (s.v[116] < 5e-5);
            s.v[3190] = if s.b[3190] { 1.0 } else { 0.0 };
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3189]) && s.b[3190]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3189]) && (!s.b[3190])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && (!s.b[3189])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(229), A::offset(s.ad_value(116), 1.0), (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3191] = (s.v[214] > 0.0);
            s.v[3191] = if s.b[3191] { 1.0 } else { 0.0 };
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && s.b[3191]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_ad_lhs(217, A::add_scaled_product(s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5)), 216);
            }
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3188])) && (!s.b[3191])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
                s.store_ad_value(232, A::add_scaled_product(A::sub(s.ad_value(404), s.ad_value(402)), 1.0, s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3192] = (s.v[79] == 1.0);
            s.v[3192] = if s.b[3192] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && s.b[3192]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3193] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3193] = if s.b[3193] { 1.0 } else { 0.0 };
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) && s.b[3193]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) {
                s.store_add(404, 404, 236);
            }
            s.b[3194] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3194] = if s.b[3194] { 1.0 } else { 0.0 };
            if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) && (!s.b[3192])) && s.b[3194]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3178]) {
            s.store_mul(3113, 982, 223);
            s.store_mul(3114, 3115, 3113);
            s.store_offset_div(100, 3114, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3196] = (p.p33 == 4.0);
        s.v[3196] = if s.b[3196] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3122);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3115)), s.ad_value(155)), 2.0);
        }

        s.b[3197] = (s.v[411] > 0.0);
        s.v[3197] = if s.b[3197] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3197]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3198] = (s.v[336] < 0.0);
        s.v[3198] = if s.b[3198] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) && s.b[3198]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3197])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3199] = (s.v[336] < 0.0);
        s.v[3199] = if s.b[3199] { 1.0 } else { 0.0 };

        if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3199]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3115, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign94760_loop_guard: usize = 0;
        while {
            let assign94760_cond_e146439: f64 = (s.v[421] + 1.0);
            let assign94760_cond_e146441: f64 = if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (s.v[97] <= assign94760_cond_e146439)) { 1.0 } else { 0.0 };
            assign94760_cond_e146441 != 0.0
        } {
            assign94760_loop_guard += 1;
            assert!(assign94760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3201] = (s.v[333] < 60.0);
            s.v[3201] = if s.b[3201] { 1.0 } else { 0.0 };
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3201]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3201])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3202] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[3202] = if s.b[3202] { 1.0 } else { 0.0 };
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3202]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25))))));
                s.store_scaled_mul_ad(336, A::square(s.ad_value(415)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (4.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (2.0), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(415), 1.0 / (3.0), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25))))));
                s.store_sub(3123, 334, 336);
                s.store_mul_ad_rhs(3124, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3202])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_ad(3123, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
                s.store_mul_ad_rhs(3124, 154, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(334)), 1.0, s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)), (-1.0)));
            }
            s.b[3203] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[3203] = if s.b[3203] { 1.0 } else { 0.0 };
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3203]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3204] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[3204] = if s.b[3204] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3203])) && s.b[3204]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3203])) && (!s.b[3204])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(229), A::offset(s.ad_value(116), 1.0), (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3205] = (s.v[214] > 0.0);
            s.v[3205] = if s.b[3205] { 1.0 } else { 0.0 };
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3205]) {
                s.store_sqrt_add(216, 3123, 214);
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(3124), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[3206] = (s.v[3123] > 0.0);
            s.v[3206] = if s.b[3206] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3205])) && s.b[3206]) {
                s.store_sqrt(216, 3123);
                s.store_scaled_div(217, 3124, 216, 0.5);
            }
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3205])) && (!s.b[3206])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_ad_value(232, A::add_scaled_product(A::sub(s.ad_value(404), s.ad_value(402)), 1.0, s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3207] = (s.v[79] > 0.0);
            s.v[3207] = if s.b[3207] { 1.0 } else { 0.0 };
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && s.b[3207]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3208] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[3208] = if s.b[3208] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) && s.b[3208]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) {
                s.store_add(404, 404, 236);
            }
            s.b[3209] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[3209] = if s.b[3209] { 1.0 } else { 0.0 };
            if ((((s.b[3111] && s.b[3112]) && s.b[3196]) && (!s.b[3207])) && s.b[3209]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            s.store_ad_value(223, {
                if (s.v[3123] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(3123)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3196]) {
            s.store_mul(3113, 982, 223);
            s.store_mul(3114, 3115, 3113);
            s.store_offset_div(100, 3114, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.b[3111] && s.b[3112]) {
            s.store_sub(399, 398, 354);
        }

        s.b[3211] = (s.v[407] < 0.0);
        s.v[3211] = if s.b[3211] { 1.0 } else { 0.0 };

        if ((s.b[3111] && s.b[3112]) && s.b[3211]) {
            s.store_neg(407, 407);
        }

        s.b[3212] = (p.p55 == 0.0);
        s.v[3212] = if s.b[3212] { 1.0 } else { 0.0 };

        s.b[3213] = (p.p50 == 0.0);
        s.v[3213] = if s.b[3213] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && s.b[3213]) {
            s.store_neg(3116, 404);
        }

        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && (!s.b[3213])) {
            s.copy_ad(3116, 396);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {
            s.store_sqrt_offset_ad(782, A::mul(A::offset(s.ad_value(3116), p.p137), A::offset(s.ad_value(3116), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(3116), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3116), p.p137), 782, 0.5);
        }

        s.b[3214] = (s.v[336] < 0.0);
        s.v[3214] = if s.b[3214] { 1.0 } else { 0.0 };

        if ((((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) && s.b[3214]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3211]) && s.b[3212]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_sub(407, 407, 603);
        }

        if (s.b[3111] && s.b[3112]) {
            s.copy_ad(698, 354);
        }

        s.b[3215] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] == 0.0));
        s.v[3215] = if s.b[3215] { 1.0 } else { 0.0 };

        if s.b[3215] {
            s.store_scalar(2623, 1.0);
            s.store_scalar(289, s.v[564]);
            s.store_scalar(290, p.p276);
            s.store_scalar(335, (s.v[188] * s.v[635]));
        }

        s.b[3216] = (s.v[949] == 1.0);
        s.v[3216] = if s.b[3216] { 1.0 } else { 0.0 };

        if (s.b[3215] && s.b[3216]) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add(s.ad_value(290), s.ad_value(791)));
            s.store_scale(339, 335, p.p66);
            s.store_sub_from_scalar(343, 1.2, 87);
            s.store_ad_value(291, A::add_scaled_products(s.ad_value(791), s.ad_value(339), 1.0, s.ad_value(338), s.ad_value(343), (-1.0)));
        }

        if (s.b[3215] && (!s.b[3216])) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p66);
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
            s.store_ad_value(291, A::add_scaled_products(A::sub(s.ad_value(791), s.ad_value(790)), s.ad_value(339), 1.0, s.ad_value(338), s.ad_value(343), (-1.0)));
        }

        s.b[3217] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] == 0.0));
        s.v[3217] = if s.b[3217] { 1.0 } else { 0.0 };

        if s.b[3217] {
            s.store_scalar(2626, 1.0);
            s.store_scalar(289, s.v[564]);
            s.store_scalar(290, p.p276);
            s.store_scale(335, 412, s.v[635]);
        }

        s.b[3218] = (s.v[949] == 1.0);
        s.v[3218] = if s.b[3218] { 1.0 } else { 0.0 };

        if (s.b[3217] && s.b[3218]) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add_scaled_inputs3(s.ad_value(290), 1.0, s.ad_value(791), 1.0, s.ad_value(790), -1.0));
            s.store_scale(339, 335, p.p63);
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
            s.store_ad_value(292, A::add_scaled_products(A::sub(s.ad_value(791), s.ad_value(790)), s.ad_value(339), 1.0, s.ad_value(338), s.ad_value(343), (-1.0)));
        }

        if (s.b[3217] && (!s.b[3218])) {
            s.store_mul_ad_product_rhs(338, 289, s.ad_value(335), A::add(s.ad_value(290), s.ad_value(791)));
            s.store_scale(339, 335, p.p63);
            s.store_sub_from_scalar(343, 1.2, 87);
            s.store_ad_value(292, A::add_scaled_products(s.ad_value(791), s.ad_value(339), 1.0, s.ad_value(338), s.ad_value(343), (-1.0)));
        }

        if s.b[768] {
            s.store_scalar(295, (s.v[505] * (-s.v[635])));
        }

        s.b[3219] = (s.v[2623] == 0.0);
        s.v[3219] = if s.b[3219] { 1.0 } else { 0.0 };

        if ((!s.b[768]) && s.b[3219]) {
            s.store_scalar(295, (((-s.v[188]) * p.p66) * s.v[635]));
        }

        s.store_mul_neg_lhs(297, 295, 734);

        if s.b[769] {
            s.store_scalar(294, (s.v[506] * (-s.v[635])));
        }

        s.b[3220] = (s.v[2626] == 0.0);
        s.v[3220] = if s.b[3220] { 1.0 } else { 0.0 };

        if ((!s.b[769]) && s.b[3220]) {
            s.store_scale(294, 412, (-(p.p63 * s.v[635])));
        }

        s.store_mul_scaled_ad_rhs(298, 294, -1.0, A::sub(s.ad_value(734), s.ad_value(733)));

        s.b[3221] = (s.v[949] == 1.0);
        s.v[3221] = if s.b[3221] { 1.0 } else { 0.0 };

        if s.b[3221] {
            s.store_scaled_sub(357, 790, 94, p.p431);
            s.store_mul(360, 338, 357);
            s.store_mul(361, 338, 357);
        }

        if (!s.b[3221]) {
            s.store_scaled_sub(357, 790, 94, (-p.p431));
            s.store_mul(362, 338, 357);
            s.store_mul(363, 338, 357);
        }

        s.v[296] = ((-s.v[525]) * s.v[582]);

        s.store_scaled_sub(293, 731, 728, (-s.v[296]));

        s.v[172] = s.v[507];

        s.b[3222] = (s.v[78] != 0.0);
        s.v[3222] = if s.b[3222] { 1.0 } else { 0.0 };

        if s.b[3222] {
            s.store_ad_value(168, A::add_scaled_inputs3(s.ad_value(790), s.v[172], s.ad_value(87), s.v[172], s.ad_value(91), (1.0 - s.v[172])));
        }

        s.b[3223] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[3223] = if s.b[3223] { 1.0 } else { 0.0 };

        if (s.b[3222] && s.b[3223]) {
            s.store_offset_sub_ad(781, s.ad_value(168), A::offset(A::add(s.ad_value(87), s.ad_value(790)), (-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
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
        if (s.b[3222] && s.b[3223]) {
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

        s.b[3224] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3224] = if s.b[3224] { 1.0 } else { 0.0 };

        s.b[3225] = (2.0 == 1.0);
        s.v[3225] = if s.b[3225] { 1.0 } else { 0.0 };

        if (((s.b[3222] && s.b[3223]) && s.b[3224]) && s.b[3225]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3226] = (2.0 == 2.0);
        s.v[3226] = if s.b[3226] { 1.0 } else { 0.0 };

        if ((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && s.b[3226]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3227] = (2.0 == 4.0);
        s.v[3227] = if s.b[3227] { 1.0 } else { 0.0 };

        if (((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3227]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3228] = (2.0 == 8.0);
        s.v[3228] = if s.b[3228] { 1.0 } else { 0.0 };

        if ((((((s.b[3222] && s.b[3223]) && s.b[3224]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3227])) && s.b[3228]) {
            s.store_scalar(720, 4.0);
        }

        if ((s.b[3222] && s.b[3223]) && s.b[3224]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign95820_loop_guard: usize = 0;
        while {
            let assign95820_cond_e148370: f64 = if (((s.b[3222] && s.b[3223]) && s.b[3224]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign95820_cond_e148370 != 0.0
        } {
            assign95820_loop_guard += 1;
            assert!(assign95820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3222] && s.b[3223]) && s.b[3224]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((s.b[3222] && s.b[3223]) && (!s.b[3224])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (s.b[3222] && s.b[3223]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), (10.0 * 2.220446049250313e-16), s.ad_value(770), 1.0));
            s.store_add_ad_lhs(168, A::offset(A::add(s.ad_value(87), s.ad_value(790)), (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16))))), 780);
        }

        if (s.b[3222] && s.b[3223]) {
        }

        if (s.b[3222] && (!s.b[3223])) {
        }

        if (s.b[3222] && (!s.b[3223])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3222] && s.b[82]) {
            s.store_scalar(303, 0.0);
        }

        s.b[3229] = ((s.v[248] < 1e-15) || (s.v[348] < 1e-6));
        s.v[3229] = if s.b[3229] { 1.0 } else { 0.0 };

        if (((!s.b[3222]) && s.b[82]) && s.b[3229]) {
            s.store_scalar(303, 0.0);
        }

        if (((!s.b[3222]) && s.b[82]) && (!s.b[3229])) {
            s.store_ad_value(303, A::div_scaled_product(A::div(s.ad_value(248), s.ad_value(238)), s.ad_value(155), 1.0, s.ad_value(162), 1.0));
        }

        s.b[3230] = (!s.b[82]);
        s.v[3230] = if s.b[3230] { 1.0 } else { 0.0 };

        if s.b[3230] {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[3230]) {
            s.store_scale(336, 684, ((1.034943e-10 * s.v[635]) * 1.3));
        }

        s.b[3231] = (p.p133 != 0.0);
        s.v[3231] = if s.b[3231] { 1.0 } else { 0.0 };

        if ((!s.b[3230]) && s.b[3231]) {
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(87), 1.0, s.ad_value(303), s.ad_value(162), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(1439), s.v[172], s.ad_value(87), s.v[172], s.ad_value(304), (1.0 - s.v[172])));
            s.store_mul_scale_ad_lhs(305, A::add_scaled_inputs3(s.ad_value(87), 1.0, s.ad_value(1439), 1.0, s.ad_value(335), -1.0), (-1.0 / (p.p133)), 336);
        }

        s.b[3232] = (p.p134 != 0.0);
        s.v[3232] = if s.b[3232] { 1.0 } else { 0.0 };

        if ((!s.b[3230]) && s.b[3232]) {
            s.store_add_scaled_inputs(305, 305, 1.0, 792, s.v[671]);
        }

        s.v[300] = s.v[670];

        s.v[302] = s.v[670];

        s.store_scaled_sub(299, 734, 733, s.v[300]);

        s.store_scale(301, 734, s.v[302]);

        s.b[3233] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3233] = if s.b[3233] { 1.0 } else { 0.0 };

        if s.b[3233] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[3234] = (p.p48 > 0.0);
        s.v[3234] = if s.b[3234] { 1.0 } else { 0.0 };

        s.b[3235] = (p.p15 > s.v[632]);
        s.v[3235] = if s.b[3235] { 1.0 } else { 0.0 };

        if ((s.b[3233] && s.b[3234]) && s.b[3235]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[3233] && s.b[3234]) && (!s.b[3235])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scalar(875, 0.0);
            s.store_scalar(876, 0.0);
            s.store_scale(877, 836, p.p15);
            s.store_scale(878, 837, p.p15);
        }

        if (s.b[3233] && (!s.b[3234])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[3233] {
            s.store_ad_value(847, A::add_scaled_inputs3(s.ad_value(873), 1.0, s.ad_value(875), 1.0, s.ad_value(877), 1.0));
        }

        s.b[3236] = (s.v[847] > 0.0);
        s.v[3236] = if s.b[3236] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3236]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_ad(848, A::div_from_scalar(s.v[820], s.ad_value(154)), A::ln(A::offset(A::div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0), 1.0)));
            s.store_ad_value(849, A::exp_scaled_input(A::offset(s.ad_value(676), (-1.0)), p.p512));
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[3233] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[3237] = (p.p48 > 0.0);
        s.v[3237] = if s.b[3237] { 1.0 } else { 0.0 };

        s.b[3238] = (p.p16 > s.v[632]);
        s.v[3238] = if s.b[3238] { 1.0 } else { 0.0 };

        if ((s.b[3233] && s.b[3237]) && s.b[3238]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

        if ((s.b[3233] && s.b[3237]) && (!s.b[3238])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[3233] && (!s.b[3237])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[3233] {
            s.store_ad_value(852, A::add_scaled_inputs3(s.ad_value(879), 1.0, s.ad_value(881), 1.0, s.ad_value(883), 1.0));
        }

        s.b[3239] = (s.v[852] > 0.0);
        s.v[3239] = if s.b[3239] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3239]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_ad(853, A::div_from_scalar(s.v[825], s.ad_value(154)), A::ln(A::offset(A::div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0), 1.0)));
            s.store_ad_value(854, A::exp_scaled_input(A::offset(s.ad_value(676), (-1.0)), p.p535));
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[3233] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[3240] = (p.p15 > s.v[632]);
        s.v[3240] = if s.b[3240] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3240]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[3233] && (!s.b[3240])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[3241] = (s.v[832] < 0.0);
        s.v[3241] = if s.b[3241] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3241]) {
            s.store_scalar(832, 0.0);
        }

        s.b[3242] = (s.v[833] < 0.0);
        s.v[3242] = if s.b[3242] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3242]) {
            s.store_scalar(833, 0.0);
        }

        s.b[3243] = (s.v[834] < 0.0);
        s.v[3243] = if s.b[3243] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3243]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[3233] {
            s.store_sub_from_scalar_ad(841, p.p506, A::scale(s.ad_value(391), p.p487));
            s.store_sub_from_scalar_ad(842, p.p507, A::scale(s.ad_value(391), p.p489));
            s.store_sub_from_scalar_ad(843, p.p508, A::scale(s.ad_value(391), p.p491));
        }

        s.b[3244] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.v[3244] = if s.b[3244] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3244]) {
            s.store_scalar(841, 0.01);
        }

        s.b[3245] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.v[3245] = if s.b[3245] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_90(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[3233] && s.b[3245]) {
            s.store_scalar(842, 0.01);
        }

        s.b[3246] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.v[3246] = if s.b[3246] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3246]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[3233] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[3247] = (p.p16 > s.v[632]);
        s.v[3247] = if s.b[3247] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3247]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[3233] && (!s.b[3247])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[3248] = (s.v[835] < 0.0);
        s.v[3248] = if s.b[3248] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3248]) {
            s.store_scalar(835, 0.0);
        }

        s.b[3249] = (s.v[838] < 0.0);
        s.v[3249] = if s.b[3249] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3249]) {
            s.store_scalar(838, 0.0);
        }

        s.b[3250] = (s.v[839] < 0.0);
        s.v[3250] = if s.b[3250] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3250]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[3233] {
            s.store_sub_from_scalar_ad(844, p.p529, A::scale(s.ad_value(391), p.p488));
            s.store_sub_from_scalar_ad(845, p.p530, A::scale(s.ad_value(391), p.p490));
            s.store_sub_from_scalar_ad(846, p.p531, A::scale(s.ad_value(391), p.p492));
        }

        s.b[3251] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.v[3251] = if s.b[3251] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3251]) {
            s.store_scalar(844, 0.01);
        }

        s.b[3252] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.v[3252] = if s.b[3252] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3252]) {
            s.store_scalar(845, 0.01);
        }

        s.b[3253] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.v[3253] = if s.b[3253] { 1.0 } else { 0.0 };

        if (s.b[3233] && s.b[3253]) {
            s.store_scalar(846, 0.01);
        }

        if (!s.b[3233]) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        s.store_scale(344, 850, p.p511);

        s.store_scale(343, 849, p.p510);

        s.b[3254] = (s.v[873] > 0.0);
        s.v[3254] = if s.b[3254] { 1.0 } else { 0.0 };

        if s.b[3254] {
            s.store_mul(334, 874, 343);
            s.store_mul_neg_lhs(332, 860, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3255] = (s.v[860] < s.v[848]);
        s.v[3255] = if s.b[3255] { 1.0 } else { 0.0 };

        if (s.b[3254] && s.b[3255]) {
            s.store_mul(332, 860, 850);
        }

        s.b[3256] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3256] = if s.b[3256] { 1.0 } else { 0.0 };

        if ((s.b[3254] && s.b[3255]) && s.b[3256]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3254] && s.b[3255]) && (!s.b[3256])) {
            s.store_exp(335, 332);
        }

        if (s.b[3254] && (!s.b[3255])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 873, 850, 335);
        }

        s.store_scale(346, 874, p.p514);

        s.b[3257] = (s.v[875] > 0.0);
        s.v[3257] = if s.b[3257] { 1.0 } else { 0.0 };

        if s.b[3257] {
            s.store_mul(334, 876, 343);
            s.store_mul_neg_lhs(332, 860, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3258] = (s.v[860] < s.v[848]);
        s.v[3258] = if s.b[3258] { 1.0 } else { 0.0 };

        if (s.b[3257] && s.b[3258]) {
            s.store_mul(332, 860, 850);
        }

        s.b[3259] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3259] = if s.b[3259] { 1.0 } else { 0.0 };

        if ((s.b[3257] && s.b[3258]) && s.b[3259]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3257] && s.b[3258]) && (!s.b[3259])) {
            s.store_exp(335, 332);
        }

        if (s.b[3257] && (!s.b[3258])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 875, 850, 335);
        }

        s.store_scale(346, 876, p.p514);

        s.b[3260] = (p.p48 > 0.0);
        s.v[3260] = if s.b[3260] { 1.0 } else { 0.0 };

        s.b[3261] = (s.v[877] > 0.0);
        s.v[3261] = if s.b[3261] { 1.0 } else { 0.0 };

        if (s.b[3260] && s.b[3261]) {
            s.store_mul(334, 878, 343);
            s.store_mul_neg_lhs(332, 868, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3262] = (s.v[868] < s.v[848]);
        s.v[3262] = if s.b[3262] { 1.0 } else { 0.0 };

        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {
            s.store_mul(332, 868, 850);
        }

        s.b[3263] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3263] = if s.b[3263] { 1.0 } else { 0.0 };

        if (((s.b[3260] && s.b[3261]) && s.b[3262]) && s.b[3263]) {
            s.store_scalar(335, 0.0);
        }

        if (((s.b[3260] && s.b[3261]) && s.b[3262]) && (!s.b[3263])) {
            s.store_exp(335, 332);
        }

        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {
            s.copy_ad(335, 851);
            s.store_mul3_lhs(338, 877, 850, 335);
        }

        if s.b[3260] {
            s.store_scale(346, 878, p.p514);
        }

        s.store_scale(344, 855, p.p534);

        s.store_scale(343, 854, p.p533);

        s.b[3264] = (s.v[879] > 0.0);
        s.v[3264] = if s.b[3264] { 1.0 } else { 0.0 };

        if s.b[3264] {
            s.store_mul(334, 880, 343);
            s.store_mul_neg_lhs(332, 859, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3265] = (s.v[859] < s.v[853]);
        s.v[3265] = if s.b[3265] { 1.0 } else { 0.0 };

        if (s.b[3264] && s.b[3265]) {
            s.store_mul(332, 859, 855);
        }

        s.b[3266] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3266] = if s.b[3266] { 1.0 } else { 0.0 };

        if ((s.b[3264] && s.b[3265]) && s.b[3266]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3264] && s.b[3265]) && (!s.b[3266])) {
            s.store_exp(335, 332);
        }

        if (s.b[3264] && (!s.b[3265])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 879, 855, 335);
        }

        s.store_scale(346, 880, p.p537);

        s.b[3267] = (s.v[881] > 0.0);
        s.v[3267] = if s.b[3267] { 1.0 } else { 0.0 };

        if s.b[3267] {
            s.store_mul(334, 882, 343);
            s.store_mul_neg_lhs(332, 859, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3268] = (s.v[859] < s.v[853]);
        s.v[3268] = if s.b[3268] { 1.0 } else { 0.0 };

        if (s.b[3267] && s.b[3268]) {
            s.store_mul(332, 859, 855);
        }

        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3269] = if s.b[3269] { 1.0 } else { 0.0 };

        if ((s.b[3267] && s.b[3268]) && s.b[3269]) {
            s.store_scalar(335, 0.0);
        }

        if ((s.b[3267] && s.b[3268]) && (!s.b[3269])) {
            s.store_exp(335, 332);
        }

        if (s.b[3267] && (!s.b[3268])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 881, 855, 335);
        }

        s.store_scale(346, 882, p.p537);

        s.b[3270] = (p.p48 > 0.0);
        s.v[3270] = if s.b[3270] { 1.0 } else { 0.0 };

        s.b[3271] = (s.v[883] > 0.0);
        s.v[3271] = if s.b[3271] { 1.0 } else { 0.0 };

        if (s.b[3270] && s.b[3271]) {
            s.store_mul(334, 884, 343);
            s.store_mul_neg_lhs(332, 867, 344);
            s.store_exp(336, 332);
            s.copy_ad(337, 336);
        }

        s.b[3272] = (s.v[867] < s.v[853]);
        s.v[3272] = if s.b[3272] { 1.0 } else { 0.0 };

        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {
            s.store_mul(332, 867, 855);
        }

        s.b[3273] = (s.v[332] < ((-3.0) * 34.0));
        s.v[3273] = if s.b[3273] { 1.0 } else { 0.0 };

        if (((s.b[3270] && s.b[3271]) && s.b[3272]) && s.b[3273]) {
            s.store_scalar(335, 0.0);
        }

        if (((s.b[3270] && s.b[3271]) && s.b[3272]) && (!s.b[3273])) {
            s.store_exp(335, 332);
        }

        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {
            s.copy_ad(335, 856);
            s.store_mul3_lhs(338, 883, 855, 335);
        }

        if s.b[3270] {
            s.store_scale(346, 884, p.p537);
        }

        s.b[3274] = (s.v[832] > 0.0);
        s.v[3274] = if s.b[3274] { 1.0 } else { 0.0 };

        s.b[3275] = (s.v[860] < 0.0);
        s.v[3275] = if s.b[3275] { 1.0 } else { 0.0 };

        if (s.b[3274] && s.b[3275]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(841)));
        }

        s.b[3276] = (p.p503 == 0.5);
        s.v[3276] = if s.b[3276] { 1.0 } else { 0.0 };

        if ((s.b[3274] && s.b[3275]) && s.b[3276]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3274] && s.b[3275]) && (!s.b[3276])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p503))
                }
            });
        }

        if (s.b[3274] && s.b[3275]) {
            s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p503)), 0.0);
        }

        if (s.b[3274] && (!s.b[3275])) {
            s.copy_ad(335, 832);
            s.store_scaled_div(336, 832, 841, p.p503);
            s.store_mul_ad_rhs(891, 860, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5));
        }

        if (!s.b[3274]) {
            s.store_scalar(891, 0.0);
        }

        s.b[3277] = (s.v[833] > 0.0);
        s.v[3277] = if s.b[3277] { 1.0 } else { 0.0 };

        s.b[3278] = (s.v[860] < 0.0);
        s.v[3278] = if s.b[3278] { 1.0 } else { 0.0 };

        if (s.b[3277] && s.b[3278]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(842)));
        }

        s.b[3279] = (p.p504 == 0.5);
        s.v[3279] = if s.b[3279] { 1.0 } else { 0.0 };

        if ((s.b[3277] && s.b[3278]) && s.b[3279]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3277] && s.b[3278]) && (!s.b[3279])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p504))
                }
            });
        }

        if (s.b[3277] && s.b[3278]) {
            s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p504)), 0.0);
        }

        if (s.b[3277] && (!s.b[3278])) {
            s.copy_ad(335, 833);
            s.store_scaled_div(336, 833, 842, p.p504);
            s.store_mul_ad_rhs(893, 860, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5));
        }

        if (!s.b[3277]) {
            s.store_scalar(893, 0.0);
        }

        s.b[3280] = (p.p48 > 0.0);
        s.v[3280] = if s.b[3280] { 1.0 } else { 0.0 };

        s.b[3281] = (s.v[834] > 0.0);
        s.v[3281] = if s.b[3281] { 1.0 } else { 0.0 };

        s.b[3282] = (s.v[868] < 0.0);
        s.v[3282] = if s.b[3282] { 1.0 } else { 0.0 };

        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(868), s.ad_value(843)));
        }

        s.b[3283] = (p.p505 == 0.5);
        s.v[3283] = if s.b[3283] { 1.0 } else { 0.0 };

        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && s.b[3283]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

    }

    pub(super) fn stamp_reactive_block_91(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && (!s.b[3283])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p505))
                }
            });
        }

        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {
            s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);
        }

        if ((s.b[3280] && s.b[3281]) && (!s.b[3282])) {
            s.copy_ad(335, 834);
            s.store_scaled_div(336, 834, 843, p.p505);
            s.store_mul_ad_rhs(895, 868, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(868), s.ad_value(336), 0.5));
        }

        if (s.b[3280] && (!s.b[3281])) {
            s.store_scalar(895, 0.0);
        }

        s.b[3284] = (s.v[834] > 0.0);
        s.v[3284] = if s.b[3284] { 1.0 } else { 0.0 };

        s.b[3285] = (s.v[860] < 0.0);
        s.v[3285] = if s.b[3285] { 1.0 } else { 0.0 };

        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(843)));
        }

        s.b[3286] = (p.p505 == 0.5);
        s.v[3286] = if s.b[3286] { 1.0 } else { 0.0 };

        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && s.b[3286]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && (!s.b[3286])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p505))
                }
            });
        }

        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {
            s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p505)), 0.0);
        }

        if (((!s.b[3280]) && s.b[3284]) && (!s.b[3285])) {
            s.copy_ad(335, 834);
            s.store_scaled_div(336, 834, 843, p.p505);
            s.store_mul_ad_rhs(895, 860, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(860), s.ad_value(336), 0.5));
        }

        if ((!s.b[3280]) && (!s.b[3284])) {
            s.store_scalar(895, 0.0);
        }

        s.b[3287] = (s.v[835] > 0.0);
        s.v[3287] = if s.b[3287] { 1.0 } else { 0.0 };

        s.b[3288] = (s.v[859] < 0.0);
        s.v[3288] = if s.b[3288] { 1.0 } else { 0.0 };

        if (s.b[3287] && s.b[3288]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(844)));
        }

        s.b[3289] = (p.p526 == 0.5);
        s.v[3289] = if s.b[3289] { 1.0 } else { 0.0 };

        if ((s.b[3287] && s.b[3288]) && s.b[3289]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3287] && s.b[3288]) && (!s.b[3289])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p526))
                }
            });
        }

        if (s.b[3287] && s.b[3288]) {
            s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p526)), 0.0);
        }

        if (s.b[3287] && (!s.b[3288])) {
            s.copy_ad(335, 835);
            s.store_scaled_div(336, 835, 844, p.p526);
            s.store_mul_ad_rhs(892, 859, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5));
        }

        if (!s.b[3287]) {
            s.store_scalar(892, 0.0);
        }

        s.b[3290] = (s.v[838] > 0.0);
        s.v[3290] = if s.b[3290] { 1.0 } else { 0.0 };

        s.b[3291] = (s.v[859] < 0.0);
        s.v[3291] = if s.b[3291] { 1.0 } else { 0.0 };

        if (s.b[3290] && s.b[3291]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(845)));
        }

        s.b[3292] = (p.p527 == 0.5);
        s.v[3292] = if s.b[3292] { 1.0 } else { 0.0 };

        if ((s.b[3290] && s.b[3291]) && s.b[3292]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((s.b[3290] && s.b[3291]) && (!s.b[3292])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p527))
                }
            });
        }

        if (s.b[3290] && s.b[3291]) {
            s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p527)), 0.0);
        }

        if (s.b[3290] && (!s.b[3291])) {
            s.copy_ad(335, 838);
            s.store_scaled_div(336, 838, 845, p.p527);
            s.store_mul_ad_rhs(894, 859, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5));
        }

        if (!s.b[3290]) {
            s.store_scalar(894, 0.0);
        }

        s.b[3293] = (p.p48 > 0.0);
        s.v[3293] = if s.b[3293] { 1.0 } else { 0.0 };

        s.b[3294] = (s.v[839] > 0.0);
        s.v[3294] = if s.b[3294] { 1.0 } else { 0.0 };

        s.b[3295] = (s.v[867] < 0.0);
        s.v[3295] = if s.b[3295] { 1.0 } else { 0.0 };

        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(867), s.ad_value(846)));
        }

        s.b[3296] = (p.p528 == 0.5);
        s.v[3296] = if s.b[3296] { 1.0 } else { 0.0 };

        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && s.b[3296]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && (!s.b[3296])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p528))
                }
            });
        }

        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {
            s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);
        }

        if ((s.b[3293] && s.b[3294]) && (!s.b[3295])) {
            s.copy_ad(335, 839);
            s.store_scaled_div(336, 839, 846, p.p528);
            s.store_mul_ad_rhs(896, 867, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(867), s.ad_value(336), 0.5));
        }

        if (s.b[3293] && (!s.b[3294])) {
            s.store_scalar(896, 0.0);
        }

        s.b[3297] = (s.v[839] > 0.0);
        s.v[3297] = if s.b[3297] { 1.0 } else { 0.0 };

        s.b[3298] = (s.v[859] < 0.0);
        s.v[3298] = if s.b[3298] { 1.0 } else { 0.0 };

        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(846)));
        }

        s.b[3299] = (p.p528 == 0.5);
        s.v[3299] = if s.b[3299] { 1.0 } else { 0.0 };

        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && s.b[3299]) {
            s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));
        }

        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && (!s.b[3299])) {
            s.store_ad_value(840, {
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p528))
                }
            });
        }

        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {
            s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p.p528)), 0.0);
        }

        if (((!s.b[3293]) && s.b[3297]) && (!s.b[3298])) {
            s.copy_ad(335, 839);
            s.store_scaled_div(336, 839, 846, p.p528);
            s.store_mul_ad_rhs(896, 859, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(859), s.ad_value(336), 0.5));
        }

        if ((!s.b[3293]) && (!s.b[3297])) {
            s.store_scalar(896, 0.0);
        }

        s.b[3300] = (p.p48 > 0.0);
        s.v[3300] = if s.b[3300] { 1.0 } else { 0.0 };

        if s.b[3300] {
            s.store_scaled_add(66, 892, 894, s.v[365]);
            s.store_scaled_add(65, 891, 893, s.v[365]);
            s.store_scale(68, 896, s.v[365]);
            s.store_scale(67, 895, s.v[365]);
        }

        if (!s.b[3300]) {
            s.store_ad_value(66, A::add_scaled_inputs3(s.ad_value(892), s.v[365], s.ad_value(894), s.v[365], s.ad_value(896), s.v[365]));
            s.store_ad_value(65, A::add_scaled_inputs3(s.ad_value(891), s.v[365], s.ad_value(893), s.v[365], s.ad_value(895), s.v[365]));
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

        s.store_ad_value(911, A::div_scaled_product(s.ad_value(909), s.ad_value(910), 2.0, A::add(s.ad_value(909), s.ad_value(910)), 1.0));

        s.store_powf(336, 676, p.p547);

        s.store_scale(913, 336, p.p544);

        s.store_sqrt_mul(912, 913, 911);

        s.store_mul_scaled_ad_rhs(934, 155, s.v[906], A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))));

        s.store_mul_scaled_ad_rhs(935, 155, s.v[906], A::add(A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), A::div_from_scalar(p.p545, s.ad_value(912))));

        s.b[3301] = (p.p539 > 0.0);
        s.v[3301] = if s.b[3301] { 1.0 } else { 0.0 };

        if s.b[3301] {
            s.store_scalar(936, s.v[820]);
            s.store_exp_mul(937, 860, 850);
        }

        s.b[3302] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);
        s.v[3302] = if s.b[3302] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3302]) {
            s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div(A::sub(s.ad_value(935), s.ad_value(934)), s.ad_value(936)))));
        }

        if (s.b[3301] && (!s.b[3302])) {
            s.store_scalar(938, 1.0);
        }

        s.b[3303] = ((p.p542 == 0.0) || (s.v[860] < s.v[934]));
        s.v[3303] = if s.b[3303] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3303]) {
            s.store_scale(941, 937, p.p541);
        }

        if (s.b[3301] && (!s.b[3303])) {
            s.store_mul_scaled_ad_rhs(941, 937, p.p541, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542))));
        }

        if s.b[3301] {
            s.store_ad_value(941, {
                if (s.v[941] > 1e20) {
                    A::constant(1e20)
                } else {
                    s.ad_value(941)
                }
            });
        }

        if s.b[3301] {
            s.store_mul(939, 907, 941);
            s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p.p13));
        }

        s.b[3304] = (p.p543 > 0.0);
        s.v[3304] = if s.b[3304] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3304]) {
            s.store_scale(922, 920, p.p543);
            s.store_scaled_voltage(924, ctx, nodes, Some(16), None, p.p543);
            s.store_scaled_sub(926, 924, 922, 1.0 / (p.p543));
            s.store_scale(928, 924, 1.0 / (p.p543));
        }

        if (s.b[3301] && (!s.b[3304])) {
            s.copy_ad(922, 920);
            s.copy_ad(928, 922);
        }

        s.b[3305] = ((p.p542 == 0.0) || (s.v[860] < s.v[935]));
        s.v[3305] = if s.b[3305] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3305]) {
            s.store_scale(942, 938, p.p541);
        }

        if (s.b[3301] && (!s.b[3305])) {
            s.store_mul_scaled_ad_rhs(942, 938, p.p541, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548), (-p.p542))));
        }

        if s.b[3301] {
            s.store_ad_value(942, {
                if (s.v[942] > 1e20) {
                    A::constant(1e20)
                } else {
                    s.ad_value(942)
                }
            });
        }

        if s.b[3301] {
            s.store_mul(940, 907, 942);
            s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p.p13));
        }

        s.b[3306] = (p.p543 > 0.0);
        s.v[3306] = if s.b[3306] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3306]) {
            s.store_scale(923, 921, p.p543);
            s.store_scaled_voltage(925, ctx, nodes, Some(17), None, p.p543);
            s.store_scaled_sub(927, 925, 923, 1.0 / (p.p543));
            s.store_scale(929, 925, 1.0 / (p.p543));
        }

        if (s.b[3301] && (!s.b[3306])) {
            s.copy_ad(923, 921);
            s.copy_ad(929, 923);
        }

        if s.b[3301] {
            s.store_sub_from_scalar(914, p.p506, 860);
            s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));
            s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);
            s.store_scaled_add(914, 914, 782, 0.5);
        }

        s.b[3307] = (s.v[914] < 0.0);
        s.v[3307] = if s.b[3307] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3307]) {
            s.store_scalar(914, 0.0);
            s.store_scalar(334, 0.0);
        }

        if s.b[3301] {
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
        if s.b[3301] {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if s.b[3301] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(915, p.p545, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[3308] = (p.p546 > 0.0);
        s.v[3308] = if s.b[3308] { 1.0 } else { 0.0 };

        if (s.b[3301] && s.b[3308]) {
            s.store_scale(930, 915, p.p546);
            s.store_scaled_voltage(931, ctx, nodes, Some(18), None, p.p546);
            s.store_scaled_sub(932, 931, 930, 1.0 / (p.p546));
            s.store_scale(933, 931, 1.0 / (p.p546));
        }

        if (s.b[3301] && (!s.b[3308])) {
            s.copy_ad(930, 915);
            s.copy_ad(933, 930);
        }

        if s.b[3301] {
            s.store_scalar(916, ((-((s.v[903] * p.p13) * 1.6021918e-19)) * p.p545));
            s.store_mul_ad_product_rhs(917, 912, s.ad_value(928), A::sub(A::exp(A::div_from_scalar((-p.p545), s.ad_value(912))), A::exp(A::div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0))));
            s.store_mul_ad_product_rhs(918, 912, s.ad_value(929), A::offset(A::exp(A::div_scaled_inputs(A::sub_from_scalar(p.p545, s.ad_value(933)), -1.0, s.ad_value(912), 1.0)), (-1.0)));
            s.store_neg_ad(919, A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(917), 1.0, s.ad_value(918), 1.0));
            s.store_add_scaled_inputs(65, 65, 1.0, 919, s.v[365]);
        }

        s.b[3309] = ((p.p539 > 0.0) && (p.p543 > 0.0));
        s.v[3309] = if s.b[3309] { 1.0 } else { 0.0 };

        s.b[3310] = ((p.p539 > 0.0) && (p.p546 > 0.0));
        s.v[3310] = if s.b[3310] { 1.0 } else { 0.0 };

        s.b[3311] = (p.p46 == 1.0);
        s.v[3311] = if s.b[3311] { 1.0 } else { 0.0 };

        s.b[3312] = ((s.v[486] > 0.0) && (s.v[454] > 0.0));
        s.v[3312] = if s.b[3312] { 1.0 } else { 0.0 };

        if (s.b[3311] && s.b[3312]) {
            s.store_mul(335, 665, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_ad_value(339, A::add_scaled_product(A::sub(s.ad_value(335), s.ad_value(155)), 1.0, s.ad_value(666), s.ad_value(1438), (-1.0)));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[3313] = ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0));
        s.v[3313] = if s.b[3313] { 1.0 } else { 0.0 };

        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {
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

        s.b[3314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[3314] = if s.b[3314] { 1.0 } else { 0.0 };

        s.b[3315] = (4.0 == 1.0);
        s.v[3315] = if s.b[3315] { 1.0 } else { 0.0 };

        if ((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && s.b[3315]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3316] = (4.0 == 2.0);
        s.v[3316] = if s.b[3316] { 1.0 } else { 0.0 };

        if (((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && s.b[3316]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3317] = (4.0 == 4.0);
        s.v[3317] = if s.b[3317] { 1.0 } else { 0.0 };

        if ((((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && (!s.b[3316])) && s.b[3317]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3318] = (4.0 == 8.0);
        s.v[3318] = if s.b[3318] { 1.0 } else { 0.0 };

        if (((((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && (!s.b[3316])) && (!s.b[3317])) && s.b[3318]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign100800_loop_guard: usize = 0;
        while {
            let assign100800_cond_e152883: f64 = if ((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign100800_cond_e152883 != 0.0
        } {
            assign100800_loop_guard += 1;
            assert!(assign100800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3311] && s.b[3312]) && s.b[3313]) && (!s.b[3314])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(341), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(340, 341, 780);
        }

        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {
        }

        if ((s.b[3311] && s.b[3312]) && (!s.b[3313])) {
        }

        if ((s.b[3311] && s.b[3312]) && (!s.b[3313])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3311] && s.b[3312]) {
            s.store_sqrt(340, 340);
            s.store_ad_value(282, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(337), A::sub_from_scalar(1.0, s.ad_value(340)), 1.0));
            s.store_div_from_scalar_offset_input(336, s.v[582], 667, s.v[582]);
            s.store_ad_value(283, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1439), s.v[488], s.ad_value(109), 1.0), 1.0, s.ad_value(336), s.ad_value(282), (-1.0)));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[3319] = (s.v[283] < 0.0);
        s.v[3319] = if s.b[3319] { 1.0 } else { 0.0 };

        if ((s.b[3311] && s.b[3312]) && s.b[3319]) {
            s.store_scalar(283, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3311] && s.b[3312]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_ad(958, s.ad_value(957), A::offset(s.ad_value(387), (-s.v[764])), 1.0);
        }

        if (s.b[3311] && s.b[3312]) {
            s.store_ad_value(958, {
                if (s.v[958] <= 0.001) {
                    A::constant(0.001)
                } else {
                    s.ad_value(958)
                }
            });
        }

        if (s.b[3311] && s.b[3312]) {
            s.store_div(339, 668, 958);
            s.store_mul(340, 669, 958);
            s.store_exp_ad(336, A::div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
        }

        s.b[3321] = (s.v[78] == 0.0);
        s.v[3321] = if s.b[3321] { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && s.b[3321]) {
            s.store_scalar(346, p.p270);
            s.store_scalar(344, p.p271);
            s.copy_ad(337, 170);
            s.store_mul_ad_lhs(335, A::mul3(s.ad_value(346), s.ad_value(344), s.ad_value(337)), 337);
            s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);
        }

        if (s.v[81] != 0.0) {
            s.store_scalar(336, s.v[565]);
        }

        s.b[3322] = ((p.p26 != 0.0) && (s.v[78] == 0.0));
        s.v[3322] = if s.b[3322] { 1.0 } else { 0.0 };

        if s.b[3322] {
            s.store_scalar(309, s.v[522]);
            s.store_scalar(311, s.v[563]);
            s.store_scale(335, 238, 6.241449993689894e18);
            s.store_sqrt_offset_ad(782, A::mul(A::sub(s.ad_value(87), s.ad_value(1435)), A::sub(s.ad_value(87), s.ad_value(1435))), ((4.0 * 0.001) * 0.001));
            s.store_scaled_offset_ad(334, A::div(A::sub(s.ad_value(87), s.ad_value(1435)), s.ad_value(782)), 1.0, 0.5);
            s.store_ad_value(339, A::add_scaled_inputs3(s.ad_value(87), 0.5, s.ad_value(1435), ((-1.0) * 0.5), s.ad_value(782), 0.5));
        }

        s.b[3323] = (s.v[339] < 0.0);
        s.v[3323] = if s.b[3323] { 1.0 } else { 0.0 };

        if (s.b[3322] && s.b[3323]) {
            s.store_scalar(339, 0.0);
            s.store_scalar(334, 0.0);
        }

        if s.b[3322] {
            s.store_mul_scaled_ad_lhs(336, A::add_scaled_inputs3(s.ad_value(185), 1.0, A::div(s.ad_value(238), s.ad_value(339)), 1.0, s.ad_value(311), 1.0), 155, 6.241449993689894e18);
            s.store_sub_ad_lhs(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);
        }

        s.b[3324] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[3324] = if s.b[3324] { 1.0 } else { 0.0 };

        if (s.b[3322] && s.b[3324]) {
            let assign101330_ad_e153420: A = A::add_scaled_product(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(335), s.ad_value(336))), A::add(s.ad_value(337), s.ad_value(336))), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div(A::add(s.ad_value(337), s.ad_value(336)), A::add(s.ad_value(335), s.ad_value(336)))), 1.0);
            s.store_ad_value(338, A::add_scaled_product(assign101330_ad_e153420, 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), s.ad_value(253), 1.0));
        }

        if (s.b[3322] && (!s.b[3324])) {
            let assign101340_ad_e153471: A = A::add_scaled_product(A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(335), s.ad_value(336))), A::add(s.ad_value(337), s.ad_value(336))), A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), s.ad_value(253), 1.0);
            s.store_ad_value(338, assign101340_ad_e153471);
        }

        s.b[3325] = (((p.p30 != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3325] = if s.b[3325] { 1.0 } else { 0.0 };

        if s.b[3325] {
            s.store_div_ad_lhs(313, A::offset(A::sub(s.ad_value(168), s.ad_value(87)), (10.0 * 2.220446049250313e-16)), 170);
        }

        if s.b[3325] {
            s.store_ad_value(313, {
                if (s.v[313] >= 0.0) {
                    s.ad_value(313)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[3325] {
            s.store_scaled_mul(346, 254, 313, 1e-7);
        }

        s.b[3326] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3326] = if s.b[3326] { 1.0 } else { 0.0 };

        if (s.b[3325] && s.b[3326]) {
            s.store_scalar(341, 1.0);
        }

        s.b[3327] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3327] = if s.b[3327] { 1.0 } else { 0.0 };

        if ((s.b[3325] && (!s.b[3326])) && s.b[3327]) {
            s.copy_ad(341, 346);
        }

        if ((s.b[3325] && (!s.b[3326])) && (!s.b[3327])) {
            s.store_ad_value(341, {
                if (s.v[313] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(313), (p.p178 - 1.0))
                }
            });
        }

        if s.b[3325] {
            s.store_mul(342, 346, 341);
            s.store_offset(343, 342, 1.0);
        }

        if s.b[3325] {
            s.store_ad_value(344, {
                if (s.v[343] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(343), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if s.b[3325] {
            s.store_mul(345, 343, 344);
            s.store_mul(316, 254, 345);
            s.store_scaled_add(314, 253, 316, 0.5);
            s.store_square(334, 125);
        }

        if s.b[3325] {
            let assign101530_ad_e153698: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0);
            s.store_ad_value(315, A::div_scaled_product(A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), assign101530_ad_e153698, 1.0, A::mul(A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), s.ad_value(314)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_93(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[3325]) {
            s.store_scalar(315, 0.0);
        }

        s.b[3328] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3328] = if s.b[3328] { 1.0 } else { 0.0 };

        if s.b[3328] {
            s.store_sqrt(322, 319);
            s.store_add(336, 127, 322);
            s.store_square(337, 317);
            s.store_square(338, 319);
            s.store_scaled_mul(339, 317, 319, 42.0);
            s.store_ad_value(339, A::add_scaled_inputs3(s.ad_value(339), 1.0, s.ad_value(337), 4.0, s.ad_value(338), 4.0));
            s.store_add_ad_rhs(339, 339, A::mul3_scaled_output(s.ad_value(322), s.ad_value(127), A::add(s.ad_value(317), s.ad_value(319)), 20.0));
            s.store_square(344, 336);
            s.store_square(344, 344);
            s.store_div_ad_rhs(323, 339, A::mul(s.ad_value(344), s.ad_value(336)));
            s.store_mul_ad_product_lhs(324, A::div_from_scalar(s.v[632], s.ad_value(170)), s.ad_value(253), 185);
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

        s.b[3329] = ((s.v[81] != 0.0) || (p.p22 == 2.0));
        s.v[3329] = if s.b[3329] { 1.0 } else { 0.0 };

        if s.b[3329] {
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.copy_ad(708, 247);
            s.store_scale(132, 132, s.v[365]);
        }

        if (!s.b[3329]) {
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

        s.b[3330] = (s.v[572] < 0.0);
        s.v[3330] = if s.b[3330] { 1.0 } else { 0.0 };

        if ((p.p29 != 0.0) && s.b[3330]) {
            s.store_scalar(572, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (p.p29 != 0.0) {
            s.store_voltage(817, ctx, nodes, Some(14), None);
            s.store_ad_value(352, A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(816), -1.0, s.ad_value(817), 1.0));
            s.copy_ad(355, 817);
        }

        if (p.p29 == 0.0) {
            s.copy_ad(817, 816);
        }

        s.b[3331] = (p.p22 > 0.0);
        s.v[3331] = if s.b[3331] { 1.0 } else { 0.0 };

        if s.b[3331] {
            s.store_ad_value(811, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0), s.v[365], s.ad_value(291), s.v[365], s.ad_value(292), s.v[365]));
            s.store_scaled_sub(810, 355, 292, s.v[365]);
            s.store_scaled_sub(812, 356, 291, s.v[365]);
            s.store_add_ad_rhs(700, 700, A::add_scaled_inputs3(s.ad_value(305), s.v[365], s.ad_value(360), ((-1.0) * s.v[365]), s.ad_value(362), (-s.v[365])));
            s.store_ad_value(701, A::add_scaled_inputs3(s.ad_value(701), 1.0, s.ad_value(361), s.v[365], s.ad_value(305), (-s.v[365])));
            s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);
            s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);
            s.store_scale(706, 358, s.v[365]);
            s.store_scale(707, 359, s.v[365]);
            s.store_offset_ad(703, A::sub_scaled_inputs(s.ad_value(299), (-s.v[365]), s.ad_value(298), s.v[365]), s.v[703]);
            s.store_offset_ad(704, A::sub_scaled_inputs(s.ad_value(301), (-s.v[365]), s.ad_value(297), s.v[365]), s.v[704]);
        }

        s.store_scaled_add(709, 280, 287, s.v[365]);

        s.store_scale(710, 281, s.v[365]);

        s.store_scale(807, 387, (4.0 * 1.3806226e-23));

        s.store_scale(712, 315, s.v[365]);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(6), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(8), None));

        s.store_scale(23, 23, p.p87);

        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }

        s.v[713] = 0.0;

        s.b[3334] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));
        s.v[3334] = if s.b[3334] { 1.0 } else { 0.0 };

        if s.b[3334] {
            s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));
            s.store_scale(344, 757, 1.0 / (s.v[365]));
            s.store_ad_value(328, A::div_scaled_product3(s.ad_value(155), s.ad_value(344), s.ad_value(344), (0.1185185185185185 * 1.6021918e-19), s.ad_value(324), 1.0));
        }

        s.b[3335] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));
        s.v[3335] = if s.b[3335] { 1.0 } else { 0.0 };

        if (s.b[3334] && s.b[3335]) {
            s.store_div(329, 254, 253);
            s.store_div_ad_lhs(330, A::sub(A::div(s.ad_value(254), s.ad_value(316)), s.ad_value(329)), 790);
            s.store_add_ad_rhs(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));
        }

        if (s.b[3334] && (!s.b[3335])) {
            s.store_div(331, 254, 316);
        }

        if s.b[3334] {
            s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);
        }

        if s.b[3334] {
            s.store_ad_value(713, {
                if (s.v[713] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(713)
                }
            });
        }

        if s.b[3334] {
            s.store_ad_value(713, {
                if ((-s.v[344]) > s.v[334]) {
                    s.ad_value(713)
                } else {
                    A::constant(0.0)
                }
            });
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

        s.b[3336] = (s.v[449] == 1.0);
        s.v[3336] = if s.b[3336] { 1.0 } else { 0.0 };

        s.b[3337] = (s.v[76] == 0.0);
        s.v[3337] = if s.b[3337] { 1.0 } else { 0.0 };

        s.b[3338] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3338] = if s.b[3338] { 1.0 } else { 0.0 };

        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {
            s.store_ad_value(335, {
                if (s.v[676] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(676), p.p416)
                }
            });
        }

        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_ad_value(334, A::sub_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), 1.0, A::sub_from_scalar(1.0, s.ad_value(676)), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_add_ad_rhs(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p439));
        }

        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3338])) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        if (s.b[3336] && (!s.b[3337])) {
            s.store_scalar(164, (s.v[630] * p.p7));
            s.store_scalar(604, p.p71);
            s.store_scalar(605, s.v[460]);
            s.store_mul(606, 794, 653);
            s.store_offset_ad(607, A::mul3(s.ad_value(795), s.ad_value(786), s.ad_value(652)), 1e-25);
            s.store_div(608, 804, 604);
            s.store_mul(609, 606, 608);
        }

        s.b[3339] = (s.v[804] >= 0.0);
        s.v[3339] = if s.b[3339] { 1.0 } else { 0.0 };

        if ((s.b[3336] && (!s.b[3337])) && s.b[3339]) {
            s.store_div(335, 609, 607);
        }

        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3339])) {
            s.store_scaled_div(335, 609, 607, -1.0);
        }

        s.b[3340] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3340] = if s.b[3340] { 1.0 } else { 0.0 };

        if ((s.b[3336] && (!s.b[3337])) && s.b[3340]) {
            s.store_scalar(337, 1.0);
        }

        s.b[3341] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3341] = if s.b[3341] { 1.0 } else { 0.0 };

        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && s.b[3341]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && (!s.b[3341])) {
            s.store_pow_ad(337, s.ad_value(335), A::offset(s.ad_value(959), (-1.0)));
        }

        if (s.b[3336] && (!s.b[3337])) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[3342] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3342] = if s.b[3342] { 1.0 } else { 0.0 };

        if ((s.b[3336] && (!s.b[3337])) && s.b[3342]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[3343] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3343] = if s.b[3343] { 1.0 } else { 0.0 };

        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && s.b[3343]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)))
                }
            });
        }

        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[3336] && (!s.b[3337])) {
            s.store_mul(610, 606, 339);
            s.copy_ad(611, 605);
            s.copy_ad(612, 614);
            s.store_div_from_scalar(335, 1.6021918e-19, 604);
            s.store_mul_ad_lhs(613, A::mul3(s.ad_value(335), s.ad_value(612), s.ad_value(610)), 611);
        }

        s.b[3344] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));
        s.v[3344] = if s.b[3344] { 1.0 } else { 0.0 };

        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {
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
        }

    }
}
