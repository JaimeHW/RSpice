#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1352] {
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

        s.b[1355] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1355]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1357] = (s.v[973] < 1000.0);
        s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1355]) && s.b[1357]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1352] && s.b[1355]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_ad_rhs(970, 970, A::powf(s.ad_value(676), p.p382));
        }

        s.b[1358] = (s.v[963] == 3.0);
        s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });

        if ((s.b[1352] && (!s.b[1355])) && s.b[1358]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1360] = (s.v[973] < 1000.0);
        s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });

        if (((s.b[1352] && (!s.b[1355])) && s.b[1358]) && s.b[1360]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1352] && (!s.b[1355])) && s.b[1358]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1352] && (!s.b[1355])) && (!s.b[1358])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1352] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1361] = (p.p39 != 2.0);
        s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1361]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1352] && (!s.b[1361])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1363] = (p.p39 != 2.0);
        s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1363]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1352] && (!s.b[1363])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1365] = (s.v[682] < 0.0);
        s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1365]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1367] = (s.v[688] < 0.0);
        s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1367]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1369] = (s.v[689] < 0.0);
        s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1369]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1352] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1371] = (s.v[766] < 0.0001);
        s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });

        if ((s.b[1352] && (p.p53 != 0.0)) && s.b[1371]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1352] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1352] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1352] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1352] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1352] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1372] = (s.v[963] == 0.0);
        s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1372]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1373] = (s.v[963] == 0.0);
        s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });

        s.b[1374] = (s.v[459] != 0.0);
        s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1373]) && s.b[1374]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1375] = (s.v[460] != 0.0);
        s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1373]) && s.b[1375]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1376] = (s.v[459] != 0.0);
        s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });

        if ((s.b[1352] && (!s.b[1373])) && s.b[1376]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1377] = (s.v[460] != 0.0);
        s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });

        if ((s.b[1352] && (!s.b[1373])) && s.b[1377]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1378] = (s.v[449] == 0.0);
        s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });

        s.b[1379] = (s.v[530] > 0.0);
        s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1378]) && s.b[1379]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1380] = (p.p39 == 1.0);
        s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1352] && s.b[1378]) && (!s.b[1379])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1381] = (s.v[540] > 0.0);
        s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1378]) && s.b[1381]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1382] = (p.p39 == 1.0);
        s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });

        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {
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
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1352] && s.b[1378]) && (!s.b[1381])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1383] = (s.v[538] > 0.0);
        s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1384] = (s.v[336] < 0.0);
        s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1384]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1385] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1386] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1352] && s.b[1378]) && (!s.b[1383])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1352] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1389] = (s.v[957] < 0.0);
        s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1389]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1390] = (s.v[957] > 0.005);
        s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1390]) {
            s.store_scalar(957, 0.005);
        }

        if (!s.b[1352]) {
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

        s.b[1391] = ((s.v[158] - p.p262) < s.v[69]);
        s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });

        if s.b[1391] {
            s.store_offset(69, 158, (-p.p262));
        }

        s.b[1392] = ((s.v[159] - p.p262) < s.v[69]);
        s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });

        if s.b[1392] {
            s.store_offset(69, 159, (-p.p262));
        }

        s.b[1393] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));
        s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });

        s.b[1394] = ((s.v[961] - p.p262) < s.v[69]);
        s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });

        if (s.b[1393] && s.b[1394]) {
            s.store_offset(69, 961, (-p.p262));
        }

        s.b[1395] = ((s.v[960] - p.p262) < s.v[69]);
        s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });

        if (s.b[1393] && s.b[1395]) {
            s.store_offset(69, 960, (-p.p262));
        }

        s.b[1396] = (s.v[70] > (s.v[69] * 0.5));
        s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });

        if s.b[1396] {
            s.store_scale(70, 69, 0.5);
        }

        s.b[1397] = param_given[338];
        s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });

        if s.b[1397] {
            s.store_scalar(72, p.p338);
        }

        if (!s.b[1397]) {
            s.copy_ad(72, 69);
        }

        s.b[1398] = param_given[339];
        s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });

        if s.b[1398] {
            s.store_scalar(73, p.p339);
        }

        s.b[1399] = param_given[338];
        s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });

        if ((!s.b[1398]) && s.b[1399]) {
            s.store_scale(73, 72, 0.5);
        }

        if ((!s.b[1398]) && (!s.b[1399])) {
            s.copy_ad(73, 70);
        }

        s.b[1400] = (s.v[73] > (s.v[72] * 0.5));
        s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });

        if s.b[1400] {
            s.store_scale(73, 72, 0.5);
        }

        s.b[1401] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));
        s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });

        s.b[1402] = (s.v[448] == 1.0);
        s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1402]) {
            s.store_scalar(74, 1.0);
        }

        s.b[1403] = (s.v[448] == 2.0);
        s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1403]) {
            s.store_scalar(74, 2.0);
        }

        s.b[1404] = (s.v[448] == 3.0);
        s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1404]) {
            s.store_scalar(74, 3.0);
        }

        s.store_scalar(77, 0.0);

        s.b[1405] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });

        if s.b[1405] {
            s.copy_ad(373, 733);
        }

        s.b[1406] = (s.v[373] >= 0.0);
        s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });

        if (s.b[1405] && s.b[1406]) {
            s.copy_ad(376, 373);
            s.copy_ad(383, 798);
        }

        if (s.b[1405] && (!s.b[1406])) {
            s.store_neg(376, 373);
            s.store_sub(383, 798, 373);
        }

        if s.b[1405] {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1407] = (s.v[108] < 1e-12);
        s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });

        if (s.b[1405] && s.b[1407]) {
            s.store_scalar(108, 1e-12);
        }

        if s.b[1405] {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1408] = (s.v[335] < 0.0);
        s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });

        if (s.b[1405] && s.b[1408]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if s.b[1405] {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if s.b[1405] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1405] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
        }

        if (!s.b[1405]) {
            s.store_scalar(384, 0.0);
        }

        s.b[1409] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));
        s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });

        if s.b[1409] {
            s.copy_ad(373, 733);
            s.copy_ad(374, 734);
            s.copy_ad(372, 735);
        }

        s.b[1410] = (s.v[373] >= 0.0);
        s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });

        if (s.b[1409] && s.b[1410]) {
            s.store_scalar(370, 1.0);
            s.store_scalar(371, 0.0);
            s.copy_ad(376, 373);
            s.copy_ad(377, 374);
            s.copy_ad(375, 372);
            s.copy_ad(383, 798);
        }

        if (s.b[1409] && (!s.b[1410])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 1.0);
            s.store_neg(376, 373);
            s.store_sub(377, 374, 373);
            s.store_sub(375, 372, 373);
            s.store_sub(383, 798, 373);
        }

        s.b[1411] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));
        s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });

        if (s.b[1409] && s.b[1411]) {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1412] = (s.v[108] < 1e-12);
        s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });

        if ((s.b[1409] && s.b[1411]) && s.b[1412]) {
            s.store_scalar(108, 1e-12);
        }

        if (s.b[1409] && s.b[1411]) {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_add(381, 377, 108);
            s.store_add(382, 375, 108);
        }

        s.b[1413] = ((p.p34 == 1.0) || (s.v[370] == 1.0));
        s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);
            s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);
            s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(344, 782, p.p292, 0.5);
        }

        s.b[1414] = (s.v[344] < 0.0);
        s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });

        if (((s.b[1409] && s.b[1411]) && s.b[1413]) && s.b[1414]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1415] = (s.v[337] < 0.0);
        s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });

        if (((s.b[1409] && s.b[1411]) && s.b[1413]) && s.b[1415]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 366);
            s.store_mul(366, 366, 337);
        }

        if ((s.b[1409] && s.b[1411]) && (!s.b[1413])) {
            s.copy_ad(366, 691);
        }

        if (s.b[1409] && s.b[1411]) {
            s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);
        }

        s.b[1416] = ((p.p34 == 1.0) || (s.v[371] == 1.0));
        s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);
            s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));
            s.store_scalar(344, (p.p292 + 1e-25));
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1417] = (s.v[337] < 0.0);
        s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });

        if (((s.b[1409] && s.b[1411]) && s.b[1416]) && s.b[1417]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 367);
            s.store_mul(367, 367, 337);
        }

        if ((s.b[1409] && s.b[1411]) && (!s.b[1416])) {
            s.copy_ad(367, 691);
        }

        s.b[1418] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });

        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1419] = (s.v[335] < 0.0);
        s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });

        if (((s.b[1409] && s.b[1411]) && s.b[1418]) && s.b[1419]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
            s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));
            s.store_mul(334, 366, 340);
            s.store_mul(335, 367, 340);
            s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);
            s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);
        }

        if ((s.b[1409] && s.b[1411]) && (!s.b[1418])) {
            s.store_scalar(384, 0.0);
        }

        if (s.b[1409] && s.b[1411]) {
            s.copy_ad(4, 366);
            s.copy_ad(5, 367);
        }

        if (s.b[1409] && (!s.b[1411])) {
            s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);
        }

        if s.b[1409] {
            s.store_scale(4, 4, 1.0 / (s.v[164]));
            s.store_scale(5, 5, 1.0 / (s.v[164]));
            s.store_add_scaled_value_products(4, s.ad_value(4), 1.0, s.ad_value(370), s.ad_value(644), 1.0, s.ad_value(371), s.ad_value(648), 1.0);
            s.store_add_scaled_value_products(5, s.ad_value(5), 1.0, s.ad_value(370), s.ad_value(648), 1.0, s.ad_value(371), s.ad_value(644), 1.0);
            s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);
            s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);
        }

        s.b[1422] = (s.v[792] > s.v[70]);
        s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });

        if s.b[1422] {
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

        if (!s.b[1422]) {
            s.copy_ad(83, 792);
            s.store_scalar(84, 1.0);
        }

        s.store_scaled_mul(335, 84, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(108, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1423] = (s.v[108] < 1e-12);
        s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });

        if s.b[1423] {
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

        s.b[1424] = (s.v[338] < 0.0);
        s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });

        if s.b[1424] {
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

        s.b[1425] = (s.v[765] == 0.0);
        s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });

        if s.b[1425] {
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1425]) {
            s.store_scalar(80, 1.0);
        }

        s.copy_ad(335, 637);

        s.store_sqrt_mul(336, 335, 158);

        s.store_add_scaled_ad_lhs(190, A::offset(s.ad_value(158), s.v[160]), 336, s.v[189]);

        s.b[1426] = (s.v[80] == 0.0);
        s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });

        if s.b[1426] {
            s.store_scalar(183, s.v[187]);
            s.store_scalar(185, s.v[188]);
            s.store_scalar(186, s.v[189]);
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        if (!s.b[1426]) {
            s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);
            s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));
            s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);
            s.store_scaled_add(336, 339, 782, 0.5);
        }

        s.b[1427] = (s.v[336] < 0.0);
        s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });

        if ((!s.b[1426]) && s.b[1427]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[1426]) {
            s.store_offset(336, 336, 1e-25);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1426]) {
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));
            s.store_scaled_abs(338, 190, 2.0);
            s.store_offset_sub(340, 339, 791, s.v[160]);
        }

        s.b[1428] = (s.v[340] > s.v[338]);
        s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });

        if ((!s.b[1426]) && s.b[1428]) {
            s.copy_ad(338, 340);
        }

        if (!s.b[1426]) {
            s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));
            s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));
        }

        if (!s.b[1426]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (!s.b[1426]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));
            s.store_offset_scaled(184, 336, p.p235, p.p237);
            s.store_scalar(341, p.p235);
        }

        s.b[1429] = ((s.v[184] * 1000000000000.0) < s.v[187]);
        s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });

        if ((!s.b[1426]) && s.b[1429]) {
            s.store_scalar(184, 0.0);
            s.store_scalar(80, 0.0);
        }

        if (!s.b[1426]) {
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

        s.b[1430] = (p.p140 != 0.0);
        s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });

        if s.b[1430] {
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

        if s.b[1430] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1430] {
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

        if (!s.b[1430]) {
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

        s.b[1431] = (s.v[615] == 1.0);
        s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });

        if s.b[1431] {
            s.copy_ad(341, 107);
            s.copy_ad(334, 642);
            s.store_offset(337, 341, (-p.p152));
        }

        s.b[1432] = (s.v[337] < (-3.0));
        s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1432]) {
            s.store_scalar(340, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[1433] = (s.v[337] < 0.0);
        s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });

        if ((s.b[1431] && (!s.b[1432])) && s.b[1433]) {
            s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if ((s.b[1431] && (!s.b[1432])) && (!s.b[1433])) {
            s.store_offset_mul_offset_rhs_ad_rhs(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if s.b[1431] {
            s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));
            s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);
        }

        s.b[1434] = (s.v[182] < 0.0);
        s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1434]) {
            s.store_scalar(182, 0.0);
            s.store_scalar(340, 0.0);
        }

        if s.b[1431] {
            s.store_mul(182, 182, 334);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1431] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1431] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);
        }

        s.b[1441] = (s.v[792] > s.v[73]);
        s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });

        if ((p.p37 != 0.0) && s.b[1441]) {
            s.store_sub(335, 792, 73);
            s.store_sub(336, 72, 73);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(1436, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1436, 1.0);
            s.store_neg(1436, 1436);
            s.store_add(1435, 73, 333);
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
        if ((p.p37 != 0.0) && s.b[1441]) {
            s.store_div_scaled_inputs_product(1436, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

        if ((p.p37 != 0.0) && (!s.b[1441])) {
            s.copy_ad(1435, 792);
            s.store_scalar(1436, 1.0);
        }

        if (p.p37 == 0.0) {
            s.copy_ad(1435, 792);
            s.store_scalar(1436, 1.0);
        }

        s.store_scaled_mul(335, 1436, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(1437, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1442] = (s.v[1437] < 1e-12);
        s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });

        if s.b[1442] {
            s.store_scalar(1437, 1e-12);
        }

        s.store_add(1438, 1435, 1437);

        s.store_add_scaled_inputs(1439, 790, 1.0, 1437, 2.0);

        s.store_add(1440, 791, 1437);

        s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));

        s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1435, 1.0, s.v[160]);

        s.b[1443] = (s.v[963] != 0.0);
        s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });

        s.b[1444] = (p.p42 == 1.0);
        s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });

        s.b[1445] = (p.p42 == 2.0);
        s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });

        s.b[1446] = (p.p42 == 3.0);
        s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });

        if (s.b[1443] && s.b[1444]) {
            s.copy_ad(1463, 960);
            s.store_scale(1546, 964, 1.6021918e-19);
            s.store_square(1545, 964);
            s.store_scale(1502, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1544, 622, 1.6021918e-19);
            s.store_scalar(1541, (1.6021918e-19 * 1.6021918e-19));
            s.store_scalar(1542, (1.034943e-10 * 1.034943e-10));
            s.store_square(1543, 965);
            s.store_div_from_scalar(1547, (2.0 * 1.034943e-10), 1546);
            s.store_scale(1548, 1546, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_scale(1549, 1546, (2.0 * 1.034943e-10));
            s.store_div_from_scalar(1550, (2.0 * 1.034943e-10), 1544);
            s.store_scale(1551, 1544, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_div(1536, 964, 622);
            s.store_div_from_scalar_offset_input(1535, 1.0, 1536, 1.0);
            s.store_scalar(1552, (1e-12 * 1000.0));
            s.store_scalar(1553, (1e-10 * 1000.0));
            s.store_scalar(1461, 0.0);
            s.store_scalar(1462, 0.0);
            s.store_scalar(1475, 0.0);
            s.store_scalar(1476, 0.0);
            s.store_scalar(1517, 0.0);
            s.store_scalar(1518, 0.0);
            s.store_scalar(1497, 0.0);
            s.store_scalar(1499, 0.0);
            s.store_scalar(1498, 0.0);
            s.store_scalar(1500, 0.0);
            s.store_scalar(1520, 0.0);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));
            s.store_div_scaled_product_by_product(1456, s.ad_value(185), s.ad_value(185), 1.0, s.ad_value(209), s.ad_value(209), 1.0);
            s.store_mul_ad_lhs(1459, A::div_scaled_value_by_product(s.ad_value(1456), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1545);
            s.store_sqrt_mul_ad(1453, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1463), s.ad_value(1435)));
        }

        s.b[1559] = (s.v[1453] > s.v[965]);
        s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
            s.store_scalar(1466, 0.0);
            s.copy_ad(1447, 965);
            s.store_scalar(1483, 0.0);
            s.store_sub_ad_rhs(1464, 1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)));
            s.store_scalar(1511, 0.0);
            s.copy_ad(1510, 1466);
            s.copy_ad(1472, 1464);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        let mut assign24640_loop_guard: usize = 0;
        while {
            let assign24640_cond_e19022: f64 = (150.0 + 1.0);
            let assign24640_cond_e19024: f64 = if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (s.v[97] <= assign24640_cond_e19022)) { 1.0 } else { 0.0 };
            assign24640_cond_e19024 != 0.0
        } {
            assign24640_loop_guard += 1;
            assert!(assign24640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
                s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
            }
            s.b[1560] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
                s.store_offset_sub(781, 1447, 965, 1e-8);
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
            s.b[1561] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            s.b[1562] = (2.0 == 1.0);
            s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && s.b[1562]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1563] = (2.0 == 2.0);
            s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && s.b[1563]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1564] = (2.0 == 4.0);
            s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && s.b[1564]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1565] = (2.0 == 8.0);
            s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && (!s.b[1564])) && s.b[1565]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24640_body27_loop_guard: usize = 0;
            while {
                let assign24640_body27_cond_e19340: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24640_body27_cond_e19340 != 0.0
            } {
                assign24640_body27_loop_guard += 1;
                assert!(assign24640_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && (!s.b[1561])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
                s.store_add_scaled_inputs3_indices(335, 1464, 1.0, 1435, (-1.0), 1463, 1.0);
            }
            s.b[1566] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
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
            s.b[1567] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            s.b[1568] = (2.0 == 1.0);
            s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && s.b[1568]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1569] = (2.0 == 2.0);
            s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && s.b[1569]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1570] = (2.0 == 4.0);
            s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && s.b[1570]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1571] = (2.0 == 8.0);
            s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && (!s.b[1570])) && s.b[1571]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign24640_body63_loop_guard: usize = 0;
            while {
                let assign24640_body63_cond_e19794: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24640_body63_cond_e19794 != 0.0
            } {
                assign24640_body63_loop_guard += 1;
                assert!(assign24640_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && (!s.b[1567])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1566])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
                s.store_sqrt_mul(1451, 1550, 336);
                s.store_mul(1497, 1447, 1546);
                s.store_mul_div_from_scalar_lhs(1529, (-1.034943e-10), 1447, 334);
                s.store_mul_neg_lhs(1498, 1451, 1544);
                s.store_mul_div_from_scalar_lhs(1531, (-1.034943e-10), 1451, 341);
                s.store_add_ad_lhs(1485, A::add_scaled_product(s.ad_value(1497), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1483)), 1.0), 1498);
                s.copy_ad(1487, 185);
                s.store_add(1488, 1529, 1531);
                s.store_add_scaled_product_right_ad(1486, 1464, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463)), (-1.0));
                s.store_scalar(1489, 0.0);
                s.store_scalar(1490, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1572] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);
            s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {
                s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {
                s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1572])) {
                s.store_sub_ad_rhs(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1464, 1464, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1573] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1464] - s.v[1472])) as f64).abs() <= 1e-12));
            s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1573]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
                s.copy_ad(1510, 1466);
                s.copy_ad(1472, 1464);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
            s.copy_ad(1513, 1464);
            s.store_mul(1451, 965, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_scaled_product_indices(1483, 1464, 1.0, 1548, 1543, 1.0);
            s.copy_ad(1461, 1483);
            s.copy_ad(1467, 1483);
            s.copy_ad(1509, 1483);
        }

        s.b[1574] = (s.v[85] > s.v[1466]);
        s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1574]) {
            s.store_scalar(1479, 1.0);
        }

        s.b[1575] = (s.v[85] > s.v[1509]);
        s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && s.b[1575]) {
            s.store_scalar(1479, 3.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && (!s.b[1575])) {
            s.store_scalar(1479, 2.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1559])) {
            s.store_scalar(1466, 0.0);
            s.copy_ad(1509, 1466);
            s.store_scalar(1467, 0.0);
            s.copy_ad(1511, 1466);
            s.copy_ad(1447, 1453);
            s.store_mul(1451, 1447, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_ad_lhs(1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)), 1464);
            s.copy_ad(1513, 1464);
        }

        s.b[1576] = (s.v[85] > s.v[1466]);
        s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && s.b[1576]) {
            s.store_scalar(1479, 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && (!s.b[1576])) {
            s.store_scalar(1479, 2.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1549, s.ad_value(1467), 1.0, s.ad_value(1435), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1577] = (s.v[335] > 0.0);
        s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1577]) {
            s.store_add_scaled_inputs3_mixed_iia(1455, 1435, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1577])) {
            s.store_sub(1455, 1435, 961);
        }

        s.b[1578] = (s.v[85] > s.v[1466]);
        s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1578]) {
            s.copy_ad(1464, 1513);
            s.store_scalar(1483, 0.0);
            s.store_add_div_lhs(1480, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1483);
        }

        s.b[1579] = (s.v[1480] < (s.v[1511] + s.v[1553]));
        s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1578]) && s.b[1579]) {
            s.store_add(1480, 1511, 1553);
        }

        s.b[1580] = (s.v[85] > s.v[1509]);
        s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1578])) && s.b[1580]) {
            s.copy_ad(1480, 1461);
        }

        s.b[1581] = (s.v[85] > s.v[1455]);
        s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {
            s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));
            s.copy_ad(1470, 1483);
            s.store_div_scaled_inputs2_mixed_aii(1480, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);
        }

        s.b[1582] = (s.v[1480] > (s.v[1467] - s.v[1552]));
        s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1582]) {
            s.store_sub(1480, 1467, 1552);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {
            s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);
            s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
        }

        s.b[1583] = ((s.v[1449] + s.v[1447]) > s.v[965]);
        s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25120_loop_guard: usize = 0;
        while {
            let assign25120_cond_e20806: f64 = (150.0 + 1.0);
            let assign25120_cond_e20808: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (s.v[97] <= assign25120_cond_e20806)) { 1.0 } else { 0.0 };
            assign25120_cond_e20808 != 0.0
        } {
            assign25120_loop_guard += 1;
            assert!(assign25120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
                s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);
                s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));
            }
            s.b[1584] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);
            s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1584]) {
                s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1584])) {
                s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);
            }
            s.b[1585] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));
            s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1585]) {
                s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
                s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));
            }
            s.b[1586] = (s.v[335] > 0.0);
            s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1586]) {
                s.store_div_scaled_inputs2_mixed_aii(1480, A::sqrt(s.ad_value(335)), 0.5, 1457, (-0.5), 1456, 1.0);
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1586])) {
                s.store_div_scaled_inputs_indices(1480, 1457, (-0.5), 1456, 1.0);
            }
            s.b[1587] = (s.v[1480] > s.v[1467]);
            s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1587]) {
                s.copy_ad(1480, 1467);
            }
            s.b[1588] = (s.v[1480] > s.v[1483]);
            s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1588]) {
                s.store_sub(1480, 1483, 1553);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
                s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);
                s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
            }
            s.b[1589] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-8);
            s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1589]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
                s.copy_ad(1470, 1483);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) {
            s.store_div_ad_rhs(1460, 1459, A::exp(A::mul(s.ad_value(154), s.ad_value(1435))));
            s.copy_ad(1470, 1483);
            s.store_div_ad(1480, A::ln(A::mul3(s.ad_value(1460), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));
            s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);
            s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
        }

        s.b[1590] = ((s.v[1449] + s.v[1447]) > s.v[965]);
        s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign25200_loop_guard: usize = 0;
        while {
            let assign25200_cond_e21389: f64 = (s.v[421] + 1.0);
            let assign25200_cond_e21391: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (s.v[97] <= assign25200_cond_e21389)) { 1.0 } else { 0.0 };
            assign25200_cond_e21391 != 0.0
        } {
            assign25200_loop_guard += 1;
            assert!(assign25200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
                s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);
                s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));
            }
            s.b[1591] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);
            s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1591]) {
                s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (!s.b[1591])) {
                s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);
            }
            s.b[1592] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));
            s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1592]) {
                s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
                s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);
                s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
            }
            s.b[1593] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-5);
            s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1593]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
                s.copy_ad(1470, 1483);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (s.b[1443] && s.b[1444]) {
            s.copy_ad(1482, 1483);
            s.store_scalar(1519, 0.12);
            s.store_scalar(79, 0.0);
            s.copy_ad(1461, 1480);
            s.copy_ad(1483, 1482);
            s.copy_ad(1469, 1461);
            s.copy_ad(1470, 1483);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        let mut assign25290_loop_guard: usize = 0;
        while {
            let assign25290_cond_e21731: f64 = (150.0 + 1.0);
            let assign25290_cond_e21733: f64 = if ((s.b[1443] && s.b[1444]) && (s.v[97] <= assign25290_cond_e21731)) { 1.0 } else { 0.0 };
            assign25290_cond_e21733 != 0.0
        } {
            assign25290_loop_guard += 1;
            assert!(assign25290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && s.b[1444]) {
                s.store_mul_sub_ad_rhs(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463));
                s.store_mul(1533, 1535, 1536);
                s.store_sub(335, 1483, 1464);
            }
            s.b[1594] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
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
            s.b[1595] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
            s.b[1596] = (2.0 == 1.0);
            s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1597] = (2.0 == 2.0);
            s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1598] = (2.0 == 4.0);
            s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1599] = (2.0 == 8.0);
            s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body29_loop_guard: usize = 0;
            while {
                let assign25290_body29_cond_e22024: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body29_cond_e22024 != 0.0
            } {
                assign25290_body29_loop_guard += 1;
                assert!(assign25290_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && (!s.b[1595])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1594])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sqrt_mul(1447, 1547, 336);
            }
            s.b[1600] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
                s.store_offset_sub(781, 1447, 965, 1e-8);
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
            s.b[1601] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
            s.b[1602] = (2.0 == 1.0);
            s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1603] = (2.0 == 2.0);
            s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1604] = (2.0 == 4.0);
            s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1605] = (2.0 == 8.0);
            s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body65_loop_guard: usize = 0;
            while {
                let assign25290_body65_cond_e22413: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body65_cond_e22413 != 0.0
            } {
                assign25290_body65_loop_guard += 1;
                assert!(assign25290_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && (!s.b[1601])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
                s.store_mul(1497, 1447, 1546);
                s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar(1.034943e-10, s.ad_value(1447)), 334, 337);
                s.store_mul_ad_product_lhs_mixed_ai(1529, A::div_from_scalar((-1.034943e-10), s.ad_value(1447)), 334, 337);
                s.store_mul_neg_lhs(1498, 1451, 1544);
                s.store_div_from_scalar(1531, (-1.034943e-10), 1451);
                s.store_scaled_mul(335, 1502, 1543, 8.0);
                s.store_div_scaled_inputs_product(1520, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1464), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1542), s.ad_value(1461), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1461), s.ad_value(1461), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), s.ad_value(1543), 1.0, s.ad_value(335), 1.0);
                s.store_div_ad_lhs(1521, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1461), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_div_ad_lhs(1522, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1461), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1461, 1483);
                s.store_exp(336, 335);
            }
            s.b[1606] = (s.v[1461] >= s.v[1483]);
            s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1606]) {
                s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(1523, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 1475, 1.0);
                s.store_neg(1525, 1523);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1606])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));
                s.store_mul_sqrt_ad_rhs(1475, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1475, 1.0);
                s.store_mul_add_ad_rhs(1523, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1525, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1607] = ((s.v[1520] > (s.v[1511] - s.v[1519])) && (s.v[1519] >= 0.0));
            s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1511, (-1.0), 1519, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1519);
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
            s.b[1608] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
            s.b[1609] = (4.0 == 1.0);
            s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1610] = (4.0 == 2.0);
            s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1611] = (4.0 == 4.0);
            s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1612] = (4.0 == 8.0);
            s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body126_loop_guard: usize = 0;
            while {
                let assign25290_body126_cond_e23194: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body126_cond_e23194 != 0.0
            } {
                assign25290_body126_loop_guard += 1;
                assert!(assign25290_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && (!s.b[1608])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1519, 726);
                s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(335, 1511, 1.0, 1519, (-1.0), 780, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1607])) {
                s.copy_ad(335, 1520);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sub(1485, 1483, 335);
                s.store_mul_neg_lhs(1487, 1521, 334);
                s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1533), s.ad_value(334)));
                s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1475), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1461)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);
                s.store_sub(1489, 1523, 185);
                s.store_add_scaled_inputs_products_indices(1490, 1525, 1.0, 1527, 1.0, 1529, 1533, 1.0, 1531, 1533, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1613] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);
            s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {
                s.store_offset(1461, 1461, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {
                s.store_offset(1483, 1483, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1613])) {
                s.store_sub_ad_rhs(1461, 1461, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1483, 1483, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1614] = (((((s.v[1461] - s.v[1469])) as f64).abs() <= 1e-12) && ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-12));
            s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.copy_ad(1469, 1461);
                s.copy_ad(1470, 1483);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));
        s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_offset_sub(781, 1483, 1461, 0.02);
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

        s.b[1618] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });

        s.b[1619] = (2.0 == 1.0);
        s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1620] = (2.0 == 2.0);
        s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1621] = (2.0 == 4.0);
        s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1622] = (2.0 == 8.0);
        s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25580_loop_guard: usize = 0;
        while {
            let assign25580_cond_e23885: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25580_cond_e23885 != 0.0
        } {
            assign25580_loop_guard += 1;
            assert!(assign25580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_offset_lhs(1483, 1461, (-0.02), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_sub_ad_rhs(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463));
            s.store_mul_sub_rhs(335, 154, 1461, 1483);
            s.store_exp(336, 335);
        }

        s.b[1623] = (s.v[1461] >= s.v[1483]);
        s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {
            s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
            s.copy_ad(1538, 1475);
            s.store_scalar(1517, 0.0);
            s.store_scalar(1477, 0.0);
            s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);
        }

        s.b[1624] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1625] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });

        s.b[1626] = (2.0 == 1.0);
        s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1627] = (2.0 == 2.0);
        s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1628] = (2.0 == 4.0);
        s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1629] = (2.0 == 8.0);
        s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26020_loop_guard: usize = 0;
        while {
            let assign26020_cond_e24416: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26020_cond_e24416 != 0.0
        } {
            assign26020_loop_guard += 1;
            assert!(assign26020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {
            s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1497, 1447, 1546);
            s.store_mul_neg_lhs(1498, 1451, 1544);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));
            s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));
            s.store_mul_sqrt_ad_rhs(1475, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1630] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1630]) {
            s.store_scalar(1477, 0.0);
            s.store_scalar(1517, 0.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1477, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1517, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_scalar(1538, 0.0);
            s.store_sub(335, 1483, 1464);
        }

        s.b[1631] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
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

        s.b[1632] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });

        s.b[1633] = (2.0 == 1.0);
        s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1634] = (2.0 == 2.0);
        s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1635] = (2.0 == 4.0);
        s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1636] = (2.0 == 8.0);
        s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26510_loop_guard: usize = 0;
        while {
            let assign26510_cond_e25091: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26510_cond_e25091 != 0.0
        } {
            assign26510_loop_guard += 1;
            assert!(assign26510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && (!s.b[1632])) {
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1631])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_sqrt_mul(1447, 1547, 336);
        }

        s.b[1637] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1638] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });

        s.b[1639] = (2.0 == 1.0);
        s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1640] = (2.0 == 2.0);
        s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1641] = (2.0 == 4.0);
        s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1642] = (2.0 == 8.0);
        s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26870_loop_guard: usize = 0;
        while {
            let assign26870_cond_e25576: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26870_cond_e25576 != 0.0
        } {
            assign26870_loop_guard += 1;
            assert!(assign26870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && (!s.b[1638])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1497, 1447, 1546);
            s.store_mul_neg_lhs(1498, 1451, 1544);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sub(335, 1483, 1464);
        }

        s.b[1643] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
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

        s.b[1644] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });

        s.b[1645] = (2.0 == 1.0);
        s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1646] = (2.0 == 2.0);
        s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1647] = (2.0 == 4.0);
        s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1648] = (2.0 == 8.0);
        s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27260_loop_guard: usize = 0;
        while {
            let assign27260_cond_e26033: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27260_cond_e26033 != 0.0
        } {
            assign27260_loop_guard += 1;
            assert!(assign27260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && (!s.b[1644])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1643])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sqrt_mul(1447, 1547, 336);
        }

        s.b[1649] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1650] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });

        s.b[1651] = (2.0 == 1.0);
        s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1652] = (2.0 == 2.0);
        s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1653] = (2.0 == 4.0);
        s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1654] = (2.0 == 8.0);
        s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27620_loop_guard: usize = 0;
        while {
            let assign27620_cond_e26422: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27620_cond_e26422 != 0.0
        } {
            assign27620_loop_guard += 1;
            assert!(assign27620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && (!s.b[1650])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_offset_lhs(1447, 965, (-1e-8), 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sub(335, 1483, 1461);
        }

        s.b[1655] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
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

        s.b[1656] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });

        s.b[1657] = (2.0 == 1.0);
        s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1658] = (2.0 == 2.0);
        s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1659] = (2.0 == 4.0);
        s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1660] = (2.0 == 8.0);
        s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27980_loop_guard: usize = 0;
        while {
            let assign27980_cond_e26810: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27980_cond_e26810 != 0.0
        } {
            assign27980_loop_guard += 1;
            assert!(assign27980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && (!s.b[1656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1655])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sqrt_mul(1449, 1547, 336);
            s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1447, (-1.0), 1449, -1.0);
        }

        s.b[1661] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
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

        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });

        s.b[1663] = (2.0 == 1.0);
        s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1664] = (2.0 == 2.0);
        s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1665] = (2.0 == 4.0);
        s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1666] = (2.0 == 8.0);
        s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28350_loop_guard: usize = 0;
        while {
            let assign28350_cond_e27209: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28350_cond_e27209 != 0.0
        } {
            assign28350_loop_guard += 1;
            assert!(assign28350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && (!s.b[1662])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);
            s.store_sub_from_scalar(1501, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1661])) {
            s.copy_ad(1501, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_neg_lhs(1496, 1501, 1546);
        }

        s.b[1667] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });

        s.b[1668] = ((s.v[1461] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));
        s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
            s.store_offset_sub(781, 1461, 1511, 0.8);
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

        s.b[1669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });

        s.b[1670] = (2.0 == 1.0);
        s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1671] = (2.0 == 2.0);
        s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1672] = (2.0 == 4.0);
        s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1673] = (2.0 == 8.0);
        s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28720_loop_guard: usize = 0;
        while {
            let assign28720_cond_e27647: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28720_cond_e27647 != 0.0
        } {
            assign28720_loop_guard += 1;
            assert!(assign28720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && (!s.b[1669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1511, (-0.8), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && (!s.b[1668])) {
            s.copy_ad(336, 1461);
            s.store_scalar(335, 1.0);
        }

        s.b[1674] = ((s.v[1520] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));
        s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_offset_sub(781, 1520, 1511, 0.8);
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });

        s.b[1676] = (2.0 == 1.0);
        s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1677] = (2.0 == 2.0);
        s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1678] = (2.0 == 4.0);
        s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1679] = (2.0 == 8.0);
        s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29070_loop_guard: usize = 0;
        while {
            let assign29070_cond_e28110: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29070_cond_e28110 != 0.0
        } {
            assign29070_loop_guard += 1;
            assert!(assign29070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && (!s.b[1675])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_offset_lhs(336, 1511, (-0.8), 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && (!s.b[1674])) {
            s.copy_ad(336, 1520);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_ad_affine_product_lhs(1505, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1511)))), (-1.6021918e-19), 0.0, 1447);
        }

        s.b[1680] = (((s.v[1461] - s.v[1511]) < 0.06) && (0.06 >= 0.0));
        s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1461), s.ad_value(1511)));
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

        s.b[1681] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });

        s.b[1682] = (2.0 == 1.0);
        s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && s.b[1682]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1683] = (2.0 == 2.0);
        s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && s.b[1683]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1684] = (2.0 == 4.0);
        s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && s.b[1684]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1685] = (2.0 == 8.0);
        s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && (!s.b[1684])) && s.b[1685]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29430_loop_guard: usize = 0;
        while {
            let assign29430_cond_e28542: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29430_cond_e28542 != 0.0
        } {
            assign29430_loop_guard += 1;
            assert!(assign29430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && (!s.b[1681])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1680])) {
            s.store_sub(336, 1461, 1511);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_sqrt_rhs(1515, 209, -1.0, 338);
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1540, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1686] = (s.v[790] > 1e-6);
        s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_div_square_rhs(336, 1502, 185);
            s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1438, -1.0, 2.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1687] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
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

        s.b[1688] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });

        s.b[1689] = (2.0 == 1.0);
        s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1690] = (2.0 == 2.0);
        s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && s.b[1690]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1691] = (2.0 == 4.0);
        s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && s.b[1691]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1692] = (2.0 == 8.0);
        s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && (!s.b[1691])) && s.b[1692]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29870_loop_guard: usize = 0;
        while {
            let assign29870_cond_e29068: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29870_cond_e29068 != 0.0
        } {
            assign29870_loop_guard += 1;
            assert!(assign29870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1687])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_offset_lhs(344, 85, 2.0, 338);
        }

        s.b[1693] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
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

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1694] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });

        s.b[1695] = (4.0 == 1.0);
        s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && s.b[1695]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1696] = (4.0 == 2.0);
        s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1697] = (4.0 == 4.0);
        s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && s.b[1697]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1698] = (4.0 == 8.0);
        s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && (!s.b[1697])) && s.b[1698]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30300_loop_guard: usize = 0;
        while {
            let assign30300_cond_e29601: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30300_cond_e29601 != 0.0
        } {
            assign30300_loop_guard += 1;
            assert!(assign30300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && (!s.b[1694])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_from_scalar(344, (0.3 + 0.2), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
        }

        s.b[1699] = ((s.v[85] < 0.5) && (0.5 >= 0.0));
        s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
            s.store_sub_from_scalar(781, 0.5, 85);
            s.store_square(722, 781);
            s.store_scalar(723, (0.5 * 0.5));
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

        s.b[1700] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });

        s.b[1701] = (2.0 == 1.0);
        s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && s.b[1701]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1702] = (2.0 == 2.0);
        s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && s.b[1702]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1703] = (2.0 == 4.0);
        s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1704] = (2.0 == 8.0);
        s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) && s.b[1704]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30730_loop_guard: usize = 0;
        while {
            let assign30730_cond_e30141: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30730_cond_e30141 != 0.0
        } {
            assign30730_loop_guard += 1;
            assert!(assign30730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && (!s.b[1700])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);
            s.store_sub_from_scalar(1537, 0.5, 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1699])) {
            s.copy_ad(1537, 85);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_scale(335, 1537, 0.8);
        }

        s.b[1705] = ((s.v[348] > (s.v[1537] - s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
            s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1537, (-1.0), 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
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

        s.b[1706] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });

        s.b[1707] = (2.0 == 1.0);
        s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && s.b[1707]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1708] = (2.0 == 2.0);
        s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && s.b[1708]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1709] = (2.0 == 4.0);
        s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1709]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1710] = (2.0 == 8.0);
        s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1709])) && s.b[1710]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign31090_loop_guard: usize = 0;
        while {
            let assign31090_cond_e30593: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31090_cond_e30593 != 0.0
        } {
            assign31090_loop_guard += 1;
            assert!(assign31090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && (!s.b[1706])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(790, 1537, 1.0, 335, (-1.0), 780, 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1705])) {
            s.copy_ad(790, 348);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1686])) {
            s.copy_ad(348, 790);
        }

        s.b[1711] = (s.v[790] <= 0.0);
        s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });

        if ((s.b[1443] && s.b[1444]) && s.b[1711]) {
            s.copy_ad(1462, 1461);
            s.copy_ad(1484, 1483);
            s.copy_ad(1465, 1464);
            s.copy_ad(1478, 1477);
            s.copy_ad(1539, 1538);
            s.copy_ad(1499, 1497);
            s.copy_ad(1500, 1498);
            s.copy_ad(1518, 1517);
            s.copy_ad(1516, 1515);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_sqrt_mul_ad(1454, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
        }

        s.b[1712] = (s.v[1454] > s.v[965]);
        s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1466, 790);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1448, 965);
            s.copy_ad(1484, 790);
            s.copy_ad(1512, 790);
            s.store_sub_ad_rhs(1465, 1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)));
            s.copy_ad(1510, 1466);
            s.copy_ad(1473, 1465);
            s.store_mul(1499, 1448, 1546);
            s.store_scalar(97, 1.0);
        }

        let mut assign31400_loop_guard: usize = 0;
        while {
            let assign31400_cond_e30960: f64 = (150.0 + 1.0);
            let assign31400_cond_e30962: f64 = if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (s.v[97] <= assign31400_cond_e30960)) { 1.0 } else { 0.0 };
            assign31400_cond_e30962 != 0.0
        } {
            assign31400_loop_guard += 1;
            assert!(assign31400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);
            }
            s.b[1713] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
                s.store_offset_sub(781, 1448, 965, 1e-8);
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
            s.b[1714] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
            s.b[1715] = (2.0 == 1.0);
            s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && s.b[1715]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1716] = (2.0 == 2.0);
            s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && s.b[1716]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1717] = (2.0 == 4.0);
            s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && s.b[1717]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1718] = (2.0 == 8.0);
            s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && (!s.b[1717])) && s.b[1718]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31400_body27_loop_guard: usize = 0;
            while {
                let assign31400_body27_cond_e31344: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body27_cond_e31344 != 0.0
            } {
                assign31400_body27_loop_guard += 1;
                assert!(assign31400_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && (!s.b[1714])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1448, 965, (-1e-8), 780);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_add_scaled_inputs3_indices(335, 1465, 1.0, 1435, (-1.0), 1463, 1.0);
            }
            s.b[1719] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
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
            s.b[1720] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
            s.b[1721] = (2.0 == 1.0);
            s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && s.b[1721]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1722] = (2.0 == 2.0);
            s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && s.b[1722]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1723] = (2.0 == 4.0);
            s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && s.b[1723]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1724] = (2.0 == 8.0);
            s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31400_body63_loop_guard: usize = 0;
            while {
                let assign31400_body63_cond_e31894: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body63_cond_e31894 != 0.0
            } {
                assign31400_body63_loop_guard += 1;
                assert!(assign31400_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && (!s.b[1720])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1719])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_sqrt_mul(1452, 1550, 336);
                s.store_mul(1499, 1448, 1546);
                s.store_mul_div_from_scalar_lhs(1530, (-1.034943e-10), 1448, 334);
                s.store_mul_neg_lhs(1500, 1452, 1544);
                s.store_mul_div_from_scalar_lhs(1532, (-1.034943e-10), 1452, 341);
                s.store_add_ad_lhs(1485, A::add_scaled_product(s.ad_value(1499), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1484)), 1.0), 1500);
                s.copy_ad(1487, 185);
                s.store_add(1488, 1530, 1532);
                s.store_add_scaled_product_right_ad(1486, 1465, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463)), (-1.0));
                s.store_scalar(1489, 0.0);
                s.store_scalar(1490, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);
                s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1725] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);
            s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {
                s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {
                s.store_offset(1465, 1465, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1725])) {
                s.store_sub_ad_rhs(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1465, 1465, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1726] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1465] - s.v[1473])) as f64).abs() <= 1e-12));
            s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1726]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.copy_ad(1510, 1466);
                s.copy_ad(1473, 1465);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1514, 1465);
            s.store_mul(1452, 965, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_scaled_product_indices(1484, 1465, 1.0, 1548, 1543, 1.0);
            s.copy_ad(1462, 1484);
            s.copy_ad(1467, 1484);
            s.copy_ad(1509, 1484);
        }

        s.b[1727] = (s.v[85] > s.v[1466]);
        s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1727]) {
            s.store_scalar(1479, 1.0);
        }

        s.b[1728] = (s.v[85] > s.v[1509]);
        s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && s.b[1728]) {
            s.store_scalar(1479, 3.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && (!s.b[1728])) {
            s.store_scalar(1479, 2.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
            s.copy_ad(1466, 790);
            s.copy_ad(1509, 1466);
            s.copy_ad(1467, 1466);
            s.copy_ad(1512, 1466);
            s.copy_ad(1448, 1454);
            s.store_mul(1452, 1448, 1536);
            s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);
            s.store_add_ad_lhs(1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)), 1465);
            s.copy_ad(1514, 1465);
        }

        s.b[1729] = (s.v[85] > s.v[1466]);
        s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1729]) {
            s.store_scalar(1479, 1.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1729])) {
            s.store_scalar(1479, 2.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_mul_add_scaled_inputs3_offset_rhs(335, 1549, s.ad_value(1467), 1.0, s.ad_value(1435), -1.0, s.ad_value(961), 1.0, 0.0);
        }

        s.b[1730] = (s.v[335] > 0.0);
        s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1730]) {
            s.store_add_scaled_inputs3_mixed_iia(1455, 1435, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1730])) {
            s.store_sub(1455, 1435, 961);
        }

        s.b[1731] = (s.v[85] > s.v[1466]);
        s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) {
            s.copy_ad(1465, 1514);
            s.copy_ad(1484, 790);
            s.store_add_div_lhs(1481, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 790);
        }

        s.b[1732] = (s.v[1481] < (s.v[1512] + s.v[1553]));
        s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) && s.b[1732]) {
            s.store_add(1481, 1512, 1553);
        }

        s.b[1733] = (s.v[85] > s.v[1509]);
        s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && s.b[1733]) {
            s.copy_ad(1481, 1462);
        }

        s.b[1734] = (s.v[85] > s.v[1455]);
        s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));
            s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));
            s.copy_ad(1471, 1484);
            s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);
        }

        s.b[1735] = (s.v[1481] > (s.v[1467] - s.v[1553]));
        s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1735]) {
            s.store_sub(1481, 1467, 1553);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);
            s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);
        }

        s.b[1736] = ((s.v[1450] + s.v[1448]) > s.v[965]);
        s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign31880_loop_guard: usize = 0;
        while {
            let assign31880_cond_e33122: f64 = (150.0 + 1.0);
            let assign31880_cond_e33124: f64 = if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (s.v[97] <= assign31880_cond_e33122)) { 1.0 } else { 0.0 };
            assign31880_cond_e33124 != 0.0
        } {
            assign31880_loop_guard += 1;
            assert!(assign31880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_inputs3_indices(1468, 1450, 1.0, 1448, 1.0, 965, -1.0);
                s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1450), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1448)));
            }
            s.b[1737] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);
            s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1737]) {
                s.store_offset(1484, 1484, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1737])) {
                s.store_sub_div_rhs_indices(1484, 1484, 1468, 1508);
            }
            s.b[1738] = (((s.v[1484] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));
            s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1738]) {
                s.store_offset_sub(1484, 1435, 1463, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_product_value_ad(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));
                s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));
            }
            s.b[1739] = (s.v[335] > 0.0);
            s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1739]) {
                s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(s.ad_value(335)), 0.5, 1457, (-0.5), 1456, 1.0);
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1739])) {
                s.store_div_scaled_inputs_indices(1481, 1457, (-0.5), 1456, 1.0);
            }
            s.b[1740] = (s.v[1481] > s.v[1467]);
            s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1740]) {
                s.copy_ad(1481, 1467);
            }
            s.b[1741] = (s.v[1481] > s.v[1484]);
            s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {
                s.store_sub(1481, 1484, 1553);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);
                s.store_div_scaled_inputs2_mixed_aia(1465, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);
                s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);
            }
            s.b[1742] = ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-8);
            s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1742]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.copy_ad(1471, 1484);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && (!s.b[1734])) {
            s.copy_ad(1484, 1483);
            s.copy_ad(1465, 1464);
            s.copy_ad(1481, 1461);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.copy_ad(1482, 1484);
            s.store_scalar(79, 0.0);
            s.copy_ad(1462, 1481);
            s.copy_ad(1484, 1482);
            s.copy_ad(1474, 1462);
            s.copy_ad(1471, 1484);
            s.store_scalar(97, 1.0);
        }

    }
}
