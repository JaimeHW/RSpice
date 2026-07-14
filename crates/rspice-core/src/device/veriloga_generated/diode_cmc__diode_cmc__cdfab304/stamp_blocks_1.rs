#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if ((s.b[418] && (!s.b[522])) && (!s.b[523])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);}
        if (s.b[418] && (!s.b[522])) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(190), s.ad_value(428)), 1.0);s.store_mul(437, 103, 372);}
        s.b[524] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[522])) && s.b[524]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {s.store_primal_sub(439, 109, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[525] = (s.v[11] == 0.5);s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[525]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[525])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));}
        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {s.store_primal_add(442, 440, 441);}
        s.b[526] = (s.v[11] == 0.5);s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[526]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[145]);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[526])) {s.store_primal_powf_scaled_input(436, 439, s.v[145], s.v[11]);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {s.store_primal_scale(443, 436, s.v[139]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 100, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[22]);}
        s.b[527] = (s.v[25] == 0.0);s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[522])) && s.b[527]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[124] * s.v[154]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[528] = (((-s.v[11]) * s.v[127]) == (-1.0));s.store_scalar(528, if s.b[528] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[528]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[528])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));}
        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[151]), 447, 450, s.v[151], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[529] = (s.v[457] > 0.0);s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[529]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[529])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[530] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[530]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[530])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[531] = (s.v[457] > 0.0);s.store_scalar(531, if s.b[531] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[531]) {s.copy_ad(458, 421);}
        s.b[532] = (s.v[456] > (-230.25850929940458));s.store_scalar(532, if s.b[532] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && s.b[532]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && (!s.b[532])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[151] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);}
        s.b[533] = (s.v[31] == 0.0);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[522])) && s.b[533]) {s.store_scalar(460, 0.0);}
        s.b[534] = (s.v[11] == 0.5);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[534]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[534])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[145]), ((s.v[8]) * (s.v[145])), s.v[11]);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), 436, 1.0);}
        s.b[535] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[535]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));}
        s.b[536] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && s.b[536]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 157, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && (!s.b[536])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 157, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[31], 0.0);}
        s.b[537] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(537, if s.b[537] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[522])) && s.b[537]) {s.store_scalar(462, 1.0);}
        s.b[538] = (s.v[435] > ((-s.v[158]) * s.v[40]));s.store_scalar(538, if s.b[538] { 1.0 } else { 0.0 });s.b[539] = (s.v[43] == 4.0);s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && s.b[539]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));}
        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && (!s.b[539])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);}
        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && (!s.b[538])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if (s.b[418] && (!s.b[522])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        if s.b[418] {s.store_add_scaled_inputs3_indices(180, 268, s.v[256], 270, s.v[257], 272, s.v[258]);}
        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[540]) {s.store_primal_scaled_mul(422, 265, 265, 4.0);s.store_primal_div(423, 265, 266);s.store_primal_add_scaled_product_indices(424, 191, 1.0, 265, 423, 1.0);s.store_primal_add(425, 266, 424);s.store_primal_sub(426, 266, 424);s.store_primal_sqrt_square_add(427, 426, 422);s.store_primal_div_scaled_product_add_scaled_denominator_indices(428, 191, 266, 2.0, 425, 1.0, 427, 1.0, 1.0);}
        s.b[541] = (s.v[191] < s.v[262]);s.store_scalar(541, if s.b[541] { 1.0 } else { 0.0 });s.b[542] = ((((0.5 * (s.v[191] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(542, if s.b[542] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[542]) {s.store_primal_exp_scaled_input(430, 191, (s.v[85] * 0.5));}
        s.b[543] = ((0.5 * (s.v[191] * s.v[85])) < (-230.25850929940458));s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && s.b[543]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(430, 1e-100, (-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && (!s.b[543])) {s.store_primal_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(191), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(191), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(191), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && s.b[541]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[544] = (s.v[62] < p.p85);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[544])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[545] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(545, if s.b[545] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[545]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[546] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(546, if s.b[546] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && s.b[546]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && s.b[541]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[547] = (s.v[64] < p.p85);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[548] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[548]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[549] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && s.b[549]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && (!s.b[549])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && s.b[541]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[550] = (s.v[63] < p.p85);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[551] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[551]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[552] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && s.b[552]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && (!s.b[552])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && (!s.b[541])) {s.store_primal_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(191), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[553] = (s.v[62] < p.p85);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[553])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[554] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[554]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[555] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && s.b[555]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && (!s.b[541])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[556] = (s.v[64] < p.p85);s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[557] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[557]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[558] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && s.b[558]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && (!s.b[558])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && (!s.b[541])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[559] = (s.v[63] < p.p85);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[560] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[560]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[561] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && s.b[561]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && (!s.b[561])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[540]) && (!s.b[541])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(372, 283, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);}
        if (s.b[418] && s.b[540]) {s.store_offset(370, 370, (-1.0));s.store_offset(371, 371, (-1.0));s.store_offset(372, 372, (-1.0));s.store_primal_div_from_scalar(429, 1.0, 430);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        s.b[562] = (s.v[191] > 0.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if ((s.b[418] && s.b[540]) && s.b[562]) {s.store_primal_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));}
        if ((s.b[418] && s.b[540]) && (!s.b[562])) {s.store_primal_sub_mixed_ai(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 191);}
        if (s.b[418] && s.b[540]) {s.store_primal_sub(432, 264, 431);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(433, 191, 0.5, 432, 0.5, 191, 432, ((4.0 * s.v[84]) * s.v[84]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(434, 191, 0.5, 267, 0.5, A::add_scaled_square_product(A::sub(s.ad_value(191), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0), (-0.5));s.store_primal_scaled_sub_mixed_ia(435, 191, A::sqrt_square_offset(s.ad_value(191), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        if (s.b[418] && (!s.b[540])) {s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(431, 0.0);s.store_scalar(428, 0.0);s.store_scalar(430, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);}
        s.b[563] = (s.v[256] == 0.0);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[563]) {s.store_scalar(268, 0.0);s.store_scalar(291, 0.0);s.store_scalar(269, 0.0);}
        s.b[564] = (s.v[122] == 0.5);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[563])) && s.b[564]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));}
        if ((s.b[418] && (!s.b[563])) && (!s.b[564])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);}
        if (s.b[418] && (!s.b[563])) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);s.store_mul(437, 101, 370);}
        s.b[565] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[563])) && s.b[565]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {s.store_primal_sub(439, 107, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[566] = (s.v[9] == 0.5);s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[566]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[566])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));}
        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {s.store_primal_add(442, 440, 441);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[567] = (s.v[9] == 0.5);s.store_scalar(567, if s.b[567] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[567]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[143]);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[567])) {s.store_primal_powf_scaled_input(436, 439, s.v[143], s.v[9]);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {s.store_primal_scale(443, 436, s.v[137]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 98, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[20]);}
        s.b[568] = (s.v[23] == 0.0);s.store_scalar(568, if s.b[568] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[563])) && s.b[568]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[122] * s.v[152]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[569] = (((-s.v[9]) * s.v[125]) == (-1.0));s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[569]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[569])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));}
        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[149]), 447, 450, s.v[149], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[570] = (s.v[457] > 0.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[570]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[570])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[571] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[571]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[571])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[572] = (s.v[457] > 0.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[572]) {s.copy_ad(458, 421);}
        s.b[573] = (s.v[456] > (-230.25850929940458));s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && s.b[573]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && (!s.b[573])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[149] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);}
        s.b[574] = (s.v[29] == 0.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[563])) && s.b[574]) {s.store_scalar(460, 0.0);}
        s.b[575] = (s.v[9] == 0.5);s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[575]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[575])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[143]), ((s.v[6]) * (s.v[143])), s.v[9]);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), 436, 1.0);}
        s.b[576] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[576]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));}
        s.b[577] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && s.b[577]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 155, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && (!s.b[577])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 155, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[29], 0.0);}
        s.b[578] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[563])) && s.b[578]) {s.store_scalar(462, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.b[579] = (s.v[435] > ((-s.v[158]) * s.v[38]));s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });s.b[580] = (s.v[41] == 4.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && s.b[580]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));}
        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && (!s.b[580])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);}
        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && (!s.b[579])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);}
        if (s.b[418] && (!s.b[563])) {s.store_mul_add_scaled_inputs4_indices_rhs(268, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(291, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[581] = (s.v[257] == 0.0);s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[581]) {s.store_scalar(270, 0.0);s.store_scalar(292, 0.0);s.store_scalar(271, 0.0);}
        s.b[582] = (s.v[123] == 0.5);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[581])) && s.b[582]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));}
        if ((s.b[418] && (!s.b[581])) && (!s.b[582])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);}
        if (s.b[418] && (!s.b[581])) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);s.store_mul(437, 102, 371);}
        s.b[583] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[581])) && s.b[583]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {s.store_primal_sub(439, 108, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[584] = (s.v[10] == 0.5);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[584]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[584])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));}
        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {s.store_primal_add(442, 440, 441);}
        s.b[585] = (s.v[10] == 0.5);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[585]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[144]);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[585])) {s.store_primal_powf_scaled_input(436, 439, s.v[144], s.v[10]);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {s.store_primal_scale(443, 436, s.v[138]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 99, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[21]);}
        s.b[586] = (s.v[24] == 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[581])) && s.b[586]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[123] * s.v[153]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[587] = (((-s.v[10]) * s.v[126]) == (-1.0));s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[587]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[587])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));}
        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[150]), 447, 450, s.v[150], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[588] = (s.v[457] > 0.0);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[588]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[588])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[589] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[589]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[589])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[590] = (s.v[457] > 0.0);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[590]) {s.copy_ad(458, 421);}
        s.b[591] = (s.v[456] > (-230.25850929940458));s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && s.b[591]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && (!s.b[591])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[150] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);}
        s.b[592] = (s.v[30] == 0.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[581])) && s.b[592]) {s.store_scalar(460, 0.0);}
        s.b[593] = (s.v[10] == 0.5);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[593]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[593])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[144]), ((s.v[7]) * (s.v[144])), s.v[10]);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), 436, 1.0);}
        s.b[594] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[594]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));}
        s.b[595] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && s.b[595]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 156, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && (!s.b[595])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 156, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[30], 0.0);}
        s.b[596] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[581])) && s.b[596]) {s.store_scalar(462, 1.0);}
        s.b[597] = (s.v[435] > ((-s.v[158]) * s.v[39]));s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });s.b[598] = (s.v[42] == 4.0);s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && s.b[598]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));}
        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && (!s.b[598])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);}
        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && (!s.b[597])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if (s.b[418] && (!s.b[581])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[599] = (s.v[258] == 0.0);s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[599]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
        s.b[600] = (s.v[124] == 0.5);s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[599])) && s.b[600]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));}
        if ((s.b[418] && (!s.b[599])) && (!s.b[600])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);}
        if (s.b[418] && (!s.b[599])) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);s.store_mul(437, 103, 372);}
        s.b[601] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[599])) && s.b[601]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {s.store_primal_sub(439, 109, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[602] = (s.v[11] == 0.5);s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[602]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[602])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));}
        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {s.store_primal_add(442, 440, 441);}
        s.b[603] = (s.v[11] == 0.5);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[603]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[145]);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[603])) {s.store_primal_powf_scaled_input(436, 439, s.v[145], s.v[11]);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {s.store_primal_scale(443, 436, s.v[139]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 100, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[22]);}
        s.b[604] = (s.v[25] == 0.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[599])) && s.b[604]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[124] * s.v[154]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {s.store_primal_mul(451, 449, 450);}
        s.b[605] = (((-s.v[11]) * s.v[127]) == (-1.0));s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[605]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[605])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));}
        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[151]), 447, 450, s.v[151], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[606] = (s.v[457] > 0.0);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[606]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[606])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[607] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[607]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[607])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[608] = (s.v[457] > 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[608]) {s.copy_ad(458, 421);}
        s.b[609] = (s.v[456] > (-230.25850929940458));s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && s.b[609]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && (!s.b[609])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[151] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);}
        s.b[610] = (s.v[31] == 0.0);s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[599])) && s.b[610]) {s.store_scalar(460, 0.0);}
        s.b[611] = (s.v[11] == 0.5);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[611]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[611])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[145]), ((s.v[8]) * (s.v[145])), s.v[11]);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), 436, 1.0);}
        s.b[612] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[612]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));}
        s.b[613] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && s.b[613]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 157, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && (!s.b[613])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 157, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[31], 0.0);}
        s.b[614] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[599])) && s.b[614]) {s.store_scalar(462, 1.0);}
        s.b[615] = (s.v[435] > ((-s.v[158]) * s.v[40]));s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });s.b[616] = (s.v[43] == 4.0);s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && s.b[616]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && (!s.b[616])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);}
        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && (!s.b[615])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if (s.b[418] && (!s.b[599])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        if s.b[418] {s.store_add_scaled_inputs3_indices(181, 268, s.v[256], 270, s.v[257], 272, s.v[258]);}
        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[617]) {s.store_primal_scaled_mul(422, 265, 265, 4.0);s.store_primal_div(423, 265, 266);s.store_primal_add_scaled_product_indices(424, 192, 1.0, 265, 423, 1.0);s.store_primal_add(425, 266, 424);s.store_primal_sub(426, 266, 424);s.store_primal_sqrt_square_add(427, 426, 422);s.store_primal_div_scaled_product_add_scaled_denominator_indices(428, 192, 266, 2.0, 425, 1.0, 427, 1.0, 1.0);}
        s.b[618] = (s.v[192] < s.v[262]);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = ((((0.5 * (s.v[192] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[619]) {s.store_primal_exp_scaled_input(430, 192, (s.v[85] * 0.5));}
        s.b[620] = ((0.5 * (s.v[192] * s.v[85])) < (-230.25850929940458));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && s.b[620]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(430, 1e-100, (-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && (!s.b[620])) {s.store_primal_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(192), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(192), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(192), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if ((s.b[418] && s.b[617]) && s.b[618]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[621] = (s.v[62] < p.p85);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {s.store_offset_sub_scaled_inputs_indices(360, 192, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[621])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[622] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[622]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[623] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && s.b[623]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[617]) && s.b[618]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[624] = (s.v[64] < p.p85);s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {s.store_offset_sub_scaled_inputs_indices(360, 192, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[625] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[625]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[626] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
    }
}
