#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11490_loop_guard: usize = 0;
        while {
            let assign11490_cond_e14353: f64 = if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11490_cond_e14353 != 0.0
        } {
            assign11490_loop_guard += 1;
            assert!(assign11490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 310);
                s.store_exp_neg_input(281, 280);
            }
            s.b[850] = (s.v[310] > 1e-8);
            s.v[850] = if s.b[850] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[850]) {
                s.store_exp_mul(278, 120, 310);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[851] = (s.v[310] < (-1e-8));
            s.v[851] = if s.b[851] { 1.0 } else { 0.0 };
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && s.b[851]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && (!s.b[851])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-6));
            }
            s.b[852] = (s.v[284] < 0.0);
            s.v[852] = if s.b[852] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[852]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_offset_ad(638, A::sub_scaled_inputs(s.ad_value(296), -1.0, s.ad_value(284), 1.0), (-1e-9));
                s.store_scale(639, 296, (-(4.0 * 1e-9)));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3(284, s.ad_value(296), -1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(471), 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                let assign11490_body27_ad_e14835: A = A::div(A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(308), 1.0, s.ad_value(310), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), s.v[536], s.ad_value(296), (0.5 * s.v[536]))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), s.v[536], s.ad_value(333), 1.0, (-1.0)));
                s.store_sub_ad_rhs(284, 310, assign11490_body27_ad_e14835);
            }
            s.b[853] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);
            s.v[853] = if s.b[853] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[853]) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(310, 284);
                s.copy_ad(314, 282);
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
            s.store_add(310, 440, 310);
            s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(584, 309);
        }

        s.b[854] = (s.v[86] < 1e-12);
        s.v[854] = if s.b[854] { 1.0 } else { 0.0 };

        if s.b[854] {
            s.copy_ad(302, 305);
            s.copy_ad(303, 306);
            s.copy_ad(304, 307);
            s.copy_ad(581, 522);
        }

        if (!s.b[854]) {
            s.copy_ad(302, 308);
            s.copy_ad(303, 309);
            s.store_sub(304, 310, 440);
        }

        if (!s.b[854]) {
            if (s.v[303] < s.v[302]) {
                s.copy_ad(581, 303);
            } else {
                s.copy_ad(581, 302);
            }
        }

        s.b[379] = (s.v[292] < 0.0);
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

        s.copy_ad(308, 302);

        s.copy_ad(309, 303);

        s.copy_ad(310, 304);

        s.copy_ad(584, 581);

        s.v[63] = 1.0;

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11690_loop_guard: usize = 0;
        while {
            let assign11690_cond_e14989: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            assign11690_cond_e14989 != 0.0
        } {
            assign11690_loop_guard += 1;
            assert!(assign11690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 310);
            s.store_mul(297, 120, 279);
            s.store_exp_neg_input(278, 297);
            s.b[855] = (s.v[279] < (-1e-8));
            s.v[855] = if s.b[855] { 1.0 } else { 0.0 };
            if s.b[855] {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_sqrt_ad_rhs(314, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 314, 1.0);
            }
            s.b[856] = (s.v[279] > (1e-8 / 10.0));
            s.v[856] = if s.b[856] { 1.0 } else { 0.0 };
            if ((!s.b[855]) && s.b[856]) {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_scaled_ad_rhs(314, 439, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0)));
                s.store_div_scaled_product_right_ad(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 314, 1.0);
            }
            if ((!s.b[855]) && (!s.b[856])) {
                s.store_scaled_mul(314, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(344, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_scaled_inputs4(309, s.ad_value(310), 1.0, s.ad_value(314), (-1.0 / (s.v[294])), s.ad_value(50), 1.0, s.ad_value(298), 1.0);
            s.store_sub_from_scalar_ad(582, 1.0, A::scale(s.ad_value(344), 1.0 / (s.v[294])));
            s.store_sub(279, 308, 584);
            s.store_mul(297, 120, 279);
            s.b[857] = ((-s.v[297]) >= 80.0);
            s.v[857] = if s.b[857] { 1.0 } else { 0.0 };
            if s.b[857] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[857]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[858] = (s.v[279] < (-1e-8));
            s.v[858] = if s.b[858] { 1.0 } else { 0.0 };
            if s.b[858] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(576, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.b[859] = (s.v[279] > 1e-8);
            s.v[859] = if s.b[859] { 1.0 } else { 0.0 };
            if ((!s.b[858]) && s.b[859]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(576, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(578, 577);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(576), s.ad_value(576), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(576), s.ad_value(577), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, s.ad_value(282), 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(576), s.ad_value(578), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(313, 576, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(579, 577, (-1.0), 141, 537, -1.0);
                s.store_add_scaled_product_indices(580, 578, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[858]) && (!s.b[859])) {
                s.store_scaled_mul(576, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(577, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.store_sub(279, 309, 584);
            s.store_mul(297, 120, 279);
            s.b[860] = ((-s.v[297]) >= 80.0);
            s.v[860] = if s.b[860] { 1.0 } else { 0.0 };
            if s.b[860] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[860]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[861] = (s.v[279] < (-1e-8));
            s.v[861] = if s.b[861] { 1.0 } else { 0.0 };
            if s.b[861] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(585, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[862] = (s.v[279] > 1e-8);
            s.v[862] = if s.b[862] { 1.0 } else { 0.0 };
            if ((!s.b[861]) && s.b[862]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(585, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(587, 586);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(585), s.ad_value(585), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(585), s.ad_value(586), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, s.ad_value(282), 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(585), s.ad_value(587), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(588, 585, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(589, 586, (-1.0), 141, 539, -1.0);
                s.store_add_scaled_product_indices(590, 587, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[861]) && (!s.b[862])) {
                s.store_scaled_mul(585, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(586, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[863] = s.b[379];
            s.v[863] = if s.b[863] { 1.0 } else { 0.0 };
            if s.b[863] {
                s.store_scalar(574, s.v[63]);
                s.store_scalar(63, s.v[29]);
            }
            if (!s.b[863]) {
                s.store_add_scaled_inputs3(346, s.ad_value(308), 1.0, s.ad_value(76), (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(314), 1.0, s.ad_value(313), 1.0, s.ad_value(576), 1.0, s.ad_value(588), 1.0), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)), -1.0);
                s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(579), 1.0, s.ad_value(577), 1.0, s.ad_value(270), 1.0));
                s.store_div_scaled_inputs(348, A::add_scaled_inputs4(s.ad_value(580), 1.0, s.ad_value(578), 1.0, s.ad_value(590), 1.0, s.ad_value(587), 1.0), -1.0, s.ad_value(270), 1.0);
                s.store_div_scaled_inputs(349, A::add_scaled_product(s.ad_value(344), 1.0, A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582), 1.0), -1.0, s.ad_value(270), 1.0);
            }
            s.b[864] = (s.v[314] <= s.v[599]);
            s.v[864] = if s.b[864] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[864]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(314), 2.0, s.ad_value(296), 1.0));
                s.store_div_scaled_product_indices(604, 296, 344, 1.0, 279, 1.0);
            }
            s.b[865] = (s.v[314] <= s.v[603]);
            s.v[865] = if s.b[865] { 1.0 } else { 0.0 };
            if (((!s.b[863]) && (!s.b[864])) && s.b[865]) {
                s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(314), 3.0, s.ad_value(602), (-3.0), s.ad_value(314), 1.0, s.ad_value(603), (-1.0)), 344);
            }
            if (((!s.b[863]) && (!s.b[864])) && (!s.b[865])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[863]) {
                s.store_div_scaled_inputs(281, s.ad_value(316), (-s.v[650]), s.ad_value(296), 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 577, 1.0 / (s.v[535]));
                s.store_scale(352, 578, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_div_scaled_inputs(281, s.ad_value(316), (-s.v[651]), s.ad_value(296), 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 587, 1.0 / (s.v[535]));
                s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 586, 582, 1.0 / (s.v[535]));
                s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);
            }
            s.b[866] = (s.v[357] > 0.0);
            s.v[866] = if s.b[866] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[866]) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);
            }
            if ((!s.b[863]) && (!s.b[866])) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));
            }
            if (!s.b[863]) {
                s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));
                s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));
                s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));
                s.store_mul_neg_lhs(362, 351, 356);
                s.store_mul(363, 347, 356);
                s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));
                s.store_mul(365, 351, 355);
                s.store_mul_neg_lhs(366, 347, 355);
                s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));
                s.store_mul_scaled_ad_rhs(368, 358, -1.0, A::add_scaled_products3(s.ad_value(359), s.ad_value(346), 1.0, s.ad_value(360), s.ad_value(350), 1.0, s.ad_value(361), s.ad_value(354), 1.0));
                s.store_mul_scaled_ad_rhs(369, 358, -1.0, A::add_scaled_products3(s.ad_value(362), s.ad_value(346), 1.0, s.ad_value(363), s.ad_value(350), 1.0, s.ad_value(364), s.ad_value(354), 1.0));
                s.store_mul_scaled_ad_rhs(370, 358, -1.0, A::add_scaled_products3(s.ad_value(365), s.ad_value(346), 1.0, s.ad_value(366), s.ad_value(350), 1.0, s.ad_value(367), s.ad_value(354), 1.0));
                s.store_abs(279, 368);
            }
            s.b[867] = (s.v[279] < ((s.v[369]) as f64).abs());
            s.v[867] = if s.b[867] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[867]) {
                s.store_abs(279, 369);
            }
            s.b[868] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.v[868] = if s.b[868] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[868]) {
                s.store_abs(279, 370);
            }
            if (!s.b[863]) {
                s.store_scalar(606, 1.0);
            }
            s.b[869] = (s.v[63] > 80.0);
            s.v[869] = if s.b[869] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[869]) {
                s.store_scalar(606, 25.0);
            }
            s.b[870] = (s.v[63] > 40.0);
            s.v[870] = if s.b[870] { 1.0 } else { 0.0 };
            if (((!s.b[863]) && (!s.b[869])) && s.b[870]) {
                s.store_scalar(606, 25.0);
            }
            s.b[871] = (s.v[63] > 20.0);
            s.v[871] = if s.b[871] { 1.0 } else { 0.0 };
            if ((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && s.b[871]) {
                s.store_scalar(606, 25.0);
            }
            s.b[872] = (s.v[63] > 10.0);
            s.v[872] = if s.b[872] { 1.0 } else { 0.0 };
            if (((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && (!s.b[871])) && s.b[872]) {
                s.store_scalar(606, 5.0);
            }
            s.b[873] = (s.v[279] > (0.1 / s.v[606]));
            s.v[873] = if s.b[873] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[873]) {
                s.store_mul_ad_rhs(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
            }
            if (!s.b[863]) {
                s.store_add(308, 308, 368);
                s.store_add(584, 584, 369);
                s.store_add(310, 310, 370);
                s.store_scale(607, 606, 1e-12);
            }
            s.b[874] = (s.v[279] < s.v[607]);
            s.v[874] = if s.b[874] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[874]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(63, 63, 1.0);
        }

        s.b[875] = (s.v[574] > 0.0);
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if s.b[875] {
            s.copy_ad(63, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[876] = (s.v[63] > s.v[29]);
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if s.b[876] {
            s.copy_ad(308, 302);
            s.copy_ad(309, 303);
            s.copy_ad(310, 304);
            s.copy_ad(584, 581);
        }

        s.copy_ad(57, 308);

        s.store_sub(59, 57, 56);

        s.copy_ad(51, 396);

        s.b[878] = ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0));
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if s.b[878] {
            s.store_scalar(34, 1.0);
        }

        s.copy_ad(317, 305);

        s.copy_ad(318, 308);

        s.store_sub(59, 318, 317);

        s.copy_ad(322, 306);

        s.copy_ad(323, 309);

        s.store_sub(155, 323, 322);

        s.store_add_scaled_inputs3(153, s.ad_value(313), 1.0, s.ad_value(311), (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311)), A::sub(s.ad_value(318), s.ad_value(317)), 0.5), -1.0);

        s.store_add_scaled_inputs3(154, s.ad_value(588), 1.0, s.ad_value(528), (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528)), A::sub(s.ad_value(323), s.ad_value(322)), 0.5), -1.0);

        s.b[879] = ((s.v[153] < 0.0) || (s.v[51] == 0.0));
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if s.b[879] {
            s.store_scalar(153, 0.0);
        }

        s.b[880] = ((s.v[154] < 0.0) || (s.v[51] == 0.0));
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[880] {
            s.store_scalar(154, 0.0);
        }

        s.store_add(151, 153, 154);

        s.store_scaled_add(384, 576, 523, (-0.5));

        s.store_offset_sub(371, 308, 305, 1e-12);

        s.store_neg_ad(373, A::sub(s.ad_value(313), s.ad_value(311)));

        s.b[881] = ((-s.v[373]) < 1e-18);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if s.b[881] {
            s.store_scalar(373, 0.0);
        }

        s.store_offset_ad(372, A::div_scaled_inputs(s.ad_value(373), (-2.0), A::mul(A::mul3(s.ad_value(120), s.ad_value(270), s.ad_value(371)), s.ad_value(371)), 1.0), 1.0);

        s.store_sub_from_scalar_ad(85, 1.0, A::div_scaled_product(s.ad_value(372), s.ad_value(371), 1.0, s.ad_value(86), 1.0));

        s.b[882] = (s.v[85] <= 0.0);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if s.b[882] {
            s.store_scalar(85, 0.0);
        }

        s.store_scaled_add(383, 311, 313, (-0.5));

        s.store_scaled_add(167, 528, 588, (-0.5));

        s.v[262] = 0.0;

        s.b[883] = (s.v[34] == 0.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        s.b[884] = ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16)));
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if (s.b[883] && s.b[884]) {
            s.store_scalar(262, 0.0);
            s.copy_ad(260, 57);
        }

        s.b[885] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((s.b[883] && s.b[884]) && s.b[885]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_scalar(263, p.p227);
            s.store_div_from_scalar_ad(282, 1.034943e-10, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(149), p.p178, s.ad_value(263), 1.0), 1.0, s.ad_value(446), s.ad_value(126), 1.0));
            s.store_add_scaled_inputs3(260, s.ad_value(51), p.p176, s.ad_value(56), p.p176, s.ad_value(57), (1.0 - p.p176));
        }

        s.b[886] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if ((s.b[883] && (!s.b[884])) && s.b[886]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_sub(284, 260, 57);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 284, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 284, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[887] = (s.v[284] < 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if ((s.b[883] && (!s.b[884])) && s.b[887]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_div_ad_rhs(283, 151, A::mul(s.ad_value(120), s.ad_value(149)));
            s.store_scale(288, 126, 9662367879.197212);
            s.store_scalar(279, 1000000000.0);
            s.store_div_scaled_inputs_product(387, s.ad_value(283), 2.0, A::mul3_scaled_output(s.ad_value(288), s.ad_value(284), s.ad_value(282), 2.0), 1.0, s.ad_value(279), s.ad_value(282), 1.0, s.ad_value(123), 1.0);
            s.store_mul(285, 387, 282);
            s.store_add_scaled_product_indices(387, 279, 4.0, 288, 284, (2.0 * 4.0));
            s.store_mul3_lhs(286, 387, 282, 282);
            s.store_sqrt_square_add(287, 285, 286);
            s.store_scaled_sub(262, 287, 285, 0.5);
            s.copy_ad(279, 262);
            s.store_mul(262, 276, 279);
        }

        if s.b[883] {
            s.store_scale(262, 262, s.v[483]);
        }

        s.store_sub(386, 123, 262);

        s.b[888] = (s.v[386] < 1e-9);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scalar(386, 1e-9);
        }

        s.store_mul_scaled_ad_rhs(91, 123, (-s.v[513]), A::add(s.ad_value(383), s.ad_value(167)));

        s.store_mul_scaled_ad_lhs(336, A::add(s.ad_value(312), s.ad_value(314)), 123, (0.5 * s.v[513]));

        s.store_scaled_sub(279, 51, 59, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));

        s.store_offset_ad(639, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);

        s.store_offset_ad(640, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));

        s.store_div_from_scalar(75, p.p217, 639);

        s.store_div_scaled_inputs(280, s.ad_value(640), (-2.0), A::square(s.ad_value(639)), 1.0);

        s.b[889] = (s.v[75] < (10.0 * 2.220446049250313e-16));
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if s.b[889] {
            s.store_scalar(75, (10.0 * 2.220446049250313e-16));
        }

        s.store_add(74, 56, 75);

        s.v[499] = (1.034943e-10 / 100.0);

        s.store_scale(500, 313, 0.0001);

        s.store_scale(501, 588, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(502, 383, 0.0001);

        s.store_scale(503, 167, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(506, 384, 0.0001);

        s.v[507] = (p.p229 * 100.0);

        s.v[591] = ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]);

        s.v[592] = ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]);

        s.store_sqrt_square_offset(639, 59, ((4.0 * 1e-6) * 1e-6));

        s.store_offset_scaled_div(278, 59, 639, 0.5, 0.5);

        s.store_offset_scaled_add(598, 59, 639, 0.5, (1e-10 * 1e-6));

        s.b[890] = (s.v[598] < 0.0);
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if s.b[890] {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));

        s.store_powf(168, 168, p.p85);

        s.store_offset_scaled(282, 168, p.p84, 1.0);

        s.v[497] = (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301))));

        s.store_sub_scaled_inputs(288, 502, 1.0, 501, s.v[497]);

        s.store_add_scaled_inputs(283, 506, s.v[592], 288, s.v[591]);

        s.store_div(156, 283, 282);

        if (p.p32 != 0.0) {
            s.store_scaled_add(596, 306, 309, 0.5);
            s.store_scaled_add(597, 307, 310, 0.5);
            s.store_add_scaled_inputs3(163, s.ad_value(596), (3.9 * 1.0 / ((11.7 * s.v[507]))), s.ad_value(597), ((-1.0) * (3.9 * 1.0 / ((11.7 * s.v[507])))), s.ad_value(440), (-(3.9 * 1.0 / ((11.7 * s.v[507])))));
            s.store_add(156, 156, 163);
        }

        if (p.p32 == 0.0) {
            s.store_scalar(596, 0.0);
            s.store_scalar(597, 0.0);
            s.store_scalar(163, 0.0);
        }

        s.store_sqrt_square_offset(639, 156, ((4.0 * 3000.0) * 3000.0));

        s.store_offset_scaled_div(279, 156, 639, 0.5, 0.5);

        s.store_offset_scaled_add(156, 156, 639, 0.5, (1e-10 * 3000.0));

        s.b[891] = (s.v[156] < 0.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if s.b[891] {
            s.store_scalar(156, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 156, p.p94);

        s.store_powf(284, 156, s.v[470]);

        s.store_scale(157, 502, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[449] * 1e-11), s.v[448])), 1.0, s.ad_value(469), s.ad_value(286), 1.0), 284, 1.0 / (p.p105));

        s.store_div_from_scalar(159, 1.0, 279);

        s.store_scale(159, 159, 0.0001);

        if (p.p32 != 0.0) {
            s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (p.p32 == 0.0) {
            s.store_sqrt_square_offset(639, 155, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(278, 155, 639, 0.5, 0.5);
            s.store_offset_scaled_add(598, 155, 639, 0.5, (1e-10 * 1e-6));
        }

        s.b[892] = (s.v[598] < 0.0);
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if ((p.p32 == 0.0) && s.b[892]) {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (p.p32 == 0.0) {
            s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));
            s.store_powf(168, 168, p.p85);
            s.store_offset_scaled(282, 168, p.p84, 1.0);
            s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));
            s.store_add_scaled_product_indices(288, 503, 1.0, 498, 500, (-1.0));
            s.store_scaled_add(508, 505, 504, (-0.5));
            s.store_add_scaled_inputs(283, 508, s.v[592], 288, s.v[591]);
            s.store_div(163, 283, 282);
        }

        s.store_sqrt_square_offset(639, 163, ((4.0 * 30.0) * 30.0));

        s.store_offset_scaled_div(279, 163, 639, 0.5, 0.5);

        s.store_offset_scaled_add(163, 163, 639, 0.5, (1e-10 * 30.0));

        s.b[893] = (s.v[163] < 0.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if s.b[893] {
            s.store_scalar(163, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 163, p.p275);

        s.store_powf(284, 163, s.v[594]);

        s.store_scale(157, 503, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[451] * 1e-11), s.v[450])), 1.0, s.ad_value(595), s.ad_value(286), 1.0), 284, 1.0 / (p.p284));

        s.store_div_from_scalar(166, 1.0, 279);

        s.store_scale(166, 166, 0.0001);

        s.store_div_scaled_inputs(454, s.ad_value(162), 0.2, s.ad_value(159), 1.0);

        s.store_div_ad_rhs(291, 153, A::mul3(s.ad_value(120), A::offset(s.ad_value(149), 1e-50), s.ad_value(386)));

        s.store_sqrt_square_sum(160, 291, 454);

        s.store_mul(161, 159, 160);

        s.store_div(279, 161, 162);

        s.b[894] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if s.b[894] {
            s.store_scalar(281, 1.0);
        }

        s.b[895] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if ((!s.b[894]) && s.b[895]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[894]) && (!s.b[895])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[896] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        if s.b[896] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[897] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if ((!s.b[896]) && s.b[897]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[896]) && (!s.b[897])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(158, 159, 283);

        s.store_div_scaled_inputs(455, s.ad_value(162), 0.2, s.ad_value(166), 1.0);

        s.store_div_ad_rhs(291, 154, A::mul3(s.ad_value(120), A::offset(s.ad_value(150), 1e-50), s.ad_value(386)));

        s.store_sqrt_square_sum(164, 291, 455);

        s.store_mul(161, 166, 164);

        s.store_div(279, 161, 162);

        s.b[898] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scalar(281, 1.0);
        }

        s.b[899] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if ((!s.b[898]) && s.b[899]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[898]) && (!s.b[899])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[900] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if s.b[900] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[901] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if ((!s.b[900]) && s.b[901]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[900]) && (!s.b[901])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(165, 166, 283);

        s.store_div_scaled_inputs(189, s.ad_value(122), s.v[466], A::sub(s.ad_value(123), s.ad_value(262)), 1.0);

        s.store_mul3_lhs(96, 189, 153, 158);

        s.store_mul3_lhs(97, 189, 154, 165);

        s.store_add(95, 96, 97);

        s.v[173] = 0.0;

        s.v[169] = 0.0;

        s.v[171] = 0.0;

        s.v[172] = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[902] = (p.p239 != 0.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if s.b[902] {
            s.store_scaled_sub(279, 51, 59, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_ad(639, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(640, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_scaled_inputs(280, s.ad_value(640), (-2.0), A::square(s.ad_value(639)), 1.0);
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 279, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[903] = (s.v[280] < 0.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (s.b[902] && s.b[903]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[902] {
            s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);
            s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);
            s.store_mul(287, 287, 282);
        }

        if (!s.b[902]) {
            s.store_scalar(287, 0.0);
        }

        s.b[904] = (p.p246 != 0.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if s.b[904] {
            s.store_mul3_affine_lhs(286, 270, 120, s.v[477], 0.0, 71);
        }

        if (!s.b[904]) {
            s.store_scalar(286, 0.0);
        }

        s.b[905] = ((s.v[287] + s.v[286]) > 0.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if s.b[905] {
            s.store_mul_add_rhs(152, 59, 287, 286);
            s.store_mul3_lhs(173, 189, 152, 158);
            s.store_div_from_scalar_offset_ad(172, 1.0, A::exp_scaled_input(s.ad_value(440), (-p.p245)), 1.0);
            s.store_sub_from_scalar(171, 1.0, 172);
            s.store_mul(169, 171, 173);
        }

        s.v[174] = 0.0;

        s.v[170] = 0.0;

        s.b[906] = (p.p239 != 0.0);
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if s.b[906] {
            s.store_scaled_sub(279, 51, 155, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_ad(639, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(640, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_scaled_inputs(280, s.ad_value(640), (-2.0), A::square(s.ad_value(639)), 1.0);
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 279, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[907] = (s.v[280] < 0.0);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        if (s.b[906] && s.b[907]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[906] {
            s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);
            s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(322), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);
            s.store_mul(287, 287, 282);
        }

        if (!s.b[906]) {
            s.store_scalar(287, 0.0);
        }

        s.b[908] = ((s.v[287] + s.v[286]) > 0.0);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if s.b[908] {
            s.store_mul_add_rhs(152, 155, 287, 286);
            s.store_mul3_lhs(174, 189, 152, 165);
        }

        s.b[909] = ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0));
        s.v[909] = if s.b[909] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[909]) {
            s.store_add_scaled_inputs3(638, s.ad_value(174), 1.0, s.ad_value(173), (-1.0), s.ad_value(173), 0.05);
            s.store_square(642, 638);
            s.store_scaled_mul(643, 173, 173, (0.05 * 0.05));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[910] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[910] = if s.b[910] { 1.0 } else { 0.0 };

        s.b[911] = (2.0 == 1.0);
        s.v[911] = if s.b[911] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[909]) && s.b[910]) && s.b[911]) {
            s.store_scalar(648, 1.0);
        }

        s.b[912] = (2.0 == 2.0);
        s.v[912] = if s.b[912] { 1.0 } else { 0.0 };

        if ((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && s.b[912]) {
            s.store_scalar(648, 2.0);
        }

        s.b[913] = (2.0 == 4.0);
        s.v[913] = if s.b[913] { 1.0 } else { 0.0 };

        if (((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
            s.store_scalar(648, 3.0);
        }

        s.b[914] = (2.0 == 8.0);
        s.v[914] = if s.b[914] { 1.0 } else { 0.0 };

        if ((((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {
            s.store_scalar(648, 4.0);
        }

        if ((s.b[908] && s.b[909]) && s.b[910]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if (((s.b[908] && s.b[909]) && s.b[910]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[908] && s.b[909]) && s.b[910]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.b[908] && s.b[909]) && (!s.b[910])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[908] && s.b[909]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul3_affine_lhs(637, 638, 173, 0.05, 0.0, 646);
            s.store_div_scaled_product3_mixed_iiia(278, 173, 645, 646, 0.05, A::offset(s.ad_value(220), 1e-50), 1.0);
            s.store_add_scaled_inputs3(174, s.ad_value(173), 1.0, s.ad_value(173), (-0.05), s.ad_value(637), 1.0);
        }

        if (s.b[908] && s.b[909]) {
        }

        if (s.b[908] && (!s.b[909])) {
        }

        if (s.b[908] && (!s.b[909])) {
            s.store_scalar(278, 1.0);
        }

        if s.b[908] {
            s.store_mul(170, 172, 174);
        }

        s.store_add(175, 169, 170);

        s.store_add(94, 95, 175);

        s.b[915] = (p.p22 != 0.0);
        s.v[915] = if s.b[915] { 1.0 } else { 0.0 };

        if s.b[915] {
            s.store_scale(279, 271, 1.034943e-10);
            s.copy_ad(280, 132);
            s.store_scalar(281, (s.v[133] - p.p57));
            s.store_div_from_scalar_square_ad(282, 1.0, s.ad_value(281));
            s.store_mul_ad_product_lhs(283, A::mul_sub_from_scalar_lhs_scaled_output(p.p55, s.ad_value(130), s.ad_value(279), 2.0), s.ad_value(280), 282);
            s.store_mul(81, 283, 135);
            s.store_scalar(282, p.p158);
            s.store_scalar(284, p.p159);
            s.store_add_scaled_product_indices(279, 282, 1.0, 284, 71, 1.0);
            s.store_mul(98, 81, 279);
            s.store_sub_from_scalar_ad(279, p.p160, A::scale(s.ad_value(51), p.p161));
            s.store_add_scaled_inputs4(99, s.ad_value(72), 1.0, s.ad_value(138), (-1.0), s.ad_value(279), 1.0, s.ad_value(98), 1.0);
            s.store_mul3_lhs(102, 119, 271, 271);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_scale(387, 120, 0.25);
            s.store_offset_ad(288, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(102), s.ad_value(387), (-1.0)), 1.0, s.ad_value(138), 1.0, s.ad_value(98), -1.0, (-p.p160)), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[915] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[915] {
            s.store_sqrt_ad(280, A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(288), (4.0 * 0.005)));
            s.store_add_scaled_inputs3(281, A::offset(A::add_scaled_inputs4(s.ad_value(288), 1.0, s.ad_value(279), 0.5, s.ad_value(280), 0.5, s.ad_value(138), -1.0), p.p160), 1.0, s.ad_value(98), 1.0, s.ad_value(70), -1.0);
            s.store_offset_mul(282, 120, 281, (-1.0));
            s.store_div_from_scalar(283, 4.0, 104);
            s.store_offset_mul(279, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[916] = (s.v[279] < 0.0);
        s.v[916] = if s.b[916] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[916]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, 1e-50);
            s.store_sqrt(105, 279);
            s.store_mul_sub_from_scalar_rhs(278, 103, 1.0, 105);
            s.store_add(107, 99, 278);
            s.store_div_from_scalar_add_ad(278, 1.0, s.ad_value(120), A::div_scalar_offset_denominator(2.0, s.ad_value(99), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(109, A::mul(A::div_scalar_by_product(1.0, s.ad_value(101), s.ad_value(102), 1.0), A::square(s.ad_value(99))), 278);
            s.store_div_scaled_value_offset_denominator(281, s.ad_value(109), 1.0, s.ad_value(99), 1e-50, 1.0);
            s.store_offset_sub(110, 109, 107, (-p.p136));
            s.store_add_scaled_ad_lhs(278, A::square(s.ad_value(110)), 109, (4.0 * p.p136));
            s.store_sqrt_square_offset(639, 278, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(280, 278, 639, 0.5, 0.5);
            s.store_offset_scaled_add(278, 278, 639, 0.5, (1e-10 * 1e-6));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[917] = (s.v[278] < 0.0);
        s.v[917] = if s.b[917] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[917]) {
            s.store_scalar(278, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_sqrt(278, 278);
            s.store_add_scaled_inputs3(111, s.ad_value(109), 1.0, s.ad_value(110), (-0.5), s.ad_value(278), (-0.5));
            s.store_div_from_scalar(279, 1.0, 278);
            s.store_mul_exp_ad_rhs(278, 101, A::mul(s.ad_value(120), s.ad_value(111)));
            s.store_add_ad_lhs(279, A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0)), 278);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[918] = (s.v[279] < 0.0);
        s.v[918] = if s.b[918] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[918]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(113, 279);
            s.store_offset_mul_ad(279, s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70)), (-1.0));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[919] = (s.v[279] < 0.0);
        s.v[919] = if s.b[919] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[919]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(114, 279);
            s.store_mul_sub_rhs(115, 100, 113, 114);
            s.store_sub(279, 107, 111);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[920] = (s.v[279] < 0.0);
        s.v[920] = if s.b[920] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[920]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_div(290, 51, 279);
            s.store_square(642, 290);
            s.store_scalar(643, 1.0);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[921] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[921] = if s.b[921] { 1.0 } else { 0.0 };

        s.b[922] = (4.0 == 1.0);
        s.v[922] = if s.b[922] { 1.0 } else { 0.0 };

        if ((s.b[915] && s.b[921]) && s.b[922]) {
            s.store_scalar(648, 1.0);
        }

        s.b[923] = (4.0 == 2.0);
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if (((s.b[915] && s.b[921]) && (!s.b[922])) && s.b[923]) {
            s.store_scalar(648, 2.0);
        }

        s.b[924] = (4.0 == 4.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if ((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
            s.store_scalar(648, 3.0);
        }

        s.b[925] = (4.0 == 8.0);
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if (((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && (!s.b[924])) && s.b[925]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[915] && s.b[921]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if ((s.b[915] && s.b[921]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[915] && s.b[921]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[915] && (!s.b[921])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[915] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(291, 290, 646, 1.0);
            s.store_div_scaled_product_offset_denominator(280, s.ad_value(645), s.ad_value(646), 1.0, s.ad_value(220), 1e-50, 1.0);
            s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));
            s.copy_ad(279, 386);
            s.store_div_scaled_product_left_ad(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 291, 1.0, 279, 1.0);
            s.store_add(94, 94, 116);
        }

        s.b[926] = ((p.p20 != 0.0) && (p.p23 != 0.0));
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_square(231, 86);
            s.store_mul3_affine_lhs(232, 122, 271, 2.0, 0.0, 151);
            s.store_sub(233, 231, 232);
            s.store_sqrt_square_offset(639, 231, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 231, 639, 0.5, 0.5);
            s.store_offset_scaled_add(231, 231, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[927] = (s.v[231] < 0.0);
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[927]) {
            s.store_scalar(231, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sqrt_square_offset(639, 233, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 233, 639, 0.5, 0.5);
            s.store_offset_scaled_add(233, 233, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[928] = (s.v[233] < 0.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[928]) {
            s.store_scalar(233, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sub(234, 231, 233);
        }

        s.b[929] = ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16)));
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[929]) {
            s.store_scalar(35, 0.0);
        }

        if (s.b[926] && (!s.b[929])) {
            s.store_scalar(35, 1.0);
        }

        s.b[930] = (s.v[185] > 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_add_scaled_inputs3(283, s.ad_value(279), 1.0, s.ad_value(122), (-1.0), s.ad_value(70), (-s.v[486]));
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 284, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[931] = (s.v[284] < 0.0);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        if (s.b[930] && s.b[931]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))));
            s.store_add_scaled_inputs3(187, s.ad_value(71), p.p123, s.ad_value(339), 1.0, s.ad_value(186), (-(s.v[487] * s.v[485])));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_scaled_add(187, 187, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[932] = (s.v[187] < 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if (s.b[930] && s.b[932]) {
            s.store_scalar(187, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(187, 187, 1e-50);
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
            s.store_mul3_affine_lhs(185, 187, 94, s.v[488], 0.0, 280);
        }

        s.b[933] = (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0));
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if s.b[933] {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
            s.store_scaled_mul(188, 278, 185, p.p145);
            s.store_offset_mul(64, 120, 56, (-1.0));
            s.store_sqrt_square_offset(639, 64, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(64, 64, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[934] = (s.v[64] < 0.0);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        if (s.b[933] && s.b[934]) {
            s.store_scalar(64, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(65, 64);
            s.store_mul(66, 64, 65);
            s.store_offset_mul(69, 120, 57, (-1.0));
            s.store_sqrt_square_offset(639, 69, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(69, 69, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[935] = (s.v[69] < 0.0);
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[933] && s.b[935]) {
            s.store_scalar(69, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(67, 69);
            s.store_mul(68, 69, 67);
            s.store_div_scaled_product_indices(279, 120, 188, 1.0, 64, 1.0);
            s.store_div_scaled_product_indices(280, 120, 188, 1.0, 69, 1.0);
            s.store_mul_ad_rhs(190, 141, A::add_scaled_products(s.ad_value(68), s.ad_value(280), 1.0, s.ad_value(66), s.ad_value(279), (-1.0)));
            s.store_mul_scaled_ad_rhs(191, 141, 0.5, A::add_scaled_products(s.ad_value(67), s.ad_value(280), -1.0, s.ad_value(65), s.ad_value(279), 1.0));
            s.store_add(192, 190, 191);
            s.store_mul3_lhs(193, 189, 192, 158);
        }

        s.v[949] = (s.v[272] * 100.0);

        s.store_scale(951, 123, 100.0);

        s.v[952] = (s.v[466] * 100.0);

        s.store_scale(953, 160, 0.01);

        s.b[956] = (p.p17 == 0.0);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if s.b[956] {
            s.store_scalar(256, 0.0);
        }

        s.b[957] = (s.v[34] == 0.0);
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[957]) {
            s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4(938, s.ad_value(72), 1.0, s.ad_value(138), (-p.p256), A::div_scaled_inputs3(s.ad_value(50), (-p.p258), s.ad_value(80), p.p206, s.ad_value(267), (-p.p206), s.ad_value(951), 1.0), 1.0, s.ad_value(948), (-p.p205));
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[958] = (s.v[947] < 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[958]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_sqrt_square_offset(639, 72, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(941, 72, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 72, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[959] = (s.v[940] < 0.0);
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[959]) {
            s.store_scalar(940, 0.0);
            s.store_scalar(941, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_scaled_offset(936, 940, (-p.p216), 10.0);
            s.store_sub_from_scalar_ad(938, 1.0, A::div_scalar_offset_denominator(1.0, A::square(s.ad_value(936)), 1.0, 1.0));
            s.store_mul(947, 947, 938);
            s.store_scale(937, 951, s.v[952]);
            s.store_div_from_scalar_offset_input(944, p.p209, 937, p.p209);
            s.store_div_from_scalar_offset_ad(941, 1.0, A::square(s.ad_value(947)), 1e-50);
            s.store_scaled_mul(938, 246, 941, (-p.p204));
        }

        s.b[960] = (s.v[938] < (-34.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {
            s.store_mul_scale_ad_lhs(940, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 937);
        }

        if (!s.b[956]) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 52, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_sub(942, 52, 51);
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 942, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p261, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p215, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[963] = (s.v[947] < 0.0);
        s.v[963] = if s.b[963] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[963]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);
        }

        s.b[964] = (s.v[938] < (-34.0));
        s.v[964] = if s.b[964] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p264));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p265)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 638, 639, 0.5, p.p265);
            s.store_scale(940, 940, ((p.p213 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs(252, s.ad_value(940), A::powf(s.ad_value(947), p.p262), 939);
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p269, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p268, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[965] = (s.v[947] < 0.0);
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[965]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p267), s.ad_value(947), p.p271);
        }

        s.b[966] = (s.v[938] < (-34.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[966]) {
            s.store_scalar(253, 0.0);
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p272));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p273)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 638, 639, 0.5, p.p273);
            s.store_scale(940, 940, ((p.p266 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs(253, s.ad_value(940), A::powf(s.ad_value(947), p.p270), 939);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_scale(938, 252, (-0.001));
        }

        s.b[967] = (s.v[938] < 1e-50);
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[967]) {
            s.store_scalar(938, 1e-50);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_add_scaled_inputs3(638, s.ad_value(252), -1.0, s.ad_value(253), 1.0, s.ad_value(938), -1.0);
            s.store_scaled_mul(639, 253, 938, (-4.0));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
        }

        if (!s.b[956]) {
            s.store_scalar(256, 0.5);
        }

        s.b[968] = (p.p18 == 0.0);
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        if (!s.b[968]) {
            s.store_add_scaled_inputs4_offset(279, s.ad_value(51), p.p198, s.ad_value(52), (-1.0), s.ad_value(82), (-p.p200), s.ad_value(266), (-p.p200), (p.p199 * p.p198));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_scaled_add(248, 247, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[969] = (s.v[248] < 0.0);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if ((!s.b[968]) && s.b[969]) {
            s.store_scalar(248, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[968]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(248), 1e-50, 1.0);
        }

        s.b[970] = (s.v[278] < (-34.0));
        s.v[970] = if s.b[970] { 1.0 } else { 0.0 };

        if ((!s.b[968]) && (!s.b[970])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.b[971] = (p.p18 == 0.0);
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if (!s.b[971]) {
            s.store_add_scaled_inputs3(279, A::add_scaled_inputs3_offset(s.ad_value(51), (-p.p198), s.ad_value(52), -1.0, s.ad_value(51), 1.0, ((p.p199) * (p.p198))), 1.0, s.ad_value(82), (-p.p200), s.ad_value(266), (-p.p200));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_scaled_add(249, 247, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[972] = (s.v[249] < 0.0);
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        if ((!s.b[971]) && s.b[972]) {
            s.store_scalar(249, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[971]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(249), 1e-50, 1.0);
        }

        s.b[973] = (s.v[278] < (-34.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        if ((!s.b[971]) && (!s.b[973])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.v[264] = p.p176;

        s.v[261] = 0.0;

        s.b[974] = (s.v[34] != 0.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[974] {
            s.store_add(280, 51, 56);
            s.store_add_scaled_inputs(260, 280, s.v[264], 57, (1.0 - s.v[264]));
        }

        s.b[975] = (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16)));
        s.v[975] = if s.b[975] { 1.0 } else { 0.0 };

        if (s.b[974] && s.b[975]) {
            s.store_offset_add(260, 56, 51, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[976] = (p.p45 != 0.0);
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        s.b[977] = (s.v[151] > 1e-15);
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        if (((!s.b[974]) && s.b[976]) && s.b[977]) {
            s.store_div_scaled_product_by_product(261, s.ad_value(151), s.ad_value(122), 1.0, s.ad_value(123), s.ad_value(149), 1.0);
        }

        s.v[435] = s.v[273];

        s.v[436] = (1.0 / s.v[435]);

        s.b[978] = (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0));
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        if s.b[978] {
            s.store_scalar(195, p.p175);
            s.store_mul_sqrt_ad_rhs(437, 141, A::div_from_scalar(s.v[624], s.ad_value(457)));
            s.store_scalar(399, ((1.0 - -1.0) / 2.0));
            s.store_scalar(400, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);
            s.store_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);
        }

        if (s.b[978] && (s.v[399] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if (s.b[978] && (s.v[400] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if s.b[978] {
            s.store_scalar(415, 0.0);
            s.store_neg(278, 415);
        }

        s.b[979] = (s.v[278] > s.v[31]);
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[979]) {
            s.store_sub(279, 278, 31);
            s.store_sub_from_scalar(280, s.v[30], 31);
            s.store_div(638, 279, 280);
            s.store_square(639, 638);
            s.store_mul(640, 639, 638);
            s.store_square(641, 639);
            s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);
            s.store_mul_sub_from_scalar_rhs(291, 280, 1.0, 291);
            s.store_neg(387, 387);
            s.store_add(288, 31, 291);
        }

        if (s.b[978] && (!s.b[979])) {
            s.copy_ad(288, 278);
        }

        if s.b[978] {
            s.store_offset_scaled(416, 288, -1.0, (-1e-12));
            s.store_scale(144, 437, s.v[436]);
            s.store_square(145, 144);
            s.store_sub_from_scalar(404, p.p39, 414);
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
            s.store_neg(419, 416);
        }

        s.b[980] = (s.v[404] < s.v[419]);
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[980]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(291), 9.0, A::offset(s.ad_value(290), (-2.0))));
            s.store_square(181, 183);
        }

        s.b[981] = (s.v[182] < (s.v[181] * 1e-8));
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if ((s.b[978] && s.b[980]) && s.b[981]) {
            s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
        }

        if ((s.b[978] && s.b[980]) && (!s.b[981])) {
            s.store_sqrt_add(180, 182, 181);
            s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);
        }

        if (s.b[978] && s.b[980]) {
            s.store_powf(178, 179, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);
            s.store_div(77, 177, 178);
            s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);
            s.store_add(279, 259, 416);
            s.store_div(280, 279, 176);
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_scalar(77, 3.0);
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_offset_ad(290, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0), 1.0);
        }

        s.b[982] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[982]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_ad(290, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0), 1.0);
        }

        s.b[983] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[983]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[984] = (s.v[77] < 3.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[984]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(425, s.ad_value(404), -1.0, s.ad_value(416), -1.0, s.ad_value(144), 1.0);
            s.store_add_scaled_inputs3(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(425), 1.0, s.ad_value(421), 2.0), 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_ad(283, A::add_scaled_square_product(s.ad_value(426), 1.0, A::square(s.ad_value(424)), s.ad_value(424), 1.0));
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
            s.store_add_scaled_inputs3(290, s.ad_value(427), 1.0, s.ad_value(428), 1.0, A::div_scaled_inputs(s.ad_value(422), 1.0, s.ad_value(421), 3.0), -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[985] = (p.p30 > 0.0);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_offset_add(420, 404, 416, 0.1);
            s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(430, 429, 203);
            s.store_mul(278, 121, 145);
            s.store_mul(434, 120, 420);
            s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_offset_sub(638, 434, 433, (-1.0));
            s.store_scale(639, 434, 4.0);
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(433, s.ad_value(434), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
            s.copy_ad(431, 77);
            s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(77, s.ad_value(432), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[986] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[986]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[987] = (p.p30 == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
            s.store_scalar(379, 0.0);
            s.store_scalar(62, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign19230_loop_guard: usize = 0;
        while {
            let assign19230_cond_e23449: f64 = (40.0 + 1.0);
            let assign19230_cond_e23451: f64 = if (((s.b[978] && (!s.b[980])) && s.b[987]) && (s.v[62] <= assign19230_cond_e23449)) { 1.0 } else { 0.0 };
            assign19230_cond_e23451 != 0.0
        } {
            assign19230_loop_guard += 1;
            assert!(assign19230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_mul_add_rhs(77, 120, 410, 416);
            }
            s.b[988] = (s.v[77] < 5.0);
            s.v[988] = if s.b[988] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[988]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_ad_lhs(208, A::mul3_scaled_output(s.ad_value(204), s.ad_value(120), s.ad_value(205), 2.0), 206);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(148, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 2.0);
            }
            s.b[989] = (s.v[77] < 80.0);
            s.v[989] = if s.b[989] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && s.b[989]) {
                s.store_exp(147, 77);
                s.store_mul_offset_rhs(207, 204, 147, (-1.0));
                s.store_mul3_lhs(208, 204, 120, 147);
            }
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && (!s.b[989])) {
                s.store_exp_mul(202, 120, 410);
                s.store_mul_sub_rhs(207, 429, 202, 203);
                s.store_mul3_lhs(208, 429, 120, 202);
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) {
                s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));
                s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);
            }
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.b[990] = (s.v[379] == 1.0);
            s.v[990] = if s.b[990] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[990]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_div_scaled_inputs(213, s.ad_value(211), -1.0, s.ad_value(212), 1.0);
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[991] = (((s.v[213]) as f64).abs() > s.v[214]);
            s.v[991] = if s.b[991] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[991]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_add(410, 410, 213);
            }
            s.b[992] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.v[992] = if s.b[992] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[992]) {
                s.store_scalar(379, 1.0);
            }
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[994] = (s.v[77] < 5.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[994]) {
            s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[994])) {
            s.store_offset(64, 77, (-1.0));
            s.store_sqrt(65, 64);
        }

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_mul(407, 437, 65);
            s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));
            s.store_mul3_lhs(409, 437, 207, 279);
            s.store_add(408, 407, 409);
        }

        if s.b[978] {
            s.store_sub(409, 408, 407);
            s.store_scale(282, 195, s.v[513]);
        }

        if (s.b[978] && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
            s.store_mul(406, 282, 407);
        }

        if (s.b[978] && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
            s.store_mul(405, 282, 407);
        }

        if s.b[978] {
            s.store_scalar(399, ((1.0 - 1.0) / 2.0));
            s.store_scalar(400, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);
            s.store_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);
        }

        if (s.b[978] && (s.v[399] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if (s.b[978] && (s.v[400] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if s.b[978] {
            s.store_scalar(415, 0.0);
            s.store_neg(278, 415);
        }

        s.b[996] = (s.v[278] > s.v[31]);
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[996]) {
            s.store_sub(279, 278, 31);
            s.store_sub_from_scalar(280, s.v[30], 31);
            s.store_div(638, 279, 280);
            s.store_square(639, 638);
            s.store_mul(640, 639, 638);
            s.store_square(641, 639);
            s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);
            s.store_mul_sub_from_scalar_rhs(291, 280, 1.0, 291);
            s.store_neg(387, 387);
            s.store_add(288, 31, 291);
        }

        if (s.b[978] && (!s.b[996])) {
            s.copy_ad(288, 278);
        }

        if s.b[978] {
            s.store_offset_scaled(416, 288, -1.0, (-1e-12));
            s.store_scale(144, 437, s.v[436]);
            s.store_square(145, 144);
            s.store_sub_from_scalar(404, p.p39, 414);
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
            s.store_neg(419, 416);
        }

        s.b[997] = (s.v[404] < s.v[419]);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[997]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(291), 9.0, A::offset(s.ad_value(290), (-2.0))));
            s.store_square(181, 183);
        }

        s.b[998] = (s.v[182] < (s.v[181] * 1e-8));
        s.v[998] = if s.b[998] { 1.0 } else { 0.0 };

        if ((s.b[978] && s.b[997]) && s.b[998]) {
            s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
        }

        if ((s.b[978] && s.b[997]) && (!s.b[998])) {
            s.store_sqrt_add(180, 182, 181);
            s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);
        }

        if (s.b[978] && s.b[997]) {
            s.store_powf(178, 179, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);
            s.store_div(77, 177, 178);
            s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);
            s.store_add(279, 259, 416);
            s.store_div(280, 279, 176);
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_scalar(77, 3.0);
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_offset_ad(290, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0), 1.0);
        }

        s.b[999] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[999]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_ad(290, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0), 1.0);
        }

        s.b[1000] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1000]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1001] = (s.v[77] < 3.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(425, s.ad_value(404), -1.0, s.ad_value(416), -1.0, s.ad_value(144), 1.0);
            s.store_add_scaled_inputs3(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(425), 1.0, s.ad_value(421), 2.0), 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_ad(283, A::add_scaled_square_product(s.ad_value(426), 1.0, A::square(s.ad_value(424)), s.ad_value(424), 1.0));
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
            s.store_add_scaled_inputs3(290, s.ad_value(427), 1.0, s.ad_value(428), 1.0, A::div_scaled_inputs(s.ad_value(422), 1.0, s.ad_value(421), 3.0), -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1002] = (p.p30 > 0.0);
        s.v[1002] = if s.b[1002] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_offset_add(420, 404, 416, 0.1);
            s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(430, 429, 203);
            s.store_mul(278, 121, 145);
            s.store_mul(434, 120, 420);
            s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_offset_sub(638, 434, 433, (-1.0));
            s.store_scale(639, 434, 4.0);
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(433, s.ad_value(434), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
            s.copy_ad(431, 77);
            s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(77, s.ad_value(432), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[1003] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.v[1003] = if s.b[1003] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1003]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[1004] = (p.p30 == 1.0);
        s.v[1004] = if s.b[1004] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
            s.store_scalar(379, 0.0);
            s.store_scalar(62, 1.0);
        }

        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (s.v[62] <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_mul_add_rhs(77, 120, 410, 416);
            }
            s.b[1005] = (s.v[77] < 5.0);
            s.v[1005] = if s.b[1005] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1005]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_ad_lhs(208, A::mul3_scaled_output(s.ad_value(204), s.ad_value(120), s.ad_value(205), 2.0), 206);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(148, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 2.0);
            }
            s.b[1006] = (s.v[77] < 80.0);
            s.v[1006] = if s.b[1006] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && s.b[1006]) {
                s.store_exp(147, 77);
                s.store_mul_offset_rhs(207, 204, 147, (-1.0));
                s.store_mul3_lhs(208, 204, 120, 147);
            }
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && (!s.b[1006])) {
                s.store_exp_mul(202, 120, 410);
                s.store_mul_sub_rhs(207, 429, 202, 203);
                s.store_mul3_lhs(208, 429, 120, 202);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) {
                s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));
                s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);
            }
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.b[1007] = (s.v[379] == 1.0);
            s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1007]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_div_scaled_inputs(213, s.ad_value(211), -1.0, s.ad_value(212), 1.0);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1008] = (((s.v[213]) as f64).abs() > s.v[214]);
            s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1008]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_add(410, 410, 213);
            }
            s.b[1009] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1009]) {
                s.store_scalar(379, 1.0);
            }
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[1011] = (s.v[77] < 5.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1011]) {
            s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1011])) {
            s.store_offset(64, 77, (-1.0));
            s.store_sqrt(65, 64);
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_mul(407, 437, 65);
            s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));
            s.store_mul3_lhs(409, 437, 207, 279);
            s.store_add(408, 407, 409);
        }

        if s.b[978] {
            s.store_sub(409, 408, 407);
            s.store_scale(282, 195, s.v[513]);
        }

        if (s.b[978] && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
            s.store_mul(406, 282, 407);
        }

        if (s.b[978] && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
            s.store_mul(405, 282, 407);
        }

        if s.b[978] {
            s.store_add_scaled_inputs(194, 413, s.v[519], 412, s.v[518]);
        }

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
            s.store_offset_ad(197, A::mul_scaled_lhs(s.ad_value(198), -1.0, A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);
        }

        if s.b[978] {
            s.store_add_scaled_inputs(194, 412, s.v[519], 413, s.v[518]);
        }

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
            s.store_offset_scaled_mul(196, 199, 52, -1.0, s.v[196]);
        }

        s.b[1013] = (((s.v[575] == 1.0) && (!s.b[518])) || ((s.v[575] != 1.0) && (!s.b[519])));
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        s.b[1014] = (p.p175 > 0.0);
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if (((!s.b[978]) && s.b[1013]) && s.b[1014]) {
            s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if (((!s.b[978]) && s.b[1013]) && (!s.b[1014])) {
            s.store_scalar(198, 0.0);
        }

        if ((!s.b[978]) && (!s.b[1013])) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_scaled_ad_rhs(197, 198, -1.0, A::sub(s.ad_value(52), s.ad_value(51)));
        }

        s.b[1015] = (((s.v[575] == 1.0) && (!s.b[519])) || ((s.v[575] != 1.0) && (!s.b[518])));
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if ((!s.b[978]) && s.b[1015]) {
            s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if ((!s.b[978]) && (!s.b[1015])) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_neg_lhs(196, 199, 52);
        }

        s.b[1016] = (s.v[34] == 0.0);
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1016]) {
            s.store_scaled_mul(279, 386, 386, (p.p223 * p.p224));
            s.store_offset_ad(280, A::add_scaled_products(s.ad_value(158), s.ad_value(86), p.p223, s.ad_value(386), s.ad_value(386), p.p224), 1e-50);
            s.store_div(221, 279, 280);
        }

        if ((s.v[38] != 0.0) && (!s.b[1016])) {
            s.store_scalar(221, (p.p223 + 1e-50));
        }

        if (s.v[38] != 0.0) {
            s.store_scale(222, 270, (p.p225 * 0.0001));
        }

        s.b[1017] = ((p.p21 != 0.0) && (s.v[34] == 0.0));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if s.b[1017] {
            s.store_scalar(223, s.v[617]);
            s.store_scalar(225, s.v[619]);
            s.store_scale(279, 149, 6.241449993689894e18);
            s.store_mul_scaled_ad_lhs(280, A::add_scaled_inputs3(s.ad_value(270), 1.0, A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50))), 1.0, s.ad_value(225), 1.0), 122, 6.241449993689894e18);
            s.store_sub_ad_lhs(281, A::div_scaled_inputs(s.ad_value(91), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[513])), s.ad_value(386), 1.0), 279);
        }

        s.b[1018] = ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if (s.b[1017] && s.b[1018]) {
            let assign21170_ad_e26697: A = A::add_scaled_product(A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::sub(s.ad_value(281), s.ad_value(279)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(281), 1.0, s.ad_value(280), 1.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0)), 1.0);
            s.store_add_scaled_product_mixed_aai(282, assign21170_ad_e26697, 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);
        }

        if (s.b[1017] && (!s.b[1018])) {
            s.store_add_scaled_inputs_product_mixed_aaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);
        }

        s.b[1019] = ((p.p23 != 0.0) && (s.v[34] == 0.0));
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if s.b[1019] {
            s.store_div_scaled_inputs2(227, s.ad_value(260), 1.0, s.ad_value(56), (-1.0), s.ad_value(386), 1.0);
            s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));
        }

        s.b[1020] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(285, 1.0);
        }

        s.b[1021] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.copy_ad(285, 289);
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_powf(285, 289, (p.p114 - 1.0));
        }

        if s.b[1019] {
            s.store_offset_mul(287, 289, 285, 1.0);
            s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));
            s.store_mul3_lhs(230, 159, 287, 288);
            s.store_scaled_add(228, 158, 230, 0.5);
            s.store_square(278, 85);
        }

        if s.b[1019] {
            let assign21340_ad_e26942: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 3.0, 1.0), 1.0, s.ad_value(278), 6.0), s.ad_value(230), s.ad_value(230)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 4.0, 3.0), 1.0, s.ad_value(278), 3.0), s.ad_value(230), s.ad_value(158)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(85), 3.0, 6.0), s.ad_value(278)), s.ad_value(158), s.ad_value(158)), 1.0);
            s.store_div_scaled_product_by_product(229, A::mul3_scaled_output(s.ad_value(270), s.ad_value(86), s.ad_value(158), s.v[466]), assign21340_ad_e26942, 1.0, A::mul3_scaled_output(s.ad_value(386), A::offset(s.ad_value(85), 1.0), s.ad_value(228), 15.0), s.ad_value(228), 1.0);
        }

        if (!s.b[1019]) {
            s.store_scalar(229, 0.0);
        }

        s.b[1022] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if s.b[1022] {
            s.store_sqrt(235, 233);
            s.store_add(280, 86, 235);
            s.store_square(281, 231);
            s.store_square(282, 233);
            s.store_scaled_mul(283, 231, 233, 42.0);
            s.store_add_scaled_inputs3(283, s.ad_value(283), 1.0, s.ad_value(281), 4.0, s.ad_value(282), 4.0);
            s.store_add_ad_rhs(283, 283, A::mul3_scaled_output(s.ad_value(235), s.ad_value(86), A::add(s.ad_value(231), s.ad_value(233)), 20.0));
            s.store_square(288, 280);
            s.store_div_ad_rhs(236, 283, A::mul(A::square(s.ad_value(288)), s.ad_value(280)));
            s.store_mul_ad_product_lhs(237, A::div_from_scalar(s.v[466], s.ad_value(386)), s.ad_value(158), 270);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1022] {
            s.store_add_ad_lhs(285, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 4.0), 233);
        }

        s.store_add(94, 94, 193);

        if s.b[517] {
            s.store_scalar(200, ((-p.p172) * s.v[277]));
            s.store_mul_sub_rhs(201, 200, 42, 40);
        }

        if (!s.b[517]) {
            s.store_scalar(200, 0.0);
            s.store_scalar(201, 0.0);
        }

        s.v[215] = 0.0;

        s.store_scaled_sub(216, 42, 41, s.v[215]);

        s.store_scale(217, 42, s.v[215]);

        s.store_add(197, 197, 216);

        s.store_add(196, 196, 217);

        s.store_scale(0, 94, s.v[394]);

        s.store_scale(279, 123, (-s.v[513]));

        s.store_scaled_add(280, 523, 576, (-0.5));

        s.store_scaled_add(281, 531, 585, (-0.5));

        s.store_scaled_mul(444, 279, 40, (0.1 * s.v[294]));

        s.store_mul_scaled_ad_rhs(443, 279, (0.1 * s.v[294]), A::sub(s.ad_value(40), s.ad_value(41)));

        s.store_mul(441, 279, 280);

        s.store_mul(442, 279, 281);

        if (p.p303 != 0.0) {
            s.store_scalar(336, 0.0);
            s.copy_ad(92, 91);
        }

        if (p.p303 == 0.0) {
            s.store_add_scaled_inputs3(92, s.ad_value(91), 1.0, s.ad_value(441), 1.0, s.ad_value(442), 1.0);
        }

        s.store_scale(93, 92, s.v[385]);

        if (s.v[38] != 0.0) {
            s.store_scalar(15, 0.0);
            s.store_scalar(14, 0.0);
            s.store_scalar(492, 0.0);
            s.store_scale(556, 336, s.v[394]);
            s.store_scale(555, 92, s.v[394]);
        }

        if (s.v[38] == 0.0) {
            s.store_sub_scaled_inputs(14, 336, (-s.v[394]), 92, s.v[394]);
            s.store_scaled_add(15, 93, 443, s.v[394]);
            s.store_add_scaled_inputs3(16, s.ad_value(92), s.v[394], s.ad_value(93), ((-1.0) * s.v[394]), s.ad_value(444), s.v[394]);
        }

        s.b[1023] = (p.p45 == 0.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if s.b[1023] {
            s.store_scalar(219, 0.0);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_product_indices(218, 56, 1.0, 261, 123, 1.0);
        }

        s.b[1024] = (s.v[218] > s.v[260]);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if ((!s.b[1023]) && s.b[1024]) {
            s.copy_ad(218, 260);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_inputs3(279, s.ad_value(51), s.v[264], s.ad_value(56), s.v[264], s.ad_value(218), (1.0 - s.v[264]));
            s.store_sqrt_div_from_scalar_ad(288, (2.0 * 1.034943e-10), s.ad_value(126));
            s.store_scale(281, 288, 1.3);
            s.store_scale(280, 281, (1.034943e-10 * s.v[513]));
            s.store_mul_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(56), 1.0 / (p.p45), s.ad_value(51), 1.0 / (p.p45), s.ad_value(279), (-1.0 / (p.p45)), s.ad_value(261), -1.0), 280);
        }

        s.b[1025] = (p.p46 != 0.0);
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if s.b[1025] {
            s.store_add_scaled_inputs(219, 219, 1.0, 50, s.v[490]);
        }

        s.b[1026] = (p.p14 == 1.0);
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if s.b[1026] {
            s.store_add_ad_rhs(14, 14, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(197), 1.0, s.ad_value(196), 1.0, s.ad_value(201), -1.0, s.ad_value(219), -1.0), s.ad_value(398)), s.v[394], s.ad_value(397), s.v[394]));
            s.store_add_scaled_inputs4(15, s.ad_value(15), 1.0, s.ad_value(219), s.v[394], s.ad_value(197), ((-1.0) * s.v[394]), s.ad_value(405), s.v[394]);
            s.store_add_scaled_inputs3(16, s.ad_value(16), 1.0, s.ad_value(406), s.v[394], s.ad_value(196), (-s.v[394]));
        }

        s.store_scale(494, 185, s.v[394]);

        s.b[1027] = (s.v[575] == 1.0);
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (!s.b[1027]) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.b[1028] = (s.v[575] == 1.0);
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if s.b[1028] {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.store_scale(573, 374, (4.0 * 1.3806226e-23));

        s.store_scale(564, 229, s.v[394]);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }

        s.b[1029] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if s.b[1029] {
            s.store_scaled_mul(278, 270, 123, (1e-6 * s.v[513]));
            s.store_scale(288, 493, 1.0 / (s.v[394]));
            s.store_div_scaled_product3_indices(241, 122, 288, 288, (0.1185185185185185 * 1.6021918e-19), 237, 1.0);
        }

        s.b[1030] = ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16)));
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[1029] && s.b[1030]) {
            s.store_div(242, 159, 158);
            s.store_div_scaled_inputs2(243, A::div(s.ad_value(159), s.ad_value(230)), 1.0, s.ad_value(242), (-1.0), s.ad_value(51), 1.0);
            s.store_add_ad_rhs(244, 242, A::div_scaled_product(s.ad_value(243), A::add(A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 1.0), s.ad_value(233)), 0.6666666666666667, A::add(s.ad_value(86), s.ad_value(235)), 1.0));
        }

        if (s.b[1029] && (!s.b[1030])) {
            s.store_div(244, 159, 230);
        }

        if s.b[1029] {
            s.store_mul3_affine_lhs(495, 241, 236, s.v[394], 0.0, 244);
        }

        if s.b[1029] {
            if (s.v[495] < 0.0) {
                s.store_scalar(495, 0.0);
            } else {
            }
        }

        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(495, 0.0);
            }
        }

        if (!s.b[1029]) {
            s.store_scalar(495, 0.0);
        }

        s.store_mul(608, 573, 564);

        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_div(610, 495, 608);
        } else {
            s.store_scalar(610, 0.0);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }

        s.b[1031] = (p.p312 == 1.0);
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if s.b[1031] {
            s.store_scalar(1035, p.p317);
            s.store_scalar(1036, p.p319);
            s.store_scalar(1037, p.p324);
            s.store_scalar(1041, p.p311);
            s.store_scaled_voltage(1039, ctx, nodes, Some(12), Some(2), p.p33);
            s.store_scale(1035, 1035, 0.0001);
            s.store_scale(1036, 1036, 0.01);
            s.store_scale(1040, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1040, p.p320);
            s.store_div(1043, 1035, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1040), 0.4, 1.8), 1.0, s.ad_value(1040), s.ad_value(1040), 0.1), A::scale_offset(s.ad_value(1040), (-p.p321), p.p321));
            s.store_div(1044, 1036, 278);
            s.store_add_ad_rhs(1037, 1037, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1043, 1043, 1032);
            s.store_offset_ad(1044, A::mul3(s.ad_value(1044), s.ad_value(1033), s.ad_value(1034)), 1e-50);
            s.store_div(1045, 1039, 1041);
            s.store_mul(1046, 1043, 1045);
        }

        s.b[1051] = (s.v[1039] >= 0.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1051]) {
            s.store_div(279, 1046, 1044);
        }

        if (s.b[1031] && (!s.b[1051])) {
            s.store_div_scaled_inputs(279, s.ad_value(1046), -1.0, s.ad_value(1044), 1.0);
        }

        s.b[1052] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1052]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1053] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((s.b[1031] && (!s.b[1052])) && s.b[1053]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1031] && (!s.b[1052])) && (!s.b[1053])) {
            s.store_pow_ad(281, s.ad_value(279), A::offset(s.ad_value(1037), (-1.0)));
        }

        if s.b[1031] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1054] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1054]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1055] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((s.b[1031] && (!s.b[1054])) && s.b[1055]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1031] && (!s.b[1054])) && (!s.b[1055])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1031] {
            s.store_div_from_scalar(279, 1.6021918e-19, 1041);
        }

        s.b[1058] = (p.p313 == 1.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if s.b[1058] {
            s.store_scalar(1062, p.p316);
            s.store_scalar(1063, p.p318);
            s.store_scalar(1064, p.p323);
            s.store_scalar(1068, p.p310);
            s.store_scaled_voltage(1066, ctx, nodes, Some(0), Some(11), p.p33);
            s.store_scale(1062, 1062, 0.0001);
            s.store_scale(1063, 1063, 0.01);
            s.store_scale(1067, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1067, p.p320);
            s.store_div(1070, 1062, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1067), 0.4, 1.8), 1.0, s.ad_value(1067), s.ad_value(1067), 0.1), A::scale_offset(s.ad_value(1067), (-p.p321), p.p321));
            s.store_div(1071, 1063, 278);
            s.store_add_ad_rhs(1064, 1064, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1070, 1070, 1059);
            s.store_offset_ad(1071, A::mul3(s.ad_value(1071), s.ad_value(1060), s.ad_value(1061)), 1e-50);
            s.store_div(1072, 1066, 1068);
            s.store_mul(1073, 1070, 1072);
        }

        s.b[1078] = (s.v[1066] >= 0.0);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1078]) {
            s.store_div(279, 1073, 1071);
        }

        if (s.b[1058] && (!s.b[1078])) {
            s.store_div_scaled_inputs(279, s.ad_value(1073), -1.0, s.ad_value(1071), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1079] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1079]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1080] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if ((s.b[1058] && (!s.b[1079])) && s.b[1080]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1058] && (!s.b[1079])) && (!s.b[1080])) {
            s.store_pow_ad(281, s.ad_value(279), A::offset(s.ad_value(1064), (-1.0)));
        }

        if s.b[1058] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1081] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1081]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1082] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if ((s.b[1058] && (!s.b[1081])) && s.b[1082]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1058] && (!s.b[1081])) && (!s.b[1082])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1058] {
            s.store_div_from_scalar(279, 1.6021918e-19, 1068);
        }

        s.b[1085] = (s.v[221] < 1e-18);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1085]) {
            s.store_scalar(221, 1e-18);
        }

        s.b[1086] = (s.v[222] < 1e-18);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1086]) {
            s.store_scalar(222, 1e-18);
        }

        if (s.v[38] != 0.0) {
            s.store_div_scaled_inputs2(549, s.ad_value(551), 1.0, s.ad_value(555), (-1.0), s.ad_value(221), 1.0);
            s.store_div_scaled_inputs2(550, s.ad_value(548), 1.0, s.ad_value(556), (-1.0), s.ad_value(222), 1.0);
            s.store_sub_scaled_inputs(554, 551, -1.0, 548, 1.0);
            s.store_scale(552, 551, s.v[385]);
            s.store_scale(553, 551, (1.0 - s.v[385]));
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(549, 0.0);
            s.store_scalar(550, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.b[1087] = (s.v[575] == 1.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if s.b[1087] {
            s.copy_ad(94, 0);
            s.copy_ad(185, 494);
            s.copy_ad(561, 14);
            s.copy_ad(93, 15);
            s.store_neg_ad(492, A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0));
            s.copy_ad(90, 492);
        }

        if (!s.b[1087]) {
            s.store_neg(94, 0);
            s.store_scalar(185, 0.0);
            s.copy_ad(561, 14);
            s.copy_ad(93, 16);
            s.store_neg_ad(492, A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0));
            s.copy_ad(90, 492);
            s.copy_ad(16, 15);
            s.copy_ad(15, 93);
        }

        if ((!s.b[1087]) && (s.v[38] != 0.0)) {
            s.copy_ad(279, 552);
            s.copy_ad(552, 553);
            s.copy_ad(553, 279);
        }

        s.b[1088] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if s.b[1088] {
            s.store_mul(547, 0, 51);
            s.store_scalar(516, s.v[468]);
            s.store_scalar(557, (1.0 / s.v[467]));
        }

        if (!s.b[1088]) {
            s.store_scalar(547, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(557, 0.0);
        }

        s.copy_ad(0, 94);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        s.b[1094] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        s.b[1095] = (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0));
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq0_e342: f64 = (p.p33 * s.v[94]);
        let eq0_e342_d_n0: f64 = (p.p33 * s.dn[94][0]);
        let eq0_e342_d_n1: f64 = (p.p33 * s.dn[94][1]);
        let eq0_e342_d_n2: f64 = (p.p33 * s.dn[94][2]);
        let eq0_e342_d_n3: f64 = (p.p33 * s.dn[94][3]);
        let eq0_e342_d_n4: f64 = (p.p33 * s.dn[94][4]);
        let eq0_e342_d_n5: f64 = (p.p33 * s.dn[94][5]);
        let eq0_e342_d_n6: f64 = (p.p33 * s.dn[94][6]);
        let eq0_e342_d_n7: f64 = (p.p33 * s.dn[94][7]);
        let eq0_e342_d_n8: f64 = (p.p33 * s.dn[94][8]);
        let eq0_e342_d_n9: f64 = (p.p33 * s.dn[94][9]);
        let eq0_e342_d_n10: f64 = (p.p33 * s.dn[94][10]);
        let eq0_e342_d_n11: f64 = (p.p33 * s.dn[94][11]);
        let eq0_e342_d_n12: f64 = (p.p33 * s.dn[94][12]);
        let eq0_e342_d_b0: f64 = (p.p33 * s.db[94][0]);
        let eq0_e342_d_b1: f64 = (p.p33 * s.db[94][1]);
        let eq0_e342_d_b2: f64 = (p.p33 * s.db[94][2]);
        let eq0_e342_d_b3: f64 = (p.p33 * s.db[94][3]);
        let eq0_e342_d_b4: f64 = (p.p33 * s.db[94][4]);
        let eq0_e342_d_b5: f64 = (p.p33 * s.db[94][5]);
        let eq0_e342_d_b6: f64 = (p.p33 * s.db[94][6]);
        let eq0_e342_d_b7: f64 = (p.p33 * s.db[94][7]);
        let eq0_value: f64 = eq0_e342;
        let eq0_node_derivatives: [f64; 13] = [eq0_e342_d_n0, eq0_e342_d_n1, eq0_e342_d_n2, eq0_e342_d_n3, eq0_e342_d_n4, eq0_e342_d_n5, eq0_e342_d_n6, eq0_e342_d_n7, eq0_e342_d_n8, eq0_e342_d_n9, eq0_e342_d_n10, eq0_e342_d_n11, eq0_e342_d_n12];
        let eq0_branch_derivatives: [f64; 8] = [eq0_e342_d_b0, eq0_e342_d_b1, eq0_e342_d_b2, eq0_e342_d_b3, eq0_e342_d_b4, eq0_e342_d_b5, eq0_e342_d_b6, eq0_e342_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e346: f64 = (s.v[257] + s.v[185]);
        let eq1_e346_d_n0: f64 = (s.dn[257][0] + s.dn[185][0]);
        let eq1_e346_d_n1: f64 = (s.dn[257][1] + s.dn[185][1]);
        let eq1_e346_d_n2: f64 = (s.dn[257][2] + s.dn[185][2]);
        let eq1_e346_d_n3: f64 = (s.dn[257][3] + s.dn[185][3]);
        let eq1_e346_d_n4: f64 = (s.dn[257][4] + s.dn[185][4]);
        let eq1_e346_d_n5: f64 = (s.dn[257][5] + s.dn[185][5]);
        let eq1_e346_d_n6: f64 = (s.dn[257][6] + s.dn[185][6]);
        let eq1_e346_d_n7: f64 = (s.dn[257][7] + s.dn[185][7]);
        let eq1_e346_d_n8: f64 = (s.dn[257][8] + s.dn[185][8]);
        let eq1_e346_d_n9: f64 = (s.dn[257][9] + s.dn[185][9]);
        let eq1_e346_d_n10: f64 = (s.dn[257][10] + s.dn[185][10]);
        let eq1_e346_d_n11: f64 = (s.dn[257][11] + s.dn[185][11]);
        let eq1_e346_d_n12: f64 = (s.dn[257][12] + s.dn[185][12]);
        let eq1_e346_d_b0: f64 = (s.db[257][0] + s.db[185][0]);
        let eq1_e346_d_b1: f64 = (s.db[257][1] + s.db[185][1]);
        let eq1_e346_d_b2: f64 = (s.db[257][2] + s.db[185][2]);
        let eq1_e346_d_b3: f64 = (s.db[257][3] + s.db[185][3]);
        let eq1_e346_d_b4: f64 = (s.db[257][4] + s.db[185][4]);
        let eq1_e346_d_b5: f64 = (s.db[257][5] + s.db[185][5]);
        let eq1_e346_d_b6: f64 = (s.db[257][6] + s.db[185][6]);
        let eq1_e346_d_b7: f64 = (s.db[257][7] + s.db[185][7]);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n1: f64 = (p.p33 * eq1_e346_d_n1);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n3: f64 = (p.p33 * eq1_e346_d_n3);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n7: f64 = (p.p33 * eq1_e346_d_n7);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n9: f64 = (p.p33 * eq1_e346_d_n9);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_e347_d_b0: f64 = (p.p33 * eq1_e346_d_b0);
        let eq1_e347_d_b1: f64 = (p.p33 * eq1_e346_d_b1);
        let eq1_e347_d_b2: f64 = (p.p33 * eq1_e346_d_b2);
        let eq1_e347_d_b3: f64 = (p.p33 * eq1_e346_d_b3);
        let eq1_e347_d_b4: f64 = (p.p33 * eq1_e346_d_b4);
        let eq1_e347_d_b5: f64 = (p.p33 * eq1_e346_d_b5);
        let eq1_e347_d_b6: f64 = (p.p33 * eq1_e346_d_b6);
        let eq1_e347_d_b7: f64 = (p.p33 * eq1_e346_d_b7);
        let eq1_value: f64 = eq1_e347;
        let eq1_node_derivatives: [f64; 13] = [eq1_e347_d_n0, eq1_e347_d_n1, eq1_e347_d_n2, eq1_e347_d_n3, eq1_e347_d_n4, eq1_e347_d_n5, eq1_e347_d_n6, eq1_e347_d_n7, eq1_e347_d_n8, eq1_e347_d_n9, eq1_e347_d_n10, eq1_e347_d_n11, eq1_e347_d_n12];
        let eq1_branch_derivatives: [f64; 8] = [eq1_e347_d_b0, eq1_e347_d_b1, eq1_e347_d_b2, eq1_e347_d_b3, eq1_e347_d_b4, eq1_e347_d_b5, eq1_e347_d_b6, eq1_e347_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e351: f64 = (s.v[258] + s.v[546]);
        let eq2_e351_d_n0: f64 = (s.dn[258][0] + s.dn[546][0]);
        let eq2_e351_d_n1: f64 = (s.dn[258][1] + s.dn[546][1]);
        let eq2_e351_d_n2: f64 = (s.dn[258][2] + s.dn[546][2]);
        let eq2_e351_d_n3: f64 = (s.dn[258][3] + s.dn[546][3]);
        let eq2_e351_d_n4: f64 = (s.dn[258][4] + s.dn[546][4]);
        let eq2_e351_d_n5: f64 = (s.dn[258][5] + s.dn[546][5]);
        let eq2_e351_d_n6: f64 = (s.dn[258][6] + s.dn[546][6]);
        let eq2_e351_d_n7: f64 = (s.dn[258][7] + s.dn[546][7]);
        let eq2_e351_d_n8: f64 = (s.dn[258][8] + s.dn[546][8]);
        let eq2_e351_d_n9: f64 = (s.dn[258][9] + s.dn[546][9]);
        let eq2_e351_d_n10: f64 = (s.dn[258][10] + s.dn[546][10]);
        let eq2_e351_d_n11: f64 = (s.dn[258][11] + s.dn[546][11]);
        let eq2_e351_d_n12: f64 = (s.dn[258][12] + s.dn[546][12]);
        let eq2_e351_d_b0: f64 = (s.db[258][0] + s.db[546][0]);
        let eq2_e351_d_b1: f64 = (s.db[258][1] + s.db[546][1]);
        let eq2_e351_d_b2: f64 = (s.db[258][2] + s.db[546][2]);
        let eq2_e351_d_b3: f64 = (s.db[258][3] + s.db[546][3]);
        let eq2_e351_d_b4: f64 = (s.db[258][4] + s.db[546][4]);
        let eq2_e351_d_b5: f64 = (s.db[258][5] + s.db[546][5]);
        let eq2_e351_d_b6: f64 = (s.db[258][6] + s.db[546][6]);
        let eq2_e351_d_b7: f64 = (s.db[258][7] + s.db[546][7]);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n1: f64 = (p.p33 * eq2_e351_d_n1);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n3: f64 = (p.p33 * eq2_e351_d_n3);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n7: f64 = (p.p33 * eq2_e351_d_n7);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n9: f64 = (p.p33 * eq2_e351_d_n9);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_e352_d_b0: f64 = (p.p33 * eq2_e351_d_b0);
        let eq2_e352_d_b1: f64 = (p.p33 * eq2_e351_d_b1);
        let eq2_e352_d_b2: f64 = (p.p33 * eq2_e351_d_b2);
        let eq2_e352_d_b3: f64 = (p.p33 * eq2_e351_d_b3);
        let eq2_e352_d_b4: f64 = (p.p33 * eq2_e351_d_b4);
        let eq2_e352_d_b5: f64 = (p.p33 * eq2_e351_d_b5);
        let eq2_e352_d_b6: f64 = (p.p33 * eq2_e351_d_b6);
        let eq2_e352_d_b7: f64 = (p.p33 * eq2_e351_d_b7);
        let eq2_value: f64 = eq2_e352;
        let eq2_node_derivatives: [f64; 13] = [eq2_e352_d_n0, eq2_e352_d_n1, eq2_e352_d_n2, eq2_e352_d_n3, eq2_e352_d_n4, eq2_e352_d_n5, eq2_e352_d_n6, eq2_e352_d_n7, eq2_e352_d_n8, eq2_e352_d_n9, eq2_e352_d_n10, eq2_e352_d_n11, eq2_e352_d_n12];
        let eq2_branch_derivatives: [f64; 8] = [eq2_e352_d_b0, eq2_e352_d_b1, eq2_e352_d_b2, eq2_e352_d_b3, eq2_e352_d_b4, eq2_e352_d_b5, eq2_e352_d_b6, eq2_e352_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e355: f64 = (p.p33 * s.v[250]);
        let eq3_e355_d_n0: f64 = (p.p33 * s.dn[250][0]);
        let eq3_e355_d_n1: f64 = (p.p33 * s.dn[250][1]);
        let eq3_e355_d_n2: f64 = (p.p33 * s.dn[250][2]);
        let eq3_e355_d_n3: f64 = (p.p33 * s.dn[250][3]);
        let eq3_e355_d_n4: f64 = (p.p33 * s.dn[250][4]);
        let eq3_e355_d_n5: f64 = (p.p33 * s.dn[250][5]);
        let eq3_e355_d_n6: f64 = (p.p33 * s.dn[250][6]);
        let eq3_e355_d_n7: f64 = (p.p33 * s.dn[250][7]);
        let eq3_e355_d_n8: f64 = (p.p33 * s.dn[250][8]);
        let eq3_e355_d_n9: f64 = (p.p33 * s.dn[250][9]);
        let eq3_e355_d_n10: f64 = (p.p33 * s.dn[250][10]);
        let eq3_e355_d_n11: f64 = (p.p33 * s.dn[250][11]);
        let eq3_e355_d_n12: f64 = (p.p33 * s.dn[250][12]);
        let eq3_e355_d_b0: f64 = (p.p33 * s.db[250][0]);
        let eq3_e355_d_b1: f64 = (p.p33 * s.db[250][1]);
        let eq3_e355_d_b2: f64 = (p.p33 * s.db[250][2]);
        let eq3_e355_d_b3: f64 = (p.p33 * s.db[250][3]);
        let eq3_e355_d_b4: f64 = (p.p33 * s.db[250][4]);
        let eq3_e355_d_b5: f64 = (p.p33 * s.db[250][5]);
        let eq3_e355_d_b6: f64 = (p.p33 * s.db[250][6]);
        let eq3_e355_d_b7: f64 = (p.p33 * s.db[250][7]);
        let eq3_value: f64 = eq3_e355;
        let eq3_node_derivatives: [f64; 13] = [eq3_e355_d_n0, eq3_e355_d_n1, eq3_e355_d_n2, eq3_e355_d_n3, eq3_e355_d_n4, eq3_e355_d_n5, eq3_e355_d_n6, eq3_e355_d_n7, eq3_e355_d_n8, eq3_e355_d_n9, eq3_e355_d_n10, eq3_e355_d_n11, eq3_e355_d_n12];
        let eq3_branch_derivatives: [f64; 8] = [eq3_e355_d_b0, eq3_e355_d_b1, eq3_e355_d_b2, eq3_e355_d_b3, eq3_e355_d_b4, eq3_e355_d_b5, eq3_e355_d_b6, eq3_e355_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e358: f64 = (p.p33 * s.v[251]);
        let eq4_e358_d_n0: f64 = (p.p33 * s.dn[251][0]);
        let eq4_e358_d_n1: f64 = (p.p33 * s.dn[251][1]);
        let eq4_e358_d_n2: f64 = (p.p33 * s.dn[251][2]);
        let eq4_e358_d_n3: f64 = (p.p33 * s.dn[251][3]);
        let eq4_e358_d_n4: f64 = (p.p33 * s.dn[251][4]);
        let eq4_e358_d_n5: f64 = (p.p33 * s.dn[251][5]);
        let eq4_e358_d_n6: f64 = (p.p33 * s.dn[251][6]);
        let eq4_e358_d_n7: f64 = (p.p33 * s.dn[251][7]);
        let eq4_e358_d_n8: f64 = (p.p33 * s.dn[251][8]);
        let eq4_e358_d_n9: f64 = (p.p33 * s.dn[251][9]);
        let eq4_e358_d_n10: f64 = (p.p33 * s.dn[251][10]);
        let eq4_e358_d_n11: f64 = (p.p33 * s.dn[251][11]);
        let eq4_e358_d_n12: f64 = (p.p33 * s.dn[251][12]);
        let eq4_e358_d_b0: f64 = (p.p33 * s.db[251][0]);
        let eq4_e358_d_b1: f64 = (p.p33 * s.db[251][1]);
        let eq4_e358_d_b2: f64 = (p.p33 * s.db[251][2]);
        let eq4_e358_d_b3: f64 = (p.p33 * s.db[251][3]);
        let eq4_e358_d_b4: f64 = (p.p33 * s.db[251][4]);
        let eq4_e358_d_b5: f64 = (p.p33 * s.db[251][5]);
        let eq4_e358_d_b6: f64 = (p.p33 * s.db[251][6]);
        let eq4_e358_d_b7: f64 = (p.p33 * s.db[251][7]);
        let eq4_value: f64 = eq4_e358;
        let eq4_node_derivatives: [f64; 13] = [eq4_e358_d_n0, eq4_e358_d_n1, eq4_e358_d_n2, eq4_e358_d_n3, eq4_e358_d_n4, eq4_e358_d_n5, eq4_e358_d_n6, eq4_e358_d_n7, eq4_e358_d_n8, eq4_e358_d_n9, eq4_e358_d_n10, eq4_e358_d_n11, eq4_e358_d_n12];
        let eq4_branch_derivatives: [f64; 8] = [eq4_e358_d_b0, eq4_e358_d_b1, eq4_e358_d_b2, eq4_e358_d_b3, eq4_e358_d_b4, eq4_e358_d_b5, eq4_e358_d_b6, eq4_e358_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_e361: f64 = (p.p33 * s.v[254]);
        let eq5_e361_d_n0: f64 = (p.p33 * s.dn[254][0]);
        let eq5_e361_d_n1: f64 = (p.p33 * s.dn[254][1]);
        let eq5_e361_d_n2: f64 = (p.p33 * s.dn[254][2]);
        let eq5_e361_d_n3: f64 = (p.p33 * s.dn[254][3]);
        let eq5_e361_d_n4: f64 = (p.p33 * s.dn[254][4]);
        let eq5_e361_d_n5: f64 = (p.p33 * s.dn[254][5]);
        let eq5_e361_d_n6: f64 = (p.p33 * s.dn[254][6]);
        let eq5_e361_d_n7: f64 = (p.p33 * s.dn[254][7]);
        let eq5_e361_d_n8: f64 = (p.p33 * s.dn[254][8]);
        let eq5_e361_d_n9: f64 = (p.p33 * s.dn[254][9]);
        let eq5_e361_d_n10: f64 = (p.p33 * s.dn[254][10]);
        let eq5_e361_d_n11: f64 = (p.p33 * s.dn[254][11]);
        let eq5_e361_d_n12: f64 = (p.p33 * s.dn[254][12]);
        let eq5_e361_d_b0: f64 = (p.p33 * s.db[254][0]);
        let eq5_e361_d_b1: f64 = (p.p33 * s.db[254][1]);
        let eq5_e361_d_b2: f64 = (p.p33 * s.db[254][2]);
        let eq5_e361_d_b3: f64 = (p.p33 * s.db[254][3]);
        let eq5_e361_d_b4: f64 = (p.p33 * s.db[254][4]);
        let eq5_e361_d_b5: f64 = (p.p33 * s.db[254][5]);
        let eq5_e361_d_b6: f64 = (p.p33 * s.db[254][6]);
        let eq5_e361_d_b7: f64 = (p.p33 * s.db[254][7]);
        let eq5_value: f64 = eq5_e361;
        let eq5_node_derivatives: [f64; 13] = [eq5_e361_d_n0, eq5_e361_d_n1, eq5_e361_d_n2, eq5_e361_d_n3, eq5_e361_d_n4, eq5_e361_d_n5, eq5_e361_d_n6, eq5_e361_d_n7, eq5_e361_d_n8, eq5_e361_d_n9, eq5_e361_d_n10, eq5_e361_d_n11, eq5_e361_d_n12];
        let eq5_branch_derivatives: [f64; 8] = [eq5_e361_d_b0, eq5_e361_d_b1, eq5_e361_d_b2, eq5_e361_d_b3, eq5_e361_d_b4, eq5_e361_d_b5, eq5_e361_d_b6, eq5_e361_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e367, eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12, eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3, eq6_e367_d_b4, eq6_e367_d_b5, eq6_e367_d_b6, eq6_e367_d_b7,) = {
    if (p.p312 != 0.0) {
        let eq6_e365: f64 = ((nv12 - nv2) / s.v[27]);
        let eq6_e365_d_n0: f64 = (-(((nv12 - nv2) * s.dn[27][0]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n1: f64 = (-(((nv12 - nv2) * s.dn[27][1]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n2: f64 = (((-s.v[27]) - ((nv12 - nv2) * s.dn[27][2])) / (s.v[27] * s.v[27]));
        let eq6_e365_d_n3: f64 = (-(((nv12 - nv2) * s.dn[27][3]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n4: f64 = (-(((nv12 - nv2) * s.dn[27][4]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n5: f64 = (-(((nv12 - nv2) * s.dn[27][5]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n6: f64 = (-(((nv12 - nv2) * s.dn[27][6]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n7: f64 = (-(((nv12 - nv2) * s.dn[27][7]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n8: f64 = (-(((nv12 - nv2) * s.dn[27][8]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n9: f64 = (-(((nv12 - nv2) * s.dn[27][9]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n10: f64 = (-(((nv12 - nv2) * s.dn[27][10]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n11: f64 = (-(((nv12 - nv2) * s.dn[27][11]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n12: f64 = ((s.v[27] - ((nv12 - nv2) * s.dn[27][12])) / (s.v[27] * s.v[27]));
        let eq6_e365_d_b0: f64 = (-(((nv12 - nv2) * s.db[27][0]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b1: f64 = (-(((nv12 - nv2) * s.db[27][1]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b2: f64 = (-(((nv12 - nv2) * s.db[27][2]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b3: f64 = (-(((nv12 - nv2) * s.db[27][3]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b4: f64 = (-(((nv12 - nv2) * s.db[27][4]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b5: f64 = (-(((nv12 - nv2) * s.db[27][5]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b6: f64 = (-(((nv12 - nv2) * s.db[27][6]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b7: f64 = (-(((nv12 - nv2) * s.db[27][7]) / (s.v[27] * s.v[27])));
        (eq6_e365, eq6_e365_d_n0, eq6_e365_d_n1, eq6_e365_d_n2, eq6_e365_d_n3, eq6_e365_d_n4, eq6_e365_d_n5, eq6_e365_d_n6, eq6_e365_d_n7, eq6_e365_d_n8, eq6_e365_d_n9, eq6_e365_d_n10, eq6_e365_d_n11, eq6_e365_d_n12, eq6_e365_d_b0, eq6_e365_d_b1, eq6_e365_d_b2, eq6_e365_d_b3, eq6_e365_d_b4, eq6_e365_d_b5, eq6_e365_d_b6, eq6_e365_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e367;
        let eq6_node_derivatives: [f64; 13] = [eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12];
        let eq6_branch_derivatives: [f64; 8] = [eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3, eq6_e367_d_b4, eq6_e367_d_b5, eq6_e367_d_b6, eq6_e367_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e372,) = {
    if (p.p312 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e372;
        stamper.stamp_potential_const_local(
            0,
            eq7_value,
        );
        let (eq8_e378, eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12, eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3, eq8_e378_d_b4, eq8_e378_d_b5, eq8_e378_d_b6, eq8_e378_d_b7,) = {
    if (p.p313 != 0.0) {
        let eq8_e376: f64 = ((nv0 - nv11) / s.v[26]);
        let eq8_e376_d_n0: f64 = ((s.v[26] - ((nv0 - nv11) * s.dn[26][0])) / (s.v[26] * s.v[26]));
        let eq8_e376_d_n1: f64 = (-(((nv0 - nv11) * s.dn[26][1]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n2: f64 = (-(((nv0 - nv11) * s.dn[26][2]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n3: f64 = (-(((nv0 - nv11) * s.dn[26][3]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n4: f64 = (-(((nv0 - nv11) * s.dn[26][4]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n5: f64 = (-(((nv0 - nv11) * s.dn[26][5]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n6: f64 = (-(((nv0 - nv11) * s.dn[26][6]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n7: f64 = (-(((nv0 - nv11) * s.dn[26][7]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n8: f64 = (-(((nv0 - nv11) * s.dn[26][8]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n9: f64 = (-(((nv0 - nv11) * s.dn[26][9]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n10: f64 = (-(((nv0 - nv11) * s.dn[26][10]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n11: f64 = (((-s.v[26]) - ((nv0 - nv11) * s.dn[26][11])) / (s.v[26] * s.v[26]));
        let eq8_e376_d_n12: f64 = (-(((nv0 - nv11) * s.dn[26][12]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b0: f64 = (-(((nv0 - nv11) * s.db[26][0]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b1: f64 = (-(((nv0 - nv11) * s.db[26][1]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b2: f64 = (-(((nv0 - nv11) * s.db[26][2]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b3: f64 = (-(((nv0 - nv11) * s.db[26][3]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b4: f64 = (-(((nv0 - nv11) * s.db[26][4]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b5: f64 = (-(((nv0 - nv11) * s.db[26][5]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b6: f64 = (-(((nv0 - nv11) * s.db[26][6]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b7: f64 = (-(((nv0 - nv11) * s.db[26][7]) / (s.v[26] * s.v[26])));
        (eq8_e376, eq8_e376_d_n0, eq8_e376_d_n1, eq8_e376_d_n2, eq8_e376_d_n3, eq8_e376_d_n4, eq8_e376_d_n5, eq8_e376_d_n6, eq8_e376_d_n7, eq8_e376_d_n8, eq8_e376_d_n9, eq8_e376_d_n10, eq8_e376_d_n11, eq8_e376_d_n12, eq8_e376_d_b0, eq8_e376_d_b1, eq8_e376_d_b2, eq8_e376_d_b3, eq8_e376_d_b4, eq8_e376_d_b5, eq8_e376_d_b6, eq8_e376_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e378;
        let eq8_node_derivatives: [f64; 13] = [eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12];
        let eq8_branch_derivatives: [f64; 8] = [eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3, eq8_e378_d_b4, eq8_e378_d_b5, eq8_e378_d_b6, eq8_e378_d_b7];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e383,) = {
    if (p.p313 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e383;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e387_d_b4: f64 = (s.db[561][4] + s.db[554][4]);
        let eq10_e387_d_b5: f64 = (s.db[561][5] + s.db[554][5]);
        let eq10_e387_d_b6: f64 = (s.db[561][6] + s.db[554][6]);
        let eq10_e387_d_b7: f64 = (s.db[561][7] + s.db[554][7]);
        let eq10_e388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq10_e387);
        let eq10_e388_d_n0: f64 = (eq10_e387_d_n0 * ddt_scale);
        let eq10_e388_d_n1: f64 = (eq10_e387_d_n1 * ddt_scale);
        let eq10_e388_d_n2: f64 = (eq10_e387_d_n2 * ddt_scale);
        let eq10_e388_d_n3: f64 = (eq10_e387_d_n3 * ddt_scale);
        let eq10_e388_d_n4: f64 = (eq10_e387_d_n4 * ddt_scale);
        let eq10_e388_d_n5: f64 = (eq10_e387_d_n5 * ddt_scale);
        let eq10_e388_d_n6: f64 = (eq10_e387_d_n6 * ddt_scale);
        let eq10_e388_d_n7: f64 = (eq10_e387_d_n7 * ddt_scale);
        let eq10_e388_d_n8: f64 = (eq10_e387_d_n8 * ddt_scale);
        let eq10_e388_d_n9: f64 = (eq10_e387_d_n9 * ddt_scale);
        let eq10_e388_d_n10: f64 = (eq10_e387_d_n10 * ddt_scale);
        let eq10_e388_d_n11: f64 = (eq10_e387_d_n11 * ddt_scale);
        let eq10_e388_d_n12: f64 = (eq10_e387_d_n12 * ddt_scale);
        let eq10_e388_d_b0: f64 = (eq10_e387_d_b0 * ddt_scale);
        let eq10_e388_d_b1: f64 = (eq10_e387_d_b1 * ddt_scale);
        let eq10_e388_d_b2: f64 = (eq10_e387_d_b2 * ddt_scale);
        let eq10_e388_d_b3: f64 = (eq10_e387_d_b3 * ddt_scale);
        let eq10_e388_d_b4: f64 = (eq10_e387_d_b4 * ddt_scale);
        let eq10_e388_d_b5: f64 = (eq10_e387_d_b5 * ddt_scale);
        let eq10_e388_d_b6: f64 = (eq10_e387_d_b6 * ddt_scale);
        let eq10_e388_d_b7: f64 = (eq10_e387_d_b7 * ddt_scale);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e388_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e388_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e388_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e388_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e388_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e388_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e388_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e388_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e388_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e388_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e388_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e388_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e388_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e388_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e388_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e388_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e388_d_b3);
        let eq10_e389_d_b4: f64 = (p.p33 * eq10_e388_d_b4);
        let eq10_e389_d_b5: f64 = (p.p33 * eq10_e388_d_b5);
        let eq10_e389_d_b6: f64 = (p.p33 * eq10_e388_d_b6);
        let eq10_e389_d_b7: f64 = (p.p33 * eq10_e388_d_b7);
        let eq10_value: f64 = eq10_e389;
        let eq10_node_derivatives: [f64; 13] = [eq10_e389_d_n0, eq10_e389_d_n1, eq10_e389_d_n2, eq10_e389_d_n3, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, eq10_e389_d_n7, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_branch_derivatives: [f64; 8] = [eq10_e389_d_b0, eq10_e389_d_b1, eq10_e389_d_b2, eq10_e389_d_b3, eq10_e389_d_b4, eq10_e389_d_b5, eq10_e389_d_b6, eq10_e389_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e393_d_b4: f64 = (s.db[93][4] + s.db[552][4]);
        let eq11_e393_d_b5: f64 = (s.db[93][5] + s.db[552][5]);
        let eq11_e393_d_b6: f64 = (s.db[93][6] + s.db[552][6]);
        let eq11_e393_d_b7: f64 = (s.db[93][7] + s.db[552][7]);
        let eq11_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq11_e393);
        let eq11_e394_d_n0: f64 = (eq11_e393_d_n0 * ddt_scale);
        let eq11_e394_d_n1: f64 = (eq11_e393_d_n1 * ddt_scale);
        let eq11_e394_d_n2: f64 = (eq11_e393_d_n2 * ddt_scale);
        let eq11_e394_d_n3: f64 = (eq11_e393_d_n3 * ddt_scale);
        let eq11_e394_d_n4: f64 = (eq11_e393_d_n4 * ddt_scale);
        let eq11_e394_d_n5: f64 = (eq11_e393_d_n5 * ddt_scale);
        let eq11_e394_d_n6: f64 = (eq11_e393_d_n6 * ddt_scale);
        let eq11_e394_d_n7: f64 = (eq11_e393_d_n7 * ddt_scale);
        let eq11_e394_d_n8: f64 = (eq11_e393_d_n8 * ddt_scale);
        let eq11_e394_d_n9: f64 = (eq11_e393_d_n9 * ddt_scale);
        let eq11_e394_d_n10: f64 = (eq11_e393_d_n10 * ddt_scale);
        let eq11_e394_d_n11: f64 = (eq11_e393_d_n11 * ddt_scale);
        let eq11_e394_d_n12: f64 = (eq11_e393_d_n12 * ddt_scale);
        let eq11_e394_d_b0: f64 = (eq11_e393_d_b0 * ddt_scale);
        let eq11_e394_d_b1: f64 = (eq11_e393_d_b1 * ddt_scale);
        let eq11_e394_d_b2: f64 = (eq11_e393_d_b2 * ddt_scale);
        let eq11_e394_d_b3: f64 = (eq11_e393_d_b3 * ddt_scale);
        let eq11_e394_d_b4: f64 = (eq11_e393_d_b4 * ddt_scale);
        let eq11_e394_d_b5: f64 = (eq11_e393_d_b5 * ddt_scale);
        let eq11_e394_d_b6: f64 = (eq11_e393_d_b6 * ddt_scale);
        let eq11_e394_d_b7: f64 = (eq11_e393_d_b7 * ddt_scale);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e394_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e394_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e394_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e394_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e394_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e394_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e394_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e394_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e394_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e394_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e394_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e394_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e394_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e394_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e394_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e394_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e394_d_b3);
        let eq11_e395_d_b4: f64 = (p.p33 * eq11_e394_d_b4);
        let eq11_e395_d_b5: f64 = (p.p33 * eq11_e394_d_b5);
        let eq11_e395_d_b6: f64 = (p.p33 * eq11_e394_d_b6);
        let eq11_e395_d_b7: f64 = (p.p33 * eq11_e394_d_b7);
        let eq11_value: f64 = eq11_e395;
        let eq11_node_derivatives: [f64; 13] = [eq11_e395_d_n0, eq11_e395_d_n1, eq11_e395_d_n2, eq11_e395_d_n3, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, eq11_e395_d_n7, eq11_e395_d_n8, eq11_e395_d_n9, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_branch_derivatives: [f64; 8] = [eq11_e395_d_b0, eq11_e395_d_b1, eq11_e395_d_b2, eq11_e395_d_b3, eq11_e395_d_b4, eq11_e395_d_b5, eq11_e395_d_b6, eq11_e395_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e399_d_b4: f64 = (s.db[90][4] + s.db[548][4]);
        let eq12_e399_d_b5: f64 = (s.db[90][5] + s.db[548][5]);
        let eq12_e399_d_b6: f64 = (s.db[90][6] + s.db[548][6]);
        let eq12_e399_d_b7: f64 = (s.db[90][7] + s.db[548][7]);
        let eq12_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq12_e399);
        let eq12_e400_d_n0: f64 = (eq12_e399_d_n0 * ddt_scale);
        let eq12_e400_d_n1: f64 = (eq12_e399_d_n1 * ddt_scale);
        let eq12_e400_d_n2: f64 = (eq12_e399_d_n2 * ddt_scale);
        let eq12_e400_d_n3: f64 = (eq12_e399_d_n3 * ddt_scale);
        let eq12_e400_d_n4: f64 = (eq12_e399_d_n4 * ddt_scale);
        let eq12_e400_d_n5: f64 = (eq12_e399_d_n5 * ddt_scale);
        let eq12_e400_d_n6: f64 = (eq12_e399_d_n6 * ddt_scale);
        let eq12_e400_d_n7: f64 = (eq12_e399_d_n7 * ddt_scale);
        let eq12_e400_d_n8: f64 = (eq12_e399_d_n8 * ddt_scale);
        let eq12_e400_d_n9: f64 = (eq12_e399_d_n9 * ddt_scale);
        let eq12_e400_d_n10: f64 = (eq12_e399_d_n10 * ddt_scale);
        let eq12_e400_d_n11: f64 = (eq12_e399_d_n11 * ddt_scale);
        let eq12_e400_d_n12: f64 = (eq12_e399_d_n12 * ddt_scale);
        let eq12_e400_d_b0: f64 = (eq12_e399_d_b0 * ddt_scale);
        let eq12_e400_d_b1: f64 = (eq12_e399_d_b1 * ddt_scale);
        let eq12_e400_d_b2: f64 = (eq12_e399_d_b2 * ddt_scale);
        let eq12_e400_d_b3: f64 = (eq12_e399_d_b3 * ddt_scale);
        let eq12_e400_d_b4: f64 = (eq12_e399_d_b4 * ddt_scale);
        let eq12_e400_d_b5: f64 = (eq12_e399_d_b5 * ddt_scale);
        let eq12_e400_d_b6: f64 = (eq12_e399_d_b6 * ddt_scale);
        let eq12_e400_d_b7: f64 = (eq12_e399_d_b7 * ddt_scale);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e400_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e400_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e400_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e400_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e400_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e400_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e400_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e400_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e400_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e400_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e400_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e400_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e400_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e400_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e400_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e400_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e400_d_b3);
        let eq12_e401_d_b4: f64 = (p.p33 * eq12_e400_d_b4);
        let eq12_e401_d_b5: f64 = (p.p33 * eq12_e400_d_b5);
        let eq12_e401_d_b6: f64 = (p.p33 * eq12_e400_d_b6);
        let eq12_e401_d_b7: f64 = (p.p33 * eq12_e400_d_b7);
        let eq12_value: f64 = eq12_e401;
        let eq12_node_derivatives: [f64; 13] = [eq12_e401_d_n0, eq12_e401_d_n1, eq12_e401_d_n2, eq12_e401_d_n3, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, eq12_e401_d_n7, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_branch_derivatives: [f64; 8] = [eq12_e401_d_b0, eq12_e401_d_b1, eq12_e401_d_b2, eq12_e401_d_b3, eq12_e401_d_b4, eq12_e401_d_b5, eq12_e401_d_b6, eq12_e401_d_b7];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(11),
            Some(12),
            multiplicity * (eq13_value),
        );
        let eq14_e412: f64 = (nv7 - 0.0);
        let eq14_value: f64 = eq14_e412;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (1.0),
        );
        let eq15_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
        );
        let eq16_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(11),
            Some(12),
            multiplicity * (eq16_value),
        );
        let eq17_e427: f64 = (s.v[609] * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (s.dn[609][0] * (nv7 - 0.0));
        let eq17_e427_d_n1: f64 = (s.dn[609][1] * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (s.dn[609][2] * (nv7 - 0.0));
        let eq17_e427_d_n3: f64 = (s.dn[609][3] * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (s.dn[609][4] * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (s.dn[609][5] * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (s.dn[609][6] * (nv7 - 0.0));
        let eq17_e427_d_n7: f64 = ((s.dn[609][7] * (nv7 - 0.0)) + s.v[609]);
        let eq17_e427_d_n8: f64 = (s.dn[609][8] * (nv7 - 0.0));
        let eq17_e427_d_n9: f64 = (s.dn[609][9] * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (s.dn[609][10] * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (s.dn[609][11] * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (s.dn[609][12] * (nv7 - 0.0));
        let eq17_e427_d_b0: f64 = (s.db[609][0] * (nv7 - 0.0));
        let eq17_e427_d_b1: f64 = (s.db[609][1] * (nv7 - 0.0));
        let eq17_e427_d_b2: f64 = (s.db[609][2] * (nv7 - 0.0));
        let eq17_e427_d_b3: f64 = (s.db[609][3] * (nv7 - 0.0));
        let eq17_e427_d_b4: f64 = (s.db[609][4] * (nv7 - 0.0));
        let eq17_e427_d_b5: f64 = (s.db[609][5] * (nv7 - 0.0));
        let eq17_e427_d_b6: f64 = (s.db[609][6] * (nv7 - 0.0));
        let eq17_e427_d_b7: f64 = (s.db[609][7] * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        let eq17_node_derivatives: [f64; 13] = [eq17_e427_d_n0, eq17_e427_d_n1, eq17_e427_d_n2, eq17_e427_d_n3, eq17_e427_d_n4, eq17_e427_d_n5, eq17_e427_d_n6, eq17_e427_d_n7, eq17_e427_d_n8, eq17_e427_d_n9, eq17_e427_d_n10, eq17_e427_d_n11, eq17_e427_d_n12];
        let eq17_branch_derivatives: [f64; 8] = [eq17_e427_d_b0, eq17_e427_d_b1, eq17_e427_d_b2, eq17_e427_d_b3, eq17_e427_d_b4, eq17_e427_d_b5, eq17_e427_d_b6, eq17_e427_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e430_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);
        let eq18_e430_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);
        let eq18_e430_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);
        let eq18_e430_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);
        let eq18_e431: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq18_e430);
        let eq18_e431_d_n0: f64 = (eq18_e430_d_n0 * ddt_scale);
        let eq18_e431_d_n1: f64 = (eq18_e430_d_n1 * ddt_scale);
        let eq18_e431_d_n2: f64 = (eq18_e430_d_n2 * ddt_scale);
        let eq18_e431_d_n3: f64 = (eq18_e430_d_n3 * ddt_scale);
        let eq18_e431_d_n4: f64 = (eq18_e430_d_n4 * ddt_scale);
        let eq18_e431_d_n5: f64 = (eq18_e430_d_n5 * ddt_scale);
        let eq18_e431_d_n6: f64 = (eq18_e430_d_n6 * ddt_scale);
        let eq18_e431_d_n7: f64 = (eq18_e430_d_n7 * ddt_scale);
        let eq18_e431_d_n8: f64 = (eq18_e430_d_n8 * ddt_scale);
        let eq18_e431_d_n9: f64 = (eq18_e430_d_n9 * ddt_scale);
        let eq18_e431_d_n10: f64 = (eq18_e430_d_n10 * ddt_scale);
        let eq18_e431_d_n11: f64 = (eq18_e430_d_n11 * ddt_scale);
        let eq18_e431_d_n12: f64 = (eq18_e430_d_n12 * ddt_scale);
        let eq18_e431_d_b0: f64 = (eq18_e430_d_b0 * ddt_scale);
        let eq18_e431_d_b1: f64 = (eq18_e430_d_b1 * ddt_scale);
        let eq18_e431_d_b2: f64 = (eq18_e430_d_b2 * ddt_scale);
        let eq18_e431_d_b3: f64 = (eq18_e430_d_b3 * ddt_scale);
        let eq18_e431_d_b4: f64 = (eq18_e430_d_b4 * ddt_scale);
        let eq18_e431_d_b5: f64 = (eq18_e430_d_b5 * ddt_scale);
        let eq18_e431_d_b6: f64 = (eq18_e430_d_b6 * ddt_scale);
        let eq18_e431_d_b7: f64 = (eq18_e430_d_b7 * ddt_scale);
        let eq18_value: f64 = eq18_e431;
        let eq18_node_derivatives: [f64; 13] = [eq18_e431_d_n0, eq18_e431_d_n1, eq18_e431_d_n2, eq18_e431_d_n3, eq18_e431_d_n4, eq18_e431_d_n5, eq18_e431_d_n6, eq18_e431_d_n7, eq18_e431_d_n8, eq18_e431_d_n9, eq18_e431_d_n10, eq18_e431_d_n11, eq18_e431_d_n12];
        let eq18_branch_derivatives: [f64; 8] = [eq18_e431_d_b0, eq18_e431_d_b1, eq18_e431_d_b2, eq18_e431_d_b3, eq18_e431_d_b4, eq18_e431_d_b5, eq18_e431_d_b6, eq18_e431_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e434_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);
        let eq19_e434_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);
        let eq19_e434_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);
        let eq19_e434_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);
        let eq19_e435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq19_e434);
        let eq19_e435_d_n0: f64 = (eq19_e434_d_n0 * ddt_scale);
        let eq19_e435_d_n1: f64 = (eq19_e434_d_n1 * ddt_scale);
        let eq19_e435_d_n2: f64 = (eq19_e434_d_n2 * ddt_scale);
        let eq19_e435_d_n3: f64 = (eq19_e434_d_n3 * ddt_scale);
        let eq19_e435_d_n4: f64 = (eq19_e434_d_n4 * ddt_scale);
        let eq19_e435_d_n5: f64 = (eq19_e434_d_n5 * ddt_scale);
        let eq19_e435_d_n6: f64 = (eq19_e434_d_n6 * ddt_scale);
        let eq19_e435_d_n7: f64 = (eq19_e434_d_n7 * ddt_scale);
        let eq19_e435_d_n8: f64 = (eq19_e434_d_n8 * ddt_scale);
        let eq19_e435_d_n9: f64 = (eq19_e434_d_n9 * ddt_scale);
        let eq19_e435_d_n10: f64 = (eq19_e434_d_n10 * ddt_scale);
        let eq19_e435_d_n11: f64 = (eq19_e434_d_n11 * ddt_scale);
        let eq19_e435_d_n12: f64 = (eq19_e434_d_n12 * ddt_scale);
        let eq19_e435_d_b0: f64 = (eq19_e434_d_b0 * ddt_scale);
        let eq19_e435_d_b1: f64 = (eq19_e434_d_b1 * ddt_scale);
        let eq19_e435_d_b2: f64 = (eq19_e434_d_b2 * ddt_scale);
        let eq19_e435_d_b3: f64 = (eq19_e434_d_b3 * ddt_scale);
        let eq19_e435_d_b4: f64 = (eq19_e434_d_b4 * ddt_scale);
        let eq19_e435_d_b5: f64 = (eq19_e434_d_b5 * ddt_scale);
        let eq19_e435_d_b6: f64 = (eq19_e434_d_b6 * ddt_scale);
        let eq19_e435_d_b7: f64 = (eq19_e434_d_b7 * ddt_scale);
        let eq19_value: f64 = eq19_e435;
        let eq19_node_derivatives: [f64; 13] = [eq19_e435_d_n0, eq19_e435_d_n1, eq19_e435_d_n2, eq19_e435_d_n3, eq19_e435_d_n4, eq19_e435_d_n5, eq19_e435_d_n6, eq19_e435_d_n7, eq19_e435_d_n8, eq19_e435_d_n9, eq19_e435_d_n10, eq19_e435_d_n11, eq19_e435_d_n12];
        let eq19_branch_derivatives: [f64; 8] = [eq19_e435_d_b0, eq19_e435_d_b1, eq19_e435_d_b2, eq19_e435_d_b3, eq19_e435_d_b4, eq19_e435_d_b5, eq19_e435_d_b6, eq19_e435_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e443,) = {
    if (p.p312 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e443;
        stamper.stamp_current_const_local(
            Some(12),
            Some(2),
            multiplicity * (eq20_value),
        );
        let (eq21_e451,) = {
    if (p.p313 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e451;
        stamper.stamp_current_const_local(
            Some(0),
            Some(11),
            multiplicity * (eq21_value),
        );
        let eq22_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(11),
            multiplicity * (eq22_value),
        );
        let eq23_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(12),
            multiplicity * (eq23_value),
        );
        let eq24_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq24_value),
        );
        let (eq25_e478, eq25_e478_d_n0, eq25_e478_d_n1, eq25_e478_d_n2, eq25_e478_d_n3, eq25_e478_d_n4, eq25_e478_d_n5, eq25_e478_d_n6, eq25_e478_d_n7, eq25_e478_d_n8, eq25_e478_d_n9, eq25_e478_d_n10, eq25_e478_d_n11, eq25_e478_d_n12, eq25_e478_d_b0, eq25_e478_d_b1, eq25_e478_d_b2, eq25_e478_d_b3, eq25_e478_d_b4, eq25_e478_d_b5, eq25_e478_d_b6, eq25_e478_d_b7,) = {
    if (p.p25 != 0.0) {
        let eq25_e476: f64 = (s.v[484] * (nv1 - nv5));
        let eq25_e476_d_n0: f64 = (s.dn[484][0] * (nv1 - nv5));
        let eq25_e476_d_n1: f64 = ((s.dn[484][1] * (nv1 - nv5)) + s.v[484]);
        let eq25_e476_d_n2: f64 = (s.dn[484][2] * (nv1 - nv5));
        let eq25_e476_d_n3: f64 = (s.dn[484][3] * (nv1 - nv5));
        let eq25_e476_d_n4: f64 = (s.dn[484][4] * (nv1 - nv5));
        let eq25_e476_d_n5: f64 = ((s.dn[484][5] * (nv1 - nv5)) + (-s.v[484]));
        let eq25_e476_d_n6: f64 = (s.dn[484][6] * (nv1 - nv5));
        let eq25_e476_d_n7: f64 = (s.dn[484][7] * (nv1 - nv5));
        let eq25_e476_d_n8: f64 = (s.dn[484][8] * (nv1 - nv5));
        let eq25_e476_d_n9: f64 = (s.dn[484][9] * (nv1 - nv5));
        let eq25_e476_d_n10: f64 = (s.dn[484][10] * (nv1 - nv5));
        let eq25_e476_d_n11: f64 = (s.dn[484][11] * (nv1 - nv5));
        let eq25_e476_d_n12: f64 = (s.dn[484][12] * (nv1 - nv5));
        let eq25_e476_d_b0: f64 = (s.db[484][0] * (nv1 - nv5));
        let eq25_e476_d_b1: f64 = (s.db[484][1] * (nv1 - nv5));
        let eq25_e476_d_b2: f64 = (s.db[484][2] * (nv1 - nv5));
        let eq25_e476_d_b3: f64 = (s.db[484][3] * (nv1 - nv5));
        let eq25_e476_d_b4: f64 = (s.db[484][4] * (nv1 - nv5));
        let eq25_e476_d_b5: f64 = (s.db[484][5] * (nv1 - nv5));
        let eq25_e476_d_b6: f64 = (s.db[484][6] * (nv1 - nv5));
        let eq25_e476_d_b7: f64 = (s.db[484][7] * (nv1 - nv5));
        (eq25_e476, eq25_e476_d_n0, eq25_e476_d_n1, eq25_e476_d_n2, eq25_e476_d_n3, eq25_e476_d_n4, eq25_e476_d_n5, eq25_e476_d_n6, eq25_e476_d_n7, eq25_e476_d_n8, eq25_e476_d_n9, eq25_e476_d_n10, eq25_e476_d_n11, eq25_e476_d_n12, eq25_e476_d_b0, eq25_e476_d_b1, eq25_e476_d_b2, eq25_e476_d_b3, eq25_e476_d_b4, eq25_e476_d_b5, eq25_e476_d_b6, eq25_e476_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e478;
        let eq25_node_derivatives: [f64; 13] = [eq25_e478_d_n0, eq25_e478_d_n1, eq25_e478_d_n2, eq25_e478_d_n3, eq25_e478_d_n4, eq25_e478_d_n5, eq25_e478_d_n6, eq25_e478_d_n7, eq25_e478_d_n8, eq25_e478_d_n9, eq25_e478_d_n10, eq25_e478_d_n11, eq25_e478_d_n12];
        let eq25_branch_derivatives: [f64; 8] = [eq25_e478_d_b0, eq25_e478_d_b1, eq25_e478_d_b2, eq25_e478_d_b3, eq25_e478_d_b4, eq25_e478_d_b5, eq25_e478_d_b6, eq25_e478_d_b7];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e483,) = {
    if (p.p25 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e483;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );
        let eq27_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7,) = {
    if s.b[1094] {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e487_d_n0: f64 = (-s.dn[547][0]);
        let eq28_e487_d_n1: f64 = (-s.dn[547][1]);
        let eq28_e487_d_n2: f64 = (-s.dn[547][2]);
        let eq28_e487_d_n3: f64 = (-s.dn[547][3]);
        let eq28_e487_d_n4: f64 = (-s.dn[547][4]);
        let eq28_e487_d_n5: f64 = (-s.dn[547][5]);
        let eq28_e487_d_n6: f64 = (-s.dn[547][6]);
        let eq28_e487_d_n7: f64 = (-s.dn[547][7]);
        let eq28_e487_d_n8: f64 = (-s.dn[547][8]);
        let eq28_e487_d_n9: f64 = (-s.dn[547][9]);
        let eq28_e487_d_n10: f64 = (-s.dn[547][10]);
        let eq28_e487_d_n11: f64 = (-s.dn[547][11]);
        let eq28_e487_d_n12: f64 = (-s.dn[547][12]);
        let eq28_e487_d_b0: f64 = (-s.db[547][0]);
        let eq28_e487_d_b1: f64 = (-s.db[547][1]);
        let eq28_e487_d_b2: f64 = (-s.db[547][2]);
        let eq28_e487_d_b3: f64 = (-s.db[547][3]);
        let eq28_e487_d_b4: f64 = (-s.db[547][4]);
        let eq28_e487_d_b5: f64 = (-s.db[547][5]);
        let eq28_e487_d_b6: f64 = (-s.db[547][6]);
        let eq28_e487_d_b7: f64 = (-s.db[547][7]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e490_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));
        let eq28_e490_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));
        let eq28_e490_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));
        let eq28_e490_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));
        let eq28_e491: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq28_e490);
        let eq28_e491_d_n0: f64 = (eq28_e490_d_n0 * ddt_scale);
        let eq28_e491_d_n1: f64 = (eq28_e490_d_n1 * ddt_scale);
        let eq28_e491_d_n2: f64 = (eq28_e490_d_n2 * ddt_scale);
        let eq28_e491_d_n3: f64 = (eq28_e490_d_n3 * ddt_scale);
        let eq28_e491_d_n4: f64 = (eq28_e490_d_n4 * ddt_scale);
        let eq28_e491_d_n5: f64 = (eq28_e490_d_n5 * ddt_scale);
        let eq28_e491_d_n6: f64 = (eq28_e490_d_n6 * ddt_scale);
        let eq28_e491_d_n7: f64 = (eq28_e490_d_n7 * ddt_scale);
        let eq28_e491_d_n8: f64 = (eq28_e490_d_n8 * ddt_scale);
        let eq28_e491_d_n9: f64 = (eq28_e490_d_n9 * ddt_scale);
        let eq28_e491_d_n10: f64 = (eq28_e490_d_n10 * ddt_scale);
        let eq28_e491_d_n11: f64 = (eq28_e490_d_n11 * ddt_scale);
        let eq28_e491_d_n12: f64 = (eq28_e490_d_n12 * ddt_scale);
        let eq28_e491_d_b0: f64 = (eq28_e490_d_b0 * ddt_scale);
        let eq28_e491_d_b1: f64 = (eq28_e490_d_b1 * ddt_scale);
        let eq28_e491_d_b2: f64 = (eq28_e490_d_b2 * ddt_scale);
        let eq28_e491_d_b3: f64 = (eq28_e490_d_b3 * ddt_scale);
        let eq28_e491_d_b4: f64 = (eq28_e490_d_b4 * ddt_scale);
        let eq28_e491_d_b5: f64 = (eq28_e490_d_b5 * ddt_scale);
        let eq28_e491_d_b6: f64 = (eq28_e490_d_b6 * ddt_scale);
        let eq28_e491_d_b7: f64 = (eq28_e490_d_b7 * ddt_scale);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = (eq28_e487_d_n0 + eq28_e491_d_n0);
        let eq28_e492_d_n1: f64 = (eq28_e487_d_n1 + eq28_e491_d_n1);
        let eq28_e492_d_n2: f64 = (eq28_e487_d_n2 + eq28_e491_d_n2);
        let eq28_e492_d_n3: f64 = (eq28_e487_d_n3 + eq28_e491_d_n3);
        let eq28_e492_d_n4: f64 = (eq28_e487_d_n4 + eq28_e491_d_n4);
        let eq28_e492_d_n5: f64 = (eq28_e487_d_n5 + eq28_e491_d_n5);
        let eq28_e492_d_n6: f64 = (eq28_e487_d_n6 + eq28_e491_d_n6);
        let eq28_e492_d_n7: f64 = (eq28_e487_d_n7 + eq28_e491_d_n7);
        let eq28_e492_d_n8: f64 = (eq28_e487_d_n8 + eq28_e491_d_n8);
        let eq28_e492_d_n9: f64 = (eq28_e487_d_n9 + eq28_e491_d_n9);
        let eq28_e492_d_n10: f64 = (eq28_e487_d_n10 + eq28_e491_d_n10);
        let eq28_e492_d_n11: f64 = (eq28_e487_d_n11 + eq28_e491_d_n11);
        let eq28_e492_d_n12: f64 = (eq28_e487_d_n12 + eq28_e491_d_n12);
        let eq28_e492_d_b0: f64 = (eq28_e487_d_b0 + eq28_e491_d_b0);
        let eq28_e492_d_b1: f64 = (eq28_e487_d_b1 + eq28_e491_d_b1);
        let eq28_e492_d_b2: f64 = (eq28_e487_d_b2 + eq28_e491_d_b2);
        let eq28_e492_d_b3: f64 = (eq28_e487_d_b3 + eq28_e491_d_b3);
        let eq28_e492_d_b4: f64 = (eq28_e487_d_b4 + eq28_e491_d_b4);
        let eq28_e492_d_b5: f64 = (eq28_e487_d_b5 + eq28_e491_d_b5);
        let eq28_e492_d_b6: f64 = (eq28_e487_d_b6 + eq28_e491_d_b6);
        let eq28_e492_d_b7: f64 = (eq28_e487_d_b7 + eq28_e491_d_b7);
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e495_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);
        let eq28_e495_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);
        let eq28_e495_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);
        let eq28_e495_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_d_b4: f64 = (eq28_e492_d_b4 + eq28_e495_d_b4);
        let eq28_e496_d_b5: f64 = (eq28_e492_d_b5 + eq28_e495_d_b5);
        let eq28_e496_d_b6: f64 = (eq28_e492_d_b6 + eq28_e495_d_b6);
        let eq28_e496_d_b7: f64 = (eq28_e492_d_b7 + eq28_e495_d_b7);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        let eq28_node_derivatives: [f64; 13] = [eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12];
        let eq28_branch_derivatives: [f64; 8] = [eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e503,) = {
    if (!s.b[1094]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e503;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7,) = {
    if s.b[1095] {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e508_d_n10: f64 = 1e-9;
        let eq30_e509: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq30_e508);
        let eq30_e509_d_n10: f64 = (eq30_e508_d_n10 * ddt_scale);
        let eq30_e510: f64 = (s.v[558] + eq30_e509);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + eq30_e509_d_n10);
        (eq30_e510, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e510_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        let eq30_node_derivatives: [f64; 13] = [eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12];
        let eq30_branch_derivatives: [f64; 8] = [eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e517,) = {
    if (!s.b[1095]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e517;
        stamper.stamp_potential_const_local(
            5,
            eq31_value,
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e522_d_n8: f64 = 1e-9;
        let eq32_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq32_e522);
        let eq32_e523_d_n8: f64 = (eq32_e522_d_n8 * ddt_scale);
        let eq32_e524: f64 = (s.v[549] + eq32_e523);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + eq32_e523_d_n8);
        (eq32_e524, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e524_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        let eq32_node_derivatives: [f64; 13] = [eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12];
        let eq32_branch_derivatives: [f64; 8] = [eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e531_d_n9: f64 = 1e-9;
        let eq33_e532: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq33_e531);
        let eq33_e532_d_n9: f64 = (eq33_e531_d_n9 * ddt_scale);
        let eq33_e533: f64 = (s.v[550] + eq33_e532);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + eq33_e532_d_n9);
        (eq33_e533, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e533_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        let eq33_node_derivatives: [f64; 13] = [eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12];
        let eq33_branch_derivatives: [f64; 8] = [eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7];
        stamper.stamp_current_dense_local(
            Some(9),
            None,
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e540,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e540;
        stamper.stamp_potential_const_local(
            6,
            eq34_value,
        );
        let (eq35_e545,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e545;
        stamper.stamp_potential_const_local(
            7,
            eq35_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e387_d_b4: f64 = (s.db[561][4] + s.db[554][4]);
        let eq10_e387_d_b5: f64 = (s.db[561][5] + s.db[554][5]);
        let eq10_e387_d_b6: f64 = (s.db[561][6] + s.db[554][6]);
        let eq10_e387_d_b7: f64 = (s.db[561][7] + s.db[554][7]);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_e389_d_b4: f64 = (p.p33 * eq10_e387_d_b4);
        let eq10_e389_d_b5: f64 = (p.p33 * eq10_e387_d_b5);
        let eq10_e389_d_b6: f64 = (p.p33 * eq10_e387_d_b6);
        let eq10_e389_d_b7: f64 = (p.p33 * eq10_e387_d_b7);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_e389_q_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_q_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_q_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_q_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_q_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_q_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_q_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_q_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_q_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_q_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_q_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_q_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_q_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_q_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_q_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_q_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_q_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_e389_q_d_b4: f64 = (p.p33 * eq10_e387_d_b4);
        let eq10_e389_q_d_b5: f64 = (p.p33 * eq10_e387_d_b5);
        let eq10_e389_q_d_b6: f64 = (p.p33 * eq10_e387_d_b6);
        let eq10_e389_q_d_b7: f64 = (p.p33 * eq10_e387_d_b7);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_q_d_n0, eq10_e389_q_d_n1, eq10_e389_q_d_n2, eq10_e389_q_d_n3, eq10_e389_q_d_n4, eq10_e389_q_d_n5, eq10_e389_q_d_n6, eq10_e389_q_d_n7, eq10_e389_q_d_n8, eq10_e389_q_d_n9, eq10_e389_q_d_n10, eq10_e389_q_d_n11, eq10_e389_q_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 8] = [eq10_e389_q_d_b0, eq10_e389_q_d_b1, eq10_e389_q_d_b2, eq10_e389_q_d_b3, eq10_e389_q_d_b4, eq10_e389_q_d_b5, eq10_e389_q_d_b6, eq10_e389_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e393_d_b4: f64 = (s.db[93][4] + s.db[552][4]);
        let eq11_e393_d_b5: f64 = (s.db[93][5] + s.db[552][5]);
        let eq11_e393_d_b6: f64 = (s.db[93][6] + s.db[552][6]);
        let eq11_e393_d_b7: f64 = (s.db[93][7] + s.db[552][7]);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_e395_d_b4: f64 = (p.p33 * eq11_e393_d_b4);
        let eq11_e395_d_b5: f64 = (p.p33 * eq11_e393_d_b5);
        let eq11_e395_d_b6: f64 = (p.p33 * eq11_e393_d_b6);
        let eq11_e395_d_b7: f64 = (p.p33 * eq11_e393_d_b7);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_e395_q_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_q_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_q_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_q_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_q_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_q_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_q_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_q_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_q_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_q_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_q_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_q_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_q_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_q_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_q_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_q_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_q_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_e395_q_d_b4: f64 = (p.p33 * eq11_e393_d_b4);
        let eq11_e395_q_d_b5: f64 = (p.p33 * eq11_e393_d_b5);
        let eq11_e395_q_d_b6: f64 = (p.p33 * eq11_e393_d_b6);
        let eq11_e395_q_d_b7: f64 = (p.p33 * eq11_e393_d_b7);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_q_d_n0, eq11_e395_q_d_n1, eq11_e395_q_d_n2, eq11_e395_q_d_n3, eq11_e395_q_d_n4, eq11_e395_q_d_n5, eq11_e395_q_d_n6, eq11_e395_q_d_n7, eq11_e395_q_d_n8, eq11_e395_q_d_n9, eq11_e395_q_d_n10, eq11_e395_q_d_n11, eq11_e395_q_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 8] = [eq11_e395_q_d_b0, eq11_e395_q_d_b1, eq11_e395_q_d_b2, eq11_e395_q_d_b3, eq11_e395_q_d_b4, eq11_e395_q_d_b5, eq11_e395_q_d_b6, eq11_e395_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e399_d_b4: f64 = (s.db[90][4] + s.db[548][4]);
        let eq12_e399_d_b5: f64 = (s.db[90][5] + s.db[548][5]);
        let eq12_e399_d_b6: f64 = (s.db[90][6] + s.db[548][6]);
        let eq12_e399_d_b7: f64 = (s.db[90][7] + s.db[548][7]);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_e401_d_b4: f64 = (p.p33 * eq12_e399_d_b4);
        let eq12_e401_d_b5: f64 = (p.p33 * eq12_e399_d_b5);
        let eq12_e401_d_b6: f64 = (p.p33 * eq12_e399_d_b6);
        let eq12_e401_d_b7: f64 = (p.p33 * eq12_e399_d_b7);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_e401_q_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_q_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_q_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_q_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_q_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_q_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_q_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_q_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_q_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_q_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_q_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_q_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_q_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_q_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_q_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_q_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_q_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_e401_q_d_b4: f64 = (p.p33 * eq12_e399_d_b4);
        let eq12_e401_q_d_b5: f64 = (p.p33 * eq12_e399_d_b5);
        let eq12_e401_q_d_b6: f64 = (p.p33 * eq12_e399_d_b6);
        let eq12_e401_q_d_b7: f64 = (p.p33 * eq12_e399_d_b7);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_q_d_n0, eq12_e401_q_d_n1, eq12_e401_q_d_n2, eq12_e401_q_d_n3, eq12_e401_q_d_n4, eq12_e401_q_d_n5, eq12_e401_q_d_n6, eq12_e401_q_d_n7, eq12_e401_q_d_n8, eq12_e401_q_d_n9, eq12_e401_q_d_n10, eq12_e401_q_d_n11, eq12_e401_q_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 8] = [eq12_e401_q_d_b0, eq12_e401_q_d_b1, eq12_e401_q_d_b2, eq12_e401_q_d_b3, eq12_e401_q_d_b4, eq12_e401_q_d_b5, eq12_e401_q_d_b6, eq12_e401_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e430_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);
        let eq18_e430_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);
        let eq18_e430_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);
        let eq18_e430_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, eq18_e430_d_n1, eq18_e430_d_n2, eq18_e430_d_n3, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, eq18_e430_d_n7, eq18_e430_d_n8, eq18_e430_d_n9, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 8] = [eq18_e430_d_b0, eq18_e430_d_b1, eq18_e430_d_b2, eq18_e430_d_b3, eq18_e430_d_b4, eq18_e430_d_b5, eq18_e430_d_b6, eq18_e430_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e434_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);
        let eq19_e434_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);
        let eq19_e434_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);
        let eq19_e434_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, eq19_e434_d_n1, eq19_e434_d_n2, eq19_e434_d_n3, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, eq19_e434_d_n7, eq19_e434_d_n8, eq19_e434_d_n9, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 8] = [eq19_e434_d_b0, eq19_e434_d_b1, eq19_e434_d_b2, eq19_e434_d_b3, eq19_e434_d_b4, eq19_e434_d_b5, eq19_e434_d_b6, eq19_e434_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12, eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7,) = {
    if s.b[1094] {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e487_d_n0: f64 = (-s.dn[547][0]);
        let eq28_e487_d_n1: f64 = (-s.dn[547][1]);
        let eq28_e487_d_n2: f64 = (-s.dn[547][2]);
        let eq28_e487_d_n3: f64 = (-s.dn[547][3]);
        let eq28_e487_d_n4: f64 = (-s.dn[547][4]);
        let eq28_e487_d_n5: f64 = (-s.dn[547][5]);
        let eq28_e487_d_n6: f64 = (-s.dn[547][6]);
        let eq28_e487_d_n7: f64 = (-s.dn[547][7]);
        let eq28_e487_d_n8: f64 = (-s.dn[547][8]);
        let eq28_e487_d_n9: f64 = (-s.dn[547][9]);
        let eq28_e487_d_n10: f64 = (-s.dn[547][10]);
        let eq28_e487_d_n11: f64 = (-s.dn[547][11]);
        let eq28_e487_d_n12: f64 = (-s.dn[547][12]);
        let eq28_e487_d_b0: f64 = (-s.db[547][0]);
        let eq28_e487_d_b1: f64 = (-s.db[547][1]);
        let eq28_e487_d_b2: f64 = (-s.db[547][2]);
        let eq28_e487_d_b3: f64 = (-s.db[547][3]);
        let eq28_e487_d_b4: f64 = (-s.db[547][4]);
        let eq28_e487_d_b5: f64 = (-s.db[547][5]);
        let eq28_e487_d_b6: f64 = (-s.db[547][6]);
        let eq28_e487_d_b7: f64 = (-s.db[547][7]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e490_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));
        let eq28_e490_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));
        let eq28_e490_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));
        let eq28_e490_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = (eq28_e487_d_n0 + eq28_e490_d_n0);
        let eq28_e492_d_n1: f64 = (eq28_e487_d_n1 + eq28_e490_d_n1);
        let eq28_e492_d_n2: f64 = (eq28_e487_d_n2 + eq28_e490_d_n2);
        let eq28_e492_d_n3: f64 = (eq28_e487_d_n3 + eq28_e490_d_n3);
        let eq28_e492_d_n4: f64 = (eq28_e487_d_n4 + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = (eq28_e487_d_n5 + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = (eq28_e487_d_n6 + eq28_e490_d_n6);
        let eq28_e492_d_n7: f64 = (eq28_e487_d_n7 + eq28_e490_d_n7);
        let eq28_e492_d_n8: f64 = (eq28_e487_d_n8 + eq28_e490_d_n8);
        let eq28_e492_d_n9: f64 = (eq28_e487_d_n9 + eq28_e490_d_n9);
        let eq28_e492_d_n10: f64 = (eq28_e487_d_n10 + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = (eq28_e487_d_n11 + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = (eq28_e487_d_n12 + eq28_e490_d_n12);
        let eq28_e492_d_b0: f64 = (eq28_e487_d_b0 + eq28_e490_d_b0);
        let eq28_e492_d_b1: f64 = (eq28_e487_d_b1 + eq28_e490_d_b1);
        let eq28_e492_d_b2: f64 = (eq28_e487_d_b2 + eq28_e490_d_b2);
        let eq28_e492_d_b3: f64 = (eq28_e487_d_b3 + eq28_e490_d_b3);
        let eq28_e492_d_b4: f64 = (eq28_e487_d_b4 + eq28_e490_d_b4);
        let eq28_e492_d_b5: f64 = (eq28_e487_d_b5 + eq28_e490_d_b5);
        let eq28_e492_d_b6: f64 = (eq28_e487_d_b6 + eq28_e490_d_b6);
        let eq28_e492_d_b7: f64 = (eq28_e487_d_b7 + eq28_e490_d_b7);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e495_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);
        let eq28_e495_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);
        let eq28_e495_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);
        let eq28_e495_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_d_b4: f64 = (eq28_e492_d_b4 + eq28_e495_d_b4);
        let eq28_e496_d_b5: f64 = (eq28_e492_d_b5 + eq28_e495_d_b5);
        let eq28_e496_d_b6: f64 = (eq28_e492_d_b6 + eq28_e495_d_b6);
        let eq28_e496_d_b7: f64 = (eq28_e492_d_b7 + eq28_e495_d_b7);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n1, eq28_e490_d_n2, eq28_e490_d_n3, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n7, eq28_e490_d_n8, eq28_e490_d_n9, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12, eq28_e490_d_b0, eq28_e490_d_b1, eq28_e490_d_b2, eq28_e490_d_b3, eq28_e490_d_b4, eq28_e490_d_b5, eq28_e490_d_b6, eq28_e490_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 8] = [eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if s.b[1095] {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e508_d_n10: f64 = 1e-9;
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (s.v[558] + eq30_e508);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + eq30_e508_d_n10);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e510_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7], eq30_e510_q, eq30_e508_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e512_q_d_n10),
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e522_d_n8: f64 = 1e-9;
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (s.v[549] + eq32_e522);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + eq32_e522_d_n8);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e524_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7], eq32_e524_q, eq32_e522_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (eq32_e526_q_d_n8),
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e531_d_n9: f64 = 1e-9;
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (s.v[550] + eq33_e531);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + eq33_e531_d_n9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e533_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7], eq33_e533_q, eq33_e531_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq33_e535_q_d_n9),
        );
    }
}
