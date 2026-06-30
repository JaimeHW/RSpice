#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1290] = (s.v[973] < 1000.0);
        s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1288]) && s.b[1290]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1287] && s.b[1288]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p.p382);
        }

        s.b[1291] = (s.v[963] == 3.0);
        s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });

        if ((s.b[1287] && (!s.b[1288])) && s.b[1291]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1293] = (s.v[973] < 1000.0);
        s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });

        if (((s.b[1287] && (!s.b[1288])) && s.b[1291]) && s.b[1293]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1287] && (!s.b[1288])) && s.b[1291]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1287] && (!s.b[1288])) && (!s.b[1291])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1287] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1294] = (p.p39 != 2.0);
        s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1294]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1287] && (!s.b[1294])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1296] = (p.p39 != 2.0);
        s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1296]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1287] && (!s.b[1296])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1298] = (s.v[682] < 0.0);
        s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1298]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1300] = (s.v[688] < 0.0);
        s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1300]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1302] = (s.v[689] < 0.0);
        s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1302]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1287] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1304] = (s.v[766] < 0.0001);
        s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });

        if ((s.b[1287] && (p.p53 != 0.0)) && s.b[1304]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1287] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1287] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1287] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1287] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1287] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1305] = (s.v[963] == 0.0);
        s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1305]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1306] = (s.v[963] == 0.0);
        s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });

        s.b[1307] = (s.v[459] != 0.0);
        s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1306]) && s.b[1307]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1308] = (s.v[460] != 0.0);
        s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1306]) && s.b[1308]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1309] = (s.v[459] != 0.0);
        s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });

        if ((s.b[1287] && (!s.b[1306])) && s.b[1309]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1310] = (s.v[460] != 0.0);
        s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });

        if ((s.b[1287] && (!s.b[1306])) && s.b[1310]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1311] = (s.v[449] == 0.0);
        s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });

        s.b[1312] = (s.v[530] > 0.0);
        s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1311]) && s.b[1312]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1313] = (p.p39 == 1.0);
        s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && s.b[1313]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && s.b[1313]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && (!s.b[1313])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1312]) && (!s.b[1313])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1287] && s.b[1311]) && (!s.b[1312])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1314] = (s.v[540] > 0.0);
        s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1311]) && s.b[1314]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1315] = (p.p39 == 1.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && s.b[1315]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && s.b[1315]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && s.b[1315]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && (!s.b[1315])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && (!s.b[1315])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1314]) && (!s.b[1315])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1287] && s.b[1311]) && (!s.b[1314])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1316] = (s.v[538] > 0.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1317] = (s.v[336] < 0.0);
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1317]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
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
        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1318] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1318]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1318])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1318])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1318])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1287] && s.b[1311]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1319] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1319]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1319]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && s.b[1319]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1319])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1319])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1287] && s.b[1311]) && s.b[1316]) && (!s.b[1319])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1287] && s.b[1311]) && (!s.b[1316])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1287] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1322] = (s.v[957] < 0.0);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1322]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1323] = (s.v[957] > 0.005);
        s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1323]) {
            s.store_scalar(957, 0.005);
        }

        s.b[1324] = (s.v[449] > 0.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1324]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }

        if (s.b[1287] && s.b[1324]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_offset_scaled(959, 387, p.p439, (((((-s.v[764])) * (p.p439))) + (s.v[959])));
        }

        if (s.b[1287] && s.b[1324]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }

        if (s.b[1287] && s.b[1324]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_offset_scaled(956, 387, p.p438, (((((-s.v[764])) * (p.p438))) + (s.v[956])));
        }

        s.b[1326] = (s.v[956] < 0.1);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1324]) && s.b[1326]) {
            s.store_scalar(956, 0.1);
        }

        if s.b[1287] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[1327] = (p.p48 > 0.0);
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        s.b[1328] = (p.p15 > s.v[632]);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1327]) && s.b[1328]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[1287] && s.b[1327]) && (!s.b[1328])) {
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
        if (s.b[1287] && (!s.b[1327])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[1287] {
            s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);
        }

        s.b[1329] = (s.v[847] > 0.0);
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1329]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));
            s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[1287] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[1330] = (p.p48 > 0.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        s.b[1331] = (p.p16 > s.v[632]);
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if ((s.b[1287] && s.b[1330]) && s.b[1331]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

        if ((s.b[1287] && s.b[1330]) && (!s.b[1331])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[1287] && (!s.b[1330])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[1287] {
            s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);
        }

        s.b[1332] = (s.v[852] > 0.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1332]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));
            s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[1287] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[1333] = (p.p15 > s.v[632]);
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1333]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[1287] && (!s.b[1333])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[1334] = (s.v[832] < 0.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1334]) {
            s.store_scalar(832, 0.0);
        }

        s.b[1335] = (s.v[833] < 0.0);
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1335]) {
            s.store_scalar(833, 0.0);
        }

        s.b[1336] = (s.v[834] < 0.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1336]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[1287] {
            s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);
            s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);
            s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);
        }

        s.b[1337] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1337]) {
            s.store_scalar(841, 0.01);
        }

        s.b[1338] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1338]) {
            s.store_scalar(842, 0.01);
        }

        s.b[1339] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1339]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1287] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[1340] = (p.p16 > s.v[632]);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1340]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[1287] && (!s.b[1340])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[1341] = (s.v[835] < 0.0);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1341]) {
            s.store_scalar(835, 0.0);
        }

        s.b[1342] = (s.v[838] < 0.0);
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1342]) {
            s.store_scalar(838, 0.0);
        }

        s.b[1343] = (s.v[839] < 0.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1343]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[1287] {
            s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);
            s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);
            s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);
        }

        s.b[1344] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1344]) {
            s.store_scalar(844, 0.01);
        }

        s.b[1345] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1345]) {
            s.store_scalar(845, 0.01);
        }

        s.b[1346] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if (s.b[1287] && s.b[1346]) {
            s.store_scalar(846, 0.01);
        }

        s.store_scaled_voltage(729, ctx, nodes, Some(6), Some(8), p.p87);

        s.store_scaled_voltage(731, ctx, nodes, Some(7), Some(8), p.p87);

        s.store_scaled_voltage(728, ctx, nodes, Some(9), Some(8), p.p87);

        s.store_scaled_voltage(733, ctx, nodes, Some(0), Some(2), p.p87);

        s.store_scaled_voltage(734, ctx, nodes, Some(7), Some(2), p.p87);

        s.store_scaled_voltage(735, ctx, nodes, Some(9), Some(2), p.p87);

        s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(6), p.p87);

        s.store_scaled_voltage(804, ctx, nodes, Some(8), Some(2), p.p87);

        s.store_scaled_voltage(857, ctx, nodes, Some(11), Some(2), p.p87);

        s.store_scaled_voltage(858, ctx, nodes, Some(10), Some(0), p.p87);

        s.store_scaled_voltage(865, ctx, nodes, Some(9), Some(8), p.p87);

        s.store_scaled_voltage(866, ctx, nodes, Some(9), Some(6), p.p87);

        s.copy_ad(859, 857);

        s.copy_ad(860, 858);

        s.copy_ad(867, 865);

        s.copy_ad(868, 866);

        s.store_scaled_voltage(798, ctx, nodes, Some(4), Some(2), p.p87);

        if (s.v[81] != 0.0) {
            s.store_voltage(747, ctx, nodes, Some(12), None);
            s.store_voltage(748, ctx, nodes, Some(13), None);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(747, 0.0);
            s.store_scalar(748, 0.0);
        }

        s.store_sub(730, 731, 729);

        s.store_sub(727, 728, 729);

        s.b[1347] = (s.v[729] >= 0.0);
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if s.b[1347] {
            s.store_scalar(949, 1.0);
            s.copy_ad(790, 729);
            s.copy_ad(791, 731);
            s.copy_ad(792, 728);
            s.copy_ad(793, 733);
            s.copy_ad(796, 734);
            s.copy_ad(797, 735);
        }

        if (!s.b[1347]) {
            s.store_scalar(949, (-1.0));
            s.store_neg(790, 729);
            s.copy_ad(791, 730);
            s.copy_ad(792, 727);
            s.store_neg(793, 733);
            s.store_sub(796, 734, 733);
            s.store_sub(797, 735, 733);
        }

        s.b[1350] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });

        if s.b[1350] {
            s.store_voltage(732, ctx, nodes, Some(5), None);
        }

        s.b[1351] = (p.p53 == 2.0);
        s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1351]) {
            s.store_offset_sub_from_scalar_ad(781, p.p433, s.ad_value(732), (-(p.p337 * 10.0)));
            s.store_scalar(782, ((4.0 * p.p433) * (p.p337 * 10.0)));
        }

        if (s.b[1350] && s.b[1351]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1350] && s.b[1351]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p.p433);
        }

        if s.b[1350] {
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
        if s.b[1350] {
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

        s.b[1353] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1353]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1355] = (s.v[973] < 1000.0);
        s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1353]) && s.b[1355]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1350] && s.b[1353]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_ad_rhs(970, 970, A::powf(s.ad_value(676), p.p382));
        }

        s.b[1356] = (s.v[963] == 3.0);
        s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });

        if ((s.b[1350] && (!s.b[1353])) && s.b[1356]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1358] = (s.v[973] < 1000.0);
        s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });

        if (((s.b[1350] && (!s.b[1353])) && s.b[1356]) && s.b[1358]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1350] && (!s.b[1353])) && s.b[1356]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1350] && (!s.b[1353])) && (!s.b[1356])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1350] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1359] = (p.p39 != 2.0);
        s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1359]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1350] && (!s.b[1359])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1361] = (p.p39 != 2.0);
        s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1361]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1350] && (!s.b[1361])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1363] = (s.v[682] < 0.0);
        s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1363]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1365] = (s.v[688] < 0.0);
        s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1365]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1367] = (s.v[689] < 0.0);
        s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1367]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1350] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1369] = (s.v[766] < 0.0001);
        s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });

        if ((s.b[1350] && (p.p53 != 0.0)) && s.b[1369]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1350] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1350] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1350] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1350] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1350] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1370] = (s.v[963] == 0.0);
        s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1370]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1371] = (s.v[963] == 0.0);
        s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });

        s.b[1372] = (s.v[459] != 0.0);
        s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1371]) && s.b[1372]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1373] = (s.v[460] != 0.0);
        s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1371]) && s.b[1373]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1374] = (s.v[459] != 0.0);
        s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });

        if ((s.b[1350] && (!s.b[1371])) && s.b[1374]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1375] = (s.v[460] != 0.0);
        s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });

        if ((s.b[1350] && (!s.b[1371])) && s.b[1375]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1376] = (s.v[449] == 0.0);
        s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });

        s.b[1377] = (s.v[530] > 0.0);
        s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1376]) && s.b[1377]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1378] = (p.p39 == 1.0);
        s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && s.b[1378]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1377]) && (!s.b[1378])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1350] && s.b[1376]) && (!s.b[1377])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1379] = (s.v[540] > 0.0);
        s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1376]) && s.b[1379]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1380] = (p.p39 == 1.0);
        s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });

        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {
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
        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && s.b[1380]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1350] && s.b[1376]) && (!s.b[1379])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1381] = (s.v[538] > 0.0);
        s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1382] = (s.v[336] < 0.0);
        s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1382]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1383] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1383])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1350] && s.b[1376]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1384] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && s.b[1384]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1350] && s.b[1376]) && s.b[1381]) && (!s.b[1384])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1350] && s.b[1376]) && (!s.b[1381])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1350] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1387] = (s.v[957] < 0.0);
        s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1387]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1388] = (s.v[957] > 0.005);
        s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });

        if (s.b[1350] && s.b[1388]) {
            s.store_scalar(957, 0.005);
        }

        if (!s.b[1350]) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        s.store_scalar(164, (s.v[630] * p.p7));

        s.store_scalar(165, (p.p67 + p.p68));

        s.store_scalar(160, s.v[462]);

        s.copy_ad(257, 681);

        s.store_scalar(161, s.v[617]);

        s.store_scalar(187, p.p95);

        s.store_scalar(188, (s.v[161] / s.v[187]));

        s.store_scalar(189, (1.0 / s.v[188]));

        s.store_div_from_scalar(412, s.v[161], 543);

        s.store_scalar(270, (p.p87 * p.p434));

        s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p.p262)), (-0.1));

        s.store_scalar(782, ((4.0 * 0.8) * 0.1));

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

        s.b[1389] = ((s.v[158] - p.p262) < s.v[69]);
        s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });

        if s.b[1389] {
            s.store_offset(69, 158, (-p.p262));
        }

        s.b[1390] = ((s.v[159] - p.p262) < s.v[69]);
        s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });

        if s.b[1390] {
            s.store_offset(69, 159, (-p.p262));
        }

        s.b[1391] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));
        s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });

        s.b[1392] = ((s.v[961] - p.p262) < s.v[69]);
        s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });

        if (s.b[1391] && s.b[1392]) {
            s.store_offset(69, 961, (-p.p262));
        }

        s.b[1393] = ((s.v[960] - p.p262) < s.v[69]);
        s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });

        if (s.b[1391] && s.b[1393]) {
            s.store_offset(69, 960, (-p.p262));
        }

        s.b[1394] = (s.v[70] > (s.v[69] * 0.5));
        s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });

        if s.b[1394] {
            s.store_scale(70, 69, 0.5);
        }

        s.b[1395] = param_given[338];
        s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });

        if s.b[1395] {
            s.store_scalar(72, p.p338);
        }

        if (!s.b[1395]) {
            s.copy_ad(72, 69);
        }

        s.b[1396] = param_given[339];
        s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });

        if s.b[1396] {
            s.store_scalar(73, p.p339);
        }

        s.b[1397] = param_given[338];
        s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });

        if ((!s.b[1396]) && s.b[1397]) {
            s.store_scale(73, 72, 0.5);
        }

        if ((!s.b[1396]) && (!s.b[1397])) {
            s.copy_ad(73, 70);
        }

        s.b[1398] = (s.v[73] > (s.v[72] * 0.5));
        s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });

        if s.b[1398] {
            s.store_scale(73, 72, 0.5);
        }

        s.b[1399] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));
        s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });

        s.b[1400] = (s.v[448] == 1.0);
        s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });

        if (s.b[1399] && s.b[1400]) {
            s.store_scalar(74, 1.0);
        }

        s.b[1401] = (s.v[448] == 2.0);
        s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });

        if (s.b[1399] && s.b[1401]) {
            s.store_scalar(74, 2.0);
        }

        s.b[1402] = (s.v[448] == 3.0);
        s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });

        if (s.b[1399] && s.b[1402]) {
            s.store_scalar(74, 3.0);
        }

        s.store_scalar(77, 0.0);

        s.b[1403] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });

        if s.b[1403] {
            s.copy_ad(373, 733);
        }

        s.b[1404] = (s.v[373] >= 0.0);
        s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });

        if (s.b[1403] && s.b[1404]) {
            s.copy_ad(376, 373);
            s.copy_ad(383, 798);
        }

        if (s.b[1403] && (!s.b[1404])) {
            s.store_neg(376, 373);
            s.store_sub(383, 798, 373);
        }

        if s.b[1403] {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1405] = (s.v[108] < 1e-12);
        s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });

        if (s.b[1403] && s.b[1405]) {
            s.store_scalar(108, 1e-12);
        }

        if s.b[1403] {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1406] = (s.v[335] < 0.0);
        s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });

        if (s.b[1403] && s.b[1406]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if s.b[1403] {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if s.b[1403] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1403] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
        }

        if (!s.b[1403]) {
            s.store_scalar(384, 0.0);
        }

        s.b[1407] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));
        s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });

        if s.b[1407] {
            s.copy_ad(373, 733);
            s.copy_ad(374, 734);
            s.copy_ad(372, 735);
        }

        s.b[1408] = (s.v[373] >= 0.0);
        s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });

        if (s.b[1407] && s.b[1408]) {
            s.store_scalar(370, 1.0);
            s.store_scalar(371, 0.0);
            s.copy_ad(376, 373);
            s.copy_ad(377, 374);
            s.copy_ad(375, 372);
            s.copy_ad(383, 798);
        }

        if (s.b[1407] && (!s.b[1408])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 1.0);
            s.store_neg(376, 373);
            s.store_sub(377, 374, 373);
            s.store_sub(375, 372, 373);
            s.store_sub(383, 798, 373);
        }

        s.b[1409] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));
        s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });

        if (s.b[1407] && s.b[1409]) {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1410] = (s.v[108] < 1e-12);
        s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });

        if ((s.b[1407] && s.b[1409]) && s.b[1410]) {
            s.store_scalar(108, 1e-12);
        }

        if (s.b[1407] && s.b[1409]) {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_add(381, 377, 108);
            s.store_add(382, 375, 108);
        }

        s.b[1411] = ((p.p34 == 1.0) || (s.v[370] == 1.0));
        s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);
            s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);
            s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(344, 782, p.p292, 0.5);
        }

        s.b[1412] = (s.v[344] < 0.0);
        s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });

        if (((s.b[1407] && s.b[1409]) && s.b[1411]) && s.b[1412]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1413] = (s.v[337] < 0.0);
        s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });

        if (((s.b[1407] && s.b[1409]) && s.b[1411]) && s.b[1413]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1411]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 366);
            s.store_mul(366, 366, 337);
        }

        if ((s.b[1407] && s.b[1409]) && (!s.b[1411])) {
            s.copy_ad(366, 691);
        }

        if (s.b[1407] && s.b[1409]) {
            s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);
        }

        s.b[1414] = ((p.p34 == 1.0) || (s.v[371] == 1.0));
        s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);
            s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));
            s.store_scalar(344, (p.p292 + 1e-25));
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1415] = (s.v[337] < 0.0);
        s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });

        if (((s.b[1407] && s.b[1409]) && s.b[1414]) && s.b[1415]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1414]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 367);
            s.store_mul(367, 367, 337);
        }

        if ((s.b[1407] && s.b[1409]) && (!s.b[1414])) {
            s.copy_ad(367, 691);
        }

        s.b[1416] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });

        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1417] = (s.v[335] < 0.0);
        s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });

        if (((s.b[1407] && s.b[1409]) && s.b[1416]) && s.b[1417]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1407] && s.b[1409]) && s.b[1416]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
            s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));
            s.store_mul(334, 366, 340);
            s.store_mul(335, 367, 340);
            s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);
            s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);
        }

        if ((s.b[1407] && s.b[1409]) && (!s.b[1416])) {
            s.store_scalar(384, 0.0);
        }

        if (s.b[1407] && s.b[1409]) {
            s.copy_ad(4, 366);
            s.copy_ad(5, 367);
        }

        if (s.b[1407] && (!s.b[1409])) {
            s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);
        }

        if s.b[1407] {
            s.store_scale(4, 4, 1.0 / (s.v[164]));
            s.store_scale(5, 5, 1.0 / (s.v[164]));
            s.store_add_scaled_value_products(4, s.ad_value(4), 1.0, s.ad_value(370), s.ad_value(644), 1.0, s.ad_value(371), s.ad_value(648), 1.0);
            s.store_add_scaled_value_products(5, s.ad_value(5), 1.0, s.ad_value(370), s.ad_value(648), 1.0, s.ad_value(371), s.ad_value(644), 1.0);
            s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);
            s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);
        }

        s.b[1420] = (s.v[792] > s.v[70]);
        s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });

        if s.b[1420] {
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

        if (!s.b[1420]) {
            s.copy_ad(83, 792);
            s.store_scalar(84, 1.0);
        }

        s.store_scaled_mul(335, 84, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(108, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1421] = (s.v[108] < 1e-12);
        s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });

        if s.b[1421] {
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

        s.b[1422] = (s.v[338] < 0.0);
        s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });

        if s.b[1422] {
            s.store_scalar(338, 0.0);
            s.store_scalar(339, 0.0);
        }

        s.store_offset(338, 338, 1e-25);

        s.store_sqrt(332, 338);

        s.store_add_mul_sub_from_scalar_rhs_indices(128, 336, 335, 1.0, 332);

        s.store_sub(129, 128, 159);

        s.store_offset(781, 129, (((-0.1)) + ((-0.05))));

        s.store_scalar(782, ((4.0 * 0.1) * 0.05));

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

        s.b[1423] = (s.v[765] == 0.0);
        s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });

        if s.b[1423] {
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1423]) {
            s.store_scalar(80, 1.0);
        }

        s.copy_ad(335, 637);

        s.store_sqrt_mul(336, 335, 158);

        s.store_add_scaled_ad_lhs(190, A::offset(s.ad_value(158), s.v[160]), 336, s.v[189]);

        s.b[1424] = (s.v[80] == 0.0);
        s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });

        if s.b[1424] {
            s.store_scalar(183, s.v[187]);
            s.store_scalar(185, s.v[188]);
            s.store_scalar(186, s.v[189]);
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        if (!s.b[1424]) {
            s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);
            s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));
            s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);
            s.store_scaled_add(336, 339, 782, 0.5);
        }

        s.b[1425] = (s.v[336] < 0.0);
        s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });

        if ((!s.b[1424]) && s.b[1425]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[1424]) {
            s.store_offset(336, 336, 1e-25);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1424]) {
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));
            s.store_scaled_abs(338, 190, 2.0);
            s.store_offset_sub(340, 339, 791, s.v[160]);
        }

        s.b[1426] = (s.v[340] > s.v[338]);
        s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });

        if ((!s.b[1424]) && s.b[1426]) {
            s.copy_ad(338, 340);
        }

        if (!s.b[1424]) {
            s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));
            s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));
        }

        if (!s.b[1424]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (!s.b[1424]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));
            s.store_offset_scaled(184, 336, p.p235, p.p237);
            s.store_scalar(341, p.p235);
        }

        s.b[1427] = ((s.v[184] * 1000000000000.0) < s.v[187]);
        s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });

        if ((!s.b[1424]) && s.b[1427]) {
            s.store_scalar(184, 0.0);
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1424]) {
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

        s.store_scalar(334, 0.95);

        s.b[338] = (!(s.v[963] > 1.0));
        s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });

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

        s.b[1428] = (p.p140 != 0.0);
        s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });

        if s.b[1428] {
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

        if s.b[1428] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1428] {
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
            s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);
            s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));
            s.store_sub(335, 173, 119);
            s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p.p140)), s.v[465]);
            s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);
            s.store_offset(178, 106, p.p221);
            s.store_square(179, 178);
            s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);
        }

        if (!s.b[1428]) {
            s.store_scalar(174, 0.0);
        }

        s.store_scale(335, 186, 1.034943e-10);

        s.copy_ad(336, 684);

        s.store_scalar(337, (s.v[582] - p.p139));

        s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));

        s.store_mul_scaled_ad_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338]);

        s.store_mul(121, 339, 181);

        s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);

        s.store_mul_scale_ad_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 181);

        s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);

        s.store_scalar(335, (s.v[470] / s.v[582]));

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

        s.store_scalar(182, 0.0);

        s.b[1429] = (s.v[615] == 1.0);
        s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });

        if s.b[1429] {
            s.copy_ad(341, 107);
            s.copy_ad(334, 642);
            s.store_offset(337, 341, (-p.p152));
        }

        s.b[1430] = (s.v[337] < (-3.0));
        s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });

        if (s.b[1429] && s.b[1430]) {
            s.store_scalar(340, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[1431] = (s.v[337] < 0.0);
        s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });

        if ((s.b[1429] && (!s.b[1430])) && s.b[1431]) {
            s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if ((s.b[1429] && (!s.b[1430])) && (!s.b[1431])) {
            s.store_offset_mul_offset_rhs_ad_rhs(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if s.b[1429] {
            s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));
            s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);
        }

        s.b[1432] = (s.v[182] < 0.0);
        s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });

        if (s.b[1429] && s.b[1432]) {
            s.store_scalar(182, 0.0);
            s.store_scalar(340, 0.0);
        }

        if s.b[1429] {
            s.store_mul(182, 182, 334);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1429] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1429] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);
        }

        s.b[1439] = (s.v[792] > s.v[73]);
        s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });

        if ((p.p37 != 0.0) && s.b[1439]) {
            s.store_sub(335, 792, 73);
            s.store_sub(336, 72, 73);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(1434, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1434, 1.0);
            s.store_neg(1434, 1434);
            s.store_add(1433, 73, 333);
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
        if ((p.p37 != 0.0) && s.b[1439]) {
            s.store_div_scaled_inputs_product(1434, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

        if ((p.p37 != 0.0) && (!s.b[1439])) {
            s.copy_ad(1433, 792);
            s.store_scalar(1434, 1.0);
        }

        if (p.p37 == 0.0) {
            s.copy_ad(1433, 792);
            s.store_scalar(1434, 1.0);
        }

        s.store_scaled_mul(335, 1434, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(1435, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1440] = (s.v[1435] < 1e-12);
        s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });

        if s.b[1440] {
            s.store_scalar(1435, 1e-12);
        }

        s.store_add(1436, 1433, 1435);

        s.store_add_scaled_inputs(1437, 790, 1.0, 1435, 2.0);

        s.store_add(1438, 791, 1435);

        s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));

        s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1433, 1.0, s.v[160]);

        s.b[1441] = (s.v[963] != 0.0);
        s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });

        s.b[1442] = (p.p42 == 1.0);
        s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });

        s.b[1443] = (p.p42 == 2.0);
        s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });

        s.b[1444] = (p.p42 == 3.0);
        s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });

        if (s.b[1441] && s.b[1442]) {
            s.copy_ad(1461, 960);
            s.store_scale(1544, 964, 1.6021918e-19);
            s.store_square(1543, 964);
            s.store_scale(1500, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1542, 622, 1.6021918e-19);
            s.store_scalar(1539, (1.6021918e-19 * 1.6021918e-19));
            s.store_scalar(1540, (1.034943e-10 * 1.034943e-10));
            s.store_square(1541, 965);
            s.store_div_from_scalar(1545, (2.0 * 1.034943e-10), 1544);
            s.store_scale(1546, 1544, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_scale(1547, 1544, (2.0 * 1.034943e-10));
            s.store_div_from_scalar(1548, (2.0 * 1.034943e-10), 1542);
            s.store_scale(1549, 1542, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_div(1534, 964, 622);
            s.store_div_from_scalar_offset_input(1533, 1.0, 1534, 1.0);
            s.store_scalar(1550, (1e-12 * 1000.0));
            s.store_scalar(1551, (1e-10 * 1000.0));
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_scalar(1474, 0.0);
            s.store_scalar(1515, 0.0);
            s.store_scalar(1516, 0.0);
            s.store_scalar(1495, 0.0);
            s.store_scalar(1497, 0.0);
            s.store_scalar(1496, 0.0);
            s.store_scalar(1498, 0.0);
            s.store_scalar(1518, 0.0);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));
            s.store_div_scaled_product_by_product(1454, s.ad_value(185), s.ad_value(185), 1.0, s.ad_value(209), s.ad_value(209), 1.0);
            s.store_mul_ad_lhs(1457, A::div_scaled_value_by_product(s.ad_value(1454), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1543);
            s.store_sqrt_mul_ad(1451, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1461), s.ad_value(1433)));
        }

        s.b[1557] = (s.v[1451] > s.v[965]);
        s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
            s.store_scalar(1464, 0.0);
            s.copy_ad(1445, 965);
            s.store_scalar(1481, 0.0);
            s.store_sub_ad_rhs(1462, 1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)));
            s.store_scalar(1509, 0.0);
            s.copy_ad(1508, 1464);
            s.copy_ad(1470, 1462);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        let mut assign24620_loop_guard: usize = 0;
        while {
            let assign24620_cond_e19015: f64 = (150.0 + 1.0);
            let assign24620_cond_e19017: f64 = if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (s.v[97] <= assign24620_cond_e19015)) { 1.0 } else { 0.0 };
            assign24620_cond_e19017 != 0.0
        } {
            assign24620_loop_guard += 1;
            assert!(assign24620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
                s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
            }
            s.b[1558] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
                s.store_offset_sub(781, 1445, 965, 1e-8);
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
            s.b[1559] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
            s.b[1560] = (2.0 == 1.0);
            s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && s.b[1560]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1561] = (2.0 == 2.0);
            s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && s.b[1561]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1562] = (2.0 == 4.0);
            s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && s.b[1562]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1563] = (2.0 == 8.0);
            s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24620_body27_loop_guard: usize = 0;
            while {
                let assign24620_body27_cond_e19333: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24620_body27_cond_e19333 != 0.0
            } {
                assign24620_body27_loop_guard += 1;
                assert!(assign24620_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && (!s.b[1559])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1445, 965, (-1e-8), 780);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
                s.store_add_scaled_inputs3_indices(335, 1462, 1.0, 1433, (-1.0), 1461, 1.0);
            }
            s.b[1564] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
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
            s.b[1565] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            s.b[1566] = (2.0 == 1.0);
            s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && s.b[1566]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1567] = (2.0 == 2.0);
            s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && s.b[1567]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1568] = (2.0 == 4.0);
            s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1569] = (2.0 == 8.0);
            s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) && s.b[1569]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24620_body63_loop_guard: usize = 0;
            while {
                let assign24620_body63_cond_e19787: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24620_body63_cond_e19787 != 0.0
            } {
                assign24620_body63_loop_guard += 1;
                assert!(assign24620_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && (!s.b[1565])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1564])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
                s.store_sqrt_mul(1449, 1548, 336);
                s.store_mul(1495, 1445, 1544);
                s.store_mul_div_from_scalar_lhs(1527, (-1.034943e-10), 1445, 334);
                s.store_mul_neg_lhs(1496, 1449, 1542);
                s.store_mul_div_from_scalar_lhs(1529, (-1.034943e-10), 1449, 341);
                s.store_add_ad_lhs(1483, A::add_scaled_product(s.ad_value(1495), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1481)), 1.0), 1496);
                s.copy_ad(1485, 185);
                s.store_add(1486, 1527, 1529);
                s.store_add_scaled_product_right_ad(1484, 1462, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461)), (-1.0));
                s.store_scalar(1487, 0.0);
                s.store_scalar(1488, 1.0);
                s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));
                s.store_div(1490, 1488, 1489);
                s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);
                s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);
                s.store_div(1493, 1485, 1489);
            }
            s.b[1570] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);
            s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {
                s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {
                s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1570])) {
                s.store_sub_ad_rhs(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));
                s.store_sub_ad_rhs(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));
            }
            s.b[1571] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1462] - s.v[1470])) as f64).abs() <= 1e-12));
            s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1571]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
                s.copy_ad(1508, 1464);
                s.copy_ad(1470, 1462);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
            s.copy_ad(1511, 1462);
            s.store_mul(1449, 965, 1534);
            s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);
            s.store_add_scaled_product_indices(1481, 1462, 1.0, 1546, 1541, 1.0);
            s.copy_ad(1459, 1481);
            s.copy_ad(1465, 1481);
            s.copy_ad(1507, 1481);
        }

        s.b[1572] = (s.v[85] > s.v[1464]);
        s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1572]) {
            s.store_scalar(1477, 1.0);
        }

        s.b[1573] = (s.v[85] > s.v[1507]);
        s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && s.b[1573]) {
            s.store_scalar(1477, 3.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && (!s.b[1573])) {
            s.store_scalar(1477, 2.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1557])) {
            s.store_scalar(1464, 0.0);
            s.copy_ad(1507, 1464);
            s.store_scalar(1465, 0.0);
            s.copy_ad(1509, 1464);
            s.copy_ad(1445, 1451);
            s.store_mul(1449, 1445, 1534);
            s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);
            s.store_add_ad_lhs(1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)), 1462);
            s.copy_ad(1511, 1462);
        }

        s.b[1574] = (s.v[85] > s.v[1464]);
        s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && s.b[1574]) {
            s.store_scalar(1477, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && (!s.b[1574])) {
            s.store_scalar(1477, 2.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1547, s.ad_value(1465), 1.0, s.ad_value(1433), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1575] = (s.v[335] > 0.0);
        s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1575]) {
            s.store_add_scaled_inputs3_mixed_iia(1453, 1433, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1575])) {
            s.store_sub(1453, 1433, 961);
        }

        s.b[1576] = (s.v[85] > s.v[1464]);
        s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1576]) {
            s.copy_ad(1462, 1511);
            s.store_scalar(1481, 0.0);
            s.store_add_div_lhs(1478, A::ln(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1481);
        }

        s.b[1577] = (s.v[1478] < (s.v[1509] + s.v[1551]));
        s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1576]) && s.b[1577]) {
            s.store_add(1478, 1509, 1551);
        }

        s.b[1578] = (s.v[85] > s.v[1507]);
        s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1576])) && s.b[1578]) {
            s.copy_ad(1478, 1459);
        }

        s.b[1579] = (s.v[85] > s.v[1453]);
        s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {
            s.store_add_scaled_product_indices(1455, 154, 1.0, 1454, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));
            s.copy_ad(1468, 1481);
            s.store_div_scaled_inputs2_mixed_aii(1478, A::sqrt(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, 1455, (-0.5), 1454, 1.0);
        }

        s.b[1580] = (s.v[1478] > (s.v[1465] - s.v[1550]));
        s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1580]) {
            s.store_sub(1478, 1465, 1550);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {
            s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);
            s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
        }

        s.b[1581] = ((s.v[1447] + s.v[1445]) > s.v[965]);
        s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25100_loop_guard: usize = 0;
        while {
            let assign25100_cond_e20799: f64 = (150.0 + 1.0);
            let assign25100_cond_e20801: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (s.v[97] <= assign25100_cond_e20799)) { 1.0 } else { 0.0 };
            assign25100_cond_e20801 != 0.0
        } {
            assign25100_loop_guard += 1;
            assert!(assign25100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
                s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);
                s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));
            }
            s.b[1582] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);
            s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1582]) {
                s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1582])) {
                s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);
            }
            s.b[1583] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));
            s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1583]) {
                s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
                s.store_add_scaled_product_value_ad(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1455, 1.0, 1454, 1456, (-4.0));
            }
            s.b[1584] = (s.v[335] > 0.0);
            s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1584]) {
                s.store_div_scaled_inputs2_mixed_aii(1478, A::sqrt(s.ad_value(335)), 0.5, 1455, (-0.5), 1454, 1.0);
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1584])) {
                s.store_div_scaled_inputs_indices(1478, 1455, (-0.5), 1454, 1.0);
            }
            s.b[1585] = (s.v[1478] > s.v[1465]);
            s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1585]) {
                s.copy_ad(1478, 1465);
            }
            s.b[1586] = (s.v[1478] > s.v[1481]);
            s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1586]) {
                s.store_sub(1478, 1481, 1551);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
                s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);
                s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
            }
            s.b[1587] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-8);
            s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1587]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
                s.copy_ad(1468, 1481);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {
            s.store_div_ad_rhs(1458, 1457, A::exp(A::mul(s.ad_value(154), s.ad_value(1433))));
            s.copy_ad(1468, 1481);
            s.store_div_ad(1478, A::ln(A::mul3(s.ad_value(1458), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));
            s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);
            s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
        }

        s.b[1588] = ((s.v[1447] + s.v[1445]) > s.v[965]);
        s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign25180_loop_guard: usize = 0;
        while {
            let assign25180_cond_e21382: f64 = (s.v[421] + 1.0);
            let assign25180_cond_e21384: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (s.v[97] <= assign25180_cond_e21382)) { 1.0 } else { 0.0 };
            assign25180_cond_e21384 != 0.0
        } {
            assign25180_loop_guard += 1;
            assert!(assign25180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
                s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);
                s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));
            }
            s.b[1589] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);
            s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1589]) {
                s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (!s.b[1589])) {
                s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);
            }
            s.b[1590] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));
            s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1590]) {
                s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
                s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);
                s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
            }
            s.b[1591] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-5);
            s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1591]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
                s.copy_ad(1468, 1481);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (s.b[1441] && s.b[1442]) {
            s.copy_ad(1480, 1481);
            s.store_scalar(1517, 0.12);
            s.store_scalar(79, 0.0);
            s.copy_ad(1459, 1478);
            s.copy_ad(1481, 1480);
            s.copy_ad(1467, 1459);
            s.copy_ad(1468, 1481);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25270_loop_guard: usize = 0;
        while {
            let assign25270_cond_e21724: f64 = (150.0 + 1.0);
            let assign25270_cond_e21726: f64 = if ((s.b[1441] && s.b[1442]) && (s.v[97] <= assign25270_cond_e21724)) { 1.0 } else { 0.0 };
            assign25270_cond_e21726 != 0.0
        } {
            assign25270_loop_guard += 1;
            assert!(assign25270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && s.b[1442]) {
                s.store_mul_sub_ad_rhs(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461));
                s.store_mul(1531, 1533, 1534);
                s.store_sub(335, 1481, 1462);
            }
            s.b[1592] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
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
            s.b[1593] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
            s.b[1594] = (2.0 == 1.0);
            s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && s.b[1594]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1595] = (2.0 == 2.0);
            s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && s.b[1595]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1596] = (2.0 == 4.0);
            s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && s.b[1596]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1597] = (2.0 == 8.0);
            s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && (!s.b[1596])) && s.b[1597]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body29_loop_guard: usize = 0;
            while {
                let assign25270_body29_cond_e22017: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body29_cond_e22017 != 0.0
            } {
                assign25270_body29_loop_guard += 1;
                assert!(assign25270_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && (!s.b[1593])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1592])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sqrt_mul(1445, 1545, 336);
            }
            s.b[1598] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
                s.store_offset_sub(781, 1445, 965, 1e-8);
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
            s.b[1599] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
            s.b[1600] = (2.0 == 1.0);
            s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && s.b[1600]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1601] = (2.0 == 2.0);
            s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && s.b[1601]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1602] = (2.0 == 4.0);
            s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && s.b[1602]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1603] = (2.0 == 8.0);
            s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && (!s.b[1602])) && s.b[1603]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body65_loop_guard: usize = 0;
            while {
                let assign25270_body65_cond_e22406: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body65_cond_e22406 != 0.0
            } {
                assign25270_body65_loop_guard += 1;
                assert!(assign25270_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && (!s.b[1599])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1445, 965, (-1e-8), 780);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
                s.store_mul(1495, 1445, 1544);
                s.store_mul_ad_product_lhs_mixed_ai(1525, A::div_from_scalar(1.034943e-10, s.ad_value(1445)), 334, 337);
                s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar((-1.034943e-10), s.ad_value(1445)), 334, 337);
                s.store_mul_neg_lhs(1496, 1449, 1542);
                s.store_div_from_scalar(1529, (-1.034943e-10), 1449);
                s.store_scaled_mul(335, 1500, 1541, 8.0);
                s.store_div_scaled_inputs_product(1518, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1462), s.ad_value(1540), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1540), s.ad_value(1459), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1459), s.ad_value(1459), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1459), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), s.ad_value(1541), 1.0, s.ad_value(335), 1.0);
                s.store_div_ad_lhs(1519, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1459), (4.0 * 2.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_div_ad_lhs(1520, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1459), (-8.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1459, 1481);
                s.store_exp(336, 335);
            }
            s.b[1604] = (s.v[1459] >= s.v[1481]);
            s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1604]) {
                s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(1521, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 1473, 1.0);
                s.store_neg(1523, 1521);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1604])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));
                s.store_mul_sqrt_ad_rhs(1473, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1473, 1.0);
                s.store_mul_add_ad_rhs(1521, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1523, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1605] = ((s.v[1518] > (s.v[1509] - s.v[1517])) && (s.v[1517] >= 0.0));
            s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
                s.store_add_scaled_inputs3_indices(781, 1518, 1.0, 1509, (-1.0), 1517, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1517);
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
            s.b[1606] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
            s.b[1607] = (4.0 == 1.0);
            s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && s.b[1607]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1608] = (4.0 == 2.0);
            s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1609] = (4.0 == 4.0);
            s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && s.b[1609]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1610] = (4.0 == 8.0);
            s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (!s.b[1609])) && s.b[1610]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body126_loop_guard: usize = 0;
            while {
                let assign25270_body126_cond_e23187: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body126_cond_e23187 != 0.0
            } {
                assign25270_body126_loop_guard += 1;
                assert!(assign25270_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && (!s.b[1606])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1517, 726);
                s.store_div_scaled_product3_indices(334, 1517, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1509, 1.0, 1517, (-1.0), 780, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1605])) {
                s.copy_ad(335, 1518);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sub(1483, 1481, 335);
                s.store_mul_neg_lhs(1485, 1519, 334);
                s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1531), s.ad_value(334)));
                s.store_add_scaled_inputs3_mixed_aii(1484, A::add_scaled_product(s.ad_value(1473), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1459)), 1.0), 1.0, 1495, 1.0, 1496, 1.0);
                s.store_sub(1487, 1521, 185);
                s.store_add_scaled_inputs_products_indices(1488, 1523, 1.0, 1525, 1.0, 1527, 1531, 1.0, 1529, 1531, 1.0);
                s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));
                s.store_div(1490, 1488, 1489);
                s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);
                s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);
                s.store_div(1493, 1485, 1489);
            }
            s.b[1611] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);
            s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {
                s.store_offset(1459, 1459, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {
                s.store_offset(1481, 1481, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1611])) {
                s.store_sub_ad_rhs(1459, 1459, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));
                s.store_sub_ad_rhs(1481, 1481, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));
            }
            s.b[1612] = (((((s.v[1459] - s.v[1467])) as f64).abs() <= 1e-12) && ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-12));
            s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1612]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.copy_ad(1467, 1459);
                s.copy_ad(1468, 1481);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1614] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.b[1615] = ((s.v[1481] > (s.v[1459] - 0.02)) && (0.02 >= 0.0));
        s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
            s.store_offset_sub(781, 1481, 1459, 0.02);
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

        s.b[1616] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });

        s.b[1617] = (2.0 == 1.0);
        s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && s.b[1617]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1618] = (2.0 == 2.0);
        s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && s.b[1618]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1619] = (2.0 == 4.0);
        s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && s.b[1619]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1620] = (2.0 == 8.0);
        s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && (!s.b[1619])) && s.b[1620]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25560_loop_guard: usize = 0;
        while {
            let assign25560_cond_e23878: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25560_cond_e23878 != 0.0
        } {
            assign25560_loop_guard += 1;
            assert!(assign25560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && (!s.b[1616])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1481, 1459, (-0.02), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_sub_ad_rhs(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461));
            s.store_mul_sub_rhs(335, 154, 1459, 1481);
            s.store_exp(336, 335);
        }

        s.b[1621] = (s.v[1459] >= s.v[1481]);
        s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {
            s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
            s.copy_ad(1536, 1473);
            s.store_scalar(1515, 0.0);
            s.store_scalar(1475, 0.0);
            s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);
        }

        s.b[1622] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1623] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });

        s.b[1624] = (2.0 == 1.0);
        s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && s.b[1624]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1625] = (2.0 == 2.0);
        s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && s.b[1625]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1626] = (2.0 == 4.0);
        s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && s.b[1626]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1627] = (2.0 == 8.0);
        s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26000_loop_guard: usize = 0;
        while {
            let assign26000_cond_e24409: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26000_cond_e24409 != 0.0
        } {
            assign26000_loop_guard += 1;
            assert!(assign26000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && (!s.b[1623])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1445, 965, (-1e-8), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {
            s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
            s.store_mul(1495, 1445, 1544);
            s.store_mul_neg_lhs(1496, 1449, 1542);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));
            s.store_mul_sqrt_ad_rhs(1473, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1628] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1628]) {
            s.store_scalar(1475, 0.0);
            s.store_scalar(1515, 0.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1628])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1475, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1515, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_scalar(1536, 0.0);
            s.store_sub(335, 1481, 1462);
        }

        s.b[1629] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
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

        s.b[1630] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });

        s.b[1631] = (2.0 == 1.0);
        s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && s.b[1631]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1632] = (2.0 == 2.0);
        s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && s.b[1632]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1633] = (2.0 == 4.0);
        s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && s.b[1633]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1634] = (2.0 == 8.0);
        s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26490_loop_guard: usize = 0;
        while {
            let assign26490_cond_e25084: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26490_cond_e25084 != 0.0
        } {
            assign26490_loop_guard += 1;
            assert!(assign26490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && (!s.b[1630])) {
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
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1629])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_sqrt_mul(1445, 1545, 336);
        }

        s.b[1635] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1636] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });

        s.b[1637] = (2.0 == 1.0);
        s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && s.b[1637]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1638] = (2.0 == 2.0);
        s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && s.b[1638]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1639] = (2.0 == 4.0);
        s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && s.b[1639]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1640] = (2.0 == 8.0);
        s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && (!s.b[1639])) && s.b[1640]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26850_loop_guard: usize = 0;
        while {
            let assign26850_cond_e25569: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26850_cond_e25569 != 0.0
        } {
            assign26850_loop_guard += 1;
            assert!(assign26850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && (!s.b[1636])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1445, 965, (-1e-8), 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));
            s.store_mul(1495, 1445, 1544);
            s.store_mul_neg_lhs(1496, 1449, 1542);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sub(335, 1481, 1462);
        }

        s.b[1641] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
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

        s.b[1642] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });

        s.b[1643] = (2.0 == 1.0);
        s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1644] = (2.0 == 2.0);
        s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && s.b[1644]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1645] = (2.0 == 4.0);
        s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && s.b[1645]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1646] = (2.0 == 8.0);
        s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27240_loop_guard: usize = 0;
        while {
            let assign27240_cond_e26026: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27240_cond_e26026 != 0.0
        } {
            assign27240_loop_guard += 1;
            assert!(assign27240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && (!s.b[1642])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1641])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sqrt_mul(1445, 1545, 336);
        }

        s.b[1647] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1648] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });

        s.b[1649] = (2.0 == 1.0);
        s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && s.b[1649]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1650] = (2.0 == 2.0);
        s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && s.b[1650]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1651] = (2.0 == 4.0);
        s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1652] = (2.0 == 8.0);
        s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27600_loop_guard: usize = 0;
        while {
            let assign27600_cond_e26415: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27600_cond_e26415 != 0.0
        } {
            assign27600_loop_guard += 1;
            assert!(assign27600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && (!s.b[1648])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1445, 965, (-1e-8), 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sub(335, 1481, 1459);
        }

        s.b[1653] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
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

        s.b[1654] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });

        s.b[1655] = (2.0 == 1.0);
        s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && s.b[1655]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1656] = (2.0 == 2.0);
        s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1657] = (2.0 == 4.0);
        s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && s.b[1657]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1658] = (2.0 == 8.0);
        s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27960_loop_guard: usize = 0;
        while {
            let assign27960_cond_e26803: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27960_cond_e26803 != 0.0
        } {
            assign27960_loop_guard += 1;
            assert!(assign27960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && (!s.b[1654])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1653])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sqrt_mul(1447, 1545, 336);
            s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1445, (-1.0), 1447, -1.0);
        }

        s.b[1659] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
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

        s.b[1660] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });

        s.b[1661] = (2.0 == 1.0);
        s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && s.b[1661]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1662] = (2.0 == 2.0);
        s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && s.b[1662]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1663] = (2.0 == 4.0);
        s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1664] = (2.0 == 8.0);
        s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28330_loop_guard: usize = 0;
        while {
            let assign28330_cond_e27202: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28330_cond_e27202 != 0.0
        } {
            assign28330_loop_guard += 1;
            assert!(assign28330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && (!s.b[1660])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);
            s.store_sub_from_scalar(1499, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1659])) {
            s.copy_ad(1499, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_neg_lhs(1494, 1499, 1544);
        }

        s.b[1665] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });

        s.b[1666] = ((s.v[1459] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));
        s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
            s.store_offset_sub(781, 1459, 1509, 0.8);
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

        s.b[1667] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });

        s.b[1668] = (2.0 == 1.0);
        s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && s.b[1668]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1669] = (2.0 == 2.0);
        s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && s.b[1669]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1670] = (2.0 == 4.0);
        s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && s.b[1670]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1671] = (2.0 == 8.0);
        s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && (!s.b[1670])) && s.b[1671]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28700_loop_guard: usize = 0;
        while {
            let assign28700_cond_e27640: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28700_cond_e27640 != 0.0
        } {
            assign28700_loop_guard += 1;
            assert!(assign28700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && (!s.b[1667])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1509, (-0.8), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && (!s.b[1666])) {
            s.copy_ad(336, 1459);
            s.store_scalar(335, 1.0);
        }

        s.b[1672] = ((s.v[1518] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));
        s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_offset_sub(781, 1518, 1509, 0.8);
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
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1673] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });

        s.b[1674] = (2.0 == 1.0);
        s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && s.b[1674]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1675] = (2.0 == 2.0);
        s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && s.b[1675]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1676] = (2.0 == 4.0);
        s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && s.b[1676]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1677] = (2.0 == 8.0);
        s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && (!s.b[1676])) && s.b[1677]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29050_loop_guard: usize = 0;
        while {
            let assign29050_cond_e28103: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29050_cond_e28103 != 0.0
        } {
            assign29050_loop_guard += 1;
            assert!(assign29050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && (!s.b[1673])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1509, (-0.8), 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && (!s.b[1672])) {
            s.copy_ad(336, 1518);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_ad_affine_product_lhs(1503, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1509)))), (-1.6021918e-19), 0.0, 1445);
        }

        s.b[1678] = (((s.v[1459] - s.v[1509]) < 0.06) && (0.06 >= 0.0));
        s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1459), s.ad_value(1509)));
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

        s.b[1679] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });

        s.b[1680] = (2.0 == 1.0);
        s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && s.b[1680]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1681] = (2.0 == 2.0);
        s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1682] = (2.0 == 4.0);
        s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && s.b[1682]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1683] = (2.0 == 8.0);
        s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && (!s.b[1682])) && s.b[1683]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29410_loop_guard: usize = 0;
        while {
            let assign29410_cond_e28535: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29410_cond_e28535 != 0.0
        } {
            assign29410_loop_guard += 1;
            assert!(assign29410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && (!s.b[1679])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1678])) {
            s.store_sub(336, 1459, 1509);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1513, 209, -1.0, 338);
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1538, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1684] = (s.v[790] > 1e-6);
        s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_div_square_rhs(336, 1500, 185);
            s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0, 2.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1685] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
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

        s.b[1686] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });

        s.b[1687] = (2.0 == 1.0);
        s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1688] = (2.0 == 2.0);
        s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && s.b[1688]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1689] = (2.0 == 4.0);
        s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && s.b[1689]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1690] = (2.0 == 8.0);
        s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29850_loop_guard: usize = 0;
        while {
            let assign29850_cond_e29061: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29850_cond_e29061 != 0.0
        } {
            assign29850_loop_guard += 1;
            assert!(assign29850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1685])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_offset_lhs(344, 85, 2.0, 338);
        }

        s.b[1691] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
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
