#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1333] && (!(((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]))) {s.store_scalar(247, (s.v[7] + ((p.p2 - 1.0) * s.v[9])));}
        if (!((((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]) || s.b[1333])) {s.store_scalar(248, 0.0);s.store_scalar(249, 0.0);s.store_scalar(246, 0.0);s.store_scalar(247, 0.0);}
        s.b[1334] = param_given[24];s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if s.b[1334] {s.store_scalar(250, ((p.p24 * p.p53) * p.p52));}
        if (!s.b[1334]) {s.copy_ad(250, 246);}
        s.b[1335] = (s.v[250] < 0.0);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if s.b[1335] {s.store_scalar(250, 0.0);}
        s.b[1336] = param_given[25];s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if s.b[1336] {s.store_scalar(251, ((p.p25 * p.p53) * p.p52));}
        if (!s.b[1336]) {s.copy_ad(251, 247);}
        s.b[1337] = (s.v[251] < 0.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if s.b[1337] {s.store_scalar(251, 0.0);}
        s.b[1338] = param_given[26];s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });s.b[1339] = (p.p137 == 0.0);s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if (s.b[1338] && s.b[1339]) {s.store_scalar(300, (p.p26 * p.p53));}
        if (s.b[1338] && (!s.b[1339])) {s.store_scalar(300, (((p.p26 * p.p53) - (s.v[35] * p.p2))).max(0.0));}
        if (!s.b[1338]) {s.copy_ad(300, 248);}
        s.b[1340] = (s.v[300] < 0.0);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if ((!s.b[1338]) && s.b[1340]) {s.store_scalar(300, 0.0);}
        s.b[1341] = param_given[27];s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });s.b[1342] = (p.p137 == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1342]) {s.store_scalar(301, (p.p27 * p.p53));}
        if (s.b[1341] && (!s.b[1342])) {s.store_scalar(301, (((p.p27 * p.p53) - (s.v[35] * p.p2))).max(0.0));}
        if (!s.b[1341]) {s.copy_ad(301, 249);}
        s.b[1343] = (s.v[301] < 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((!s.b[1341]) && s.b[1343]) {s.store_scalar(301, 0.0);}
        s.store_add_scaled_inputs_mixed_ai(341, A::add_scaled_products(s.ad_value(250), s.ad_value(435), 1.0, s.ad_value(300), s.ad_value(436), 1.0), 1.0, 437, (s.v[35] * p.p2));s.b[1344] = (s.v[341] > 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if s.b[1344] {s.store_scale(343, 393, p.p725);s.store_scaled_limited_exp_ad(351, A::div_from_scalar((-p.p731), s.ad_value(343)), p.p733);s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p727, s.ad_value(341)), 10.0);s.store_sub_offset_lhs(25, 14, 1.0, 351);s.store_mul_ln_mixed_ia(350, 343, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(351), 4.0)), 0.5), 1e-38));s.store_limited_exp_div(12, 350, 343);s.store_mul_scale_offset_mixed_ia(349, 341, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(351), s.ad_value(12)), (-1.0), s.ad_value(351), 1.0), 1.0, (-1.0));s.store_div_scaled_product_mixed_iai(348, 341, A::add(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12))), 1.0, 343, 1.0);}
        if s.b[1344] {
            s.store_offset_ad(14, {
                if (!(((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1344] {s.store_sub_from_scalar_scaled_mul_mixed_ia(347, (-p.p731), 343, A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p733)), 1e-38)), 1.0);s.store_scale_ad(13, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(347), p.p731), -1.0, s.ad_value(343), 1.0), p.p733);s.store_mul_scale_offset_indices(346, 341, 13, 1.0, 1.0);s.store_div_scaled_product_indices(345, 341, 13, -1.0, 343, 1.0);}
        if (!s.b[1344]) {s.store_scalar(343, 0.0);s.store_scalar(351, 0.0);s.store_scalar(350, 0.0);s.store_scalar(349, 0.0);s.store_scalar(348, 0.0);s.store_scalar(347, 0.0);s.store_scalar(346, 0.0);s.store_scalar(345, 0.0);}
        s.store_add_scaled_inputs_mixed_ai(342, A::add_scaled_products(s.ad_value(251), s.ad_value(438), 1.0, s.ad_value(301), s.ad_value(439), 1.0), 1.0, 440, (s.v[35] * p.p2));s.b[1345] = (s.v[342] > 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if s.b[1345] {s.store_scale(344, 393, p.p726);s.store_scaled_limited_exp_ad(358, A::div_from_scalar((-p.p732), s.ad_value(344)), p.p734);s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p728, s.ad_value(342)), 10.0);s.store_sub_offset_lhs(25, 14, 1.0, 358);s.store_mul_ln_mixed_ia(357, 344, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(358), 4.0)), 0.5), 1e-38));s.store_limited_exp_div(12, 357, 344);s.store_mul_scale_offset_mixed_ia(356, 342, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(358), s.ad_value(12)), (-1.0), s.ad_value(358), 1.0), 1.0, (-1.0));s.store_div_scaled_product_mixed_iai(355, 342, A::add(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12))), 1.0, 344, 1.0);}
        if s.b[1345] {
            s.store_offset_ad(14, {
                if (!(((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if s.b[1345] {s.store_sub_from_scalar_scaled_mul_mixed_ia(354, (-p.p732), 344, A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p734)), 1e-38)), 1.0);s.store_scale_ad(13, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(354), p.p732), -1.0, s.ad_value(344), 1.0), p.p734);s.store_mul_scale_offset_indices(353, 342, 13, 1.0, 1.0);s.store_div_scaled_product_indices(352, 342, 13, -1.0, 344, 1.0);}
        if (!s.b[1345]) {s.store_scalar(344, 0.0);s.store_scalar(358, 0.0);s.store_scalar(357, 0.0);s.store_scalar(356, 0.0);s.store_scalar(355, 0.0);s.store_scalar(354, 0.0);s.store_scalar(353, 0.0);s.store_scalar(352, 0.0);}
        s.b[1346] = (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0))));s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1346] {s.store_scalar(12, ((s.v[98]) as f64).powf(p.p921));s.store_scalar(643, (s.v[100] + p.p914));s.store_powf(13, 643, p.p922);s.store_add_scaled_inputs3(644, A::div_from_scalar(p.p918, s.ad_value(12)), 1.0, A::div_from_scalar(p.p919, s.ad_value(13)), 1.0, A::div_from_scalar(p.p920, A::mul(s.ad_value(12), s.ad_value(13))), 1.0);s.store_offset(645, 644, 1.0);s.store_scalar(12, ((s.v[98]) as f64).powf(p.p927));s.store_powf(13, 643, p.p928);s.store_add_scaled_inputs3(646, A::div_from_scalar(p.p924, s.ad_value(12)), 1.0, A::div_from_scalar(p.p925, s.ad_value(13)), 1.0, A::div_from_scalar(p.p926, A::mul(s.ad_value(12), s.ad_value(13))), 1.0);s.store_offset(647, 646, 1.0);s.store_offset(12, 395, (-1.0));s.store_offset_mul_ad(648, s.ad_value(645), A::scale_offset(s.ad_value(12), p.p917, 1.0), 1e-9);s.store_scalar(662, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[1346] && (s.v[662] < p.p2)) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1346] {s.store_div_from_scalar_offset_scaled_input(12, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p17 + (0.5 * s.v[99])));s.store_div_from_scalar_offset_scaled_input(13, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p18 + (0.5 * s.v[99])));s.store_offset(649, 12, s.v[649]);s.store_offset(650, 13, s.v[650]);s.store_primal_offset(662, 662, 1.0);}
        }
        if s.b[1346] {s.store_scalar(651, (1.0 / (p.p912 + (0.5 * s.v[99]))));s.store_scalar(652, (1.0 / (p.p913 + (0.5 * s.v[99]))));s.store_primal_add(653, 651, 652);s.store_mul_div_from_scalar_lhs_ad_indices(654, p.p915, 648, 653);s.store_add(655, 649, 650);s.store_mul_div_from_scalar_lhs_ad_indices(656, p.p915, 648, 655);s.store_div_scaled_offset_numerator_mixed_ia(657, 656, 1.0, 1.0, A::offset(s.ad_value(654), 1.0), 1.0);s.store_div_scaled_offset_numerator_mixed_ia(658, 656, p.p916, 1.0, A::scale_offset(s.ad_value(654), p.p916, 1.0), 1.0);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(659, p.p923, 647, A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul_div_from_scalar_lhs_ad(660, p.p929, A::powf(s.ad_value(647), p.p930), A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul_div_from_scalar_lhs_ad(661, p.p931, A::powf(s.ad_value(647), p.p932), A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul(397, 397, 657);s.store_mul(409, 409, 658);s.store_add(494, 494, 660);s.store_add(420, 420, 661);}
        s.b[1347] = (p.p37 == 1.0);s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if (s.b[1346] && s.b[1347]) {s.store_mul_div_scaled_inputs_mixed_aii(688, A::sub(s.ad_value(655), s.ad_value(653)), 625, 1.0, 647, 1.0);s.store_mul_div_scaled_inputs_mixed_aia(689, A::sub(s.ad_value(655), s.ad_value(653)), 626, 1.0, A::powf(s.ad_value(647), p.p930), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1346] && s.b[1347]) {s.store_mul_div_scaled_inputs_mixed_aia(690, A::sub(s.ad_value(655), s.ad_value(653)), 627, 1.0, A::powf(s.ad_value(647), p.p932), 1.0);}
        if s.b[1346] {s.store_add(624, 624, 689);s.store_add(616, 616, 690);}
        if (!s.b[1346]) {s.store_scalar(659, 0.0);s.store_scalar(688, 0.0);}
        s.b[1348] = (p.p43 == 1.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if s.b[1348] {s.store_scalar(668, (p.p1 / p.p2));s.store_scalar(669, p.p20);s.store_scalar(670, p.p21);s.store_scalar(671, p.p22);}
        s.b[1349] = (((!param_given[20]) && (!param_given[21])) && (!param_given[22]));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });s.b[1350] = (param_given[23] && (p.p23 > 0.0));s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1349]) && s.b[1350]) {s.store_offset(13, 668, p.p23);s.store_scalar(14, (1.0 / p.p947));s.store_div_from_scalar_scaled_input(669, (p.p947 * p.p947), 13, p.p23);s.store_div_scaled_add_product_mixed_aaai(670, A::limited_exp_scaled_input(s.ad_value(14), ((-10.0) * p.p23)), ((0.1 * p.p23) + (0.01 * p.p947)), A::scale_offset(s.ad_value(13), 0.1, (0.01 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-10.0), s.ad_value(14))), (-1.0), 668, 1.0);s.store_div_scaled_add_product_mixed_aaai(671, A::limited_exp_scaled_input(s.ad_value(14), ((-20.0) * p.p23)), ((0.05 * p.p23) + (0.0025 * p.p947)), A::scale_offset(s.ad_value(13), 0.05, (0.0025 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-20.0), s.ad_value(14))), (-1.0), 668, 1.0);}
        s.store_mul_add_scaled_inputs3_offset_rhs_indices(663, 578, 669, 1.0, 670, p.p933, 671, p.p934, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(664, 579, 669, 1.0, 670, p.p933, 671, p.p934, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(665, 630, 669, 1.0, 670, p.p933, 671, p.p934, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(666, 629, 669, 1.0, 670, p.p933, 671, p.p934, 0.0);s.store_offset_mul_ad(667, s.ad_value(580), A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934), 1.0);s.store_mul(397, 397, 667);s.store_add(494, 494, 664);s.store_mul_voltage_ad(64, s.ad_value(187), ctx, nodes, Some(9), Some(11));s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(5), Some(11));s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(7), Some(11));s.store_sub(74, 66, 70);s.copy_ad(68, 66);s.copy_ad(56, 74);s.copy_ad(50, 70);s.copy_ad(48, 66);s.store_mul_voltage_ad(306, s.ad_value(187), ctx, nodes, Some(12), Some(7));s.store_mul_voltage_ad(307, s.ad_value(187), ctx, nodes, Some(13), Some(5));s.store_mul_voltage_ad(308, s.ad_value(187), ctx, nodes, Some(13), Some(5));s.store_mul_voltage_ad(309, s.ad_value(187), ctx, nodes, Some(13), Some(14));s.store_sub(54, 64, 66);s.store_sub(52, 64, 70);s.store_mul_voltage_ad(230, s.ad_value(187), ctx, nodes, Some(10), Some(5));s.store_mul_voltage_ad(231, s.ad_value(187), ctx, nodes, Some(10), Some(7));s.copy_ad(232, 230);s.b[1351] = ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if s.b[1351] {s.store_add_scaled_product_mixed_iia(68, 66, 1.0, 187, A::voltage(ctx, nodes, Some(6), Some(5)), (1.0 - (p.p1111 / p.p1110)));s.store_add_scaled_inputs3_indices(308, 307, 1.0, 66, 1.0, 68, -1.0);s.store_add_scaled_inputs3_indices(232, 230, 1.0, 66, 1.0, 68, -1.0);}
        s.copy_ad(69, 68);s.store_mul_voltage_ad(72, s.ad_value(187), ctx, nodes, Some(7), Some(11));s.store_scalar(57, 1.0);s.b[1352] = (s.v[74] < 0.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if s.b[1352] {s.store_scalar(57, (-1.0));s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(7), Some(11));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1352] {s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(5), Some(11));s.copy_ad(72, 69);s.store_mul_voltage_ad(68, s.ad_value(187), ctx, nodes, Some(7), Some(11));}
        s.store_sub(74, 66, 70);s.store_sub(75, 68, 72);s.store_scale(12, 75, p.p956);
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
        s.store_offset_sub_scaled_inputs_indices(76, 13, (2.0 / p.p956), 75, 1.0, (-((2.0 / p.p956) * ((2.0) as f64).ln())));s.store_add_scaled_inputs3_indices(62, 72, (-1.0), 75, (-0.5), 76, (-(-0.5)));s.store_scale(12, 74, p.p956);
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
        s.store_offset_sub_scaled_inputs_indices(76, 13, (2.0 / p.p956), 74, 1.0, (-((2.0 / p.p956) * ((2.0) as f64).ln())));s.store_add_scaled_inputs3_indices(61, 70, (-1.0), 74, (-0.5), 76, (-(-0.5)));s.store_tanh_ad(12, A::div_scaled_inputs(s.ad_value(56), p.p1123, s.ad_value(393), 1.0));s.store_offset_scaled(102, 12, 0.5, 0.5);s.store_sub_from_scalar(103, 1.0, 102);s.b[1353] = (p.p44 != 0.0);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if s.b[1353] {s.store_add_scaled_products_indices(486, 485, 103, 1.0, 484, 102, 1.0);s.store_add_scaled_products_indices(492, 421, 103, 1.0, 420, 102, 1.0);s.store_add_scaled_products_indices(519, 518, 103, 1.0, 517, 102, 1.0);s.store_add_scaled_products_indices(541, 540, 103, 1.0, 539, 102, 1.0);s.store_add_scaled_products_indices(166, 165, 103, 1.0, 164, 102, 1.0);s.store_add_scaled_products_indices(502, 410, 103, 1.0, 409, 102, 1.0);s.store_add_scaled_products_indices(536, 414, 103, 1.0, 413, 102, 1.0);s.store_add_scaled_products_indices(499, 398, 103, 1.0, 397, 102, 1.0);s.store_add_scaled_products_indices(506, 400, 103, 1.0, 399, 102, 1.0);s.store_add_scaled_products_indices(516, 402, 103, 1.0, 401, 102, 1.0);s.store_add_scaled_products_indices(510, 404, 103, 1.0, 403, 102, 1.0);s.store_add_scaled_products_indices(513, 406, 103, 1.0, 405, 102, 1.0);s.store_add_scaled_products_indices(553, 552, 103, 1.0, 551, 102, 1.0);s.store_add_scaled_products_indices(558, 416, 103, 1.0, 415, 102, 1.0);}
        if (!s.b[1353]) {s.copy_ad(486, 484);s.copy_ad(492, 420);s.copy_ad(519, 517);s.copy_ad(541, 539);s.copy_ad(166, 164);s.copy_ad(502, 409);s.copy_ad(536, 413);s.copy_ad(499, 397);s.copy_ad(506, 399);s.copy_ad(516, 401);s.copy_ad(510, 403);s.copy_ad(513, 405);s.copy_ad(553, 551);s.copy_ad(558, 415);}
        s.b[1354] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if s.b[1354] {s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));}
        if (!s.b[1354]) {s.store_add_scaled_inputs3_offset_mixed_iia(110, 127, 0.5, 61, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));}
        s.store_sqrt(111, 110);s.store_mul(112, 114, 111);s.store_div_from_scalar(97, s.v[26], 112);s.store_add_scaled_inputs_products_indices(113, 483, 1.0, 422, 1.0, 486, 76, 1.0, 487, 61, (-1.0));s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);s.b[1355] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if s.b[1355] {s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);}
        if (!s.b[1355]) {s.store_scaled_add_offset_sqrt_square_offset(104, 13, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);}
        s.store_mul(106, 104, 108);s.store_div_from_scalar(107, 1.0, 106);s.store_mul_scale_offset_mixed_ia(123, 76, A::add_scaled_product(s.ad_value(492), 1.0, s.ad_value(493), s.ad_value(61), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_sub_scaled_inputs_mixed_ia(123, 123, 0.5, A::sqrt_square_offset(s.ad_value(123), ((0.25 * 0.005) * 0.005)), 0.5, (0.25 * 0.005));s.store_mul_scale_offset(124, A::add_scaled_product(A::offset(s.ad_value(454), (p.p869 / s.v[30])), 1.0, s.ad_value(455), s.ad_value(61), 1.0), A::powf(s.ad_value(395), p.p868), 1.0, (-1.0));s.b[1356] = (s.v[116] > 0.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if s.b[1356] {s.store_mul_scale_offset_indices(12, 76, 117, -1.0, 0.0);}
        s.b[1357] = (s.v[12] < (-80.0));s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if (s.b[1356] && s.b[1357]) {s.store_scalar(14, 1.804851387e-35);}
        if (s.b[1356] && (!s.b[1357])) {s.store_limited_exp(14, 12);}
        if s.b[1356] {s.store_offset_mul_offset_rhs(15, 116, 14, 1.0, s.v[30]);s.store_mul_scaled_ln_ad_rhs(115, 106, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[30], s.ad_value(15)), 1e-38));}
        if (!s.b[1356]) {s.store_scalar(115, 0.0);}
        s.store_add_div_rhs_mixed_ia(16, 121, 118, A::pow_from_scalar(s.v[30], s.ad_value(119)));s.store_add_scaled_product_mixed_iia(115, 115, 1.0, 16, A::tanh(A::mul(s.ad_value(120), s.ad_value(76))), (-1.0));s.store_offset(482, 482, p.p35);s.store_mul(65, 64, 107);s.store_mul(73, 70, 107);s.store_mul(58, 482, 107);s.store_add_scaled_products_mixed_iaii(122, 495, A::sub(s.ad_value(111), s.ad_value(128)), 1.0, 494, 61, (-1.0));s.store_add_mixed_ai(79, A::add(A::add_scaled_inputs4(s.ad_value(123), 1.0, s.ad_value(115), 1.0, s.ad_value(122), 1.0, s.ad_value(124), -1.0), s.ad_value(659)), 663);s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));s.store_scaled_sqrt_mul_scaled_lhs(125, 481, ((2.0 * 1.60219e-19) * s.v[26]), 109, 1.0 / (s.v[46]));
        if (!(((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_sqrt_square_offset_ad(12, A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), ((4.0 * 0.001) * 0.001), 0.5);
        } else {
            if (((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(12, ((-0.001) * 0.001), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0));
            } else {
                s.store_scalar(12, 0.0);
            }
        }
        s.store_offset_div_scaled_inputs_sqrt_rhs(90, 125, 1.0, 12, 2.0, 1.0);s.store_scaled_sqrt_mul_scaled_lhs(125, 481, ((2.0 * 1.60219e-19) * s.v[26]), 107, 1.0 / (s.v[46]));s.store_div_from_scalar(126, 1.0, 125);s.store_div(89, 88, 104);s.store_scalar(13, 1.0);s.store_scale(204, 59, 1.0 / (s.v[13]));s.store_scale(205, 125, 1.0 / (s.v[13]));s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));s.b[1358] = (s.v[204] < 0.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if s.b[1358] {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1358] {s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (!s.b[1358]) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 125, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 89, (-2.0), 73, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);s.b[1359] = (s.v[20] <= (-68.0));s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if s.b[1359] {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1360] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (s.b[1359] && s.b[1360]) {s.store_limited_exp(15, 16);}
        s.b[1361] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if ((s.b[1359] && (!s.b[1360])) && s.b[1361]) {s.store_limited_exp(15, 20);}
        if ((s.b[1359] && (!s.b[1360])) && (!s.b[1361])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if s.b[1359] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(200, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (!s.b[1359]) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1359]) {s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1362] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if s.b[1362] {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (!s.b[1362]) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        s.store_sqrt(96, 93);s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);s.b[1363] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if s.b[1363] {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (!s.b[1363]) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);s.store_scalar(155, (1e-8 / (s.v[47] * p.p77)));s.store_mul_mixed_ia(12, 106, A::add_scaled_inputs_product(s.ad_value(59), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));s.b[1364] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (!s.b[1364]) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        s.store_mul3_affine_lhs(130, 90, 106, 2.0, 0.0, 200);s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(130), s.ad_value(84)), 1.0, 0.5), s.ad_value(513));s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(132), s.ad_value(407)), 1.0);s.store_offset(16, 15, 1.0);s.b[1365] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if s.b[1365] {s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);}
        if (!s.b[1365]) {s.store_scaled_add_offset_sqrt_square_offset(133, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        s.store_primal_div_from_scalar_scaled_ad(235, 1.0, A::pow_from_scalar((s.v[29] * 1000000.0), s.ad_value(527)), p.p2);s.b[1366] = (p.p42 == 1.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(243, 0.0);}
        if (!s.b[1366]) {s.store_offset_mul(12, 526, 130, 1.0);s.store_mul_sub_rhs(13, 543, 111, 128);s.store_add_mixed_ai(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);s.store_add_mixed_ia(15, 14, A::sqrt_square_offset(s.ad_value(14), 0.01));}
        s.b[1367] = (p.p42 == 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1367]) {s.store_mul_ad_affine_product_lhs(243, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2, 0.0, 408);}
        if ((!s.b[1366]) && (!s.b[1367])) {s.store_mul_add_mixed_iai(243, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), 240);}
        s.store_pow_ad(12, s.ad_value(133), A::div_from_scalar(1.0, s.ad_value(166)));s.store_mul(23, 453, 61);s.store_sqrt_square_offset(24, 23, 0.1);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);s.store_div_scaled_product_offset_denominator_mixed_iia(14, 200, 13, (10.0 * p.p433), A::mul(s.ad_value(200), s.ad_value(13)), (10.0 * p.p433), 1.0);s.b[1368] = (s.v[536] < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if s.b[1368] {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);}
        if (!s.b[1368]) {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);}
        s.b[1369] = (s.v[243] > 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if s.b[1369] {s.store_mul3_affine_lhs(23, 90, 106, ((s.v[29] * 2.0) * s.v[46]), 0.0, 502);s.store_div_scaled_product3_indices(24, 23, 138, 243, 1.0, 106, 2.0);s.store_div_scaled_product_offset_denominator_mixed_iaa(12, 138, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1370] = (s.v[13] != 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1370]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if (s.b[1369] && (!s.b[1370])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if s.b[1369] {s.store_add_scaled_value_products_mixed_aiiia(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1371] = (s.v[13] != 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1371]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if (s.b[1369] && (!s.b[1371])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if s.b[1369] {s.store_add_scaled_value_products3_mixed_iiiiaia(18, 15, 1.0, 12, 17, 1.0, 24, A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(12, 12, 16, 18);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        s.b[1372] = (s.v[13] != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1372]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if (s.b[1369] && (!s.b[1372])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if s.b[1369] {s.store_add_scaled_value_products_mixed_aiiia(16, A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1373] = (s.v[13] != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (s.b[1369] && s.b[1373]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if (s.b[1369] && (!s.b[1373])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if s.b[1369] {s.store_add_scaled_value_products3_mixed_iiiiaia(18, 15, 1.0, 12, 17, 1.0, 24, A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(131, 12, 16, 18);}
        if (!s.b[1369]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(12, 138, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 0.5, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0, 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1374] = (s.v[13] != 0.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1374]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if ((!s.b[1369]) && (!s.b[1374])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
        if (!s.b[1369]) {s.store_add_scaled_products_mixed_iiia(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1375] = (s.v[13] != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1375]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if ((!s.b[1369]) && (!s.b[1375])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if (!s.b[1369]) {s.store_add_scaled_value_products_mixed_iiiia(18, 15, 1.0, 12, 17, 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(12, 12, 16, 18);s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 12, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);}
        s.b[1376] = (s.v[13] != 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1376]) {s.store_asinh(147, 13);s.store_add_scaled_product_mixed_iai(15, 14, 1.0, A::div_from_scalar(1.0, s.ad_value(13)), 147, 1.0);}
        if ((!s.b[1369]) && (!s.b[1376])) {s.store_add_div_from_scalar_rhs(15, 14, 1.0, 14);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1369]) {s.store_add_scaled_products_mixed_iiia(16, 12, 15, 1.0, 138, A::add_scaled_inputs4(A::square(s.ad_value(200)), 1.0, s.ad_value(200), 1.0, A::square(s.ad_value(12)), -1.0, s.ad_value(12), -1.0), (-1.0));}
        s.b[1377] = (s.v[13] != 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((!s.b[1369]) && s.b[1377]) {s.store_div_scaled_product_mixed_iaa(17, 138, A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0), (-2.0), A::square(s.ad_value(13)), 1.0);}
        if ((!s.b[1369]) && (!s.b[1377])) {s.store_mul_div_scaled_inputs_indices(17, 138, 13, (-2.0), 14, 1.0);}
        if (!s.b[1369]) {s.store_add_scaled_value_products_mixed_iiiia(18, 15, 1.0, 12, 17, 1.0, 138, A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0);s.store_sub_div_rhs_indices(131, 12, 16, 18);}
        s.store_add_scaled_inputs4_mixed_iiia(143, 91, 1.0, 89, (-2.0), 131, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(125), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(136, 143, 106);s.b[1378] = ((p.p1130 == 0.0) && (p.p1131 == 0.0));s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if s.b[1378] {s.store_scalar(782, 1.0);}
        if (!s.b[1378]) {s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);s.store_offset_div_scaled_inputs2_mixed_iaa(782, 13, p.p1130, A::mul3_scaled_output(s.ad_value(13), A::powf(s.ad_value(200), p.p1132), s.ad_value(106), p.p1131), (-1.0), A::scale_offset(s.ad_value(61), p.p1133, 1.0), 1.0, 1.0);}
        s.b[1379] = ((0.1 == 0.0) && (s.v[782] < ((-2500.0) * 0.0005)));s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((!s.b[1378]) && s.b[1379]) {s.store_div_from_scalar_scaled_input(782, ((-0.0005) * 0.0005), 782, 16.0);}
        if ((!s.b[1378]) && (!s.b[1379])) {s.store_scaled_add_offset_sqrt_square_offset(782, 782, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        s.b[1380] = ((0.0 == 0.0) && ((s.v[136] - s.v[70]) < ((-2500.0) * 0.001)));s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if s.b[1380] {s.store_div_from_scalar_ad(140, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(136), 16.0, s.ad_value(70), 16.0));}
        if (!s.b[1380]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(140, 136, 0.5, 70, ((-1.0) * 0.5), 136, 70, ((0.25 * 0.001) * 0.001), 0.5);}
        s.store_div(140, 140, 782);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(140)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 74, 20);s.store_mul_add_lhs(142, 139, 70, 107);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 125, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 89, (-2.0), 142, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);s.b[1381] = (s.v[20] <= (-68.0));s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if s.b[1381] {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1382] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (s.b[1381] && s.b[1382]) {s.store_limited_exp(15, 16);}
        s.b[1383] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((s.b[1381] && (!s.b[1382])) && s.b[1383]) {s.store_limited_exp(15, 20);}
        if ((s.b[1381] && (!s.b[1382])) && (!s.b[1383])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if s.b[1381] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(144, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (!s.b[1381]) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1381]) {s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 200, (-1.0), 144, -1.0, (-1.0));s.b[1384] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if s.b[1384] {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (!s.b[1384]) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        s.store_sqrt(14, 12);s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), s.ad_value(14)), 1.0);s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));s.store_div_from_scalar_add_ad(12, 1.0, A::offset(s.ad_value(200), 1.0), s.ad_value(144));s.store_mul(13, 217, 12);s.store_add_scaled_inputs_product_mixed_iiaa(189, 59, 1.0, 91, (-1.0), A::offset(s.ad_value(90), (-1.0)), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(13), 0.3333333333333333), (-1.0));s.store_scale(14, 90, 0.3333333333333333);s.store_mul(15, 13, 12);s.store_mul_mixed_ia(190, 14, A::add_scaled_inputs_product(s.ad_value(200), 2.0, s.ad_value(144), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 0.8, 1.0), 1.0, s.ad_value(144), 1.2), s.ad_value(15), 0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        s.store_mul_mixed_ia(193, 14, A::add_scaled_inputs_product(s.ad_value(200), 1.0, s.ad_value(144), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 1.2, 1.0), 1.0, s.ad_value(144), 0.8), s.ad_value(15), 0.5));s.b[1385] = ((0.0 == 0.0) && ((s.v[106] * s.v[189]) < ((-2500.0) * 0.1)));s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if s.b[1385] {s.store_div_scalar_by_product_indices(81, ((-0.1) * 0.1), 106, 189, 16.0);}
        if (!s.b[1385]) {s.store_add_scaled_product_mixed_aii(81, A::sqrt_square_offset(A::mul(s.ad_value(106), s.ad_value(189)), ((0.25 * 0.1) * 0.1)), 0.5, 106, 189, 0.5);}
        s.store_mul_add_rhs(80, 106, 190, 193);s.store_add_scaled_inputs(156, 81, s.v[155], 80, (s.v[158] * s.v[155]));s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(80), s.ad_value(81)), 1.0, 0.5), s.ad_value(513));s.store_add_scaled_product(15, A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(156), s.ad_value(407)), 1.0);s.store_offset(16, 15, 1.0);s.b[1386] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if s.b[1386] {s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);}
        if (!s.b[1386]) {s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        s.store_div_scaled_inputs_mixed_ia(134, 502, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);s.store_scale(135, 134, s.v[30]);s.b[1387] = (s.v[537] > 0.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if s.b[1387] {s.store_offset_div_scaled_product_indices(172, 537, 80, 1.0, 135, 1.0, 1.0);}
        if (!s.b[1387]) {s.store_div_from_scalar_sub_from_scalar_ad(172, 1.0, 1.0, A::div_scaled_product(s.ad_value(537), s.ad_value(80), 1.0, s.ad_value(135), 1.0));}
        s.copy_ad(171, 519);s.store_sub(167, 74, 139);s.store_add_scaled_inputs(174, 80, 1.0, 106, 2.0);s.b[1388] = (s.v[171] > 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if s.b[1388] {s.store_div_add_scaled_inputs_rhs_indices(15, 174, 140, 1.0, 174, 1.0);}
        if s.b[1388] {
            if (!((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(16, A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_product(16, ((-0.001) * 0.001), 520, 61, 1.0);
                } else {
                    s.store_scalar(16, 0.0);
                }
            }
        }
        if s.b[1388] {s.store_div_from_scalar(17, 1.0, 16);s.store_mul_product3_mixed_iaii(173, 17, A::div(s.ad_value(174), s.ad_value(171)), 15, 172, 1.0);s.store_offset_div(175, 167, 173, 1.0);}
        if (!s.b[1388]) {s.store_scalar(175, 1.0);}
        s.b[1389] = (s.v[525] <= 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1389] {s.store_scalar(105, 1.0);}
        if (!s.b[1389]) {s.store_div_scaled_inputs_indices(21, 525, ((s.v[30]) as f64).sqrt(), 174, 1.0);s.store_div_from_scalar_offset_input(105, 1.0, 21, 1.0);}
        s.store_add(170, 140, 135);s.b[1390] = (s.v[541] > 0.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (p.p350 < 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1391]) {s.store_div_scaled_value_by_product_mixed_iai(13, 541, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), 105, 1.0);}
        if (s.b[1390] && (!s.b[1391])) {s.store_div_scaled_product_offset_rhs_mixed_iai(13, 541, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, 105, 1.0);}
        if s.b[1390] {s.store_offset_mul_ad(176, s.ad_value(13), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(13), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);}
        s.b[1392] = (p.p350 < 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if ((!s.b[1390]) && s.b[1392]) {s.store_div_scaled_value_by_product_mixed_iai(13, 541, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0)), 105, 1.0);}
        if ((!s.b[1390]) && (!s.b[1392])) {s.store_div_scaled_product_offset_rhs_mixed_iai(13, 541, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0, 1.0, 105, 1.0);}
        if (!s.b[1390]) {s.store_offset(176, 13, 1.0);}
        s.store_mul(175, 175, 176);s.store_limited_exp_mul(13, 524, 74);s.b[1393] = (s.v[523] > 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if s.b[1393] {s.store_scalar(14, (1.0 + (p.p369 * s.v[30])));s.store_div_scaled_offset_numerator_mixed_ai(168, A::mul(s.ad_value(14), s.ad_value(13)), 1.0, 1.0, 523, 1.0);s.store_mul(168, 168, 105);}
        if (!s.b[1393]) {s.store_scalar(168, 5.540622384e34);}
        s.store_div(16, 167, 168);s.store_offset(12, 16, 1.0);s.store_mul(175, 175, 12);s.b[1394] = (s.v[522] > 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });s.b[1395] = (s.v[167] > ((s.v[521] * s.v[129]) / 80.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (s.b[1394] && s.b[1395]) {s.store_div_scaled_product_indices(12, 521, 129, 1.0, 167, 1.0);s.store_div_scaled_inputs_limited_exp_lhs(169, 12, s.v[30], 522, 1.0);}
        if (s.b[1394] && (!s.b[1395])) {s.store_div_from_scalar(169, (5.540622384e34 * s.v[30]), 522);}
        if (!s.b[1394]) {s.store_scalar(169, 5.540622384e34);}
        s.store_offset_div(177, 167, 169, 1.0);s.store_mul(175, 175, 177);s.store_pow_ad(12, s.ad_value(159), A::div_from_scalar(1.0, s.ad_value(166)));s.store_mul(23, 453, 61);s.store_sqrt_square_offset(24, 23, 0.1);s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24))), 0.5);s.store_div_scaled_product_offset_denominator_mixed_iia(14, 80, 13, (10.0 * p.p433), A::mul(s.ad_value(80), s.ad_value(13)), (10.0 * p.p433), 1.0);s.b[1396] = (s.v[536] < 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if s.b[1396] {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);}
    }
}
