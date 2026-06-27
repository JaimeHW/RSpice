#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) {
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2752] = (p.p113 > 0.0);
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        s.b[2753] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && s.b[2753]) {
        }

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && (!s.b[2753])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && (!s.b[2753])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2754] = (s.v[336] < 0.0);
        s.v[2754] = if s.b[2754] { 1.0 } else { 0.0 };

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && s.b[2754]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2755] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2755] = if s.b[2755] { 1.0 } else { 0.0 };

        if ((s.b[2747] && (!((s.b[2744] || s.b[2745]) || s.b[2746]))) && s.b[2755]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2763, 0.4);
            s.store_scalar(2764, 0.0);
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
            s.store_scalar(2765, 0.0);
            s.store_scalar(2766, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2761, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2761), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2762, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2768] = (s.v[2763] > (s.v[2762] * 0.5));
        s.v[2768] = if s.b[2768] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2768]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2769] = param_given[338];
        s.v[2769] = if s.b[2769] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2769]) {
            s.store_scalar(2762, p.p338);
        }

        s.b[2770] = param_given[339];
        s.v[2770] = if s.b[2770] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2770]) {
            s.store_scalar(2763, p.p339);
        }

        s.b[2771] = param_given[338];
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2770])) && s.b[2771]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2772] = (s.v[2763] > (s.v[2762] * 0.5));
        s.v[2772] = if s.b[2772] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2772]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2773] = (p.p38 == 1.0);
        s.v[2773] = if s.b[2773] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2773]) {
            s.store_neg(334, 396);
        }

        s.b[2774] = (s.v[334] > s.v[2763]);
        s.v[2774] = if s.b[2774] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2773]) && s.b[2774]) {
            s.store_sub(335, 334, 2763);
            s.store_sub(336, 2762, 2763);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2763, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2773]) && (!s.b[2774])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2773]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2773])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2757, 0.0);
            s.store_scale(2758, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2775] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2775] = if s.b[2775] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2775]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2775])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign75970_loop_guard: usize = 0;
        while {
            let assign75970_cond_e115458: f64 = if (((s.v[2621] != 0.0) && (!s.b[2775])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign75970_cond_e115458 != 0.0
        } {
            assign75970_loop_guard += 1;
            assert!(assign75970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2775])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2775])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2776] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2776] = if s.b[2776] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
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

        s.b[2777] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2777] = if s.b[2777] { 1.0 } else { 0.0 };

        s.b[2778] = (1.0 == 1.0);
        s.v[2778] = if s.b[2778] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && s.b[2778]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2779] = (1.0 == 2.0);
        s.v[2779] = if s.b[2779] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && s.b[2779]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2780] = (1.0 == 4.0);
        s.v[2780] = if s.b[2780] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && (!s.b[2779])) && s.b[2780]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2781] = (1.0 == 8.0);
        s.v[2781] = if s.b[2781] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && (!s.b[2779])) && (!s.b[2780])) && s.b[2781]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign76300_loop_guard: usize = 0;
        while {
            let assign76300_cond_e115808: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign76300_cond_e115808 != 0.0
        } {
            assign76300_loop_guard += 1;
            assert!(assign76300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && (!s.b[2777])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

    }

    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2776])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2782] = (s.v[402] < s.v[403]);
        s.v[2782] = if s.b[2782] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2782]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2783] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2783] = if s.b[2783] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2782]) && s.b[2783]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2782]) && (!s.b[2783])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2782]) {
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
            s.copy_ad(2765, 404);
        }

        s.b[2784] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2784] = if s.b[2784] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2784]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2784])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2785] = (s.v[116] >= 3.0);
        s.v[2785] = if s.b[2785] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2785]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2785])) {
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

        s.b[2786] = (p.p33 > 0.0);
        s.v[2786] = if s.b[2786] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2787] = (p.p33 == 2.0);
        s.v[2787] = if s.b[2787] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && (!s.b[2787])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.copy_ad(445, 116);
        }

        s.b[2788] = (p.p33 == 2.0);
        s.v[2788] = if s.b[2788] { 1.0 } else { 0.0 };

        s.b[2789] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2789] = if s.b[2789] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
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

        s.b[2790] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2790] = if s.b[2790] { 1.0 } else { 0.0 };

        s.b[2791] = (2.0 == 1.0);
        s.v[2791] = if s.b[2791] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && s.b[2791]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2792] = (2.0 == 2.0);
        s.v[2792] = if s.b[2792] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && s.b[2792]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2793] = (2.0 == 4.0);
        s.v[2793] = if s.b[2793] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && (!s.b[2792])) && s.b[2793]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2794] = (2.0 == 8.0);
        s.v[2794] = if s.b[2794] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && (!s.b[2792])) && (!s.b[2793])) && s.b[2794]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign77430_loop_guard: usize = 0;
        while {
            let assign77430_cond_e117365: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign77430_cond_e117365 != 0.0
        } {
            assign77430_loop_guard += 1;
            assert!(assign77430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && (!s.b[2790])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && (!s.b[2789])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && (!s.b[2788])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2795] = (p.p33 == 1.0);
        s.v[2795] = if s.b[2795] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2796] = (s.v[411] > 0.0);
        s.v[2796] = if s.b[2796] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2796]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2797] = (s.v[336] < 0.0);
        s.v[2797] = if s.b[2797] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) && s.b[2797]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2798] = (s.v[336] < 0.0);
        s.v[2798] = if s.b[2798] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2798]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2799] = (s.v[333] < 60.0);
        s.v[2799] = if s.b[2799] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2799]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2799])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2800] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2800] = if s.b[2800] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2800]) {
            s.store_offset(2764, 2764, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2801] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2801] = if s.b[2801] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2801]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2801])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2802, 354, 2758);
        }

        s.b[2804] = (p.p33 == 2.0);
        s.v[2804] = if s.b[2804] { 1.0 } else { 0.0 };

        s.b[2805] = ((s.v[2802] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2805] = if s.b[2805] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_add_scaled_inputs3_indices(781, 2802, 1.0, 386, (-1.0), 386, 0.1);
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

        s.b[2806] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2806] = if s.b[2806] { 1.0 } else { 0.0 };

        s.b[2807] = (2.0 == 1.0);
        s.v[2807] = if s.b[2807] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && s.b[2807]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2808] = (2.0 == 2.0);
        s.v[2808] = if s.b[2808] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && s.b[2808]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2809] = (2.0 == 4.0);
        s.v[2809] = if s.b[2809] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && (!s.b[2808])) && s.b[2809]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2810] = (2.0 == 8.0);
        s.v[2810] = if s.b[2810] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && (!s.b[2808])) && (!s.b[2809])) && s.b[2810]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign78240_loop_guard: usize = 0;
        while {
            let assign78240_cond_e118435: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78240_cond_e118435 != 0.0
        } {
            assign78240_loop_guard += 1;
            assert!(assign78240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && (!s.b[2806])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && (!s.b[2805])) {
            s.copy_ad(335, 2802);
            s.store_scalar(334, 1.0);
        }

        s.b[2811] = (s.v[334] < 1.0);
        s.v[2811] = if s.b[2811] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2811]) {
            s.store_offset(2764, 2764, 2.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2804])) {
            if (s.v[2802] <= s.v[386]) {
                s.copy_ad(335, 2802);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2812] = (s.v[2802] >= s.v[386]);
        s.v[2812] = if s.b[2812] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2804])) && s.b[2812]) {
            s.store_offset(2764, 2764, 2.0);
        }

        s.b[2813] = (s.v[2764] >= 2.0);
        s.v[2813] = if s.b[2813] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) {
            s.copy_ad(2803, 404);
            s.store_mul(354, 335, 2758);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2814] = (p.p33 == 2.0);
        s.v[2814] = if s.b[2814] { 1.0 } else { 0.0 };

        s.b[2815] = ((s.v[404] > (s.v[2803] - 0.1)) && (0.1 >= 0.0));
        s.v[2815] = if s.b[2815] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
            s.store_offset_sub(781, 404, 2803, 0.1);
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

        s.b[2816] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2816] = if s.b[2816] { 1.0 } else { 0.0 };

        s.b[2817] = (2.0 == 1.0);
        s.v[2817] = if s.b[2817] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2818] = (2.0 == 2.0);
        s.v[2818] = if s.b[2818] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && s.b[2818]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2819] = (2.0 == 4.0);
        s.v[2819] = if s.b[2819] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && (!s.b[2818])) && s.b[2819]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2820] = (2.0 == 8.0);
        s.v[2820] = if s.b[2820] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && (!s.b[2818])) && (!s.b[2819])) && s.b[2820]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign78690_loop_guard: usize = 0;
        while {
            let assign78690_cond_e119043: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78690_cond_e119043 != 0.0
        } {
            assign78690_loop_guard += 1;
            assert!(assign78690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && (!s.b[2816])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2803, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && (!s.b[2815])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && (!s.b[2815])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && (!s.b[2814])) {
            if (s.v[404] <= s.v[2803]) {
            } else {
                s.copy_ad(404, 2803);
            }
        }

    }

    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.copy_ad(2765, 404);
        }

        s.b[2821] = (p.p33 == 1.0);
        s.v[2821] = if s.b[2821] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2758)), s.ad_value(155)), 2.0);
        }

        s.b[2822] = (s.v[411] > 0.0);
        s.v[2822] = if s.b[2822] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2822]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2823] = (s.v[336] < 0.0);
        s.v[2823] = if s.b[2823] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) && s.b[2823]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2824] = (s.v[336] < 0.0);
        s.v[2824] = if s.b[2824] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2824]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign79050_loop_guard: usize = 0;
        while {
            let assign79050_cond_e119567: f64 = (s.v[421] + 1.0);
            let assign79050_cond_e119569: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (s.v[97] <= assign79050_cond_e119567)) { 1.0 } else { 0.0 };
            assign79050_cond_e119569 != 0.0
        } {
            assign79050_loop_guard += 1;
            assert!(assign79050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2826] = (s.v[333] < 60.0);
            s.v[2826] = if s.b[2826] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2826]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2826])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2827] = (s.v[116] < 0.0);
            s.v[2827] = if s.b[2827] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2827]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2828] = (s.v[116] < 1e-6);
            s.v[2828] = if s.b[2828] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2829] = (s.v[338] > 0.0);
            s.v[2829] = if s.b[2829] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && s.b[2829]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && (!s.b[2829])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2830] = (s.v[338] > 0.0);
            s.v[2830] = if s.b[2830] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && s.b[2830]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && (!s.b[2830])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2831] = (s.v[116] < 0.0);
            s.v[2831] = if s.b[2831] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2831]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2832] = (s.v[116] < 60.0);
            s.v[2832] = if s.b[2832] { 1.0 } else { 0.0 };
            s.b[2833] = (s.v[116] < 5e-5);
            s.v[2833] = if s.b[2833] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && s.b[2833]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && (!s.b[2833])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2832])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2834] = (s.v[214] > 0.0);
            s.v[2834] = if s.b[2834] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2834]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2834])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2835] = (s.v[79] == 1.0);
            s.v[2835] = if s.b[2835] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2835]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2836] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2836] = if s.b[2836] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2836]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_add(404, 404, 236);
            }
            s.b[2837] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2837] = if s.b[2837] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2837]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_mul(2756, 982, 223);
            s.store_mul(2757, 2758, 2756);
            s.store_offset_div(100, 2757, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2839] = (p.p33 == 4.0);
        s.v[2839] = if s.b[2839] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2765);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2758)), s.ad_value(155)), 2.0);
        }

        s.b[2840] = (s.v[411] > 0.0);
        s.v[2840] = if s.b[2840] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2840]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2841] = (s.v[336] < 0.0);
        s.v[2841] = if s.b[2841] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) && s.b[2841]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2842] = (s.v[336] < 0.0);
        s.v[2842] = if s.b[2842] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2842]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign79440_loop_guard: usize = 0;
        while {
            let assign79440_cond_e121222: f64 = (s.v[421] + 1.0);
            let assign79440_cond_e121224: f64 = if (((s.v[2621] != 0.0) && s.b[2839]) && (s.v[97] <= assign79440_cond_e121222)) { 1.0 } else { 0.0 };
            assign79440_cond_e121224 != 0.0
        } {
            assign79440_loop_guard += 1;
            assert!(assign79440_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2844] = (s.v[333] < 60.0);
            s.v[2844] = if s.b[2844] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2844]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2844])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2845] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2845] = if s.b[2845] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2845]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2766, 334, 336);
                s.store_mul_add_scaled_product_rhs(2767, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2845])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2766, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2767, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2846] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2846] = if s.b[2846] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2846]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2847] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2847] = if s.b[2847] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2846])) && s.b[2847]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2846])) && (!s.b[2847])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2848] = (s.v[214] > 0.0);
            s.v[2848] = if s.b[2848] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2848]) {
                s.store_sqrt_add(216, 2766, 214);
                s.store_div_scaled_inputs2_indices(217, 2767, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2849] = (s.v[2766] > 0.0);
            s.v[2849] = if s.b[2849] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2848])) && s.b[2849]) {
                s.store_sqrt(216, 2766);
                s.store_div_scaled_inputs_indices(217, 2767, 0.5, 216, 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2848])) && (!s.b[2849])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2850] = (s.v[79] > 0.0);
            s.v[2850] = if s.b[2850] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2850]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2851] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2851] = if s.b[2851] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) && s.b[2851]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_add(404, 404, 236);
            }
            s.b[2852] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2852] = if s.b[2852] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) && s.b[2852]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            if (s.v[2766] >= 0.0) {
                s.store_scaled_sqrt(223, 2766, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_mul(2756, 982, 223);
            s.store_mul(2757, 2758, 2756);
            s.store_offset_div(100, 2757, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2854] = (s.v[407] < 0.0);
        s.v[2854] = if s.b[2854] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2854]) {
            s.store_neg(407, 407);
        }

        s.b[2855] = (p.p55 == 0.0);
        s.v[2855] = if s.b[2855] { 1.0 } else { 0.0 };

        s.b[2856] = (p.p50 == 0.0);
        s.v[2856] = if s.b[2856] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && s.b[2856]) {
            s.store_neg(2759, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && (!s.b[2856])) {
            s.copy_ad(2759, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2759), p.p137, A::offset(s.ad_value(2759), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2759), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2759), p.p137), 782, 0.5);
        }

        s.b[2857] = (s.v[336] < 0.0);
        s.v[2857] = if s.b[2857] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && s.b[2857]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2858] = (2.0 == 1.0);
        s.v[2858] = if s.b[2858] { 1.0 } else { 0.0 };

        s.b[2859] = (2.0 == 2.0);
        s.v[2859] = if s.b[2859] { 1.0 } else { 0.0 };

        s.b[2860] = (2.0 == 3.0);
        s.v[2860] = if s.b[2860] { 1.0 } else { 0.0 };

        s.b[2861] = (2.0 == 4.0);
        s.v[2861] = if s.b[2861] { 1.0 } else { 0.0 };

        s.b[2862] = (p.p55 == 1.0);
        s.v[2862] = if s.b[2862] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2858]) && s.b[2862]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && s.b[2858]) && (!s.b[2862])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2858]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2859] && (!s.b[2858]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2863] = (p.p55 == 1.0);
        s.v[2863] = if s.b[2863] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && s.b[2863]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && (!s.b[2863])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) {
            s.copy_ad(697, 404);
        }

        s.b[2864] = (p.p430 == 0.0);
        s.v[2864] = if s.b[2864] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && s.b[2864]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2861] && (!((s.b[2858] || s.b[2859]) || s.b[2860])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2621] = 0.0;

        s.b[2865] = (3.0 == 1.0);
        s.v[2865] = if s.b[2865] { 1.0 } else { 0.0 };

        s.b[2866] = (3.0 == 2.0);
        s.v[2866] = if s.b[2866] { 1.0 } else { 0.0 };

        s.b[2867] = (3.0 == 3.0);
        s.v[2867] = if s.b[2867] { 1.0 } else { 0.0 };

        s.b[2868] = (3.0 == 4.0);
        s.v[2868] = if s.b[2868] { 1.0 } else { 0.0 };

        s.b[2869] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2869] = if s.b[2869] { 1.0 } else { 0.0 };

        if (s.b[2865] && s.b[2869]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2619, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2870] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2870] = if s.b[2870] { 1.0 } else { 0.0 };

        if ((s.b[2866] && (!s.b[2865])) && s.b[2870]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2871] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2871] = if s.b[2871] { 1.0 } else { 0.0 };

        if ((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2622, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2872] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2872] = if s.b[2872] { 1.0 } else { 0.0 };

        if (((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) {
            s.store_neg(407, 407);
        }

    }

    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) {
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2873] = (p.p113 > 0.0);
        s.v[2873] = if s.b[2873] { 1.0 } else { 0.0 };

        s.b[2874] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2874] = if s.b[2874] { 1.0 } else { 0.0 };

        if (((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) && s.b[2874]) {
        }

        if (((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) && (!s.b[2874])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) && (!s.b[2874])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2875] = (s.v[336] < 0.0);
        s.v[2875] = if s.b[2875] { 1.0 } else { 0.0 };

        if (((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) && s.b[2875]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2867] && (!(s.b[2865] || s.b[2866]))) && s.b[2871]) && s.b[2872]) && s.b[2873]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2876] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2876] = if s.b[2876] { 1.0 } else { 0.0 };

        if ((s.b[2868] && (!((s.b[2865] || s.b[2866]) || s.b[2867]))) && s.b[2876]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2884, 0.4);
            s.store_scalar(2885, 0.0);
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
            s.store_scalar(2886, 0.0);
            s.store_scalar(2887, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2882, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2882), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2883, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2889] = (s.v[2884] > (s.v[2883] * 0.5));
        s.v[2889] = if s.b[2889] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2889]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2890] = param_given[338];
        s.v[2890] = if s.b[2890] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2890]) {
            s.store_scalar(2883, p.p338);
        }

        s.b[2891] = param_given[339];
        s.v[2891] = if s.b[2891] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2891]) {
            s.store_scalar(2884, p.p339);
        }

        s.b[2892] = param_given[338];
        s.v[2892] = if s.b[2892] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2891])) && s.b[2892]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2893] = (s.v[2884] > (s.v[2883] * 0.5));
        s.v[2893] = if s.b[2893] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2893]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2894] = (p.p38 == 1.0);
        s.v[2894] = if s.b[2894] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2894]) {
            s.store_neg(334, 396);
        }

        s.b[2895] = (s.v[334] > s.v[2884]);
        s.v[2895] = if s.b[2895] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2894]) && s.b[2895]) {
            s.store_sub(335, 334, 2884);
            s.store_sub(336, 2883, 2884);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2884, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2894]) && (!s.b[2895])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2894]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2894])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2878, 0.0);
            s.store_scale(2879, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2896] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2896] = if s.b[2896] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2896]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2896])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign81210_loop_guard: usize = 0;
        while {
            let assign81210_cond_e123769: f64 = if (((s.v[2621] != 0.0) && (!s.b[2896])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign81210_cond_e123769 != 0.0
        } {
            assign81210_loop_guard += 1;
            assert!(assign81210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2896])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2896])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2897] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2897] = if s.b[2897] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
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

        s.b[2898] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2898] = if s.b[2898] { 1.0 } else { 0.0 };

        s.b[2899] = (1.0 == 1.0);
        s.v[2899] = if s.b[2899] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && s.b[2899]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2900] = (1.0 == 2.0);
        s.v[2900] = if s.b[2900] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && s.b[2900]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2901] = (1.0 == 4.0);
        s.v[2901] = if s.b[2901] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && (!s.b[2900])) && s.b[2901]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2902] = (1.0 == 8.0);
        s.v[2902] = if s.b[2902] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && (!s.b[2900])) && (!s.b[2901])) && s.b[2902]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign81540_loop_guard: usize = 0;
        while {
            let assign81540_cond_e124119: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign81540_cond_e124119 != 0.0
        } {
            assign81540_loop_guard += 1;
            assert!(assign81540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && (!s.b[2898])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

    }

    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2897])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2903] = (s.v[402] < s.v[403]);
        s.v[2903] = if s.b[2903] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2903]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2904] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2904] = if s.b[2904] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2903]) && s.b[2904]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2903]) && (!s.b[2904])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2903]) {
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
            s.copy_ad(2886, 404);
        }

        s.b[2905] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2905] = if s.b[2905] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2905]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2905])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2906] = (s.v[116] >= 3.0);
        s.v[2906] = if s.b[2906] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2906]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2906])) {
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

        s.b[2907] = (p.p33 > 0.0);
        s.v[2907] = if s.b[2907] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2908] = (p.p33 == 2.0);
        s.v[2908] = if s.b[2908] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && (!s.b[2908])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.copy_ad(445, 116);
        }

        s.b[2909] = (p.p33 == 2.0);
        s.v[2909] = if s.b[2909] { 1.0 } else { 0.0 };

        s.b[2910] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2910] = if s.b[2910] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
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

        s.b[2911] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2911] = if s.b[2911] { 1.0 } else { 0.0 };

        s.b[2912] = (2.0 == 1.0);
        s.v[2912] = if s.b[2912] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && s.b[2912]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2913] = (2.0 == 2.0);
        s.v[2913] = if s.b[2913] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && s.b[2913]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2914] = (2.0 == 4.0);
        s.v[2914] = if s.b[2914] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && (!s.b[2913])) && s.b[2914]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2915] = (2.0 == 8.0);
        s.v[2915] = if s.b[2915] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && (!s.b[2913])) && (!s.b[2914])) && s.b[2915]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign82670_loop_guard: usize = 0;
        while {
            let assign82670_cond_e125676: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign82670_cond_e125676 != 0.0
        } {
            assign82670_loop_guard += 1;
            assert!(assign82670_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && (!s.b[2911])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && (!s.b[2910])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && (!s.b[2909])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2916] = (p.p33 == 1.0);
        s.v[2916] = if s.b[2916] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2917] = (s.v[411] > 0.0);
        s.v[2917] = if s.b[2917] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2917]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2918] = (s.v[336] < 0.0);
        s.v[2918] = if s.b[2918] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) && s.b[2918]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2919] = (s.v[336] < 0.0);
        s.v[2919] = if s.b[2919] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2919]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2920] = (s.v[333] < 60.0);
        s.v[2920] = if s.b[2920] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2920]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2920])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2921] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2921] = if s.b[2921] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2921]) {
            s.store_offset(2885, 2885, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2922] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2922] = if s.b[2922] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2922]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2922])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2923, 354, 2879);
        }

        s.b[2925] = (p.p33 == 2.0);
        s.v[2925] = if s.b[2925] { 1.0 } else { 0.0 };

        s.b[2926] = ((s.v[2923] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2926] = if s.b[2926] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
            s.store_add_scaled_inputs3_indices(781, 2923, 1.0, 386, (-1.0), 386, 0.1);
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

        s.b[2927] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2927] = if s.b[2927] { 1.0 } else { 0.0 };

        s.b[2928] = (2.0 == 1.0);
        s.v[2928] = if s.b[2928] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && s.b[2928]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2929] = (2.0 == 2.0);
        s.v[2929] = if s.b[2929] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && s.b[2929]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2930] = (2.0 == 4.0);
        s.v[2930] = if s.b[2930] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && (!s.b[2929])) && s.b[2930]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2931] = (2.0 == 8.0);
        s.v[2931] = if s.b[2931] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && (!s.b[2929])) && (!s.b[2930])) && s.b[2931]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign83480_loop_guard: usize = 0;
        while {
            let assign83480_cond_e126746: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83480_cond_e126746 != 0.0
        } {
            assign83480_loop_guard += 1;
            assert!(assign83480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && (!s.b[2927])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && (!s.b[2926])) {
            s.copy_ad(335, 2923);
            s.store_scalar(334, 1.0);
        }

        s.b[2932] = (s.v[334] < 1.0);
        s.v[2932] = if s.b[2932] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2932]) {
            s.store_offset(2885, 2885, 2.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2925])) {
            if (s.v[2923] <= s.v[386]) {
                s.copy_ad(335, 2923);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2933] = (s.v[2923] >= s.v[386]);
        s.v[2933] = if s.b[2933] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2925])) && s.b[2933]) {
            s.store_offset(2885, 2885, 2.0);
        }

        s.b[2934] = (s.v[2885] >= 2.0);
        s.v[2934] = if s.b[2934] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) {
            s.copy_ad(2924, 404);
            s.store_mul(354, 335, 2879);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2935] = (p.p33 == 2.0);
        s.v[2935] = if s.b[2935] { 1.0 } else { 0.0 };

        s.b[2936] = ((s.v[404] > (s.v[2924] - 0.1)) && (0.1 >= 0.0));
        s.v[2936] = if s.b[2936] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
            s.store_offset_sub(781, 404, 2924, 0.1);
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

        s.b[2937] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2937] = if s.b[2937] { 1.0 } else { 0.0 };

        s.b[2938] = (2.0 == 1.0);
        s.v[2938] = if s.b[2938] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2939] = (2.0 == 2.0);
        s.v[2939] = if s.b[2939] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && s.b[2939]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2940] = (2.0 == 4.0);
        s.v[2940] = if s.b[2940] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && (!s.b[2939])) && s.b[2940]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2941] = (2.0 == 8.0);
        s.v[2941] = if s.b[2941] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && (!s.b[2939])) && (!s.b[2940])) && s.b[2941]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign83930_loop_guard: usize = 0;
        while {
            let assign83930_cond_e127354: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83930_cond_e127354 != 0.0
        } {
            assign83930_loop_guard += 1;
            assert!(assign83930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && (!s.b[2937])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2924, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && (!s.b[2936])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && (!s.b[2936])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && (!s.b[2935])) {
            if (s.v[404] <= s.v[2924]) {
            } else {
                s.copy_ad(404, 2924);
            }
        }

    }

    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.copy_ad(2886, 404);
        }

        s.b[2942] = (p.p33 == 1.0);
        s.v[2942] = if s.b[2942] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2879)), s.ad_value(155)), 2.0);
        }

        s.b[2943] = (s.v[411] > 0.0);
        s.v[2943] = if s.b[2943] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2943]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2944] = (s.v[336] < 0.0);
        s.v[2944] = if s.b[2944] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) && s.b[2944]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2945] = (s.v[336] < 0.0);
        s.v[2945] = if s.b[2945] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2945]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign84290_loop_guard: usize = 0;
        while {
            let assign84290_cond_e127878: f64 = (s.v[421] + 1.0);
            let assign84290_cond_e127880: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (s.v[97] <= assign84290_cond_e127878)) { 1.0 } else { 0.0 };
            assign84290_cond_e127880 != 0.0
        } {
            assign84290_loop_guard += 1;
            assert!(assign84290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2947] = (s.v[333] < 60.0);
            s.v[2947] = if s.b[2947] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2947]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2947])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2948] = (s.v[116] < 0.0);
            s.v[2948] = if s.b[2948] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2948]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2949] = (s.v[116] < 1e-6);
            s.v[2949] = if s.b[2949] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2950] = (s.v[338] > 0.0);
            s.v[2950] = if s.b[2950] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) && s.b[2950]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) && (!s.b[2950])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2951] = (s.v[338] > 0.0);
            s.v[2951] = if s.b[2951] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) && s.b[2951]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) && (!s.b[2951])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2952] = (s.v[116] < 0.0);
            s.v[2952] = if s.b[2952] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2952]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2953] = (s.v[116] < 60.0);
            s.v[2953] = if s.b[2953] { 1.0 } else { 0.0 };
            s.b[2954] = (s.v[116] < 5e-5);
            s.v[2954] = if s.b[2954] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2953]) && s.b[2954]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2953]) && (!s.b[2954])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && (!s.b[2953])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2955] = (s.v[214] > 0.0);
            s.v[2955] = if s.b[2955] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2955]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && (!s.b[2955])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2956] = (s.v[79] == 1.0);
            s.v[2956] = if s.b[2956] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2956]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2957] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2957] = if s.b[2957] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) && s.b[2957]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_add(404, 404, 236);
            }
            s.b[2958] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2958] = if s.b[2958] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) && s.b[2958]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_mul(2877, 982, 223);
            s.store_mul(2878, 2879, 2877);
            s.store_offset_div(100, 2878, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2960] = (p.p33 == 4.0);
        s.v[2960] = if s.b[2960] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2886);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2879)), s.ad_value(155)), 2.0);
        }

        s.b[2961] = (s.v[411] > 0.0);
        s.v[2961] = if s.b[2961] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2961]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2962] = (s.v[336] < 0.0);
        s.v[2962] = if s.b[2962] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) && s.b[2962]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2963] = (s.v[336] < 0.0);
        s.v[2963] = if s.b[2963] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2963]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign84680_loop_guard: usize = 0;
        while {
            let assign84680_cond_e129533: f64 = (s.v[421] + 1.0);
            let assign84680_cond_e129535: f64 = if (((s.v[2621] != 0.0) && s.b[2960]) && (s.v[97] <= assign84680_cond_e129533)) { 1.0 } else { 0.0 };
            assign84680_cond_e129535 != 0.0
        } {
            assign84680_loop_guard += 1;
            assert!(assign84680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2965] = (s.v[333] < 60.0);
            s.v[2965] = if s.b[2965] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2965]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2965])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2966] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2966] = if s.b[2966] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2966]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2887, 334, 336);
                s.store_mul_add_scaled_product_rhs(2888, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2966])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2887, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2888, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2967] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2967] = if s.b[2967] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2967]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2968] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2968] = if s.b[2968] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2967])) && s.b[2968]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2967])) && (!s.b[2968])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2969] = (s.v[214] > 0.0);
            s.v[2969] = if s.b[2969] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2969]) {
                s.store_sqrt_add(216, 2887, 214);
                s.store_div_scaled_inputs2_indices(217, 2888, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2970] = (s.v[2887] > 0.0);
            s.v[2970] = if s.b[2970] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2969])) && s.b[2970]) {
                s.store_sqrt(216, 2887);
                s.store_div_scaled_inputs_indices(217, 2888, 0.5, 216, 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2969])) && (!s.b[2970])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2971] = (s.v[79] > 0.0);
            s.v[2971] = if s.b[2971] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2971]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2972] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) && s.b[2972]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_add(404, 404, 236);
            }
            s.b[2973] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) && s.b[2973]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            if (s.v[2887] >= 0.0) {
                s.store_scaled_sqrt(223, 2887, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_mul(2877, 982, 223);
            s.store_mul(2878, 2879, 2877);
            s.store_offset_div(100, 2878, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2975] = (s.v[407] < 0.0);
        s.v[2975] = if s.b[2975] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2975]) {
            s.store_neg(407, 407);
        }

        s.b[2976] = (p.p55 == 0.0);
        s.v[2976] = if s.b[2976] { 1.0 } else { 0.0 };

        s.b[2977] = (p.p50 == 0.0);
        s.v[2977] = if s.b[2977] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && s.b[2977]) {
            s.store_neg(2880, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && (!s.b[2977])) {
            s.copy_ad(2880, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2880), p.p137, A::offset(s.ad_value(2880), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2880), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2880), p.p137), 782, 0.5);
        }

        s.b[2978] = (s.v[336] < 0.0);
        s.v[2978] = if s.b[2978] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && s.b[2978]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2979] = (3.0 == 1.0);
        s.v[2979] = if s.b[2979] { 1.0 } else { 0.0 };

        s.b[2980] = (3.0 == 2.0);
        s.v[2980] = if s.b[2980] { 1.0 } else { 0.0 };

        s.b[2981] = (3.0 == 3.0);
        s.v[2981] = if s.b[2981] { 1.0 } else { 0.0 };

        s.b[2982] = (3.0 == 4.0);
        s.v[2982] = if s.b[2982] { 1.0 } else { 0.0 };

        s.b[2983] = (p.p55 == 1.0);
        s.v[2983] = if s.b[2983] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2979]) && s.b[2983]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && s.b[2979]) && (!s.b[2983])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2979]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2980] && (!s.b[2979]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2984] = (p.p55 == 1.0);
        s.v[2984] = if s.b[2984] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && s.b[2984]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && (!s.b[2984])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) {
            s.copy_ad(697, 404);
        }

        s.b[2985] = (p.p430 == 0.0);
        s.v[2985] = if s.b[2985] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && s.b[2985]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2982] && (!((s.b[2979] || s.b[2980]) || s.b[2981])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2621] = 0.0;

        s.b[2986] = (4.0 == 1.0);
        s.v[2986] = if s.b[2986] { 1.0 } else { 0.0 };

        s.b[2987] = (4.0 == 2.0);
        s.v[2987] = if s.b[2987] { 1.0 } else { 0.0 };

        s.b[2988] = (4.0 == 3.0);
        s.v[2988] = if s.b[2988] { 1.0 } else { 0.0 };

        s.b[2989] = (4.0 == 4.0);
        s.v[2989] = if s.b[2989] { 1.0 } else { 0.0 };

        s.b[2990] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2990] = if s.b[2990] { 1.0 } else { 0.0 };

        if (s.b[2986] && s.b[2990]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2619, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2991] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2991] = if s.b[2991] { 1.0 } else { 0.0 };

        if ((s.b[2987] && (!s.b[2986])) && s.b[2991]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2992] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2992] = if s.b[2992] { 1.0 } else { 0.0 };

        if ((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2622, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2993] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2993] = if s.b[2993] { 1.0 } else { 0.0 };

        if (((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) {
            s.store_neg(407, 407);
        }

    }

    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) {
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2994] = (p.p113 > 0.0);
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        s.b[2995] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2995] = if s.b[2995] { 1.0 } else { 0.0 };

        if (((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) && s.b[2995]) {
        }

        if (((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) && (!s.b[2995])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) && (!s.b[2995])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2996] = (s.v[336] < 0.0);
        s.v[2996] = if s.b[2996] { 1.0 } else { 0.0 };

        if (((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) && s.b[2996]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2988] && (!(s.b[2986] || s.b[2987]))) && s.b[2992]) && s.b[2993]) && s.b[2994]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2997] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2997] = if s.b[2997] { 1.0 } else { 0.0 };

        if ((s.b[2989] && (!((s.b[2986] || s.b[2987]) || s.b[2988]))) && s.b[2997]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(3005, 0.4);
            s.store_scalar(3006, 0.0);
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
            s.store_scalar(3007, 0.0);
            s.store_scalar(3008, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3003, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3003), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3004, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3010] = (s.v[3005] > (s.v[3004] * 0.5));
        s.v[3010] = if s.b[3010] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3010]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3011] = param_given[338];
        s.v[3011] = if s.b[3011] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3011]) {
            s.store_scalar(3004, p.p338);
        }

        s.b[3012] = param_given[339];
        s.v[3012] = if s.b[3012] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3012]) {
            s.store_scalar(3005, p.p339);
        }

        s.b[3013] = param_given[338];
        s.v[3013] = if s.b[3013] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3012])) && s.b[3013]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3014] = (s.v[3005] > (s.v[3004] * 0.5));
        s.v[3014] = if s.b[3014] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3014]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3015] = (p.p38 == 1.0);
        s.v[3015] = if s.b[3015] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3015]) {
            s.store_neg(334, 396);
        }

        s.b[3016] = (s.v[334] > s.v[3005]);
        s.v[3016] = if s.b[3016] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3015]) && s.b[3016]) {
            s.store_sub(335, 334, 3005);
            s.store_sub(336, 3004, 3005);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3005, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[3015]) && (!s.b[3016])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[3015]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3015])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2999, 0.0);
            s.store_scale(3000, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3017] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[3017] = if s.b[3017] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[3017]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3017])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign86450_loop_guard: usize = 0;
        while {
            let assign86450_cond_e132080: f64 = if (((s.v[2621] != 0.0) && (!s.b[3017])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign86450_cond_e132080 != 0.0
        } {
            assign86450_loop_guard += 1;
            assert!(assign86450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[3017])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[3017])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3018] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3018] = if s.b[3018] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
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

        s.b[3019] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3019] = if s.b[3019] { 1.0 } else { 0.0 };

        s.b[3020] = (1.0 == 1.0);
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && s.b[3020]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3021] = (1.0 == 2.0);
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && s.b[3021]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3022] = (1.0 == 4.0);
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && (!s.b[3021])) && s.b[3022]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3023] = (1.0 == 8.0);
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && (!s.b[3021])) && (!s.b[3022])) && s.b[3023]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign86780_loop_guard: usize = 0;
        while {
            let assign86780_cond_e132430: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86780_cond_e132430 != 0.0
        } {
            assign86780_loop_guard += 1;
            assert!(assign86780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && (!s.b[3019])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

    }

    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
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
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3025] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[3024]) && s.b[3025]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
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
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3027] = (s.v[116] >= 3.0);
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3027]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3027])) {
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
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
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
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
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

    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
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
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
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
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3043])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
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
            s.store_add_scaled_inputs3_indices(781, 3044, 1.0, 386, (-1.0), 386, 0.1);
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
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
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
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
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
            s.store_add_offset_lhs(404, 3045, (-0.1), 780);
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

    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
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
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
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
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
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
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
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

    pub(super) fn stamp_reactive_block_83(
        s: &mut ReactiveScratch,
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
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
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
                s.store_add_scaled_inputs4_indices(3008, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
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
                s.store_div_scaled_inputs2_indices(217, 3009, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[3091] = (s.v[3008] > 0.0);
            s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3090])) && s.b[3091]) {
                s.store_sqrt(216, 3008);
                s.store_div_scaled_inputs_indices(217, 3009, 0.5, 216, 1.0);
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
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
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
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
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
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
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
            s.store_mul_scaled_ln_ad_rhs(3114, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3114), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

    }

    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
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
            s.store_offset_add_scaled_inputs_indices(3115, 781, (-0.5), 782, (-0.5), 0.8);
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
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3129] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
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
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3136] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && s.b[3136]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
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
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3138] = (s.v[116] >= 3.0);
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

    }
}
