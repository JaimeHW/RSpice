#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[2625] && s.b[2629]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2630] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if ((s.b[2626] && (!s.b[2625])) && s.b[2630]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2631] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2624, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2632] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if (((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_ad(996, A::div_scaled_product(s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0), (-p.p137));
        }

        s.b[2633] = (p.p113 > 0.0);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        s.b[2634] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2634]) {
        }

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_ad_value(396, A::div_scaled_product(s.ad_value(783), s.ad_value(784), 1.0, s.ad_value(782), 1.0));
        }

        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(396), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2635] = (s.v[336] < 0.0);
        s.v[2635] = if s.b[2635] { 1.0 } else { 0.0 };

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2635]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2636] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2636] = if s.b[2636] { 1.0 } else { 0.0 };

        if ((s.b[2628] && (!((s.b[2625] || s.b[2626]) || s.b[2627]))) && s.b[2636]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2644, 0.4);
            s.store_scalar(2645, 0.0);
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
            s.store_scalar(2646, 0.0);
            s.store_scalar(2647, 0.0);
            s.store_mul_scaled_ad_rhs(2642, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2642), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(2643, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[2649] = (s.v[2644] > (s.v[2643] * 0.5));
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2649]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2650] = param_given[338];
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2650]) {
            s.store_scalar(2643, p.p338);
        }

        s.b[2651] = param_given[339];
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2651]) {
            s.store_scalar(2644, p.p339);
        }

        s.b[2652] = param_given[338];
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2651])) && s.b[2652]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2653] = (s.v[2644] > (s.v[2643] * 0.5));
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2653]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2654] = (p.p38 == 1.0);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2654]) {
            s.store_neg(334, 396);
        }

        s.b[2655] = (s.v[334] > s.v[2644]);
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2654]) && s.b[2655]) {
            s.store_sub(335, 334, 2644);
            s.store_sub(336, 2643, 2644);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_ad_value(334, A::add_scaled_sub_value_product(1.0, s.ad_value(780), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
            s.store_neg(345, 345);
            s.store_add(344, 2644, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[2654]) && (!s.b[2655])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[2654]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2654])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2638, 0.0);
            s.store_scale(2639, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2656] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2656]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2656])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign70750_loop_guard: usize = 0;
        while {
            let assign70750_cond_e107160: f64 = if (((s.v[2623] != 0.0) && (!s.b[2656])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign70750_cond_e107160 != 0.0
        } {
            assign70750_loop_guard += 1;
            assert!(assign70750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2656])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2656])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[2657] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(402), 1.0, s.ad_value(397), 1.0, s.ad_value(335), 1.0));
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2658] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        s.b[2659] = (1.0 == 1.0);
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && s.b[2659]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2660] = (1.0 == 2.0);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && s.b[2660]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2661] = (1.0 == 4.0);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && s.b[2661]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2662] = (1.0 == 8.0);
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && (!s.b[2661])) && s.b[2662]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign71080_loop_guard: usize = 0;
        while {
            let assign71080_cond_e107510: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign71080_cond_e107510 != 0.0
        } {
            assign71080_loop_guard += 1;
            assert!(assign71080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && (!s.b[2658])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2657])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2663] = (s.v[402] < s.v[403]);
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2663]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[2664] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2664] = if s.b[2664] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2663]) && s.b[2664]) {
            s.store_scaled_div(274, 277, 278, 0.5);
        }

        if (((s.v[2623] != 0.0) && s.b[2663]) && (!s.b[2664])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2663]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_ad_value(272, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, s.ad_value(273), 2.0, s.ad_value(273), s.ad_value(273), 1.414213562373095));
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2646, 404);
        }

        s.b[2665] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2665]) {
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(402), 1.0, s.ad_value(213), s.ad_value(154), 0.5));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2665])) {
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2666] = (s.v[116] >= 3.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2666]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2666])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_ad_value(437, A::div_scaled_inputs(A::add(s.ad_value(402), s.ad_value(397)), -1.0, s.ad_value(212), 1.0));
            s.store_ad_value(441, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0));
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0));
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2667] = (p.p33 > 0.0);
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_ad_value(447, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), s.ad_value(154), s.ad_value(397), 1.0));
        }

        s.b[2668] = (p.p33 == 2.0);
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_ad_value(447, A::add_scaled_inputs3(s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2668])) {
            s.store_ad_value(447, {
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_ad_value(447, {
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_ad_value(446, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_ad_value(446, {
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.copy_ad(445, 116);
        }

        s.b[2669] = (p.p33 == 2.0);
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        s.b[2670] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
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

        s.b[2671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        s.b[2672] = (2.0 == 1.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && s.b[2672]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2673] = (2.0 == 2.0);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && s.b[2673]) {
            s.store_scalar(720, 2.0);
        }

    }

    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2674] = (2.0 == 4.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && s.b[2674]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2675] = (2.0 == 8.0);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && (!s.b[2674])) && s.b[2675]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign72210_loop_guard: usize = 0;
        while {
            let assign72210_cond_e109067: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign72210_cond_e109067 != 0.0
        } {
            assign72210_loop_guard += 1;
            assert!(assign72210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && (!s.b[2671])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_ad_value(335, A::div_scaled_product3(s.ad_value(446), s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0));
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && (!s.b[2670])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2669])) {
            s.store_ad_value(116, {
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.b[2676] = (p.p33 == 1.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2677] = (s.v[411] > 0.0);
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2677]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2678] = (s.v[336] < 0.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) && s.b[2678]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2679] = (s.v[336] < 0.0);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2679]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2680] = (s.v[333] < 60.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2680]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2680])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2681] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2681]) {
            s.store_offset(2645, 2645, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2682] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2682]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2682])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2683, 354, 2639);
        }

        s.b[2685] = (p.p33 == 2.0);
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        s.b[2686] = ((s.v[2683] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(2683), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1));
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

        s.b[2687] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        s.b[2688] = (2.0 == 1.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && s.b[2688]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2689] = (2.0 == 2.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && s.b[2689]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2690] = (2.0 == 4.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && s.b[2690]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2691] = (2.0 == 8.0);
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && (!s.b[2690])) && s.b[2691]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign73020_loop_guard: usize = 0;
        while {
            let assign73020_cond_e110137: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73020_cond_e110137 != 0.0
        } {
            assign73020_loop_guard += 1;
            assert!(assign73020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(386), s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && (!s.b[2686])) {
            s.copy_ad(335, 2683);
            s.store_scalar(334, 1.0);
        }

        s.b[2692] = (s.v[334] < 1.0);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2692]) {
            s.store_offset(2645, 2645, 2.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) {
            s.store_ad_value(335, {
                if (s.v[2683] <= s.v[386]) {
                    s.ad_value(2683)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.b[2693] = (s.v[2683] >= s.v[386]);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) && s.b[2693]) {
            s.store_offset(2645, 2645, 2.0);
        }

        s.b[2694] = (s.v[2645] >= 2.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) {
            s.copy_ad(2684, 404);
            s.store_mul(354, 335, 2639);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.b[2695] = (p.p33 == 2.0);
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        s.b[2696] = ((s.v[404] > (s.v[2684] - 0.1)) && (0.1 >= 0.0));
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_offset_sub(781, 404, 2684, 0.1);
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

        s.b[2697] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        s.b[2698] = (2.0 == 1.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2699] = (2.0 == 2.0);
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && s.b[2699]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2700] = (2.0 == 4.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && s.b[2700]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2701] = (2.0 == 8.0);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && (!s.b[2700])) && s.b[2701]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign73470_loop_guard: usize = 0;
        while {
            let assign73470_cond_e110745: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73470_cond_e110745 != 0.0
        } {
            assign73470_loop_guard += 1;
            assert!(assign73470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && (!s.b[2697])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_add_ad_lhs(404, A::offset(s.ad_value(2684), (-0.1)), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && (!s.b[2695])) {
            s.store_ad_value(404, {
                if (s.v[404] <= s.v[2684]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(2684)
                }
            });
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.copy_ad(2646, 404);
        }

        s.b[2702] = (p.p33 == 1.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);
        }

        s.b[2703] = (s.v[411] > 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2703]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2704] = (s.v[336] < 0.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) && s.b[2704]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2705] = (s.v[336] < 0.0);
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2705]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign73830_loop_guard: usize = 0;
        while {
            let assign73830_cond_e111269: f64 = (s.v[421] + 1.0);
            let assign73830_cond_e111271: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (s.v[97] <= assign73830_cond_e111269)) { 1.0 } else { 0.0 };
            assign73830_cond_e111271 != 0.0
        } {
            assign73830_loop_guard += 1;
            assert!(assign73830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2707] = (s.v[333] < 60.0);
            s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2707]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2707])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2708] = (s.v[116] < 0.0);
            s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2708]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2709] = (s.v[116] < 1e-6);
            s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) {
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2710] = (s.v[338] > 0.0);
            s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(338, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
            }
            s.b[2711] = (s.v[338] > 0.0);
            s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2712] = (s.v[116] < 0.0);
            s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2712]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2713] = (s.v[116] < 60.0);
            s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };
            s.b[2714] = (s.v[116] < 5e-5);
            s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && s.b[2714]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && (!s.b[2714])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2713])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2715] = (s.v[214] > 0.0);
            s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2715]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_ad_lhs(217, A::add_scaled_product(s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5)), 216);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2715])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2716] = (s.v[79] == 1.0);
            s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2716]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2717] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2717] = if s.b[2717] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2717]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_add(404, 404, 236);
            }
            s.b[2718] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2718]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_mul(2637, 982, 223);
            s.store_mul(2638, 2639, 2637);
            s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2720] = (p.p33 == 4.0);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2646);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);
        }

        s.b[2721] = (s.v[411] > 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2721]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2722] = (s.v[336] < 0.0);
        s.v[2722] = if s.b[2722] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) && s.b[2722]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2723] = (s.v[336] < 0.0);
        s.v[2723] = if s.b[2723] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2723]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign74220_loop_guard: usize = 0;
        while {
            let assign74220_cond_e112924: f64 = (s.v[421] + 1.0);
            let assign74220_cond_e112926: f64 = if (((s.v[2623] != 0.0) && s.b[2720]) && (s.v[97] <= assign74220_cond_e112924)) { 1.0 } else { 0.0 };
            assign74220_cond_e112926 != 0.0
        } {
            assign74220_loop_guard += 1;
            assert!(assign74220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2725] = (s.v[333] < 60.0);
            s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2725]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2725])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2726] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2726]) {
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2647, 334, 336);
                s.store_mul_ad_rhs(2648, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2726])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(2647, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_mul_sub_ad_rhs(2648, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2727] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2727]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2728] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2728] = if s.b[2728] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && s.b[2728]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && (!s.b[2728])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2729] = (s.v[214] > 0.0);
            s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2729]) {
                s.store_sqrt_add(216, 2647, 214);
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(2648), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[2730] = (s.v[2647] > 0.0);
            s.v[2730] = if s.b[2730] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && s.b[2730]) {
                s.store_sqrt(216, 2647);
                s.store_scaled_div(217, 2648, 216, 0.5);
            }
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && (!s.b[2730])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2731] = (s.v[79] > 0.0);
            s.v[2731] = if s.b[2731] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2731]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2732] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2732] = if s.b[2732] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2732]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_add(404, 404, 236);
            }
            s.b[2733] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2733] = if s.b[2733] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2733]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_ad_value(223, {
                if (s.v[2647] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(2647)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_mul(2637, 982, 223);
            s.store_mul(2638, 2639, 2637);
            s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2623] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2735] = (s.v[407] < 0.0);
        s.v[2735] = if s.b[2735] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2735]) {
            s.store_neg(407, 407);
        }

        s.b[2736] = (p.p55 == 0.0);
        s.v[2736] = if s.b[2736] { 1.0 } else { 0.0 };

        s.b[2737] = (p.p50 == 0.0);
        s.v[2737] = if s.b[2737] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2737]) {
            s.store_neg(2640, 404);
        }

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && (!s.b[2737])) {
            s.copy_ad(2640, 396);
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2640), p.p137, A::offset(s.ad_value(2640), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(2640), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2640), p.p137), 782, 0.5);
        }

        s.b[2738] = (s.v[336] < 0.0);
        s.v[2738] = if s.b[2738] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2738]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_sub(407, 407, 603);
        }

        s.b[2739] = (1.0 == 1.0);
        s.v[2739] = if s.b[2739] { 1.0 } else { 0.0 };

        s.b[2740] = (1.0 == 2.0);
        s.v[2740] = if s.b[2740] { 1.0 } else { 0.0 };

        s.b[2741] = (1.0 == 3.0);
        s.v[2741] = if s.b[2741] { 1.0 } else { 0.0 };

        s.b[2742] = (1.0 == 4.0);
        s.v[2742] = if s.b[2742] { 1.0 } else { 0.0 };

        s.b[2743] = (p.p55 == 1.0);
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2739]) && s.b[2743]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && s.b[2739]) && (!s.b[2743])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && s.b[2739]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2740] && (!s.b[2739]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2744] = (p.p55 == 1.0);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2744]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && (!s.b[2744])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {
            s.copy_ad(697, 404);
        }

        s.b[2745] = (p.p430 == 0.0);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2745]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2623] != 0.0) && (s.b[2742] && (!((s.b[2739] || s.b[2740]) || s.b[2741])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2623] = 0.0;

        s.b[2746] = (2.0 == 1.0);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        s.b[2747] = (2.0 == 2.0);
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        s.b[2748] = (2.0 == 3.0);
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        s.b[2749] = (2.0 == 4.0);
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        s.b[2750] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        if (s.b[2746] && s.b[2750]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2751] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2752] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2624, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2753] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_ad(996, A::div_scaled_product(s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0), (-p.p137));
        }

        s.b[2754] = (p.p113 > 0.0);
        s.v[2754] = if s.b[2754] { 1.0 } else { 0.0 };

        s.b[2755] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2755] = if s.b[2755] { 1.0 } else { 0.0 };

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2755]) {
        }

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_ad_value(396, A::div_scaled_product(s.ad_value(783), s.ad_value(784), 1.0, s.ad_value(782), 1.0));
        }

        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(396), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2756] = (s.v[336] < 0.0);
        s.v[2756] = if s.b[2756] { 1.0 } else { 0.0 };

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2756]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2757] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2757] = if s.b[2757] { 1.0 } else { 0.0 };

        if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2765, 0.4);
            s.store_scalar(2766, 0.0);
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
            s.store_scalar(2767, 0.0);
            s.store_scalar(2768, 0.0);
            s.store_mul_scaled_ad_rhs(2763, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2763), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(2764, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        s.b[2770] = (s.v[2765] > (s.v[2764] * 0.5));
        s.v[2770] = if s.b[2770] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2770]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2771] = param_given[338];
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2771]) {
            s.store_scalar(2764, p.p338);
        }

        s.b[2772] = param_given[339];
        s.v[2772] = if s.b[2772] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2772]) {
            s.store_scalar(2765, p.p339);
        }

        s.b[2773] = param_given[338];
        s.v[2773] = if s.b[2773] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2772])) && s.b[2773]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2774] = (s.v[2765] > (s.v[2764] * 0.5));
        s.v[2774] = if s.b[2774] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2774]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2775] = (p.p38 == 1.0);
        s.v[2775] = if s.b[2775] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2775]) {
            s.store_neg(334, 396);
        }

        s.b[2776] = (s.v[334] > s.v[2765]);
        s.v[2776] = if s.b[2776] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2775]) && s.b[2776]) {
            s.store_sub(335, 334, 2765);
            s.store_sub(336, 2764, 2765);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_ad_value(334, A::add_scaled_sub_value_product(1.0, s.ad_value(780), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
            s.store_neg(345, 345);
            s.store_add(344, 2765, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[2775]) && (!s.b[2776])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[2775]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2775])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2759, 0.0);
            s.store_scale(2760, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2777] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2777] = if s.b[2777] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2777]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2777])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign75990_loop_guard: usize = 0;
        while {
            let assign75990_cond_e115471: f64 = if (((s.v[2623] != 0.0) && (!s.b[2777])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign75990_cond_e115471 != 0.0
        } {
            assign75990_loop_guard += 1;
            assert!(assign75990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2777])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2777])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[2778] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2778] = if s.b[2778] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
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

        s.b[2779] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2779] = if s.b[2779] { 1.0 } else { 0.0 };

        s.b[2780] = (1.0 == 1.0);
        s.v[2780] = if s.b[2780] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && s.b[2780]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2781] = (1.0 == 2.0);
        s.v[2781] = if s.b[2781] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && s.b[2781]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2782] = (1.0 == 4.0);
        s.v[2782] = if s.b[2782] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && s.b[2782]) {
            s.store_scalar(720, 3.0);
        }

    }

    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2783] = (1.0 == 8.0);
        s.v[2783] = if s.b[2783] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && (!s.b[2782])) && s.b[2783]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign76320_loop_guard: usize = 0;
        while {
            let assign76320_cond_e115821: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign76320_cond_e115821 != 0.0
        } {
            assign76320_loop_guard += 1;
            assert!(assign76320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && (!s.b[2779])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2778])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2784] = (s.v[402] < s.v[403]);
        s.v[2784] = if s.b[2784] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2784]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[2785] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2785] = if s.b[2785] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2784]) && s.b[2785]) {
            s.store_scaled_div(274, 277, 278, 0.5);
        }

        if (((s.v[2623] != 0.0) && s.b[2784]) && (!s.b[2785])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2784]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_ad_value(272, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, s.ad_value(273), 2.0, s.ad_value(273), s.ad_value(273), 1.414213562373095));
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2767, 404);
        }

        s.b[2786] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2786] = if s.b[2786] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2786]) {
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(402), 1.0, s.ad_value(213), s.ad_value(154), 0.5));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2786])) {
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2787] = (s.v[116] >= 3.0);
        s.v[2787] = if s.b[2787] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2787]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2787])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_ad_value(437, A::div_scaled_inputs(A::add(s.ad_value(402), s.ad_value(397)), -1.0, s.ad_value(212), 1.0));
            s.store_ad_value(441, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0));
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0));
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2788] = (p.p33 > 0.0);
        s.v[2788] = if s.b[2788] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_ad_value(447, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), s.ad_value(154), s.ad_value(397), 1.0));
        }

        s.b[2789] = (p.p33 == 2.0);
        s.v[2789] = if s.b[2789] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_ad_value(447, A::add_scaled_inputs3(s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2789])) {
            s.store_ad_value(447, {
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_ad_value(447, {
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_ad_value(446, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_ad_value(446, {
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.copy_ad(445, 116);
        }

        s.b[2790] = (p.p33 == 2.0);
        s.v[2790] = if s.b[2790] { 1.0 } else { 0.0 };

        s.b[2791] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2791] = if s.b[2791] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
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

        s.b[2792] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2792] = if s.b[2792] { 1.0 } else { 0.0 };

        s.b[2793] = (2.0 == 1.0);
        s.v[2793] = if s.b[2793] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && s.b[2793]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2794] = (2.0 == 2.0);
        s.v[2794] = if s.b[2794] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && s.b[2794]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2795] = (2.0 == 4.0);
        s.v[2795] = if s.b[2795] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && s.b[2795]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2796] = (2.0 == 8.0);
        s.v[2796] = if s.b[2796] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && (!s.b[2795])) && s.b[2796]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign77450_loop_guard: usize = 0;
        while {
            let assign77450_cond_e117378: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign77450_cond_e117378 != 0.0
        } {
            assign77450_loop_guard += 1;
            assert!(assign77450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && (!s.b[2792])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_ad_value(335, A::div_scaled_product3(s.ad_value(446), s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0));
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && (!s.b[2791])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2790])) {
            s.store_ad_value(116, {
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.b[2797] = (p.p33 == 1.0);
        s.v[2797] = if s.b[2797] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2798] = (s.v[411] > 0.0);
        s.v[2798] = if s.b[2798] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2798]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2799] = (s.v[336] < 0.0);
        s.v[2799] = if s.b[2799] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) && s.b[2799]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2800] = (s.v[336] < 0.0);
        s.v[2800] = if s.b[2800] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2800]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2801] = (s.v[333] < 60.0);
        s.v[2801] = if s.b[2801] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2801]) {
            s.store_exp(335, 333);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2801])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2802] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2802] = if s.b[2802] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2802]) {
            s.store_offset(2766, 2766, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2803] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2803] = if s.b[2803] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2803]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2803])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2804, 354, 2760);
        }

        s.b[2806] = (p.p33 == 2.0);
        s.v[2806] = if s.b[2806] { 1.0 } else { 0.0 };

        s.b[2807] = ((s.v[2804] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2807] = if s.b[2807] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(2804), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1));
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

        s.b[2808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2808] = if s.b[2808] { 1.0 } else { 0.0 };

        s.b[2809] = (2.0 == 1.0);
        s.v[2809] = if s.b[2809] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && s.b[2809]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2810] = (2.0 == 2.0);
        s.v[2810] = if s.b[2810] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && s.b[2810]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2811] = (2.0 == 4.0);
        s.v[2811] = if s.b[2811] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && s.b[2811]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2812] = (2.0 == 8.0);
        s.v[2812] = if s.b[2812] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && (!s.b[2811])) && s.b[2812]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign78260_loop_guard: usize = 0;
        while {
            let assign78260_cond_e118448: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78260_cond_e118448 != 0.0
        } {
            assign78260_loop_guard += 1;
            assert!(assign78260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && (!s.b[2808])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(386), s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && (!s.b[2807])) {
            s.copy_ad(335, 2804);
            s.store_scalar(334, 1.0);
        }

        s.b[2813] = (s.v[334] < 1.0);
        s.v[2813] = if s.b[2813] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2813]) {
            s.store_offset(2766, 2766, 2.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) {
            s.store_ad_value(335, {
                if (s.v[2804] <= s.v[386]) {
                    s.ad_value(2804)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.b[2814] = (s.v[2804] >= s.v[386]);
        s.v[2814] = if s.b[2814] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) && s.b[2814]) {
            s.store_offset(2766, 2766, 2.0);
        }

        s.b[2815] = (s.v[2766] >= 2.0);
        s.v[2815] = if s.b[2815] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) {
            s.copy_ad(2805, 404);
            s.store_mul(354, 335, 2760);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.b[2816] = (p.p33 == 2.0);
        s.v[2816] = if s.b[2816] { 1.0 } else { 0.0 };

        s.b[2817] = ((s.v[404] > (s.v[2805] - 0.1)) && (0.1 >= 0.0));
        s.v[2817] = if s.b[2817] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_offset_sub(781, 404, 2805, 0.1);
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

        s.b[2818] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2818] = if s.b[2818] { 1.0 } else { 0.0 };

        s.b[2819] = (2.0 == 1.0);
        s.v[2819] = if s.b[2819] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2820] = (2.0 == 2.0);
        s.v[2820] = if s.b[2820] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2821] = (2.0 == 4.0);
        s.v[2821] = if s.b[2821] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2822] = (2.0 == 8.0);
        s.v[2822] = if s.b[2822] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) && s.b[2822]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign78710_loop_guard: usize = 0;
        while {
            let assign78710_cond_e119056: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78710_cond_e119056 != 0.0
        } {
            assign78710_loop_guard += 1;
            assert!(assign78710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && (!s.b[2818])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_add_ad_lhs(404, A::offset(s.ad_value(2805), (-0.1)), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && (!s.b[2816])) {
            s.store_ad_value(404, {
                if (s.v[404] <= s.v[2805]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(2805)
                }
            });
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.copy_ad(2767, 404);
        }

        s.b[2823] = (p.p33 == 1.0);
        s.v[2823] = if s.b[2823] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);
        }

        s.b[2824] = (s.v[411] > 0.0);
        s.v[2824] = if s.b[2824] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2824]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2825] = (s.v[336] < 0.0);
        s.v[2825] = if s.b[2825] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) && s.b[2825]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2826] = (s.v[336] < 0.0);
        s.v[2826] = if s.b[2826] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2826]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign79070_loop_guard: usize = 0;
        while {
            let assign79070_cond_e119580: f64 = (s.v[421] + 1.0);
            let assign79070_cond_e119582: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (s.v[97] <= assign79070_cond_e119580)) { 1.0 } else { 0.0 };
            assign79070_cond_e119582 != 0.0
        } {
            assign79070_loop_guard += 1;
            assert!(assign79070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2828] = (s.v[333] < 60.0);
            s.v[2828] = if s.b[2828] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2828]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2828])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2829] = (s.v[116] < 0.0);
            s.v[2829] = if s.b[2829] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2829]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2830] = (s.v[116] < 1e-6);
            s.v[2830] = if s.b[2830] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) {
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2831] = (s.v[338] > 0.0);
            s.v[2831] = if s.b[2831] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && s.b[2831]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && (!s.b[2831])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(338, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
            }
            s.b[2832] = (s.v[338] > 0.0);
            s.v[2832] = if s.b[2832] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2832]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2832])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2833] = (s.v[116] < 0.0);
            s.v[2833] = if s.b[2833] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2833]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2834] = (s.v[116] < 60.0);
            s.v[2834] = if s.b[2834] { 1.0 } else { 0.0 };
            s.b[2835] = (s.v[116] < 5e-5);
            s.v[2835] = if s.b[2835] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && s.b[2835]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && (!s.b[2835])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2834])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2836] = (s.v[214] > 0.0);
            s.v[2836] = if s.b[2836] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2836]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_ad_lhs(217, A::add_scaled_product(s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5)), 216);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2836])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2837] = (s.v[79] == 1.0);
            s.v[2837] = if s.b[2837] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2837]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_scaled_div(236, 232, 233, -1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2838] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2838] = if s.b[2838] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2838]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_add(404, 404, 236);
            }
            s.b[2839] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2839] = if s.b[2839] { 1.0 } else { 0.0 };
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2839]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_mul(2758, 982, 223);
            s.store_mul(2759, 2760, 2758);
            s.store_offset_div(100, 2759, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2841] = (p.p33 == 4.0);
        s.v[2841] = if s.b[2841] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2767);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);
        }

        s.b[2842] = (s.v[411] > 0.0);
        s.v[2842] = if s.b[2842] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2842]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2843] = (s.v[336] < 0.0);
        s.v[2843] = if s.b[2843] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) && s.b[2843]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2844] = (s.v[336] < 0.0);
        s.v[2844] = if s.b[2844] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2844]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_73(
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
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
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
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2768, 334, 336);
                s.store_mul_ad_rhs(2769, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2847])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(2768, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
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
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(2769), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[2851] = (s.v[2768] > 0.0);
            s.v[2851] = if s.b[2851] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2850])) && s.b[2851]) {
                s.store_sqrt(216, 2768);
                s.store_scaled_div(217, 2769, 216, 0.5);
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
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2852] = (s.v[79] > 0.0);
            s.v[2852] = if s.b[2852] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2852]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) {
                s.store_scaled_div(236, 232, 233, -1.0);
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
            if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2852])) && s.b[2854]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2841]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_ad_value(223, {
                if (s.v[2768] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(2768)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2761), p.p137, A::offset(s.ad_value(2761), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(2761), p.p137), s.ad_value(782)), 1.0, 0.5);
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
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2623] != 0.0) && s.b[2856]) && s.b[2857]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
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

        if (s.b[2867] && s.b[2871]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2621, 1.0);
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

        if ((s.b[2868] && (!s.b[2867])) && s.b[2872]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2873] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2873] = if s.b[2873] { 1.0 } else { 0.0 };

        if ((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2624, 1.0);
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
        }

    }

    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) {
            s.store_scalar(335, p.p63);
            s.store_offset_ad(996, A::div_scaled_product(s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0), (-p.p137));
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
            s.store_ad_value(396, A::div_scaled_product(s.ad_value(783), s.ad_value(784), 1.0, s.ad_value(782), 1.0));
        }

        if ((((s.b[2869] && (!(s.b[2867] || s.b[2868]))) && s.b[2873]) && s.b[2874]) && s.b[2875]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(396), p.p137), s.ad_value(782)), 1.0, 0.5);
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

        if ((s.b[2870] && (!((s.b[2867] || s.b[2868]) || s.b[2869]))) && s.b[2878]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2886, 0.4);
            s.store_scalar(2887, 0.0);
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
            s.store_scalar(2888, 0.0);
            s.store_scalar(2889, 0.0);
            s.store_mul_scaled_ad_rhs(2884, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2884), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(2885, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
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
            s.store_ad_value(334, A::add_scaled_sub_value_product(1.0, s.ad_value(780), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
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
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
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
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[2899] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2899] = if s.b[2899] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
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

        s.b[2900] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2900] = if s.b[2900] { 1.0 } else { 0.0 };

        s.b[2901] = (1.0 == 1.0);
        s.v[2901] = if s.b[2901] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && s.b[2901]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2902] = (1.0 == 2.0);
        s.v[2902] = if s.b[2902] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && s.b[2902]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2903] = (1.0 == 4.0);
        s.v[2903] = if s.b[2903] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && (!s.b[2902])) && s.b[2903]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2904] = (1.0 == 8.0);
        s.v[2904] = if s.b[2904] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (!s.b[2901])) && (!s.b[2902])) && (!s.b[2903])) && s.b[2904]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign81560_loop_guard: usize = 0;
        while {
            let assign81560_cond_e124132: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign81560_cond_e124132 != 0.0
        } {
            assign81560_loop_guard += 1;
            assert!(assign81560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && s.b[2900]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) && (!s.b[2900])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2899]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
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
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2905] = (s.v[402] < s.v[403]);
        s.v[2905] = if s.b[2905] { 1.0 } else { 0.0 };

        if ((s.v[2623] != 0.0) && s.b[2905]) {
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(333), 9.0, A::offset(s.ad_value(332), (-2.0))));
            s.store_square(276, 278);
        }

        s.b[2906] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2906] = if s.b[2906] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2905]) && s.b[2906]) {
            s.store_scaled_div(274, 277, 278, 0.5);
        }

        if (((s.v[2623] != 0.0) && s.b[2905]) && (!s.b[2906])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2905]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_ad_value(272, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, s.ad_value(273), 2.0, s.ad_value(273), s.ad_value(273), 1.414213562373095));
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
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(402), 1.0, s.ad_value(213), s.ad_value(154), 0.5));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2907])) {
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2908] = (s.v[116] >= 3.0);
        s.v[2908] = if s.b[2908] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2908]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_ad(332, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2908])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_ad_value(437, A::div_scaled_inputs(A::add(s.ad_value(402), s.ad_value(397)), -1.0, s.ad_value(212), 1.0));
            s.store_ad_value(441, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0));
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0));
            s.store_ad_value(89, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2909] = (p.p33 > 0.0);
        s.v[2909] = if s.b[2909] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_ad_value(447, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), s.ad_value(154), s.ad_value(397), 1.0));
        }

        s.b[2910] = (p.p33 == 2.0);
        s.v[2910] = if s.b[2910] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2910]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_ad_value(447, A::add_scaled_inputs3(s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && (!s.b[2910])) {
            s.store_ad_value(447, {
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_ad_value(447, {
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_ad_value(446, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(154), s.ad_value(397), 1.0));
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.store_ad_value(446, {
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) {
            s.copy_ad(445, 116);
        }

        s.b[2911] = (p.p33 == 2.0);
        s.v[2911] = if s.b[2911] { 1.0 } else { 0.0 };

        s.b[2912] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2912] = if s.b[2912] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
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

        s.b[2913] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2913] = if s.b[2913] { 1.0 } else { 0.0 };

        s.b[2914] = (2.0 == 1.0);
        s.v[2914] = if s.b[2914] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && s.b[2914]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2915] = (2.0 == 2.0);
        s.v[2915] = if s.b[2915] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && s.b[2915]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2916] = (2.0 == 4.0);
        s.v[2916] = if s.b[2916] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && (!s.b[2915])) && s.b[2916]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2917] = (2.0 == 8.0);
        s.v[2917] = if s.b[2917] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (!s.b[2914])) && (!s.b[2915])) && (!s.b[2916])) && s.b[2917]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign82690_loop_guard: usize = 0;
        while {
            let assign82690_cond_e125689: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign82690_cond_e125689 != 0.0
        } {
            assign82690_loop_guard += 1;
            assert!(assign82690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && s.b[2913]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) && (!s.b[2913])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_ad_value(335, A::div_scaled_product3(s.ad_value(446), s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0));
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0));
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && s.b[2912]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && s.b[2911]) && (!s.b[2912])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2909]) && (!s.b[2911])) {
            s.store_ad_value(116, {
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.b[2918] = (p.p33 == 1.0);
        s.v[2918] = if s.b[2918] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2919] = (s.v[411] > 0.0);
        s.v[2919] = if s.b[2919] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2919]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && (!s.b[2919])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
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
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
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

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2918]) && s.b[2923]) {
            s.store_offset(2887, 2887, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.store_ad_value(404, A::add_scaled_product(s.ad_value(397), (-1.0), s.ad_value(116), s.ad_value(155), 1.0));
        }

        s.b[2924] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2924] = if s.b[2924] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2924]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2924])) {
            s.store_mul_scaled_ad_rhs(336, 116, 0.7071067811865475, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
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
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(2925), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1));
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

        s.b[2929] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2929] = if s.b[2929] { 1.0 } else { 0.0 };

        s.b[2930] = (2.0 == 1.0);
        s.v[2930] = if s.b[2930] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && s.b[2930]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2931] = (2.0 == 2.0);
        s.v[2931] = if s.b[2931] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && s.b[2931]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2932] = (2.0 == 4.0);
        s.v[2932] = if s.b[2932] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && (!s.b[2931])) && s.b[2932]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2933] = (2.0 == 8.0);
        s.v[2933] = if s.b[2933] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (!s.b[2930])) && (!s.b[2931])) && (!s.b[2932])) && s.b[2933]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign83500_loop_guard: usize = 0;
        while {
            let assign83500_cond_e126759: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83500_cond_e126759 != 0.0
        } {
            assign83500_loop_guard += 1;
            assert!(assign83500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && s.b[2929]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) && (!s.b[2929])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(386), s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2928]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && (!s.b[2928])) {
            s.copy_ad(335, 2925);
            s.store_scalar(334, 1.0);
        }

        s.b[2934] = (s.v[334] < 1.0);
        s.v[2934] = if s.b[2934] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2927]) && s.b[2934]) {
            s.store_offset(2887, 2887, 2.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2927])) {
            s.store_ad_value(335, {
                if (s.v[2925] <= s.v[386]) {
                    s.ad_value(2925)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.b[2935] = (s.v[2925] >= s.v[386]);
        s.v[2935] = if s.b[2935] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && (!s.b[2927])) && s.b[2935]) {
            s.store_offset(2887, 2887, 2.0);
        }

        s.b[2936] = (s.v[2887] >= 2.0);
        s.v[2936] = if s.b[2936] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) {
            s.copy_ad(2926, 404);
            s.store_mul(354, 335, 2881);
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
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

        s.b[2939] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2939] = if s.b[2939] { 1.0 } else { 0.0 };

        s.b[2940] = (2.0 == 1.0);
        s.v[2940] = if s.b[2940] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2941] = (2.0 == 2.0);
        s.v[2941] = if s.b[2941] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && s.b[2941]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2942] = (2.0 == 4.0);
        s.v[2942] = if s.b[2942] { 1.0 } else { 0.0 };

        if (((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && (!s.b[2941])) && s.b[2942]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2943] = (2.0 == 8.0);
        s.v[2943] = if s.b[2943] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign83950_loop_guard: usize = 0;
        while {
            let assign83950_cond_e127367: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83950_cond_e127367 != 0.0
        } {
            assign83950_loop_guard += 1;
            assert!(assign83950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && s.b[2939]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) && (!s.b[2939])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_add_ad_lhs(404, A::offset(s.ad_value(2926), (-0.1)), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2936]) && (!s.b[2937])) {
            s.store_ad_value(404, {
                if (s.v[404] <= s.v[2926]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(2926)
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.v[2623] != 0.0) && (!s.b[2905])) {
            s.copy_ad(2888, 404);
        }

        s.b[2944] = (p.p33 == 1.0);
        s.v[2944] = if s.b[2944] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2881)), s.ad_value(155)), 2.0);
        }

        s.b[2945] = (s.v[411] > 0.0);
        s.v[2945] = if s.b[2945] { 1.0 } else { 0.0 };

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2945]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2945])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
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
            s.store_scalar(97, 1.0);
        }

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
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
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
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2952] = (s.v[338] > 0.0);
            s.v[2952] = if s.b[2952] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && s.b[2951]) && s.b[2952]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, s.ad_value(223), 1.0));
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && s.b[2951]) && (!s.b[2952])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && (!s.b[2951])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(338, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
            }
            s.b[2953] = (s.v[338] > 0.0);
            s.v[2953] = if s.b[2953] { 1.0 } else { 0.0 };
            if ((((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2950])) && (!s.b[2951])) && s.b[2953]) {
                s.store_sqrt(223, 338);
                s.store_ad_value(420, A::div_scaled_product(s.ad_value(154), A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, s.ad_value(223), 1.0));
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
                s.store_div_ad_lhs(217, A::add_scaled_product(s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5)), 216);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2954])) && (!s.b[2957])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2958] = (s.v[79] == 1.0);
            s.v[2958] = if s.b[2958] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && s.b[2958]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) {
                s.store_scaled_div(236, 232, 233, -1.0);
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
            if (((((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) && (!s.b[2958])) && s.b[2960]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2905])) && s.b[2944]) {
                s.store_offset(97, 97, 1.0);
            }
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
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2881)), s.ad_value(155)), 2.0);
        }

        s.b[2963] = (s.v[411] > 0.0);
        s.v[2963] = if s.b[2963] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2963]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2963])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0, 0.5);
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
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_78(
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
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(419), -1.0, s.ad_value(418)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
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
                s.store_ad_value(334, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(116)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(336, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(415)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2889, 334, 336);
                s.store_mul_ad_rhs(2890, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)));
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2968])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_ad_value(2889, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
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
                s.store_div_ad_lhs(217, A::add_scaled_inputs(s.ad_value(2890), 0.5, s.ad_value(215), 0.5), 216);
            }
            s.b[2972] = (s.v[2889] > 0.0);
            s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2971])) && s.b[2972]) {
                s.store_sqrt(216, 2889);
                s.store_scaled_div(217, 2890, 216, 0.5);
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
                s.store_ad_value(232, A::add_scaled_inputs_product(s.ad_value(404), 1.0, s.ad_value(402), (-1.0), s.ad_value(212), s.ad_value(216), 1.0));
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2973] = (s.v[79] > 0.0);
            s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };
            if (((s.v[2623] != 0.0) && s.b[2962]) && s.b[2973]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) {
                s.store_scaled_div(236, 232, 233, -1.0);
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
            if ((((s.v[2623] != 0.0) && s.b[2962]) && (!s.b[2973])) && s.b[2975]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2962]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2962]) {
            s.store_ad_value(223, {
                if (s.v[2889] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(2889)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
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
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2882), p.p137, A::offset(s.ad_value(2882), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(2882), p.p137), s.ad_value(782)), 1.0, 0.5);
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
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1)));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2623] != 0.0) && s.b[2977]) && s.b[2978]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(603, A::add_scaled_inputs3(s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
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

        if (s.b[2988] && s.b[2992]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2621, 1.0);
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

        if ((s.b[2989] && (!s.b[2988])) && s.b[2993]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2994] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        if ((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) {
            s.store_scalar(2623, 1.0);
            s.store_scalar(2624, 1.0);
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
        }

    }

    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) {
            s.store_scalar(335, p.p63);
            s.store_offset_ad(996, A::div_scaled_product(s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0), (-p.p137));
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
            s.store_ad_value(396, A::div_scaled_product(s.ad_value(783), s.ad_value(784), 1.0, s.ad_value(782), 1.0));
        }

        if ((((s.b[2990] && (!(s.b[2988] || s.b[2989]))) && s.b[2994]) && s.b[2995]) && s.b[2996]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div(A::offset(s.ad_value(396), p.p137), s.ad_value(782)), 1.0, 0.5);
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

        if ((s.b[2991] && (!((s.b[2988] || s.b[2989]) || s.b[2990]))) && s.b[2999]) {
            s.store_scalar(2623, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(3007, 0.4);
            s.store_scalar(3008, 0.0);
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
            s.store_scalar(3009, 0.0);
            s.store_scalar(3010, 0.0);
            s.store_mul_scaled_ad_rhs(3005, 155, 2.0, A::ln(A::div(s.ad_value(409), s.ad_value(394))));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3005), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(3006, 0.8, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
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
            s.store_ad_value(334, A::add_scaled_sub_value_product(1.0, s.ad_value(780), 1.0, s.ad_value(781), s.ad_value(345), 1.0));
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
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
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
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_add(335, 781, 782, 0.5, 0.5);
        }

        s.b[3020] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) {
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

        s.b[3021] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

        s.b[3022] = (1.0 == 1.0);
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && s.b[3022]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3023] = (1.0 == 2.0);
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && s.b[3023]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3024] = (1.0 == 4.0);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && (!s.b[3023])) && s.b[3024]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3025] = (1.0 == 8.0);
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (!s.b[3022])) && (!s.b[3023])) && (!s.b[3024])) && s.b[3025]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign86800_loop_guard: usize = 0;
        while {
            let assign86800_cond_e132443: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86800_cond_e132443 != 0.0
        } {
            assign86800_loop_guard += 1;
            assert!(assign86800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && s.b[3021]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[3020]) && (!s.b[3021])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

    }
}
