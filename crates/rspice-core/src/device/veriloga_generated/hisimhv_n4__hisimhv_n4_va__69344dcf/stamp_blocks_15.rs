#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1348] && (!s.b[1357])) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));}
        s.b[1359] = (p.p39 != 2.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1359]) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 1.0, 390, p.p391);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        if (s.b[1348] && (!s.b[1359])) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 1.0, 392, p.p391);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        s.b[1361] = (s.v[682] < 0.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1361]) {s.store_scalar(682, 0.0);}
        s.b[1363] = (s.v[688] < 0.0);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1363]) {s.store_scalar(688, 0.0);}
        s.b[1365] = (s.v[689] < 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1365]) {s.store_scalar(689, 0.0);}
        if (s.b[1348] && (p.p53 != 0.0)) {s.store_add_scaled_inputs_mixed_ai(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));}
        s.b[1367] = (s.v[766] < 0.0001);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((s.b[1348] && (p.p53 != 0.0)) && s.b[1367]) {s.store_scalar(766, 0.0001);}
        if s.b[1348] {s.store_add_scaled_inputs_mixed_ai(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 1.0, 390, p.p331);s.store_offset(781, 336, (-0.05));s.store_scalar(782, 0.0);}
        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1348] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1348] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div(684, 335, 586);s.store_sqrt_div(685, 335, 621);}
        s.b[1368] = (s.v[963] == 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1368]) {s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div(335, 394, 586);s.store_square(210, 335);}
        s.b[1369] = (s.v[963] == 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });s.b[1370] = (s.v[459] != 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1369]) && s.b[1370]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));}
        s.b[1371] = (s.v[460] != 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1369]) && s.b[1371]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));}
        s.b[1372] = (s.v[459] != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((s.b[1348] && (!s.b[1369])) && s.b[1372]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));}
        s.b[1373] = (s.v[460] != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((s.b[1348] && (!s.b[1369])) && s.b[1373]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1374] = (s.v[449] == 0.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });s.b[1375] = (s.v[530] > 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1374]) && s.b[1375]) {s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));}
        s.b[1376] = (p.p39 == 1.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, 390, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, 392, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if ((s.b[1348] && s.b[1374]) && (!s.b[1375])) {s.store_scalar(690, 0.0);}
        s.b[1377] = (s.v[540] > 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1374]) && s.b[1377]) {s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));}
        s.b[1378] = (p.p39 == 1.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, 390, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, 392, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if ((s.b[1348] && s.b[1374]) && (!s.b[1377])) {s.store_scalar(691, 0.0);}
        s.b[1379] = (s.v[538] > 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(336, 782, p.p99, 0.5);}
        s.b[1380] = (s.v[336] < 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1380]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_div_from_scalar(342, (-p.p98), 336);s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_scalar(782, 0.0);}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1381] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1382] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1348] && s.b[1374]) && (!s.b[1379])) {s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);}
        if s.b[1348] {s.store_scaled_sqrt(139, 155, s.v[639]);s.store_square(694, 139);s.store_scaled_square(140, 394, s.v[640]);s.store_offset_scaled(427, 391, p.p448, p.p447);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1348] {s.store_scalar(957, p.p193);}
        s.b[1385] = (s.v[957] < 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1385]) {s.store_scalar(957, 0.0);}
        s.b[1386] = (s.v[957] > 0.005);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (s.b[1348] && s.b[1386]) {s.store_scalar(957, 0.005);}
        if (!s.b[1348]) {s.store_scalar(387, (ctx_temp + p.p11));}
        s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(165, (p.p67 + p.p68));s.store_scalar(160, s.v[462]);s.copy_ad(257, 681);s.store_scalar(161, s.v[617]);s.store_scalar(187, p.p95);s.store_scalar(188, (s.v[161] / s.v[187]));s.store_scalar(189, (1.0 / s.v[188]));s.store_primal_div_from_scalar(412, s.v[161], 543);s.store_scalar(270, (p.p87 * p.p434));s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p.p262)), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, (-0.5), 782, (-0.5), 0.8);s.copy_ad(69, 335);s.b[1387] = ((s.v[158] - p.p262) < s.v[69]);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if s.b[1387] {s.store_offset(69, 158, (-p.p262));}
        s.b[1388] = ((s.v[159] - p.p262) < s.v[69]);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if s.b[1388] {s.store_offset(69, 159, (-p.p262));}
        s.b[1389] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });s.b[1390] = ((s.v[961] - p.p262) < s.v[69]);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if (s.b[1389] && s.b[1390]) {s.store_offset(69, 961, (-p.p262));}
        s.b[1391] = ((s.v[960] - p.p262) < s.v[69]);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if (s.b[1389] && s.b[1391]) {s.store_offset(69, 960, (-p.p262));}
        s.b[1392] = (s.v[70] > (s.v[69] * 0.5));s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if s.b[1392] {s.store_scale(70, 69, 0.5);}
        s.b[1393] = param_given[338];s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if s.b[1393] {s.store_scalar(72, p.p338);}
        if (!s.b[1393]) {s.copy_ad(72, 69);}
        s.b[1394] = param_given[339];s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if s.b[1394] {s.store_scalar(73, p.p339);}
        s.b[1395] = param_given[338];s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((!s.b[1394]) && s.b[1395]) {s.store_scale(73, 72, 0.5);}
        if ((!s.b[1394]) && (!s.b[1395])) {s.copy_ad(73, 70);}
        s.b[1396] = (s.v[73] > (s.v[72] * 0.5));s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if s.b[1396] {s.store_scale(73, 72, 0.5);}
        s.b[1397] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });s.b[1398] = (s.v[448] == 1.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if (s.b[1397] && s.b[1398]) {s.store_scalar(74, 1.0);}
        s.b[1399] = (s.v[448] == 2.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if (s.b[1397] && s.b[1399]) {s.store_scalar(74, 2.0);}
        s.b[1400] = (s.v[448] == 3.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if (s.b[1397] && s.b[1400]) {s.store_scalar(74, 3.0);}
        s.store_scalar(77, 0.0);s.b[1401] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if s.b[1401] {s.copy_ad(373, 733);}
        s.b[1402] = (s.v[373] >= 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (s.b[1401] && s.b[1402]) {s.copy_ad(376, 373);s.store_scalar(383, s.v[798]);}
        if (s.b[1401] && (!s.b[1402])) {s.store_neg(376, 373);s.store_sub_from_scalar(383, s.v[798], 373);}
        if s.b[1401] {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1401] {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1403] = (s.v[108] < 1e-12);s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if (s.b[1401] && s.b[1403]) {s.store_scalar(108, 1e-12);}
        if s.b[1401] {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 1.0, 383, p.p332);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1404] = (s.v[335] < 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (s.b[1401] && s.b[1404]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if s.b[1401] {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));}
        if s.b[1401] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1401] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);}
        if (!s.b[1401]) {s.store_scalar(384, 0.0);}
        s.b[1405] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if s.b[1405] {s.copy_ad(373, 733);s.copy_ad(374, 734);s.copy_ad(372, 735);}
        s.b[1406] = (s.v[373] >= 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (s.b[1405] && s.b[1406]) {s.store_scalar(370, 1.0);s.store_scalar(371, 0.0);s.copy_ad(376, 373);s.copy_ad(377, 374);s.copy_ad(375, 372);s.store_scalar(383, s.v[798]);}
        if (s.b[1405] && (!s.b[1406])) {s.store_scalar(370, 0.0);s.store_scalar(371, 1.0);s.store_neg(376, 373);s.store_sub(377, 374, 373);s.store_sub(375, 372, 373);s.store_sub_from_scalar(383, s.v[798], 373);}
        s.b[1407] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (s.b[1405] && s.b[1407]) {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1408] = (s.v[108] < 1e-12);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if ((s.b[1405] && s.b[1407]) && s.b[1408]) {s.store_scalar(108, 1e-12);}
        if (s.b[1405] && s.b[1407]) {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_add(381, 377, 108);s.store_add(382, 375, 108);}
        s.b[1409] = ((p.p34 == 1.0) || (s.v[370] == 1.0));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(344, 782, p.p292, 0.5);}
        s.b[1410] = (s.v[344] < 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1410]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
        s.b[1411] = (s.v[337] < 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1411]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 366);s.store_mul(366, 366, 337);}
        if ((s.b[1405] && s.b[1407]) && (!s.b[1409])) {s.copy_ad(366, 691);}
        if (s.b[1405] && s.b[1407]) {s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);}
        s.b[1412] = ((p.p34 == 1.0) || (s.v[371] == 1.0));s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));s.store_scalar(344, (p.p292 + 1e-25));s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1413] = (s.v[337] < 0.0);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if (((s.b[1405] && s.b[1407]) && s.b[1412]) && s.b[1413]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 367);s.store_mul(367, 367, 337);}
        if ((s.b[1405] && s.b[1407]) && (!s.b[1412])) {s.copy_ad(367, 691);}
        s.b[1414] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 1.0, 383, p.p332);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1415] = (s.v[335] < 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (((s.b[1405] && s.b[1407]) && s.b[1414]) && s.b[1415]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));}
        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));s.store_mul(334, 366, 340);s.store_mul(335, 367, 340);s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);}
        if ((s.b[1405] && s.b[1407]) && (!s.b[1414])) {s.store_scalar(384, 0.0);}
        if (s.b[1405] && s.b[1407]) {s.copy_ad(4, 366);s.copy_ad(5, 367);}
        if (s.b[1405] && (!s.b[1407])) {s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);}
        if s.b[1405] {s.store_scale(4, 4, 1.0 / (s.v[164]));s.store_scale(5, 5, 1.0 / (s.v[164]));s.store_add_scaled_value_products_indices(4, 4, 1.0, 370, 644, 1.0, 371, 648, 1.0);s.store_add_scaled_value_products_indices(5, 5, 1.0, 370, 648, 1.0, 371, 644, 1.0);s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);}
        s.b[1418] = (s.v[792] > s.v[70]);s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if s.b[1418] {s.store_sub(335, 792, 70);s.store_sub(336, 69, 70);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(84, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 84, 1.0);s.store_neg(84, 84);s.store_add(83, 70, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(84, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if (!s.b[1418]) {s.copy_ad(83, 792);s.store_scalar(84, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(335, 84, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1419] = (s.v[108] < 1e-12);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if s.b[1419] {s.store_scalar(108, 1e-12);}
        s.store_add(105, 83, 108);s.store_add_scaled_inputs(106, 790, 1.0, 108, 2.0);s.store_add(107, 791, 108);s.store_scale(335, 636, (s.v[189] * s.v[189]));s.store_offset(336, 791, (-s.v[160]));s.store_offset_mul_ad(337, A::div_from_scalar(2.0, s.ad_value(335)), A::add_scaled_inputs3(s.ad_value(336), 1.0, A::div_from_scalar(1.0, s.ad_value(678)), (-1.0), s.ad_value(83), -1.0), 1.0);s.store_sqrt_square_offset(782, 337, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(339, 337, 782, 0.5, 0.5);s.store_scaled_add(338, 337, 782, 0.5);s.b[1420] = (s.v[338] < 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if s.b[1420] {s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);}
        s.store_offset(338, 338, 1e-25);s.store_sqrt(332, 338);s.store_add_mul_sub_from_scalar_rhs_indices(128, 336, 335, 1.0, 332);s.store_sub(129, 128, 159);s.store_offset(781, 129, (((-0.1)) + ((-0.05))));s.store_scalar(782, ((4.0 * 0.1) * 0.05));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(129, 781, 0.5, 782, 0.5, 0.1);s.store_div(335, 790, 129);s.copy_ad(781, 335);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(332, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(334, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(332), -1.0, 0.0, 332);s.store_sub_from_scalar(332, 1.0, 332);s.store_neg(334, 334);s.store_square(208, 332);s.b[1421] = (s.v[765] == 0.0);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if s.b[1421] {s.store_scalar(80, 0.0);}
        if (!s.b[1421]) {s.store_scalar(80, 1.0);}
        s.copy_ad(335, 637);s.store_sqrt_mul(336, 335, 158);s.store_add_scaled_inputs_mixed_ai(190, A::offset(s.ad_value(158), s.v[160]), 1.0, 336, s.v[189]);s.b[1422] = (s.v[80] == 0.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if s.b[1422] {s.store_scalar(183, s.v[187]);s.store_scalar(185, s.v[188]);s.store_scalar(186, s.v[189]);s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1422]) {s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);s.store_scaled_add(336, 339, 782, 0.5);}
        s.b[1423] = (s.v[336] < 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if ((!s.b[1422]) && s.b[1423]) {s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[1422]) {s.store_offset(336, 336, 1e-25);s.store_div_from_scalar(337, 1.0, 336);s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));s.store_scaled_abs(338, 190, 2.0);s.store_offset_sub(340, 339, 791, s.v[160]);}
        s.b[1424] = (s.v[340] > s.v[338]);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if ((!s.b[1422]) && s.b[1424]) {s.copy_ad(338, 340);}
        if (!s.b[1422]) {s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));}
        if (!s.b[1422]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (!s.b[1422]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));s.store_offset_scaled(184, 336, p.p235, p.p237);s.store_scalar(341, p.p235);}
        s.b[1425] = ((s.v[184] * 1000000000000.0) < s.v[187]);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((!s.b[1422]) && s.b[1425]) {s.store_scalar(184, 0.0);s.store_scalar(80, 0.0);}
        if (!s.b[1422]) {s.store_offset(183, 184, s.v[187]);s.store_div_from_scalar(185, s.v[161], 183);s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));s.store_scale(186, 183, 1.0 / (s.v[161]));s.store_scalar(335, (1.0 / s.v[161]));s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        s.copy_ad(364, 105);s.copy_ad(335, 637);s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);s.store_add_mixed_ai(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);s.copy_ad(123, 158);s.store_scalar(334, 0.95);s.b[338] = (!(s.v[963] > 1.0));s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));
        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p.p366);
        } else {
            s.store_scalar(339, 0.0);
        }
        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);s.store_sqrt(181, 180);s.b[1426] = (p.p140 != 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if s.b[1426] {s.copy_ad(335, 637);s.store_sub_from_scalar(336, p.p224, 364);s.store_offset(337, 336, 1e-25);s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));s.store_scaled_add(339, 337, 338, 0.5);s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);s.store_div_from_scalar(341, 1.0, 339);s.store_scale(175, 341, p.p223);s.store_mul_scale_offset_indices(342, 341, 175, -1.0, 0.0);s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));}
        if s.b[1426] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1426] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1426] {s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);s.store_div(343, 334, 176);s.store_mul_sub_lhs(177, 239, 176, 186);s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);s.store_add_scaled_product_mixed_aii(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 685);s.store_scalar(338, (1.0 / (p.p140 * p.p140)));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_sub(335, 173, 119);s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p.p140)), s.v[465]);s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);s.store_offset(178, 106, p.p221);s.store_square(179, 178);s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);}
        if (!s.b[1426]) {s.store_scalar(174, 0.0);}
        s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[582] - p.p139));s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));s.store_mul_scale_offset_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338], 0.0);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_scale_offset_mixed_ia(341, 181, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 0.0);s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);s.store_scalar(335, (s.v[470] / s.v[582]));s.store_offset_scaled(338, 180, s.v[335], s.v[468]);s.store_add_scaled_product_mixed_iia(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p.p150, 1.0), s.v[469]);s.store_mul(122, 121, 339);s.store_div_from_scalar(335, 1.0, 185);s.store_square(336, 335);s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));s.store_square(338, 337);s.store_sub(339, 335, 337);s.store_mul_sub_rhs(340, 239, 336, 338);s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);s.store_add_scaled_inputs3_offset_indices(118, 157, 1.0, 336, s.v[189], 120, -1.0, s.v[160]);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_scalar(182, 0.0);s.b[1427] = (s.v[615] == 1.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if s.b[1427] {s.copy_ad(341, 107);s.copy_ad(334, 642);s.store_offset(337, 341, (-p.p152));}
        s.b[1428] = (s.v[337] < (-3.0));s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if (s.b[1427] && s.b[1428]) {s.store_scalar(340, 0.0);s.store_scalar(182, 0.0);}
        s.b[1429] = (s.v[337] < 0.0);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1427] && (!s.b[1428])) && s.b[1429]) {s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if ((s.b[1427] && (!s.b[1428])) && (!s.b[1429])) {s.store_offset_mul_offset_rhs_mixed_ia(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if s.b[1427] {s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);}
        s.b[1430] = (s.v[182] < 0.0);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if (s.b[1427] && s.b[1430]) {s.store_scalar(182, 0.0);s.store_scalar(340, 0.0);}
        if s.b[1427] {s.store_mul(182, 182, 334);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1427] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1427] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);}
        s.b[1437] = (s.v[792] > s.v[73]);s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if ((p.p37 != 0.0) && s.b[1437]) {s.store_sub(335, 792, 73);s.store_sub(336, 72, 73);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(1432, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1432, 1.0);s.store_neg(1432, 1432);s.store_add(1431, 73, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(1432, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if ((p.p37 != 0.0) && (!s.b[1437])) {s.copy_ad(1431, 792);s.store_scalar(1432, 1.0);}
        if (p.p37 == 0.0) {s.copy_ad(1431, 792);s.store_scalar(1432, 1.0);}
        s.store_scaled_mul(335, 1432, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(1433, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1438] = (s.v[1433] < 1e-12);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if s.b[1438] {s.store_scalar(1433, 1e-12);}
        s.store_add(1434, 1431, 1433);s.store_add_scaled_inputs(1435, 790, 1.0, 1433, 2.0);s.store_add(1436, 791, 1433);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1431, 1.0, s.v[160]);s.b[1439] = (s.v[963] != 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });s.b[1440] = (p.p42 == 1.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });s.b[1441] = (p.p42 == 2.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });s.b[1442] = (p.p42 == 3.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if (s.b[1439] && s.b[1440]) {s.copy_ad(1459, 960);s.store_scale(1542, 964, 1.6021918e-19);s.store_square(1541, 964);s.store_scale(1498, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1540, 622, 1.6021918e-19);s.store_scalar(1537, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1538, (1.034943e-10 * 1.034943e-10));s.store_square(1539, 965);s.store_div_from_scalar(1543, (2.0 * 1.034943e-10), 1542);s.store_scale(1544, 1542, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1545, 1542, (2.0 * 1.034943e-10));s.store_div_from_scalar(1546, (2.0 * 1.034943e-10), 1540);s.store_scale(1547, 1540, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1532, 964, 622);s.store_div_from_scalar_offset_input(1531, 1.0, 1532, 1.0);s.store_scalar(1548, (1e-12 * 1000.0));s.store_scalar(1549, (1e-10 * 1000.0));s.store_scalar(1457, 0.0);s.store_scalar(1458, 0.0);s.store_scalar(1471, 0.0);s.store_scalar(1472, 0.0);s.store_scalar(1513, 0.0);s.store_scalar(1514, 0.0);s.store_scalar(1493, 0.0);s.store_scalar(1495, 0.0);s.store_scalar(1494, 0.0);s.store_scalar(1496, 0.0);s.store_scalar(1516, 0.0);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1452, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1455, A::div_scaled_value_by_product(s.ad_value(1452), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1541);s.store_sqrt_mul_ad(1449, A::div_scaled_product(s.ad_value(1543), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1459), s.ad_value(1431)));}
        s.b[1555] = (s.v[1449] > s.v[965]);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.store_scalar(1462, 0.0);s.copy_ad(1443, 965);s.store_scalar(1479, 0.0);s.store_sub_mixed_ia(1460, 1479, A::mul3(s.ad_value(1544), s.ad_value(1443), s.ad_value(1443)));s.store_scalar(1507, 0.0);s.copy_ad(1506, 1462);s.copy_ad(1468, 1460);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (150.0 + 1.0);let t5: f64 = if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (s.v[97] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);}
            s.b[1556] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {s.store_offset_sub(781, 1443, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1557] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });s.b[1558] = (2.0 == 1.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && s.b[1558]) {s.store_scalar(720, 1.0);}
            s.b[1559] = (2.0 == 2.0);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && s.b[1559]) {s.store_scalar(720, 2.0);}
            s.b[1560] = (2.0 == 4.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && s.b[1560]) {s.store_scalar(720, 3.0);}
            s.b[1561] = (2.0 == 8.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && (!s.b[1560])) && s.b[1561]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {s.store_scalar(719, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && (!s.b[1557])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1443, 965, (-1e-8), 780);}
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {s.store_scalar(334, 1.0);}
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.store_add_scaled_inputs3_indices(335, 1460, 1.0, 1431, (-1.0), 1459, 1.0);}
            s.b[1562] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1563] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });s.b[1564] = (2.0 == 1.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && s.b[1564]) {s.store_scalar(720, 1.0);}
            s.b[1565] = (2.0 == 2.0);s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && s.b[1565]) {s.store_scalar(720, 2.0);}
            s.b[1566] = (2.0 == 4.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && s.b[1566]) {s.store_scalar(720, 3.0);}
            s.b[1567] = (2.0 == 8.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && (!s.b[1566])) && s.b[1567]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && (!s.b[1563])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1562])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.store_sqrt_mul(1447, 1546, 336);s.store_mul(1493, 1443, 1542);s.store_mul_div_from_scalar_lhs_ad_indices(1525, (-1.034943e-10), 1443, 334);s.store_mul_scale_offset_indices(1494, 1540, 1447, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1527, (-1.034943e-10), 1447, 341);s.store_add_mixed_ai(1481, A::add_scaled_product(s.ad_value(1493), 1.0, s.ad_value(185), A::sub(s.ad_value(1462), s.ad_value(1479)), 1.0), 1494);s.copy_ad(1483, 185);s.store_add(1484, 1525, 1527);s.store_add_scaled_product_mixed_iia(1482, 1460, 1.0, 1531, A::sub(A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459)), (-1.0));s.store_scalar(1485, 0.0);s.store_scalar(1486, 1.0);s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));s.store_div(1488, 1486, 1487);s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);s.store_div(1491, 1483, 1487);}
            s.b[1568] = (((((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482]))) as f64).abs() > 0.5);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {s.store_offset(1460, 1460, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1568])) {s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));s.store_sub_mixed_ia(1460, 1460, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));}
            s.b[1569] = (((((s.v[1462] - s.v[1506])) as f64).abs() <= 1e-12) && ((((s.v[1460] - s.v[1468])) as f64).abs() <= 1e-12));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1569]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.copy_ad(1506, 1462);s.copy_ad(1468, 1460);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {s.copy_ad(1509, 1460);s.store_mul(1447, 965, 1532);s.store_add_scaled_inputs3_mixed_aii(1460, A::mul3(s.ad_value(1547), s.ad_value(1447), s.ad_value(1447)), 1.0, 1431, 1.0, 1459, -1.0);s.store_add_scaled_product_indices(1479, 1460, 1.0, 1544, 1539, 1.0);s.copy_ad(1457, 1479);s.copy_ad(1463, 1479);s.copy_ad(1505, 1479);}
        s.b[1570] = (s.v[85] > s.v[1462]);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1570]) {s.store_scalar(1475, 1.0);}
        s.b[1571] = (s.v[85] > s.v[1505]);s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && s.b[1571]) {s.store_scalar(1475, 3.0);}
        if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && (!s.b[1571])) {s.store_scalar(1475, 2.0);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1555])) {s.store_scalar(1462, 0.0);s.copy_ad(1505, 1462);s.store_scalar(1463, 0.0);s.copy_ad(1507, 1462);s.copy_ad(1443, 1449);s.store_mul(1447, 1443, 1532);s.store_add_scaled_inputs3_mixed_aii(1460, A::mul3(s.ad_value(1547), s.ad_value(1447), s.ad_value(1447)), 1.0, 1431, 1.0, 1459, -1.0);s.store_add_mixed_ai(1479, A::mul3(s.ad_value(1544), s.ad_value(1443), s.ad_value(1443)), 1460);s.copy_ad(1509, 1460);}
        s.b[1572] = (s.v[85] > s.v[1462]);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1555])) && s.b[1572]) {s.store_scalar(1475, 1.0);}
        if (((s.b[1439] && s.b[1440]) && (!s.b[1555])) && (!s.b[1572])) {s.store_scalar(1475, 2.0);}
        if (s.b[1439] && s.b[1440]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1545, 1463, 1.0, 1431, -1.0, 961, 1.0, 0.0);}
        s.b[1573] = (s.v[335] > 0.0);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1573]) {s.store_add_scaled_inputs3_mixed_iia(1451, 1431, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1573])) {s.store_sub(1451, 1431, 961);}
        s.b[1574] = (s.v[85] > s.v[1462]);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1574]) {s.copy_ad(1460, 1509);s.store_scalar(1479, 0.0);s.store_add_div_lhs(1476, A::ln(A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1479);}
        s.b[1575] = (s.v[1476] < (s.v[1507] + s.v[1549]));s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1574]) && s.b[1575]) {s.store_add(1476, 1507, 1549);}
        s.b[1576] = (s.v[85] > s.v[1505]);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && (!s.b[1574])) && s.b[1576]) {s.copy_ad(1476, 1457);}
        s.b[1577] = (s.v[85] > s.v[1451]);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) {s.store_add_scaled_product_indices(1453, 154, 1.0, 1452, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1479, (-1.0));s.copy_ad(1466, 1479);s.store_div_scaled_inputs2_mixed_aii(1476, A::sqrt(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1454), (-4.0))), 0.5, 1453, (-0.5), 1452, 1.0);}
        s.b[1578] = (s.v[1476] > (s.v[1463] - s.v[1548]));s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1578]) {s.store_sub(1476, 1463, 1548);}
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) {s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        s.b[1579] = ((s.v[1445] + s.v[1443]) > s.v[965]);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {s.store_scalar(97, 1.0);}
        let mut t9: usize = 0;
        while {
            let t7: f64 = (150.0 + 1.0);let t8: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (s.v[97] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {s.store_add_scaled_inputs3_indices(1464, 1445, 1.0, 1443, 1.0, 965, -1.0);s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1445), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1443)));}
            s.b[1580] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1580]) {s.store_offset(1479, 1479, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (!s.b[1580])) {s.store_sub_div_rhs_indices(1479, 1479, 1464, 1504);}
            s.b[1581] = (((s.v[1479] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1581]) {s.store_offset_sub(1479, 1431, 1459, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {s.store_add_scaled_product_mixed_aii(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1479, (-1.0));s.store_add_scaled_square_product_indices(335, 1453, 1.0, 1452, 1454, (-4.0));}
            s.b[1582] = (s.v[335] > 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1582]) {s.store_div_scaled_inputs2_sqrt_first(1476, 335, 0.5, 1453, (-0.5), 1452, 1.0);}
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (!s.b[1582])) {s.store_div_scaled_inputs_indices(1476, 1453, (-0.5), 1452, 1.0);}
            s.b[1583] = (s.v[1476] > s.v[1463]);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1583]) {s.copy_ad(1476, 1463);}
            s.b[1584] = (s.v[1476] > s.v[1479]);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1584]) {s.store_sub(1476, 1479, 1549);s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);s.store_div_scaled_inputs2_mixed_aia(1460, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), 1.0, 1459, (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);}
            s.b[1585] = ((((s.v[1479] - s.v[1466])) as f64).abs() <= 1e-8);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1585]) {s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {s.copy_ad(1466, 1479);s.store_primal_offset(97, 97, 1.0);}
        }
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) {s.store_div_mixed_ia(1456, 1455, A::exp(A::mul(s.ad_value(154), s.ad_value(1431))));s.copy_ad(1466, 1479);s.store_div_ad(1476, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);}
        s.b[1586] = ((s.v[1445] + s.v[1443]) > s.v[965]);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) {s.store_scalar(97, 1.0);}
    }
}
