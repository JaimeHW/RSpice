#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[57] + 1.0);let t1: f64 = if ((!s.b[737]) && (s.v[167] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;
            if t2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (!s.b[737]) {s.store_sub(417, 349, 515);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1075] = (s.v[337] < 80.0);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1075]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if ((!s.b[737]) && (!s.b[1075])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if (!s.b[737]) {s.store_mul(421, 225, 422);}
            s.b[1076] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1076]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1077] = (s.v[181] < 0.0);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1076]) && s.b[1077]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1078] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1076])) && s.b[1078]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if (((!s.b[737]) && (!s.b[1076])) && (!s.b[1078])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1079] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1079]) {s.store_scalar(338, (-1.0));}
            s.b[1080] = (s.v[338] == (-1.0));s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1080]) {s.store_scalar(401, 0.0);}
            if ((!s.b[737]) && (!s.b[1080])) {s.store_mul(401, 444, 242);}
            s.b[1081] = (s.v[401] < (p[237] * 1.01));s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1081]) {s.store_scalar(339, 1.0);}
            if ((!s.b[737]) && (!s.b[1081])) {s.store_scalar(339, 2.0);}
            if (!s.b[737]) {s.store_mul(370, 229, 401);}
            s.b[1082] = (s.v[181] < 0.0);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1082]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1083] = (s.v[181] < 1e-7);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1082])) && s.b[1083]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            s.b[1084] = (s.v[181] < 80.0);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && s.b[1084]) {s.store_exp(243, 181);s.store_mul_sub_mixed_iia(488, 487, 243, A::offset(s.ad_value(181), 1.0));s.store_mul_ad_product_rhs_mixed_ia(489, 487, 225, A::offset(s.ad_value(243), (-1.0)));}
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && (!s.b[1084])) {s.store_exp_mul(485, 225, 349);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(485), s.ad_value(486)));}
            if (((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) {s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if (!s.b[737]) {s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1085] = (s.v[430] == 1.0);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1085]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if ((!s.b[737]) && (!s.b[1085])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1086] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1086]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((!s.b[737]) && (!s.b[1085])) {s.store_add(349, 349, 494);}
            s.b[1087] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1087]) {s.store_scalar(430, 1.0);}
            if (!s.b[737]) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[737]) {s.store_primal_offset(167, 167, (-1.0));s.copy_ad(371, 370);s.copy_ad(356, 371);s.copy_ad(161, 349);s.store_div(568, 371, 238);s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));s.store_scale(328, 568, 2.0);s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));s.store_mul(245, 238, 170);s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));s.store_mul3_lhs(244, 238, 488, 328);s.store_neg(355, 244);s.store_mul(192, 244, 324);}
        s.b[1088] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if ((!s.b[737]) && s.b[1088]) {s.store_scalar(338, 4.0);s.store_scalar(145, 1.0);s.store_sub(329, 159, 161);s.store_mul(437, 323, 329);s.store_scale(327, 108, (-s.v[98]));s.store_mul(196, 327, 437);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_mul_scale_offset_indices(329, 437, 534, -1.0, 0.0);s.store_scale(468, 329, s.v[438]);s.store_sub(467, 329, 468);s.store_scalar(470, 0.0);s.store_scalar(469, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);s.store_scalar(145, 1.0);s.copy_ad(352, 349);s.copy_ad(162, 161);s.copy_ad(314, 162);s.store_scalar(612, 1.0);}
        s.b[1089] = (s.v[612] == 0.0);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if ((!s.b[737]) && s.b[1089]) {s.copy_ad(453, 157);s.store_scalar(1096, 1e-50);s.store_div_square_rhs(1091, 545, 323);s.store_offset_mul_ad(1093, A::div_from_scalar(2.0, s.ad_value(1091)), A::sub(s.ad_value(159), s.ad_value(1096)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1091), 1.0);}
        s.b[1097] = ((s.v[1093] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {s.store_sub(44, 332, 1093);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1098] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });s.b[1099] = (4.0 == 1.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && s.b[1099]) {s.store_scalar(55, 1.0);}
        s.b[1100] = (4.0 == 2.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && s.b[1100]) {s.store_scalar(55, 2.0);}
        s.b[1101] = (4.0 == 4.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if (((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && s.b[1101]) {s.store_scalar(55, 3.0);}
        s.b[1102] = (4.0 == 8.0);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && (!s.b[1101])) && s.b[1102]) {s.store_scalar(55, 4.0);}
        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {s.store_scalar(54, 0.0);}
        let mut t4: usize = 0;
        while {
            let t3: f64 = if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;
            if t4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && (!s.b[1098])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {s.store_sub(1093, 332, 43);}
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1097])) {
        }
        if ((!s.b[737]) && s.b[1089]) {s.store_sqrt(1092, 1093);s.store_add_mul_sub_from_scalar_rhs_indices(1096, 159, 1091, 1.0, 1092);s.store_sqrt_square_offset(44, 1096, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1096, 1096, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1103] = (s.v[1096] < 0.0);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (((!s.b[737]) && s.b[1089]) && s.b[1103]) {s.store_scalar(1096, 0.0);}
        if ((!s.b[737]) && s.b[1089]) {s.store_div(1090, 157, 1096);s.store_pow_offset_rhs(1091, 1090, 138, (-1.0));s.store_mul(1095, 1091, 1090);s.store_offset(1092, 1095, 1.0);s.store_pow_ad(1093, s.ad_value(1092), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(1094, 1093, 1092);s.store_div(452, 157, 1094);s.copy_ad(157, 452);s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));}
        s.b[1104] = (s.v[157] <= 0.0);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if (((!s.b[737]) && s.b[1089]) && s.b[1104]) {s.store_scalar(164, 0.0);s.copy_ad(162, 161);s.store_scalar(430, 0.0);}
        s.b[1105] = (s.v[144] >= 1.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1105]) {s.store_scalar(352, s.v[622]);s.store_sub_from_scalar(165, s.v[622], 161);}
        s.b[1106] = (s.v[144] == 0.0);s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1107] = (s.v[165] < 0.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1107]) {s.store_scalar(165, 0.0);}
        s.b[1108] = (s.v[165] > s.v[157]);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && (!s.b[1107])) && s.b[1108]) {s.copy_ad(165, 157);}
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1104])) {s.copy_ad(164, 165);s.store_add(162, 161, 164);s.store_scalar(430, 0.0);}
        if ((!s.b[737]) && s.b[1089]) {s.copy_ad(352, 162);s.store_scalar(168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t5: f64 = (s.v[58] + 1.0);let t6: f64 = if (((!s.b[737]) && s.b[1089]) && (s.v[168] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[737]) && s.b[1089]) {s.store_sub(418, 352, 515);s.store_mul(181, 225, 418);s.store_mul_sub_rhs(337, 420, 418, 419);}
            s.b[1109] = (s.v[337] < 80.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1089]) && s.b[1109]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1109])) {s.store_sub(422, 418, 419);s.store_scalar(423, 1.0);}
            if ((!s.b[737]) && s.b[1089]) {s.store_mul(421, 225, 422);}
            s.b[1110] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1089]) && s.b[1110]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1111] = (s.v[181] < 0.0);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && s.b[1089]) && s.b[1110]) && s.b[1111]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1112] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && s.b[1112]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && (!s.b[1112])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1113] = (s.v[338] == (-1.0));s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1089]) && s.b[1113]) {s.store_scalar(401, 0.0);}
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1113])) {s.store_mul(401, 444, 242);}
            if ((!s.b[737]) && s.b[1089]) {s.store_mul(370, 229, 401);}
            s.b[1114] = (s.v[181] < 0.0);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1089]) && s.b[1114]) {s.store_neg(499, 242);s.store_neg(500, 443);}
            s.b[1115] = (s.v[181] < 1e-7);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && s.b[1115]) {s.copy_ad(499, 242);s.copy_ad(500, 443);}
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && (!s.b[1115])) {s.store_mul_sub_rhs(501, 225, 352, 157);s.store_exp(502, 501);s.store_mul_mixed_ia(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(498, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(499, 242, 497);s.store_div_scaled_add_product_indices(500, 498, 0.5, 443, 242, (2.0 * 0.5), 499, 1.0);}
            if ((!s.b[737]) && s.b[1089]) {s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));s.store_offset_mul(504, 240, 500, 1.0);}
            s.b[1116] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1089]) && s.b[1116]) {s.store_scalar(168, (s.v[58] + 1.0));}
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {s.store_div_scaled_inputs_indices(495, 503, -1.0, 504, 1.0);}
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1117] = (((s.v[495]) as f64).abs() > s.v[496]);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1117]) {s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {s.store_add(352, 352, 495);}
            s.b[1118] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1118]) {s.store_scalar(430, 1.0);}
            if ((!s.b[737]) && s.b[1089]) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[737]) && s.b[1089]) {s.store_primal_offset(168, 168, (-1.0));s.copy_ad(372, 370);s.copy_ad(359, 372);s.copy_ad(162, 352);s.store_div(569, 372, 238);s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(499), s.ad_value(171));s.store_mul3_lhs(358, 238, 497, 328);s.store_neg(358, 358);s.store_sub(164, 162, 161);s.copy_ad(157, 453);s.store_div(328, 225, 169);s.store_mul(505, 328, 164);s.store_offset(506, 505, 1.0);s.store_sqrt(507, 506);s.store_div_from_scalar_offset_input(508, 1.0, 507, 1.0);s.store_div(509, 508, 170);s.store_scaled_add(510, 568, 569, 0.5);s.store_add_scaled_inputs4_indices(328, 159, 1.0, 227, 1.0, 161, (-(2.0 * 0.5)), 164, (-0.5));s.store_sub(329, 509, 510);s.store_mul(330, 225, 323);s.store_mul(331, 225, 238);s.store_add_scaled_products_indices(511, 330, 328, 1.0, 331, 329, 1.0);s.store_scaled_add(424, 359, 356, 0.5);s.store_scaled_add(425, 358, 355, (-0.5));s.store_sub(426, 359, 356);s.store_sub(427, 355, 358);s.store_square(428, 238);}
        s.b[1119] = (s.v[339] <= 1.0);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if (((!s.b[737]) && s.b[1089]) && s.b[1119]) {s.store_add_scaled_inputs3_mixed_aia(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, 427, (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);}
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1119])) {s.store_mul(246, 164, 511);}
        s.b[1120] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        if (((!s.b[737]) && s.b[1089]) && s.b[1120]) {s.store_scalar(246, 0.0);}
        s.b[1121] = (s.v[339] <= 1.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });s.b[1122] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && s.b[1122]) {s.store_add_scaled_product_mixed_aia(437, A::div_scaled_product(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), 1.0, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);s.store_div(437, 437, 246);}
        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && (!s.b[1122])) {s.copy_ad(437, 424);}
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1121])) {s.store_scaled_add(437, 359, 356, 0.5);}
        if ((!s.b[737]) && s.b[1089]) {s.store_scale(328, 240, 2.0);s.store_mul_sub_rhs(512, 328, 510, 170);s.store_add(191, 164, 512);s.store_div_from_scalar(328, 1.0, 192);s.store_mul(329, 191, 328);s.store_sub_from_scalar(330, 1.0, 329);s.store_sub_from_scalar(336, 1.0, 330);s.store_square(49, 336);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[737]) && s.b[1089]) {s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1123] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });s.b[1124] = (4.0 == 1.0);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && s.b[1124]) {s.store_scalar(55, 1.0);}
        s.b[1125] = (4.0 == 2.0);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && s.b[1125]) {s.store_scalar(55, 2.0);}
        s.b[1126] = (4.0 == 4.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if ((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && s.b[1126]) {s.store_scalar(55, 3.0);}
        s.b[1127] = (4.0 == 8.0);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if (((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && (!s.b[1126])) && s.b[1127]) {s.store_scalar(55, 4.0);}
        if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {s.store_scalar(54, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1123])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((!s.b[737]) && s.b[1089]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(337, 336, 53, 1.0);s.store_sub_from_scalar(190, 1.0, 337);s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);}
        if ((!s.b[737]) && s.b[1089]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[737]) && s.b[1089]) {s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);}
        s.b[1128] = (s.v[339] <= 1.0);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });s.b[1129] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && s.b[1129]) {s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));s.store_div(436, 436, 246);}
        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && (!s.b[1129])) {s.copy_ad(436, 425);}
        if (((!s.b[737]) && s.b[1089]) && (!s.b[1128])) {s.store_scaled_add(436, 355, 358, (-0.5));}
        s.b[1133] = (s.v[612] == 0.0);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if s.b[1133] {s.store_offset(480, 190, 0.5);s.store_mul(481, 479, 478);s.store_div_scaled_inputs_indices(482, 480, 0.4, 481, 1.0);s.store_sub_from_scalar(438, 0.6, 482);}
        s.b[1134] = (s.v[438] > (0.5 + 1e-8));s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1134]) {s.store_scalar(438, 0.5);}
        if s.b[1133] {s.copy_ad(439, 438);s.store_scalar(438, 0.5);}
        s.b[1136] = (s.v[145] == 0.0);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });s.b[1152] = ((p[190] < (10.0 * 2.220446049250313e-16)) && (p[191] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1136]) && s.b[1152]) {s.store_scalar(316, 0.0);s.copy_ad(314, 162);}
        s.b[1153] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if (((s.b[1133] && s.b[1136]) && s.b[1152]) && s.b[1153]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {s.store_scalar(1151, (if (p[43] == 1.0) { p[237] } else { s.v[402] }));}
        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {s.store_primal_div_from_scalar(1137, 1.0, 1151);s.store_mul(1138, 244, 1137);s.store_scale(1139, 1138, p[191]);s.store_add_scaled_product_indices(1142, 1139, 1.0, 80, 229, 1.0);s.store_div_from_scalar(1138, 1.0, 1142);s.store_scale(1141, 1138, 1.034943e-10);s.store_scalar(1138, (1.0 - p[189]));s.store_add_scaled_inputs_product_indices(314, 157, p[189], 161, p[189], 1138, 162, 1.0);}
        s.b[1154] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1154]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {s.store_sub(1144, 314, 162);s.store_sqrt_square_offset(44, 1144, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1143, 1144, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1155] = (s.v[1143] < 0.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1155]) {s.store_scalar(1143, 0.0);}
        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {s.store_mul(1140, 225, 244);s.store_div_from_scalar(1138, 1.0, 1140);s.store_mul(1142, 246, 1138);}
        s.b[1156] = (s.v[1142] < s.v[227]);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1156]) {s.copy_ad(1142, 227);}
        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {s.store_scale(1148, 229, 9662367879.197212);s.store_scalar(1138, (100000.0 * 10000.0));s.store_scalar(1139, (1.0 / s.v[97]));s.store_mul_mixed_ai(1150, A::add_scaled_inputs_product(s.ad_value(1142), 2.0, A::mul3_scaled_output(s.ad_value(1148), s.ad_value(1143), s.ad_value(1141), 2.0), 1.0, s.ad_value(1138), s.ad_value(1141), 1.0), 1139);s.store_mul(1145, 1150, 1141);s.store_add_scaled_product_indices(1149, 1138, 4.0, 1148, 1143, (2.0 * 4.0));s.store_mul3_lhs(1146, 1149, 1141, 1141);s.store_sqrt_square_add(1147, 1145, 1146);s.store_mul_sub_scaled_inputs_rhs_indices(316, 326, 1147, 0.5, 1145, 0.5);}
        if (s.b[1133] && s.b[1136]) {s.store_scale(316, 316, s.v[127]);}
        if s.b[1133] {s.store_sub_from_scalar(441, s.v[97], 316);}
        s.b[1157] = (s.v[441] < 1e-9);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1157]) {s.store_scalar(441, 1e-9);}
        if s.b[1133] {s.store_scale(328, 108, (-s.v[98]));s.store_mul(196, 328, 437);s.store_mul(197, 328, 436);s.store_mul(198, 197, 438);}
        s.b[1158] = (p[43] == 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1158]) {s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_mul_scale_offset_mixed_ia(392, 108, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 0.0);}
        if s.b[1133] {s.store_scaled_sub(1159, 157, 164, 0.5);s.store_scale(44, 1159, (2.0 * 1.0 / (p[227])));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(177, p[227], 45);}
        s.b[1160] = (s.v[177] < (10.0 * 2.220446049250313e-16));s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1160]) {s.store_scalar(177, (10.0 * 2.220446049250313e-16));}
        if s.b[1133] {s.store_add(176, 161, 177);s.store_scalar(1170, (1.034943e-10 / 100.0));s.store_scale(1171, 437, 0.0001);s.store_scale(1172, 436, 0.0001);s.store_div_from_scalar(1161, p[92], 1170);s.store_primal_div_from_scalar(1162, p[93], 1170);s.store_scalar(1163, p[94]);s.store_offset_mul_ad(1164, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1163), 1.0);s.store_add_scaled_products_indices(1165, 1161, 1171, 1.0, 1162, 1172, 1.0);s.store_div(1166, 1165, 1164);s.copy_ad(248, 1166);s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));s.store_offset_add_scaled_inputs_indices(1163, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));}
        s.b[1173] = (s.v[1163] < 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1173]) {s.store_scalar(1163, 0.0);}
        if s.b[1133] {s.store_powf(1165, 1163, (p[97] - 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1133] {s.store_mul(1167, 1165, 1163);s.store_powf(1168, 1163, (s.v[111] - 1.0));s.store_mul(1169, 1168, 1163);s.store_scale(249, 1172, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(1161, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p[96] * 1e-11), p[95])), 1.0, s.ad_value(543), s.ad_value(1167), 1.0), 1.0, 1169, 1.0 / (p[106]));s.store_div_from_scalar(251, 1.0, 1161);s.store_scale(251, 251, 0.0001);s.store_mul3_lhs(1174, 225, 244, 441);s.store_sqrt_square_offset(44, 1174, ((4.0 * 1e-50) * 1e-50));s.store_offset_add_scaled_inputs_indices(1174, 1174, 0.5, 44, 0.5, (1e-10 * 1e-50));}
        s.b[1182] = (s.v[1174] < 0.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1182]) {s.store_scalar(1174, 0.0);}
        if s.b[1133] {s.store_div_from_scalar(1175, 1.0, 1174);s.store_mul(1176, 246, 1175);s.store_div_scaled_inputs_indices(1174, 253, 0.2, 251, 1.0);s.store_sqrt_square_sum(252, 1176, 1174);s.store_mul(1177, 251, 252);s.store_div(1175, 1177, 253);}
        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1183]) {s.store_scalar(1178, 1.0);}
        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1133] && (!s.b[1183])) && s.b[1184]) {s.copy_ad(1178, 1175);}
        if ((s.b[1133] && (!s.b[1183])) && (!s.b[1184])) {s.store_powf(1178, 1175, (p[113] - 1.0));}
        if s.b[1133] {s.store_mul(1174, 1175, 1178);s.store_offset(1179, 1174, 1.0);}
        s.b[1185] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1185]) {s.store_div_from_scalar(1180, 1.0, 1179);}
        s.b[1186] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if ((s.b[1133] && (!s.b[1185])) && s.b[1186]) {s.store_div_from_scalar_sqrt_ad(1180, 1.0, s.ad_value(1179));}
        if ((s.b[1133] && (!s.b[1185])) && (!s.b[1186])) {s.store_powf(1181, 1179, (((-1.0) / p[113]) - 1.0));s.store_mul(1180, 1179, 1181);}
        if s.b[1133] {s.store_mul(250, 251, 1180);s.store_div_scaled_product_mixed_iia(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);s.store_mul3_lhs(200, 264, 246, 250);s.store_scalar(201, 0.0);}
        s.b[1196] = ((p[281] > 0.0) && (p[244] != 0.0));s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1196]) {s.store_scaled_sub(1187, 157, 164, 0.5);s.store_scale(44, 1187, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(1193, 0.01, 45);s.store_sub_from_scalar_ad(1187, 1.1, A::add(s.ad_value(161), s.ad_value(1193)));s.store_sqrt_square_offset(44, 1187, ((4.0 * 0.05) * 0.05));s.store_offset_add_scaled_inputs_indices(1195, 1187, 0.5, 44, 0.5, (1e-10 * 0.05));}
        s.b[1197] = (s.v[1195] < 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1196]) && s.b[1197]) {s.store_scalar(1195, 0.0);}
        if (s.b[1133] && s.b[1196]) {s.store_scale(1188, 225, s.v[116]);s.store_mul(1189, 323, 1188);s.store_powf(1188, 1195, p[245]);s.store_mul(1190, 1189, 1188);s.store_offset_scaled(1191, 173, p[246], 1.0);s.store_scalar(1188, s.v[117]);}
        s.b[1198] = ((s.v[56] < 3.0) || (p[43] == 1.0));s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1196]) && s.b[1198]) {s.store_add_scaled_inputs3_indices(1192, 161, 1.0, 1193, 1.0, 172, -1.0);}
        if ((s.b[1133] && s.b[1196]) && (!s.b[1198])) {s.store_add_scaled_inputs3_indices(1192, 161, 1.0, 1193, 1.0, 350, -1.0);}
        if (s.b[1133] && s.b[1196]) {s.store_add_product3_rhs_indices(1191, 1191, 173, 1188, 1192, 1.0);s.store_mul(1193, 1190, 1191);s.copy_ad(1190, 1193);}
        if (s.b[1133] && (!s.b[1196])) {s.store_scalar(1190, 0.0);}
        s.b[1199] = (p[248] != 0.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1199]) {s.store_scale(1187, 225, s.v[118]);s.store_mul(1195, 323, 1187);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1133] && s.b[1199]) {s.store_mul(1194, 1195, 173);}
        if (s.b[1133] && (!s.b[1199])) {s.store_scalar(1194, 0.0);}
        s.b[1200] = ((s.v[1190] + s.v[1194]) > 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1200]) {s.store_mul_add_rhs(247, 164, 1190, 1194);s.store_mul3_lhs(201, 264, 247, 250);}
        if s.b[1133] {s.store_add(199, 200, 201);s.copy_ad(203, 201);}
        s.b[1210] = (p[33] != 0.0);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1210]) {s.copy_ad(1203, 549);s.store_scalar(1204, (s.v[124] - p[71]));s.store_div_from_scalar_square_ad(1205, 1.0, s.ad_value(1204));s.store_mul_ad_product_lhs_mixed_ai(1206, A::mul_sub_from_scalar_lhs_scaled_output(p[69], s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), 1203, 1205);s.store_mul(186, 1206, 235);s.store_offset_scaled(1202, 173, p[155], p[154]);s.store_mul(206, 186, 1202);s.store_sub_from_scalar_scaled_input(1201, p[156], 157, p[157]);s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1201, 1.0, 206, 1.0, (-s.v[123]));s.store_mul3_lhs(210, 205, 324, 324);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_offset_sub_ad(1207, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p[156])))), s.ad_value(206), 1e-50);s.store_offset_sub(1201, 174, 1207, (-0.005));}
        if (s.b[1133] && s.b[1210]) {s.store_scalar(327, (if (s.v[1207] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_add_scaled_square_product(1203, 1201, 1.0, 327, 1207, (4.0 * 0.005));s.store_sub_mixed_ai(1204, A::add_scaled_inputs4_offset(s.ad_value(1207), 1.0, s.ad_value(1201), 0.5, s.ad_value(1203), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p[156]))), 514);s.store_offset_mul(1205, 225, 1204, (-1.0));s.store_div_from_scalar(1206, 4.0, 212);s.store_offset_mul(1202, 1205, 1206, 1.0);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1211] = (s.v[1201] < 0.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1211]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(213, 1201, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));s.store_mul_ln_mixed_ia(216, 327, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))));s.store_div_scaled_value_offset_denominator(1204, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);s.store_offset_sub(217, 216, 215, (-0.002));s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));s.store_div_from_scalar(1201, 1.0, 327);s.store_mul_exp_mixed_ia(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));s.store_add_offset_lhs_mixed_ai(1202, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1212] = (s.v[1201] < 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1212]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(219, 1201, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(1202, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1213] = (s.v[1201] < 0.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1213]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(220, 1201, (10.0 * 2.220446049250313e-16));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_sub(1202, 215, 218);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1214] = (s.v[1201] < 0.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1214]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_div_scaled_value_offset_denominator(1208, s.ad_value(157), 1.0, s.ad_value(1201), (10.0 * 2.220446049250313e-16), 1.0);s.store_square(49, 1208);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1215] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });s.b[1216] = (4.0 == 1.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((s.b[1133] && s.b[1210]) && s.b[1215]) && s.b[1216]) {s.store_scalar(55, 1.0);}
        s.b[1217] = (4.0 == 2.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {s.store_scalar(55, 2.0);}
        s.b[1218] = (4.0 == 4.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if (((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && s.b[1218]) {s.store_scalar(55, 3.0);}
        s.b[1219] = (4.0 == 8.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && (!s.b[1218])) && s.b[1219]) {s.store_scalar(55, 4.0);}
        if ((s.b[1133] && s.b[1210]) && s.b[1215]) {s.store_scalar(54, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if (((s.b[1133] && s.b[1210]) && s.b[1215]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1133] && s.b[1210]) && s.b[1215]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[1133] && s.b[1210]) && (!s.b[1215])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (s.b[1133] && s.b[1210]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(1209, 1208, 53, 1.0);s.store_scale(214, 227, ((2.0 * s.v[126]) * p[9]));s.store_div_scaled_product_mixed_aii(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1209, 1.0, 441, 1.0);s.store_add(199, 199, 222);}
        s.b[1220] = ((p[30] != 0.0) && (p[32] != 0.0));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1220]) {s.store_square(294, 192);s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);s.store_sub(296, 294, 295);s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1221] = (s.v[294] < 0.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1220]) && s.b[1221]) {s.store_scalar(294, 0.0);}
        if (s.b[1133] && s.b[1220]) {s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1222] = (s.v[296] < 0.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1220]) && s.b[1222]) {s.store_scalar(296, 0.0);}
        if (s.b[1133] && s.b[1220]) {s.store_sub(297, 294, 296);}
        s.b[1223] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1220]) && s.b[1223]) {s.store_scalar(146, 0.0);}
        if ((s.b[1133] && s.b[1220]) && (!s.b[1223])) {s.store_scalar(146, 1.0);}
        s.copy_ad(202, 199);s.store_scalar(204, 0.0);s.b[1224] = ((p[281] > 0.0) && (p[285] > 0.0));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if s.b[1224] {s.store_scalar(1231, s.v[99]);s.store_scalar(1235, p[237]);s.store_offset_add_scaled_inputs3_offset_indices(1236, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p[286]));s.store_offset(1237, 182, p[286]);s.store_scalar(1239, p[285]);s.store_scalar(1238, p[283]);s.store_scalar(1229, s.v[70]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1224] {s.store_mul_ln_mixed_ia(1230, 227, A::div_scaled_product_by_product(s.ad_value(1229), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));}
        if s.b[1224] {
            if (p[43] == 1.0) {
                s.copy_ad(1227, 435);
            } else {
                s.copy_ad(1227, 350);
            }
        }
        if s.b[1224] {s.store_sqrt_ad(1232, A::div_scaled_product3(A::sub(s.ad_value(1230), s.ad_value(1227)), s.ad_value(536), s.ad_value(1229), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1229)), 1.0));s.store_mul(1226, 1232, 1231);s.store_div_scaled_product_add_scaled_denominator_indices(1225, 1226, 1226, (-0.25), 157, 1.0, 1226, 1.0, 1.0);s.copy_ad(1251, 1225);s.copy_ad(1252, 1237);s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1236), s.ad_value(1251))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if s.b[1224] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[1224] {s.store_add_product3_rhs_mixed_iia(376, 1236, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);}
        s.b[1253] = (s.v[158] < ((s.v[123] + s.v[1252]) * 0.5));s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1253]) {s.store_scalar(144, 0.0);}
        s.b[1254] = ((s.v[144] == 0.0) || (1.0 != 0.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1254]) {s.store_mul_sub_rhs(181, 225, 376, 1251);}
        s.b[1255] = (s.v[181] < 3.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((s.b[1224] && s.b[1254]) && s.b[1255]) {s.store_mul_sub_rhs(337, 225, 1236, 1251);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 1251, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[1256] = ((s.v[158] - s.v[383]) <= s.v[1252]);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });s.b[1257] = (p[43] == 0.0);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if ((((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {s.copy_ad(378, 376);}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        s.b[1258] = (p[43] == 0.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });s.b[1259] = ((s.v[158] - s.v[383]) <= s.v[1252]);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && s.b[1259]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);s.copy_ad(378, 376);}
        s.b[1260] = ((s.v[1236] - s.v[383]) > 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if ((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);}
        s.b[1261] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1262] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });s.b[1263] = (2.0 == 1.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && s.b[1263]) {s.store_scalar(55, 1.0);}
        s.b[1264] = (2.0 == 2.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && s.b[1264]) {s.store_scalar(55, 2.0);}
        s.b[1265] = (2.0 == 4.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && s.b[1265]) {s.store_scalar(55, 3.0);}
        s.b[1266] = (2.0 == 8.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {s.store_scalar(55, 4.0);}
        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {s.store_scalar(54, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && (!s.b[1262])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && (!s.b[1261])) {s.copy_ad(378, 376);}
        if s.b[1224] {s.store_offset(336, 1251, (5e-12 / 2.0));}
        s.b[1267] = (s.v[378] < s.v[336]);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1267]) {s.copy_ad(378, 336);}
        if s.b[1224] {s.copy_ad(1234, 378);s.copy_ad(163, 376);}
        if (s.b[1224] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1234]) >= 0.0) {
                s.store_sub(166, 376, 1234);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (s.b[1224] && (0.0 != 0.0)) {s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p[287])) + ((-0.03))));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1224] && (0.0 != 0.0)) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1268] = (s.v[165] < 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if ((s.b[1224] && (0.0 != 0.0)) && s.b[1268]) {s.store_scalar(165, 0.0);}
        s.b[1269] = (s.v[165] > s.v[157]);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1224] && (0.0 != 0.0)) && (!s.b[1268])) && s.b[1269]) {s.copy_ad(165, 157);}
        if (s.b[1224] && (0.0 != 0.0)) {s.store_add(163, 1234, 165);}
        s.b[1270] = (p[282] == 1.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1270]) {s.copy_ad(378, 1234);s.copy_ad(1271, 1225);s.store_offset_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 1271, 1.0, s.v[123], p[286]);}
        s.b[1273] = (s.v[158] < s.v[160]);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {s.store_scalar(338, (-1.0));s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));s.store_mul_sub_rhs(336, 225, 1236, 1271);s.store_div_scalar_by_product_indices(328, 1.0, 225, 238, 1.0);s.store_mul(337, 328, 323);s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);s.store_offset(331, 336, (-2.0));s.store_scaled_mul(332, 337, 331, 9.0);s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);s.store_square(259, 261);}
        s.b[1274] = (s.v[260] < (s.v[259] * 1e-8));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && s.b[1274]) {s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));}
        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && (!s.b[1274])) {s.store_sqrt_add(258, 260, 259);s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);}
        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {s.store_powf(256, 257, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);s.store_div_from_scalar(328, 1.0, 256);s.store_mul(181, 255, 328);s.store_add_scaled_product_indices(313, 1271, 1.0, 181, 227, 1.0);s.store_sub(328, 313, 1271);s.store_div(329, 328, 254);s.store_sqrt_square_offset(330, 329, 1.0);s.store_add_div_lhs_indices(1234, 328, 330, 1271);}
        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1271), (-p[287])));s.store_scalar(430, 0.0);s.copy_ad(1272, 378);s.store_scale(419, 229, ((p[237] * (p[237] * 0.5)) * 9662367879.197212));s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);s.store_div_ln_lhs(420, 328, 419);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t10: usize = 0;
        while {
            let te: f64 = (s.v[57] + 1.0);let tf: f64 = if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (s.v[167] <= te)) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;
            if t10 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t10, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_sub(417, 1272, 1271);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1275] = (s.v[337] < 80.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1275]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1275])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_mul(421, 225, 422);}
            s.b[1276] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1277] = (s.v[181] < 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) && s.b[1277]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1278] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && s.b[1278]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && (!s.b[1278])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1279] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1279]) {s.store_scalar(338, (-1.0));}
            s.b[1280] = (s.v[181] < 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1280]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1281] = (s.v[181] < 1e-7);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && s.b[1281]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && (!s.b[1281])) {s.store_mul_scale_offset_indices(501, 225, 1272, 1.0, (-p[287]));s.store_exp(502, 501);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_add_scaled_inputs_product_indices(492, 1272, 1.0, 1236, (-1.0), 240, 490, 1.0);s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1282] = (s.v[430] == 1.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1282]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1272]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1272))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1283] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1283]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {s.store_add(1272, 1272, 494);}
            s.b[1284] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1284]) {s.store_scalar(430, 1.0);}
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.copy_ad(1234, 1272);}
        if s.b[1224] {s.store_mul_sub_scaled_inputs_rhs_indices(332, 225, 1234, -1.0, 1225, -1.0);}
        if s.b[1224] {s.store_scalar(1249, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));}
        if s.b[1224] {s.store_mul(1250, 1249, 332);s.store_exp(333, 332);s.store_sub_offset_lhs(334, 333, (-1.0), 332);}
        s.b[1285] = (s.v[332] > 1e-7);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1285]) {s.store_mul_scaled_sqrt_rhs(437, 238, -1.0, 334);}
        s.b[1286] = (s.v[1250] > 1e-7);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if ((s.b[1224] && (!s.b[1285])) && s.b[1286]) {s.store_mul_sqrt_rhs(437, 238, 334);}
        if ((s.b[1224] && (!s.b[1285])) && (!s.b[1286])) {s.store_mul_ad_affine_product_rhs(437, 1249, s.ad_value(1250), A::sqrt_scaled_lhs_product_offset(s.ad_value(1250), 0.3333333333333333, A::scale_offset(s.ad_value(1250), 0.25, 1.0), 1.0), (-0.7071067811865475), 0.0);}
        if s.b[1224] {s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));s.store_offset_add_scaled_inputs_indices(1246, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));}
        s.b[1287] = (s.v[1246] < 0.0);s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1287]) {s.store_scalar(1246, 0.0);}
        if s.b[1224] {s.store_div_scaled_inputs_indices(1247, 1246, 1.0, 536, 1.6021918e-19);s.store_sub(328, 1247, 1238);s.store_scale(1248, 1247, 0.01);s.store_sqrt_add_scaled_square_product(44, 328, 1.0, 1248, 1248, 4.0);s.store_add_scaled_inputs3_indices(329, 328, 0.5, 44, 0.5, 1248, 1e-10);}
        s.b[1288] = (s.v[329] < 0.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1288]) {s.store_scalar(329, 0.0);}
        if s.b[1224] {s.store_div_scaled_product_by_product_indices(1245, 329, 329, 1.0, 1247, 1247, 1.0);s.store_add_scaled_product_mixed_iai(1228, 1225, 1.0, A::sub(s.ad_value(1234), s.ad_value(1225)), 1245, 1.0);s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1228))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1228), s.ad_value(157)))));s.store_primal_sqrt_scaled_input(1241, 1229, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_mul_sqrt_rhs(1242, 1241, 227);s.store_mul_sub_rhs(1233, 225, 1228, 1225);}
        s.b[1289] = ((s.v[1233] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1289]) {s.store_sub_scaled_inputs(44, 225, 0.2, 1233, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 225, 225, (0.2 * 0.2));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1290] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });s.b[1291] = (1.0 == 1.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((s.b[1224] && s.b[1289]) && s.b[1290]) && s.b[1291]) {s.store_scalar(55, 1.0);}
        s.b[1292] = (1.0 == 2.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if ((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && s.b[1292]) {s.store_scalar(55, 2.0);}
        s.b[1293] = (1.0 == 4.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if (((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {s.store_scalar(55, 3.0);}
        s.b[1294] = (1.0 == 8.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {s.store_scalar(55, 4.0);}
        if ((s.b[1224] && s.b[1289]) && s.b[1290]) {s.store_scalar(54, 0.0);}
        let mut t12: usize = 0;
        while {
            let t11: f64 = if (((s.b[1224] && s.b[1289]) && s.b[1290]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;
            if t12 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t12, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1224] && s.b[1289]) && s.b[1290]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[1224] && s.b[1289]) && (!s.b[1290])) {s.store_powf(53, 53, (1.0 / 2.0));}
        if (s.b[1224] && s.b[1289]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);}
        if (s.b[1224] && (!s.b[1289])) {s.copy_ad(328, 1233);}
        if s.b[1224] {s.store_sqrt_offset_input(1243, 328, (10.0 * 2.220446049250313e-16));s.store_mul(1244, 1242, 1243);}
    }
}
