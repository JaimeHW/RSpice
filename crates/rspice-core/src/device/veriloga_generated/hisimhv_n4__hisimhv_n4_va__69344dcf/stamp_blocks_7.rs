#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1288] = (s.v[973] < 1000.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1286]) && s.b[1288]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1285] && s.b[1286]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p.p382);
        }

        s.b[1289] = (s.v[963] == 3.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1291] = (s.v[973] < 1000.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((s.b[1285] && (!s.b[1286])) && s.b[1289]) && s.b[1291]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1285] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1292] = (p.p39 != 2.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1292]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1285] && (!s.b[1292])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1294] = (p.p39 != 2.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1294]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1285] && (!s.b[1294])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1296] = (s.v[682] < 0.0);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1296]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1298] = (s.v[688] < 0.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1298]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1300] = (s.v[689] < 0.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1300]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1285] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1302] = (s.v[766] < 0.0001);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if ((s.b[1285] && (p.p53 != 0.0)) && s.b[1302]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1285] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1303] = (s.v[963] == 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1303]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1304] = (s.v[963] == 0.0);
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        s.b[1305] = (s.v[459] != 0.0);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1304]) && s.b[1305]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1306] = (s.v[460] != 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1304]) && s.b[1306]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1307] = (s.v[459] != 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((s.b[1285] && (!s.b[1304])) && s.b[1307]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1308] = (s.v[460] != 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if ((s.b[1285] && (!s.b[1304])) && s.b[1308]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1309] = (s.v[449] == 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        s.b[1310] = (s.v[530] > 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1309]) && s.b[1310]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1311] = (p.p39 == 1.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1310])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1312] = (s.v[540] > 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1309]) && s.b[1312]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1313] = (p.p39 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1312])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1314] = (s.v[538] > 0.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1315] = (s.v[336] < 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1315]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1316] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1317] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1314])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1285] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1320] = (s.v[957] < 0.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1320]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1321] = (s.v[957] > 0.005);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1321]) {
            s.store_scalar(957, 0.005);
        }

        s.b[1322] = (s.v[449] > 0.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }

        if (s.b[1285] && s.b[1322]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_offset_scaled(959, 387, p.p439, (((((-s.v[764])) * (p.p439))) + (s.v[959])));
        }

        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }

        if (s.b[1285] && s.b[1322]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_offset_scaled(956, 387, p.p438, (((((-s.v[764])) * (p.p438))) + (s.v[956])));
        }

        s.b[1324] = (s.v[956] < 0.1);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1322]) && s.b[1324]) {
            s.store_scalar(956, 0.1);
        }

        if s.b[1285] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[1325] = (p.p48 > 0.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        s.b[1326] = (p.p15 > s.v[632]);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1325]) && s.b[1326]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[1285] && s.b[1325]) && (!s.b[1326])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scalar(875, 0.0);
            s.store_scalar(876, 0.0);
            s.store_scale(877, 836, p.p15);
            s.store_scale(878, 837, p.p15);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[1285] && (!s.b[1325])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[1285] {
            s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);
        }

        s.b[1327] = (s.v[847] > 0.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1327]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));
            s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[1285] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[1328] = (p.p48 > 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        s.b[1329] = (p.p16 > s.v[632]);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if ((s.b[1285] && s.b[1328]) && s.b[1329]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

        if ((s.b[1285] && s.b[1328]) && (!s.b[1329])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[1285] && (!s.b[1328])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[1285] {
            s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);
        }

        s.b[1330] = (s.v[852] > 0.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1330]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));
            s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[1285] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[1331] = (p.p15 > s.v[632]);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1331]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[1285] && (!s.b[1331])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[1332] = (s.v[832] < 0.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1332]) {
            s.store_scalar(832, 0.0);
        }

        s.b[1333] = (s.v[833] < 0.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1333]) {
            s.store_scalar(833, 0.0);
        }

        s.b[1334] = (s.v[834] < 0.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1334]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[1285] {
            s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);
            s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);
            s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);
        }

        s.b[1335] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1335]) {
            s.store_scalar(841, 0.01);
        }

        s.b[1336] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1336]) {
            s.store_scalar(842, 0.01);
        }

        s.b[1337] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1337]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1285] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[1338] = (p.p16 > s.v[632]);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1338]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[1285] && (!s.b[1338])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[1339] = (s.v[835] < 0.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1339]) {
            s.store_scalar(835, 0.0);
        }

        s.b[1340] = (s.v[838] < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1340]) {
            s.store_scalar(838, 0.0);
        }

        s.b[1341] = (s.v[839] < 0.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1341]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[1285] {
            s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);
            s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);
            s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);
        }

        s.b[1342] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1342]) {
            s.store_scalar(844, 0.01);
        }

        s.b[1343] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1343]) {
            s.store_scalar(845, 0.01);
        }

        s.b[1344] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (s.b[1285] && s.b[1344]) {
            s.store_scalar(846, 0.01);
        }

        s.store_scaled_voltage(729, ctx, nodes, Some(5), Some(7), p.p87);

        s.store_scaled_voltage(731, ctx, nodes, Some(6), Some(7), p.p87);

        s.store_scaled_voltage(728, ctx, nodes, Some(8), Some(7), p.p87);

        s.store_scaled_voltage(733, ctx, nodes, Some(0), Some(2), p.p87);

        s.store_scaled_voltage(734, ctx, nodes, Some(6), Some(2), p.p87);

        s.store_scaled_voltage(735, ctx, nodes, Some(8), Some(2), p.p87);

        s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(5), p.p87);

        s.store_scaled_voltage(804, ctx, nodes, Some(7), Some(2), p.p87);

        s.store_scaled_voltage(857, ctx, nodes, Some(10), Some(2), p.p87);

        s.store_scaled_voltage(858, ctx, nodes, Some(9), Some(0), p.p87);

        s.store_scaled_voltage(865, ctx, nodes, Some(8), Some(7), p.p87);

        s.store_scaled_voltage(866, ctx, nodes, Some(8), Some(5), p.p87);

        s.copy_ad(859, 857);

        s.copy_ad(860, 858);

        s.copy_ad(867, 865);

        s.copy_ad(868, 866);

        s.v[798] = 0.0;

        if (s.v[81] != 0.0) {
            s.store_voltage(747, ctx, nodes, Some(11), None);
            s.store_voltage(748, ctx, nodes, Some(12), None);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(747, 0.0);
            s.store_scalar(748, 0.0);
        }

        s.store_sub(730, 731, 729);

        s.store_sub(727, 728, 729);

        s.b[1345] = (s.v[729] >= 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if s.b[1345] {
            s.store_scalar(949, 1.0);
            s.copy_ad(790, 729);
            s.copy_ad(791, 731);
            s.copy_ad(792, 728);
            s.copy_ad(793, 733);
            s.copy_ad(796, 734);
            s.copy_ad(797, 735);
        }

        if (!s.b[1345]) {
            s.store_scalar(949, (-1.0));
            s.store_neg(790, 729);
            s.copy_ad(791, 730);
            s.copy_ad(792, 727);
            s.store_neg(793, 733);
            s.store_sub(796, 734, 733);
            s.store_sub(797, 735, 733);
        }

        s.b[1348] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if s.b[1348] {
            s.store_voltage(732, ctx, nodes, Some(4), None);
        }

        s.b[1349] = (p.p53 == 2.0);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1349]) {
            s.store_offset_sub_from_scalar_ad(781, p.p433, s.ad_value(732), (-(p.p337 * 10.0)));
            s.store_scalar(782, ((4.0 * p.p433) * (p.p337 * 10.0)));
        }

        if (s.b[1348] && s.b[1349]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1348] && s.b[1349]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p.p433);
        }

        if s.b[1348] {
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_add(387, 387, 732);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1348] {
            s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));
            s.store_scale(676, 387, 1.0 / (s.v[764]));
            s.store_ln(590, 676);
            s.store_sub_scaled_ad_lhs(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 392, s.v[456]);
            s.store_sqrt(677, 393);
            s.store_div_from_scalar(335, 1.0, 387);
            s.store_scalar(336, (1.0 / s.v[764]));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p.p260, 336, (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));
            s.store_sqrt(192, 337);
            s.store_mul(193, 337, 192);
            s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);
            s.store_div_from_scalar(155, 1.0, 154);
            s.store_square(156, 154);
            s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));
            s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);
            s.store_exp_scaled_input(335, 590, s.v[480]);
            s.store_div(679, 335, 573);
        }

        s.b[1351] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1351]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1353] = (s.v[973] < 1000.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1351]) && s.b[1353]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1348] && s.b[1351]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_ad_rhs(970, 970, A::powf(s.ad_value(676), p.p382));
        }

        s.b[1354] = (s.v[963] == 3.0);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if ((s.b[1348] && (!s.b[1351])) && s.b[1354]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1356] = (s.v[973] < 1000.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (((s.b[1348] && (!s.b[1351])) && s.b[1354]) && s.b[1356]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1348] && (!s.b[1351])) && s.b[1354]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1348] && (!s.b[1351])) && (!s.b[1354])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1348] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1357] = (p.p39 != 2.0);
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1357]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1348] && (!s.b[1357])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1359] = (p.p39 != 2.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1359]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1348] && (!s.b[1359])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1361] = (s.v[682] < 0.0);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1361]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1363] = (s.v[688] < 0.0);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1363]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1365] = (s.v[689] < 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1365]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1348] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1367] = (s.v[766] < 0.0001);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((s.b[1348] && (p.p53 != 0.0)) && s.b[1367]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1348] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1348] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1348] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1368] = (s.v[963] == 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1368]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1369] = (s.v[963] == 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        s.b[1370] = (s.v[459] != 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1369]) && s.b[1370]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1371] = (s.v[460] != 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1369]) && s.b[1371]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1372] = (s.v[459] != 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((s.b[1348] && (!s.b[1369])) && s.b[1372]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1373] = (s.v[460] != 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if ((s.b[1348] && (!s.b[1369])) && s.b[1373]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1374] = (s.v[449] == 0.0);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        s.b[1375] = (s.v[530] > 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1374]) && s.b[1375]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1376] = (p.p39 == 1.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1375])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1377] = (s.v[540] > 0.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1374]) && s.b[1377]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1378] = (p.p39 == 1.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1377])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1379] = (s.v[538] > 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1380] = (s.v[336] < 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1380]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1381] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1382] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1379])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1348] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1385] = (s.v[957] < 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1385]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1386] = (s.v[957] > 0.005);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (s.b[1348] && s.b[1386]) {
            s.store_scalar(957, 0.005);
        }

        if (!s.b[1348]) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        s.v[164] = (s.v[630] * p.p7);

        s.v[165] = (p.p67 + p.p68);

        s.v[160] = s.v[462];

        s.copy_ad(257, 681);

        s.v[161] = s.v[617];

        s.v[187] = p.p95;

        s.v[188] = (s.v[161] / s.v[187]);

        s.v[189] = (1.0 / s.v[188]);

        s.store_div_from_scalar(412, s.v[161], 543);

        s.v[270] = (p.p87 * p.p434);

        s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p.p262)), (-0.1));

        s.v[782] = ((4.0 * 0.8) * 0.1);

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_square_add(782, 781, 782);

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(335, 781, (-0.5), 782, (-0.5), 0.8);

        s.copy_ad(69, 335);

        s.b[1387] = ((s.v[158] - p.p262) < s.v[69]);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if s.b[1387] {
            s.store_offset(69, 158, (-p.p262));
        }

        s.b[1388] = ((s.v[159] - p.p262) < s.v[69]);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if s.b[1388] {
            s.store_offset(69, 159, (-p.p262));
        }

        s.b[1389] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        s.b[1390] = ((s.v[961] - p.p262) < s.v[69]);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if (s.b[1389] && s.b[1390]) {
            s.store_offset(69, 961, (-p.p262));
        }

        s.b[1391] = ((s.v[960] - p.p262) < s.v[69]);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (s.b[1389] && s.b[1391]) {
            s.store_offset(69, 960, (-p.p262));
        }

        s.b[1392] = (s.v[70] > (s.v[69] * 0.5));
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if s.b[1392] {
            s.store_scale(70, 69, 0.5);
        }

        s.b[1393] = param_given[338];
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if s.b[1393] {
            s.store_scalar(72, p.p338);
        }

        if (!s.b[1393]) {
            s.copy_ad(72, 69);
        }

        s.b[1394] = param_given[339];
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if s.b[1394] {
            s.store_scalar(73, p.p339);
        }

        s.b[1395] = param_given[338];
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if ((!s.b[1394]) && s.b[1395]) {
            s.store_scale(73, 72, 0.5);
        }

        if ((!s.b[1394]) && (!s.b[1395])) {
            s.copy_ad(73, 70);
        }

        s.b[1396] = (s.v[73] > (s.v[72] * 0.5));
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if s.b[1396] {
            s.store_scale(73, 72, 0.5);
        }

        s.b[1397] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        s.b[1398] = (s.v[448] == 1.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if (s.b[1397] && s.b[1398]) {
            s.store_scalar(74, 1.0);
        }

        s.b[1399] = (s.v[448] == 2.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if (s.b[1397] && s.b[1399]) {
            s.store_scalar(74, 2.0);
        }

        s.b[1400] = (s.v[448] == 3.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if (s.b[1397] && s.b[1400]) {
            s.store_scalar(74, 3.0);
        }

        s.v[77] = 0.0;

        s.b[1401] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if s.b[1401] {
            s.copy_ad(373, 733);
        }

        s.b[1402] = (s.v[373] >= 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (s.b[1401] && s.b[1402]) {
            s.copy_ad(376, 373);
            s.store_scalar(383, s.v[798]);
        }

        if (s.b[1401] && (!s.b[1402])) {
            s.store_neg(376, 373);
            s.store_sub_from_scalar(383, s.v[798], 373);
        }

        if s.b[1401] {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1403] = (s.v[108] < 1e-12);
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (s.b[1401] && s.b[1403]) {
            s.store_scalar(108, 1e-12);
        }

        if s.b[1401] {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1404] = (s.v[335] < 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (s.b[1401] && s.b[1404]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if s.b[1401] {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if s.b[1401] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1401] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
        }

        if (!s.b[1401]) {
            s.store_scalar(384, 0.0);
        }

        s.b[1405] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if s.b[1405] {
            s.copy_ad(373, 733);
            s.copy_ad(374, 734);
            s.copy_ad(372, 735);
        }

        s.b[1406] = (s.v[373] >= 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (s.b[1405] && s.b[1406]) {
            s.store_scalar(370, 1.0);
            s.store_scalar(371, 0.0);
            s.copy_ad(376, 373);
            s.copy_ad(377, 374);
            s.copy_ad(375, 372);
            s.store_scalar(383, s.v[798]);
        }

        if (s.b[1405] && (!s.b[1406])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 1.0);
            s.store_neg(376, 373);
            s.store_sub(377, 374, 373);
            s.store_sub(375, 372, 373);
            s.store_sub_from_scalar(383, s.v[798], 373);
        }

        s.b[1407] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if (s.b[1405] && s.b[1407]) {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1408] = (s.v[108] < 1e-12);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if ((s.b[1405] && s.b[1407]) && s.b[1408]) {
            s.store_scalar(108, 1e-12);
        }

        if (s.b[1405] && s.b[1407]) {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_add(381, 377, 108);
            s.store_add(382, 375, 108);
        }

        s.b[1409] = ((p.p34 == 1.0) || (s.v[370] == 1.0));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);
            s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);
            s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(344, 782, p.p292, 0.5);
        }

        s.b[1410] = (s.v[344] < 0.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1410]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1411] = (s.v[337] < 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1411]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 366);
            s.store_mul(366, 366, 337);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1409])) {
            s.copy_ad(366, 691);
        }

        if (s.b[1405] && s.b[1407]) {
            s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);
        }

        s.b[1412] = ((p.p34 == 1.0) || (s.v[371] == 1.0));
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);
            s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));
            s.store_scalar(344, (p.p292 + 1e-25));
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1413] = (s.v[337] < 0.0);
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if (((s.b[1405] && s.b[1407]) && s.b[1412]) && s.b[1413]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 367);
            s.store_mul(367, 367, 337);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1412])) {
            s.copy_ad(367, 691);
        }

        s.b[1414] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1415] = (s.v[335] < 0.0);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if (((s.b[1405] && s.b[1407]) && s.b[1414]) && s.b[1415]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
            s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));
            s.store_mul(334, 366, 340);
            s.store_mul(335, 367, 340);
            s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);
            s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1414])) {
            s.store_scalar(384, 0.0);
        }

        if (s.b[1405] && s.b[1407]) {
            s.copy_ad(4, 366);
            s.copy_ad(5, 367);
        }

        if (s.b[1405] && (!s.b[1407])) {
            s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);
        }

        if s.b[1405] {
            s.store_scale(4, 4, 1.0 / (s.v[164]));
            s.store_scale(5, 5, 1.0 / (s.v[164]));
            s.store_add_scaled_value_products(4, s.ad_value(4), 1.0, s.ad_value(370), s.ad_value(644), 1.0, s.ad_value(371), s.ad_value(648), 1.0);
            s.store_add_scaled_value_products(5, s.ad_value(5), 1.0, s.ad_value(370), s.ad_value(648), 1.0, s.ad_value(371), s.ad_value(644), 1.0);
            s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);
            s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);
        }

        s.b[1418] = (s.v[792] > s.v[70]);
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if s.b[1418] {
            s.store_sub(335, 792, 70);
            s.store_sub(336, 69, 70);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(84, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 84, 1.0);
            s.store_neg(84, 84);
            s.store_add(83, 70, 333);
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_mul(338, 335, 337);
            s.store_square(339, 338);
            s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);
            s.store_div_scaled_inputs_product(84, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

        if (!s.b[1418]) {
            s.copy_ad(83, 792);
            s.store_scalar(84, 1.0);
        }

        s.store_scaled_mul(335, 84, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(108, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1419] = (s.v[108] < 1e-12);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if s.b[1419] {
            s.store_scalar(108, 1e-12);
        }

        s.store_add(105, 83, 108);

        s.store_add_scaled_inputs(106, 790, 1.0, 108, 2.0);

        s.store_add(107, 791, 108);

        s.store_scale(335, 636, (s.v[189] * s.v[189]));

        s.store_offset(336, 791, (-s.v[160]));

        s.store_offset_mul_ad(337, A::div_from_scalar(2.0, s.ad_value(335)), A::add_scaled_inputs3(s.ad_value(336), 1.0, A::div_from_scalar(1.0, s.ad_value(678)), (-1.0), s.ad_value(83), -1.0), 1.0);

        s.store_sqrt_square_offset(782, 337, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(339, 337, 782, 0.5, 0.5);

        s.store_scaled_add(338, 337, 782, 0.5);

        s.b[1420] = (s.v[338] < 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if s.b[1420] {
            s.store_scalar(338, 0.0);
            s.store_scalar(339, 0.0);
        }

        s.store_offset(338, 338, 1e-25);

        s.store_sqrt(332, 338);

        s.store_add_ad_rhs(128, 336, A::mul_sub_from_scalar_rhs(s.ad_value(335), 1.0, s.ad_value(332)));

        s.store_sub(129, 128, 159);

        s.store_offset(781, 129, (((-0.1)) + ((-0.05))));

        s.v[782] = ((4.0 * 0.1) * 0.05);

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_square_add(782, 781, 782);

        s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(129, 781, 0.5, 782, 0.5, 0.1);

        s.store_div(335, 790, 129);

        s.copy_ad(781, 335);

        s.store_square(782, 781);

        s.store_mul(783, 782, 781);

        s.store_square(784, 782);

        s.store_div_from_scalar_ad(332, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(334, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(332), -1.0, 0.0, 332);

        s.store_sub_from_scalar(332, 1.0, 332);

        s.store_neg(334, 334);

        s.store_square(208, 332);

        s.b[1421] = (s.v[765] == 0.0);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if s.b[1421] {
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1421]) {
            s.store_scalar(80, 1.0);
        }

        s.copy_ad(335, 637);

        s.store_sqrt_mul(336, 335, 158);

        s.store_add_scaled_ad_lhs(190, A::offset(s.ad_value(158), s.v[160]), 336, s.v[189]);

        s.b[1422] = (s.v[80] == 0.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if s.b[1422] {
            s.store_scalar(183, s.v[187]);
            s.store_scalar(185, s.v[188]);
            s.store_scalar(186, s.v[189]);
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        if (!s.b[1422]) {
            s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);
            s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));
            s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);
            s.store_scaled_add(336, 339, 782, 0.5);
        }

        s.b[1423] = (s.v[336] < 0.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if ((!s.b[1422]) && s.b[1423]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[1422]) {
            s.store_offset(336, 336, 1e-25);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1422]) {
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));
            s.store_scaled_abs(338, 190, 2.0);
            s.store_offset_sub(340, 339, 791, s.v[160]);
        }

        s.b[1424] = (s.v[340] > s.v[338]);
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if ((!s.b[1422]) && s.b[1424]) {
            s.copy_ad(338, 340);
        }

        if (!s.b[1422]) {
            s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));
            s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));
        }

        if (!s.b[1422]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (!s.b[1422]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));
            s.store_offset_scaled(184, 336, p.p235, p.p237);
            s.store_scalar(341, p.p235);
        }

        s.b[1425] = ((s.v[184] * 1000000000000.0) < s.v[187]);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if ((!s.b[1422]) && s.b[1425]) {
            s.store_scalar(184, 0.0);
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1422]) {
            s.store_offset(183, 184, s.v[187]);
            s.store_div_from_scalar(185, s.v[161], 183);
            s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));
            s.store_scale(186, 183, 1.0 / (s.v[161]));
            s.store_scalar(335, (1.0 / s.v[161]));
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        s.copy_ad(364, 105);

        s.copy_ad(335, 637);

        s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);

        s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);

        s.store_add_ad_lhs(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);

        s.copy_ad(123, 158);

        s.v[334] = 0.95;

        s.b[338] = (!(s.v[963] > 1.0));
        s.v[338] = if s.b[338] { 1.0 } else { 0.0 };

        s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));

        s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));

        s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));

        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p.p366);
        } else {
            s.store_scalar(339, 0.0);
        }

        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);

        s.store_sqrt(181, 180);

        s.b[1426] = (p.p140 != 0.0);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if s.b[1426] {
            s.copy_ad(335, 637);
            s.store_sub_from_scalar(336, p.p224, 364);
            s.store_offset(337, 336, 1e-25);
            s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));
            s.store_scaled_add(339, 337, 338, 0.5);
            s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);
            s.store_div_from_scalar(341, 1.0, 339);
            s.store_scale(175, 341, p.p223);
            s.store_mul_neg_lhs(342, 175, 341);
            s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));
            s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));
        }

        if s.b[1426] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1426] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));
            s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);
            s.store_div(343, 334, 176);
            s.store_mul_sub_lhs(177, 239, 176, 186);
            s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);
            s.store_add_scaled_product_value_ad(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);
            s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 685);
            s.store_scalar(338, (1.0 / (p.p140 * p.p140)));
            s.store_mul_ad_product_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), s.ad_value(336), 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.ad_value(338), 181);
            s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));
            s.store_sub(335, 173, 119);
            s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p.p140)), s.v[465]);
            s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);
            s.store_offset(178, 106, p.p221);
            s.store_square(179, 178);
            s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);
        }

        if (!s.b[1426]) {
            s.store_scalar(174, 0.0);
        }

        s.store_scale(335, 186, 1.034943e-10);

        s.copy_ad(336, 684);

        s.v[337] = (s.v[582] - p.p139);

        s.v[338] = (1.0 / (s.v[337] * s.v[337]));

        s.store_mul_scaled_ad_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338]);

        s.store_mul(121, 339, 181);

        s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);

        s.store_mul_scale_ad_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 181);

        s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);

        s.v[335] = (s.v[470] / s.v[582]);

        s.store_offset_scaled(338, 180, s.v[335], s.v[468]);

        s.store_add_scaled_product_right_ad(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p.p150, 1.0), s.v[469]);

        s.store_mul(122, 121, 339);

        s.store_div_from_scalar(335, 1.0, 185);

        s.store_square(336, 335);

        s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));

        s.store_square(338, 337);

        s.store_sub(339, 335, 337);

        s.store_mul_sub_rhs(340, 239, 336, 338);

        s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));

        s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);

        s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);

        s.store_add_scaled_inputs3_offset_indices(118, 157, 1.0, 336, s.v[189], 120, -1.0, s.v[160]);

        s.store_mul(212, 209, 186);

        s.store_square(213, 212);

        s.v[182] = 0.0;

        s.b[1427] = (s.v[615] == 1.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if s.b[1427] {
            s.copy_ad(341, 107);
            s.copy_ad(334, 642);
            s.store_offset(337, 341, (-p.p152));
        }

        s.b[1428] = (s.v[337] < (-3.0));
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if (s.b[1427] && s.b[1428]) {
            s.store_scalar(340, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[1429] = (s.v[337] < 0.0);
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if ((s.b[1427] && (!s.b[1428])) && s.b[1429]) {
            s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if ((s.b[1427] && (!s.b[1428])) && (!s.b[1429])) {
            s.store_offset_mul_offset_rhs_ad_rhs(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if s.b[1427] {
            s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));
            s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);
        }

        s.b[1430] = (s.v[182] < 0.0);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if (s.b[1427] && s.b[1430]) {
            s.store_scalar(182, 0.0);
            s.store_scalar(340, 0.0);
        }

        if s.b[1427] {
            s.store_mul(182, 182, 334);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1427] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1427] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);
        }

        s.b[1437] = (s.v[792] > s.v[73]);
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((p.p37 != 0.0) && s.b[1437]) {
            s.store_sub(335, 792, 73);
            s.store_sub(336, 72, 73);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(1432, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1432, 1.0);
            s.store_neg(1432, 1432);
            s.store_add(1431, 73, 333);
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_mul(338, 335, 337);
            s.store_square(339, 338);
            s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((p.p37 != 0.0) && s.b[1437]) {
            s.store_div_scaled_inputs_product(1432, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

        if ((p.p37 != 0.0) && (!s.b[1437])) {
            s.copy_ad(1431, 792);
            s.store_scalar(1432, 1.0);
        }

        if (p.p37 == 0.0) {
            s.copy_ad(1431, 792);
            s.store_scalar(1432, 1.0);
        }

        s.store_scaled_mul(335, 1432, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(1433, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1438] = (s.v[1433] < 1e-12);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if s.b[1438] {
            s.store_scalar(1433, 1e-12);
        }

        s.store_add(1434, 1431, 1433);

        s.store_add_scaled_inputs(1435, 790, 1.0, 1433, 2.0);

        s.store_add(1436, 791, 1433);

        s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));

        s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1431, 1.0, s.v[160]);

        s.b[1439] = (s.v[963] != 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        s.b[1440] = (p.p42 == 1.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        s.b[1441] = (p.p42 == 2.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        s.b[1442] = (p.p42 == 3.0);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if (s.b[1439] && s.b[1440]) {
            s.copy_ad(1459, 960);
            s.store_scale(1542, 964, 1.6021918e-19);
            s.store_square(1541, 964);
            s.store_scale(1498, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1540, 622, 1.6021918e-19);
            s.store_scalar(1537, (1.6021918e-19 * 1.6021918e-19));
            s.store_scalar(1538, (1.034943e-10 * 1.034943e-10));
            s.store_square(1539, 965);
            s.store_div_from_scalar(1543, (2.0 * 1.034943e-10), 1542);
            s.store_scale(1544, 1542, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_scale(1545, 1542, (2.0 * 1.034943e-10));
            s.store_div_from_scalar(1546, (2.0 * 1.034943e-10), 1540);
            s.store_scale(1547, 1540, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_div(1532, 964, 622);
            s.store_div_from_scalar_offset_input(1531, 1.0, 1532, 1.0);
            s.store_scalar(1548, (1e-12 * 1000.0));
            s.store_scalar(1549, (1e-10 * 1000.0));
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1471, 0.0);
            s.store_scalar(1472, 0.0);
            s.store_scalar(1513, 0.0);
            s.store_scalar(1514, 0.0);
            s.store_scalar(1493, 0.0);
            s.store_scalar(1495, 0.0);
            s.store_scalar(1494, 0.0);
            s.store_scalar(1496, 0.0);
            s.store_scalar(1516, 0.0);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));
            s.store_div_scaled_product_by_product(1452, s.ad_value(185), s.ad_value(185), 1.0, s.ad_value(209), s.ad_value(209), 1.0);
            s.store_mul_ad_lhs(1455, A::div_scaled_value_by_product(s.ad_value(1452), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1541);
            s.store_sqrt_mul_ad(1449, A::div_scaled_product(s.ad_value(1543), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1459), s.ad_value(1431)));
        }

        s.b[1555] = (s.v[1449] > s.v[965]);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
            s.store_scalar(1462, 0.0);
            s.copy_ad(1443, 965);
            s.store_scalar(1479, 0.0);
            s.store_sub_ad_rhs(1460, 1479, A::mul3(s.ad_value(1544), s.ad_value(1443), s.ad_value(1443)));
            s.store_scalar(1507, 0.0);
            s.copy_ad(1506, 1462);
            s.copy_ad(1468, 1460);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        let mut assign24600_loop_guard: usize = 0;
        while {
            let assign24600_cond_e19002: f64 = (150.0 + 1.0);
            let assign24600_cond_e19004: f64 = if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (s.v[97] <= assign24600_cond_e19002)) { 1.0 } else { 0.0 };
            assign24600_cond_e19004 != 0.0
        } {
            assign24600_loop_guard += 1;
            assert!(assign24600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
            }
            s.b[1556] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
                s.store_offset_sub(781, 1443, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
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
            s.b[1557] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };
            s.b[1558] = (2.0 == 1.0);
            s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && s.b[1558]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1559] = (2.0 == 2.0);
            s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && s.b[1559]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1560] = (2.0 == 4.0);
            s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && s.b[1560]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1561] = (2.0 == 8.0);
            s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && (!s.b[1560])) && s.b[1561]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24600_body27_loop_guard: usize = 0;
            while {
                let assign24600_body27_cond_e19320: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24600_body27_cond_e19320 != 0.0
            } {
                assign24600_body27_loop_guard += 1;
                assert!(assign24600_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && (!s.b[1557])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_add_scaled_inputs3_indices(335, 1460, 1.0, 1431, (-1.0), 1459, 1.0);
            }
            s.b[1562] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
                s.store_sub_from_scalar(781, 0.1, 335);
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
            s.b[1563] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };
            s.b[1564] = (2.0 == 1.0);
            s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && s.b[1564]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1565] = (2.0 == 2.0);
            s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && s.b[1565]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1566] = (2.0 == 4.0);
            s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && s.b[1566]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1567] = (2.0 == 8.0);
            s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && (!s.b[1566])) && s.b[1567]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24600_body63_loop_guard: usize = 0;
            while {
                let assign24600_body63_cond_e19774: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24600_body63_cond_e19774 != 0.0
            } {
                assign24600_body63_loop_guard += 1;
                assert!(assign24600_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && (!s.b[1563])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1562])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_sqrt_mul(1447, 1546, 336);
                s.store_mul(1493, 1443, 1542);
                s.store_mul_div_from_scalar_lhs(1525, (-1.034943e-10), 1443, 334);
                s.store_mul_neg_lhs(1494, 1447, 1540);
                s.store_mul_div_from_scalar_lhs(1527, (-1.034943e-10), 1447, 341);
                s.store_add_ad_lhs(1481, A::add_scaled_product(s.ad_value(1493), 1.0, s.ad_value(185), A::sub(s.ad_value(1462), s.ad_value(1479)), 1.0), 1494);
                s.copy_ad(1483, 185);
                s.store_add(1484, 1525, 1527);
                s.store_add_scaled_product_right_ad(1482, 1460, 1.0, 1531, A::sub(A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459)), (-1.0));
                s.store_scalar(1485, 0.0);
                s.store_scalar(1486, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1568] = (((((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482]))) as f64).abs() > 0.5);
            s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {
                s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {
                s.store_offset(1460, 1460, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1568])) {
                s.store_sub_ad_rhs(1462, 1462, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1460, 1460, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1569] = (((((s.v[1462] - s.v[1506])) as f64).abs() <= 1e-12) && ((((s.v[1460] - s.v[1468])) as f64).abs() <= 1e-12));
            s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1569]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.copy_ad(1506, 1462);
                s.copy_ad(1468, 1460);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
            s.copy_ad(1509, 1460);
            s.store_mul(1447, 965, 1532);
            s.store_add_scaled_inputs3_mixed_aii(1460, A::mul3(s.ad_value(1547), s.ad_value(1447), s.ad_value(1447)), 1.0, 1431, 1.0, 1459, -1.0);
            s.store_add_scaled_product_indices(1479, 1460, 1.0, 1544, 1539, 1.0);
            s.copy_ad(1457, 1479);
            s.copy_ad(1463, 1479);
            s.copy_ad(1505, 1479);
        }

        s.b[1570] = (s.v[85] > s.v[1462]);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1570]) {
            s.store_scalar(1475, 1.0);
        }

        s.b[1571] = (s.v[85] > s.v[1505]);
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && s.b[1571]) {
            s.store_scalar(1475, 3.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && (!s.b[1571])) {
            s.store_scalar(1475, 2.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1555])) {
            s.store_scalar(1462, 0.0);
            s.copy_ad(1505, 1462);
            s.store_scalar(1463, 0.0);
            s.copy_ad(1507, 1462);
            s.copy_ad(1443, 1449);
            s.store_mul(1447, 1443, 1532);
            s.store_add_scaled_inputs3_mixed_aii(1460, A::mul3(s.ad_value(1547), s.ad_value(1447), s.ad_value(1447)), 1.0, 1431, 1.0, 1459, -1.0);
            s.store_add_ad_lhs(1479, A::mul3(s.ad_value(1544), s.ad_value(1443), s.ad_value(1443)), 1460);
            s.copy_ad(1509, 1460);
        }

        s.b[1572] = (s.v[85] > s.v[1462]);
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1555])) && s.b[1572]) {
            s.store_scalar(1475, 1.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1555])) && (!s.b[1572])) {
            s.store_scalar(1475, 2.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1545, s.ad_value(1463), 1.0, s.ad_value(1431), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1573] = (s.v[335] > 0.0);
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1573]) {
            s.store_add_scaled_inputs3_mixed_iia(1451, 1431, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1573])) {
            s.store_sub(1451, 1431, 961);
        }

        s.b[1574] = (s.v[85] > s.v[1462]);
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1574]) {
            s.copy_ad(1460, 1509);
            s.store_scalar(1479, 0.0);
            s.store_add_div_lhs(1476, A::ln(A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1479);
        }

        s.b[1575] = (s.v[1476] < (s.v[1507] + s.v[1549]));
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1574]) && s.b[1575]) {
            s.store_add(1476, 1507, 1549);
        }

        s.b[1576] = (s.v[85] > s.v[1505]);
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1574])) && s.b[1576]) {
            s.copy_ad(1476, 1457);
        }

        s.b[1577] = (s.v[85] > s.v[1451]);
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) {
            s.store_add_scaled_product_indices(1453, 154, 1.0, 1452, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1479, (-1.0));
            s.copy_ad(1466, 1479);
            s.store_div_scaled_inputs2_mixed_aii(1476, A::sqrt(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1454), (-4.0))), 0.5, 1453, (-0.5), 1452, 1.0);
        }

        s.b[1578] = (s.v[1476] > (s.v[1463] - s.v[1548]));
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1578]) {
            s.store_sub(1476, 1463, 1548);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) {
            s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);
            s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
        }

        s.b[1579] = ((s.v[1445] + s.v[1443]) > s.v[965]);
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25080_loop_guard: usize = 0;
        while {
            let assign25080_cond_e20786: f64 = (150.0 + 1.0);
            let assign25080_cond_e20788: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (s.v[97] <= assign25080_cond_e20786)) { 1.0 } else { 0.0 };
            assign25080_cond_e20788 != 0.0
        } {
            assign25080_loop_guard += 1;
            assert!(assign25080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {
                s.store_add_scaled_inputs3_indices(1464, 1445, 1.0, 1443, 1.0, 965, -1.0);
                s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1445), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1443)));
            }
            s.b[1580] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);
            s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1580]) {
                s.store_offset(1479, 1479, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (!s.b[1580])) {
                s.store_sub_div_rhs_indices(1479, 1479, 1464, 1504);
            }
            s.b[1581] = (((s.v[1479] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));
            s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1581]) {
                s.store_offset_sub(1479, 1431, 1459, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {
                s.store_add_scaled_product_value_ad(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1479, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1453, 1.0, 1452, 1454, (-4.0));
            }
            s.b[1582] = (s.v[335] > 0.0);
            s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1582]) {
                s.store_div_scaled_inputs2_mixed_aii(1476, A::sqrt(s.ad_value(335)), 0.5, 1453, (-0.5), 1452, 1.0);
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && (!s.b[1582])) {
                s.store_div_scaled_inputs_indices(1476, 1453, (-0.5), 1452, 1.0);
            }
            s.b[1583] = (s.v[1476] > s.v[1463]);
            s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1583]) {
                s.copy_ad(1476, 1463);
            }
            s.b[1584] = (s.v[1476] > s.v[1479]);
            s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1584]) {
                s.store_sub(1476, 1479, 1549);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {
                s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);
                s.store_div_scaled_inputs2_mixed_aia(1460, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), 1.0, 1459, (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
            }
            s.b[1585] = ((((s.v[1479] - s.v[1466])) as f64).abs() <= 1e-8);
            s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) && s.b[1585]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && s.b[1577]) && s.b[1579]) {
                s.copy_ad(1466, 1479);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_div_ad_rhs(1456, 1455, A::exp(A::mul(s.ad_value(154), s.ad_value(1431))));
            s.copy_ad(1466, 1479);
            s.store_div_ad(1476, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));
            s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);
            s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
        }

        s.b[1586] = ((s.v[1445] + s.v[1443]) > s.v[965]);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign25160_loop_guard: usize = 0;
        while {
            let assign25160_cond_e21369: f64 = (s.v[421] + 1.0);
            let assign25160_cond_e21371: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) && (s.v[97] <= assign25160_cond_e21369)) { 1.0 } else { 0.0 };
            assign25160_cond_e21371 != 0.0
        } {
            assign25160_loop_guard += 1;
            assert!(assign25160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) {
                s.store_add_scaled_inputs3_indices(1464, 1445, 1.0, 1443, 1.0, 965, -1.0);
                s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1445), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1443)));
            }
            s.b[1587] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);
            s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) && s.b[1587]) {
                s.store_offset(1479, 1479, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) && (!s.b[1587])) {
                s.store_sub_div_rhs_indices(1479, 1479, 1464, 1504);
            }
            s.b[1588] = (((s.v[1479] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));
            s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) && s.b[1588]) {
                s.store_offset_sub(1479, 1431, 1459, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) {
                s.store_sqrt_mul_sub_rhs(1445, 1543, 1479, 1476);
                s.store_div_scaled_inputs2_mixed_aia(1460, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), 1.0, 1459, (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
            }
            s.b[1589] = ((((s.v[1479] - s.v[1466])) as f64).abs() <= 1e-5);
            s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) && s.b[1589]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1574])) && (!s.b[1576])) && (!s.b[1577])) && s.b[1586]) {
                s.copy_ad(1466, 1479);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (s.b[1439] && s.b[1440]) {
            s.copy_ad(1478, 1479);
            s.store_scalar(1515, 0.12);
            s.store_scalar(79, 0.0);
            s.copy_ad(1457, 1476);
            s.copy_ad(1479, 1478);
            s.copy_ad(1465, 1457);
            s.copy_ad(1466, 1479);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25250_loop_guard: usize = 0;
        while {
            let assign25250_cond_e21711: f64 = (150.0 + 1.0);
            let assign25250_cond_e21713: f64 = if ((s.b[1439] && s.b[1440]) && (s.v[97] <= assign25250_cond_e21711)) { 1.0 } else { 0.0 };
            assign25250_cond_e21713 != 0.0
        } {
            assign25250_loop_guard += 1;
            assert!(assign25250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && s.b[1440]) {
                s.store_mul_sub_ad_rhs(1460, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459));
                s.store_mul(1529, 1531, 1532);
                s.store_sub(335, 1479, 1460);
            }
            s.b[1590] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
                s.store_sub_from_scalar(781, 0.1, 335);
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
            s.b[1591] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };
            s.b[1592] = (2.0 == 1.0);
            s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && s.b[1592]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1593] = (2.0 == 2.0);
            s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && s.b[1593]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1594] = (2.0 == 4.0);
            s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && s.b[1594]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1595] = (2.0 == 8.0);
            s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && (!s.b[1594])) && s.b[1595]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body29_loop_guard: usize = 0;
            while {
                let assign25250_body29_cond_e22004: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body29_cond_e22004 != 0.0
            } {
                assign25250_body29_loop_guard += 1;
                assert!(assign25250_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1590]) && (!s.b[1591])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1590])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sqrt_mul(1443, 1543, 336);
            }
            s.b[1596] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
                s.store_offset_sub(781, 1443, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
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
            s.b[1597] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };
            s.b[1598] = (2.0 == 1.0);
            s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1599] = (2.0 == 2.0);
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && s.b[1599]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1600] = (2.0 == 4.0);
            s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && s.b[1600]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1601] = (2.0 == 8.0);
            s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && (!s.b[1600])) && s.b[1601]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body65_loop_guard: usize = 0;
            while {
                let assign25250_body65_cond_e22393: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body65_cond_e22393 != 0.0
            } {
                assign25250_body65_loop_guard += 1;
                assert!(assign25250_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1596]) && (!s.b[1597])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1596])) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1596])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
                s.store_mul(1493, 1443, 1542);
                s.store_mul_ad_product_lhs(1523, A::div_from_scalar(1.034943e-10, s.ad_value(1443)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1525, A::div_from_scalar((-1.034943e-10), s.ad_value(1443)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1494, 1447, 1540);
                s.store_div_from_scalar(1527, (-1.034943e-10), 1447);
                s.store_scaled_mul(335, 1498, 1539, 8.0);
                s.store_div_scaled_inputs_product(1516, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1460), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1538), s.ad_value(1457), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1457), s.ad_value(1457), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1457), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), s.ad_value(1539), 1.0, s.ad_value(335), 1.0);
                s.store_div_ad_lhs(1517, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1457), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_div_ad_lhs(1518, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1457), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1457, 1479);
                s.store_exp(336, 335);
            }
            s.b[1602] = (s.v[1457] >= s.v[1479]);
            s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1602]) {
                s.store_mul_scaled_sqrt_ad_rhs(1471, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(1519, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 1471, 1.0);
                s.store_neg(1521, 1519);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1602])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)));
                s.store_mul_sqrt_ad_rhs(1471, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1471, 1.0);
                s.store_mul_add_ad_rhs(1519, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1521, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1603] = ((s.v[1516] > (s.v[1507] - s.v[1515])) && (s.v[1515] >= 0.0));
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_add_scaled_inputs3_indices(781, 1516, 1.0, 1507, (-1.0), 1515, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1515);
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
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1604] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };
            s.b[1605] = (4.0 == 1.0);
            s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && s.b[1605]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1606] = (4.0 == 2.0);
            s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1607] = (4.0 == 4.0);
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1607]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1608] = (4.0 == 8.0);
            s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1607])) && s.b[1608]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body126_loop_guard: usize = 0;
            while {
                let assign25250_body126_cond_e23174: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body126_cond_e23174 != 0.0
            } {
                assign25250_body126_loop_guard += 1;
                assert!(assign25250_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1603]) && (!s.b[1604])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1515, 726);
                s.store_div_scaled_product3_indices(334, 1515, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1507, 1.0, 1515, (-1.0), 780, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1603])) {
                s.copy_ad(335, 1516);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sub(1481, 1479, 335);
                s.store_mul_neg_lhs(1483, 1517, 334);
                s.store_sub_from_scalar_ad(1484, 1.0, A::mul3(s.ad_value(1518), s.ad_value(1529), s.ad_value(334)));
                s.store_add_scaled_inputs3_mixed_aii(1482, A::add_scaled_product(s.ad_value(1471), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1457)), 1.0), 1.0, 1493, 1.0, 1494, 1.0);
                s.store_sub(1485, 1519, 185);
                s.store_add_scaled_inputs_products_indices(1486, 1521, 1.0, 1523, 1.0, 1525, 1529, 1.0, 1527, 1529, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1609] = (((((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482]))) as f64).abs() > 0.5);
            s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1609]) {
                s.store_offset(1457, 1457, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1609]) {
                s.store_offset(1479, 1479, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1609])) {
                s.store_sub_ad_rhs(1457, 1457, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1479, 1479, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1610] = (((((s.v[1457] - s.v[1465])) as f64).abs() <= 1e-12) && ((((s.v[1479] - s.v[1466])) as f64).abs() <= 1e-12));
            s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1610]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.copy_ad(1465, 1457);
                s.copy_ad(1466, 1479);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1612] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.b[1613] = ((s.v[1479] > (s.v[1457] - 0.02)) && (0.02 >= 0.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_offset_sub(781, 1479, 1457, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
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

        s.b[1614] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        s.b[1615] = (2.0 == 1.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && s.b[1615]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1616] = (2.0 == 2.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && s.b[1616]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1617] = (2.0 == 4.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1617]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1618] = (2.0 == 8.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1617])) && s.b[1618]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25540_loop_guard: usize = 0;
        while {
            let assign25540_cond_e23865: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25540_cond_e23865 != 0.0
        } {
            assign25540_loop_guard += 1;
            assert!(assign25540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && (!s.b[1614])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1479, 1457, (-0.02), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && (!s.b[1613])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && (!s.b[1613])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_sub_ad_rhs(1460, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459));
            s.store_mul_sub_rhs(335, 154, 1457, 1479);
            s.store_exp(336, 335);
        }

        s.b[1619] = (s.v[1457] >= s.v[1479]);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1619]) {
            s.store_mul_scaled_sqrt_ad_rhs(1471, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
            s.copy_ad(1534, 1471);
            s.store_scalar(1513, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
        }

        s.b[1620] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
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

        s.b[1621] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        s.b[1622] = (2.0 == 1.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && s.b[1622]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1623] = (2.0 == 2.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && s.b[1623]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1624] = (2.0 == 4.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1624]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1625] = (2.0 == 8.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1624])) && s.b[1625]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25980_loop_guard: usize = 0;
        while {
            let assign25980_cond_e24396: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25980_cond_e24396 != 0.0
        } {
            assign25980_loop_guard += 1;
            assert!(assign25980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && (!s.b[1620])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && (!s.b[1620])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1619]) {
            s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1493, 1443, 1542);
            s.store_mul_neg_lhs(1494, 1447, 1540);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)));
            s.store_mul_sqrt_ad_rhs(1471, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1626] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1626]) {
            s.store_scalar(1473, 0.0);
            s.store_scalar(1513, 0.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1626])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1473, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1513, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_scalar(1534, 0.0);
            s.store_sub(335, 1479, 1460);
        }

        s.b[1627] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
            s.store_sub_from_scalar(781, 0.1, 335);
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

        s.b[1628] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1629] = (2.0 == 1.0);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && s.b[1629]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1630] = (2.0 == 2.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && s.b[1630]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1631] = (2.0 == 4.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && s.b[1631]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1632] = (2.0 == 8.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && (!s.b[1631])) && s.b[1632]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26470_loop_guard: usize = 0;
        while {
            let assign26470_cond_e25071: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26470_cond_e25071 != 0.0
        } {
            assign26470_loop_guard += 1;
            assert!(assign26470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && (!s.b[1628])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1627])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_sqrt_mul(1443, 1543, 336);
        }

        s.b[1633] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
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

        s.b[1634] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        s.b[1635] = (2.0 == 1.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && s.b[1635]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1636] = (2.0 == 2.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && s.b[1636]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1637] = (2.0 == 4.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && s.b[1637]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1638] = (2.0 == 8.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && (!s.b[1637])) && s.b[1638]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26830_loop_guard: usize = 0;
        while {
            let assign26830_cond_e25556: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26830_cond_e25556 != 0.0
        } {
            assign26830_loop_guard += 1;
            assert!(assign26830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && (!s.b[1634])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1633])) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1633])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1493, 1443, 1542);
            s.store_mul_neg_lhs(1494, 1447, 1540);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sub(335, 1479, 1460);
        }

        s.b[1639] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
            s.store_sub_from_scalar(781, 0.1, 335);
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

        s.b[1640] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        s.b[1641] = (2.0 == 1.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1642] = (2.0 == 2.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1643] = (2.0 == 4.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && s.b[1643]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1644] = (2.0 == 8.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && (!s.b[1643])) && s.b[1644]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27220_loop_guard: usize = 0;
        while {
            let assign27220_cond_e26013: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27220_cond_e26013 != 0.0
        } {
            assign27220_loop_guard += 1;
            assert!(assign27220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1639]) && (!s.b[1640])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1639])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sqrt_mul(1443, 1543, 336);
        }

        s.b[1645] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
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

        s.b[1646] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        s.b[1647] = (2.0 == 1.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && s.b[1647]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1648] = (2.0 == 2.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && s.b[1648]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1649] = (2.0 == 4.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && s.b[1649]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1650] = (2.0 == 8.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && (!s.b[1649])) && s.b[1650]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27580_loop_guard: usize = 0;
        while {
            let assign27580_cond_e26402: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27580_cond_e26402 != 0.0
        } {
            assign27580_loop_guard += 1;
            assert!(assign27580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1645]) && (!s.b[1646])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1645])) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1645])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sub(335, 1479, 1457);
        }

        s.b[1651] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_square(722, 781);
            s.store_scalar(723, (0.05 * 0.05));
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

        s.b[1652] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        s.b[1653] = (2.0 == 1.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && s.b[1653]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1654] = (2.0 == 2.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1655] = (2.0 == 4.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && s.b[1655]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1656] = (2.0 == 8.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27940_loop_guard: usize = 0;
        while {
            let assign27940_cond_e26790: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27940_cond_e26790 != 0.0
        } {
            assign27940_loop_guard += 1;
            assert!(assign27940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1651]) && (!s.b[1652])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1651])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sqrt_mul(1445, 1543, 336);
            s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1443, (-1.0), 1445, -1.0);
        }

        s.b[1657] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-18 * 1e-18));
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

        s.b[1658] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        s.b[1659] = (2.0 == 1.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && s.b[1659]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1660] = (2.0 == 2.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && s.b[1660]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1661] = (2.0 == 4.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && s.b[1661]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1662] = (2.0 == 8.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && (!s.b[1661])) && s.b[1662]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28310_loop_guard: usize = 0;
        while {
            let assign28310_cond_e27189: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28310_cond_e27189 != 0.0
        } {
            assign28310_loop_guard += 1;
            assert!(assign28310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1657]) && (!s.b[1658])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);
            s.store_sub_from_scalar(1497, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1657])) {
            s.copy_ad(1497, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_neg_lhs(1492, 1497, 1542);
        }

        s.b[1663] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        s.b[1664] = ((s.v[1457] > (s.v[1507] - 0.8)) && (0.8 >= 0.0));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
            s.store_offset_sub(781, 1457, 1507, 0.8);
            s.store_square(722, 781);
            s.store_scalar(723, (0.8 * 0.8));
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

        s.b[1665] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        s.b[1666] = (2.0 == 1.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && s.b[1666]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1667] = (2.0 == 2.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && s.b[1667]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1668] = (2.0 == 4.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1668]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1669] = (2.0 == 8.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28680_loop_guard: usize = 0;
        while {
            let assign28680_cond_e27627: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28680_cond_e27627 != 0.0
        } {
            assign28680_loop_guard += 1;
            assert!(assign28680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1507, (-0.8), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && (!s.b[1664])) {
            s.copy_ad(336, 1457);
            s.store_scalar(335, 1.0);
        }

        s.b[1670] = ((s.v[1516] > (s.v[1507] - 0.8)) && (0.8 >= 0.0));
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_offset_sub(781, 1516, 1507, 0.8);
            s.store_square(722, 781);
            s.store_scalar(723, (0.8 * 0.8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        s.b[1672] = (2.0 == 1.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && s.b[1672]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1673] = (2.0 == 2.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && s.b[1673]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1674] = (2.0 == 4.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && s.b[1674]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1675] = (2.0 == 8.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && (!s.b[1674])) && s.b[1675]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29030_loop_guard: usize = 0;
        while {
            let assign29030_cond_e28090: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29030_cond_e28090 != 0.0
        } {
            assign29030_loop_guard += 1;
            assert!(assign29030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && (!s.b[1671])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1507, (-0.8), 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && (!s.b[1670])) {
            s.copy_ad(336, 1516);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_ad_affine_product_lhs(1501, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1507)))), (-1.6021918e-19), 0.0, 1443);
        }

        s.b[1676] = (((s.v[1457] - s.v[1507]) < 0.06) && (0.06 >= 0.0));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1457), s.ad_value(1507)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.06 * 0.06));
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

        s.b[1677] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        s.b[1678] = (2.0 == 1.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && s.b[1678]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1679] = (2.0 == 2.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && s.b[1679]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1680] = (2.0 == 4.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && s.b[1680]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1681] = (2.0 == 8.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29390_loop_guard: usize = 0;
        while {
            let assign29390_cond_e28522: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29390_cond_e28522 != 0.0
        } {
            assign29390_loop_guard += 1;
            assert!(assign29390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1676]) && (!s.b[1677])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1676])) {
            s.store_sub(336, 1457, 1507);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1511, 209, -1.0, 338);
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1536, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1682] = (s.v[790] > 1e-6);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_div_square_rhs(336, 1498, 185);
            s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1434, -1.0, 2.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1683] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
            s.store_sub_from_scalar(781, 2.0, 338);
            s.store_square(722, 781);
            s.store_scalar(723, (2.0 * 2.0));
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

        s.b[1684] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        s.b[1685] = (2.0 == 1.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && s.b[1685]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1686] = (2.0 == 2.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && s.b[1686]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1687] = (2.0 == 4.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && s.b[1687]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1688] = (2.0 == 8.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && (!s.b[1687])) && s.b[1688]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29830_loop_guard: usize = 0;
        while {
            let assign29830_cond_e29048: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29830_cond_e29048 != 0.0
        } {
            assign29830_loop_guard += 1;
            assert!(assign29830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && (!s.b[1684])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1683])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_offset_lhs(344, 85, 2.0, 338);
        }

        s.b[1689] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_sub_from_scalar(781, (0.3 + 0.2), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
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
        }

    }
}
