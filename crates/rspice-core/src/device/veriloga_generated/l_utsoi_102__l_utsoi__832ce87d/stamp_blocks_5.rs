#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
    ) {
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1221] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if (((s.b[1218] && (!s.b[1219])) && s.b[1220]) && s.b[1221]) {s.store_exp(689, 688);}
        s.b[1222] = (s.v[688] < (-80.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && s.b[1222]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && (!s.b[1222])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(712, 688, 693);}
        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {s.store_mul_scale_offset_mixed_ia(694, 709, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(695, 702, 708, A::offset(A::mul(s.ad_value(694), s.ad_value(702)), 1.0));}
        s.b[1223] = ((((-s.v[695])) as f64).abs() < 80.0);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1223]) {s.store_exp_neg_input(687, 695);}
        s.b[1224] = ((-s.v[695]) < (-80.0));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && s.b[1224]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && (!s.b[1224])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(687, A::neg(s.ad_value(695)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {s.store_sub_from_scalar(693, 1.0, 687);s.store_add_scaled_inputs_product_mixed_iiia(696, 702, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(702), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));}
        s.b[1225] = ((((-s.v[696])) as f64).abs() < 80.0);s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1225]) {s.store_exp_neg_input(689, 696);}
        s.b[1226] = ((-s.v[696]) < (-80.0));s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && s.b[1226]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && (!s.b[1226])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {s.store_add_scaled_inputs3_mixed_iia(690, 702, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(702), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(712, 696, 697);}
        if (s.b[1218] && (!s.b[1219])) {s.store_neg(712, 712);}
        s.store_div_mixed_ai(704, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);s.store_square(705, 704);s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);let t0: f64 = (1e-5 * s.v[706]);s.store_scalar(707, t0);s.store_div_from_scalar(708, 1.0, 706);s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);s.b[1227] = (((p[3] > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p[4] > 0.0) && (s.v[90] > 0.0)));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });s.b[1228] = (((s.v[701]) as f64).abs() <= s.v[707]);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if (s.b[1227] && s.b[1228]) {s.store_mul_scale_offset_indices(711, 708, 701, -1.0, 0.0);}
        s.b[1229] = (s.v[701] < (-s.v[707]));s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {s.store_neg(679, 701);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1230] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (((s.b[1227] && (!s.b[1228])) && s.b[1229]) && s.b[1230]) {s.store_exp(689, 688);}
        s.b[1231] = (s.v[688] < (-80.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && s.b[1231]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && (!s.b[1231])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
    ) {
        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(711, 688, 693);}
        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {s.store_mul_scale_offset_mixed_ia(694, 709, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(695, 701, 708, A::offset(A::mul(s.ad_value(694), s.ad_value(701)), 1.0));}
        s.b[1232] = ((((-s.v[695])) as f64).abs() < 80.0);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1232]) {s.store_exp_neg_input(687, 695);}
        s.b[1233] = ((-s.v[695]) < (-80.0));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && s.b[1233]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && (!s.b[1233])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(687, A::neg(s.ad_value(695)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {s.store_sub_from_scalar(693, 1.0, 687);s.store_add_scaled_inputs_product_mixed_iiia(696, 701, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(701), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));}
        s.b[1234] = ((((-s.v[696])) as f64).abs() < 80.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1234]) {s.store_exp_neg_input(689, 696);}
        s.b[1235] = ((-s.v[696]) < (-80.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && s.b[1235]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && (!s.b[1235])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {s.store_add_scaled_inputs3_mixed_iia(690, 701, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(701), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(711, 696, 697);}
        if (s.b[1227] && (!s.b[1228])) {s.store_neg(711, 711);}
        s.b[1236] = (s.v[160] > 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });s.b[1237] = (((s.v[703]) as f64).abs() <= s.v[707]);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if (s.b[1236] && s.b[1237]) {s.store_mul_scale_offset_indices(713, 708, 703, -1.0, 0.0);}
        s.b[1238] = (s.v[703] < (-s.v[707]));s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_neg(679, 703);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
    ) {
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1239] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if (((s.b[1236] && (!s.b[1237])) && s.b[1238]) && s.b[1239]) {s.store_exp(689, 688);}
        s.b[1240] = (s.v[688] < (-80.0));s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(713, 688, 693);}
        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {s.store_mul_scale_offset_mixed_ia(694, 709, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(695, 703, 708, A::offset(A::mul(s.ad_value(694), s.ad_value(703)), 1.0));}
        s.b[1241] = ((((-s.v[695])) as f64).abs() < 80.0);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1241]) {s.store_exp_neg_input(687, 695);}
        s.b[1242] = ((-s.v[695]) < (-80.0));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && s.b[1242]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && (!s.b[1242])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(687, A::neg(s.ad_value(695)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {s.store_sub_from_scalar(693, 1.0, 687);s.store_add_scaled_inputs_product_mixed_iiia(696, 703, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(703), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));}
        s.b[1243] = ((((-s.v[696])) as f64).abs() < 80.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1243]) {s.store_exp_neg_input(689, 696);}
        s.b[1244] = ((-s.v[696]) < (-80.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
    ) {
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && s.b[1244]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && (!s.b[1244])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {s.store_add_scaled_inputs3_mixed_iia(690, 703, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(703), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(713, 696, 697);}
        if (s.b[1236] && (!s.b[1237])) {s.store_neg(713, 713);}
        s.store_mul_add_scaled_inputs_rhs_indices(714, 219, 700, -1.0, 710, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(715, 219, 701, -1.0, 711, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(345, 219, 702, -1.0, 712, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(346, 219, 703, -1.0, 713, -1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(729, 0.0);s.store_scalar(730, 0.0);s.store_scalar(347, 0.0);s.store_scalar(348, 0.0);s.store_scalar(349, 0.0);s.store_scalar(749, 0.0);s.store_scalar(750, 0.0);s.b[1245] = (p[3] > 0.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
    ) {
        s.b[1246] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1246]) {s.store_add(716, 714, 281);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(714)), 0.0001));}
        s.b[1247] = ((((0.5 * s.v[700])) as f64).abs() < 80.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1247]) {s.store_exp_scaled_input(0, 700, 0.5);}
        s.b[1248] = ((0.5 * s.v[700]) < (-80.0));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && (!s.b[1248])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(700), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);s.store_add_scaled_products_indices(722, 71, 2, 1.0, 69, 3, 1.0);s.store_scaled_mul(723, 73, 3, 1e-6);s.store_mul_div_scaled_inputs_indices(2, 275, 81, (-1.0), 718, 1.0);}
        s.b[1249] = (s.v[720] < 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1249]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 721, 0.5, 718, 721, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1246]) {s.store_add_scaled_product_mixed_aii(724, A::offset(s.ad_value(710), 3.0), 1.0, 717, 220, 1.0);}
        s.b[1250] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1250]) {s.store_exp(725, 724);}
        s.b[1251] = (s.v[724] < (-80.0));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && s.b[1251]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && (!s.b[1251])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_add_mixed_ai(724, A::add_scaled_product(A::offset(s.ad_value(710), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 700);}
        s.b[1252] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1252]) {s.store_exp(726, 724);}
        s.b[1253] = (s.v[724] < (-80.0));s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && s.b[1253]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && (!s.b[1253])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(726, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_mul_scale_offset_mixed_ia(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), 1.0, (-1.5));}
        s.b[1254] = (s.v[0] > 0.0);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1254]) {s.store_offset_mul_offset_rhs_mixed_ia(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1255] = (s.v[0] > (-80.0));s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1254])) && s.b[1255]) {s.store_exp(727, 0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1254])) && (!s.b[1255])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[1256] = (s.v[2] > 0.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1256]) {s.store_offset_mul_offset_rhs_mixed_ia(728, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
    ) {
        s.b[1257] = (s.v[2] > (-80.0));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1256])) && s.b[1257]) {s.store_exp(728, 2);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1256])) && (!s.b[1257])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(728, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[1245] && s.b[1246]) {s.store_div_scaled_offset_numerator_mixed_ia(0, 725, 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);}
        s.b[1258] = (s.v[0] < 1e-80);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1258]) {s.store_scalar(0, 1e-80);}
        if (s.b[1245] && s.b[1246]) {s.store_mul_sub_rhs(2, 85, 328, 86);}
        s.b[1259] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1259]) {s.store_exp(3, 2);}
        s.b[1260] = (s.v[2] < (-80.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && s.b[1260]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && (!s.b[1260])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 699, 1.0);}
        s.b[1261] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1261]) {s.store_exp(5, 4);}
        s.b[1262] = (s.v[4] < (-80.0));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && s.b[1262]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && (!s.b[1262])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_sub_ad(729, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(722), s.ad_value(727), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(723), s.ad_value(728), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));}
        s.b[1263] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1263]) {s.store_add(716, 715, 281);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(715)), 0.0001));}
        s.b[1264] = ((((0.5 * s.v[701])) as f64).abs() < 80.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1264]) {s.store_exp_scaled_input(0, 701, 0.5);}
        s.b[1265] = ((0.5 * s.v[701]) < (-80.0));s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && s.b[1265]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && (!s.b[1265])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(701), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);s.store_add_scaled_products_indices(722, 72, 2, 1.0, 70, 3, 1.0);s.store_scaled_mul(723, 74, 3, 1e-6);s.store_mul_div_scaled_inputs_indices(2, 275, 81, (-1.0), 718, 1.0);}
        s.b[1266] = (s.v[720] < 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1266]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 721, 0.5, 718, 721, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1263]) {s.store_add_scaled_product_mixed_aii(724, A::offset(s.ad_value(711), 3.0), 1.0, 717, 220, 1.0);}
        s.b[1267] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
    ) {
        if ((s.b[1245] && s.b[1263]) && s.b[1267]) {s.store_exp(725, 724);}
        s.b[1268] = (s.v[724] < (-80.0));s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && s.b[1268]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && (!s.b[1268])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_add_mixed_ai(724, A::add_scaled_product(A::offset(s.ad_value(711), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 701);}
        s.b[1269] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1269]) {s.store_exp(726, 724);}
        s.b[1270] = (s.v[724] < (-80.0));s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && s.b[1270]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && (!s.b[1270])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(726, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_mul_scale_offset_mixed_ia(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), 1.0, (-1.5));}
        s.b[1271] = (s.v[0] > 0.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1271]) {s.store_offset_mul_offset_rhs_mixed_ia(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1272] = (s.v[0] > (-80.0));s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1271])) && s.b[1272]) {s.store_exp(727, 0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1271])) && (!s.b[1272])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[1273] = (s.v[2] > 0.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1273]) {s.store_offset_mul_offset_rhs_mixed_ia(728, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1274] = (s.v[2] > (-80.0));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1273])) && s.b[1274]) {s.store_exp(728, 2);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1273])) && (!s.b[1274])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(728, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[1245] && s.b[1263]) {s.store_div_scaled_offset_numerator_mixed_ia(0, 725, 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);}
        s.b[1275] = (s.v[0] < 1e-80);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1275]) {s.store_scalar(0, 1e-80);}
        if (s.b[1245] && s.b[1263]) {s.store_mul_sub_rhs(2, 85, 326, 86);}
        s.b[1276] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1276]) {s.store_exp(3, 2);}
        s.b[1277] = (s.v[2] < (-80.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && s.b[1277]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && (!s.b[1277])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 698, 1.0);}
        s.b[1278] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1278]) {s.store_exp(5, 4);}
        s.b[1279] = (s.v[4] < (-80.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && s.b[1279]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && (!s.b[1279])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
    ) {
        if (s.b[1245] && s.b[1263]) {s.store_sub_ad(730, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(722), s.ad_value(727), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(723), s.ad_value(728), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));}
        s.b[1280] = (s.v[68] > 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1280]) {s.store_mul_scale_offset_indices(731, 382, 432, -1.0, 0.0);}
        s.b[1281] = (((((2.0 * s.v[731]) - s.v[407])) as f64).abs() < 80.0);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1281]) {s.store_exp_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0));}
        s.b[1282] = (((2.0 * s.v[731]) - s.v[407]) < (-80.0));s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && s.b[1282]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && (!s.b[1282])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul_sub_mixed_iaa(732, 222, A::offset(s.ad_value(731), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0)));s.store_scaled_add(733, 388, 408, 0.5);s.store_mul(734, 222, 733);s.store_add(716, 734, 280);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(734)), 0.0001));}
        s.b[1283] = (s.v[79] < 0.0);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1283]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 276, 0.5, 718, 276, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1280]) {s.store_add(736, 396, 230);s.store_sub(735, 736, 733);s.store_mul_add_scaled_product_rhs_mixed_iai(724, 282, 735, 1.0, A::add_scaled_inputs3(s.ad_value(717), 1.0, s.ad_value(279), (-1.0), s.ad_value(732), -1.0), 223, 1.0);}
        s.b[1284] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1284]) {s.store_exp(725, 724);}
        s.b[1285] = (s.v[724] < (-80.0));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && s.b[1285]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && (!s.b[1285])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul_ad_affine_product_lhs(724, A::sub(s.ad_value(331), s.ad_value(732)), s.ad_value(223), -1.0, 0.0, 282);}
        s.b[1286] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1286]) {s.store_exp(0, 724);}
        s.b[1287] = (s.v[724] < (-80.0));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && s.b[1287]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && (!s.b[1287])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul(726, 725, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
    ) {
        if (s.b[1245] && s.b[1280]) {s.store_mul_scale_offset_mixed_ia(0, 274, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(718), 1.0)), 1.0, (-1.5));}
        s.b[1288] = (s.v[0] > 0.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1288]) {s.store_offset_mul_offset_rhs_mixed_ia(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1289] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1288])) && s.b[1289]) {s.store_exp(727, 0);}
        s.b[1290] = (s.v[0] < (-80.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if ((((s.b[1245] && s.b[1280]) && (!s.b[1288])) && (!s.b[1289])) && s.b[1290]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1245] && s.b[1280]) && (!s.b[1288])) && (!s.b[1289])) && (!s.b[1290])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(727, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul_ad_product_rhs_mixed_ia(737, 68, 727, A::ln(A::div_scaled_offset_numerator(s.ad_value(725), 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0)));}
        s.b[1291] = ((s.v[736] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1291]) {s.store_scalar(738, 1.0);s.store_scalar(739, 0.5);}
        if ((s.b[1245] && s.b[1280]) && (!s.b[1291])) {s.store_add_scaled_product_indices(0, 78, 1.0, 79, 718, 2.0);s.store_mul_div_mixed_iia(740, 223, 87, A::mul(s.ad_value(0), s.ad_value(274)));s.store_div(741, 731, 740);s.store_mul3_lhs(742, 740, 430, 397);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(744, 742, 1.0, 742, 1.0, 0.5);s.store_sub_from_scalar_scaled_input(743, 0.5, 744, 3.0);}
        s.b[1292] = (s.v[741] < 0.001);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && s.b[1292]) {s.store_square(745, 741);s.store_offset_mul_ad(738, s.ad_value(745), A::add_scaled_product(A::scale_offset(s.ad_value(742), 0.3333333333333, 0.1666666666667), 1.0, s.ad_value(745), A::scale_offset(s.ad_value(742), 0.2, 0.05), 0.1666666666667), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(739, 738, 0.5, 741, A::mul(s.ad_value(745), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(744), 0.25, 0.4), 1.0, s.ad_value(745), s.ad_value(744), 0.125, 0.0285714285714)), 1.0, (-0.1666666666667));}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {s.store_div_from_scalar(746, 1.0, 741);}
        s.b[1293] = (((s.v[741]) as f64).abs() < 80.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {s.store_exp(747, 741);}
        s.b[1294] = (s.v[741] < (-80.0));s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(747, 1.80485e-35, A::neg(s.ad_value(741)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(747, 741, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {s.store_div_from_scalar(748, 1.0, 747);s.store_sub(0, 747, 748);s.store_add(3, 747, 748);s.store_add_scaled_products_mixed_aiii(738, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(742), s.ad_value(0)), 746, 0.5, 742, 3, 0.5);s.store_scaled_sub_ad(739, A::add_scaled_product(s.ad_value(738), 1.0, s.ad_value(0), A::sub(s.ad_value(744), A::mul3(s.ad_value(743), s.ad_value(746), s.ad_value(746))), (-1.0)), A::mul3(s.ad_value(743), s.ad_value(3), s.ad_value(746)), 0.5);}
        if (s.b[1245] && s.b[1280]) {s.store_mul(347, 737, 738);s.store_mul(750, 737, 739);s.store_sub(749, 347, 750);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1295] = (s.v[330] < 0.0);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1295]) {s.store_add(348, 750, 729);s.store_add(349, 749, 730);}
        if (s.b[1245] && (!s.b[1295])) {s.store_add(348, 749, 729);s.store_add(349, 750, 730);}
        s.store_scalar(351, 0.0);s.b[1296] = (((p[4] > 0.0) && (s.v[89] > 0.0)) && (s.v[714] < 0.0));s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if s.b[1296] {s.store_sqrt_offset_ad(751, A::add(A::square(s.ad_value(714)), A::mul3(A::square(s.ad_value(95)), s.ad_value(327), s.ad_value(327))), 1e-6);s.store_div_scaled_inputs_indices(0, 91, -1.0, 751, 1.0);}
        s.b[1297] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if (s.b[1296] && s.b[1297]) {s.store_exp(3, 0);}
        s.b[1298] = (s.v[0] < (-80.0));s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if ((s.b[1296] && (!s.b[1297])) && s.b[1298]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1296] && (!s.b[1297])) && (!s.b[1298])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1296] {s.store_mul(4, 97, 699);}
        s.b[1299] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1296] && s.b[1299]) {s.store_exp(5, 4);}
        s.b[1300] = (s.v[4] < (-80.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if ((s.b[1296] && (!s.b[1299])) && s.b[1300]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1296] && (!s.b[1299])) && (!s.b[1300])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1296] {s.store_scaled_mul_ad(351, A::mul3(A::mul3_scaled_output(s.ad_value(89), s.ad_value(699), s.ad_value(714), -1.0), s.ad_value(751), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);}
        s.store_scalar(350, 0.0);s.b[1301] = (((p[4] > 0.0) && (s.v[90] > 0.0)) && (s.v[715] < 0.0));s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if s.b[1301] {s.store_sqrt_offset_ad(752, A::add(A::square(s.ad_value(715)), A::mul3(A::square(s.ad_value(96)), s.ad_value(329), s.ad_value(329))), 1e-6);s.store_div_scaled_inputs_indices(0, 92, -1.0, 752, 1.0);}
        s.b[1302] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (s.b[1301] && s.b[1302]) {s.store_exp(3, 0);}
        s.b[1303] = (s.v[0] < (-80.0));s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if ((s.b[1301] && (!s.b[1302])) && s.b[1303]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1301] && (!s.b[1302])) && (!s.b[1303])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1301] {s.store_mul(4, 98, 698);}
        s.b[1304] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if (s.b[1301] && s.b[1304]) {s.store_exp(5, 4);}
        s.b[1305] = (s.v[4] < (-80.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1301] && (!s.b[1304])) && s.b[1305]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1301] && (!s.b[1304])) && (!s.b[1305])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1301] {s.store_scaled_mul_ad(350, A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(698), s.ad_value(715), -1.0), s.ad_value(752), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);}
        s.store_scalar(352, 0.0);s.b[1306] = (p[12] > 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if s.b[1306] {s.store_mul(754, 332, 285);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
    ) {
        if s.b[1306] {s.store_mul_scale_offset_mixed_ia(755, 285, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(756, 754, 755, 0.5);s.store_sub_mixed_ai(757, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub(s.ad_value(331), s.ad_value(100)), s.ad_value(285), 1.0), 230);s.store_sub_mixed_ai(758, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(101), 1.0), s.ad_value(285), 1.0), 230);s.store_div_from_scalar_offset_input(759, 1.0, 105, 1.0);s.store_div_from_scalar_offset_input(760, 1.0, 106, 1.0);s.store_mul(761, 109, 285);s.store_mul_scaled_offset_ad_rhs(0, 761, 2.0, A::sqrt(A::offset(A::div(s.ad_value(755), s.ad_value(761)), 1.0)), (-1.0));s.store_mul(762, 107, 0);s.store_mul(763, 108, 0);s.store_add_scaled_product_mixed_iai(764, 756, 1.0, A::add(s.ad_value(757), s.ad_value(762)), 759, 1.0);s.store_add_scaled_product_mixed_iai(765, 756, 1.0, A::add(s.ad_value(758), s.ad_value(763)), 760, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(766, A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), s.ad_value(221))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(767, A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), s.ad_value(221))), 0.01), (-0.5));s.store_div(768, 242, 759);s.store_div(769, 243, 760);s.store_div_from_scalar(770, 1.0, 768);s.store_div_from_scalar(771, 1.0, 769);s.store_div_from_scalar_add_ad(772, 1.0, A::offset(s.ad_value(770), 1.0), s.ad_value(771));s.store_div_square_rhs(773, 286, 386);s.store_mul_sub_rhs(774, 772, 766, 767);}
        s.b[1307] = ((((s.v[767] - s.v[766])) as f64).abs() <= 1e-12);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1307]) {s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(772), s.ad_value(770)), 1.0, 772, 771, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
    ) {
        if (s.b[1306] && s.b[1307]) {s.store_mul_add_scaled_inputs4_rhs_mixed_iaaa(3, 774, 771, 1.0, A::mul3_scaled_output(s.ad_value(770), s.ad_value(772), s.ad_value(770), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(771), s.ad_value(772), s.ad_value(771), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(772)), -1.0);s.store_div_scaled_product_mixed_aii(4, A::sub(s.ad_value(2), s.ad_value(3)), 773, 0.5, 772, 1.0);}
        if (s.b[1306] && (!s.b[1307])) {s.store_exp_mul_scaled_lhs_indices(2, 770, -1.0, 774);s.store_exp_ad(3, A::mul(A::sub(s.ad_value(771), A::div_from_scalar(1.0, s.ad_value(772))), s.ad_value(774)));s.store_div_scaled_product_mixed_iai(4, 773, A::sub(s.ad_value(2), s.ad_value(3)), 1.0, 774, 2.0);}
        if s.b[1306] {s.copy_ad(775, 4);}
        s.b[1308] = (s.v[766] < 80.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1308]) {s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(s.ad_value(766))), 1.0));s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        s.b[1309] = (s.v[766] < 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });s.b[1310] = (s.v[766] > (-80.0));s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && s.b[1310]) {s.store_exp(780, 766);}
        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && (!s.b[1310])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(780, 1.80485e-35, A::neg(s.ad_value(766)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1306] && (!s.b[1308])) && s.b[1309]) {s.store_mul(0, 775, 780);}
        if ((s.b[1306] && (!s.b[1308])) && (!s.b[1309])) {s.store_add_ln_lhs(780, 775, 766);s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        if s.b[1306] {s.copy_ad(776, 0);}
        s.b[1311] = ((s.v[766] - s.v[407]) < 80.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1311]) {s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(A::sub(s.ad_value(766), s.ad_value(407)))), 1.0));s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        s.b[1312] = ((s.v[766] - s.v[407]) < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });s.b[1313] = ((s.v[766] - s.v[407]) > (-80.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && s.b[1313]) {s.store_exp_sub(780, 766, 407);}
        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && (!s.b[1313])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(780, 1.80485e-35, A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1306] && (!s.b[1311])) && s.b[1312]) {s.store_mul(0, 775, 780);}
        if ((s.b[1306] && (!s.b[1311])) && (!s.b[1312])) {s.store_add_scaled_inputs3_mixed_aii(780, A::ln(s.ad_value(775)), 1.0, 766, 1.0, 407, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1306] && (!s.b[1311])) && (!s.b[1312])) {s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        if s.b[1306] {s.copy_ad(777, 0);s.store_mul_scale_offset(778, A::sub(s.ad_value(776), s.ad_value(777)), A::add_scaled_inputs(s.ad_value(776), 0.5, s.ad_value(777), 0.5), 1.0, 1.0);s.store_mul_square_lhs(779, 284, 110);s.store_div_scaled_product3_indices(352, 779, 237, 778, 1.0, 418, 1.0);}
        s.store_scalar(353, 0.0);s.store_scalar(354, 0.0);s.b[1314] = (p[8] != 0.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if s.b[1314] {s.store_div_scaled_add_product_indices(753, 335, 1.0, 115, 407, (-1.0), 223, 1.0);}
        s.b[1315] = (s.v[753] > 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (s.b[1314] && s.b[1315]) {s.store_div_scaled_value_offset_denominator(3, s.ad_value(113), (-1.0), s.ad_value(753), 1e-30, 1.0);}
        s.b[1316] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if ((s.b[1314] && s.b[1315]) && s.b[1316]) {s.store_exp(0, 3);}
        s.b[1317] = (s.v[3] < (-80.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && s.b[1317]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && (!s.b[1317])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1314] && s.b[1315]) {s.store_mul3_lhs(353, 112, 753, 0);s.store_mul_add_rhs(354, 353, 344, 352);}
        s.b[1318] = (s.v[6] > 0.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {s.store_mul_abs_mixed_ia(0, 168, A::mul(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(332)));}
        s.b[1319] = (s.v[0] > (100000000.0 * p[16]));s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (s.b[1318] && s.b[1319]) {s.store_div_from_scalar(355, (-(p[16] + (0.25 / p[16]))), 168);}
        if (s.b[1318] && (!s.b[1319])) {s.store_div_scaled_inputs_mixed_ai(355, A::offset(A::sub_scaled_inputs(A::offset(s.ad_value(0), p[16]), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(0), (-p[16])), 1.0), 0.5), (0.25 / p[16])), -1.0, 168, 1.0);}
        if s.b[1318] {s.store_div(356, 215, 168);}
        if (!s.b[1318]) {s.store_scalar(355, 0.0);s.store_scaled_voltage(356, ctx, nodes, Some(4), None, 0.001);}
        s.b[1604] = (p[11] > 0.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if s.b[1604] {s.copy_ad(1414, 130);s.copy_ad(1415, 131);s.copy_ad(1416, 135);s.copy_ad(1417, 136);s.copy_ad(1418, 140);s.copy_ad(1419, 141);s.copy_ad(1420, 270);s.copy_ad(1421, 212);s.copy_ad(1422, 158);s.store_sub_mixed_ai(1423, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(1414)), s.ad_value(223), 1.0), 230);s.store_add_scaled_product_mixed_iai(1424, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1415), 1.0), 223, 1.0);s.store_sub(1425, 1424, 230);}
        s.b[1605] = (p[2] > 0.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1605]) {s.store_scale(0, 16, p[14]);s.store_div_scaled_offset_numerator_mixed_ia(1426, 242, 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);s.store_ln(1427, 1426);}
        s.b[1606] = (s.v[1427] > 1e-8);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
    ) {
        if ((s.b[1604] && s.b[1605]) && s.b[1606]) {s.store_div_scaled_product_offset_denominator_mixed_iai(1428, 1427, A::offset(s.ad_value(1426), 1.0), 2.0, 1426, (-1.0), 1.0);}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1606])) {s.store_scaled_offset(1428, 1427, 2.0, 2.0);}
        if (s.b[1604] && s.b[1605]) {s.store_div_square_rhs(1429, 249, 241);s.store_div_from_scalar(1430, 1.0, 242);s.store_div_from_scalar(1431, 1.0, 243);s.store_div_from_scalar_add_ad(1458, 1.0, A::offset(s.ad_value(1430), 1.0), s.ad_value(1431));s.store_mul_sub_rhs(1459, 1458, 1423, 1425);s.store_add_scaled_product_indices(1432, 1423, 1.0, 1459, 1430, (-1.0));s.store_add_scaled_product_indices(1433, 1425, 1.0, 1459, 1431, 1.0);s.store_div_from_scalar_offset_input(1338, 1.0, 242, 1.0);s.store_div_from_scalar_offset_input(1339, 1.0, 243, 1.0);s.store_offset_ln_ad(1341, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);s.store_offset_ln_ad(1342, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);}
        s.b[1607] = (((s.v[1341] - s.v[1432]) / 1.5) < 80.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1607]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.6666666666666666, s.ad_value(1432), 0.6666666666666666));}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1607])) {s.store_scaled_sub(1340, 1341, 1432, 0.6666666666666666);}
        if (s.b[1604] && s.b[1605]) {s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 1.5);s.store_mul_add_scaled_product_rhs_indices(1344, 1339, 1345, 1.0, 243, 1425, 1.0);}
        s.b[1608] = (((s.v[1342] - s.v[1344]) / 1.5) < 80.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1608]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.6666666666666666, s.ad_value(1344), 0.6666666666666666));}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1608])) {s.store_scaled_sub(1340, 1342, 1344, 0.6666666666666666);}
        if (s.b[1604] && s.b[1605]) {s.store_sub_scaled_inputs(1, 1342, 1.0, 1340, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 1425);s.store_sub(1390, 2, 3);}
        s.b[1609] = ((((-s.v[262])) as f64).abs() < 80.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1609]) {s.store_exp_neg_input(1391, 262);}
        s.b[1610] = ((-s.v[262]) < (-80.0));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1605]) && (!s.b[1609])) && s.b[1610]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1391, 1.80485e-35, A::neg(A::neg(s.ad_value(262))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1604] && s.b[1605]) && (!s.b[1609])) && (!s.b[1610])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1391, A::neg(s.ad_value(262)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1611] = (((s.v[1390]) as f64).abs() <= s.v[261]);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1611]) {s.store_scaled_square(1388, 260, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 1390, 260, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(1390), 1.0, s.ad_value(1391)), s.ad_value(256), s.ad_value(1388)), 1.0));}
        s.b[1612] = (s.v[1390] < (-s.v[261]));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {s.store_neg(1392, 1390);s.store_scaled_mul(1393, 1392, 260, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(1394, 1393, 10.0, (-6.0), 64.0, 0.5);s.store_sub(1387, 1392, 1394);s.store_add_scaled_square_product_mixed_iia(1395, 1387, 1.0, 257, A::offset(s.ad_value(1394), 1.0), 1.0);s.store_sub_scaled_inputs(1397, 1387, 2.0, 257, 1.0);}
    }
}
