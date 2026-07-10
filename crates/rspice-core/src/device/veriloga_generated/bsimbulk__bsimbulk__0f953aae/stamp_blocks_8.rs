#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1396]) {s.store_scaled_mul_ad(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(106), 1.0, s.ad_value(12), s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);}
        s.store_mul_sub_scaled_inputs_rhs_indices(13, 138, 200, 2.0, 144, 2.0);s.store_sqrt_square_offset(14, 13, 1.0);s.b[1397] = (s.v[13] != 0.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_add_scaled_product_mixed_iaa(162, 14, 0.5, A::div_from_scalar(1.0, s.ad_value(13)), A::asinh(s.ad_value(13)), 0.5);}
        if (!s.b[1397]) {s.store_scaled_add_mixed_ia(162, 14, A::div_from_scalar(1.0, s.ad_value(14)), 0.5);}
        s.copy_ad(163, 162);s.store_scalar(241, 0.0);s.store_scalar(242, 0.0);s.b[1398] = (p.p42 == 1.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if s.b[1398] {s.store_scalar(244, 0.0);s.store_scalar(245, 1.0);s.store_mul_voltage_ad(71, s.ad_value(187), ctx, nodes, Some(8), Some(11));s.store_sub(53, 64, 71);s.store_sub(14, 53, 63);s.store_sqrt_square_offset(15, 14, 0.01);s.store_scaled_add(77, 14, 15, 0.5);s.store_offset_mul(17, 526, 77, 1.0);s.copy_ad(51, 71);s.store_add_scaled_product_mixed_aii(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 51, 1.0);s.store_scaled_add_mixed_ia(16, 18, A::sqrt_square_offset(s.ad_value(18), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(241, 408, 239, 1.0, A::add_scaled_product(s.ad_value(529), 1.0, s.ad_value(531), s.ad_value(16), 1.0), 235, 1.0);s.store_mul_voltage_ad(67, s.ad_value(187), ctx, nodes, Some(6), Some(11));s.store_sub(55, 64, 67);s.store_sub(14, 55, 63);s.store_sqrt_square_offset(15, 14, 0.01);s.store_scaled_add(78, 14, 15, 0.5);s.store_offset_mul(17, 526, 78, 1.0);s.copy_ad(49, 67);s.store_add_scaled_product_mixed_aii(18, A::div_from_scalar(1.0, s.ad_value(17)), 1.0, 543, 49, 1.0);s.store_scaled_add_mixed_ia(16, 18, A::sqrt_square_offset(s.ad_value(18), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(242, 408, 240, 1.0, A::add_scaled_product(s.ad_value(528), 1.0, s.ad_value(530), s.ad_value(16), 1.0), 235, 1.0);}
        if (!s.b[1398]) {s.store_offset_mul(12, 526, 80, 1.0);s.store_mul_sub_rhs(13, 543, 111, 128);s.store_add_mixed_ai(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);s.store_scaled_add_mixed_ia(15, 14, A::sqrt_square_offset(s.ad_value(14), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(244, s.ad_value(408), A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), p.p2, 0.0, 235);s.copy_ad(242, 240);s.copy_ad(241, 239);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1398]) {s.store_offset_product3(245, A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30])), 1.0);}
        s.b[1399] = (p.p42 == 2.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((!s.b[1398]) && s.b[1399]) {s.store_mul_add_mixed_iai(244, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), 240);s.store_scalar(242, 0.0);s.store_scalar(241, 0.0);s.store_offset_product3(245, A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30])), 1.0);}
        s.store_add_div_rhs_mixed_ia(12, 150, 153, A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(104), s.ad_value(393), 2.0));s.store_sub(216, 200, 144);s.store_mul3_lhs(13, 12, 216, 216);s.store_offset(14, 13, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(15, 14, 0.5, A::sqrt_square_offset(s.ad_value(14), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(154, A::sqrt(A::offset(s.ad_value(15), 1.0)), 1.0, 0.5);s.store_offset_sub_scaled_inputs(154, A::offset(s.ad_value(154), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(154), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(12, 200, 144);s.store_sub(13, 200, 144);s.store_div_add_scaled_inputs_rhs_indices(14, 13, 12, 1.0, 610, 1.0);s.store_mul3_lhs(15, 609, 14, 14);s.store_offset(611, 15, 1.0);s.store_div_mixed_ia(21, 633, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(636), A::mul3(s.ad_value(639), s.ad_value(13), s.ad_value(13)))), s.ad_value(12), 1.0, s.ad_value(104), s.ad_value(393), 2.0));s.store_limited_exp_neg_input(628, 21);s.store_mul3_lhs(160, 159, 162, 245);s.store_div(157, 499, 160);s.store_mul_ad_product_lhs_mixed_ai(188, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(200), s.ad_value(144)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)))), s.ad_value(175), 1.0, s.ad_value(154), 1.0), 611, 628);s.store_scale(188, 188, p.p36);s.b[1400] = ((p.p42 == 1.0) && (p.p1094 == 1.0));s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1400] {s.store_mul_ln_mixed_ia(753, 108, A::div_scaled_inputs(s.ad_value(481), p.p1117, A::square(s.ad_value(28)), 1.0));}
        s.b[1401] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1401]) {s.store_mul_sqrt_mixed_ia(753, 108, A::offset(A::square(s.ad_value(753)), 1e-6));}
        if s.b[1400] {s.store_sub_from_scalar_scaled_input(16, 1.0, 50, p.p1113);}
        s.b[1402] = ((0.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.001)));s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1402]) {s.store_div_from_scalar_scaled_input(16, ((-0.001) * 0.001), 16, 16.0);}
        if (s.b[1400] && (!s.b[1402])) {s.store_scaled_add_mixed_ia(16, 16, A::sqrt_square_offset(s.ad_value(16), ((0.25 * 0.001) * 0.001)), 0.5);}
        if s.b[1400] {s.store_offset(13, 200, (-p.p1102));}
        s.b[1403] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1403]) {s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);}
        if (s.b[1400] && (!s.b[1403])) {s.store_scaled_add_offset_sqrt_square_offset(13, 13, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1400] {s.store_div_scaled_value_offset_denominator(14, s.ad_value(13), (10.0 * p.p1103), s.ad_value(13), (10.0 * p.p1103), 1.0);s.store_mul_scale_offset_rhs(754, 763, 14, p.p1101, 1.0);s.store_scale(23, 754, ((p.p2 * s.v[29]) * 1.60219e-19));}
        s.b[1404] = (p.p1110 != 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1404]) {s.store_abs_voltage(757, ctx, nodes, Some(6), Some(5));}
        s.b[1405] = (p.p1127 == 0.0);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1405]) {s.store_scalar(21, 1.0);}
        s.b[1406] = ((0.0 == 0.0) && ((s.v[757] - p.p1126) < ((-2500.0) * 0.5)));s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && s.b[1406]) {s.store_div_from_scalar_offset_scaled_input(22, ((-0.5) * 0.5), 757, 16.0, ((-p.p1126) * 16.0));}
        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && (!s.b[1406])) {s.store_scaled_add_sqrt_square_offset_ad(22, A::offset(s.ad_value(757), (-p.p1126)), ((0.25 * 0.5) * 0.5), 0.5);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1405])) {s.store_offset_scaled(21, 22, p.p1127, 1.0);}
        s.b[1408] = ((p.p1098 != 0.0) && (p.p514 > 0.0));s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1408]) {s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p514) as f64).ln())) / p.p515)));s.store_mul_ad_affine_product_rhs(750, 23, s.ad_value(21), A::scale_offset(A::powf(s.ad_value(760), p.p515), p.p514, 1.0), p.p1099, 0.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1408])) {s.store_scaled_mul(750, 23, 21, p.p1099);}
        if (s.b[1400] && s.b[1404]) {s.store_offset_div(14, 50, 753, 1.0);}
        s.b[1409] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1409]) {s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1409])) {s.store_scaled_add_mixed_ia(14, 14, A::sqrt_square_offset(s.ad_value(14), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1404]) {s.store_sub_scaled_inputs_mixed_ai(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 1.0, 50, p.p1125);}
        s.b[1410] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1410]) {s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1410])) {s.store_scaled_add_mixed_ia(18, 18, A::sqrt_square_offset(s.ad_value(18), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1404]) {s.store_mul(750, 18, 750);s.store_mul3_affine_lhs(19, 762, 235, p.p1110, 0.0, 16);s.store_mul(755, 750, 19);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1400] && s.b[1404]) {let t0: A = A::powf(s.ad_value(757), (4.0 - p.p1107));s.store_div_ad(752, t0, A::add_scaled_inputs(t0, 1.0, A::powf(s.ad_value(755), (4.0 - p.p1107)), p.p1122));s.store_powf(17, 752, (1.0 / p.p1107));s.store_div_scaled_product_indices(20, 17, 757, 1.0, 755, 1.0);}
        s.b[1411] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1404]) && s.b[1411]) {s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);}
        if ((s.b[1400] && s.b[1404]) && (!s.b[1411])) {s.store_scaled_add_mixed_ia(20, 20, A::sqrt_square_offset(s.ad_value(20), ((0.25 * 0.001) * 0.001)), 0.5);}
        if (s.b[1400] && s.b[1404]) {s.store_mul_powf_mixed_ia(759, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));}
        s.b[1412] = (p.p1112 != 0.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1412]) {s.store_abs_voltage(758, ctx, nodes, Some(7), Some(8));}
        s.b[1414] = ((p.p1098 != 0.0) && (p.p516 > 0.0));s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1414]) {s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p516) as f64).ln())) / p.p517)));s.store_mul_scaled_powf_scale_offset_rhs(751, 23, p.p1109, 760, p.p517, p.p516, 1.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1414])) {s.store_scale(751, 23, p.p1109);}
        if (s.b[1400] && s.b[1412]) {s.store_offset_div(14, 50, 753, 1.0);}
        s.b[1415] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1415]) {s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1415])) {s.store_scaled_add_mixed_ia(14, 14, A::sqrt_square_offset(s.ad_value(14), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_sub_scaled_inputs_mixed_ai(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 1.0, 50, p.p1125);}
        s.b[1416] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1416]) {s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1416])) {s.store_scaled_add_mixed_ia(18, 18, A::sqrt_square_offset(s.ad_value(18), ((0.25 * 0.05) * 0.05)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_mul(751, 18, 751);s.store_mul3_affine_lhs(19, 762, 235, p.p1112, 0.0, 16);s.store_mul(756, 751, 19);let t1: A = A::powf(s.ad_value(758), (4.0 - p.p1107));s.store_div_ad(752, t1, A::add_scaled_inputs(t1, 1.0, A::powf(s.ad_value(756), (4.0 - p.p1107)), p.p1122));s.store_powf(17, 752, (1.0 / p.p1107));s.store_div_scaled_product_indices(20, 17, 758, 1.0, 756, 1.0);}
        s.b[1417] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1412]) && s.b[1417]) {s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);}
        if ((s.b[1400] && s.b[1412]) && (!s.b[1417])) {s.store_scaled_add_mixed_ia(20, 20, A::sqrt_square_offset(s.ad_value(20), ((0.25 * 0.001) * 0.001)), 0.5);}
        if (s.b[1400] && s.b[1412]) {s.store_mul_powf_mixed_ia(761, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));}
        s.b[1418] = ((p.p1110 != 0.0) && (p.p1112 != 0.0));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if (s.b[1400] && s.b[1418]) {s.store_div_scaled_product_mixed_iia(17, 57, 188, 1.0, A::min(s.ad_value(750), s.ad_value(751)), 1.0);s.store_offset_sub_scaled_inputs(17, A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(17), (-1.0)), ((0.25 * p.p1108) * p.p1108)), 0.5, (0.25 * p.p1108));s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));}
        s.b[1419] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if ((s.b[1400] && s.b[1418]) && s.b[1419]) {s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1400] && s.b[1418]) && (!s.b[1419])) {s.store_scaled_add_offset_sqrt_square_offset(17, 17, (-1.0), (-(-1.0)), ((0.25 * p.p1108) * p.p1108), 0.5);}
        if (s.b[1400] && s.b[1418]) {s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));s.store_mul_ad_product_lhs_mixed_ia(188, 57, A::min(s.ad_value(750), s.ad_value(751)), 17);}
        s.b[1420] = (p.p1110 != 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {s.store_div_scaled_product_indices(17, 57, 188, 1.0, 750, 1.0);s.store_offset_sub_scaled_inputs(17, A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(17), (-1.0)), ((0.25 * p.p1108) * p.p1108)), 0.5, (0.25 * p.p1108));s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));}
        s.b[1421] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && s.b[1421]) {s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);}
        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && (!s.b[1421])) {s.store_scaled_add_offset_sqrt_square_offset(17, 17, (-1.0), (-(-1.0)), ((0.25 * p.p1108) * p.p1108), 0.5);}
        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));s.store_mul3_lhs(188, 57, 750, 17);}
        s.b[1422] = (p.p1112 != 0.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {s.store_div_scaled_product_indices(17, 57, 188, 1.0, 751, 1.0);s.store_offset_sub_scaled_inputs(17, A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(17), (-1.0)), ((0.25 * p.p1108) * p.p1108)), 0.5, (0.25 * p.p1108));s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));}
        s.b[1423] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && s.b[1423]) {s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);}
        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && (!s.b[1423])) {s.store_scaled_add_offset_sqrt_square_offset(17, 17, (-1.0), (-(-1.0)), ((0.25 * p.p1108) * p.p1108), 0.5);}
        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));s.store_mul3_lhs(188, 57, 751, 17);}
        s.store_scalar(774, 0.0);s.store_scalar(775, 0.0);s.store_scalar(776, 0.0);s.store_scalar(777, 0.0);s.b[1424] = (((p.p42 == 1.0) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if s.b[1424] {s.store_offset_scaled(764, 232, -1.0, (-p.p1114));s.store_div(764, 764, 108);s.store_scaled_sqrt_scaled_input(765, 109, (((2.0 * 1.60219e-19) * s.v[26]) * p.p1117), 1.0 / (s.v[46]));s.store_ln_ad(766, A::max_with_scalar(A::div_from_scalar(p.p1117, s.ad_value(28)), 1e-38));s.store_scalar(13, 1.0);s.store_div(204, 764, 13);s.store_div(205, 765, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1425] = (s.v[204] < 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1425]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1424] && s.b[1425]) {s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1424] && (!s.b[1425])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(767, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1424] {s.store_scaled_add_offset_sqrt_square_offset(20, 767, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 765, 1.0);s.store_add_scaled_inputs3_mixed_iia(13, 767, 1.0, 766, (-2.0), A::div(s.ad_value(69), s.ad_value(108)), -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1426] = (s.v[20] <= (-68.0));s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1426]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1427] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1426]) && s.b[1427]) {s.store_limited_exp(15, 16);}
        s.b[1428] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && s.b[1428]) {s.store_limited_exp(15, 20);}
        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && (!s.b[1428])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1424] && s.b[1426]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(768, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (s.b[1424] && (!s.b[1426])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1424] && (!s.b[1426])) {s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(768, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1429] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1429]) {s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);}
        if (s.b[1424] && (!s.b[1429])) {s.store_scaled_add_offset_sqrt_square_offset(769, 767, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1424] {s.store_sqrt(770, 769);s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1430] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1430]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);}
        if (s.b[1424] && (!s.b[1430])) {s.store_scaled_add_offset_sqrt_square_offset(12, 771, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1424] {s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);s.store_mul3_ad_middle(775, A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), 108, A::add_scaled_inputs_product(s.ad_value(764), 1.0, s.ad_value(773), (-1.0), s.ad_value(772), s.ad_value(768), (-2.0)));}
        s.b[1431] = (p.p1118 > 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1431]) {s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));}
        if (s.b[1424] && (!s.b[1431])) {s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);}
        if s.b[1424] {s.store_mul_product3_indices(774, 768, 772, 108, 12, (((p.p2 * s.v[33]) * p.p1116) * 2.0));}
        s.b[1432] = (p.p1096 == 1.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1424] && s.b[1432]) {s.store_offset_ad(764, A::mul_scaled_lhs(s.ad_value(187), -1.0, A::voltage(ctx, nodes, Some(10), Some(7))), (-p.p1114));s.store_div(764, 764, 108);s.store_scalar(13, 1.0);s.store_div(204, 764, 13);s.store_div(205, 765, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1433] = (s.v[204] < 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1432]) && s.b[1433]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if ((s.b[1424] && s.b[1432]) && (!s.b[1433])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(767, A::square(s.ad_value(14)), 1.0, 15);}
        if (s.b[1424] && s.b[1432]) {s.store_scaled_add_offset_sqrt_square_offset(20, 767, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 765, 1.0);s.store_add_scaled_inputs3_mixed_iia(13, 767, 1.0, 766, (-2.0), A::div_scaled_product(s.ad_value(187), A::voltage(ctx, nodes, Some(7), Some(11)), 1.0, s.ad_value(108), 1.0), -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1424] && s.b[1432]) {s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1434] = (s.v[20] <= (-68.0));s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1435] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if (((s.b[1424] && s.b[1432]) && s.b[1434]) && s.b[1435]) {s.store_limited_exp(15, 16);}
        s.b[1436] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && s.b[1436]) {s.store_limited_exp(15, 20);}
        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && (!s.b[1436])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(768, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if ((s.b[1424] && s.b[1432]) && (!s.b[1434])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1424] && s.b[1432]) && (!s.b[1434])) {s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(768, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1437] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1432]) && s.b[1437]) {s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);}
        if ((s.b[1424] && s.b[1432]) && (!s.b[1437])) {s.store_scaled_add_offset_sqrt_square_offset(769, 767, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if (s.b[1424] && s.b[1432]) {s.store_sqrt(770, 769);s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);}
        s.b[1438] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1432]) && s.b[1438]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);}
        if ((s.b[1424] && s.b[1432]) && (!s.b[1438])) {s.store_scaled_add_offset_sqrt_square_offset(12, 771, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if (s.b[1424] && s.b[1432]) {s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);s.store_mul3_ad_middle(777, A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), 108, A::add_scaled_inputs_product(s.ad_value(764), 1.0, s.ad_value(773), (-1.0), s.ad_value(772), s.ad_value(768), (-2.0)));}
        s.b[1439] = (p.p1118 > 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if ((s.b[1424] && s.b[1432]) && s.b[1439]) {s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));}
        if ((s.b[1424] && s.b[1432]) && (!s.b[1439])) {s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);}
        if (s.b[1424] && s.b[1432]) {s.store_mul_product3_indices(776, 768, 772, 108, 12, (((p.p2 * s.v[33]) * p.p1116) * 2.0));}
        s.store_scalar(254, 0.0);s.b[1440] = (p.p7 > 1.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if s.b[1440] {s.store_scaled_mul(255, 157, 80, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));s.store_scale(21, 108, p.p755);s.store_scaled_mul(12, 21, 157, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));s.store_scaled_add(254, 12, 255, (p.p754 * p.p2));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1441] = (p.p7 == 2.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (s.b[1440] && s.b[1441]) {s.store_primal_div_from_scalar(253, 1.0, 252);}
        s.b[1442] = (s.v[253] < p.p1093);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if ((s.b[1440] && s.b[1441]) && s.b[1442]) {s.store_scalar(253, p.p1093);s.store_primal_div_from_scalar(252, 1.0, 253);}
        if (s.b[1440] && s.b[1441]) {s.store_add(23, 252, 254);s.store_div_scaled_product_indices(254, 252, 254, 1.0, 23, 1.0);}
        s.b[1443] = (p.p1094 == 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });s.b[1444] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });s.b[1445] = (s.v[167] > (s.v[558] / 80.0));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (!s.b[1444])) && s.b[1445]) {s.store_div_scaled_inputs_indices(13, 558, -1.0, 167, 1.0);}
        s.b[1446] = (p.p1094 == 1.0);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[1446]) {s.store_mul_scale_offset_mixed_ia(184, 140, A::mul(s.ad_value(555), s.ad_value(74)), 1.0, 1.0);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(184)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(183, 74, 20);s.store_sub(185, 74, 183);}
        s.b[1447] = ((0.0 == 0.0) && (s.v[185] < ((-2500.0) * 0.001)));s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[1446]) && s.b[1447]) {s.store_div_from_scalar_scaled_input(185, ((-0.001) * 0.001), 185, 16.0);}
        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1447])) {s.store_scaled_add_mixed_ia(185, 185, A::sqrt_square_offset(s.ad_value(185), ((0.25 * 0.001) * 0.001)), 0.5);}
        if ((!s.b[1443]) && s.b[1446]) {s.store_mul_scaled_offset_ad_rhs(181, 558, 0.5, A::powf(s.ad_value(183), s.v[556]), 1.0);s.store_offset_scaled_ad(13, A::limited_exp_scaled_input(s.ad_value(76), p.p492), p.p493, 1.0);s.store_div(182, 553, 13);s.store_mul_add_scaled_product_rhs_mixed_aii(14, 182, A::scale_offset(s.ad_value(61), p.p505, 1.0), 1.0, 61, 61, p.p506);}
        s.b[1448] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 1e-12)));s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[1446]) && s.b[1448]) {s.store_div_from_scalar_scaled_input(182, ((-1e-12) * 1e-12), 14, 16.0);}
        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1448])) {s.store_scaled_add_mixed_ia(182, 14, A::sqrt_square_offset(s.ad_value(14), ((0.25 * 1e-12) * 1e-12)), 0.5);}
        s.b[1449] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });s.b[1450] = (s.v[185] > (s.v[181] / 80.0));s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[1446]) && (!s.b[1449])) && s.b[1450]) {s.store_div_scaled_inputs_mixed_ia(13, 181, -1.0, A::powf(s.ad_value(185), p.p524), 1.0);}
        s.b[1451] = ((p.p1094 == 1.0) && (p.p1098 == 1.0));s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if s.b[1451] {s.store_offset(13, 200, (-p.p1105));}
        s.b[1452] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if (s.b[1451] && s.b[1452]) {s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);}
        if (s.b[1451] && (!s.b[1452])) {s.store_scaled_add_offset_sqrt_square_offset(13, 13, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1451] {s.store_div_scaled_value_offset_denominator(14, s.ad_value(13), (10.0 * p.p1106), s.ad_value(13), (10.0 * p.p1106), 1.0);s.store_mul_scale_offset_rhs(754, 763, 14, p.p1104, 1.0);s.store_div_scaled_inputs_indices(778, 188, p.p502, 754, ((p.p2 * s.v[29]) * 1.60219e-19));s.store_offset_scaled(779, 778, 1.0 / (p.p1099), (-1.0));}
        s.b[1453] = ((0.0 == 0.0) && (s.v[779] < ((-2500.0) * p.p504)));s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if (s.b[1451] && s.b[1453]) {s.store_div_from_scalar_scaled_input(779, ((-p.p504) * p.p504), 779, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);
        if (s.b[1451] && (!s.b[1453])) {s.store_scaled_add_mixed_ia(779, 779, A::sqrt_square_offset(s.ad_value(779), ((0.25 * p.p504) * p.p504)), 0.5);}
        if s.b[1451] {s.store_scale(779, 779, p.p1099);}
        s.b[1454] = (p.p514 > 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });s.b[1455] = ((0.0 == 0.0) && (((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) - (p.p514 * ((s.v[760]) as f64).powf(p.p513))) < ((-2500.0) * 0.05)));s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if ((s.b[1451] && s.b[1454]) && s.b[1455]) {s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 16.0, A::powf(s.ad_value(760), p.p513), (p.p514 * 16.0)));}
        if ((s.b[1451] && s.b[1454]) && (!s.b[1455])) {let t2: A = A::powf(s.ad_value(760), p.p513);s.store_add_scaled_inputs3_offset(14, A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), 0.5, t2, ((-p.p514) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 1.0, t2, p.p514), ((0.25 * 0.05) * 0.05)), 0.5, ((-p.p503) * 0.5));}
        s.b[1456] = ((0.0 == 0.0) && ((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) < ((-2500.0) * 0.05)));s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((s.b[1451] && (!s.b[1454])) && s.b[1456]) {s.store_div_from_scalar_scaled_ad(14, ((-0.05) * 0.05), A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 16.0);}
        if ((s.b[1451] && (!s.b[1454])) && (!s.b[1456])) {s.store_scaled_add_sqrt_square_offset_ad(14, A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), ((0.25 * 0.05) * 0.05), 0.5);}
        if s.b[1451] {s.store_scale(15, 779, ((2.0 * 1.60219e-19) / (p.p110 * 8.85418e-12)));s.store_powf_ad(15, A::mul(s.ad_value(15), s.ad_value(14)), 0.5);s.store_add_scaled_product_indices(16, 61, p.p507, 61, 61, p.p508);s.store_add_scaled_inputs_mixed_ia(17, 14, p.p509, A::powf(s.ad_value(14), p.p511), p.p510);s.store_scaled_add_mixed_ai(18, A::offset(s.ad_value(16), 1.0), 17, p.p500);}
        s.b[1458] = (s.v[15] > (p.p501 / 80.0));s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if (s.b[1451] && s.b[1458]) {s.store_div_from_scalar(13, (-p.p501), 15);}
        s.b[1459] = ((p.p46 != 0.0) || (p.p47 != 0.0));s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });
        if s.b[1459] {s.store_mul_add_scaled_inputs4_indices_rhs(277, 106, 59, 1.0, 91, (-1.0), 200, 1.0, 144, 1.0);s.store_sqrt_square_offset(13, 277, 0.0001);s.store_scaled_sub(279, 13, 277, 0.5);s.store_scaled_add(278, 277, 13, 0.5);}
        s.b[1460] = (p.p47 != 0.0);s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });
        if (s.b[1459] && s.b[1460]) {s.store_div_scaled_value_by_product_indices(13, 277, 1.0, 589, 108, 1.0);s.store_add_scaled_product_indices(14, 586, 1.0, 587, 279, (-1.0));s.store_offset_mul(15, 588, 279, 1.0);s.store_scaled_mul(16, 14, 15, ((-745669000000.0) * p.p77));s.store_limited_exp(17, 16);s.store_scalar(18, 4.97232e-7);s.store_div_scaled_inputs2_by_product_indices(13, 277, 1.0, 584, (-1.0), 585, 108, 1.0);s.store_add_scaled_product_indices(14, 581, 1.0, 582, 278, (-1.0));s.store_offset_mul(15, 583, 278, 1.0);s.store_scaled_mul(16, 14, 15, ((-982222000000.0) * p.p77));s.store_limited_exp(17, 16);s.store_scalar(18, 3.75956e-7);}
        s.b[1461] = (p.p46 != 0.0);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if (s.b[1459] && s.b[1461]) {s.store_add_scaled_product_indices(13, 590, 1.0, 591, 278, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1459] && s.b[1461]) {s.store_offset_mul(14, 592, 278, 1.0);s.store_scaled_mul(15, 13, 14, s.v[295]);s.store_mul_product3_mixed_aiia(16, A::limited_exp(s.ad_value(15)), 90, 106, A::add(s.ad_value(200), s.ad_value(144)), 1.0);s.store_offset_sqrt_ad(280, A::offset(A::square(s.ad_value(139)), 0.01), (-0.1));s.store_scale(13, 280, s.v[600]);s.store_limited_exp_neg_input(289, 13);s.store_offset_add(15, 13, 289, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(16, 1.0, A::mul_offset_lhs(s.ad_value(13), 1.0, s.ad_value(289)), 0.0001);s.store_offset_square(17, 13, 0.0002);s.store_sub(14, 52, 63);s.store_sqrt_square_offset(77, 14, 0.0001);}
        s.b[1463] = (p.p1041 == 1.0);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1461]) && s.b[1463]) {
            if (!((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(13, A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(13, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)));
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        s.b[1464] = (s.v[595] < 0.01);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1461]) && s.b[1463]) && s.b[1464]) {s.store_scalar(595, 0.01);}
        if ((s.b[1459] && s.b[1461]) && (!s.b[1463])) {s.store_add_scaled_product_indices(13, 593, 1.0, 594, 77, (-1.0));}
        if (s.b[1459] && s.b[1461]) {s.store_offset_mul(14, 595, 77, 1.0);s.store_mul3_lhs(15, 297, 13, 14);s.store_limited_exp(16, 15);s.store_sub(14, 54, 63);s.store_sqrt_square_offset(78, 14, 0.0001);}
        s.b[1465] = (p.p1041 == 1.0);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1461]) && s.b[1465]) {
            if (!((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(13, A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(13, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)));
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        s.b[1466] = (s.v[598] < 0.01);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1461]) && s.b[1465]) && s.b[1466]) {s.store_scalar(598, 0.01);}
        if ((s.b[1459] && s.b[1461]) && (!s.b[1465])) {s.store_add_scaled_product_indices(13, 596, 1.0, 597, 78, (-1.0));}
        if (s.b[1459] && s.b[1461]) {s.store_offset_mul(14, 598, 78, 1.0);s.store_mul3_lhs(15, 297, 13, 14);s.store_limited_exp(16, 15);}
        s.b[1467] = (p.p45 != 0.0);s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });
        if s.b[1467] {s.store_scalar(12, (s.v[47] * p.p77));}
        s.b[1468] = (((s.v[559] <= 0.0) || (s.v[417] <= 0.0)) || (s.v[561] < 0.0));s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if (s.b[1467] && s.b[1468]) {s.store_scalar(18, 0.0);}
        if (s.b[1467] && (!s.b[1468])) {s.store_div_scaled_inputs3_indices(13, 54, -1.0, 562, (-1.0), 63, 1.0, 12, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1467] && (!s.b[1468])) {
            if (!(s.v[13] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_mixed_ia(13, 13, A::sqrt_square_offset(s.ad_value(13), ((4.0 * 0.01) * 0.01)), 0.5);
            } else {
                if (s.v[13] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(13, ((-0.01) * 0.01), 13);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        if (s.b[1467] && (!s.b[1468])) {s.store_div_scaled_value_offset_denominator(14, s.ad_value(417), 1.0, s.ad_value(13), 0.001, 1.0);}
        s.b[1469] = (s.v[561] != 0.0);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {s.store_mul_square_lhs(15, 48, 48);s.store_offset_add_ad(16, s.ad_value(561), A::abs(s.ad_value(15)), 0.0001);}
        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {
            s.store_offset_ad(17, {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(15), s.ad_value(16)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1467] && (!s.b[1468])) && (!s.b[1469])) {s.store_scalar(17, 1.0);}
        if (s.b[1467] && (!s.b[1468])) {s.store_mul_product3_mixed_iiia(18, 17, 559, 13, A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]);}
        s.b[1470] = (((s.v[563] <= 0.0) || (s.v[418] <= 0.0)) || (s.v[565] < 0.0));s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if (s.b[1467] && s.b[1470]) {s.store_scalar(18, 0.0);}
        if (s.b[1467] && (!s.b[1470])) {s.store_div_scaled_inputs3_indices(13, 52, -1.0, 566, (-1.0), 63, 1.0, 12, 1.0);}
        if (s.b[1467] && (!s.b[1470])) {
            if (!(s.v[13] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_mixed_ia(13, 13, A::sqrt_square_offset(s.ad_value(13), ((4.0 * 0.01) * 0.01)), 0.5);
            } else {
                if (s.v[13] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(13, ((-0.01) * 0.01), 13);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        if (s.b[1467] && (!s.b[1470])) {s.store_div_scaled_value_offset_denominator(14, s.ad_value(418), 1.0, s.ad_value(13), 0.001, 1.0);}
        s.b[1471] = (s.v[565] != 0.0);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {s.store_mul_square_lhs(15, 50, 50);s.store_offset_add_ad(16, s.ad_value(565), A::abs(s.ad_value(15)), 0.0001);}
        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {
            s.store_offset_ad(17, {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(15), s.ad_value(16)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1467] && (!s.b[1470])) && (!s.b[1471])) {s.store_scalar(17, 1.0);}
        if (s.b[1467] && (!s.b[1470])) {s.store_mul_product3_mixed_iiia(18, 17, 563, 13, A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]);}
        s.store_div(12, 306, 343);s.store_offset_limited_exp(13, 12, (-1.0));s.store_add_scaled_product_right_sub(14, 346, 1.0, 345, 306, 347, 1.0);s.store_mul(15, 13, 14);s.store_div_scaled_offset_numerator_indices(13, 306, 1.0, p.p731, 343, 1.0);s.store_limited_exp_neg_input(14, 13);s.store_mul_add_scaled_inputs3_offset_rhs_limited_exp_first(16, 341, 12, 1.0, 351, 1.0, 14, (-p.p733), (-1.0));s.store_add_scaled_product_right_sub(17, 349, 1.0, 348, 306, 350, 1.0);s.b[1472] = (s.v[341] > 0.0);s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1472] {let t3: A = A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(347), (-1.0), s.ad_value(343), 1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, t3, 1.0 / (2.0)), 1.0, 16, t3, 1.0, 1.0 / (2.0));}
        s.b[1473] = (s.v[441] > 0.0);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });s.b[1474] = ((p.p748 - s.v[306]) < (p.p748 * 0.001));s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if (s.b[1473] && s.b[1474]) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 447, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1473] && (!s.b[1474])) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 447, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p748, A::sub_from_scalar(p.p748, s.ad_value(306)), 1.0), (-1.0));}
        s.b[1475] = (s.v[443] > 0.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });s.b[1476] = ((p.p750 - s.v[306]) < (p.p750 * 0.001));s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if (s.b[1475] && s.b[1476]) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 449, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1475] && (!s.b[1476])) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 449, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p750, A::sub_from_scalar(p.p750, s.ad_value(306)), 1.0), (-1.0));}
        s.b[1477] = (s.v[445] > 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });s.b[1478] = ((p.p752 - s.v[306]) < (p.p752 * 0.001));s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if (s.b[1477] && s.b[1478]) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 451, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1477] && (!s.b[1478])) {s.store_div_scaled_value_by_product_indices(12, 306, -1.0, 394, 451, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p752, A::sub_from_scalar(p.p752, s.ad_value(306)), 1.0), (-1.0));}
        s.store_div(12, 307, 344);s.store_offset_limited_exp(13, 12, (-1.0));s.store_add_scaled_product_right_sub(14, 353, 1.0, 352, 307, 354, 1.0);s.store_mul3_lhs(15, 302, 13, 14);s.store_div_scaled_offset_numerator_indices(13, 307, 1.0, p.p732, 344, 1.0);s.store_limited_exp_neg_input(14, 13);s.store_mul_ad_product_rhs_mixed_ia(16, 302, 342, A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(358), 1.0, s.ad_value(14), (-p.p734), (-1.0)));s.store_mul_add_scaled_product_rhs_mixed_iia(17, 302, 356, 1.0, 355, A::sub(s.ad_value(307), s.ad_value(357)), 1.0);s.b[1479] = (s.v[342] > 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });s.b[1480] = (s.v[302] > 0.0);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (s.b[1479] && s.b[1480]) {let t4: A = A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, t4, 1.0 / (2.0)), 1.0, 16, t4, 1.0, 1.0 / (2.0));}
        s.b[1481] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (s.b[1479] && s.b[1481]) {s.store_div(12, 309, 344);s.store_offset_limited_exp(13, 12, (-1.0));s.store_add_scaled_product_right_sub(14, 353, 1.0, 352, 309, 354, 1.0);s.store_scaled_mul(15, 13, 14, p.p1128);s.store_div_scaled_offset_numerator_indices(13, 309, 1.0, p.p732, 344, 1.0);s.store_limited_exp_neg_input(14, 13);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(16, 342, A::limited_exp(s.ad_value(12)), p.p1128, 358, p.p1128, 14, (((-p.p734)) * (p.p1128)), (((-1.0)) * (p.p1128)));s.store_add_scaled_product_right_sub(17, 356, p.p1128, 355, 309, 357, p.p1128);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1479] && s.b[1481]) {let t5: A = A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(18, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, t5, 1.0 / (2.0)), 1.0, 16, t5, 1.0, 1.0 / (2.0));}
        s.b[1482] = (s.v[442] > 0.0);s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });s.b[1483] = ((p.p749 - s.v[307]) < (p.p749 * 0.001));s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if (s.b[1482] && s.b[1483]) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 448, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1482] && (!s.b[1483])) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 448, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(307)), 1.0), (-1.0));}
        s.b[1484] = (s.v[444] > 0.0);s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });s.b[1485] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });s.b[1486] = (s.v[301] > (s.v[35] * p.p2));s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((s.b[1484] && s.b[1485]) && s.b[1486]) {s.store_mul_ad_product_lhs_mixed_ia(14, 302, A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), 444);}
        if ((s.b[1484] && s.b[1485]) && (!s.b[1486])) {s.store_mul3_lhs(14, 302, 301, 444);}
        if (s.b[1484] && (!s.b[1485])) {s.store_mul3_lhs(14, 302, 301, 444);}
        s.b[1487] = ((p.p751 - s.v[307]) < (p.p751 * 0.001));s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1487]) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 450, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1484] && (!s.b[1487])) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 450, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(307)), 1.0), (-1.0));}
        s.b[1488] = (s.v[446] > 0.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });s.b[1489] = ((p.p753 - s.v[307]) < (p.p753 * 0.001));s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if (s.b[1488] && s.b[1489]) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 452, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if (s.b[1488] && (!s.b[1489])) {s.store_div_scaled_value_by_product_indices(12, 307, -1.0, 394, 452, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p753, A::sub_from_scalar(p.p753, s.ad_value(307)), 1.0), (-1.0));}
        s.b[1490] = (p.p1128 > 0.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });s.b[1491] = (s.v[442] > 0.0);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });s.b[1492] = ((p.p749 - s.v[309]) < (p.p749 * 0.001));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if ((s.b[1490] && s.b[1491]) && s.b[1492]) {s.store_div_scaled_value_by_product_indices(12, 309, -1.0, 394, 448, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if ((s.b[1490] && s.b[1491]) && (!s.b[1492])) {s.store_div_scaled_value_by_product_indices(12, 309, -1.0, 394, 448, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(309)), 1.0), (-1.0));}
        s.b[1493] = (s.v[444] > 0.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });s.b[1494] = (s.v[301] > (s.v[35] * p.p2));s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if ((s.b[1490] && s.b[1493]) && s.b[1494]) {s.store_mul_scale_offset_rhs(14, 444, 301, p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2))));}
        if ((s.b[1490] && s.b[1493]) && (!s.b[1494])) {s.store_scaled_mul(14, 301, 444, p.p1128);}
        s.b[1495] = ((p.p751 - s.v[309]) < (p.p751 * 0.001));s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if ((s.b[1490] && s.b[1493]) && s.b[1495]) {s.store_div_scaled_value_by_product_indices(12, 309, -1.0, 394, 450, 1.0);s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));}
        if ((s.b[1490] && s.b[1493]) && (!s.b[1495])) {s.store_div_scaled_value_by_product_indices(12, 309, -1.0, 394, 450, 1.0);s.store_offset_ad(13, A::limited_exp_div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(309)), 1.0), (-1.0));}
        s.store_mul(312, 423, 250);s.store_mul(315, 424, 300);
    }
}
