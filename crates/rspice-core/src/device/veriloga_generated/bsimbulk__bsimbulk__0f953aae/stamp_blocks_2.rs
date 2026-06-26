#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1344] {
            let assign15280_ad_e22663: A = {
                if (!(((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0), A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15280_ad_e22663, 10.0);
        }

        if s.b[1344] {
            s.store_sub_from_scalar_ad(347, (-p.p731), A::mul(s.ad_value(343), A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p733)), 1e-38))));
            s.store_scaled_limited_exp_ad(13, A::div_scaled_inputs(A::offset(s.ad_value(347), p.p731), -1.0, s.ad_value(343), 1.0), p.p733);
            s.store_mul_offset_rhs(346, 341, 13, 1.0);
            s.store_div_scaled_product_indices(345, 341, 13, -1.0, 343, 1.0);
        }

        if (!s.b[1344]) {
            s.store_scalar(343, 0.0);
            s.store_scalar(351, 0.0);
            s.store_scalar(350, 0.0);
            s.store_scalar(349, 0.0);
            s.store_scalar(348, 0.0);
            s.store_scalar(347, 0.0);
            s.store_scalar(346, 0.0);
            s.store_scalar(345, 0.0);
        }

        s.store_add_scaled_ad_lhs(342, A::add_scaled_products(s.ad_value(251), s.ad_value(438), 1.0, s.ad_value(301), s.ad_value(439), 1.0), 440, (s.v[35] * p.p2));

        s.b[1345] = (s.v[342] > 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if s.b[1345] {
            s.store_scale(344, 393, p.p726);
            s.store_scaled_limited_exp_ad(358, A::div_from_scalar((-p.p732), s.ad_value(344)), p.p734);
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p728, s.ad_value(342)), 10.0);
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 358);
            s.store_mul_ln_ad_rhs(357, 344, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(358), 4.0)), 0.5), 1e-38));
            s.store_limited_exp_div(12, 357, 344);
            s.store_mul_offset_ad_rhs(356, 342, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(358), s.ad_value(12)), (-1.0), s.ad_value(358), 1.0), (-1.0));
            s.store_div_scaled_product_right_ad(355, 342, A::add(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12))), 1.0, 344, 1.0);
        }

        if s.b[1345] {
            let assign15510_ad_e22914: A = {
                if (!(((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0), A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15510_ad_e22914, 10.0);
        }

        if s.b[1345] {
            s.store_sub_from_scalar_ad(354, (-p.p732), A::mul(s.ad_value(344), A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p734)), 1e-38))));
            s.store_scaled_limited_exp_ad(13, A::div_scaled_inputs(A::offset(s.ad_value(354), p.p732), -1.0, s.ad_value(344), 1.0), p.p734);
            s.store_mul_offset_rhs(353, 342, 13, 1.0);
            s.store_div_scaled_product_indices(352, 342, 13, -1.0, 344, 1.0);
        }

        if (!s.b[1345]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(358, 0.0);
            s.store_scalar(357, 0.0);
            s.store_scalar(356, 0.0);
            s.store_scalar(355, 0.0);
            s.store_scalar(354, 0.0);
            s.store_scalar(353, 0.0);
            s.store_scalar(352, 0.0);
        }

        s.b[1346] = (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0))));
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if s.b[1346] {
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p921));
            s.store_scalar(643, (s.v[100] + p.p914));
            s.store_powf(13, 643, p.p922);
            s.store_add_scaled_inputs3(644, A::div_from_scalar(p.p918, s.ad_value(12)), 1.0, A::div_from_scalar(p.p919, s.ad_value(13)), 1.0, A::div_from_scalar(p.p920, A::mul(s.ad_value(12), s.ad_value(13))), 1.0);
            s.store_offset(645, 644, 1.0);
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p927));
            s.store_powf(13, 643, p.p928);
            s.store_add_scaled_inputs3(646, A::div_from_scalar(p.p924, s.ad_value(12)), 1.0, A::div_from_scalar(p.p925, s.ad_value(13)), 1.0, A::div_from_scalar(p.p926, A::mul(s.ad_value(12), s.ad_value(13))), 1.0);
            s.store_offset(647, 646, 1.0);
            s.store_offset(12, 395, (-1.0));
            s.store_offset_mul_ad(648, s.ad_value(645), A::scale_offset(s.ad_value(12), p.p917, 1.0), 1e-9);
            s.store_scalar(662, 0.0);
        }

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if (s.b[1346] && (s.v[662] < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1346] {
                s.store_div_from_scalar_offset_scaled_input(12, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p17 + (0.5 * s.v[99])));
                s.store_div_from_scalar_offset_scaled_input(13, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p18 + (0.5 * s.v[99])));
                s.store_offset(649, 12, s.v[649]);
                s.store_offset(650, 13, s.v[650]);
                s.store_offset(662, 662, 1.0);
            }
        }

        if s.b[1346] {
            s.store_scalar(651, (1.0 / (p.p912 + (0.5 * s.v[99]))));
            s.store_scalar(652, (1.0 / (p.p913 + (0.5 * s.v[99]))));
            s.store_add(653, 651, 652);
            s.store_mul_div_from_scalar_lhs(654, p.p915, 648, 653);
            s.store_add(655, 649, 650);
            s.store_mul_div_from_scalar_lhs(656, p.p915, 648, 655);
            s.store_div_scaled_offset_numerator(657, s.ad_value(656), 1.0, 1.0, A::offset(s.ad_value(654), 1.0), 1.0);
            s.store_div_scaled_offset_numerator(658, s.ad_value(656), p.p916, 1.0, A::scale_offset(s.ad_value(654), p.p916, 1.0), 1.0);
            s.store_mul_ad(659, A::div_from_scalar(p.p923, s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(660, A::div_from_scalar(p.p929, A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(661, A::div_from_scalar(p.p931, A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul(397, 397, 657);
            s.store_mul(409, 409, 658);
            s.store_add(494, 494, 660);
            s.store_add(420, 420, 661);
        }

        s.b[1347] = (p.p37 == 1.0);
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if (s.b[1346] && s.b[1347]) {
            s.store_mul_ad(688, A::div(s.ad_value(625), s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(689, A::div(s.ad_value(626), A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(690, A::div(s.ad_value(627), A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if s.b[1346] {
            s.store_add(624, 624, 689);
            s.store_add(616, 616, 690);
        }

        if (!s.b[1346]) {
            s.store_scalar(659, 0.0);
            s.store_scalar(688, 0.0);
        }

        s.b[1348] = (p.p43 == 1.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if s.b[1348] {
            s.store_scalar(668, (p.p1 / p.p2));
            s.store_scalar(669, p.p20);
            s.store_scalar(670, p.p21);
            s.store_scalar(671, p.p22);
        }

        s.b[1349] = (((!param_given[20]) && (!param_given[21])) && (!param_given[22]));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        s.b[1350] = (param_given[23] && (p.p23 > 0.0));
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1349]) && s.b[1350]) {
            s.store_offset(13, 668, p.p23);
            s.store_scalar(14, (1.0 / p.p947));
            s.store_div_from_scalar_scaled_input(669, (p.p947 * p.p947), 13, p.p23);
            s.store_div_scaled_add_product(670, A::limited_exp_scaled_input(s.ad_value(14), ((-10.0) * p.p23)), ((0.1 * p.p23) + (0.01 * p.p947)), A::scale_offset(s.ad_value(13), 0.1, (0.01 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-10.0), s.ad_value(14))), (-1.0), s.ad_value(668), 1.0);
            s.store_div_scaled_add_product(671, A::limited_exp_scaled_input(s.ad_value(14), ((-20.0) * p.p23)), ((0.05 * p.p23) + (0.0025 * p.p947)), A::scale_offset(s.ad_value(13), 0.05, (0.0025 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-20.0), s.ad_value(14))), (-1.0), s.ad_value(668), 1.0);
        }

        s.store_mul_ad_rhs(663, 578, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(664, 579, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(665, 630, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(666, 629, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_offset_mul_ad(667, s.ad_value(580), A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934), 1.0);

        s.store_mul(397, 397, 667);

        s.store_add(494, 494, 664);

        s.store_mul_voltage_ad(64, s.ad_value(187), ctx, nodes, Some(9), Some(11));

        s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(5), Some(11));

        s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(7), Some(11));

        s.store_sub(74, 66, 70);

        s.copy_ad(68, 66);

        s.copy_ad(56, 74);

        s.copy_ad(50, 70);

        s.copy_ad(48, 66);

        s.store_mul_voltage_ad(306, s.ad_value(187), ctx, nodes, Some(12), Some(7));

        s.store_mul_voltage_ad(307, s.ad_value(187), ctx, nodes, Some(13), Some(5));

        s.store_mul_voltage_ad(308, s.ad_value(187), ctx, nodes, Some(13), Some(5));

        s.store_mul_voltage_ad(309, s.ad_value(187), ctx, nodes, Some(13), Some(14));

        s.store_sub(54, 64, 66);

        s.store_sub(52, 64, 70);

        s.store_mul_voltage_ad(230, s.ad_value(187), ctx, nodes, Some(10), Some(5));

        s.store_mul_voltage_ad(231, s.ad_value(187), ctx, nodes, Some(10), Some(7));

        s.copy_ad(232, 230);

        s.b[1351] = ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if s.b[1351] {
            s.store_add_scaled_product_right_ad(68, 66, 1.0, 187, A::voltage(ctx, nodes, Some(6), Some(5)), (1.0 - (p.p1111 / p.p1110)));
            s.store_add_scaled_inputs3(308, s.ad_value(307), 1.0, s.ad_value(66), 1.0, s.ad_value(68), -1.0);
            s.store_add_scaled_inputs3(232, s.ad_value(230), 1.0, s.ad_value(66), 1.0, s.ad_value(68), -1.0);
        }

        s.copy_ad(69, 68);

        s.store_mul_voltage_ad(72, s.ad_value(187), ctx, nodes, Some(7), Some(11));

        s.v[57] = 1.0;

        s.b[1352] = (s.v[74] < 0.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if s.b[1352] {
            s.store_scalar(57, (-1.0));
            s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(7), Some(11));
            s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(5), Some(11));
            s.copy_ad(72, 69);
            s.store_mul_voltage_ad(68, s.ad_value(187), ctx, nodes, Some(7), Some(11));
        }

        s.store_sub(74, 66, 70);

        s.store_sub(75, 68, 72);

        s.store_scale(12, 75, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub_scaled_inputs(s.ad_value(13), (2.0 / p.p956), s.ad_value(75), 1.0), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

        s.store_neg_ad(62, A::add_scaled_inputs3(s.ad_value(72), 1.0, s.ad_value(75), 0.5, s.ad_value(76), (-0.5)));

        s.store_scale(12, 74, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub_scaled_inputs(s.ad_value(13), (2.0 / p.p956), s.ad_value(74), 1.0), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_neg_ad(61, A::add_scaled_inputs3(s.ad_value(70), 1.0, s.ad_value(74), 0.5, s.ad_value(76), (-0.5)));

        s.store_tanh_ad(12, A::div_scaled_inputs(s.ad_value(56), p.p1123, s.ad_value(393), 1.0));

        s.store_offset_scaled(102, 12, 0.5, 0.5);

        s.store_sub_from_scalar(103, 1.0, 102);

        s.b[1353] = (p.p44 != 0.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if s.b[1353] {
            s.store_add_scaled_products_indices(486, 485, 103, 1.0, 484, 102, 1.0);
            s.store_add_scaled_products_indices(492, 421, 103, 1.0, 420, 102, 1.0);
            s.store_add_scaled_products_indices(519, 518, 103, 1.0, 517, 102, 1.0);
            s.store_add_scaled_products_indices(541, 540, 103, 1.0, 539, 102, 1.0);
            s.store_add_scaled_products_indices(166, 165, 103, 1.0, 164, 102, 1.0);
            s.store_add_scaled_products_indices(502, 410, 103, 1.0, 409, 102, 1.0);
            s.store_add_scaled_products_indices(536, 414, 103, 1.0, 413, 102, 1.0);
            s.store_add_scaled_products_indices(499, 398, 103, 1.0, 397, 102, 1.0);
            s.store_add_scaled_products_indices(506, 400, 103, 1.0, 399, 102, 1.0);
            s.store_add_scaled_products_indices(516, 402, 103, 1.0, 401, 102, 1.0);
            s.store_add_scaled_products_indices(510, 404, 103, 1.0, 403, 102, 1.0);
            s.store_add_scaled_products_indices(513, 406, 103, 1.0, 405, 102, 1.0);
            s.store_add_scaled_products_indices(553, 552, 103, 1.0, 551, 102, 1.0);
            s.store_add_scaled_products_indices(558, 416, 103, 1.0, 415, 102, 1.0);
        }

        if (!s.b[1353]) {
            s.copy_ad(486, 484);
            s.copy_ad(492, 420);
            s.copy_ad(519, 517);
            s.copy_ad(541, 539);
            s.copy_ad(166, 164);
            s.copy_ad(502, 409);
            s.copy_ad(536, 413);
            s.copy_ad(499, 397);
            s.copy_ad(506, 399);
            s.copy_ad(516, 401);
            s.copy_ad(510, 403);
            s.copy_ad(513, 405);
            s.copy_ad(553, 551);
            s.copy_ad(558, 415);
        }

        s.b[1354] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if s.b[1354] {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));
        }

        if (!s.b[1354]) {
            s.store_add_scaled_inputs3_offset(110, s.ad_value(127), 0.5, s.ad_value(61), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
        }

        s.store_sqrt(111, 110);

        s.store_mul(112, 114, 111);

        s.store_div_from_scalar(97, s.v[26], 112);

        s.store_ad_value(113, A::add_scaled_inputs_products(s.ad_value(483), 1.0, s.ad_value(422), 1.0, s.ad_value(486), s.ad_value(76), 1.0, s.ad_value(487), s.ad_value(61), (-1.0)));

        s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);

        s.b[1355] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if s.b[1355] {
            s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);
        }

        if (!s.b[1355]) {
            s.store_scaled_add_ad(104, A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(13), (-1.0), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        s.store_mul(106, 104, 108);

        s.store_div_from_scalar(107, 1.0, 106);

        s.store_mul_neg_ad_lhs(123, A::add_scaled_product(s.ad_value(492), 1.0, s.ad_value(493), s.ad_value(61), 1.0), 76);

        s.store_offset_ad(123, A::sub_scaled_inputs(s.ad_value(123), 0.5, A::sqrt(A::offset(A::mul(s.ad_value(123), s.ad_value(123)), ((0.25 * 0.005) * 0.005))), 0.5), (0.25 * 0.005));

        s.store_ad_value(124, A::mul_offset_rhs(A::add_scaled_product(A::offset(s.ad_value(454), (p.p869 / s.v[30])), 1.0, s.ad_value(455), s.ad_value(61), 1.0), A::powf(s.ad_value(395), p.p868), (-1.0)));

        s.b[1356] = (s.v[116] > 0.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if s.b[1356] {
            s.store_mul_neg_lhs(12, 117, 76);
        }

        s.b[1357] = (s.v[12] < (-80.0));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if (s.b[1356] && s.b[1357]) {
            s.store_scalar(14, 1.804851387e-35);
        }

        if (s.b[1356] && (!s.b[1357])) {
            s.store_limited_exp(14, 12);
        }

        if s.b[1356] {
            s.store_offset_ad(15, A::mul_offset_rhs(s.ad_value(116), s.ad_value(14), 1.0), s.v[30]);
            s.store_mul_scaled_ad_rhs(115, 106, -1.0, A::ln(A::max_with_scalar(A::div_from_scalar(s.v[30], s.ad_value(15)), 1e-38)));
        }

        if (!s.b[1356]) {
            s.store_scalar(115, 0.0);
        }

        s.store_add_ad_rhs(16, 121, A::div(s.ad_value(118), A::pow_from_scalar(s.v[30], s.ad_value(119))));

        s.store_add_scaled_product_right_ad(115, 115, 1.0, 16, A::tanh(A::mul(s.ad_value(120), s.ad_value(76))), (-1.0));

        s.store_offset(482, 482, p.p35);

        s.store_mul(65, 64, 107);

        s.store_mul(73, 70, 107);

        s.store_mul(58, 482, 107);

        s.store_add_scaled_products_left_right_ad(122, 495, A::sub(s.ad_value(111), s.ad_value(128)), 1.0, 494, 61, (-1.0));

        s.store_add_ad_lhs(79, A::add(A::add_scaled_inputs4(s.ad_value(123), 1.0, s.ad_value(115), 1.0, s.ad_value(122), 1.0, s.ad_value(124), -1.0), s.ad_value(659)), 663);

        s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));

        s.store_scaled_sqrt_ad(125, A::mul_scaled_lhs(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(109)), 1.0 / (s.v[46]));

        if (!(((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_ad(12, A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5);
        } else {
            if (((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(12, ((-0.001) * 0.001), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0));
            } else {
                s.store_scalar(12, 0.0);
            }
        }

        s.store_offset_ad(90, A::div_scaled_inputs(s.ad_value(125), 1.0, A::sqrt(s.ad_value(12)), 2.0), 1.0);

        s.store_scaled_sqrt_ad(125, A::mul_scaled_lhs(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(107)), 1.0 / (s.v[46]));

        s.store_div_from_scalar(126, 1.0, 125);

        s.store_div(89, 88, 104);

        s.v[13] = 1.0;

        s.store_scale(204, 59, 1.0 / (s.v[13]));

        s.store_scale(205, 125, 1.0 / (s.v[13]));

        s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));

        s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));

        s.b[1358] = (s.v[204] < 0.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if s.b[1358] {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (!s.b[1358]) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(125), 1.0);

        s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(89), (-2.0), s.ad_value(73), -1.0);

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));

        s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);

        s.copy_ad(94, 96);

        s.b[1359] = (s.v[20] <= (-68.0));
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if s.b[1359] {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1360] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if (s.b[1359] && s.b[1360]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1361] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if ((s.b[1359] && (!s.b[1360])) && s.b[1361]) {
            s.store_limited_exp(15, 20);
        }

        if ((s.b[1359] && (!s.b[1360])) && (!s.b[1361])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if s.b[1359] {
            s.store_mul_ad_rhs(200, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (!s.b[1359]) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1362] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if s.b[1362] {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (!s.b[1362]) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_sqrt(96, 93);

        s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);

        s.b[1363] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if s.b[1363] {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (!s.b[1363]) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);

        s.v[155] = (1e-8 / (s.v[47] * p.p77));

        s.store_mul_ad_rhs(12, 106, A::add_scaled_inputs_product(s.ad_value(59), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));

        s.b[1364] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if s.b[1364] {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

        if (!s.b[1364]) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        s.store_mul3_affine_lhs(130, 90, 106, 2.0, 0.0, 200);

        s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));

        s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(130), s.ad_value(84)), 1.0, 0.5), s.ad_value(513));

        s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(132), s.ad_value(407)), 1.0);

        s.store_offset(16, 15, 1.0);

        s.b[1365] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if s.b[1365] {
            s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (!s.b[1365]) {
            s.store_scaled_add_ad(133, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        s.store_div_from_scalar_scaled_ad(235, 1.0, A::pow_from_scalar((s.v[29] * 1000000.0), s.ad_value(527)), p.p2);

        s.b[1366] = (p.p42 == 1.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if s.b[1366] {
            s.store_scalar(243, 0.0);
        }

        if (!s.b[1366]) {
            s.store_offset_mul(12, 526, 130, 1.0);
            s.store_mul_sub_rhs(13, 543, 111, 128);
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
            s.store_add_ad_rhs(15, 14, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01)));
        }

        s.b[1367] = (p.p42 == 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_mul_ad_affine_product_lhs(243, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2, 0.0, 408);
        }

        if ((!s.b[1366]) && (!s.b[1367])) {
            s.store_mul_add_ad_lhs(243, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), s.ad_value(240), 408);
        }

        s.store_pow_ad(12, s.ad_value(133), A::div_from_scalar(1.0, s.ad_value(166)));

        s.store_mul(23, 453, 61);

        s.store_sqrt_square_offset(24, 23, 0.1);

        s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(23), A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);

        s.store_div_scaled_product_offset_denominator(14, s.ad_value(200), s.ad_value(13), (10.0 * p.p433), A::mul(s.ad_value(200), s.ad_value(13)), (10.0 * p.p433), 1.0);

        s.b[1368] = (s.v[536] < 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if s.b[1368] {
            s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);
        }

        if (!s.b[1368]) {
            s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);
        }

        s.b[1369] = (s.v[243] > 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if s.b[1369] {
            s.store_mul3_affine_lhs(23, 90, 106, ((s.v[29] * 2.0) * s.v[46]), 0.0, 502);
            s.store_div_scaled_product3_indices(24, 23, 138, 243, 1.0, 106, 2.0);
            s.store_div_scaled_product_offset_denominator(12, s.ad_value(138), A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1370] = (s.v[13] != 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1369] && s.b[1370]) {
            s.store_asinh(147, 13);
            s.store_add_scaled_product_left_ad(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);
        }

        if (s.b[1369] && (!s.b[1370])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_add_scaled_value_products(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, s.ad_value(12), s.ad_value(15), 1.0, s.ad_value(138), A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));
        }

        s.b[1371] = (s.v[13] != 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1371]) {
            s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);
        }

        if (s.b[1369] && (!s.b[1371])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(18, A::add_scaled_value_products3(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0, s.ad_value(24), A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1372] = (s.v[13] != 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1372]) {
            s.store_asinh(147, 13);
            s.store_add_scaled_product_left_ad(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);
        }

        if (s.b[1369] && (!s.b[1372])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_add_scaled_value_products(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, s.ad_value(12), s.ad_value(15), 1.0, s.ad_value(138), A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));
        }

        s.b[1373] = (s.v[13] != 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1373]) {
            s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);
        }

        if (s.b[1369] && (!s.b[1373])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(18, A::add_scaled_value_products3(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0, s.ad_value(24), A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        if (!s.b[1369]) {
            s.store_div_scaled_product_offset_denominator(12, s.ad_value(138), A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1374] = (s.v[13] != 0.0);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1374]) {
            s.store_asinh(147, 13);
            s.store_add_scaled_product_left_ad(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);
        }

        if ((!s.b[1369]) && (!s.b[1374])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_add_scaled_products_right_right_ad(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));
        }

        s.b[1375] = (s.v[13] != 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1375]) {
            s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);
        }

        if ((!s.b[1369]) && (!s.b[1375])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_add_scaled_value_products(18, s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1376] = (s.v[13] != 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1376]) {
            s.store_asinh(147, 13);
            s.store_add_scaled_product_left_ad(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);
        }

        if ((!s.b[1369]) && (!s.b[1376])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_add_scaled_products_right_right_ad(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));
        }

        s.b[1377] = (s.v[13] != 0.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1377]) {
            s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);
        }

        if ((!s.b[1369]) && (!s.b[1377])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_add_scaled_value_products(18, s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        s.store_add_scaled_inputs4(143, s.ad_value(91), 1.0, s.ad_value(89), (-2.0), s.ad_value(131), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(125), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);

        s.store_mul(136, 143, 106);

        s.b[1378] = ((p.p1130 == 0.0) && (p.p1131 == 0.0));
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if s.b[1378] {
            s.store_scalar(782, 1.0);
        }

        if (!s.b[1378]) {
            s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);
            s.store_offset_ad(782, A::div_scaled_inputs2(s.ad_value(13), p.p1130, A::mul3_scaled_output(s.ad_value(13), A::powf(s.ad_value(200), p.p1132), s.ad_value(106), p.p1131), (-1.0), A::scale_offset(s.ad_value(61), p.p1133, 1.0), 1.0), 1.0);
        }

        s.b[1379] = ((0.1 == 0.0) && (s.v[782] < ((-2500.0) * 0.0005)));
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1379]) {
            s.store_div_from_scalar_scaled_input(782, ((-0.0005) * 0.0005), 782, 16.0);
        }

        if ((!s.b[1378]) && (!s.b[1379])) {
            s.store_scaled_add_ad(782, A::offset(s.ad_value(782), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(782), (-0.1), A::offset(s.ad_value(782), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        s.b[1380] = ((0.0 == 0.0) && ((s.v[136] - s.v[70]) < ((-2500.0) * 0.001)));
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if s.b[1380] {
            s.store_div_from_scalar_ad(140, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(136), 16.0, s.ad_value(70), 16.0));
        }

        if (!s.b[1380]) {
            s.store_add_scaled_inputs3(140, s.ad_value(136), 0.5, s.ad_value(70), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(136), s.ad_value(70)), A::sub(s.ad_value(136), s.ad_value(70))), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        s.store_div(140, 140, 782);

        s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(140)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));

        s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));

        s.store_mul(139, 74, 20);

        s.store_mul_add_lhs(142, 139, 70, 107);

        s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(125), 1.0);

        s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(89), (-2.0), s.ad_value(142), -1.0);

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));

        s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);

        s.copy_ad(94, 96);

        s.b[1381] = (s.v[20] <= (-68.0));
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if s.b[1381] {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1382] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (s.b[1381] && s.b[1382]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1383] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((s.b[1381] && (!s.b[1382])) && s.b[1383]) {
            s.store_limited_exp(15, 20);
        }

        if ((s.b[1381] && (!s.b[1382])) && (!s.b[1383])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if s.b[1381] {
            s.store_mul_ad_rhs(144, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (!s.b[1381]) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.store_add_scaled_inputs3_offset(92, s.ad_value(91), 1.0, s.ad_value(200), (-1.0), s.ad_value(144), -1.0, (-1.0));

        s.b[1384] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if s.b[1384] {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (!s.b[1384]) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_sqrt(14, 12);

        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), s.ad_value(14)), 1.0);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_div_from_scalar_add_ad(12, 1.0, A::offset(s.ad_value(200), 1.0), s.ad_value(144));

        s.store_mul(13, 217, 12);

        s.store_add_scaled_inputs_product_mixed_iiaa(189, 59, 1.0, 91, (-1.0), A::offset(s.ad_value(90), (-1.0)), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(13), 0.3333333333333333), (-1.0));

        s.store_scale(14, 90, 0.3333333333333333);

        s.store_mul(15, 13, 12);

        s.store_mul_ad_rhs(190, 14, A::add_scaled_inputs_product(s.ad_value(200), 2.0, s.ad_value(144), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 0.8, 1.0), 1.0, s.ad_value(144), 1.2), s.ad_value(15), 0.5));

        s.store_mul_ad_rhs(193, 14, A::add_scaled_inputs_product(s.ad_value(200), 1.0, s.ad_value(144), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 1.2, 1.0), 1.0, s.ad_value(144), 0.8), s.ad_value(15), 0.5));

        s.b[1385] = ((0.0 == 0.0) && ((s.v[106] * s.v[189]) < ((-2500.0) * 0.1)));
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if s.b[1385] {
            s.store_div_from_scalar_ad(81, ((-0.1) * 0.1), A::mul_scaled_output(s.ad_value(106), s.ad_value(189), 16.0));
        }

        if (!s.b[1385]) {
            s.store_add_scaled_product_value_ad(81, A::sqrt(A::offset(A::mul3(s.ad_value(106), s.ad_value(189), A::mul(s.ad_value(106), s.ad_value(189))), ((0.25 * 0.1) * 0.1))), 0.5, 106, 189, 0.5);
        }

        s.store_mul_add_rhs(80, 106, 190, 193);

        s.store_add_scaled_inputs(156, 81, s.v[155], 80, (s.v[158] * s.v[155]));

        s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(80), s.ad_value(81)), 1.0, 0.5), s.ad_value(513));

        s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(156), s.ad_value(407)), 1.0);

        s.store_offset(16, 15, 1.0);

        s.b[1386] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if s.b[1386] {
            s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (!s.b[1386]) {
            s.store_scaled_add_ad(159, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        s.store_div_scaled_inputs(134, s.ad_value(502), 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);

        s.store_scale(135, 134, s.v[30]);

        s.b[1387] = (s.v[537] > 0.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if s.b[1387] {
            s.store_offset_ad(172, A::div_scaled_product(s.ad_value(537), s.ad_value(80), 1.0, s.ad_value(135), 1.0), 1.0);
        }

        if (!s.b[1387]) {
            s.store_div_from_scalar_sub_from_scalar_ad(172, 1.0, 1.0, A::div_scaled_product(s.ad_value(537), s.ad_value(80), 1.0, s.ad_value(135), 1.0));
        }

        s.copy_ad(171, 519);

        s.store_sub(167, 74, 139);

        s.store_add_scaled_inputs(174, 80, 1.0, 106, 2.0);

        s.b[1388] = (s.v[171] > 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if s.b[1388] {
            s.store_div_ad_rhs(15, 174, A::add(s.ad_value(140), s.ad_value(174)));
        }

        if s.b[1388] {
            let assign19470_ad_e27354: A = {
                if (!((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(520), s.ad_value(61)), 1.0, A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(520), s.ad_value(61)), 1.0, 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(16, assign19470_ad_e27354);
        }

        if s.b[1388] {
            s.store_div_from_scalar(17, 1.0, 16);
            s.store_mul_ad_lhs(173, A::mul3(A::div(s.ad_value(174), s.ad_value(171)), s.ad_value(15), s.ad_value(172)), 17);
            s.store_offset_div(175, 167, 173, 1.0);
        }

        if (!s.b[1388]) {
            s.store_scalar(175, 1.0);
        }

        s.b[1389] = (s.v[525] <= 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if s.b[1389] {
            s.store_scalar(105, 1.0);
        }

        if (!s.b[1389]) {
            s.store_div_scaled_inputs(21, s.ad_value(525), ((s.v[30]) as f64).sqrt(), s.ad_value(174), 1.0);
            s.store_div_from_scalar_offset_input(105, 1.0, 21, 1.0);
        }

        s.store_add(170, 140, 135);

        s.b[1390] = (s.v[541] > 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (p.p350 < 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1391]) {
            s.store_div_scaled_value_by_product(13, s.ad_value(541), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), s.ad_value(105), 1.0);
        }

        if (s.b[1390] && (!s.b[1391])) {
            s.store_div_scaled_product_offset_rhs(13, s.ad_value(541), A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, s.ad_value(105), 1.0);
        }

        if s.b[1390] {
            s.store_offset_mul_ad(176, s.ad_value(13), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(13), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);
        }

        s.b[1392] = (p.p350 < 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1390]) && s.b[1392]) {
            s.store_div_scaled_value_by_product(13, s.ad_value(541), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), s.ad_value(105), 1.0);
        }

        if ((!s.b[1390]) && (!s.b[1392])) {
            s.store_div_scaled_product_offset_rhs(13, s.ad_value(541), A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, s.ad_value(105), 1.0);
        }

        if (!s.b[1390]) {
            s.store_offset(176, 13, 1.0);
        }

        s.store_mul(175, 175, 176);

        s.store_limited_exp_mul(13, 524, 74);

        s.b[1393] = (s.v[523] > 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if s.b[1393] {
            s.store_scalar(14, (1.0 + (p.p369 * s.v[30])));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1393] {
            s.store_div_scaled_offset_numerator(168, A::mul(s.ad_value(14), s.ad_value(13)), 1.0, 1.0, s.ad_value(523), 1.0);
            s.store_mul(168, 168, 105);
        }

        if (!s.b[1393]) {
            s.store_scalar(168, 5.540622384e34);
        }

        s.store_div(16, 167, 168);

        s.store_offset(12, 16, 1.0);

        s.store_mul(175, 175, 12);

        s.b[1394] = (s.v[522] > 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = (s.v[167] > ((s.v[521] * s.v[129]) / 80.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (s.b[1394] && s.b[1395]) {
            s.store_div_scaled_product_indices(12, 521, 129, 1.0, 167, 1.0);
            s.store_div_scaled_inputs(169, A::limited_exp(s.ad_value(12)), s.v[30], s.ad_value(522), 1.0);
        }

        if (s.b[1394] && (!s.b[1395])) {
            s.store_div_from_scalar(169, (5.540622384e34 * s.v[30]), 522);
        }

        if (!s.b[1394]) {
            s.store_scalar(169, 5.540622384e34);
        }

        s.store_offset_div(177, 167, 169, 1.0);

        s.store_mul(175, 175, 177);

        s.store_pow_ad(12, s.ad_value(159), A::div_from_scalar(1.0, s.ad_value(166)));

        s.store_mul(23, 453, 61);

        s.store_sqrt_square_offset(24, 23, 0.1);

        s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(23), A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);

        s.store_div_scaled_product_offset_denominator(14, s.ad_value(80), s.ad_value(13), (10.0 * p.p433), A::mul(s.ad_value(80), s.ad_value(13)), (10.0 * p.p433), 1.0);

        s.b[1396] = (s.v[536] < 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if s.b[1396] {
            s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);
        }

        if (!s.b[1396]) {
            s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);
        }

        s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_sqrt_square_offset(14, 13, 1.0);

        s.b[1397] = (s.v[13] != 0.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if s.b[1397] {
            s.store_add_scaled_product_mixed_iaa(162, 14, 0.5, A::div_from_scalar(1.0, s.ad_value(13)), A::asinh(s.ad_value(13)), 0.5);
        }

        if (!s.b[1397]) {
            s.store_scaled_add_ad_rhs(162, 14, A::div_from_scalar(1.0, s.ad_value(14)), 0.5);
        }

        s.copy_ad(163, 162);

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.b[1398] = (p.p42 == 1.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if s.b[1398] {
            s.store_scalar(244, 0.0);
            s.store_scalar(245, 1.0);
            s.store_mul_voltage_ad(71, s.ad_value(187), ctx, nodes, Some(8), Some(11));
            s.store_sub(53, 64, 71);
            s.store_sub(14, 53, 63);
            s.store_sqrt_square_offset(15, 14, 0.01);
            s.store_scaled_add(77, 14, 15, 0.5);
            s.store_offset_mul(17, 526, 77, 1.0);
            s.copy_ad(51, 71);
            s.store_add_scaled_product_value_ad(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 51, 1.0);
            s.store_scaled_add_ad_rhs(16, 18, A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01)), 0.5);
            s.store_mul_ad_rhs(241, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(529), 1.0, s.ad_value(531), s.ad_value(16), 1.0), s.ad_value(235), 1.0));
            s.store_mul_voltage_ad(67, s.ad_value(187), ctx, nodes, Some(6), Some(11));
            s.store_sub(55, 64, 67);
            s.store_sub(14, 55, 63);
            s.store_sqrt_square_offset(15, 14, 0.01);
            s.store_scaled_add(78, 14, 15, 0.5);
            s.store_offset_mul(17, 526, 78, 1.0);
            s.copy_ad(49, 67);
            s.store_add_scaled_product_value_ad(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 49, 1.0);
            s.store_scaled_add_ad_rhs(16, 18, A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01)), 0.5);
            s.store_mul_ad_rhs(242, 408, A::add_scaled_product(s.ad_value(240), 1.0, A::add_scaled_product(s.ad_value(528), 1.0, s.ad_value(530), s.ad_value(16), 1.0), s.ad_value(235), 1.0));
        }

        if (!s.b[1398]) {
            s.store_offset_mul(12, 526, 80, 1.0);
            s.store_mul_sub_rhs(13, 543, 111, 128);
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
            s.store_scaled_add_ad_rhs(15, 14, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(244, s.ad_value(408), A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), p.p2, 0.0, 235);
            s.copy_ad(242, 240);
            s.copy_ad(241, 239);
            s.store_offset_ad(245, A::mul3_scaled_output(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30]))), 1.0);
        }

        s.b[1399] = (p.p42 == 2.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if ((!s.b[1398]) && s.b[1399]) {
            s.store_mul_add_ad_rhs(244, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), s.ad_value(240));
            s.store_scalar(242, 0.0);
            s.store_scalar(241, 0.0);
            s.store_offset_ad(245, A::mul3_scaled_output(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30]))), 1.0);
        }

        s.store_add_ad_rhs(12, 150, A::div(s.ad_value(153), A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(104), s.ad_value(393), 2.0)));

        s.store_sub(216, 200, 144);

        s.store_mul3_lhs(13, 12, 216, 216);

        s.store_offset(14, 13, ((1.0) + ((-0.001))));

        s.store_offset_ad(15, A::add_scaled_inputs(s.ad_value(14), 0.5, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.004)), 0.5), (-1.0));

        s.store_scaled_offset_ad(154, A::sqrt(A::offset(s.ad_value(15), 1.0)), 1.0, 0.5);

        s.store_offset_ad(154, A::sub_scaled_inputs(A::offset(s.ad_value(154), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(154), (-1.0), A::offset(s.ad_value(154), (-1.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));

        s.store_add(12, 200, 144);

        s.store_sub(13, 200, 144);

        s.store_div_ad_rhs(14, 13, A::add(s.ad_value(12), s.ad_value(610)));

        s.store_mul3_lhs(15, 609, 14, 14);

        s.store_offset(611, 15, 1.0);

        s.store_div_ad_rhs(21, 633, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(636), A::mul3(s.ad_value(639), s.ad_value(13), s.ad_value(13)))), s.ad_value(12), 1.0, s.ad_value(104), s.ad_value(393), 2.0));

        s.store_limited_exp_neg_input(628, 21);

        s.store_mul3_lhs(160, 159, 162, 245);

        s.store_div(157, 499, 160);

        s.store_mul_ad_product_lhs(188, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(200), s.ad_value(144)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)))), s.ad_value(175), 1.0, s.ad_value(154), 1.0), s.ad_value(611), 628);

        s.store_scale(188, 188, p.p36);

        s.b[1400] = ((p.p42 == 1.0) && (p.p1094 == 1.0));
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if s.b[1400] {
            s.store_mul_ln_ad_rhs(753, 108, A::div_scaled_inputs(s.ad_value(481), p.p1117, A::powf(s.ad_value(28), 2.0), 1.0));
        }

        s.b[1401] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1401]) {
            s.store_mul_sqrt_ad_rhs(753, 108, A::offset(A::square(s.ad_value(753)), 1e-6));
        }

        if s.b[1400] {
            s.store_sub_from_scalar_ad(16, 1.0, A::scale(s.ad_value(50), p.p1113));
        }

        s.b[1402] = ((0.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.001)));
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1402]) {
            s.store_div_from_scalar_scaled_input(16, ((-0.001) * 0.001), 16, 16.0);
        }

        if (s.b[1400] && (!s.b[1402])) {
            s.store_scaled_add_ad_rhs(16, 16, A::sqrt(A::offset(A::mul(s.ad_value(16), s.ad_value(16)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1400] {
            s.store_offset(13, 200, (-p.p1102));
        }

        s.b[1403] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1403]) {
            s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);
        }

        if (s.b[1400] && (!s.b[1403])) {
            s.store_scaled_add_ad(13, A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(13), (-0.1), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1400] {
            s.store_div_scaled_value_offset_denominator(14, s.ad_value(13), (10.0 * p.p1103), s.ad_value(13), (10.0 * p.p1103), 1.0);
            s.store_mul_ad_rhs(754, 763, A::scale_offset(s.ad_value(14), p.p1101, 1.0));
            s.store_scale(23, 754, ((p.p2 * s.v[29]) * 1.60219e-19));
        }

        s.b[1404] = (p.p1110 != 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1404]) {
            s.store_abs_voltage(757, ctx, nodes, Some(6), Some(5));
        }

        s.b[1405] = (p.p1127 == 0.0);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1405]) {
            s.store_scalar(21, 1.0);
        }

        s.b[1406] = ((0.0 == 0.0) && ((s.v[757] - p.p1126) < ((-2500.0) * 0.5)));
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && s.b[1406]) {
            s.store_div_from_scalar_offset_scaled_input(22, ((-0.5) * 0.5), 757, 16.0, ((-p.p1126) * 16.0));
        }

        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && (!s.b[1406])) {
            s.store_scaled_add_ad(22, A::offset(s.ad_value(757), (-p.p1126)), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(757), (-p.p1126), A::offset(s.ad_value(757), (-p.p1126))), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1405])) {
            s.store_offset_scaled(21, 22, p.p1127, 1.0);
        }

        s.b[1408] = ((p.p1098 != 0.0) && (p.p514 > 0.0));
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1408]) {
            s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p514) as f64).ln())) / p.p515)));
            s.store_mul_ad_affine_product_rhs(750, 23, s.ad_value(21), A::scale_offset(A::powf(s.ad_value(760), p.p515), p.p514, 1.0), p.p1099, 0.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1408])) {
            s.store_scaled_mul(750, 23, 21, p.p1099);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_offset_div(14, 50, 753, 1.0);
        }

        s.b[1409] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1409]) {
            s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1409])) {
            s.store_scaled_add_ad_rhs(14, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_sub_scaled_ad_lhs(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 50, p.p1125);
        }

        s.b[1410] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1410]) {
            s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1410])) {
            s.store_scaled_add_ad_rhs(18, 18, A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_mul(750, 18, 750);
            s.store_mul3_affine_lhs(19, 762, 235, p.p1110, 0.0, 16);
            s.store_mul(755, 750, 19);
            s.store_div_ad(752, A::powf(s.ad_value(757), (4.0 - p.p1107)), A::add_scaled_inputs(A::powf(s.ad_value(757), (4.0 - p.p1107)), 1.0, A::powf(s.ad_value(755), (4.0 - p.p1107)), p.p1122));
            s.store_powf(17, 752, (1.0 / p.p1107));
            s.store_div_scaled_product_indices(20, 17, 757, 1.0, 755, 1.0);
        }

        s.b[1411] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1411]) {
            s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1411])) {
            s.store_scaled_add_ad_rhs(20, 20, A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_mul_powf_ad_rhs(759, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));
        }

        s.b[1412] = (p.p1112 != 0.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1412]) {
            s.store_abs_voltage(758, ctx, nodes, Some(7), Some(8));
        }

        s.b[1414] = ((p.p1098 != 0.0) && (p.p516 > 0.0));
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1414]) {
            s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p516) as f64).ln())) / p.p517)));
            s.store_mul_scaled_ad_rhs(751, 23, p.p1109, A::scale_offset(A::powf(s.ad_value(760), p.p517), p.p516, 1.0));
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1414])) {
            s.store_scale(751, 23, p.p1109);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_offset_div(14, 50, 753, 1.0);
        }

        s.b[1415] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1415]) {
            s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1415])) {
            s.store_scaled_add_ad_rhs(14, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_sub_scaled_ad_lhs(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 50, p.p1125);
        }

        s.b[1416] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1416]) {
            s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1416])) {
            s.store_scaled_add_ad_rhs(18, 18, A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_mul(751, 18, 751);
            s.store_mul3_affine_lhs(19, 762, 235, p.p1112, 0.0, 16);
            s.store_mul(756, 751, 19);
            s.store_div_ad(752, A::powf(s.ad_value(758), (4.0 - p.p1107)), A::add_scaled_inputs(A::powf(s.ad_value(758), (4.0 - p.p1107)), 1.0, A::powf(s.ad_value(756), (4.0 - p.p1107)), p.p1122));
            s.store_powf(17, 752, (1.0 / p.p1107));
            s.store_div_scaled_product_indices(20, 17, 758, 1.0, 756, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1417] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1417]) {
            s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1417])) {
            s.store_scaled_add_ad_rhs(20, 20, A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_mul_powf_ad_rhs(761, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));
        }

        s.b[1418] = ((p.p1110 != 0.0) && (p.p1112 != 0.0));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1418]) {
            s.store_div_scaled_product_denominator_ad(17, 57, 188, 1.0, A::min(s.ad_value(750), s.ad_value(751)), 1.0);
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-1.0), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1419] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1418]) && s.b[1419]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if ((s.b[1400] && s.b[1418]) && (!s.b[1419])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-(-1.0)), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if (s.b[1400] && s.b[1418]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul_ad_product_lhs(188, s.ad_value(57), A::min(s.ad_value(750), s.ad_value(751)), 17);
        }

        s.b[1420] = (p.p1110 != 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {
            s.store_div_scaled_product_indices(17, 57, 188, 1.0, 750, 1.0);
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-1.0), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1421] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && s.b[1421]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && (!s.b[1421])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-(-1.0)), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul3_lhs(188, 57, 750, 17);
        }

        s.b[1422] = (p.p1112 != 0.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {
            s.store_div_scaled_product_indices(17, 57, 188, 1.0, 751, 1.0);
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-1.0), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1423] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && s.b[1423]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && (!s.b[1423])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(17), (-(-1.0)), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul3_lhs(188, 57, 751, 17);
        }

        s.v[774] = 0.0;

        s.v[775] = 0.0;

        s.v[776] = 0.0;

        s.v[777] = 0.0;

        s.b[1424] = (((p.p42 == 1.0) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if s.b[1424] {
            s.store_offset_scaled(764, 232, -1.0, (-p.p1114));
            s.store_div(764, 764, 108);
            s.store_scaled_sqrt_scaled_input(765, 109, (((2.0 * 1.60219e-19) * s.v[26]) * p.p1117), 1.0 / (s.v[46]));
            s.store_ln_ad(766, A::max_with_scalar(A::div_from_scalar(p.p1117, s.ad_value(28)), 1e-38));
            s.store_scalar(13, 1.0);
            s.store_div(204, 764, 13);
            s.store_div(205, 765, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1425] = (s.v[204] < 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1425]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1424] && (!s.b[1425])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1424] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(767), (-1.0), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(765), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(767), 1.0, s.ad_value(766), (-2.0), A::div(s.ad_value(69), s.ad_value(108)), -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1426] = (s.v[20] <= (-68.0));
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1426]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1427] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1426]) && s.b[1427]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1428] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && s.b[1428]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && (!s.b[1428])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1424] && s.b[1426]) {
            s.store_mul_ad_rhs(768, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1424] && (!s.b[1426])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(768, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1429] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1429]) {
            s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);
        }

        if (s.b[1424] && (!s.b[1429])) {
            s.store_scaled_add_ad(769, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(767), (-1.0), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1424] {
            s.store_sqrt(770, 769);
            s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);
        }

        s.b[1430] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1430]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);
        }

        if (s.b[1424] && (!s.b[1430])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(771), (-1.0), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1424] {
            s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);
            s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);
            s.store_ad_value(775, A::mul3(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108), A::add_scaled_inputs_product(s.ad_value(764), 1.0, s.ad_value(773), (-1.0), s.ad_value(772), s.ad_value(768), (-2.0))));
        }

        s.b[1431] = (p.p1118 > 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1431]) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));
        }

        if (s.b[1424] && (!s.b[1431])) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if s.b[1424] {
            s.store_mul_ad_lhs(774, A::mul3_scaled_output(s.ad_value(772), s.ad_value(108), s.ad_value(12), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), 768);
        }

        s.b[1432] = (p.p1096 == 1.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1432]) {
            s.store_offset_ad(764, A::mul_scaled_lhs(s.ad_value(187), -1.0, A::voltage(ctx, nodes, Some(10), Some(7))), (-p.p1114));
            s.store_div(764, 764, 108);
            s.store_scalar(13, 1.0);
            s.store_div(204, 764, 13);
            s.store_div(205, 765, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1433] = (s.v[204] < 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1433]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1433])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(767), (-1.0), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(765), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(767), 1.0, s.ad_value(766), (-2.0), A::div_scaled_product(s.ad_value(187), A::voltage(ctx, nodes, Some(7), Some(11)), 1.0, s.ad_value(108), 1.0), -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1434] = (s.v[20] <= (-68.0));
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1435] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if (((s.b[1424] && s.b[1432]) && s.b[1434]) && s.b[1435]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1436] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && s.b[1436]) {
            s.store_limited_exp(15, 20);
        }

        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && (!s.b[1436])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {
            s.store_mul_ad_rhs(768, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1434])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        if ((s.b[1424] && s.b[1432]) && (!s.b[1434])) {
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(768, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1437] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1437]) {
            s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1437])) {
            s.store_scaled_add_ad(769, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(767), (-1.0), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_sqrt(770, 769);
            s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);
        }

        s.b[1438] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1438]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1438])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(771), (-1.0), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);
            s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);
            s.store_ad_value(777, A::mul3(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108), A::add_scaled_inputs_product(s.ad_value(764), 1.0, s.ad_value(773), (-1.0), s.ad_value(772), s.ad_value(768), (-2.0))));
        }

        s.b[1439] = (p.p1118 > 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1439]) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1439])) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_mul_ad_lhs(776, A::mul3_scaled_output(s.ad_value(772), s.ad_value(108), s.ad_value(12), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), 768);
        }

        s.v[254] = 0.0;

        s.b[1440] = (p.p7 > 1.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if s.b[1440] {
            s.store_scaled_mul(255, 157, 80, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));
            s.store_scale(21, 108, p.p755);
            s.store_scaled_mul(12, 21, 157, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));
            s.store_scaled_add(254, 12, 255, (p.p754 * p.p2));
        }

        s.b[1441] = (p.p7 == 2.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (s.b[1440] && s.b[1441]) {
            s.store_div_from_scalar(253, 1.0, 252);
        }

        s.b[1442] = (s.v[253] < p.p1093);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if ((s.b[1440] && s.b[1441]) && s.b[1442]) {
            s.store_scalar(253, p.p1093);
            s.store_div_from_scalar(252, 1.0, 253);
        }

        if (s.b[1440] && s.b[1441]) {
            s.store_add(23, 252, 254);
            s.store_div_scaled_product_indices(254, 252, 254, 1.0, 23, 1.0);
        }

        s.b[1443] = (p.p1094 == 0.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        s.b[1444] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        s.b[1445] = (s.v[167] > (s.v[558] / 80.0));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (!s.b[1444])) && s.b[1445]) {
            s.store_div_scaled_inputs(13, s.ad_value(558), -1.0, s.ad_value(167), 1.0);
        }

        s.b[1446] = (p.p1094 == 1.0);
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[1446]) {
            s.store_mul_offset_ad_lhs(184, A::mul(s.ad_value(555), s.ad_value(74)), 1.0, 140);
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(184)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(183, 74, 20);
            s.store_sub(185, 74, 183);
        }

        s.b[1447] = ((0.0 == 0.0) && (s.v[185] < ((-2500.0) * 0.001)));
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[1446]) && s.b[1447]) {
            s.store_div_from_scalar_scaled_input(185, ((-0.001) * 0.001), 185, 16.0);
        }

        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1447])) {
            s.store_scaled_add_ad_rhs(185, 185, A::sqrt(A::offset(A::mul(s.ad_value(185), s.ad_value(185)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if ((!s.b[1443]) && s.b[1446]) {
            s.store_mul_scaled_ad_rhs(181, 558, 0.5, A::offset(A::powf(s.ad_value(183), s.v[556]), 1.0));
            s.store_offset_scaled_ad(13, A::limited_exp_scaled_input(s.ad_value(76), p.p492), p.p493, 1.0);
            s.store_div(182, 553, 13);
            s.store_mul_ad_rhs(14, 182, A::add_scaled_product(A::scale_offset(s.ad_value(61), p.p505, 1.0), 1.0, s.ad_value(61), s.ad_value(61), p.p506));
        }

        s.b[1448] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 1e-12)));
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[1446]) && s.b[1448]) {
            s.store_div_from_scalar_scaled_input(182, ((-1e-12) * 1e-12), 14, 16.0);
        }

        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1448])) {
            s.store_scaled_add_ad_rhs(182, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 1e-12) * 1e-12))), 0.5);
        }

        s.b[1449] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        s.b[1450] = (s.v[185] > (s.v[181] / 80.0));
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[1446]) && (!s.b[1449])) && s.b[1450]) {
            s.store_div_scaled_inputs(13, s.ad_value(181), -1.0, A::powf(s.ad_value(185), p.p524), 1.0);
        }

        s.b[1451] = ((p.p1094 == 1.0) && (p.p1098 == 1.0));
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if s.b[1451] {
            s.store_offset(13, 200, (-p.p1105));
        }

        s.b[1452] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1452]) {
            s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);
        }

        if (s.b[1451] && (!s.b[1452])) {
            s.store_scaled_add_ad(13, A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(13), (-0.1), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1451] {
            s.store_div_scaled_value_offset_denominator(14, s.ad_value(13), (10.0 * p.p1106), s.ad_value(13), (10.0 * p.p1106), 1.0);
            s.store_mul_ad_rhs(754, 763, A::scale_offset(s.ad_value(14), p.p1104, 1.0));
            s.store_div_scaled_inputs(778, s.ad_value(188), p.p502, s.ad_value(754), ((p.p2 * s.v[29]) * 1.60219e-19));
            s.store_offset_scaled(779, 778, 1.0 / (p.p1099), (-1.0));
        }

        s.b[1453] = ((0.0 == 0.0) && (s.v[779] < ((-2500.0) * p.p504)));
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1453]) {
            s.store_div_from_scalar_scaled_input(779, ((-p.p504) * p.p504), 779, 16.0);
        }

        if (s.b[1451] && (!s.b[1453])) {
            s.store_scaled_add_ad_rhs(779, 779, A::sqrt(A::offset(A::mul(s.ad_value(779), s.ad_value(779)), ((0.25 * p.p504) * p.p504))), 0.5);
        }

        if s.b[1451] {
            s.store_scale(779, 779, p.p1099);
        }

        s.b[1454] = (p.p514 > 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        s.b[1455] = ((0.0 == 0.0) && (((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) - (p.p514 * ((s.v[760]) as f64).powf(p.p513))) < ((-2500.0) * 0.05)));
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if ((s.b[1451] && s.b[1454]) && s.b[1455]) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 16.0, A::powf(s.ad_value(760), p.p513), (p.p514 * 16.0)));
        }

        if ((s.b[1451] && s.b[1454]) && (!s.b[1455])) {
            let assign23370_ad_e32293: A = A::mul(A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 1.0, A::powf(s.ad_value(760), p.p513), p.p514), A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 1.0, A::powf(s.ad_value(760), p.p513), p.p514));
            s.store_add_scaled_inputs3_offset(14, A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), 0.5, A::powf(s.ad_value(760), p.p513), ((-p.p514) * 0.5), A::sqrt(A::offset(assign23370_ad_e32293, ((0.25 * 0.05) * 0.05))), 0.5, ((-p.p503) * 0.5));
        }

        s.b[1456] = ((0.0 == 0.0) && ((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) < ((-2500.0) * 0.05)));
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((s.b[1451] && (!s.b[1454])) && s.b[1456]) {
            s.store_div_from_scalar_scaled_ad(14, ((-0.05) * 0.05), A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 16.0);
        }

        if ((s.b[1451] && (!s.b[1454])) && (!s.b[1456])) {
            let assign23400_ad_e32396: A = A::add(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), A::sqrt(A::offset(A::mul_offset_lhs(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503), A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503))), ((0.25 * 0.05) * 0.05))));
            s.store_scale_ad(14, assign23400_ad_e32396, 0.5);
        }

        if s.b[1451] {
            s.store_scale(15, 779, ((2.0 * 1.60219e-19) / (p.p110 * 8.85418e-12)));
            s.store_powf_ad(15, A::mul(s.ad_value(15), s.ad_value(14)), 0.5);
            s.store_add_scaled_product_indices(16, 61, p.p507, 61, 61, p.p508);
            s.store_add_scaled_inputs_ad_rhs(17, 14, p.p509, A::powf(s.ad_value(14), p.p511), p.p510);
            s.store_scaled_add_ad_lhs(18, A::offset(s.ad_value(16), 1.0), 17, p.p500);
        }

        s.b[1458] = (s.v[15] > (p.p501 / 80.0));
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1458]) {
            s.store_div_from_scalar(13, (-p.p501), 15);
        }

        s.b[1459] = ((p.p46 != 0.0) || (p.p47 != 0.0));
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if s.b[1459] {
            s.store_mul_ad_rhs(277, 106, A::add_scaled_inputs4(s.ad_value(59), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), 1.0, s.ad_value(144), 1.0));
            s.store_sqrt_square_offset(13, 277, 0.0001);
            s.store_scaled_sub(279, 13, 277, 0.5);
            s.store_scaled_add(278, 277, 13, 0.5);
        }

        s.b[1460] = (p.p47 != 0.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        if (s.b[1459] && s.b[1460]) {
            s.store_div_scaled_value_by_product(13, s.ad_value(277), 1.0, s.ad_value(589), s.ad_value(108), 1.0);
            s.store_add_scaled_product_indices(14, 586, 1.0, 587, 279, (-1.0));
            s.store_offset_mul(15, 588, 279, 1.0);
            s.store_scaled_mul(16, 14, 15, ((-745669000000.0) * p.p77));
            s.store_limited_exp(17, 16);
            s.store_scalar(18, 4.97232e-7);
            s.store_div_scaled_inputs2_by_product(13, s.ad_value(277), 1.0, s.ad_value(584), (-1.0), s.ad_value(585), s.ad_value(108), 1.0);
            s.store_add_scaled_product_indices(14, 581, 1.0, 582, 278, (-1.0));
            s.store_offset_mul(15, 583, 278, 1.0);
            s.store_scaled_mul(16, 14, 15, ((-982222000000.0) * p.p77));
            s.store_limited_exp(17, 16);
            s.store_scalar(18, 3.75956e-7);
        }

        s.b[1461] = (p.p46 != 0.0);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if (s.b[1459] && s.b[1461]) {
            s.store_add_scaled_product_indices(13, 590, 1.0, 591, 278, (-1.0));
            s.store_offset_mul(14, 592, 278, 1.0);
            s.store_scaled_mul(15, 13, 14, s.v[295]);
            s.store_mul_ad(16, A::mul3(s.ad_value(90), s.ad_value(106), A::add(s.ad_value(200), s.ad_value(144))), A::limited_exp(s.ad_value(15)));
            s.store_offset_sqrt_ad(280, A::offset(A::square(s.ad_value(139)), 0.01), (-0.1));
            s.store_scale(13, 280, s.v[600]);
            s.store_limited_exp_neg_input(289, 13);
            s.store_offset_add(15, 13, 289, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(16, 1.0, A::mul_offset_lhs(s.ad_value(13), 1.0, s.ad_value(289)), 0.0001);
            s.store_offset_square(17, 13, 0.0002);
            s.store_sub(14, 52, 63);
            s.store_sqrt_square_offset(77, 14, 0.0001);
        }

        s.b[1463] = (p.p1041 == 1.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1461]) && s.b[1463]) {
            let assign24060_ad_e33151: A = {
                if (!((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)), A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(13, assign24060_ad_e33151);
        }

        s.b[1464] = (s.v[595] < 0.01);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1461]) && s.b[1463]) && s.b[1464]) {
            s.store_scalar(595, 0.01);
        }

        if ((s.b[1459] && s.b[1461]) && (!s.b[1463])) {
            s.store_add_scaled_product_indices(13, 593, 1.0, 594, 77, (-1.0));
        }

        if (s.b[1459] && s.b[1461]) {
            s.store_offset_mul(14, 595, 77, 1.0);
            s.store_mul3_lhs(15, 297, 13, 14);
            s.store_limited_exp(16, 15);
            s.store_sub(14, 54, 63);
            s.store_sqrt_square_offset(78, 14, 0.0001);
        }

        s.b[1465] = (p.p1041 == 1.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1461]) && s.b[1465]) {
            let assign24180_ad_e33317: A = {
                if (!((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)), A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(13, assign24180_ad_e33317);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1466] = (s.v[598] < 0.01);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1461]) && s.b[1465]) && s.b[1466]) {
            s.store_scalar(598, 0.01);
        }

        if ((s.b[1459] && s.b[1461]) && (!s.b[1465])) {
            s.store_add_scaled_product_indices(13, 596, 1.0, 597, 78, (-1.0));
        }

        if (s.b[1459] && s.b[1461]) {
            s.store_offset_mul(14, 598, 78, 1.0);
            s.store_mul3_lhs(15, 297, 13, 14);
            s.store_limited_exp(16, 15);
        }

        s.b[1467] = (p.p45 != 0.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if s.b[1467] {
            s.store_scalar(12, (s.v[47] * p.p77));
        }

        s.b[1468] = (((s.v[559] <= 0.0) || (s.v[417] <= 0.0)) || (s.v[561] < 0.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if (s.b[1467] && s.b[1468]) {
            s.store_scalar(18, 0.0);
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_div_scaled_inputs3(13, s.ad_value(54), -1.0, s.ad_value(562), (-1.0), s.ad_value(63), 1.0, s.ad_value(12), 1.0);
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_ad_value(13, {
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(13), 0.5, A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_div_scaled_value_offset_denominator(14, s.ad_value(417), 1.0, s.ad_value(13), 0.001, 1.0);
        }

        s.b[1469] = (s.v[561] != 0.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {
            s.store_mul_square_lhs(15, 48, 48);
            s.store_offset_add_ad(16, s.ad_value(561), A::abs(s.ad_value(15)), 0.0001);
        }

        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {
            let assign24440_ad_e33600: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24440_ad_e33600, (-1e-6));
        }

        if ((s.b[1467] && (!s.b[1468])) && (!s.b[1469])) {
            s.store_scalar(17, 1.0);
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_mul_ad_lhs(18, A::mul3_scaled_output(s.ad_value(559), s.ad_value(13), A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]), 17);
        }

        s.b[1470] = (((s.v[563] <= 0.0) || (s.v[418] <= 0.0)) || (s.v[565] < 0.0));
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if (s.b[1467] && s.b[1470]) {
            s.store_scalar(18, 0.0);
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_div_scaled_inputs3(13, s.ad_value(52), -1.0, s.ad_value(566), (-1.0), s.ad_value(63), 1.0, s.ad_value(12), 1.0);
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_ad_value(13, {
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(13), 0.5, A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_div_scaled_value_offset_denominator(14, s.ad_value(418), 1.0, s.ad_value(13), 0.001, 1.0);
        }

        s.b[1471] = (s.v[565] != 0.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {
            s.store_mul_square_lhs(15, 50, 50);
            s.store_offset_add_ad(16, s.ad_value(565), A::abs(s.ad_value(15)), 0.0001);
        }

        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {
            let assign24560_ad_e33803: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24560_ad_e33803, (-1e-6));
        }

        if ((s.b[1467] && (!s.b[1470])) && (!s.b[1471])) {
            s.store_scalar(17, 1.0);
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_mul_ad_lhs(18, A::mul3_scaled_output(s.ad_value(563), s.ad_value(13), A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]), 17);
        }

        s.store_div(12, 306, 343);

        s.store_offset_limited_exp(13, 12, (-1.0));

        s.store_add_scaled_product_right_ad(14, 346, 1.0, 345, A::sub(s.ad_value(306), s.ad_value(347)), 1.0);

        s.store_mul(15, 13, 14);

        s.store_div_scaled_offset_numerator(13, s.ad_value(306), 1.0, p.p731, s.ad_value(343), 1.0);

        s.store_limited_exp_neg_input(14, 13);

        s.store_mul_ad_rhs(16, 341, A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(351), 1.0, s.ad_value(14), (-p.p733), (-1.0)));

        s.store_add_scaled_product_right_ad(17, 349, 1.0, 348, A::sub(s.ad_value(306), s.ad_value(350)), 1.0);

        s.b[1472] = (s.v[341] > 0.0);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if s.b[1472] {
            s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(347), (-1.0), s.ad_value(343), 1.0)), 1.0 / (2.0)), 1.0, 16, A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(347), (-1.0), s.ad_value(343), 1.0)), 1.0, 1.0 / (2.0));
        }

        s.b[1473] = (s.v[441] > 0.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        s.b[1474] = ((p.p748 - s.v[306]) < (p.p748 * 0.001));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if (s.b[1473] && s.b[1474]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(447), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1473] && (!s.b[1474])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(447), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p748, A::sub_from_scalar(p.p748, s.ad_value(306)), 1.0), (-1.0));
        }

        s.b[1475] = (s.v[443] > 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = ((p.p750 - s.v[306]) < (p.p750 * 0.001));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (s.b[1475] && s.b[1476]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(449), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1475] && (!s.b[1476])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(449), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p750, A::sub_from_scalar(p.p750, s.ad_value(306)), 1.0), (-1.0));
        }

        s.b[1477] = (s.v[445] > 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = ((p.p752 - s.v[306]) < (p.p752 * 0.001));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (s.b[1477] && s.b[1478]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(451), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1477] && (!s.b[1478])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(451), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p752, A::sub_from_scalar(p.p752, s.ad_value(306)), 1.0), (-1.0));
        }

        s.store_div(12, 307, 344);

        s.store_offset_limited_exp(13, 12, (-1.0));

        s.store_add_scaled_product_right_ad(14, 353, 1.0, 352, A::sub(s.ad_value(307), s.ad_value(354)), 1.0);

        s.store_mul3_lhs(15, 302, 13, 14);

        s.store_div_scaled_offset_numerator(13, s.ad_value(307), 1.0, p.p732, s.ad_value(344), 1.0);

        s.store_limited_exp_neg_input(14, 13);

        s.store_mul_ad_product_rhs(16, 302, s.ad_value(342), A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(358), 1.0, s.ad_value(14), (-p.p734), (-1.0)));

        s.store_mul_ad_rhs(17, 302, A::add_scaled_product(s.ad_value(356), 1.0, s.ad_value(355), A::sub(s.ad_value(307), s.ad_value(357)), 1.0));

        s.b[1479] = (s.v[342] > 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        s.b[1480] = (s.v[302] > 0.0);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (s.b[1479] && s.b[1480]) {
            s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, 16, A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0));
        }

        s.b[1481] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (s.b[1479] && s.b[1481]) {
            s.store_div(12, 309, 344);
            s.store_offset_limited_exp(13, 12, (-1.0));
            s.store_add_scaled_product_right_ad(14, 353, 1.0, 352, A::sub(s.ad_value(309), s.ad_value(354)), 1.0);
            s.store_scaled_mul(15, 13, 14, p.p1128);
            s.store_div_scaled_offset_numerator(13, s.ad_value(309), 1.0, p.p732, s.ad_value(344), 1.0);
            s.store_limited_exp_neg_input(14, 13);
            s.store_mul_scaled_ad_rhs(16, 342, p.p1128, A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(358), 1.0, s.ad_value(14), (-p.p734), (-1.0)));
            s.store_add_scaled_product_right_ad(17, 356, p.p1128, 355, A::sub(s.ad_value(309), s.ad_value(357)), p.p1128);
            s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, 16, A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0));
        }

        s.b[1482] = (s.v[442] > 0.0);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        s.b[1483] = ((p.p749 - s.v[307]) < (p.p749 * 0.001));
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (s.b[1482] && s.b[1483]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(448), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1482] && (!s.b[1483])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(448), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(307)), 1.0), (-1.0));
        }

        s.b[1484] = (s.v[444] > 0.0);
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        s.b[1485] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        s.b[1486] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((s.b[1484] && s.b[1485]) && s.b[1486]) {
            s.store_mul_ad_product_lhs(14, s.ad_value(302), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), 444);
        }

        if ((s.b[1484] && s.b[1485]) && (!s.b[1486])) {
            s.store_mul3_lhs(14, 302, 301, 444);
        }

        if (s.b[1484] && (!s.b[1485])) {
            s.store_mul3_lhs(14, 302, 301, 444);
        }

        s.b[1487] = ((p.p751 - s.v[307]) < (p.p751 * 0.001));
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1487]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(450), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1484] && (!s.b[1487])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(450), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(307)), 1.0), (-1.0));
        }

        s.b[1488] = (s.v[446] > 0.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        s.b[1489] = ((p.p753 - s.v[307]) < (p.p753 * 0.001));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (s.b[1488] && s.b[1489]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(452), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if (s.b[1488] && (!s.b[1489])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(452), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p753, A::sub_from_scalar(p.p753, s.ad_value(307)), 1.0), (-1.0));
        }

        s.b[1490] = (p.p1128 > 0.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        s.b[1491] = (s.v[442] > 0.0);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        s.b[1492] = ((p.p749 - s.v[309]) < (p.p749 * 0.001));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1491]) && s.b[1492]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(448), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if ((s.b[1490] && s.b[1491]) && (!s.b[1492])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(448), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(309)), 1.0), (-1.0));
        }

        s.b[1493] = (s.v[444] > 0.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        s.b[1494] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1493]) && s.b[1494]) {
            s.store_mul_ad_lhs(14, A::scale_offset(s.ad_value(301), p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2)))), 444);
        }

        if ((s.b[1490] && s.b[1493]) && (!s.b[1494])) {
            s.store_scaled_mul(14, 301, 444, p.p1128);
        }

        s.b[1495] = ((p.p751 - s.v[309]) < (p.p751 * 0.001));
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1493]) && s.b[1495]) {
            s.store_div_scaled_value_by_product(12, s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(450), 1.0);
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
        }

        if ((s.b[1490] && s.b[1493]) && (!s.b[1495])) {
            s.store_div_scaled_value_by_product(12, s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(450), 1.0);
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(309)), 1.0), (-1.0));
        }

        s.store_mul(312, 423, 250);

        s.store_mul(315, 424, 300);

        s.store_scale(318, 428, (s.v[35] * p.p2));

        s.v[313] = ((0.1) as f64).powf((-p.p713));

        s.b[1496] = (p.p713 == 1.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if s.b[1496] {
            s.store_scalar(314, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1496]) {
            s.store_scalar(314, ((1.0 / (1.0 - p.p713)) * (1.0 - (((0.05 * p.p713) * (1.0 + p.p713)) * s.v[313]))));
        }

        s.v[316] = ((0.1) as f64).powf((-p.p715));

        s.b[1497] = (p.p715 == 1.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if s.b[1497] {
            s.store_scalar(317, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1497]) {
            s.store_scalar(317, ((1.0 / (1.0 - p.p715)) * (1.0 - (((0.05 * p.p715) * (1.0 + p.p715)) * s.v[316]))));
        }

        s.v[319] = ((0.1) as f64).powf((-p.p717));

        s.b[1498] = (p.p717 == 1.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if s.b[1498] {
            s.store_scalar(320, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1498]) {
            s.store_scalar(320, ((1.0 / (1.0 - p.p717)) * (1.0 - (((0.05 * p.p717) * (1.0 + p.p717)) * s.v[319]))));
        }

        s.b[1499] = (s.v[312] > 0.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if s.b[1499] {
            s.store_div(13, 306, 429);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1500] = (s.v[13] < 0.9);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if (s.b[1499] && s.b[1500]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1501] = (p.p713 != 1.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        s.b[1502] = (p.p713 == 0.5);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && s.b[1502]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && (!s.b[1502])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p713));
        }

        if ((s.b[1499] && s.b[1500]) && s.b[1501]) {
            s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p713)), 0.0);
        }

        if ((s.b[1499] && s.b[1500]) && (!s.b[1501])) {
            s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1499] && (!s.b[1500])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p713), (((((-1.0)) * ((5.0 * p.p713)))) + ((1.0 + p.p713)))), s.v[313]);
            s.store_mul_ad_product_rhs(331, 429, s.ad_value(312), A::add(s.ad_value(14), s.ad_value(314)));
        }

        if (!s.b[1499]) {
            s.store_scalar(331, 0.0);
        }

        s.b[1503] = (s.v[315] > 0.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if s.b[1503] {
            s.store_div(13, 306, 430);
        }

        s.b[1504] = (s.v[13] < 0.9);
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if (s.b[1503] && s.b[1504]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1505] = (p.p715 != 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = (p.p715 == 0.5);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && s.b[1506]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p715));
        }

        if ((s.b[1503] && s.b[1504]) && s.b[1505]) {
            s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p715)), 0.0);
        }

        if ((s.b[1503] && s.b[1504]) && (!s.b[1505])) {
            s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1503] && (!s.b[1504])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p715), (((((-1.0)) * ((5.0 * p.p715)))) + ((1.0 + p.p715)))), s.v[316]);
            s.store_mul_ad_product_rhs(332, 430, s.ad_value(315), A::add(s.ad_value(14), s.ad_value(317)));
        }

        if (!s.b[1503]) {
            s.store_scalar(332, 0.0);
        }

        s.b[1507] = (s.v[318] > 0.0);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if s.b[1507] {
            s.store_div(13, 306, 431);
        }

        s.b[1508] = (s.v[13] < 0.9);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (s.b[1507] && s.b[1508]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1509] = (p.p717 != 1.0);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        s.b[1510] = (p.p717 == 0.5);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && s.b[1510]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p717));
        }

        if ((s.b[1507] && s.b[1508]) && s.b[1509]) {
            s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p717)), 0.0);
        }

        if ((s.b[1507] && s.b[1508]) && (!s.b[1509])) {
            s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1507] && (!s.b[1508])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p717), (((((-1.0)) * ((5.0 * p.p717)))) + ((1.0 + p.p717)))), s.v[319]);
            s.store_mul_ad_product_rhs(333, 431, s.ad_value(318), A::add(s.ad_value(14), s.ad_value(320)));
        }

        if (!s.b[1507]) {
            s.store_scalar(333, 0.0);
        }

        s.store_add_scaled_inputs3(330, s.ad_value(331), 1.0, s.ad_value(332), 1.0, s.ad_value(333), 1.0);

        s.store_mul3_lhs(321, 302, 426, 251);

        s.b[1511] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        s.b[1512] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (s.b[1511] && s.b[1512]) {
            s.store_mul_ad_product_rhs(324, 302, s.ad_value(427), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))));
        }

        if (s.b[1511] && (!s.b[1512])) {
            s.store_mul3_lhs(324, 302, 427, 301);
        }

        if (!s.b[1511]) {
            s.store_mul3_lhs(324, 302, 427, 301);
        }

        s.store_scale(327, 425, (s.v[35] * p.p2));

        s.v[322] = ((0.1) as f64).powf((-p.p714));

        s.b[1513] = (p.p714 == 1.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if s.b[1513] {
            s.store_scalar(323, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1513]) {
            s.store_scalar(323, ((1.0 / (1.0 - p.p714)) * (1.0 - (((0.05 * p.p714) * (1.0 + p.p714)) * s.v[322]))));
        }

        s.v[325] = ((0.1) as f64).powf((-p.p716));

        s.b[1514] = (p.p716 == 1.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if s.b[1514] {
            s.store_scalar(326, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1514]) {
            s.store_scalar(326, ((1.0 / (1.0 - p.p716)) * (1.0 - (((0.05 * p.p716) * (1.0 + p.p716)) * s.v[325]))));
        }

        s.v[328] = ((0.1) as f64).powf((-p.p718));

        s.b[1515] = (p.p718 == 1.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if s.b[1515] {
            s.store_scalar(329, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1515]) {
            s.store_scalar(329, ((1.0 / (1.0 - p.p718)) * (1.0 - (((0.05 * p.p718) * (1.0 + p.p718)) * s.v[328]))));
        }

        s.b[1516] = (s.v[321] > 0.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if s.b[1516] {
            s.store_div(13, 308, 432);
        }

        s.b[1517] = (s.v[13] < 0.9);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1518] = (p.p714 != 1.0);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        s.b[1519] = (p.p714 == 0.5);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && s.b[1519]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && (!s.b[1519])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p714));
        }

        if ((s.b[1516] && s.b[1517]) && s.b[1518]) {
            s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);
        }

        if ((s.b[1516] && s.b[1517]) && (!s.b[1518])) {
            s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1516] && (!s.b[1517])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p714), (((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))), s.v[322]);
            s.store_mul_ad_product_rhs(335, 432, s.ad_value(321), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (!s.b[1516]) {
            s.store_scalar(335, 0.0);
        }

        s.b[1520] = (s.v[324] > 0.0);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if s.b[1520] {
            s.store_div(13, 308, 433);
        }

        s.b[1521] = (s.v[13] < 0.9);
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if (s.b[1520] && s.b[1521]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1522] = (p.p716 != 1.0);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        s.b[1523] = (p.p716 == 0.5);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && s.b[1523]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && (!s.b[1523])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p716));
        }

        if ((s.b[1520] && s.b[1521]) && s.b[1522]) {
            s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);
        }

        if ((s.b[1520] && s.b[1521]) && (!s.b[1522])) {
            s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1520] && (!s.b[1521])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p716), (((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))), s.v[325]);
            s.store_mul_ad_product_rhs(336, 433, s.ad_value(324), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (!s.b[1520]) {
            s.store_scalar(336, 0.0);
        }

        s.b[1524] = (s.v[327] > 0.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if s.b[1524] {
            s.store_div(13, 308, 434);
        }

        s.b[1525] = (s.v[13] < 0.9);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if (s.b[1524] && s.b[1525]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1526] = (p.p718 != 1.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        s.b[1527] = (p.p718 == 0.5);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && s.b[1527]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && (!s.b[1527])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p718));
        }

        if ((s.b[1524] && s.b[1525]) && s.b[1526]) {
            s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p718)), 0.0);
        }

        if ((s.b[1524] && s.b[1525]) && (!s.b[1526])) {
            s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1524] && (!s.b[1525])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p718), (((((-1.0)) * ((5.0 * p.p718)))) + ((1.0 + p.p718)))), s.v[328]);
            s.store_mul_ad_product_rhs(337, 434, s.ad_value(327), A::add(s.ad_value(14), s.ad_value(329)));
        }

        if (!s.b[1524]) {
            s.store_scalar(337, 0.0);
        }

        s.store_add_scaled_inputs3(334, s.ad_value(335), 1.0, s.ad_value(336), 1.0, s.ad_value(337), 1.0);

        s.b[1528] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if s.b[1528] {
            s.store_scaled_mul(321, 426, 251, p.p1128);
        }

        s.b[1529] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1529]) {
            s.store_mul_ad_rhs(324, 427, A::scale_offset(s.ad_value(301), p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2)))));
        }

        if (s.b[1528] && (!s.b[1529])) {
            s.store_scaled_mul(324, 427, 301, p.p1128);
        }

        s.b[1530] = (s.v[321] > 0.0);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1530]) {
            s.store_div(13, 309, 432);
        }

        s.b[1531] = (s.v[13] < 0.9);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if ((s.b[1528] && s.b[1530]) && s.b[1531]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1532] = (p.p714 != 1.0);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        s.b[1533] = (p.p714 == 0.5);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && s.b[1533]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && (!s.b[1533])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p714));
        }

        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) {
            s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);
        }

        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && (!s.b[1532])) {
            s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if ((s.b[1528] && s.b[1530]) && (!s.b[1531])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p714), (((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))), s.v[322]);
            s.store_mul_ad_product_rhs(339, 432, s.ad_value(321), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (s.b[1528] && (!s.b[1530])) {
            s.store_scalar(339, 0.0);
        }

        s.b[1534] = (s.v[324] > 0.0);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1534]) {
            s.store_div(13, 309, 433);
        }

        s.b[1535] = (s.v[13] < 0.9);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if ((s.b[1528] && s.b[1534]) && s.b[1535]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1536] = (p.p716 != 1.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        s.b[1537] = (p.p716 == 0.5);
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && s.b[1537]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && (!s.b[1537])) {
            s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p716));
        }

        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) {
            s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);
        }

        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && (!s.b[1536])) {
            s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if ((s.b[1528] && s.b[1534]) && (!s.b[1535])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p716), (((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))), s.v[325]);
            s.store_mul_ad_product_rhs(340, 433, s.ad_value(324), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (s.b[1528] && (!s.b[1534])) {
            s.store_scalar(340, 0.0);
        }

        if s.b[1528] {
            s.store_add(338, 339, 340);
        }

        if (!s.b[1528]) {
            s.store_scalar(338, 0.0);
        }

        s.b[1538] = (p.p38 != 0.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if s.b[1538] {
            s.store_powf_ad(13, A::scale(s.ad_value(481), 1.0000000000000001e-23), p.p954);
            s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);
            s.store_div_scaled_product_right_ad(15, 187, A::voltage(ctx, nodes, Some(11), Some(7)), p.p953, 108, 1.0);
        }

        s.store_div_scaled_inputs(360, s.ad_value(502), 2.0, s.ad_value(157), 1.0);

        s.b[1539] = (p.p784 <= 0.0);
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if s.b[1539] {
            s.store_scalar(363, 0.0);
        }

        if (!s.b[1539]) {
            s.store_div_scaled_offset_numerator(12, A::div(s.ad_value(167), s.ad_value(129)), 1.0, p.p784, s.ad_value(360), 1.0);
            s.store_mul_ln_ad_rhs(363, 129, A::max_with_scalar(s.ad_value(12), 1e-38));
        }

        s.b[1540] = (s.v[363] < 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((!s.b[1539]) && s.b[1540]) {
            s.store_scalar(363, 0.0);
        }

        s.store_mul_scaled_ad_rhs(367, 108, 1.0 / (1.60219e-19), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(483)));

        s.store_mul_ad_affine_product_lhs(366, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(144), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(188)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);

        s.store_mul3_affine_lhs(737, 108, 188, 1.60219e-19, 0.0, 188);

        s.store_add_scaled_product_value_ad(738, A::scale_offset(s.ad_value(366), p.p799, p.p785), 1.0, 366, 366, p.p800);

        s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));

        s.store_scale(740, 108, (p.p785 * 1.60219e-19));

        s.b[1541] = (p.p1065 == 1.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if s.b[1541] {
            s.store_scalar(745, s.v[30]);
            s.store_div_scaled_inputs2(712, s.ad_value(64), 1.0, s.ad_value(482), (-1.0), s.ad_value(108), 1.0);
            s.store_scaled_sqrt_ad(713, A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108)), 1.0 / (s.v[46]));
            s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));
            s.store_scalar(13, 1.0);
            s.store_div(204, 712, 13);
            s.store_div(205, 713, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1542] = (s.v[204] < 0.0);
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1542]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1541] && (!s.b[1542])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(715, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1541] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(715), (-1.0), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(713), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(713), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(715), 1.0, s.ad_value(714), (-2.0), s.ad_value(73), -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1543] = (s.v[20] <= (-68.0));
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1543]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1544] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((s.b[1541] && s.b[1543]) && s.b[1544]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1545] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && s.b[1545]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && (!s.b[1545])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1541] && s.b[1543]) {
            s.store_mul_ad_rhs(717, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1541] && (!s.b[1543])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(717, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1546] = ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0)));
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1546]) {
            s.store_div_from_scalar_scaled_input(716, ((-2.0) * 2.0), 715, 16.0);
        }

        if (s.b[1541] && (!s.b[1546])) {
            s.store_scaled_add_ad(716, A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(715), (-1.0), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1541] {
            s.store_offset_ad(718, A::div_scaled_inputs(s.ad_value(713), 1.0, A::sqrt(s.ad_value(716)), 2.0), 1.0);
            s.copy_ad(719, 157);
            s.store_scale(726, 719, (s.v[46] * s.v[29]));
            s.store_scale(725, 157, (s.v[46] * s.v[29]));
            s.store_div_scaled_product_by_product(720, s.ad_value(188), s.ad_value(746), 1.0, A::mul3_scaled_output(s.ad_value(718), s.ad_value(726), s.ad_value(108), 2.0), s.ad_value(108), 1.0);
            s.store_div_scaled_product_by_product(722, s.ad_value(188), A::sub(s.ad_value(745), s.ad_value(746)), 1.0, A::mul3_scaled_output(s.ad_value(90), s.ad_value(725), s.ad_value(106), 2.0), s.ad_value(106), 1.0);
            s.store_add_scaled_inputs3_offset(12, A::square(s.ad_value(717)), 4.0, s.ad_value(717), 4.0, s.ad_value(720), (-4.0), 1.0);
            s.store_offset_scaled_ad(723, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(144)), 4.0, s.ad_value(144), 4.0, s.ad_value(722), 4.0), 1.0)), 0.5, (-0.5));
        }

        s.b[1548] = (s.v[30] != s.v[746]);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1548]) {
            s.store_mul3_affine_lhs(724, 90, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 723);
            s.store_add_scaled_inputs3(361, s.ad_value(745), 1.0, s.ad_value(359), (-2.0), s.ad_value(746), -1.0);
            s.store_square(362, 361);
            s.store_scale(13, 362, (10000000000.0 * s.v[46]));
            s.store_scaled_ln_ad(14, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(724), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38), p.p785);
            s.store_scaled_sub(15, 724, 366, p.p799);
            s.store_scaled_sub_ad(16, A::square(s.ad_value(724)), A::square(s.ad_value(366)), (0.5 * p.p800));
            s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));
            s.store_add_scaled_product(732, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(17), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(13)), A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0), 1.0);
            s.store_mul3_affine_lhs(18, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(733, A::div(s.ad_value(740), s.ad_value(18)), s.ad_value(188), 188);
            s.store_add(19, 733, 732);
        }

        if s.b[1541] {
            s.store_scale(20, 108, (p.p1067 * 1.60219e-19));
            s.store_mul3_affine_lhs(21, 746, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(741, A::div(s.ad_value(20), s.ad_value(21)), s.ad_value(188), 188);
            s.copy_ad(22, 741);
        }

        s.b[1551] = (p.p801 >= (s.v[30] / 2.0));
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if ((!s.b[1541]) && s.b[1551]) {
            s.store_scalar(359, 0.0);
        }

        if ((!s.b[1541]) && (!s.b[1551])) {
            s.store_scalar(359, p.p801);
        }

        s.b[1552] = (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        s.b[1553] = ((p.p786 != 0.0) && (p.p785 > 0.0));
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if (((!s.b[1541]) && s.b[1552]) && s.b[1553]) {
            s.store_div(13, 80, 641);
            s.store_offset_pow_ad(14, s.ad_value(13), s.ad_value(642), 1.0);
            s.store_div(15, 640, 14);
            s.store_scale(16, 15, 1.0 / (p.p785));
            s.store_scaled_add_ad(17, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * p.p798) * p.p798))), 0.5);
            s.store_scale(364, 17, p.p785);
        }

        if (((!s.b[1541]) && s.b[1552]) && (!s.b[1553])) {
            s.store_scalar(364, p.p785);
        }

        if ((!s.b[1541]) && s.b[1552]) {
            s.store_sub_from_scalar_ad(361, s.v[30], A::scale(s.ad_value(359), 2.0));
            s.store_square(362, 361);
            s.store_scale(12, 362, (10000000000.0 * s.v[46]));
            s.store_mul_ad_affine_product_lhs(365, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(200), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);
            s.store_mul_ln_ad_rhs(13, 364, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));
            s.store_scaled_sub(14, 365, 366, p.p799);
            s.store_scaled_sub_ad(15, A::square(s.ad_value(365)), A::square(s.ad_value(366)), (0.5 * p.p800));
            s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));
            s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);
            s.store_mul3_affine_lhs(17, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_scaled_mul(740, 364, 108, 1.60219e-19);
            s.store_mul_ad_product_lhs(369, A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(188), 188);
            s.store_add(18, 369, 368);
        }

        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));

        s.store_square(13, 12);

        s.store_offset_scaled(15, 13, (((p.p814 * s.v[30])) * (p.p811)), p.p811);

        s.store_offset_scaled(16, 13, (((p.p815 * s.v[30])) * (p.p812)), p.p812);

        s.store_offset_scaled(17, 13, (((p.p1044 * s.v[30])) * (p.p1043)), p.p1043);

        s.store_square(389, 17);

        s.store_square(388, 16);

        s.b[1555] = (p.p48 == 0.0);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        s.b[1556] = (p.p48 == 1.0);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if s.b[1555] {
            s.store_scaled_mul(196, 108, 190, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));
            s.store_scaled_mul(197, 108, 193, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));
            s.store_mul_abs_ad_rhs(12, 157, A::add(s.ad_value(196), s.ad_value(197)));
            s.store_offset_mul(13, 12, 244, (s.v[30] * s.v[30]));
        }

        if (s.b[1556] && (!s.b[1555])) {
            s.store_scaled_mul(382, 90, 106, 2.0);
            s.store_mul_scale_ad_lhs(12, A::mul3(s.ad_value(157), s.ad_value(163), s.ad_value(175)), s.v[46], 382);
            s.store_scaled_add(13, 200, 144, 0.5);
            s.store_offset(15, 13, 0.5);
            s.store_square(16, 15);
            s.store_mul(17, 16, 15);
            s.store_sub(18, 200, 144);
            s.store_square(19, 18);
            s.store_mul(20, 19, 18);
            s.store_mul_ad_lhs(21, A::scale_offset(s.ad_value(13), 6.0, 0.5), 19);
            s.store_scale(381, 163, s.v[30]);
            s.store_scale(22, 381, 1.0 / (s.v[30]));
            s.store_offset_ad(24, A::div_scaled_product_by_product(s.ad_value(389), s.ad_value(139), 1.0, s.ad_value(140), A::offset(s.ad_value(80), p.p1045), 1.0), 1.0);
            s.store_offset_scaled(24, 24, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));
        }

        s.b[1557] = ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1)));
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((s.b[1556] && (!s.b[1555])) && s.b[1557]) {
            s.store_div_from_scalar_scaled_input(24, ((-0.1) * 0.1), 24, 16.0);
        }

        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1557])) {
            s.store_scaled_add_ad_rhs(24, 24, A::sqrt(A::offset(A::mul(s.ad_value(24), s.ad_value(24)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if (s.b[1556] && (!s.b[1555])) {
            s.store_div_scaled_product3_mixed_aaii(378, A::mul3(s.ad_value(381), s.ad_value(22), s.ad_value(22)), A::add_scaled_inputs3(A::div(s.ad_value(13), s.ad_value(16)), 1.0, A::div(s.ad_value(21), A::mul_scaled_lhs(s.ad_value(16), 60.0, s.ad_value(16))), (-1.0), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(19), 1.0, s.ad_value(16), s.ad_value(17), 144.0), 1.0), 388, (15.0 * 1.0 / (4.0)), 12, ((p.p2 * s.v[29]) * 12.0));
        }

        s.copy_ad(60, 59);

        s.v[218] = 0.0;

        s.b[1562] = (p.p40 == 1.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if s.b[1562] {
            s.store_offset(549, 549, p.p35);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1562] {
            s.store_mul(65, 64, 109);
            s.store_mul(73, 72, 109);
            s.store_mul(58, 549, 109);
            s.store_sub(60, 65, 58);
            s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));
            s.store_scaled_sqrt_ad(234, A::mul_scaled_lhs(s.ad_value(550), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(109)), 1.0 / (s.v[46]));
            s.store_div_from_scalar(126, 1.0, 234);
            s.store_div_scaled_inputs(206, s.ad_value(479), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(108), (s.v[46] * s.v[46]));
        }

        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div_from_scalar(218, 1.0, 206);
            } else {
                s.store_scalar(218, 0.0);
            }
        }

        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div(203, 550, 479);
            } else {
                s.store_scalar(203, 0.0);
            }
        }

        if s.b[1562] {
            s.store_offset(13, 203, 1.0);
            s.store_div(204, 60, 13);
            s.store_div(205, 234, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1563] = (s.v[204] < 0.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1563]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1562] && (!s.b[1563])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1562] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(234), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(73), -1.0);
            s.store_sub_scaled_ad_rhs(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1564] = (s.v[20] <= (-68.0));
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1564]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1565] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if ((s.b[1562] && s.b[1564]) && s.b[1565]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1566] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && s.b[1566]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && (!s.b[1566])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1562] && s.b[1564]) {
            s.store_mul_ad_rhs(200, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0));
        }

        if (s.b[1562] && (!s.b[1564])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1567] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1567]) {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (s.b[1562] && (!s.b[1567])) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_sqrt(96, 93);
            s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);
        }

        s.b[1568] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1568]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1562] && (!s.b[1568])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_offset_div_ad(90, s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);
            s.store_mul_ad_rhs(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1569] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1569]) {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

        if (s.b[1562] && (!s.b[1569])) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_mul3_affine_lhs(130, 90, 108, 2.0, 0.0, 200);
            s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));
            s.store_mul_ad(15, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(62), 1.0), A::pow(s.ad_value(132), s.ad_value(407)));
            s.store_offset(16, 15, 1.0);
        }

        s.b[1570] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1570]) {
            s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (s.b[1562] && (!s.b[1570])) {
            s.store_scaled_add_ad(133, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        if s.b[1562] {
            s.store_div_scaled_product_by_product(137, s.ad_value(499), s.ad_value(108), 1.0, s.ad_value(133), s.ad_value(411), s.v[34]);
            s.store_div_scaled_product_offset_denominator(131, s.ad_value(137), A::add(A::square(s.ad_value(200)), s.ad_value(200)), 1.0, A::mul_offset_rhs(s.ad_value(137), s.ad_value(200), 1.0), 1.0, 1.0);
            s.store_add_scaled_inputs4(145, s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(131), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_mul(146, 145, 108);
        }

        s.b[1571] = ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001)));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1571]) {
            s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(146), 16.0, s.ad_value(72), 16.0));
        }

        if (s.b[1562] && (!s.b[1571])) {
            s.store_add_scaled_inputs3(141, s.ad_value(146), 0.5, s.ad_value(72), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(146), s.ad_value(72)), A::sub(s.ad_value(146), s.ad_value(72))), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        s.b[1572] = ((p.p1134 == 0.0) && (p.p1135 == 0.0));
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1572]) {
            s.store_scalar(783, p.p1129);
        }

        if (s.b[1562] && (!s.b[1572])) {
            s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);
            s.store_offset_ad(783, A::div_scaled_inputs2(s.ad_value(13), p.p1134, A::mul3_scaled_output(s.ad_value(13), s.ad_value(200), s.ad_value(106), p.p1135), (-1.0), A::scale_offset(s.ad_value(61), p.p1136, 1.0), 1.0), 1.0);
        }

        s.b[1573] = ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005)));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if ((s.b[1562] && (!s.b[1572])) && s.b[1573]) {
            s.store_div_from_scalar_scaled_input(783, ((-0.0005) * 0.0005), 783, 16.0);
        }

        if ((s.b[1562] && (!s.b[1572])) && (!s.b[1573])) {
            s.store_scaled_add_ad(783, A::offset(s.ad_value(783), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(783), (-0.1), A::offset(s.ad_value(783), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if s.b[1562] {
            s.store_div(141, 141, 783);
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(139, 75, 20);
            s.store_mul_add_lhs(142, 139, 72, 109);
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(234), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(142), -1.0);
            s.store_sub_scaled_ad_rhs(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1574] = (s.v[20] <= (-68.0));
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1574]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1575] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if ((s.b[1562] && s.b[1574]) && s.b[1575]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1576] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && s.b[1576]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && (!s.b[1576])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1562] && s.b[1574]) {
            s.store_mul_ad_rhs(144, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0));
        }

        if (s.b[1562] && (!s.b[1574])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        if s.b[1562] {
            s.store_add_scaled_inputs3_offset(92, s.ad_value(91), 1.0, s.ad_value(200), (-1.0), s.ad_value(144), -1.0, (-1.0));
        }

        s.b[1577] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1577]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1562] && (!s.b[1577])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_sqrt(14, 12);
            s.store_add_ad(15, A::offset(s.ad_value(203), 1.0), A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));
            s.store_offset_ad(16, A::mul3(s.ad_value(203), s.ad_value(14), s.ad_value(126)), 0.5);
            s.store_sqrt_add_ad(17, A::square(s.ad_value(16)), A::mul3(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(218)));
            s.store_div_ad_rhs(90, 15, A::add(s.ad_value(16), s.ad_value(17)));
            s.store_mul_ad_rhs(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1578] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1578]) {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1562] && (!s.b[1578])) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_mul_ad_rhs(13, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(144), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1579] = ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1)));
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1579]) {
            s.store_div_from_scalar_scaled_input(85, ((-0.1) * 0.1), 13, 16.0);
        }

        if (s.b[1562] && (!s.b[1579])) {
            s.store_scaled_add_ad_rhs(85, 13, A::sqrt(A::offset(A::mul(s.ad_value(13), s.ad_value(13)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_scaled_add(86, 84, 85, 0.5);
            s.store_mul_ad_product_rhs(80, 90, s.ad_value(108), A::add(s.ad_value(200), s.ad_value(144)));
            s.store_add_scaled_inputs(156, 86, s.v[155], 80, (s.v[158] * s.v[155]));
            s.store_offset(13, 203, 1.0);
            s.store_div_scaled_inputs2(204, s.ad_value(60), 1.0, s.ad_value(109), p.p136, s.ad_value(13), 1.0);
            s.store_div(205, 234, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1580] = (s.v[204] < 0.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1580]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1562] && (!s.b[1580])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1562] {
            s.store_mul_ad(15, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(62), 1.0), A::pow(s.ad_value(156), s.ad_value(407)));
            s.store_offset(16, 15, 1.0);
        }

        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1581]) {
            s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (s.b[1562] && (!s.b[1581])) {
            s.store_scaled_add_ad(159, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        if s.b[1562] {
            s.store_div_scaled_product_by_product(138, s.ad_value(499), s.ad_value(108), 2.0, s.ad_value(159), s.ad_value(411), s.v[34]);
            s.store_sub(87, 200, 144);
            s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);
            s.store_sqrt_offset_input(161, 13, 1.0);
            s.store_scaled_offset(162, 161, 1.0, 0.5);
            s.store_div_scaled_inputs(134, s.ad_value(411), 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);
            s.store_scale(135, 134, s.v[34]);
            s.store_add(170, 141, 135);
            s.store_sub(167, 75, 139);
        }

        s.b[1582] = (s.v[542] != 0.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if s.b[1582] {
            s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[1582]) {
            s.store_scalar(176, 1.0);
        }

        s.store_square(207, 176);

        s.store_div_from_scalar(208, 1.0, 176);

        s.store_div_from_scalar(209, 1.0, 207);

        s.store_offset(210, 176, (-1.0));

        s.store_sub(213, 60, 91);

        s.store_sub(216, 200, 144);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);

        s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);

        s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if s.b[1583] {
            s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);
        }

        if (!s.b[1583]) {
            s.store_scaled_add_ad_rhs(13, 211, A::sqrt(A::offset(A::mul(s.ad_value(211), s.ad_value(211)), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if s.b[1584] {
            s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);
        }

        if (!s.b[1584]) {
            s.store_scaled_add_ad_rhs(14, 212, A::sqrt(A::offset(A::mul(s.ad_value(212), s.ad_value(212)), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);

        s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);

        s.store_div_ad_rhs(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));

        s.store_div_ad_rhs(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));

        s.store_add(15, 214, 215);

        s.store_scaled_div_ad_rhs(16, 217, A::mul(A::square(s.ad_value(15)), s.ad_value(15)), 0.3333333333333333);

        s.store_div_scaled_product3_mixed_iiia(17, 783, 162, 208, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0);

        s.store_mul_scale_ad_lhs(18, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 17);

        s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);

        s.store_scaled_mul(20, 217, 17, 0.3333333333333333);

        s.store_div_scaled_product_mixed_iaa(202, 212, A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0);

        s.store_add_ad_lhs(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);

        s.store_add_scaled_products_left_right_ad(189, 208, A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, 210, 201, 1.0);

        s.store_add(21, 200, 144);

        s.store_mul3_lhs(22, 217, 17, 17);

        s.store_add_ad(194, A::mul3(s.ad_value(90), s.ad_value(208), A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(217), s.ad_value(17), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(90), s.ad_value(210), s.ad_value(144), 2.0));

        s.store_mul_ad_product_rhs(191, 90, s.ad_value(209), A::add_scaled_product(s.ad_value(21), 0.5, s.ad_value(216), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), 1.0, s.ad_value(22), 0.2), (-1.0 / (6.0))));

        s.store_mul_ad_product_lhs(192, s.ad_value(90), A::sub(s.ad_value(176), s.ad_value(208)), 144);

        s.store_add(193, 191, 192);

        s.store_sub(190, 194, 193);

        s.b[1585] = ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694)));
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if s.b[1585] {
            s.store_div_from_scalar_ad(83, ((-p.p694) * p.p694), A::mul_scaled_output(s.ad_value(108), s.ad_value(189), 16.0));
        }

        if (!s.b[1585]) {
            s.store_add_scaled_product_value_ad(83, A::sqrt(A::offset(A::mul3(s.ad_value(108), s.ad_value(189), A::mul(s.ad_value(108), s.ad_value(189))), ((0.25 * p.p694) * p.p694))), 0.5, 108, 189, 0.5);
        }

        s.store_mul_add_rhs(82, 108, 190, 193);

        s.store_add_scaled_inputs(12, 82, 1.0 / (p.p207), 83, (p.p208 * 1.0 / (p.p207)));

        s.store_offset_powf_ad(13, s.ad_value(12), (0.7 * p.p206), 1.0);

        s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);

        s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(227), 1.0 / (s.v[47])));

        s.store_mul_ad_affine_product_lhs(195, A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), s.ad_value(108), (((-p.p2) * s.v[33]) * s.v[34]), 0.0, 189);

        s.store_scaled_mul(199, 228, 108, ((p.p2 * s.v[33]) * s.v[34]));

        s.store_mul_neg_lhs(196, 199, 190);

        s.store_mul_neg_lhs(197, 199, 193);

        s.store_neg_ad(198, A::add_scaled_inputs3(s.ad_value(195), 1.0, s.ad_value(196), 1.0, s.ad_value(197), 1.0));

        s.b[1586] = (!param_given[666]);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if s.b[1586] {
            s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));
        }

        s.store_offset(225, 544, p.p671);

        s.store_offset(226, 544, p.p672);

        s.b[1587] = (p.p41 == 0.0);
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if s.b[1587] {
            s.store_scaled_mul(223, 225, 231, ((-s.v[33]) * p.p2));
            s.store_scaled_mul(224, 226, 232, ((-s.v[33]) * p.p2));
        }

        if (!s.b[1587]) {
            s.store_sqrt_offset_ad(12, A::mul_offset_lhs(A::sub(s.ad_value(231), s.ad_value(63)), 0.02, A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(219, s.ad_value(231), 0.5, s.ad_value(63), ((-1.0) * 0.5), s.ad_value(12), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(18, 219, A::powf(A::offset(A::powf(A::scale(s.ad_value(219), (-1.0 / (p.p692))), p.p693), 1.0), (1.0 / p.p693)));
            s.store_sqrt_sub_from_scalar_ad(13, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(547), 1.0));
            s.store_add_scaled_products_right_right_ad(223, 225, 231, ((-s.v[33]) * p.p2), 545, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(63), (-1.0), s.ad_value(219), -1.0), 1.0, s.ad_value(547), s.ad_value(13), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));
            s.store_sqrt_offset_ad(12, A::mul_offset_lhs(A::sub(s.ad_value(232), s.ad_value(63)), 0.02, A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(220, s.ad_value(232), 0.5, s.ad_value(63), ((-1.0) * 0.5), s.ad_value(12), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(18, 220, A::powf(A::offset(A::powf(A::scale(s.ad_value(220), (-1.0 / (p.p690))), p.p691), 1.0), (1.0 / p.p691)));
            s.store_sqrt_sub_from_scalar_ad(14, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(548), 1.0));
            s.store_add_scaled_products_right_right_ad(224, 226, 232, ((-s.v[33]) * p.p2), 546, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(63), (-1.0), s.ad_value(220), -1.0), 1.0, s.ad_value(548), s.ad_value(14), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));
        }

        s.store_ad_value(221, A::mul_scaled_lhs(s.ad_value(187), (((-p.p2) * s.v[34]) * p.p673), A::voltage(ctx, nodes, Some(10), Some(11))));

        s.b[1588] = (p.p37 == 1.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if s.b[1588] {
            s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));
            s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);
            s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19));
        }

        if s.b[1588] {
            let assign31550_ad_e41781: A = {
                if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(674, 612, assign31550_ad_e41781);
        }

        if s.b[1588] {
            s.store_mul_offset_ad_rhs(673, 616, A::mul_offset_rhs(s.ad_value(623), s.ad_value(395), (-1.0)), 1.0);
        }

        s.b[1589] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1589]) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));
        }

        if (s.b[1588] && (!s.b[1589])) {
            s.store_add_scaled_inputs3_offset(110, s.ad_value(127), 0.5, s.ad_value(61), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
        }

        if s.b[1588] {
            s.store_sqrt(111, 110);
            s.store_mul(112, 114, 111);
            s.store_div_from_scalar(97, s.v[26], 112);
            s.store_ad_value(113, A::add_scaled_inputs_products(s.ad_value(613), 1.0, s.ad_value(674), 1.0, s.ad_value(614), s.ad_value(76), 1.0, s.ad_value(615), s.ad_value(61), (-1.0)));
            s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);
        }

        s.b[1590] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1590]) {
            s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);
        }

        if (s.b[1588] && (!s.b[1590])) {
            s.store_scaled_add_ad(104, A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(13), (-1.0), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if s.b[1588] {
            s.store_mul(106, 104, 108);
            s.store_div_from_scalar(107, 1.0, 106);
            s.store_mul(65, 64, 107);
            s.store_mul(73, 70, 107);
            s.store_mul(58, 482, 107);
            s.store_mul_neg_ad_lhs(677, A::add_scaled_product(s.ad_value(673), 1.0, s.ad_value(617), s.ad_value(61), 1.0), 76);
            s.store_ad_value(124, A::mul_offset_rhs(A::add_scaled_inputs_product(s.ad_value(618), 1.0, s.ad_value(619), 1.0 / (s.v[30]), s.ad_value(620), s.ad_value(61), 1.0), A::pow(s.ad_value(395), s.ad_value(621)), (-1.0)));
            s.store_mul_ad_rhs(679, 129, A::scale_offset(s.ad_value(61), p.p1016, 1.0));
        }

        s.b[1591] = (s.v[679] > 0.0);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1591]) {
            s.store_div_from_scalar(12, (p.p1015 * s.v[30]), 679);
        }

        s.b[1592] = (s.v[12] < 40.0);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1591]) && s.b[1592]) {
            s.store_div_from_scalar_offset_ad(676, (0.5 * p.p1014), A::cosh(s.ad_value(12)), (-1.0));
        }

        if ((s.b[1588] && s.b[1591]) && (!s.b[1592])) {
            s.store_scaled_limited_exp_scaled_input(676, 12, -1.0, p.p1014);
        }

        if (s.b[1588] && (!s.b[1591])) {
            s.store_scalar(676, 0.0);
        }

        if s.b[1588] {
            s.store_mul_sub_rhs(678, 676, 675, 127);
            s.store_add_ad_lhs(79, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(677), 1.0, s.ad_value(124), (-1.0), s.ad_value(678), 1.0, s.ad_value(688), 1.0, p.p961), 1.0, A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61), (-1.0)), 665);
            s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));
            s.store_scalar(680, (p.p958 * (1.0 + (p.p959 * ((s.v[30]) as f64).powf((-p.p960))))));
            s.store_scaled_sqrt_ad(687, A::mul_scaled_lhs(s.ad_value(686), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(107)), 1.0 / (s.v[46]));
            s.store_mul_offset_rhs(687, 687, 680, 1.0);
            s.store_div(685, 684, 104);
            s.store_scalar(13, 1.0);
            s.store_div(204, 59, 13);
            s.store_div(205, 687, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1593] = (s.v[204] < 0.0);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1593]) {
            s.store_div_scaled_inputs2(15, s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1588] && (!s.b[1593])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1588] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(685), (-2.0), s.ad_value(73), -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1594] = (s.v[20] <= (-68.0));
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1594]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1595] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1594]) && s.b[1595]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1596] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && s.b[1596]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && (!s.b[1596])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1594]) {
            s.store_mul_ad_rhs(693, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1588] && (!s.b[1594])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(693, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        if s.b[1588] {
            s.store_add_scaled_product_indices(681, 106, 2.0, 106, 693, 2.0);
            s.copy_ad(682, 681);
            s.store_add(682, 682, 70);
        }

        s.b[1597] = ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001)));
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1597]) {
            s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(682), 16.0, s.ad_value(70), 16.0));
        }

        if (s.b[1588] && (!s.b[1597])) {
            s.store_add_scaled_inputs3(683, s.ad_value(682), 0.5, s.ad_value(70), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(682), s.ad_value(70)), A::sub(s.ad_value(682), s.ad_value(70))), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1588] {
            s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(139, 74, 20);
            s.store_mul_add_lhs(142, 139, 70, 107);
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0);
            s.store_add_scaled_inputs3(13, s.ad_value(91), 1.0, s.ad_value(685), (-2.0), s.ad_value(142), -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1598] = (s.v[20] <= (-68.0));
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1598]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1599] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1598]) && s.b[1599]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1600] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && s.b[1600]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_div_scaled_inputs2(14, s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1598]) {
            s.store_mul_ad_rhs(692, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1588] && (!s.b[1598])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_add_scaled_inputs3(16, s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(692, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1601] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1601]) {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (s.b[1588] && (!s.b[1601])) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(96, 93);
            s.store_add_scaled_inputs3_offset(92, s.ad_value(91), 1.0, s.ad_value(693), (-1.0), s.ad_value(692), -1.0, (-1.0));
        }

        s.b[1602] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1602]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1588] && (!s.b[1602])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(14, 12);
            s.store_offset_div_ad(691, s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14)), 1.0);
            s.store_mul_ad_lhs(672, A::mul3(A::mul3_scaled_output(s.ad_value(691), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((p.p957 * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692)))), 175);
            s.store_add(188, 672, 188);
            s.store_scalar(696, (p.p785 * p.p1062));
            s.store_scalar(697, (p.p799 * p.p1062));
            s.store_scalar(698, (p.p800 * p.p1062));
            s.store_sub_from_scalar_ad(694, s.v[30], A::scale(s.ad_value(359), 2.0));
            s.store_square(695, 694);
            s.store_mul_scaled_ad_rhs(367, 108, 1.0 / (1.60219e-19), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(613)));
            s.store_mul3_affine_lhs(366, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 692);
            s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(672)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);
            s.store_mul3_affine_lhs(737, 108, 672, 1.60219e-19, 0.0, 672);
            s.store_add_ad(738, A::add_scaled_product(s.ad_value(696), 1.0, s.ad_value(697), s.ad_value(366), 1.0), A::mul3(s.ad_value(698), s.ad_value(366), s.ad_value(366)));
            s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));
            s.store_scaled_mul(740, 696, 108, 1.60219e-19);
            s.store_mul3_affine_lhs(365, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 693);
            s.store_mul_ln_ad_rhs(13, 696, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));
            s.store_mul_sub_rhs(14, 697, 365, 366);
            s.store_mul_scaled_ad_rhs(15, 698, 0.5, A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))));
            s.store_scale(16, 695, (10000000000.0 * (p.p957 * p.p2)));
            s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);
            s.store_mul3_affine_lhs(17, 694, 367, ((p.p957 * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(369, A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(672), 672);
            s.store_add(18, 369, 368);
        }

        s.b[1603] = (s.v[18] > 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1603]) {
            s.store_div_scaled_product_indices(19, 368, 369, 1.0, 18, 1.0);
            s.store_offset_scaled_ad(20, A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p.p1064), p.p1063, 1.0);
        }

        s.b[1604] = (s.v[57] > 0.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if s.b[1604] {
            s.store_scaled_mul(785, 187, 196, p.p29);
            s.store_scaled_mul(786, 187, 197, p.p29);
        }

        if (!s.b[1604]) {
            s.store_scaled_mul(785, 187, 197, p.p29);
            s.store_scaled_mul(786, 187, 196, p.p29);
        }

        s.b[1605] = ((p.p1094 == 1.0) && (p.p1095 == 1.0));
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if s.b[1605] {
            s.store_add(221, 221, 774);
            s.store_add(224, 224, 775);
        }

        s.b[1606] = (p.p1096 == 1.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if (s.b[1605] && s.b[1606]) {
            s.store_add(221, 221, 776);
            s.store_add(223, 223, 777);
        }

        s.store_scaled_mul(787, 187, 198, p.p29);

        s.b[1612] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if s.b[1612] {
            s.store_div_from_scalar(372, 1.0, 242);
        }

        s.b[1613] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (s.b[1612] && s.b[1613]) {
            s.store_div_from_scalar(374, 1.0, 759);
        }

        s.b[1614] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if s.b[1614] {
            s.store_div_from_scalar(371, 1.0, 241);
        }

        s.b[1615] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (s.b[1614] && s.b[1615]) {
            s.store_div_from_scalar(373, 1.0, 761);
        }

        s.b[1621] = ((p.p49 != 0.0) && (p.p909 > 0.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if s.b[1621] {
            s.store_mul_voltage_ad(749, A::mul3(s.ad_value(187), s.ad_value(57), s.ad_value(188)), ctx, nodes, Some(5), Some(7));
        }

        s.b[1622] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        s.b[1623] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1621] && s.b[1622]) && s.b[1623]) {
            s.store_add_scaled_value_products(749, s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), s.ad_value(372), 1.0, A::square(A::voltage(ctx, nodes, Some(6), Some(5))), s.ad_value(374), 1.0);
        }

        if ((s.b[1621] && s.b[1622]) && (!s.b[1623])) {
            s.store_add_scaled_product_left_ad(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), 372, 1.0);
        }

        s.b[1624] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        s.b[1625] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1621] && s.b[1624]) && s.b[1625]) {
            s.store_add_scaled_value_products(749, s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), s.ad_value(371), 1.0, A::square(A::voltage(ctx, nodes, Some(8), Some(7))), s.ad_value(373), 1.0);
        }

        if ((s.b[1621] && s.b[1624]) && (!s.b[1625])) {
            s.store_add_scaled_product_left_ad(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 371, 1.0);
        }

        s.b[1627] = (p.p8 != 0.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        s.b[1628] = (p.p1097 == 0.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1630] = ((p.p8 != 0.0) && (p.p1097 == 1.0));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

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
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq0_e1199,) = {
    if (s.b[896] && s.b[897]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1199;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e1207, eq1_e1207_d_n0, eq1_e1207_d_n1, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14, eq1_e1207_d_n15, eq1_e1207_d_n16, eq1_e1207_d_b0, eq1_e1207_d_b1, eq1_e1207_d_b2, eq1_e1207_d_b3, eq1_e1207_d_b4, eq1_e1207_d_b5, eq1_e1207_d_b6, eq1_e1207_d_b7, eq1_e1207_d_b8, eq1_e1207_d_b9, eq1_e1207_d_b10, eq1_e1207_d_b11, eq1_e1207_d_b12, eq1_e1207_d_b13,) = {
    if s.b[1538] {
        let eq1_e1203: f64 = (s.v[187] * p.p28);
        let eq1_e1203_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq1_e1203_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq1_e1203_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq1_e1203_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq1_e1203_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq1_e1203_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq1_e1203_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq1_e1203_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq1_e1203_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq1_e1203_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq1_e1203_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq1_e1203_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq1_e1203_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq1_e1203_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq1_e1203_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq1_e1203_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq1_e1203_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq1_e1203_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq1_e1203_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq1_e1203_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq1_e1203_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq1_e1203_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq1_e1203_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq1_e1203_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq1_e1203_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq1_e1203_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq1_e1203_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq1_e1203_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq1_e1203_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq1_e1203_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq1_e1203_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq1_e1205: f64 = (eq1_e1203 * s.v[706]);
        let eq1_e1205_d_n0: f64 = ((eq1_e1203_d_n0 * s.v[706]) + (eq1_e1203 * s.dn[706][0]));
        let eq1_e1205_d_n1: f64 = ((eq1_e1203_d_n1 * s.v[706]) + (eq1_e1203 * s.dn[706][1]));
        let eq1_e1205_d_n2: f64 = ((eq1_e1203_d_n2 * s.v[706]) + (eq1_e1203 * s.dn[706][2]));
        let eq1_e1205_d_n3: f64 = ((eq1_e1203_d_n3 * s.v[706]) + (eq1_e1203 * s.dn[706][3]));
        let eq1_e1205_d_n4: f64 = ((eq1_e1203_d_n4 * s.v[706]) + (eq1_e1203 * s.dn[706][4]));
        let eq1_e1205_d_n5: f64 = ((eq1_e1203_d_n5 * s.v[706]) + (eq1_e1203 * s.dn[706][5]));
        let eq1_e1205_d_n6: f64 = ((eq1_e1203_d_n6 * s.v[706]) + (eq1_e1203 * s.dn[706][6]));
        let eq1_e1205_d_n7: f64 = ((eq1_e1203_d_n7 * s.v[706]) + (eq1_e1203 * s.dn[706][7]));
        let eq1_e1205_d_n8: f64 = ((eq1_e1203_d_n8 * s.v[706]) + (eq1_e1203 * s.dn[706][8]));
        let eq1_e1205_d_n9: f64 = ((eq1_e1203_d_n9 * s.v[706]) + (eq1_e1203 * s.dn[706][9]));
        let eq1_e1205_d_n10: f64 = ((eq1_e1203_d_n10 * s.v[706]) + (eq1_e1203 * s.dn[706][10]));
        let eq1_e1205_d_n11: f64 = ((eq1_e1203_d_n11 * s.v[706]) + (eq1_e1203 * s.dn[706][11]));
        let eq1_e1205_d_n12: f64 = ((eq1_e1203_d_n12 * s.v[706]) + (eq1_e1203 * s.dn[706][12]));
        let eq1_e1205_d_n13: f64 = ((eq1_e1203_d_n13 * s.v[706]) + (eq1_e1203 * s.dn[706][13]));
        let eq1_e1205_d_n14: f64 = ((eq1_e1203_d_n14 * s.v[706]) + (eq1_e1203 * s.dn[706][14]));
        let eq1_e1205_d_n15: f64 = ((eq1_e1203_d_n15 * s.v[706]) + (eq1_e1203 * s.dn[706][15]));
        let eq1_e1205_d_n16: f64 = ((eq1_e1203_d_n16 * s.v[706]) + (eq1_e1203 * s.dn[706][16]));
        let eq1_e1205_d_b0: f64 = ((eq1_e1203_d_b0 * s.v[706]) + (eq1_e1203 * s.db[706][0]));
        let eq1_e1205_d_b1: f64 = ((eq1_e1203_d_b1 * s.v[706]) + (eq1_e1203 * s.db[706][1]));
        let eq1_e1205_d_b2: f64 = ((eq1_e1203_d_b2 * s.v[706]) + (eq1_e1203 * s.db[706][2]));
        let eq1_e1205_d_b3: f64 = ((eq1_e1203_d_b3 * s.v[706]) + (eq1_e1203 * s.db[706][3]));
        let eq1_e1205_d_b4: f64 = ((eq1_e1203_d_b4 * s.v[706]) + (eq1_e1203 * s.db[706][4]));
        let eq1_e1205_d_b5: f64 = ((eq1_e1203_d_b5 * s.v[706]) + (eq1_e1203 * s.db[706][5]));
        let eq1_e1205_d_b6: f64 = ((eq1_e1203_d_b6 * s.v[706]) + (eq1_e1203 * s.db[706][6]));
        let eq1_e1205_d_b7: f64 = ((eq1_e1203_d_b7 * s.v[706]) + (eq1_e1203 * s.db[706][7]));
        let eq1_e1205_d_b8: f64 = ((eq1_e1203_d_b8 * s.v[706]) + (eq1_e1203 * s.db[706][8]));
        let eq1_e1205_d_b9: f64 = ((eq1_e1203_d_b9 * s.v[706]) + (eq1_e1203 * s.db[706][9]));
        let eq1_e1205_d_b10: f64 = ((eq1_e1203_d_b10 * s.v[706]) + (eq1_e1203 * s.db[706][10]));
        let eq1_e1205_d_b11: f64 = ((eq1_e1203_d_b11 * s.v[706]) + (eq1_e1203 * s.db[706][11]));
        let eq1_e1205_d_b12: f64 = ((eq1_e1203_d_b12 * s.v[706]) + (eq1_e1203 * s.db[706][12]));
        let eq1_e1205_d_b13: f64 = ((eq1_e1203_d_b13 * s.v[706]) + (eq1_e1203 * s.db[706][13]));
        (eq1_e1205, eq1_e1205_d_n0, eq1_e1205_d_n1, eq1_e1205_d_n2, eq1_e1205_d_n3, eq1_e1205_d_n4, eq1_e1205_d_n5, eq1_e1205_d_n6, eq1_e1205_d_n7, eq1_e1205_d_n8, eq1_e1205_d_n9, eq1_e1205_d_n10, eq1_e1205_d_n11, eq1_e1205_d_n12, eq1_e1205_d_n13, eq1_e1205_d_n14, eq1_e1205_d_n15, eq1_e1205_d_n16, eq1_e1205_d_b0, eq1_e1205_d_b1, eq1_e1205_d_b2, eq1_e1205_d_b3, eq1_e1205_d_b4, eq1_e1205_d_b5, eq1_e1205_d_b6, eq1_e1205_d_b7, eq1_e1205_d_b8, eq1_e1205_d_b9, eq1_e1205_d_b10, eq1_e1205_d_b11, eq1_e1205_d_b12, eq1_e1205_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1207;
        let eq1_node_derivatives: [f64; 17] = [eq1_e1207_d_n0, eq1_e1207_d_n1, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14, eq1_e1207_d_n15, eq1_e1207_d_n16];
        let eq1_branch_derivatives: [f64; 14] = [eq1_e1207_d_b0, eq1_e1207_d_b1, eq1_e1207_d_b2, eq1_e1207_d_b3, eq1_e1207_d_b4, eq1_e1207_d_b5, eq1_e1207_d_b6, eq1_e1207_d_b7, eq1_e1207_d_b8, eq1_e1207_d_b9, eq1_e1207_d_b10, eq1_e1207_d_b11, eq1_e1207_d_b12, eq1_e1207_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1218,) = {
    if s.b[1541] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1218;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq2_value),
        );
        let (eq3_e1230,) = {
    if (!s.b[1541]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1230;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
        );
        let (eq4_e1238,) = {
    if s.b[1555] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1238;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq4_value),
        );
        let (eq5_e1247,) = {
    if (s.b[1556] && (!s.b[1555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1247;
        stamper.stamp_current_const_local(
            Some(16),
            None,
            multiplicity * (eq5_value),
        );
        let (eq6_e1262,) = {
    if (s.b[1556] && (!s.b[1555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1262;
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (eq6_value),
        );
        let (eq7_e1272, eq7_e1272_d_n0, eq7_e1272_d_n1, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n15, eq7_e1272_d_n16, eq7_e1272_d_b0, eq7_e1272_d_b1, eq7_e1272_d_b2, eq7_e1272_d_b3, eq7_e1272_d_b4, eq7_e1272_d_b5, eq7_e1272_d_b6, eq7_e1272_d_b7, eq7_e1272_d_b8, eq7_e1272_d_b9, eq7_e1272_d_b10, eq7_e1272_d_b11, eq7_e1272_d_b12, eq7_e1272_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq7_e1268: f64 = (-s.v[385]);
        let eq7_e1268_d_n0: f64 = (-s.dn[385][0]);
        let eq7_e1268_d_n1: f64 = (-s.dn[385][1]);
        let eq7_e1268_d_n2: f64 = (-s.dn[385][2]);
        let eq7_e1268_d_n3: f64 = (-s.dn[385][3]);
        let eq7_e1268_d_n4: f64 = (-s.dn[385][4]);
        let eq7_e1268_d_n5: f64 = (-s.dn[385][5]);
        let eq7_e1268_d_n6: f64 = (-s.dn[385][6]);
        let eq7_e1268_d_n7: f64 = (-s.dn[385][7]);
        let eq7_e1268_d_n8: f64 = (-s.dn[385][8]);
        let eq7_e1268_d_n9: f64 = (-s.dn[385][9]);
        let eq7_e1268_d_n10: f64 = (-s.dn[385][10]);
        let eq7_e1268_d_n11: f64 = (-s.dn[385][11]);
        let eq7_e1268_d_n12: f64 = (-s.dn[385][12]);
        let eq7_e1268_d_n13: f64 = (-s.dn[385][13]);
        let eq7_e1268_d_n14: f64 = (-s.dn[385][14]);
        let eq7_e1268_d_n15: f64 = (-s.dn[385][15]);
        let eq7_e1268_d_n16: f64 = (-s.dn[385][16]);
        let eq7_e1268_d_b0: f64 = (-s.db[385][0]);
        let eq7_e1268_d_b1: f64 = (-s.db[385][1]);
        let eq7_e1268_d_b2: f64 = (-s.db[385][2]);
        let eq7_e1268_d_b3: f64 = (-s.db[385][3]);
        let eq7_e1268_d_b4: f64 = (-s.db[385][4]);
        let eq7_e1268_d_b5: f64 = (-s.db[385][5]);
        let eq7_e1268_d_b6: f64 = (-s.db[385][6]);
        let eq7_e1268_d_b7: f64 = (-s.db[385][7]);
        let eq7_e1268_d_b8: f64 = (-s.db[385][8]);
        let eq7_e1268_d_b9: f64 = (-s.db[385][9]);
        let eq7_e1268_d_b10: f64 = (-s.db[385][10]);
        let eq7_e1268_d_b11: f64 = (-s.db[385][11]);
        let eq7_e1268_d_b12: f64 = (-s.db[385][12]);
        let eq7_e1268_d_b13: f64 = (-s.db[385][13]);
        let eq7_e1270: f64 = (eq7_e1268 * (nv16 - 0.0));
        let eq7_e1270_d_n0: f64 = (eq7_e1268_d_n0 * (nv16 - 0.0));
        let eq7_e1270_d_n1: f64 = (eq7_e1268_d_n1 * (nv16 - 0.0));
        let eq7_e1270_d_n2: f64 = (eq7_e1268_d_n2 * (nv16 - 0.0));
        let eq7_e1270_d_n3: f64 = (eq7_e1268_d_n3 * (nv16 - 0.0));
        let eq7_e1270_d_n4: f64 = (eq7_e1268_d_n4 * (nv16 - 0.0));
        let eq7_e1270_d_n5: f64 = (eq7_e1268_d_n5 * (nv16 - 0.0));
        let eq7_e1270_d_n6: f64 = (eq7_e1268_d_n6 * (nv16 - 0.0));
        let eq7_e1270_d_n7: f64 = (eq7_e1268_d_n7 * (nv16 - 0.0));
        let eq7_e1270_d_n8: f64 = (eq7_e1268_d_n8 * (nv16 - 0.0));
        let eq7_e1270_d_n9: f64 = (eq7_e1268_d_n9 * (nv16 - 0.0));
        let eq7_e1270_d_n10: f64 = (eq7_e1268_d_n10 * (nv16 - 0.0));
        let eq7_e1270_d_n11: f64 = (eq7_e1268_d_n11 * (nv16 - 0.0));
        let eq7_e1270_d_n12: f64 = (eq7_e1268_d_n12 * (nv16 - 0.0));
        let eq7_e1270_d_n13: f64 = (eq7_e1268_d_n13 * (nv16 - 0.0));
        let eq7_e1270_d_n14: f64 = (eq7_e1268_d_n14 * (nv16 - 0.0));
        let eq7_e1270_d_n15: f64 = (eq7_e1268_d_n15 * (nv16 - 0.0));
        let eq7_e1270_d_n16: f64 = ((eq7_e1268_d_n16 * (nv16 - 0.0)) + eq7_e1268);
        let eq7_e1270_d_b0: f64 = (eq7_e1268_d_b0 * (nv16 - 0.0));
        let eq7_e1270_d_b1: f64 = (eq7_e1268_d_b1 * (nv16 - 0.0));
        let eq7_e1270_d_b2: f64 = (eq7_e1268_d_b2 * (nv16 - 0.0));
        let eq7_e1270_d_b3: f64 = (eq7_e1268_d_b3 * (nv16 - 0.0));
        let eq7_e1270_d_b4: f64 = (eq7_e1268_d_b4 * (nv16 - 0.0));
        let eq7_e1270_d_b5: f64 = (eq7_e1268_d_b5 * (nv16 - 0.0));
        let eq7_e1270_d_b6: f64 = (eq7_e1268_d_b6 * (nv16 - 0.0));
        let eq7_e1270_d_b7: f64 = (eq7_e1268_d_b7 * (nv16 - 0.0));
        let eq7_e1270_d_b8: f64 = (eq7_e1268_d_b8 * (nv16 - 0.0));
        let eq7_e1270_d_b9: f64 = (eq7_e1268_d_b9 * (nv16 - 0.0));
        let eq7_e1270_d_b10: f64 = (eq7_e1268_d_b10 * (nv16 - 0.0));
        let eq7_e1270_d_b11: f64 = (eq7_e1268_d_b11 * (nv16 - 0.0));
        let eq7_e1270_d_b12: f64 = (eq7_e1268_d_b12 * (nv16 - 0.0));
        let eq7_e1270_d_b13: f64 = (eq7_e1268_d_b13 * (nv16 - 0.0));
        (eq7_e1270, eq7_e1270_d_n0, eq7_e1270_d_n1, eq7_e1270_d_n2, eq7_e1270_d_n3, eq7_e1270_d_n4, eq7_e1270_d_n5, eq7_e1270_d_n6, eq7_e1270_d_n7, eq7_e1270_d_n8, eq7_e1270_d_n9, eq7_e1270_d_n10, eq7_e1270_d_n11, eq7_e1270_d_n12, eq7_e1270_d_n13, eq7_e1270_d_n14, eq7_e1270_d_n15, eq7_e1270_d_n16, eq7_e1270_d_b0, eq7_e1270_d_b1, eq7_e1270_d_b2, eq7_e1270_d_b3, eq7_e1270_d_b4, eq7_e1270_d_b5, eq7_e1270_d_b6, eq7_e1270_d_b7, eq7_e1270_d_b8, eq7_e1270_d_b9, eq7_e1270_d_b10, eq7_e1270_d_b11, eq7_e1270_d_b12, eq7_e1270_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1272;
        let eq7_node_derivatives: [f64; 17] = [eq7_e1272_d_n0, eq7_e1272_d_n1, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n15, eq7_e1272_d_n16];
        let eq7_branch_derivatives: [f64; 14] = [eq7_e1272_d_b0, eq7_e1272_d_b1, eq7_e1272_d_b2, eq7_e1272_d_b3, eq7_e1272_d_b4, eq7_e1272_d_b5, eq7_e1272_d_b6, eq7_e1272_d_b7, eq7_e1272_d_b8, eq7_e1272_d_b9, eq7_e1272_d_b10, eq7_e1272_d_b11, eq7_e1272_d_b12, eq7_e1272_d_b13];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16, eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1279_d_n0: f64 = (s.dn[378][0] * s.v[46]);
        let eq8_e1279_d_n1: f64 = (s.dn[378][1] * s.v[46]);
        let eq8_e1279_d_n2: f64 = (s.dn[378][2] * s.v[46]);
        let eq8_e1279_d_n3: f64 = (s.dn[378][3] * s.v[46]);
        let eq8_e1279_d_n4: f64 = (s.dn[378][4] * s.v[46]);
        let eq8_e1279_d_n5: f64 = (s.dn[378][5] * s.v[46]);
        let eq8_e1279_d_n6: f64 = (s.dn[378][6] * s.v[46]);
        let eq8_e1279_d_n7: f64 = (s.dn[378][7] * s.v[46]);
        let eq8_e1279_d_n8: f64 = (s.dn[378][8] * s.v[46]);
        let eq8_e1279_d_n9: f64 = (s.dn[378][9] * s.v[46]);
        let eq8_e1279_d_n10: f64 = (s.dn[378][10] * s.v[46]);
        let eq8_e1279_d_n11: f64 = (s.dn[378][11] * s.v[46]);
        let eq8_e1279_d_n12: f64 = (s.dn[378][12] * s.v[46]);
        let eq8_e1279_d_n13: f64 = (s.dn[378][13] * s.v[46]);
        let eq8_e1279_d_n14: f64 = (s.dn[378][14] * s.v[46]);
        let eq8_e1279_d_n15: f64 = (s.dn[378][15] * s.v[46]);
        let eq8_e1279_d_n16: f64 = (s.dn[378][16] * s.v[46]);
        let eq8_e1279_d_b0: f64 = (s.db[378][0] * s.v[46]);
        let eq8_e1279_d_b1: f64 = (s.db[378][1] * s.v[46]);
        let eq8_e1279_d_b2: f64 = (s.db[378][2] * s.v[46]);
        let eq8_e1279_d_b3: f64 = (s.db[378][3] * s.v[46]);
        let eq8_e1279_d_b4: f64 = (s.db[378][4] * s.v[46]);
        let eq8_e1279_d_b5: f64 = (s.db[378][5] * s.v[46]);
        let eq8_e1279_d_b6: f64 = (s.db[378][6] * s.v[46]);
        let eq8_e1279_d_b7: f64 = (s.db[378][7] * s.v[46]);
        let eq8_e1279_d_b8: f64 = (s.db[378][8] * s.v[46]);
        let eq8_e1279_d_b9: f64 = (s.db[378][9] * s.v[46]);
        let eq8_e1279_d_b10: f64 = (s.db[378][10] * s.v[46]);
        let eq8_e1279_d_b11: f64 = (s.db[378][11] * s.v[46]);
        let eq8_e1279_d_b12: f64 = (s.db[378][12] * s.v[46]);
        let eq8_e1279_d_b13: f64 = (s.db[378][13] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * s.v[29]);
        let eq8_e1281_d_n1: f64 = (eq8_e1279_d_n1 * s.v[29]);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * s.v[29]);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * s.v[29]);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * s.v[29]);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * s.v[29]);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * s.v[29]);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * s.v[29]);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * s.v[29]);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * s.v[29]);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * s.v[29]);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * s.v[29]);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * s.v[29]);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * s.v[29]);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * s.v[29]);
        let eq8_e1281_d_n15: f64 = (eq8_e1279_d_n15 * s.v[29]);
        let eq8_e1281_d_n16: f64 = (eq8_e1279_d_n16 * s.v[29]);
        let eq8_e1281_d_b0: f64 = (eq8_e1279_d_b0 * s.v[29]);
        let eq8_e1281_d_b1: f64 = (eq8_e1279_d_b1 * s.v[29]);
        let eq8_e1281_d_b2: f64 = (eq8_e1279_d_b2 * s.v[29]);
        let eq8_e1281_d_b3: f64 = (eq8_e1279_d_b3 * s.v[29]);
        let eq8_e1281_d_b4: f64 = (eq8_e1279_d_b4 * s.v[29]);
        let eq8_e1281_d_b5: f64 = (eq8_e1279_d_b5 * s.v[29]);
        let eq8_e1281_d_b6: f64 = (eq8_e1279_d_b6 * s.v[29]);
        let eq8_e1281_d_b7: f64 = (eq8_e1279_d_b7 * s.v[29]);
        let eq8_e1281_d_b8: f64 = (eq8_e1279_d_b8 * s.v[29]);
        let eq8_e1281_d_b9: f64 = (eq8_e1279_d_b9 * s.v[29]);
        let eq8_e1281_d_b10: f64 = (eq8_e1279_d_b10 * s.v[29]);
        let eq8_e1281_d_b11: f64 = (eq8_e1279_d_b11 * s.v[29]);
        let eq8_e1281_d_b12: f64 = (eq8_e1279_d_b12 * s.v[29]);
        let eq8_e1281_d_b13: f64 = (eq8_e1279_d_b13 * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1283_d_b0: f64 = (eq8_e1281_d_b0 * p.p2);
        let eq8_e1283_d_b1: f64 = (eq8_e1281_d_b1 * p.p2);
        let eq8_e1283_d_b2: f64 = (eq8_e1281_d_b2 * p.p2);
        let eq8_e1283_d_b3: f64 = (eq8_e1281_d_b3 * p.p2);
        let eq8_e1283_d_b4: f64 = (eq8_e1281_d_b4 * p.p2);
        let eq8_e1283_d_b5: f64 = (eq8_e1281_d_b5 * p.p2);
        let eq8_e1283_d_b6: f64 = (eq8_e1281_d_b6 * p.p2);
        let eq8_e1283_d_b7: f64 = (eq8_e1281_d_b7 * p.p2);
        let eq8_e1283_d_b8: f64 = (eq8_e1281_d_b8 * p.p2);
        let eq8_e1283_d_b9: f64 = (eq8_e1281_d_b9 * p.p2);
        let eq8_e1283_d_b10: f64 = (eq8_e1281_d_b10 * p.p2);
        let eq8_e1283_d_b11: f64 = (eq8_e1281_d_b11 * p.p2);
        let eq8_e1283_d_b12: f64 = (eq8_e1281_d_b12 * p.p2);
        let eq8_e1283_d_b13: f64 = (eq8_e1281_d_b13 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1285_d_b0: f64 = (eq8_e1283_d_b0 * s.v[30]);
        let eq8_e1285_d_b1: f64 = (eq8_e1283_d_b1 * s.v[30]);
        let eq8_e1285_d_b2: f64 = (eq8_e1283_d_b2 * s.v[30]);
        let eq8_e1285_d_b3: f64 = (eq8_e1283_d_b3 * s.v[30]);
        let eq8_e1285_d_b4: f64 = (eq8_e1283_d_b4 * s.v[30]);
        let eq8_e1285_d_b5: f64 = (eq8_e1283_d_b5 * s.v[30]);
        let eq8_e1285_d_b6: f64 = (eq8_e1283_d_b6 * s.v[30]);
        let eq8_e1285_d_b7: f64 = (eq8_e1283_d_b7 * s.v[30]);
        let eq8_e1285_d_b8: f64 = (eq8_e1283_d_b8 * s.v[30]);
        let eq8_e1285_d_b9: f64 = (eq8_e1283_d_b9 * s.v[30]);
        let eq8_e1285_d_b10: f64 = (eq8_e1283_d_b10 * s.v[30]);
        let eq8_e1285_d_b11: f64 = (eq8_e1283_d_b11 * s.v[30]);
        let eq8_e1285_d_b12: f64 = (eq8_e1283_d_b12 * s.v[30]);
        let eq8_e1285_d_b13: f64 = (eq8_e1283_d_b13 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1287_d_b0: f64 = (eq8_e1285_d_b0 * (nv15 - 0.0));
        let eq8_e1287_d_b1: f64 = (eq8_e1285_d_b1 * (nv15 - 0.0));
        let eq8_e1287_d_b2: f64 = (eq8_e1285_d_b2 * (nv15 - 0.0));
        let eq8_e1287_d_b3: f64 = (eq8_e1285_d_b3 * (nv15 - 0.0));
        let eq8_e1287_d_b4: f64 = (eq8_e1285_d_b4 * (nv15 - 0.0));
        let eq8_e1287_d_b5: f64 = (eq8_e1285_d_b5 * (nv15 - 0.0));
        let eq8_e1287_d_b6: f64 = (eq8_e1285_d_b6 * (nv15 - 0.0));
        let eq8_e1287_d_b7: f64 = (eq8_e1285_d_b7 * (nv15 - 0.0));
        let eq8_e1287_d_b8: f64 = (eq8_e1285_d_b8 * (nv15 - 0.0));
        let eq8_e1287_d_b9: f64 = (eq8_e1285_d_b9 * (nv15 - 0.0));
        let eq8_e1287_d_b10: f64 = (eq8_e1285_d_b10 * (nv15 - 0.0));
        let eq8_e1287_d_b11: f64 = (eq8_e1285_d_b11 * (nv15 - 0.0));
        let eq8_e1287_d_b12: f64 = (eq8_e1285_d_b12 * (nv15 - 0.0));
        let eq8_e1287_d_b13: f64 = (eq8_e1285_d_b13 * (nv15 - 0.0));
        let eq8_e1288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq8_e1287);
        let eq8_e1288_d_n0: f64 = (eq8_e1287_d_n0 * ddt_scale);
        let eq8_e1288_d_n1: f64 = (eq8_e1287_d_n1 * ddt_scale);
        let eq8_e1288_d_n2: f64 = (eq8_e1287_d_n2 * ddt_scale);
        let eq8_e1288_d_n3: f64 = (eq8_e1287_d_n3 * ddt_scale);
        let eq8_e1288_d_n4: f64 = (eq8_e1287_d_n4 * ddt_scale);
        let eq8_e1288_d_n5: f64 = (eq8_e1287_d_n5 * ddt_scale);
        let eq8_e1288_d_n6: f64 = (eq8_e1287_d_n6 * ddt_scale);
        let eq8_e1288_d_n7: f64 = (eq8_e1287_d_n7 * ddt_scale);
        let eq8_e1288_d_n8: f64 = (eq8_e1287_d_n8 * ddt_scale);
        let eq8_e1288_d_n9: f64 = (eq8_e1287_d_n9 * ddt_scale);
        let eq8_e1288_d_n10: f64 = (eq8_e1287_d_n10 * ddt_scale);
        let eq8_e1288_d_n11: f64 = (eq8_e1287_d_n11 * ddt_scale);
        let eq8_e1288_d_n12: f64 = (eq8_e1287_d_n12 * ddt_scale);
        let eq8_e1288_d_n13: f64 = (eq8_e1287_d_n13 * ddt_scale);
        let eq8_e1288_d_n14: f64 = (eq8_e1287_d_n14 * ddt_scale);
        let eq8_e1288_d_n15: f64 = (eq8_e1287_d_n15 * ddt_scale);
        let eq8_e1288_d_n16: f64 = (eq8_e1287_d_n16 * ddt_scale);
        let eq8_e1288_d_b0: f64 = (eq8_e1287_d_b0 * ddt_scale);
        let eq8_e1288_d_b1: f64 = (eq8_e1287_d_b1 * ddt_scale);
        let eq8_e1288_d_b2: f64 = (eq8_e1287_d_b2 * ddt_scale);
        let eq8_e1288_d_b3: f64 = (eq8_e1287_d_b3 * ddt_scale);
        let eq8_e1288_d_b4: f64 = (eq8_e1287_d_b4 * ddt_scale);
        let eq8_e1288_d_b5: f64 = (eq8_e1287_d_b5 * ddt_scale);
        let eq8_e1288_d_b6: f64 = (eq8_e1287_d_b6 * ddt_scale);
        let eq8_e1288_d_b7: f64 = (eq8_e1287_d_b7 * ddt_scale);
        let eq8_e1288_d_b8: f64 = (eq8_e1287_d_b8 * ddt_scale);
        let eq8_e1288_d_b9: f64 = (eq8_e1287_d_b9 * ddt_scale);
        let eq8_e1288_d_b10: f64 = (eq8_e1287_d_b10 * ddt_scale);
        let eq8_e1288_d_b11: f64 = (eq8_e1287_d_b11 * ddt_scale);
        let eq8_e1288_d_b12: f64 = (eq8_e1287_d_b12 * ddt_scale);
        let eq8_e1288_d_b13: f64 = (eq8_e1287_d_b13 * ddt_scale);
        (eq8_e1288, eq8_e1288_d_n0, eq8_e1288_d_n1, eq8_e1288_d_n2, eq8_e1288_d_n3, eq8_e1288_d_n4, eq8_e1288_d_n5, eq8_e1288_d_n6, eq8_e1288_d_n7, eq8_e1288_d_n8, eq8_e1288_d_n9, eq8_e1288_d_n10, eq8_e1288_d_n11, eq8_e1288_d_n12, eq8_e1288_d_n13, eq8_e1288_d_n14, eq8_e1288_d_n15, eq8_e1288_d_n16, eq8_e1288_d_b0, eq8_e1288_d_b1, eq8_e1288_d_b2, eq8_e1288_d_b3, eq8_e1288_d_b4, eq8_e1288_d_b5, eq8_e1288_d_b6, eq8_e1288_d_b7, eq8_e1288_d_b8, eq8_e1288_d_b9, eq8_e1288_d_b10, eq8_e1288_d_b11, eq8_e1288_d_b12, eq8_e1288_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1290;
        let eq8_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16];
        let eq8_branch_derivatives: [f64; 14] = [eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e1307,) = {
    if (s.b[1556] && (!s.b[1555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e1307;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq9_value),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq10_e1318, eq10_e1318_d_n0, eq10_e1318_d_n1, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n15, eq10_e1318_d_n16, eq10_e1318_d_b0, eq10_e1318_d_b1, eq10_e1318_d_b2, eq10_e1318_d_b3, eq10_e1318_d_b4, eq10_e1318_d_b5, eq10_e1318_d_b6, eq10_e1318_d_b7, eq10_e1318_d_b8, eq10_e1318_d_b9, eq10_e1318_d_b10, eq10_e1318_d_b11, eq10_e1318_d_b12, eq10_e1318_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq10_e1314: f64 = (s.v[384] * p.p28);
        let eq10_e1314_d_n0: f64 = (s.dn[384][0] * p.p28);
        let eq10_e1314_d_n1: f64 = (s.dn[384][1] * p.p28);
        let eq10_e1314_d_n2: f64 = (s.dn[384][2] * p.p28);
        let eq10_e1314_d_n3: f64 = (s.dn[384][3] * p.p28);
        let eq10_e1314_d_n4: f64 = (s.dn[384][4] * p.p28);
        let eq10_e1314_d_n5: f64 = (s.dn[384][5] * p.p28);
        let eq10_e1314_d_n6: f64 = (s.dn[384][6] * p.p28);
        let eq10_e1314_d_n7: f64 = (s.dn[384][7] * p.p28);
        let eq10_e1314_d_n8: f64 = (s.dn[384][8] * p.p28);
        let eq10_e1314_d_n9: f64 = (s.dn[384][9] * p.p28);
        let eq10_e1314_d_n10: f64 = (s.dn[384][10] * p.p28);
        let eq10_e1314_d_n11: f64 = (s.dn[384][11] * p.p28);
        let eq10_e1314_d_n12: f64 = (s.dn[384][12] * p.p28);
        let eq10_e1314_d_n13: f64 = (s.dn[384][13] * p.p28);
        let eq10_e1314_d_n14: f64 = (s.dn[384][14] * p.p28);
        let eq10_e1314_d_n15: f64 = (s.dn[384][15] * p.p28);
        let eq10_e1314_d_n16: f64 = (s.dn[384][16] * p.p28);
        let eq10_e1314_d_b0: f64 = (s.db[384][0] * p.p28);
        let eq10_e1314_d_b1: f64 = (s.db[384][1] * p.p28);
        let eq10_e1314_d_b2: f64 = (s.db[384][2] * p.p28);
        let eq10_e1314_d_b3: f64 = (s.db[384][3] * p.p28);
        let eq10_e1314_d_b4: f64 = (s.db[384][4] * p.p28);
        let eq10_e1314_d_b5: f64 = (s.db[384][5] * p.p28);
        let eq10_e1314_d_b6: f64 = (s.db[384][6] * p.p28);
        let eq10_e1314_d_b7: f64 = (s.db[384][7] * p.p28);
        let eq10_e1314_d_b8: f64 = (s.db[384][8] * p.p28);
        let eq10_e1314_d_b9: f64 = (s.db[384][9] * p.p28);
        let eq10_e1314_d_b10: f64 = (s.db[384][10] * p.p28);
        let eq10_e1314_d_b11: f64 = (s.db[384][11] * p.p28);
        let eq10_e1314_d_b12: f64 = (s.db[384][12] * p.p28);
        let eq10_e1314_d_b13: f64 = (s.db[384][13] * p.p28);
        let eq10_e1316: f64 = (eq10_e1314 * (nv16 - 0.0));
        let eq10_e1316_d_n0: f64 = (eq10_e1314_d_n0 * (nv16 - 0.0));
        let eq10_e1316_d_n1: f64 = (eq10_e1314_d_n1 * (nv16 - 0.0));
        let eq10_e1316_d_n2: f64 = (eq10_e1314_d_n2 * (nv16 - 0.0));
        let eq10_e1316_d_n3: f64 = (eq10_e1314_d_n3 * (nv16 - 0.0));
        let eq10_e1316_d_n4: f64 = (eq10_e1314_d_n4 * (nv16 - 0.0));
        let eq10_e1316_d_n5: f64 = (eq10_e1314_d_n5 * (nv16 - 0.0));
        let eq10_e1316_d_n6: f64 = (eq10_e1314_d_n6 * (nv16 - 0.0));
        let eq10_e1316_d_n7: f64 = (eq10_e1314_d_n7 * (nv16 - 0.0));
        let eq10_e1316_d_n8: f64 = (eq10_e1314_d_n8 * (nv16 - 0.0));
        let eq10_e1316_d_n9: f64 = (eq10_e1314_d_n9 * (nv16 - 0.0));
        let eq10_e1316_d_n10: f64 = (eq10_e1314_d_n10 * (nv16 - 0.0));
        let eq10_e1316_d_n11: f64 = (eq10_e1314_d_n11 * (nv16 - 0.0));
        let eq10_e1316_d_n12: f64 = (eq10_e1314_d_n12 * (nv16 - 0.0));
        let eq10_e1316_d_n13: f64 = (eq10_e1314_d_n13 * (nv16 - 0.0));
        let eq10_e1316_d_n14: f64 = (eq10_e1314_d_n14 * (nv16 - 0.0));
        let eq10_e1316_d_n15: f64 = (eq10_e1314_d_n15 * (nv16 - 0.0));
        let eq10_e1316_d_n16: f64 = ((eq10_e1314_d_n16 * (nv16 - 0.0)) + eq10_e1314);
        let eq10_e1316_d_b0: f64 = (eq10_e1314_d_b0 * (nv16 - 0.0));
        let eq10_e1316_d_b1: f64 = (eq10_e1314_d_b1 * (nv16 - 0.0));
        let eq10_e1316_d_b2: f64 = (eq10_e1314_d_b2 * (nv16 - 0.0));
        let eq10_e1316_d_b3: f64 = (eq10_e1314_d_b3 * (nv16 - 0.0));
        let eq10_e1316_d_b4: f64 = (eq10_e1314_d_b4 * (nv16 - 0.0));
        let eq10_e1316_d_b5: f64 = (eq10_e1314_d_b5 * (nv16 - 0.0));
        let eq10_e1316_d_b6: f64 = (eq10_e1314_d_b6 * (nv16 - 0.0));
        let eq10_e1316_d_b7: f64 = (eq10_e1314_d_b7 * (nv16 - 0.0));
        let eq10_e1316_d_b8: f64 = (eq10_e1314_d_b8 * (nv16 - 0.0));
        let eq10_e1316_d_b9: f64 = (eq10_e1314_d_b9 * (nv16 - 0.0));
        let eq10_e1316_d_b10: f64 = (eq10_e1314_d_b10 * (nv16 - 0.0));
        let eq10_e1316_d_b11: f64 = (eq10_e1314_d_b11 * (nv16 - 0.0));
        let eq10_e1316_d_b12: f64 = (eq10_e1314_d_b12 * (nv16 - 0.0));
        let eq10_e1316_d_b13: f64 = (eq10_e1314_d_b13 * (nv16 - 0.0));
        (eq10_e1316, eq10_e1316_d_n0, eq10_e1316_d_n1, eq10_e1316_d_n2, eq10_e1316_d_n3, eq10_e1316_d_n4, eq10_e1316_d_n5, eq10_e1316_d_n6, eq10_e1316_d_n7, eq10_e1316_d_n8, eq10_e1316_d_n9, eq10_e1316_d_n10, eq10_e1316_d_n11, eq10_e1316_d_n12, eq10_e1316_d_n13, eq10_e1316_d_n14, eq10_e1316_d_n15, eq10_e1316_d_n16, eq10_e1316_d_b0, eq10_e1316_d_b1, eq10_e1316_d_b2, eq10_e1316_d_b3, eq10_e1316_d_b4, eq10_e1316_d_b5, eq10_e1316_d_b6, eq10_e1316_d_b7, eq10_e1316_d_b8, eq10_e1316_d_b9, eq10_e1316_d_b10, eq10_e1316_d_b11, eq10_e1316_d_b12, eq10_e1316_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1318;
        let eq10_node_derivatives: [f64; 17] = [eq10_e1318_d_n0, eq10_e1318_d_n1, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n15, eq10_e1318_d_n16];
        let eq10_branch_derivatives: [f64; 14] = [eq10_e1318_d_b0, eq10_e1318_d_b1, eq10_e1318_d_b2, eq10_e1318_d_b3, eq10_e1318_d_b4, eq10_e1318_d_b5, eq10_e1318_d_b6, eq10_e1318_d_b7, eq10_e1318_d_b8, eq10_e1318_d_b9, eq10_e1318_d_b10, eq10_e1318_d_b11, eq10_e1318_d_b12, eq10_e1318_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16, eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1329_d_b0: f64 = ((s.db[57][0] * s.v[378]) + (eq11_e1327 * s.db[378][0]));
        let eq11_e1329_d_b1: f64 = ((s.db[57][1] * s.v[378]) + (eq11_e1327 * s.db[378][1]));
        let eq11_e1329_d_b2: f64 = ((s.db[57][2] * s.v[378]) + (eq11_e1327 * s.db[378][2]));
        let eq11_e1329_d_b3: f64 = ((s.db[57][3] * s.v[378]) + (eq11_e1327 * s.db[378][3]));
        let eq11_e1329_d_b4: f64 = ((s.db[57][4] * s.v[378]) + (eq11_e1327 * s.db[378][4]));
        let eq11_e1329_d_b5: f64 = ((s.db[57][5] * s.v[378]) + (eq11_e1327 * s.db[378][5]));
        let eq11_e1329_d_b6: f64 = ((s.db[57][6] * s.v[378]) + (eq11_e1327 * s.db[378][6]));
        let eq11_e1329_d_b7: f64 = ((s.db[57][7] * s.v[378]) + (eq11_e1327 * s.db[378][7]));
        let eq11_e1329_d_b8: f64 = ((s.db[57][8] * s.v[378]) + (eq11_e1327 * s.db[378][8]));
        let eq11_e1329_d_b9: f64 = ((s.db[57][9] * s.v[378]) + (eq11_e1327 * s.db[378][9]));
        let eq11_e1329_d_b10: f64 = ((s.db[57][10] * s.v[378]) + (eq11_e1327 * s.db[378][10]));
        let eq11_e1329_d_b11: f64 = ((s.db[57][11] * s.v[378]) + (eq11_e1327 * s.db[378][11]));
        let eq11_e1329_d_b12: f64 = ((s.db[57][12] * s.v[378]) + (eq11_e1327 * s.db[378][12]));
        let eq11_e1329_d_b13: f64 = ((s.db[57][13] * s.v[378]) + (eq11_e1327 * s.db[378][13]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1331_d_b0: f64 = (eq11_e1329_d_b0 * s.v[46]);
        let eq11_e1331_d_b1: f64 = (eq11_e1329_d_b1 * s.v[46]);
        let eq11_e1331_d_b2: f64 = (eq11_e1329_d_b2 * s.v[46]);
        let eq11_e1331_d_b3: f64 = (eq11_e1329_d_b3 * s.v[46]);
        let eq11_e1331_d_b4: f64 = (eq11_e1329_d_b4 * s.v[46]);
        let eq11_e1331_d_b5: f64 = (eq11_e1329_d_b5 * s.v[46]);
        let eq11_e1331_d_b6: f64 = (eq11_e1329_d_b6 * s.v[46]);
        let eq11_e1331_d_b7: f64 = (eq11_e1329_d_b7 * s.v[46]);
        let eq11_e1331_d_b8: f64 = (eq11_e1329_d_b8 * s.v[46]);
        let eq11_e1331_d_b9: f64 = (eq11_e1329_d_b9 * s.v[46]);
        let eq11_e1331_d_b10: f64 = (eq11_e1329_d_b10 * s.v[46]);
        let eq11_e1331_d_b11: f64 = (eq11_e1329_d_b11 * s.v[46]);
        let eq11_e1331_d_b12: f64 = (eq11_e1329_d_b12 * s.v[46]);
        let eq11_e1331_d_b13: f64 = (eq11_e1329_d_b13 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1333_d_b0: f64 = (eq11_e1331_d_b0 * s.v[29]);
        let eq11_e1333_d_b1: f64 = (eq11_e1331_d_b1 * s.v[29]);
        let eq11_e1333_d_b2: f64 = (eq11_e1331_d_b2 * s.v[29]);
        let eq11_e1333_d_b3: f64 = (eq11_e1331_d_b3 * s.v[29]);
        let eq11_e1333_d_b4: f64 = (eq11_e1331_d_b4 * s.v[29]);
        let eq11_e1333_d_b5: f64 = (eq11_e1331_d_b5 * s.v[29]);
        let eq11_e1333_d_b6: f64 = (eq11_e1331_d_b6 * s.v[29]);
        let eq11_e1333_d_b7: f64 = (eq11_e1331_d_b7 * s.v[29]);
        let eq11_e1333_d_b8: f64 = (eq11_e1331_d_b8 * s.v[29]);
        let eq11_e1333_d_b9: f64 = (eq11_e1331_d_b9 * s.v[29]);
        let eq11_e1333_d_b10: f64 = (eq11_e1331_d_b10 * s.v[29]);
        let eq11_e1333_d_b11: f64 = (eq11_e1331_d_b11 * s.v[29]);
        let eq11_e1333_d_b12: f64 = (eq11_e1331_d_b12 * s.v[29]);
        let eq11_e1333_d_b13: f64 = (eq11_e1331_d_b13 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1335_d_b0: f64 = (eq11_e1333_d_b0 * p.p2);
        let eq11_e1335_d_b1: f64 = (eq11_e1333_d_b1 * p.p2);
        let eq11_e1335_d_b2: f64 = (eq11_e1333_d_b2 * p.p2);
        let eq11_e1335_d_b3: f64 = (eq11_e1333_d_b3 * p.p2);
        let eq11_e1335_d_b4: f64 = (eq11_e1333_d_b4 * p.p2);
        let eq11_e1335_d_b5: f64 = (eq11_e1333_d_b5 * p.p2);
        let eq11_e1335_d_b6: f64 = (eq11_e1333_d_b6 * p.p2);
        let eq11_e1335_d_b7: f64 = (eq11_e1333_d_b7 * p.p2);
        let eq11_e1335_d_b8: f64 = (eq11_e1333_d_b8 * p.p2);
        let eq11_e1335_d_b9: f64 = (eq11_e1333_d_b9 * p.p2);
        let eq11_e1335_d_b10: f64 = (eq11_e1333_d_b10 * p.p2);
        let eq11_e1335_d_b11: f64 = (eq11_e1333_d_b11 * p.p2);
        let eq11_e1335_d_b12: f64 = (eq11_e1333_d_b12 * p.p2);
        let eq11_e1335_d_b13: f64 = (eq11_e1333_d_b13 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1337_d_b0: f64 = (eq11_e1335_d_b0 * s.v[30]);
        let eq11_e1337_d_b1: f64 = (eq11_e1335_d_b1 * s.v[30]);
        let eq11_e1337_d_b2: f64 = (eq11_e1335_d_b2 * s.v[30]);
        let eq11_e1337_d_b3: f64 = (eq11_e1335_d_b3 * s.v[30]);
        let eq11_e1337_d_b4: f64 = (eq11_e1335_d_b4 * s.v[30]);
        let eq11_e1337_d_b5: f64 = (eq11_e1335_d_b5 * s.v[30]);
        let eq11_e1337_d_b6: f64 = (eq11_e1335_d_b6 * s.v[30]);
        let eq11_e1337_d_b7: f64 = (eq11_e1335_d_b7 * s.v[30]);
        let eq11_e1337_d_b8: f64 = (eq11_e1335_d_b8 * s.v[30]);
        let eq11_e1337_d_b9: f64 = (eq11_e1335_d_b9 * s.v[30]);
        let eq11_e1337_d_b10: f64 = (eq11_e1335_d_b10 * s.v[30]);
        let eq11_e1337_d_b11: f64 = (eq11_e1335_d_b11 * s.v[30]);
        let eq11_e1337_d_b12: f64 = (eq11_e1335_d_b12 * s.v[30]);
        let eq11_e1337_d_b13: f64 = (eq11_e1335_d_b13 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1339_d_b0: f64 = (eq11_e1337_d_b0 * (nv15 - 0.0));
        let eq11_e1339_d_b1: f64 = (eq11_e1337_d_b1 * (nv15 - 0.0));
        let eq11_e1339_d_b2: f64 = (eq11_e1337_d_b2 * (nv15 - 0.0));
        let eq11_e1339_d_b3: f64 = (eq11_e1337_d_b3 * (nv15 - 0.0));
        let eq11_e1339_d_b4: f64 = (eq11_e1337_d_b4 * (nv15 - 0.0));
        let eq11_e1339_d_b5: f64 = (eq11_e1337_d_b5 * (nv15 - 0.0));
        let eq11_e1339_d_b6: f64 = (eq11_e1337_d_b6 * (nv15 - 0.0));
        let eq11_e1339_d_b7: f64 = (eq11_e1337_d_b7 * (nv15 - 0.0));
        let eq11_e1339_d_b8: f64 = (eq11_e1337_d_b8 * (nv15 - 0.0));
        let eq11_e1339_d_b9: f64 = (eq11_e1337_d_b9 * (nv15 - 0.0));
        let eq11_e1339_d_b10: f64 = (eq11_e1337_d_b10 * (nv15 - 0.0));
        let eq11_e1339_d_b11: f64 = (eq11_e1337_d_b11 * (nv15 - 0.0));
        let eq11_e1339_d_b12: f64 = (eq11_e1337_d_b12 * (nv15 - 0.0));
        let eq11_e1339_d_b13: f64 = (eq11_e1337_d_b13 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1340_d_b0: f64 = (0.5 * eq11_e1339_d_b0);
        let eq11_e1340_d_b1: f64 = (0.5 * eq11_e1339_d_b1);
        let eq11_e1340_d_b2: f64 = (0.5 * eq11_e1339_d_b2);
        let eq11_e1340_d_b3: f64 = (0.5 * eq11_e1339_d_b3);
        let eq11_e1340_d_b4: f64 = (0.5 * eq11_e1339_d_b4);
        let eq11_e1340_d_b5: f64 = (0.5 * eq11_e1339_d_b5);
        let eq11_e1340_d_b6: f64 = (0.5 * eq11_e1339_d_b6);
        let eq11_e1340_d_b7: f64 = (0.5 * eq11_e1339_d_b7);
        let eq11_e1340_d_b8: f64 = (0.5 * eq11_e1339_d_b8);
        let eq11_e1340_d_b9: f64 = (0.5 * eq11_e1339_d_b9);
        let eq11_e1340_d_b10: f64 = (0.5 * eq11_e1339_d_b10);
        let eq11_e1340_d_b11: f64 = (0.5 * eq11_e1339_d_b11);
        let eq11_e1340_d_b12: f64 = (0.5 * eq11_e1339_d_b12);
        let eq11_e1340_d_b13: f64 = (0.5 * eq11_e1339_d_b13);
        let eq11_e1341: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq11_e1340);
        let eq11_e1341_d_n0: f64 = (eq11_e1340_d_n0 * ddt_scale);
        let eq11_e1341_d_n1: f64 = (eq11_e1340_d_n1 * ddt_scale);
        let eq11_e1341_d_n2: f64 = (eq11_e1340_d_n2 * ddt_scale);
        let eq11_e1341_d_n3: f64 = (eq11_e1340_d_n3 * ddt_scale);
        let eq11_e1341_d_n4: f64 = (eq11_e1340_d_n4 * ddt_scale);
        let eq11_e1341_d_n5: f64 = (eq11_e1340_d_n5 * ddt_scale);
        let eq11_e1341_d_n6: f64 = (eq11_e1340_d_n6 * ddt_scale);
        let eq11_e1341_d_n7: f64 = (eq11_e1340_d_n7 * ddt_scale);
        let eq11_e1341_d_n8: f64 = (eq11_e1340_d_n8 * ddt_scale);
        let eq11_e1341_d_n9: f64 = (eq11_e1340_d_n9 * ddt_scale);
        let eq11_e1341_d_n10: f64 = (eq11_e1340_d_n10 * ddt_scale);
        let eq11_e1341_d_n11: f64 = (eq11_e1340_d_n11 * ddt_scale);
        let eq11_e1341_d_n12: f64 = (eq11_e1340_d_n12 * ddt_scale);
        let eq11_e1341_d_n13: f64 = (eq11_e1340_d_n13 * ddt_scale);
        let eq11_e1341_d_n14: f64 = (eq11_e1340_d_n14 * ddt_scale);
        let eq11_e1341_d_n15: f64 = (eq11_e1340_d_n15 * ddt_scale);
        let eq11_e1341_d_n16: f64 = (eq11_e1340_d_n16 * ddt_scale);
        let eq11_e1341_d_b0: f64 = (eq11_e1340_d_b0 * ddt_scale);
        let eq11_e1341_d_b1: f64 = (eq11_e1340_d_b1 * ddt_scale);
        let eq11_e1341_d_b2: f64 = (eq11_e1340_d_b2 * ddt_scale);
        let eq11_e1341_d_b3: f64 = (eq11_e1340_d_b3 * ddt_scale);
        let eq11_e1341_d_b4: f64 = (eq11_e1340_d_b4 * ddt_scale);
        let eq11_e1341_d_b5: f64 = (eq11_e1340_d_b5 * ddt_scale);
        let eq11_e1341_d_b6: f64 = (eq11_e1340_d_b6 * ddt_scale);
        let eq11_e1341_d_b7: f64 = (eq11_e1340_d_b7 * ddt_scale);
        let eq11_e1341_d_b8: f64 = (eq11_e1340_d_b8 * ddt_scale);
        let eq11_e1341_d_b9: f64 = (eq11_e1340_d_b9 * ddt_scale);
        let eq11_e1341_d_b10: f64 = (eq11_e1340_d_b10 * ddt_scale);
        let eq11_e1341_d_b11: f64 = (eq11_e1340_d_b11 * ddt_scale);
        let eq11_e1341_d_b12: f64 = (eq11_e1340_d_b12 * ddt_scale);
        let eq11_e1341_d_b13: f64 = (eq11_e1340_d_b13 * ddt_scale);
        let eq11_e1342: f64 = (p.p29 * eq11_e1341);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1341_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1341_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1341_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1341_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1341_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1341_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1341_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1341_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1341_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1341_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1341_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1341_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1341_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1341_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1341_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1341_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1341_d_n16);
        let eq11_e1342_d_b0: f64 = (p.p29 * eq11_e1341_d_b0);
        let eq11_e1342_d_b1: f64 = (p.p29 * eq11_e1341_d_b1);
        let eq11_e1342_d_b2: f64 = (p.p29 * eq11_e1341_d_b2);
        let eq11_e1342_d_b3: f64 = (p.p29 * eq11_e1341_d_b3);
        let eq11_e1342_d_b4: f64 = (p.p29 * eq11_e1341_d_b4);
        let eq11_e1342_d_b5: f64 = (p.p29 * eq11_e1341_d_b5);
        let eq11_e1342_d_b6: f64 = (p.p29 * eq11_e1341_d_b6);
        let eq11_e1342_d_b7: f64 = (p.p29 * eq11_e1341_d_b7);
        let eq11_e1342_d_b8: f64 = (p.p29 * eq11_e1341_d_b8);
        let eq11_e1342_d_b9: f64 = (p.p29 * eq11_e1341_d_b9);
        let eq11_e1342_d_b10: f64 = (p.p29 * eq11_e1341_d_b10);
        let eq11_e1342_d_b11: f64 = (p.p29 * eq11_e1341_d_b11);
        let eq11_e1342_d_b12: f64 = (p.p29 * eq11_e1341_d_b12);
        let eq11_e1342_d_b13: f64 = (p.p29 * eq11_e1341_d_b13);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16, eq11_e1342_d_b0, eq11_e1342_d_b1, eq11_e1342_d_b2, eq11_e1342_d_b3, eq11_e1342_d_b4, eq11_e1342_d_b5, eq11_e1342_d_b6, eq11_e1342_d_b7, eq11_e1342_d_b8, eq11_e1342_d_b9, eq11_e1342_d_b10, eq11_e1342_d_b11, eq11_e1342_d_b12, eq11_e1342_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1344;
        let eq11_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16];
        let eq11_branch_derivatives: [f64; 14] = [eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16, eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1353_d_n0: f64 = (-s.dn[57][0]);
        let eq12_e1353_d_n1: f64 = (-s.dn[57][1]);
        let eq12_e1353_d_n2: f64 = (-s.dn[57][2]);
        let eq12_e1353_d_n3: f64 = (-s.dn[57][3]);
        let eq12_e1353_d_n4: f64 = (-s.dn[57][4]);
        let eq12_e1353_d_n5: f64 = (-s.dn[57][5]);
        let eq12_e1353_d_n6: f64 = (-s.dn[57][6]);
        let eq12_e1353_d_n7: f64 = (-s.dn[57][7]);
        let eq12_e1353_d_n8: f64 = (-s.dn[57][8]);
        let eq12_e1353_d_n9: f64 = (-s.dn[57][9]);
        let eq12_e1353_d_n10: f64 = (-s.dn[57][10]);
        let eq12_e1353_d_n11: f64 = (-s.dn[57][11]);
        let eq12_e1353_d_n12: f64 = (-s.dn[57][12]);
        let eq12_e1353_d_n13: f64 = (-s.dn[57][13]);
        let eq12_e1353_d_n14: f64 = (-s.dn[57][14]);
        let eq12_e1353_d_n15: f64 = (-s.dn[57][15]);
        let eq12_e1353_d_n16: f64 = (-s.dn[57][16]);
        let eq12_e1353_d_b0: f64 = (-s.db[57][0]);
        let eq12_e1353_d_b1: f64 = (-s.db[57][1]);
        let eq12_e1353_d_b2: f64 = (-s.db[57][2]);
        let eq12_e1353_d_b3: f64 = (-s.db[57][3]);
        let eq12_e1353_d_b4: f64 = (-s.db[57][4]);
        let eq12_e1353_d_b5: f64 = (-s.db[57][5]);
        let eq12_e1353_d_b6: f64 = (-s.db[57][6]);
        let eq12_e1353_d_b7: f64 = (-s.db[57][7]);
        let eq12_e1353_d_b8: f64 = (-s.db[57][8]);
        let eq12_e1353_d_b9: f64 = (-s.db[57][9]);
        let eq12_e1353_d_b10: f64 = (-s.db[57][10]);
        let eq12_e1353_d_b11: f64 = (-s.db[57][11]);
        let eq12_e1353_d_b12: f64 = (-s.db[57][12]);
        let eq12_e1353_d_b13: f64 = (-s.db[57][13]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = ((eq12_e1353_d_n0 * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = ((eq12_e1353_d_n1 * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = ((eq12_e1353_d_n2 * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = ((eq12_e1353_d_n3 * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = ((eq12_e1353_d_n4 * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = ((eq12_e1353_d_n5 * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = ((eq12_e1353_d_n6 * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = ((eq12_e1353_d_n7 * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = ((eq12_e1353_d_n8 * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = ((eq12_e1353_d_n9 * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = ((eq12_e1353_d_n10 * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = ((eq12_e1353_d_n11 * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = ((eq12_e1353_d_n12 * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = ((eq12_e1353_d_n13 * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = ((eq12_e1353_d_n14 * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = ((eq12_e1353_d_n15 * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = ((eq12_e1353_d_n16 * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1355_d_b0: f64 = ((eq12_e1353_d_b0 * s.v[378]) + (eq12_e1353 * s.db[378][0]));
        let eq12_e1355_d_b1: f64 = ((eq12_e1353_d_b1 * s.v[378]) + (eq12_e1353 * s.db[378][1]));
        let eq12_e1355_d_b2: f64 = ((eq12_e1353_d_b2 * s.v[378]) + (eq12_e1353 * s.db[378][2]));
        let eq12_e1355_d_b3: f64 = ((eq12_e1353_d_b3 * s.v[378]) + (eq12_e1353 * s.db[378][3]));
        let eq12_e1355_d_b4: f64 = ((eq12_e1353_d_b4 * s.v[378]) + (eq12_e1353 * s.db[378][4]));
        let eq12_e1355_d_b5: f64 = ((eq12_e1353_d_b5 * s.v[378]) + (eq12_e1353 * s.db[378][5]));
        let eq12_e1355_d_b6: f64 = ((eq12_e1353_d_b6 * s.v[378]) + (eq12_e1353 * s.db[378][6]));
        let eq12_e1355_d_b7: f64 = ((eq12_e1353_d_b7 * s.v[378]) + (eq12_e1353 * s.db[378][7]));
        let eq12_e1355_d_b8: f64 = ((eq12_e1353_d_b8 * s.v[378]) + (eq12_e1353 * s.db[378][8]));
        let eq12_e1355_d_b9: f64 = ((eq12_e1353_d_b9 * s.v[378]) + (eq12_e1353 * s.db[378][9]));
        let eq12_e1355_d_b10: f64 = ((eq12_e1353_d_b10 * s.v[378]) + (eq12_e1353 * s.db[378][10]));
        let eq12_e1355_d_b11: f64 = ((eq12_e1353_d_b11 * s.v[378]) + (eq12_e1353 * s.db[378][11]));
        let eq12_e1355_d_b12: f64 = ((eq12_e1353_d_b12 * s.v[378]) + (eq12_e1353 * s.db[378][12]));
        let eq12_e1355_d_b13: f64 = ((eq12_e1353_d_b13 * s.v[378]) + (eq12_e1353 * s.db[378][13]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1357_d_b0: f64 = (eq12_e1355_d_b0 * s.v[46]);
        let eq12_e1357_d_b1: f64 = (eq12_e1355_d_b1 * s.v[46]);
        let eq12_e1357_d_b2: f64 = (eq12_e1355_d_b2 * s.v[46]);
        let eq12_e1357_d_b3: f64 = (eq12_e1355_d_b3 * s.v[46]);
        let eq12_e1357_d_b4: f64 = (eq12_e1355_d_b4 * s.v[46]);
        let eq12_e1357_d_b5: f64 = (eq12_e1355_d_b5 * s.v[46]);
        let eq12_e1357_d_b6: f64 = (eq12_e1355_d_b6 * s.v[46]);
        let eq12_e1357_d_b7: f64 = (eq12_e1355_d_b7 * s.v[46]);
        let eq12_e1357_d_b8: f64 = (eq12_e1355_d_b8 * s.v[46]);
        let eq12_e1357_d_b9: f64 = (eq12_e1355_d_b9 * s.v[46]);
        let eq12_e1357_d_b10: f64 = (eq12_e1355_d_b10 * s.v[46]);
        let eq12_e1357_d_b11: f64 = (eq12_e1355_d_b11 * s.v[46]);
        let eq12_e1357_d_b12: f64 = (eq12_e1355_d_b12 * s.v[46]);
        let eq12_e1357_d_b13: f64 = (eq12_e1355_d_b13 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1359_d_b0: f64 = (eq12_e1357_d_b0 * s.v[29]);
        let eq12_e1359_d_b1: f64 = (eq12_e1357_d_b1 * s.v[29]);
        let eq12_e1359_d_b2: f64 = (eq12_e1357_d_b2 * s.v[29]);
        let eq12_e1359_d_b3: f64 = (eq12_e1357_d_b3 * s.v[29]);
        let eq12_e1359_d_b4: f64 = (eq12_e1357_d_b4 * s.v[29]);
        let eq12_e1359_d_b5: f64 = (eq12_e1357_d_b5 * s.v[29]);
        let eq12_e1359_d_b6: f64 = (eq12_e1357_d_b6 * s.v[29]);
        let eq12_e1359_d_b7: f64 = (eq12_e1357_d_b7 * s.v[29]);
        let eq12_e1359_d_b8: f64 = (eq12_e1357_d_b8 * s.v[29]);
        let eq12_e1359_d_b9: f64 = (eq12_e1357_d_b9 * s.v[29]);
        let eq12_e1359_d_b10: f64 = (eq12_e1357_d_b10 * s.v[29]);
        let eq12_e1359_d_b11: f64 = (eq12_e1357_d_b11 * s.v[29]);
        let eq12_e1359_d_b12: f64 = (eq12_e1357_d_b12 * s.v[29]);
        let eq12_e1359_d_b13: f64 = (eq12_e1357_d_b13 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1361_d_b0: f64 = (eq12_e1359_d_b0 * p.p2);
        let eq12_e1361_d_b1: f64 = (eq12_e1359_d_b1 * p.p2);
        let eq12_e1361_d_b2: f64 = (eq12_e1359_d_b2 * p.p2);
        let eq12_e1361_d_b3: f64 = (eq12_e1359_d_b3 * p.p2);
        let eq12_e1361_d_b4: f64 = (eq12_e1359_d_b4 * p.p2);
        let eq12_e1361_d_b5: f64 = (eq12_e1359_d_b5 * p.p2);
        let eq12_e1361_d_b6: f64 = (eq12_e1359_d_b6 * p.p2);
        let eq12_e1361_d_b7: f64 = (eq12_e1359_d_b7 * p.p2);
        let eq12_e1361_d_b8: f64 = (eq12_e1359_d_b8 * p.p2);
        let eq12_e1361_d_b9: f64 = (eq12_e1359_d_b9 * p.p2);
        let eq12_e1361_d_b10: f64 = (eq12_e1359_d_b10 * p.p2);
        let eq12_e1361_d_b11: f64 = (eq12_e1359_d_b11 * p.p2);
        let eq12_e1361_d_b12: f64 = (eq12_e1359_d_b12 * p.p2);
        let eq12_e1361_d_b13: f64 = (eq12_e1359_d_b13 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1363_d_b0: f64 = (eq12_e1361_d_b0 * s.v[30]);
        let eq12_e1363_d_b1: f64 = (eq12_e1361_d_b1 * s.v[30]);
        let eq12_e1363_d_b2: f64 = (eq12_e1361_d_b2 * s.v[30]);
        let eq12_e1363_d_b3: f64 = (eq12_e1361_d_b3 * s.v[30]);
        let eq12_e1363_d_b4: f64 = (eq12_e1361_d_b4 * s.v[30]);
        let eq12_e1363_d_b5: f64 = (eq12_e1361_d_b5 * s.v[30]);
        let eq12_e1363_d_b6: f64 = (eq12_e1361_d_b6 * s.v[30]);
        let eq12_e1363_d_b7: f64 = (eq12_e1361_d_b7 * s.v[30]);
        let eq12_e1363_d_b8: f64 = (eq12_e1361_d_b8 * s.v[30]);
        let eq12_e1363_d_b9: f64 = (eq12_e1361_d_b9 * s.v[30]);
        let eq12_e1363_d_b10: f64 = (eq12_e1361_d_b10 * s.v[30]);
        let eq12_e1363_d_b11: f64 = (eq12_e1361_d_b11 * s.v[30]);
        let eq12_e1363_d_b12: f64 = (eq12_e1361_d_b12 * s.v[30]);
        let eq12_e1363_d_b13: f64 = (eq12_e1361_d_b13 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1365_d_b0: f64 = (eq12_e1363_d_b0 * (nv15 - 0.0));
        let eq12_e1365_d_b1: f64 = (eq12_e1363_d_b1 * (nv15 - 0.0));
        let eq12_e1365_d_b2: f64 = (eq12_e1363_d_b2 * (nv15 - 0.0));
        let eq12_e1365_d_b3: f64 = (eq12_e1363_d_b3 * (nv15 - 0.0));
        let eq12_e1365_d_b4: f64 = (eq12_e1363_d_b4 * (nv15 - 0.0));
        let eq12_e1365_d_b5: f64 = (eq12_e1363_d_b5 * (nv15 - 0.0));
        let eq12_e1365_d_b6: f64 = (eq12_e1363_d_b6 * (nv15 - 0.0));
        let eq12_e1365_d_b7: f64 = (eq12_e1363_d_b7 * (nv15 - 0.0));
        let eq12_e1365_d_b8: f64 = (eq12_e1363_d_b8 * (nv15 - 0.0));
        let eq12_e1365_d_b9: f64 = (eq12_e1363_d_b9 * (nv15 - 0.0));
        let eq12_e1365_d_b10: f64 = (eq12_e1363_d_b10 * (nv15 - 0.0));
        let eq12_e1365_d_b11: f64 = (eq12_e1363_d_b11 * (nv15 - 0.0));
        let eq12_e1365_d_b12: f64 = (eq12_e1363_d_b12 * (nv15 - 0.0));
        let eq12_e1365_d_b13: f64 = (eq12_e1363_d_b13 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1366_d_b0: f64 = (0.5 * eq12_e1365_d_b0);
        let eq12_e1366_d_b1: f64 = (0.5 * eq12_e1365_d_b1);
        let eq12_e1366_d_b2: f64 = (0.5 * eq12_e1365_d_b2);
        let eq12_e1366_d_b3: f64 = (0.5 * eq12_e1365_d_b3);
        let eq12_e1366_d_b4: f64 = (0.5 * eq12_e1365_d_b4);
        let eq12_e1366_d_b5: f64 = (0.5 * eq12_e1365_d_b5);
        let eq12_e1366_d_b6: f64 = (0.5 * eq12_e1365_d_b6);
        let eq12_e1366_d_b7: f64 = (0.5 * eq12_e1365_d_b7);
        let eq12_e1366_d_b8: f64 = (0.5 * eq12_e1365_d_b8);
        let eq12_e1366_d_b9: f64 = (0.5 * eq12_e1365_d_b9);
        let eq12_e1366_d_b10: f64 = (0.5 * eq12_e1365_d_b10);
        let eq12_e1366_d_b11: f64 = (0.5 * eq12_e1365_d_b11);
        let eq12_e1366_d_b12: f64 = (0.5 * eq12_e1365_d_b12);
        let eq12_e1366_d_b13: f64 = (0.5 * eq12_e1365_d_b13);
        let eq12_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq12_e1366);
        let eq12_e1367_d_n0: f64 = (eq12_e1366_d_n0 * ddt_scale);
        let eq12_e1367_d_n1: f64 = (eq12_e1366_d_n1 * ddt_scale);
        let eq12_e1367_d_n2: f64 = (eq12_e1366_d_n2 * ddt_scale);
        let eq12_e1367_d_n3: f64 = (eq12_e1366_d_n3 * ddt_scale);
        let eq12_e1367_d_n4: f64 = (eq12_e1366_d_n4 * ddt_scale);
        let eq12_e1367_d_n5: f64 = (eq12_e1366_d_n5 * ddt_scale);
        let eq12_e1367_d_n6: f64 = (eq12_e1366_d_n6 * ddt_scale);
        let eq12_e1367_d_n7: f64 = (eq12_e1366_d_n7 * ddt_scale);
        let eq12_e1367_d_n8: f64 = (eq12_e1366_d_n8 * ddt_scale);
        let eq12_e1367_d_n9: f64 = (eq12_e1366_d_n9 * ddt_scale);
        let eq12_e1367_d_n10: f64 = (eq12_e1366_d_n10 * ddt_scale);
        let eq12_e1367_d_n11: f64 = (eq12_e1366_d_n11 * ddt_scale);
        let eq12_e1367_d_n12: f64 = (eq12_e1366_d_n12 * ddt_scale);
        let eq12_e1367_d_n13: f64 = (eq12_e1366_d_n13 * ddt_scale);
        let eq12_e1367_d_n14: f64 = (eq12_e1366_d_n14 * ddt_scale);
        let eq12_e1367_d_n15: f64 = (eq12_e1366_d_n15 * ddt_scale);
        let eq12_e1367_d_n16: f64 = (eq12_e1366_d_n16 * ddt_scale);
        let eq12_e1367_d_b0: f64 = (eq12_e1366_d_b0 * ddt_scale);
        let eq12_e1367_d_b1: f64 = (eq12_e1366_d_b1 * ddt_scale);
        let eq12_e1367_d_b2: f64 = (eq12_e1366_d_b2 * ddt_scale);
        let eq12_e1367_d_b3: f64 = (eq12_e1366_d_b3 * ddt_scale);
        let eq12_e1367_d_b4: f64 = (eq12_e1366_d_b4 * ddt_scale);
        let eq12_e1367_d_b5: f64 = (eq12_e1366_d_b5 * ddt_scale);
        let eq12_e1367_d_b6: f64 = (eq12_e1366_d_b6 * ddt_scale);
        let eq12_e1367_d_b7: f64 = (eq12_e1366_d_b7 * ddt_scale);
        let eq12_e1367_d_b8: f64 = (eq12_e1366_d_b8 * ddt_scale);
        let eq12_e1367_d_b9: f64 = (eq12_e1366_d_b9 * ddt_scale);
        let eq12_e1367_d_b10: f64 = (eq12_e1366_d_b10 * ddt_scale);
        let eq12_e1367_d_b11: f64 = (eq12_e1366_d_b11 * ddt_scale);
        let eq12_e1367_d_b12: f64 = (eq12_e1366_d_b12 * ddt_scale);
        let eq12_e1367_d_b13: f64 = (eq12_e1366_d_b13 * ddt_scale);
        let eq12_e1368: f64 = (p.p29 * eq12_e1367);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1367_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1367_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1367_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1367_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1367_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1367_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1367_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1367_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1367_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1367_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1367_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1367_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1367_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1367_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1367_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1367_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1367_d_n16);
        let eq12_e1368_d_b0: f64 = (p.p29 * eq12_e1367_d_b0);
        let eq12_e1368_d_b1: f64 = (p.p29 * eq12_e1367_d_b1);
        let eq12_e1368_d_b2: f64 = (p.p29 * eq12_e1367_d_b2);
        let eq12_e1368_d_b3: f64 = (p.p29 * eq12_e1367_d_b3);
        let eq12_e1368_d_b4: f64 = (p.p29 * eq12_e1367_d_b4);
        let eq12_e1368_d_b5: f64 = (p.p29 * eq12_e1367_d_b5);
        let eq12_e1368_d_b6: f64 = (p.p29 * eq12_e1367_d_b6);
        let eq12_e1368_d_b7: f64 = (p.p29 * eq12_e1367_d_b7);
        let eq12_e1368_d_b8: f64 = (p.p29 * eq12_e1367_d_b8);
        let eq12_e1368_d_b9: f64 = (p.p29 * eq12_e1367_d_b9);
        let eq12_e1368_d_b10: f64 = (p.p29 * eq12_e1367_d_b10);
        let eq12_e1368_d_b11: f64 = (p.p29 * eq12_e1367_d_b11);
        let eq12_e1368_d_b12: f64 = (p.p29 * eq12_e1367_d_b12);
        let eq12_e1368_d_b13: f64 = (p.p29 * eq12_e1367_d_b13);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16, eq12_e1368_d_b0, eq12_e1368_d_b1, eq12_e1368_d_b2, eq12_e1368_d_b3, eq12_e1368_d_b4, eq12_e1368_d_b5, eq12_e1368_d_b6, eq12_e1368_d_b7, eq12_e1368_d_b8, eq12_e1368_d_b9, eq12_e1368_d_b10, eq12_e1368_d_b11, eq12_e1368_d_b12, eq12_e1368_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1370;
        let eq12_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16];
        let eq12_branch_derivatives: [f64; 14] = [eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_value: f64 = (nv16 - 0.0);
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq13_value),
            16,
            multiplicity * (1.0),
        );
        let eq14_value: f64 = (nv15 - 0.0);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq14_value),
            15,
            multiplicity * (1.0),
        );
        let (eq15_e1387,) = {
    if s.b[1560] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e1387;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq15_value),
        );
        let (eq16_e1402,) = {
    if s.b[1560] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1402;
        stamper.stamp_current_const_local(
            Some(9),
            Some(5),
            multiplicity * (eq16_value),
        );
        let (eq17_e1415,) = {
    if s.b[1561] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1415;
        stamper.stamp_current_const_local(
            Some(9),
            Some(11),
            multiplicity * (eq17_value),
        );
        let (eq18_e1426,) = {
    if s.b[1588] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e1426;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq18_value),
        );
        let eq19_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[787]);
        let eq19_e1428_d_n0: f64 = (s.dn[787][0] * ddt_scale);
        let eq19_e1428_d_n1: f64 = (s.dn[787][1] * ddt_scale);
        let eq19_e1428_d_n2: f64 = (s.dn[787][2] * ddt_scale);
        let eq19_e1428_d_n3: f64 = (s.dn[787][3] * ddt_scale);
        let eq19_e1428_d_n4: f64 = (s.dn[787][4] * ddt_scale);
        let eq19_e1428_d_n5: f64 = (s.dn[787][5] * ddt_scale);
        let eq19_e1428_d_n6: f64 = (s.dn[787][6] * ddt_scale);
        let eq19_e1428_d_n7: f64 = (s.dn[787][7] * ddt_scale);
        let eq19_e1428_d_n8: f64 = (s.dn[787][8] * ddt_scale);
        let eq19_e1428_d_n9: f64 = (s.dn[787][9] * ddt_scale);
        let eq19_e1428_d_n10: f64 = (s.dn[787][10] * ddt_scale);
        let eq19_e1428_d_n11: f64 = (s.dn[787][11] * ddt_scale);
        let eq19_e1428_d_n12: f64 = (s.dn[787][12] * ddt_scale);
        let eq19_e1428_d_n13: f64 = (s.dn[787][13] * ddt_scale);
        let eq19_e1428_d_n14: f64 = (s.dn[787][14] * ddt_scale);
        let eq19_e1428_d_n15: f64 = (s.dn[787][15] * ddt_scale);
        let eq19_e1428_d_n16: f64 = (s.dn[787][16] * ddt_scale);
        let eq19_e1428_d_b0: f64 = (s.db[787][0] * ddt_scale);
        let eq19_e1428_d_b1: f64 = (s.db[787][1] * ddt_scale);
        let eq19_e1428_d_b2: f64 = (s.db[787][2] * ddt_scale);
        let eq19_e1428_d_b3: f64 = (s.db[787][3] * ddt_scale);
        let eq19_e1428_d_b4: f64 = (s.db[787][4] * ddt_scale);
        let eq19_e1428_d_b5: f64 = (s.db[787][5] * ddt_scale);
        let eq19_e1428_d_b6: f64 = (s.db[787][6] * ddt_scale);
        let eq19_e1428_d_b7: f64 = (s.db[787][7] * ddt_scale);
        let eq19_e1428_d_b8: f64 = (s.db[787][8] * ddt_scale);
        let eq19_e1428_d_b9: f64 = (s.db[787][9] * ddt_scale);
        let eq19_e1428_d_b10: f64 = (s.db[787][10] * ddt_scale);
        let eq19_e1428_d_b11: f64 = (s.db[787][11] * ddt_scale);
        let eq19_e1428_d_b12: f64 = (s.db[787][12] * ddt_scale);
        let eq19_e1428_d_b13: f64 = (s.db[787][13] * ddt_scale);
        let eq19_value: f64 = eq19_e1428;
        let eq19_node_derivatives: [f64; 17] = [eq19_e1428_d_n0, eq19_e1428_d_n1, eq19_e1428_d_n2, eq19_e1428_d_n3, eq19_e1428_d_n4, eq19_e1428_d_n5, eq19_e1428_d_n6, eq19_e1428_d_n7, eq19_e1428_d_n8, eq19_e1428_d_n9, eq19_e1428_d_n10, eq19_e1428_d_n11, eq19_e1428_d_n12, eq19_e1428_d_n13, eq19_e1428_d_n14, eq19_e1428_d_n15, eq19_e1428_d_n16];
        let eq19_branch_derivatives: [f64; 14] = [eq19_e1428_d_b0, eq19_e1428_d_b1, eq19_e1428_d_b2, eq19_e1428_d_b3, eq19_e1428_d_b4, eq19_e1428_d_b5, eq19_e1428_d_b6, eq19_e1428_d_b7, eq19_e1428_d_b8, eq19_e1428_d_b9, eq19_e1428_d_b10, eq19_e1428_d_b11, eq19_e1428_d_b12, eq19_e1428_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[785]);
        let eq20_e1430_d_n0: f64 = (s.dn[785][0] * ddt_scale);
        let eq20_e1430_d_n1: f64 = (s.dn[785][1] * ddt_scale);
        let eq20_e1430_d_n2: f64 = (s.dn[785][2] * ddt_scale);
        let eq20_e1430_d_n3: f64 = (s.dn[785][3] * ddt_scale);
        let eq20_e1430_d_n4: f64 = (s.dn[785][4] * ddt_scale);
        let eq20_e1430_d_n5: f64 = (s.dn[785][5] * ddt_scale);
        let eq20_e1430_d_n6: f64 = (s.dn[785][6] * ddt_scale);
        let eq20_e1430_d_n7: f64 = (s.dn[785][7] * ddt_scale);
        let eq20_e1430_d_n8: f64 = (s.dn[785][8] * ddt_scale);
        let eq20_e1430_d_n9: f64 = (s.dn[785][9] * ddt_scale);
        let eq20_e1430_d_n10: f64 = (s.dn[785][10] * ddt_scale);
        let eq20_e1430_d_n11: f64 = (s.dn[785][11] * ddt_scale);
        let eq20_e1430_d_n12: f64 = (s.dn[785][12] * ddt_scale);
        let eq20_e1430_d_n13: f64 = (s.dn[785][13] * ddt_scale);
        let eq20_e1430_d_n14: f64 = (s.dn[785][14] * ddt_scale);
        let eq20_e1430_d_n15: f64 = (s.dn[785][15] * ddt_scale);
        let eq20_e1430_d_n16: f64 = (s.dn[785][16] * ddt_scale);
        let eq20_e1430_d_b0: f64 = (s.db[785][0] * ddt_scale);
        let eq20_e1430_d_b1: f64 = (s.db[785][1] * ddt_scale);
        let eq20_e1430_d_b2: f64 = (s.db[785][2] * ddt_scale);
        let eq20_e1430_d_b3: f64 = (s.db[785][3] * ddt_scale);
        let eq20_e1430_d_b4: f64 = (s.db[785][4] * ddt_scale);
        let eq20_e1430_d_b5: f64 = (s.db[785][5] * ddt_scale);
        let eq20_e1430_d_b6: f64 = (s.db[785][6] * ddt_scale);
        let eq20_e1430_d_b7: f64 = (s.db[785][7] * ddt_scale);
        let eq20_e1430_d_b8: f64 = (s.db[785][8] * ddt_scale);
        let eq20_e1430_d_b9: f64 = (s.db[785][9] * ddt_scale);
        let eq20_e1430_d_b10: f64 = (s.db[785][10] * ddt_scale);
        let eq20_e1430_d_b11: f64 = (s.db[785][11] * ddt_scale);
        let eq20_e1430_d_b12: f64 = (s.db[785][12] * ddt_scale);
        let eq20_e1430_d_b13: f64 = (s.db[785][13] * ddt_scale);
        let eq20_value: f64 = eq20_e1430;
        let eq20_node_derivatives: [f64; 17] = [eq20_e1430_d_n0, eq20_e1430_d_n1, eq20_e1430_d_n2, eq20_e1430_d_n3, eq20_e1430_d_n4, eq20_e1430_d_n5, eq20_e1430_d_n6, eq20_e1430_d_n7, eq20_e1430_d_n8, eq20_e1430_d_n9, eq20_e1430_d_n10, eq20_e1430_d_n11, eq20_e1430_d_n12, eq20_e1430_d_n13, eq20_e1430_d_n14, eq20_e1430_d_n15, eq20_e1430_d_n16];
        let eq20_branch_derivatives: [f64; 14] = [eq20_e1430_d_b0, eq20_e1430_d_b1, eq20_e1430_d_b2, eq20_e1430_d_b3, eq20_e1430_d_b4, eq20_e1430_d_b5, eq20_e1430_d_b6, eq20_e1430_d_b7, eq20_e1430_d_b8, eq20_e1430_d_b9, eq20_e1430_d_b10, eq20_e1430_d_b11, eq20_e1430_d_b12, eq20_e1430_d_b13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
    }
}
