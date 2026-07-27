#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1285] {s.store_scalar(336, (1.0 / s.v[764]));s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p[260], 336, (-p[260]), A::square(s.ad_value(335)), p[261], A::square(s.ad_value(336)), (-p[261]), (s.v[616] + p[259]));s.store_sqrt(192, 337);s.store_mul(193, 337, 192);s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);s.store_div_from_scalar(155, 1.0, 154);s.store_square(156, 154);s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);s.store_exp_scaled_input(335, 590, s.v[480]);s.store_div(679, 335, 573);}
        s.b[1286] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1286]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1288] = (s.v[973] < 1000.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1286]) && s.b[1288]) {s.store_scalar(973, 1000.0);}
        if (s.b[1285] && s.b[1286]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p[382]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        s.b[1289] = (s.v[963] == 3.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1291] = (s.v[973] < 1000.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((s.b[1285] && (!s.b[1286])) && s.b[1289]) && s.b[1291]) {s.store_scalar(973, 1000.0);}
        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_offset_scaled(976, 676, p[365], (((((-1.0)) * (p[365]))) + (p[364])));}
        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {s.store_scalar(961, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));s.store_scalar(977, 0.0);}
        if s.b[1285] {s.store_mul(680, 638, 155);s.store_scale(335, 387, 1.0 / (s.v[764]));s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));}
        s.b[1292] = (p[39] != 2.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1292]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p[90], 1.0), 1.0, s.ad_value(390), p[91]));}
        if (s.b[1285] && (!s.b[1292])) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p[90], 1.0), 1.0, s.ad_value(392), p[91]));}
        s.b[1294] = (p[39] != 2.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1294]) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(389), p[324], 1.0), s.v[627], 390, (p[325] * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(389), p[390], 1.0), 1.0, 390, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        if (s.b[1285] && (!s.b[1294])) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(391), p[324], 1.0), s.v[627], 392, (p[325] * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(391), p[390], 1.0), 1.0, 392, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        s.b[1296] = (s.v[682] < 0.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1296]) {s.store_scalar(682, 0.0);}
        s.b[1298] = (s.v[688] < 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1298]) {s.store_scalar(688, 0.0);}
        s.b[1300] = (s.v[689] < 0.0);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1300]) {s.store_scalar(689, 0.0);}
        if (s.b[1285] && (p[53] != 0.0)) {s.store_add_scaled_inputs_mixed_ai(766, A::scale_offset(s.ad_value(389), p[328], s.v[541]), s.v[675], 390, (p[329] * s.v[675]));}
        s.b[1302] = (s.v[766] < 0.0001);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (p[53] != 0.0)) && s.b[1302]) {s.store_scalar(766, 0.0001);}
        if s.b[1285] {s.store_add_scaled_inputs_mixed_ai(336, A::scale_offset(s.ad_value(389), p[330], s.v[529]), 1.0, 390, p[331]);s.store_offset(781, 336, (-0.05));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1285] {s.store_scalar(782, 0.0);}
        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1285] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1285] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div(684, 335, 586);s.store_sqrt_div(685, 335, 621);}
        s.b[1303] = (s.v[963] == 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1303]) {s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div(335, 394, 586);s.store_square(210, 335);}
        s.b[1304] = (s.v[963] == 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });s.b[1305] = (s.v[459] != 0.0);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1304]) && s.b[1305]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));}
        s.b[1306] = (s.v[460] != 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1304]) && s.b[1306]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));}
        s.b[1307] = (s.v[459] != 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (!s.b[1304])) && s.b[1307]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));}
        s.b[1308] = (s.v[460] != 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (!s.b[1304])) && s.b[1308]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));}
        s.b[1309] = (s.v[449] == 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });s.b[1310] = (s.v[530] > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1310]) {s.store_scale(336, 645, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));}
        s.b[1311] = (p[39] == 1.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, 390, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, 392, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if ((s.b[1285] && s.b[1309]) && (!s.b[1310])) {s.store_scalar(690, 0.0);}
        s.b[1312] = (s.v[540] > 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1312]) {s.store_scale(336, 645, ((((p[69] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[70] * p[100]) * 1000000.0) + p[101])));}
        s.b[1313] = (p[39] == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, 390, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, 392, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if ((s.b[1285] && s.b[1309]) && (!s.b[1312])) {s.store_scalar(691, 0.0);}
        s.b[1314] = (s.v[538] > 0.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_scale(338, 646, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));s.store_scalar(335, (((1.0 - s.v[535]) * p[63]) * 1000000.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_scalar(782, ((((p[99] * p[99]) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p[99], s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(336, 782, p[99], 0.5);}
        s.b[1315] = (s.v[336] < 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1315]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_div_from_scalar(342, (-p[98]), 336);s.store_offset_scaled(337, 342, (p[63] * 1000000.0), ((1.0) + (p[98])));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p[98] + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p[98] + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p[98] + 1.0), 781, (-0.5), 782, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1316] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_scale(338, 646, ((((p[69] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[70] * p[100]) * 1000000.0) + p[101])));s.store_scalar(335, (((1.0 - s.v[535]) * p[66]) * 1000000.0));s.store_offset_scaled(337, 342, (p[66] * 1000000.0), ((1.0) + (p[98])));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p[98] + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p[98] + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p[98] + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1317] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1285] && s.b[1309]) && (!s.b[1314])) {s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);}
        if s.b[1285] {s.store_scaled_sqrt(139, 155, s.v[639]);s.store_square(694, 139);s.store_scaled_square(140, 394, s.v[640]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1285] {s.store_offset_scaled(427, 391, p[448], p[447]);s.store_scalar(957, p[193]);}
        s.b[1320] = (s.v[957] < 0.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1320]) {s.store_scalar(957, 0.0);}
        s.b[1321] = (s.v[957] > 0.005);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1321]) {s.store_scalar(957, 0.005);}
        s.b[1322] = (s.v[449] > 0.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[416]);
            }
        }
        if (s.b[1285] && s.b[1322]) {s.store_div_from_scalar(794, s.v[569], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[418]), p[418]));s.store_div_from_scalar(795, s.v[570], 334);s.store_offset_scaled(959, 387, p[439], (((((-s.v[764])) * (p[439]))) + (s.v[959])));}
        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[415]);
            }
        }
        if (s.b[1285] && s.b[1322]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[417]), p[417]));s.store_div_from_scalar(788, s.v[568], 334);s.store_offset_scaled(956, 387, p[438], (((((-s.v[764])) * (p[438]))) + (s.v[956])));}
        s.b[1324] = (s.v[956] < 0.1);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1322]) && s.b[1324]) {s.store_scalar(956, 0.1);}
        if s.b[1285] {s.store_square(334, 676);s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (p[498])), p[495]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[497])), s.v[819]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[509]), 1.0 / (p[498])), p[495]);}
        s.b[1325] = (p[48] > 0.0);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });s.b[1326] = (p[15] > s.v[632]);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && s.b[1325]) && s.b[1326]) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, (p[15] - s.v[632]));s.store_scale(876, 831, (p[15] - s.v[632]));s.store_scale(877, 836, s.v[632]);s.store_scale(878, 837, s.v[632]);}
        if ((s.b[1285] && s.b[1325]) && (!s.b[1326])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scale(877, 836, p[15]);s.store_scale(878, 837, p[15]);}
        if (s.b[1285] && (!s.b[1325])) {s.store_scale(873, 828, p[13]);s.store_scale(874, 830, p[13]);s.store_scale(875, 829, p[15]);s.store_scale(876, 831, p[15]);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);}
        if s.b[1285] {s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);}
        s.b[1327] = (s.v[847] > 0.0);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1327]) {s.store_offset(336, 847, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p[512]);s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));s.store_exp_mul(851, 848, 850);}
        if s.b[1285] {s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[522]), 1.0 / (p[521])), p[518]);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[520])), s.v[824]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[532]), 1.0 / (p[521])), p[518]);}
        s.b[1328] = (p[48] > 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });s.b[1329] = (p[16] > s.v[632]);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && s.b[1328]) && s.b[1329]) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, (p[16] - s.v[632]));s.store_scale(882, 831, (p[16] - s.v[632]));s.store_scale(883, 836, s.v[632]);s.store_scale(884, 837, s.v[632]);}
        if ((s.b[1285] && s.b[1328]) && (!s.b[1329])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scale(883, 836, p[16]);s.store_scale(884, 837, p[16]);}
        if (s.b[1285] && (!s.b[1328])) {s.store_scale(879, 828, p[14]);s.store_scale(880, 830, p[14]);s.store_scale(881, 829, p[16]);s.store_scale(882, 831, p[16]);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);}
        if s.b[1285] {s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);}
        s.b[1330] = (s.v[852] > 0.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1330]) {s.store_offset(337, 852, 1e-25);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1330]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p[535]);s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));s.store_exp_mul(856, 853, 855);}
        if s.b[1285] {s.store_offset_scaled(832, 391, ((p[481]) * ((p[500] * p[13]))), (p[500] * p[13]));}
        s.b[1331] = (p[15] > s.v[632]);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1331]) {s.store_offset_scaled(833, 391, ((p[483]) * ((p[501] * (p[15] - s.v[632])))), (p[501] * (p[15] - s.v[632])));s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * s.v[632]))), (p[502] * s.v[632]));}
        if (s.b[1285] && (!s.b[1331])) {s.store_scalar(833, 0.0);s.store_offset_scaled(834, 391, ((p[485]) * ((p[502] * p[15]))), (p[502] * p[15]));}
        s.b[1332] = (s.v[832] < 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1332]) {s.store_scalar(832, 0.0);}
        s.b[1333] = (s.v[833] < 0.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1333]) {s.store_scalar(833, 0.0);}
        s.b[1334] = (s.v[834] < 0.0);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1334]) {s.store_scalar(834, 0.0);}
        if s.b[1285] {s.store_sub_from_scalar_scaled_input(841, p[506], 391, p[487]);s.store_sub_from_scalar_scaled_input(842, p[507], 391, p[489]);s.store_sub_from_scalar_scaled_input(843, p[508], 391, p[491]);}
        s.b[1335] = ((s.v[841] < 0.01) && (p[13] > 0.0));s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1335]) {s.store_scalar(841, 0.01);}
        s.b[1336] = ((s.v[842] < 0.01) && (p[15] > s.v[632]));s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1336]) {s.store_scalar(842, 0.01);}
        s.b[1337] = ((s.v[843] < 0.01) && (p[15] > 0.0));s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1337]) {s.store_scalar(843, 0.01);}
        if s.b[1285] {s.store_offset_scaled(835, 391, ((p[482]) * ((p[523] * p[14]))), (p[523] * p[14]));}
    }
}
