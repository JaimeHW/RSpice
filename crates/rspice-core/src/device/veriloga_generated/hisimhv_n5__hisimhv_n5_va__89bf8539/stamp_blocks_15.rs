#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.b[1345] = ((s.v[845] < 0.01) && (p[16] > s.v[632]));s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1287] && s.b[1345]) {s.store_scalar(845, 0.01);}
        s.b[1346] = ((s.v[846] < 0.01) && (p[16] > 0.0));s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (s.b[1287] && s.b[1346]) {s.store_scalar(846, 0.01);}
        s.store_scaled_voltage(729, ctx, nodes, Some(6), Some(8), p[87]);s.store_scaled_voltage(731, ctx, nodes, Some(7), Some(8), p[87]);s.store_scaled_voltage(728, ctx, nodes, Some(9), Some(8), p[87]);s.store_scaled_voltage(733, ctx, nodes, Some(0), Some(2), p[87]);s.store_scaled_voltage(734, ctx, nodes, Some(7), Some(2), p[87]);s.store_scaled_voltage(735, ctx, nodes, Some(9), Some(2), p[87]);s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(6), p[87]);s.store_scaled_voltage(804, ctx, nodes, Some(8), Some(2), p[87]);s.store_scaled_voltage(857, ctx, nodes, Some(11), Some(2), p[87]);s.store_scaled_voltage(858, ctx, nodes, Some(10), Some(0), p[87]);s.store_scaled_voltage(865, ctx, nodes, Some(9), Some(8), p[87]);s.store_scaled_voltage(866, ctx, nodes, Some(9), Some(6), p[87]);s.copy_ad(859, 857);s.copy_ad(860, 858);s.copy_ad(867, 865);s.copy_ad(868, 866);s.store_scaled_voltage(798, ctx, nodes, Some(4), Some(2), p[87]);
        if (s.v[81] != 0.0) {s.store_voltage(747, ctx, nodes, Some(12), None);s.store_voltage(748, ctx, nodes, Some(13), None);}
        if (s.v[81] == 0.0) {s.store_scalar(747, 0.0);s.store_scalar(748, 0.0);}
        s.store_sub(730, 731, 729);s.store_sub(727, 728, 729);s.b[1347] = (s.v[729] >= 0.0);s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if s.b[1347] {s.store_scalar(949, 1.0);s.copy_ad(790, 729);s.copy_ad(791, 731);s.copy_ad(792, 728);s.copy_ad(793, 733);s.copy_ad(796, 734);s.copy_ad(797, 735);}
        if (!s.b[1347]) {s.store_scalar(949, (-1.0));s.store_neg(790, 729);s.copy_ad(791, 730);s.copy_ad(792, 727);s.store_neg(793, 733);s.store_sub(796, 734, 733);s.store_sub(797, 735, 733);}
        s.b[1350] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if s.b[1350] {s.store_voltage(732, ctx, nodes, Some(5), None);}
        s.b[1351] = (p[53] == 2.0);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1351]) {s.store_offset_sub_from_scalar_ad(781, p[433], s.ad_value(732), (-(p[337] * 10.0)));s.store_scalar(782, ((4.0 * p[433]) * (p[337] * 10.0)));}
        if (s.b[1350] && s.b[1351]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1350] && s.b[1351]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p[433]);}
        if s.b[1350] {s.store_scalar(387, (ctx_temp + p[11]));s.copy_ad(388, 387);s.store_add(387, 387, 732);s.store_offset(389, 388, (-s.v[764]));s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));s.store_offset(391, 387, (-s.v[764]));s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));s.store_scale(676, 387, 1.0 / (s.v[764]));s.store_ln(590, 676);s.store_sub_scaled_inputs_mixed_ai(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 1.0, 392, s.v[456]);s.store_sqrt(677, 393);s.store_div_from_scalar(335, 1.0, 387);s.store_scalar(336, (1.0 / s.v[764]));s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p[260], 336, (-p[260]), A::square(s.ad_value(335)), p[261], A::square(s.ad_value(336)), (-p[261]), (s.v[616] + p[259]));s.store_sqrt(192, 337);s.store_mul(193, 337, 192);s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);s.store_div_from_scalar(155, 1.0, 154);s.store_square(156, 154);s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);s.store_exp_scaled_input(335, 590, s.v[480]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1350] {s.store_div(679, 335, 573);}
        s.b[1353] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1353]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1355] = (s.v[973] < 1000.0);s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1353]) && s.b[1355]) {s.store_scalar(973, 1000.0);}
        if (s.b[1350] && s.b[1353]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_div_mixed_ia(970, 970, A::powf(s.ad_value(676), p[382]));}
        s.b[1356] = (s.v[963] == 3.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if ((s.b[1350] && (!s.b[1353])) && s.b[1356]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1358] = (s.v[973] < 1000.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if (((s.b[1350] && (!s.b[1353])) && s.b[1356]) && s.b[1358]) {s.store_scalar(973, 1000.0);}
        if ((s.b[1350] && (!s.b[1353])) && s.b[1356]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_offset_scaled(976, 676, p[365], (((((-1.0)) * (p[365]))) + (p[364])));}
        if ((s.b[1350] && (!s.b[1353])) && (!s.b[1356])) {s.store_scalar(961, 0.0);s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));s.store_scalar(977, 0.0);}
        if s.b[1350] {s.store_mul(680, 638, 155);s.store_scale(335, 387, 1.0 / (s.v[764]));s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));}
        s.b[1359] = (p[39] != 2.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1359]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p[90], 1.0), 1.0, s.ad_value(390), p[91]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1350] && (!s.b[1359])) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p[90], 1.0), 1.0, s.ad_value(392), p[91]));}
        s.b[1361] = (p[39] != 2.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1361]) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(389), p[324], 1.0), s.v[627], 390, (p[325] * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(389), p[390], 1.0), 1.0, 390, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        if (s.b[1350] && (!s.b[1361])) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(391), p[324], 1.0), s.v[627], 392, (p[325] * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(391), p[390], 1.0), 1.0, 392, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        s.b[1363] = (s.v[682] < 0.0);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1363]) {s.store_scalar(682, 0.0);}
        s.b[1365] = (s.v[688] < 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1365]) {s.store_scalar(688, 0.0);}
        s.b[1367] = (s.v[689] < 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1367]) {s.store_scalar(689, 0.0);}
        if (s.b[1350] && (p[53] != 0.0)) {s.store_add_scaled_inputs_mixed_ai(766, A::scale_offset(s.ad_value(389), p[328], s.v[541]), s.v[675], 390, (p[329] * s.v[675]));}
        s.b[1369] = (s.v[766] < 0.0001);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((s.b[1350] && (p[53] != 0.0)) && s.b[1369]) {s.store_scalar(766, 0.0001);}
        if s.b[1350] {s.store_add_scaled_inputs_mixed_ai(336, A::scale_offset(s.ad_value(389), p[330], s.v[529]), 1.0, 390, p[331]);s.store_offset(781, 336, (-0.05));s.store_scalar(782, 0.0);}
        if s.b[1350] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1350] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1350] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1350] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div(684, 335, 586);s.store_sqrt_div(685, 335, 621);}
        s.b[1370] = (s.v[963] == 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1370]) {s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div(335, 394, 586);s.store_square(210, 335);}
        s.b[1371] = (s.v[963] == 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });s.b[1372] = (s.v[459] != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1371]) && s.b[1372]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));}
        s.b[1373] = (s.v[460] != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1371]) && s.b[1373]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));}
        s.b[1374] = (s.v[459] != 0.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((s.b[1350] && (!s.b[1371])) && s.b[1374]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));}
        s.b[1375] = (s.v[460] != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((s.b[1350] && (!s.b[1371])) && s.b[1375]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1376] = (s.v[449] == 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });s.b[1377] = (s.v[530] > 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1376]) && s.b[1377]) {s.store_scale(336, 645, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));}
        s.b[1378] = (p[39] == 1.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, 390, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, 392, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if ((s.b[1350] && s.b[1376]) && (!s.b[1377])) {s.store_scalar(690, 0.0);}
        s.b[1379] = (s.v[540] > 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1376]) && s.b[1379]) {s.store_scale(336, 645, ((((p[69] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[70] * p[100]) * 1000000.0) + p[101])));}
        s.b[1380] = (p[39] == 1.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, 390, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, 392, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if ((s.b[1350] && s.b[1376]) && (!s.b[1379])) {s.store_scalar(691, 0.0);}
        s.b[1381] = (s.v[538] > 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_scale(338, 646, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));s.store_scalar(335, (((1.0 - s.v[535]) * p[63]) * 1000000.0));s.store_scalar(782, ((((p[99] * p[99]) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p[99], s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(336, 782, p[99], 0.5);}
        s.b[1382] = (s.v[336] < 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1382]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_div_from_scalar(342, (-p[98]), 336);s.store_offset_scaled(337, 342, (p[63] * 1000000.0), ((1.0) + (p[98])));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p[98] + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p[98] + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p[98] + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_scalar(782, 0.0);}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1383] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_scale(338, 646, ((((p[69] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[70] * p[100]) * 1000000.0) + p[101])));s.store_scalar(335, (((1.0 - s.v[535]) * p[66]) * 1000000.0));s.store_offset_scaled(337, 342, (p[66] * 1000000.0), ((1.0) + (p[98])));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p[98] + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p[98] + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p[98] + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1384] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1350] && s.b[1376]) && (!s.b[1381])) {s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);}
        if s.b[1350] {s.store_scaled_sqrt(139, 155, s.v[639]);s.store_square(694, 139);s.store_scaled_square(140, 394, s.v[640]);s.store_offset_scaled(427, 391, p[448], p[447]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1350] {s.store_scalar(957, p[193]);}
        s.b[1387] = (s.v[957] < 0.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1387]) {s.store_scalar(957, 0.0);}
        s.b[1388] = (s.v[957] > 0.005);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if (s.b[1350] && s.b[1388]) {s.store_scalar(957, 0.005);}
        if (!s.b[1350]) {s.store_scalar(387, (ctx_temp + p[11]));}
        s.store_scalar(164, (s.v[630] * p[7]));s.store_scalar(165, (p[67] + p[68]));s.store_scalar(160, s.v[462]);s.copy_ad(257, 681);s.store_scalar(161, s.v[617]);s.store_scalar(187, p[95]);s.store_scalar(188, (s.v[161] / s.v[187]));s.store_scalar(189, (1.0 / s.v[188]));s.store_primal_div_from_scalar(412, s.v[161], 543);s.store_scalar(270, (p[87] * p[434]));s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p[262])), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, (-0.5), 782, (-0.5), 0.8);s.copy_ad(69, 335);s.b[1389] = ((s.v[158] - p[262]) < s.v[69]);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if s.b[1389] {s.store_offset(69, 158, (-p[262]));}
        s.b[1390] = ((s.v[159] - p[262]) < s.v[69]);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if s.b[1390] {s.store_offset(69, 159, (-p[262]));}
        s.b[1391] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });s.b[1392] = ((s.v[961] - p[262]) < s.v[69]);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (s.b[1391] && s.b[1392]) {s.store_offset(69, 961, (-p[262]));}
        s.b[1393] = ((s.v[960] - p[262]) < s.v[69]);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if (s.b[1391] && s.b[1393]) {s.store_offset(69, 960, (-p[262]));}
        s.b[1394] = (s.v[70] > (s.v[69] * 0.5));s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if s.b[1394] {s.store_scale(70, 69, 0.5);}
        s.b[1395] = param_given[338];s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if s.b[1395] {s.store_scalar(72, p[338]);}
        if (!s.b[1395]) {s.copy_ad(72, 69);}
        s.b[1396] = param_given[339];s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if s.b[1396] {s.store_scalar(73, p[339]);}
        s.b[1397] = param_given[338];s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if ((!s.b[1396]) && s.b[1397]) {s.store_scale(73, 72, 0.5);}
        if ((!s.b[1396]) && (!s.b[1397])) {s.copy_ad(73, 70);}
        s.b[1398] = (s.v[73] > (s.v[72] * 0.5));s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if s.b[1398] {s.store_scale(73, 72, 0.5);}
        s.b[1399] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });s.b[1400] = (s.v[448] == 1.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if (s.b[1399] && s.b[1400]) {s.store_scalar(74, 1.0);}
        s.b[1401] = (s.v[448] == 2.0);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if (s.b[1399] && s.b[1401]) {s.store_scalar(74, 2.0);}
        s.b[1402] = (s.v[448] == 3.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (s.b[1399] && s.b[1402]) {s.store_scalar(74, 3.0);}
        s.store_scalar(77, 0.0);s.b[1403] = (((s.v[449] == 1.0) && (p[54] == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if s.b[1403] {s.copy_ad(373, 733);}
        s.b[1404] = (s.v[373] >= 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (s.b[1403] && s.b[1404]) {s.copy_ad(376, 373);s.copy_ad(383, 798);}
        if (s.b[1403] && (!s.b[1404])) {s.store_neg(376, 373);s.store_sub(383, 798, 373);}
        if s.b[1403] {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1403] {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1405] = (s.v[108] < 1e-12);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if (s.b[1403] && s.b[1405]) {s.store_scalar(108, 1e-12);}
        if s.b[1403] {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p[335], A::scale(s.ad_value(380), p[333])), 1.0, 383, p[332]);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1406] = (s.v[335] < 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (s.b[1403] && s.b[1406]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if s.b[1403] {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p[334], s.ad_value(384), (-(0.1 * p[334])));s.store_scalar(782, ((4.0 * p[334]) * (0.1 * p[334])));}
        if s.b[1403] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1403] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p[334]);}
        if (!s.b[1403]) {s.store_scalar(384, 0.0);}
        s.b[1407] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if s.b[1407] {s.copy_ad(373, 733);s.copy_ad(374, 734);s.copy_ad(372, 735);}
        s.b[1408] = (s.v[373] >= 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (s.b[1407] && s.b[1408]) {s.store_scalar(370, 1.0);s.store_scalar(371, 0.0);s.copy_ad(376, 373);s.copy_ad(377, 374);s.copy_ad(375, 372);s.copy_ad(383, 798);}
        if (s.b[1407] && (!s.b[1408])) {s.store_scalar(370, 0.0);s.store_scalar(371, 1.0);s.store_neg(376, 373);s.store_sub(377, 374, 373);s.store_sub(375, 372, 373);s.store_sub(383, 798, 373);}
        s.b[1409] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p[54] == 1.0));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if (s.b[1407] && s.b[1409]) {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1410] = (s.v[108] < 1e-12);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if ((s.b[1407] && s.b[1409]) && s.b[1410]) {s.store_scalar(108, 1e-12);}
        if (s.b[1407] && s.b[1409]) {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_add(381, 377, 108);s.store_add(382, 375, 108);}
        s.b[1411] = ((p[34] == 1.0) || (s.v[370] == 1.0));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);s.store_scalar(782, ((((p[292] * p[292]) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p[292], s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(344, 782, p[292], 0.5);}
        s.b[1412] = (s.v[344] < 0.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (((s.b[1407] && s.b[1409]) && s.b[1411]) && s.b[1412]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
        s.b[1413] = (s.v[337] < 0.0);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if (((s.b[1407] && s.b[1409]) && s.b[1411]) && s.b[1413]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 366);s.store_mul(366, 366, 337);}
        if ((s.b[1407] && s.b[1409]) && (!s.b[1411])) {s.copy_ad(366, 691);}
        if (s.b[1407] && s.b[1409]) {s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);}
        s.b[1414] = ((p[34] == 1.0) || (s.v[371] == 1.0));s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p[262]));s.store_scalar(344, (p[292] + 1e-25));s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1415] = (s.v[337] < 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (((s.b[1407] && s.b[1409]) && s.b[1414]) && s.b[1415]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 367);s.store_mul(367, 367, 337);}
        if ((s.b[1407] && s.b[1409]) && (!s.b[1414])) {s.copy_ad(367, 691);}
        s.b[1416] = (((p[54] == 1.0) && (p[34] == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p[335], A::scale(s.ad_value(380), p[333])), 1.0, 383, p[332]);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1417] = (s.v[335] < 0.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if (((s.b[1407] && s.b[1409]) && s.b[1416]) && s.b[1417]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p[334], s.ad_value(384), (-(0.1 * p[334])));s.store_scalar(782, ((4.0 * p[334]) * (0.1 * p[334])));}
        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p[334]);s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p[334], s.ad_value(384));s.store_mul(334, 366, 340);s.store_mul(335, 367, 340);s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);}
        if ((s.b[1407] && s.b[1409]) && (!s.b[1416])) {s.store_scalar(384, 0.0);}
        if (s.b[1407] && s.b[1409]) {s.copy_ad(4, 366);s.copy_ad(5, 367);}
        if (s.b[1407] && (!s.b[1409])) {s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);}
        if s.b[1407] {s.store_scale(4, 4, 1.0 / (s.v[164]));s.store_scale(5, 5, 1.0 / (s.v[164]));s.store_add_scaled_value_products_indices(4, 4, 1.0, 370, 644, 1.0, 371, 648, 1.0);s.store_add_scaled_value_products_indices(5, 5, 1.0, 370, 648, 1.0, 371, 644, 1.0);s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);}
        s.b[1420] = (s.v[792] > s.v[70]);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if s.b[1420] {s.store_sub(335, 792, 70);s.store_sub(336, 69, 70);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(84, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 84, 1.0);s.store_neg(84, 84);s.store_add(83, 70, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(84, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if (!s.b[1420]) {s.copy_ad(83, 792);s.store_scalar(84, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(335, 84, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[262])));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1421] = (s.v[108] < 1e-12);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if s.b[1421] {s.store_scalar(108, 1e-12);}
        s.store_add(105, 83, 108);s.store_add_scaled_inputs(106, 790, 1.0, 108, 2.0);s.store_add(107, 791, 108);s.store_scale(335, 636, (s.v[189] * s.v[189]));s.store_offset(336, 791, (-s.v[160]));s.store_offset_mul_ad(337, A::div_from_scalar(2.0, s.ad_value(335)), A::add_scaled_inputs3(s.ad_value(336), 1.0, A::div_from_scalar(1.0, s.ad_value(678)), (-1.0), s.ad_value(83), -1.0), 1.0);s.store_sqrt_square_offset(782, 337, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(339, 337, 782, 0.5, 0.5);s.store_scaled_add(338, 337, 782, 0.5);s.b[1422] = (s.v[338] < 0.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if s.b[1422] {s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);}
        s.store_offset(338, 338, 1e-25);s.store_sqrt(332, 338);s.store_add_mul_sub_from_scalar_rhs_indices(128, 336, 335, 1.0, 332);s.store_sub(129, 128, 159);s.store_offset(781, 129, (((-0.1)) + ((-0.05))));s.store_scalar(782, ((4.0 * 0.1) * 0.05));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(129, 781, 0.5, 782, 0.5, 0.1);s.store_div(335, 790, 129);s.copy_ad(781, 335);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(332, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(334, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(332), -1.0, 0.0, 332);s.store_sub_from_scalar(332, 1.0, 332);s.store_neg(334, 334);s.store_square(208, 332);s.b[1423] = (s.v[765] == 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if s.b[1423] {s.store_scalar(80, 0.0);}
        if (!s.b[1423]) {s.store_scalar(80, 1.0);}
        s.copy_ad(335, 637);s.store_sqrt_mul(336, 335, 158);s.store_add_scaled_inputs_mixed_ai(190, A::offset(s.ad_value(158), s.v[160]), 1.0, 336, s.v[189]);s.b[1424] = (s.v[80] == 0.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if s.b[1424] {s.store_scalar(183, s.v[187]);s.store_scalar(185, s.v[188]);s.store_scalar(186, s.v[189]);s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1424]) {s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p[236]);s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);s.store_scaled_add(336, 339, 782, 0.5);}
        s.b[1425] = (s.v[336] < 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1425]) {s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[1424]) {s.store_offset(336, 336, 1e-25);s.store_div_from_scalar(337, 1.0, 336);s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));s.store_scaled_abs(338, 190, 2.0);s.store_offset_sub(340, 339, 791, s.v[160]);}
        s.b[1426] = (s.v[340] > s.v[338]);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1426]) {s.copy_ad(338, 340);}
        if (!s.b[1424]) {s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));}
        if (!s.b[1424]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (!s.b[1424]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));s.store_offset_scaled(184, 336, p[235], p[237]);s.store_scalar(341, p[235]);}
        s.b[1427] = ((s.v[184] * 1000000000000.0) < s.v[187]);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1427]) {s.store_scalar(184, 0.0);s.store_scalar(80, 0.0);}
        if (!s.b[1424]) {s.store_offset(183, 184, s.v[187]);s.store_div_from_scalar(185, s.v[161], 183);s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));s.store_scale(186, 183, 1.0 / (s.v[161]));s.store_scalar(335, (1.0 / s.v[161]));s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        s.copy_ad(364, 105);s.copy_ad(335, 637);s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);s.store_add_mixed_ai(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);s.copy_ad(123, 158);s.store_scalar(334, 0.95);s.b[338] = (!(s.v[963] > 1.0));s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));
        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p[366]);
        } else {
            s.store_scalar(339, 0.0);
        }
        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);s.store_sqrt(181, 180);s.b[1428] = (p[140] != 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if s.b[1428] {s.copy_ad(335, 637);s.store_sub_from_scalar(336, p[224], 364);s.store_offset(337, 336, 1e-25);s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));s.store_scaled_add(339, 337, 338, 0.5);s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);s.store_div_from_scalar(341, 1.0, 339);s.store_scale(175, 341, p[223]);s.store_mul_scale_offset_indices(342, 341, 175, -1.0, 0.0);s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));}
        if s.b[1428] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1428] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1428] {s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);s.store_div(343, 334, 176);s.store_mul_sub_lhs(177, 239, 176, 186);s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);s.store_add_scaled_product_mixed_aii(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 685);s.store_scalar(338, (1.0 / (p[140] * p[140])));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_sub(335, 173, 119);s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p[140])), s.v[465]);s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);s.store_offset(178, 106, p[221]);s.store_square(179, 178);s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);}
        if (!s.b[1428]) {s.store_scalar(174, 0.0);}
        s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[582] - p[139]));s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));s.store_mul_scale_offset_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338], 0.0);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_scale_offset_mixed_ia(341, 181, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 0.0);s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);s.store_scalar(335, (s.v[470] / s.v[582]));s.store_offset_scaled(338, 180, s.v[335], s.v[468]);s.store_add_scaled_product_mixed_iia(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p[150], 1.0), s.v[469]);s.store_mul(122, 121, 339);s.store_div_from_scalar(335, 1.0, 185);s.store_square(336, 335);s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));s.store_square(338, 337);s.store_sub(339, 335, 337);s.store_mul_sub_rhs(340, 239, 336, 338);s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);s.store_add_scaled_inputs3_offset_indices(118, 157, 1.0, 336, s.v[189], 120, -1.0, s.v[160]);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_scalar(182, 0.0);s.b[1429] = (s.v[615] == 1.0);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if s.b[1429] {s.copy_ad(341, 107);s.copy_ad(334, 642);s.store_offset(337, 341, (-p[152]));}
        s.b[1430] = (s.v[337] < (-3.0));s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if (s.b[1429] && s.b[1430]) {s.store_scalar(340, 0.0);s.store_scalar(182, 0.0);}
        s.b[1431] = (s.v[337] < 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1429] && (!s.b[1430])) && s.b[1431]) {s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if ((s.b[1429] && (!s.b[1430])) && (!s.b[1431])) {s.store_offset_mul_offset_rhs_mixed_ia(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if s.b[1429] {s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);}
        s.b[1432] = (s.v[182] < 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1429] && s.b[1432]) {s.store_scalar(182, 0.0);s.store_scalar(340, 0.0);}
        if s.b[1429] {s.store_mul(182, 182, 334);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1429] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1429] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);}
        s.b[1439] = (s.v[792] > s.v[73]);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if ((p[37] != 0.0) && s.b[1439]) {s.store_sub(335, 792, 73);s.store_sub(336, 72, 73);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(1434, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1434, 1.0);s.store_neg(1434, 1434);s.store_add(1433, 73, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(1434, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if ((p[37] != 0.0) && (!s.b[1439])) {s.copy_ad(1433, 792);s.store_scalar(1434, 1.0);}
        if (p[37] == 0.0) {s.copy_ad(1433, 792);s.store_scalar(1434, 1.0);}
        s.store_scaled_mul(335, 1434, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[262])));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(1435, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1440] = (s.v[1435] < 1e-12);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if s.b[1440] {s.store_scalar(1435, 1e-12);}
        s.store_add(1436, 1433, 1435);s.store_add_scaled_inputs(1437, 790, 1.0, 1435, 2.0);s.store_add(1438, 791, 1435);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1433, 1.0, s.v[160]);s.b[1441] = (s.v[963] != 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });s.b[1442] = (p[42] == 1.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });s.b[1443] = (p[42] == 2.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });s.b[1444] = (p[42] == 3.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (s.b[1441] && s.b[1442]) {s.copy_ad(1461, 960);s.store_scale(1544, 964, 1.6021918e-19);s.store_square(1543, 964);s.store_scale(1500, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1542, 622, 1.6021918e-19);s.store_scalar(1539, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1540, (1.034943e-10 * 1.034943e-10));s.store_square(1541, 965);s.store_div_from_scalar(1545, (2.0 * 1.034943e-10), 1544);s.store_scale(1546, 1544, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1547, 1544, (2.0 * 1.034943e-10));s.store_div_from_scalar(1548, (2.0 * 1.034943e-10), 1542);s.store_scale(1549, 1542, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1534, 964, 622);s.store_div_from_scalar_offset_input(1533, 1.0, 1534, 1.0);s.store_scalar(1550, (1e-12 * 1000.0));s.store_scalar(1551, (1e-10 * 1000.0));s.store_scalar(1459, 0.0);s.store_scalar(1460, 0.0);s.store_scalar(1473, 0.0);s.store_scalar(1474, 0.0);s.store_scalar(1515, 0.0);s.store_scalar(1516, 0.0);s.store_scalar(1495, 0.0);s.store_scalar(1497, 0.0);s.store_scalar(1496, 0.0);s.store_scalar(1498, 0.0);s.store_scalar(1518, 0.0);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1454, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1457, A::div_scaled_value_by_product(s.ad_value(1454), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1543);s.store_sqrt_mul_ad(1451, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1461), s.ad_value(1433)));}
        s.b[1557] = (s.v[1451] > s.v[965]);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_scalar(1464, 0.0);s.copy_ad(1445, 965);s.store_scalar(1481, 0.0);s.store_sub_mixed_ia(1462, 1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)));s.store_scalar(1509, 0.0);s.copy_ad(1508, 1464);s.copy_ad(1470, 1462);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (150.0 + 1.0);let t5: f64 = if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (s.v[97] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;
            if t6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1558] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1559] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });s.b[1560] = (2.0 == 1.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && s.b[1560]) {s.store_scalar(720, 1.0);}
            s.b[1561] = (2.0 == 2.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && s.b[1561]) {s.store_scalar(720, 2.0);}
            s.b[1562] = (2.0 == 4.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && s.b[1562]) {s.store_scalar(720, 3.0);}
            s.b[1563] = (2.0 == 8.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {s.store_scalar(719, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;
                if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && (!s.b[1559])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {s.store_scalar(334, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_add_scaled_inputs3_indices(335, 1462, 1.0, 1433, (-1.0), 1461, 1.0);}
            s.b[1564] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1565] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = (2.0 == 1.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && s.b[1566]) {s.store_scalar(720, 1.0);}
            s.b[1567] = (2.0 == 2.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && s.b[1567]) {s.store_scalar(720, 2.0);}
            s.b[1568] = (2.0 == 4.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {s.store_scalar(720, 3.0);}
            s.b[1569] = (2.0 == 8.0);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) && s.b[1569]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;
                if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && (!s.b[1565])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1564])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul(1449, 1548, 336);s.store_mul(1495, 1445, 1544);s.store_mul_div_from_scalar_lhs_ad_indices(1527, (-1.034943e-10), 1445, 334);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1529, (-1.034943e-10), 1449, 341);s.store_add_mixed_ai(1483, A::add_scaled_product(s.ad_value(1495), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1481)), 1.0), 1496);s.copy_ad(1485, 185);s.store_add(1486, 1527, 1529);s.store_add_scaled_product_mixed_iia(1484, 1462, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461)), (-1.0));s.store_scalar(1487, 0.0);s.store_scalar(1488, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1570] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1570])) {s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1571] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1462] - s.v[1470])) as f64).abs() <= 1e-12));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1571]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.copy_ad(1508, 1464);s.copy_ad(1470, 1462);s.store_primal_offset(97, 97, 1.0);}
        }
    }
}
