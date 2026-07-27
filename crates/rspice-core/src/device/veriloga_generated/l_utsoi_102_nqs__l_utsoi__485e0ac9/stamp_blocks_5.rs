#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_product_by_product_mixed_aiii(348, A::mul3(s.ad_value(347), s.ad_value(390), s.ad_value(433)), 344, 1.0, 345, 346, 1.0);s.store_mul_scale_offset_indices(704, 224, 330, -1.0, 0.0);s.store_mul_scale_offset_indices(705, 224, 332, -1.0, 0.0);s.store_add_scaled_product_indices(0, 234, 1.0, 163, 224, p[14]);s.store_add(706, 704, 0);s.store_add(707, 705, 0);s.store_scalar(714, 0.0);s.store_scalar(715, 0.0);s.store_scalar(716, 0.0);s.store_scalar(717, 0.0);s.store_div_mixed_ai(708, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);s.store_square(709, 708);s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);let t0: f64 = (1e-5 * s.v[710]);s.store_scalar(711, t0);s.store_div_from_scalar(712, 1.0, 710);s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);s.b[1213] = (((p[3] > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p[4] > 0.0) && (s.v[89] > 0.0)));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });s.b[1214] = (((s.v[704]) as f64).abs() <= s.v[711]);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (s.b[1213] && s.b[1214]) {s.store_mul_scale_offset_indices(714, 712, 704, -1.0, 0.0);}
        s.b[1215] = (s.v[704] < (-s.v[711]));s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_neg(683, 704);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(686, 683, 685, 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1216] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((s.b[1213] && (!s.b[1214])) && s.b[1215]) && s.b[1216]) {s.store_exp(693, 692);}
        s.b[1217] = (s.v[692] < (-80.0));s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
    ) {
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(714, 692, 697);}
        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {s.store_mul_scale_offset_mixed_ia(698, 713, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(699, 704, 712, A::offset(A::mul(s.ad_value(698), s.ad_value(704)), 1.0));}
        s.b[1218] = ((((-s.v[699])) as f64).abs() < 80.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if (((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && s.b[1218]) {s.store_exp_neg_input(691, 699);}
        s.b[1219] = ((-s.v[699]) < (-80.0));s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1218])) && s.b[1219]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(691, 1.80485e-35, A::neg(A::neg(s.ad_value(699))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1218])) && (!s.b[1219])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(691, A::neg(s.ad_value(699)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {s.store_sub_from_scalar(697, 1.0, 691);s.store_add_scaled_inputs_product_mixed_iiia(700, 704, 1.0, 709, 0.5, 708, A::sqrt(A::add_scaled_inputs3(s.ad_value(704), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0));}
        s.b[1220] = ((((-s.v[700])) as f64).abs() < 80.0);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && s.b[1220]) {s.store_exp_neg_input(693, 700);}
        s.b[1221] = ((-s.v[700]) < (-80.0));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1220])) && s.b[1221]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1220])) && (!s.b[1221])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {s.store_add_scaled_inputs3_mixed_iia(694, 704, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(695, 704, 700, 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(714, 700, 701);}
        if (s.b[1213] && (!s.b[1214])) {s.store_neg(714, 714);}
        s.b[1222] = (s.v[159] > 0.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = (((s.v[706]) as f64).abs() <= s.v[711]);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1223]) {s.store_mul_scale_offset_indices(716, 712, 706, -1.0, 0.0);}
        s.b[1224] = (s.v[706] < (-s.v[711]));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_neg(683, 706);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(686, 683, 685, 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
    ) {
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1225] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (((s.b[1222] && (!s.b[1223])) && s.b[1224]) && s.b[1225]) {s.store_exp(693, 692);}
        s.b[1226] = (s.v[692] < (-80.0));s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && s.b[1226]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && (!s.b[1226])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(716, 692, 697);}
        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {s.store_mul_scale_offset_mixed_ia(698, 713, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(699, 706, 712, A::offset(A::mul(s.ad_value(698), s.ad_value(706)), 1.0));}
        s.b[1227] = ((((-s.v[699])) as f64).abs() < 80.0);s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if (((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && s.b[1227]) {s.store_exp_neg_input(691, 699);}
        s.b[1228] = ((-s.v[699]) < (-80.0));s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1227])) && s.b[1228]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(691, 1.80485e-35, A::neg(A::neg(s.ad_value(699))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1227])) && (!s.b[1228])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(691, A::neg(s.ad_value(699)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {s.store_sub_from_scalar(697, 1.0, 691);s.store_add_scaled_inputs_product_mixed_iiia(700, 706, 1.0, 709, 0.5, 708, A::sqrt(A::add_scaled_inputs3(s.ad_value(706), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0));}
        s.b[1229] = ((((-s.v[700])) as f64).abs() < 80.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if (((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && s.b[1229]) {s.store_exp_neg_input(693, 700);}
        s.b[1230] = ((-s.v[700]) < (-80.0));s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1229])) && s.b[1230]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1229])) && (!s.b[1230])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {s.store_add_scaled_inputs3_mixed_iia(694, 706, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(695, 706, 700, 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(716, 700, 701);}
        if (s.b[1222] && (!s.b[1223])) {s.store_neg(716, 716);}
        s.store_div_mixed_ai(708, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);s.store_square(709, 708);s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);let t1: f64 = (1e-5 * s.v[710]);s.store_scalar(711, t1);s.store_div_from_scalar(712, 1.0, 710);s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);s.b[1231] = (((p[3] > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p[4] > 0.0) && (s.v[90] > 0.0)));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });s.b[1232] = (((s.v[705]) as f64).abs() <= s.v[711]);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (s.b[1231] && s.b[1232]) {s.store_mul_scale_offset_indices(715, 712, 705, -1.0, 0.0);}
        s.b[1233] = (s.v[705] < (-s.v[711]));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {s.store_neg(683, 705);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(686, 683, 685, 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1234] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if (((s.b[1231] && (!s.b[1232])) && s.b[1233]) && s.b[1234]) {s.store_exp(693, 692);}
        s.b[1235] = (s.v[692] < (-80.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && s.b[1235]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && (!s.b[1235])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
    ) {
        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(715, 692, 697);}
        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {s.store_mul_scale_offset_mixed_ia(698, 713, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(699, 705, 712, A::offset(A::mul(s.ad_value(698), s.ad_value(705)), 1.0));}
        s.b[1236] = ((((-s.v[699])) as f64).abs() < 80.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if (((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && s.b[1236]) {s.store_exp_neg_input(691, 699);}
        s.b[1237] = ((-s.v[699]) < (-80.0));s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1236])) && s.b[1237]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(691, 1.80485e-35, A::neg(A::neg(s.ad_value(699))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1236])) && (!s.b[1237])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(691, A::neg(s.ad_value(699)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {s.store_sub_from_scalar(697, 1.0, 691);s.store_add_scaled_inputs_product_mixed_iiia(700, 705, 1.0, 709, 0.5, 708, A::sqrt(A::add_scaled_inputs3(s.ad_value(705), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0));}
        s.b[1238] = ((((-s.v[700])) as f64).abs() < 80.0);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if (((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && s.b[1238]) {s.store_exp_neg_input(693, 700);}
        s.b[1239] = ((-s.v[700]) < (-80.0));s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && s.b[1239]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && (!s.b[1239])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {s.store_add_scaled_inputs3_mixed_iia(694, 705, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(695, 705, 700, 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(715, 700, 701);}
        if (s.b[1231] && (!s.b[1232])) {s.store_neg(715, 715);}
        s.b[1240] = (s.v[160] > 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });s.b[1241] = (((s.v[707]) as f64).abs() <= s.v[711]);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if (s.b[1240] && s.b[1241]) {s.store_mul_scale_offset_indices(717, 712, 707, -1.0, 0.0);}
        s.b[1242] = (s.v[707] < (-s.v[711]));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_neg(683, 707);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
    ) {
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_add_scaled_sub_square_product_mixed_ia(686, 683, 685, 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1243] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((s.b[1240] && (!s.b[1241])) && s.b[1242]) && s.b[1243]) {s.store_exp(693, 692);}
        s.b[1244] = (s.v[692] < (-80.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && s.b[1244]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && (!s.b[1244])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(717, 692, 697);}
        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {s.store_mul_scale_offset_mixed_ia(698, 713, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(699, 707, 712, A::offset(A::mul(s.ad_value(698), s.ad_value(707)), 1.0));}
        s.b[1245] = ((((-s.v[699])) as f64).abs() < 80.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if (((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && s.b[1245]) {s.store_exp_neg_input(691, 699);}
        s.b[1246] = ((-s.v[699]) < (-80.0));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1245])) && s.b[1246]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(691, 1.80485e-35, A::neg(A::neg(s.ad_value(699))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1245])) && (!s.b[1246])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(691, A::neg(s.ad_value(699)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {s.store_sub_from_scalar(697, 1.0, 691);s.store_add_scaled_inputs_product_mixed_iiia(700, 707, 1.0, 709, 0.5, 708, A::sqrt(A::add_scaled_inputs3(s.ad_value(707), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0));}
        s.b[1247] = ((((-s.v[700])) as f64).abs() < 80.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if (((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && s.b[1247]) {s.store_exp_neg_input(693, 700);}
        s.b[1248] = ((-s.v[700]) < (-80.0));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
    ) {
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && (!s.b[1248])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {s.store_add_scaled_inputs3_mixed_iia(694, 707, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(695, 707, 700, 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(717, 700, 701);}
        if (s.b[1240] && (!s.b[1241])) {s.store_neg(717, 717);}
        s.store_mul_add_scaled_inputs_rhs_indices(718, 223, 704, -1.0, 714, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(719, 223, 705, -1.0, 715, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(349, 223, 706, -1.0, 716, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(350, 223, 707, -1.0, 717, -1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(733, 0.0);s.store_scalar(734, 0.0);s.store_scalar(351, 0.0);s.store_scalar(352, 0.0);s.store_scalar(353, 0.0);s.store_scalar(753, 0.0);s.store_scalar(754, 0.0);s.b[1249] = (p[3] > 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
    ) {
        s.b[1250] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1250]) {s.store_add(720, 718, 285);s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(718)), 0.0001));}
        s.b[1251] = ((((0.5 * s.v[704])) as f64).abs() < 80.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1251]) {s.store_exp_scaled_input(0, 704, 0.5);}
        s.b[1252] = ((0.5 * s.v[704]) < (-80.0));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(704), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(723, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(724, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(725, 282, 2, 1.0, 281, 3, 1.0);s.store_add_scaled_products_indices(726, 71, 2, 1.0, 69, 3, 1.0);s.store_scaled_mul(727, 73, 3, 1e-6);s.store_mul_div_scaled_inputs_indices(2, 279, 81, (-1.0), 722, 1.0);}
        s.b[1253] = (s.v[724] < 0.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1253]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(722, 722, 0.5, 725, 0.5, 722, 725, 1e-6, (-0.5));}
        if (s.b[1249] && s.b[1250]) {s.store_add_scaled_product_mixed_aii(728, A::offset(s.ad_value(714), 3.0), 1.0, 721, 224, 1.0);}
        s.b[1254] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1254]) {s.store_exp(729, 728);}
        s.b[1255] = (s.v[728] < (-80.0));s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1254])) && s.b[1255]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(729, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1254])) && (!s.b[1255])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(729, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_add_mixed_ai(728, A::add_scaled_product(A::offset(s.ad_value(714), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0), 704);}
        s.b[1256] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1256]) {s.store_exp(730, 728);}
        s.b[1257] = (s.v[728] < (-80.0));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1256])) && s.b[1257]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(730, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1256])) && (!s.b[1257])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(730, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_mul_scale_offset_mixed_ia(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), 1.0, (-1.5));}
        s.b[1258] = (s.v[0] > 0.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1258]) {s.store_offset_mul_offset_rhs_mixed_ia(731, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1259] = (s.v[0] > (-80.0));s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1258])) && s.b[1259]) {s.store_exp(731, 0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1258])) && (!s.b[1259])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(731, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[1260] = (s.v[2] > 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1260]) {s.store_offset_mul_offset_rhs_mixed_ia(732, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
    ) {
        s.b[1261] = (s.v[2] > (-80.0));s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1260])) && s.b[1261]) {s.store_exp(732, 2);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1260])) && (!s.b[1261])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(732, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[1249] && s.b[1250]) {s.store_div_scaled_offset_numerator_mixed_ia(0, 729, 1.0, 1.0, A::offset(s.ad_value(730), 1.0), 1.0);}
        s.b[1262] = (s.v[0] < 1e-80);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1262]) {s.store_scalar(0, 1e-80);}
        if (s.b[1249] && s.b[1250]) {s.store_mul_sub_rhs(2, 85, 332, 86);}
        s.b[1263] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1263]) {s.store_exp(3, 2);}
        s.b[1264] = (s.v[2] < (-80.0));s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1263])) && s.b[1264]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1263])) && (!s.b[1264])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 703, 1.0);}
        s.b[1265] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1265]) {s.store_exp(5, 4);}
        s.b[1266] = (s.v[4] < (-80.0));s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1265])) && s.b[1266]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1265])) && (!s.b[1266])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_sub_ad(733, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(726), s.ad_value(731), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(727), s.ad_value(732), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));}
        s.b[1267] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1267]) {s.store_add(720, 719, 285);s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(719)), 0.0001));}
        s.b[1268] = ((((0.5 * s.v[705])) as f64).abs() < 80.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1268]) {s.store_exp_scaled_input(0, 705, 0.5);}
        s.b[1269] = ((0.5 * s.v[705]) < (-80.0));s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && s.b[1269]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && (!s.b[1269])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(705), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1267]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(723, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(724, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(725, 282, 2, 1.0, 281, 3, 1.0);s.store_add_scaled_products_indices(726, 72, 2, 1.0, 70, 3, 1.0);s.store_scaled_mul(727, 74, 3, 1e-6);s.store_mul_div_scaled_inputs_indices(2, 279, 81, (-1.0), 722, 1.0);}
        s.b[1270] = (s.v[724] < 0.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1270]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(722, 722, 0.5, 725, 0.5, 722, 725, 1e-6, (-0.5));}
        if (s.b[1249] && s.b[1267]) {s.store_add_scaled_product_mixed_aii(728, A::offset(s.ad_value(715), 3.0), 1.0, 721, 224, 1.0);}
        s.b[1271] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
    ) {
        if ((s.b[1249] && s.b[1267]) && s.b[1271]) {s.store_exp(729, 728);}
        s.b[1272] = (s.v[728] < (-80.0));s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1271])) && s.b[1272]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(729, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1271])) && (!s.b[1272])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(729, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1267]) {s.store_add_mixed_ai(728, A::add_scaled_product(A::offset(s.ad_value(715), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0), 705);}
        s.b[1273] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1273]) {s.store_exp(730, 728);}
        s.b[1274] = (s.v[728] < (-80.0));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1273])) && s.b[1274]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(730, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1273])) && (!s.b[1274])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(730, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1267]) {s.store_mul_scale_offset_mixed_ia(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), 1.0, (-1.5));}
        s.b[1275] = (s.v[0] > 0.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1275]) {s.store_offset_mul_offset_rhs_mixed_ia(731, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1276] = (s.v[0] > (-80.0));s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1275])) && s.b[1276]) {s.store_exp(731, 0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1275])) && (!s.b[1276])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(731, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[1277] = (s.v[2] > 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1277]) {s.store_offset_mul_offset_rhs_mixed_ia(732, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1278] = (s.v[2] > (-80.0));s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1277])) && s.b[1278]) {s.store_exp(732, 2);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1277])) && (!s.b[1278])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(732, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[1249] && s.b[1267]) {s.store_div_scaled_offset_numerator_mixed_ia(0, 729, 1.0, 1.0, A::offset(s.ad_value(730), 1.0), 1.0);}
        s.b[1279] = (s.v[0] < 1e-80);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1279]) {s.store_scalar(0, 1e-80);}
        if (s.b[1249] && s.b[1267]) {s.store_mul_sub_rhs(2, 85, 330, 86);}
        s.b[1280] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1280]) {s.store_exp(3, 2);}
        s.b[1281] = (s.v[2] < (-80.0));s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1280])) && s.b[1281]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1280])) && (!s.b[1281])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1267]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 702, 1.0);}
        s.b[1282] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1282]) {s.store_exp(5, 4);}
        s.b[1283] = (s.v[4] < (-80.0));s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1282])) && s.b[1283]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1282])) && (!s.b[1283])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
    ) {
        if (s.b[1249] && s.b[1267]) {s.store_sub_ad(734, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(726), s.ad_value(731), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(727), s.ad_value(732), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));}
        s.b[1284] = (s.v[68] > 0.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1284]) {s.store_mul_scale_offset_indices(735, 386, 436, -1.0, 0.0);}
        s.b[1285] = (((((2.0 * s.v[735]) - s.v[411])) as f64).abs() < 80.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1285]) {s.store_exp_ad(0, A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0));}
        s.b[1286] = (((2.0 * s.v[735]) - s.v[411]) < (-80.0));s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1284]) && (!s.b[1285])) && s.b[1286]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1285])) && (!s.b[1286])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(0, A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1284]) {s.store_mul_sub_mixed_iaa(736, 226, A::offset(s.ad_value(735), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0)));s.store_scaled_add(737, 392, 412, 0.5);s.store_mul(738, 226, 737);s.store_add(720, 738, 284);s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(738)), 0.0001));}
        s.b[1287] = (s.v[79] < 0.0);s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1287]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(722, 722, 0.5, 280, 0.5, 722, 280, 1e-6, (-0.5));}
        if (s.b[1249] && s.b[1284]) {s.store_add(740, 400, 234);s.store_sub(739, 740, 737);s.store_mul_add_scaled_product_rhs_mixed_iai(728, 286, 739, 1.0, A::add_scaled_inputs3(s.ad_value(721), 1.0, s.ad_value(283), (-1.0), s.ad_value(736), -1.0), 227, 1.0);}
        s.b[1288] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1288]) {s.store_exp(729, 728);}
        s.b[1289] = (s.v[728] < (-80.0));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1284]) && (!s.b[1288])) && s.b[1289]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(729, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1288])) && (!s.b[1289])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(729, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1284]) {s.store_mul_ad_affine_product_lhs(728, A::sub(s.ad_value(335), s.ad_value(736)), s.ad_value(227), -1.0, 0.0, 286);}
        s.b[1290] = (((s.v[728]) as f64).abs() < 80.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1290]) {s.store_exp(0, 728);}
        s.b[1291] = (s.v[728] < (-80.0));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1284]) && (!s.b[1290])) && s.b[1291]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(728)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1290])) && (!s.b[1291])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 728, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1284]) {s.store_mul(730, 729, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
    ) {
        if (s.b[1249] && s.b[1284]) {s.store_mul_scale_offset_mixed_ia(0, 278, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(722), 1.0)), 1.0, (-1.5));}
        s.b[1292] = (s.v[0] > 0.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1292]) {s.store_offset_mul_offset_rhs_mixed_ia(731, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);}
        s.b[1293] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1284]) && (!s.b[1292])) && s.b[1293]) {s.store_exp(731, 0);}
        s.b[1294] = (s.v[0] < (-80.0));s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((((s.b[1249] && s.b[1284]) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(731, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1249] && s.b[1284]) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(731, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1284]) {s.store_mul_ad_product_rhs_mixed_ia(741, 68, 731, A::ln(A::div_scaled_offset_numerator(s.ad_value(729), 1.0, 1.0, A::offset(s.ad_value(730), 1.0), 1.0)));}
        s.b[1295] = ((s.v[740] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && s.b[1295]) {s.store_scalar(742, 1.0);s.store_scalar(743, 0.5);}
        if ((s.b[1249] && s.b[1284]) && (!s.b[1295])) {s.store_add_scaled_product_indices(0, 78, 1.0, 79, 722, 2.0);s.store_mul_div_mixed_iia(744, 227, 87, A::mul(s.ad_value(0), s.ad_value(278)));s.store_div(745, 735, 744);s.store_mul3_lhs(746, 744, 434, 401);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(748, 746, 1.0, 746, 1.0, 0.5);s.store_sub_from_scalar_scaled_input(747, 0.5, 748, 3.0);}
        s.b[1296] = (s.v[745] < 0.001);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && s.b[1296]) {s.store_square(749, 745);s.store_offset_mul_ad(742, s.ad_value(749), A::add_scaled_product(A::scale_offset(s.ad_value(746), 0.3333333333333, 0.1666666666667), 1.0, s.ad_value(749), A::scale_offset(s.ad_value(746), 0.2, 0.05), 0.1666666666667), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(743, 742, 0.5, 745, A::mul(s.ad_value(749), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(748), 0.25, 0.4), 1.0, s.ad_value(749), s.ad_value(748), 0.125, 0.0285714285714)), 1.0, (-0.1666666666667));}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_div_from_scalar(750, 1.0, 745);}
        s.b[1297] = (((s.v[745]) as f64).abs() < 80.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if ((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && s.b[1297]) {s.store_exp(751, 745);}
        s.b[1298] = (s.v[745] < (-80.0));s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && s.b[1298]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(751, 1.80485e-35, A::neg(s.ad_value(745)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && (!s.b[1298])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(751, 745, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_div_from_scalar(752, 1.0, 751);s.store_sub(0, 751, 752);s.store_add(3, 751, 752);s.store_add_scaled_products_mixed_aiii(742, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(746), s.ad_value(0)), 750, 0.5, 746, 3, 0.5);s.store_scaled_sub_ad(743, A::add_scaled_product(s.ad_value(742), 1.0, s.ad_value(0), A::sub(s.ad_value(748), A::mul3(s.ad_value(747), s.ad_value(750), s.ad_value(750))), (-1.0)), A::mul3(s.ad_value(747), s.ad_value(3), s.ad_value(750)), 0.5);}
        if (s.b[1249] && s.b[1284]) {s.store_mul(351, 741, 742);s.store_mul(754, 741, 743);s.store_sub(753, 351, 754);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1299] = (s.v[334] < 0.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1299]) {s.store_add(352, 754, 733);s.store_add(353, 753, 734);}
        if (s.b[1249] && (!s.b[1299])) {s.store_add(352, 753, 733);s.store_add(353, 754, 734);}
        s.store_scalar(355, 0.0);s.b[1300] = (((p[4] > 0.0) && (s.v[89] > 0.0)) && (s.v[718] < 0.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if s.b[1300] {s.store_sqrt_offset_ad(755, A::add(A::square(s.ad_value(718)), A::mul3(A::square(s.ad_value(95)), s.ad_value(331), s.ad_value(331))), 1e-6);s.store_div_scaled_inputs_indices(0, 91, -1.0, 755, 1.0);}
        s.b[1301] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if (s.b[1300] && s.b[1301]) {s.store_exp(3, 0);}
        s.b[1302] = (s.v[0] < (-80.0));s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if ((s.b[1300] && (!s.b[1301])) && s.b[1302]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1300] && (!s.b[1301])) && (!s.b[1302])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1300] {s.store_mul(4, 97, 703);}
        s.b[1303] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if (s.b[1300] && s.b[1303]) {s.store_exp(5, 4);}
        s.b[1304] = (s.v[4] < (-80.0));s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((s.b[1300] && (!s.b[1303])) && s.b[1304]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1300] && (!s.b[1303])) && (!s.b[1304])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1300] {s.store_scaled_mul_ad(355, A::mul3(A::mul3_scaled_output(s.ad_value(89), s.ad_value(703), s.ad_value(718), -1.0), s.ad_value(755), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);}
        s.store_scalar(354, 0.0);s.b[1305] = (((p[4] > 0.0) && (s.v[90] > 0.0)) && (s.v[719] < 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if s.b[1305] {s.store_sqrt_offset_ad(756, A::add(A::square(s.ad_value(719)), A::mul3(A::square(s.ad_value(96)), s.ad_value(333), s.ad_value(333))), 1e-6);s.store_div_scaled_inputs_indices(0, 92, -1.0, 756, 1.0);}
        s.b[1306] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1305] && s.b[1306]) {s.store_exp(3, 0);}
        s.b[1307] = (s.v[0] < (-80.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1305] && (!s.b[1306])) && s.b[1307]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1305] && (!s.b[1306])) && (!s.b[1307])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1305] {s.store_mul(4, 98, 702);}
        s.b[1308] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1305] && s.b[1308]) {s.store_exp(5, 4);}
        s.b[1309] = (s.v[4] < (-80.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1305] && (!s.b[1308])) && s.b[1309]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1305] && (!s.b[1308])) && (!s.b[1309])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1305] {s.store_scaled_mul_ad(354, A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(702), s.ad_value(719), -1.0), s.ad_value(756), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);}
        s.store_scalar(356, 0.0);s.b[1310] = (p[12] > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if s.b[1310] {s.store_mul(758, 336, 289);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
    ) {
        if s.b[1310] {s.store_mul_scale_offset_mixed_ia(759, 289, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(760, 758, 759, 0.5);s.store_sub_mixed_ai(761, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub(s.ad_value(335), s.ad_value(100)), s.ad_value(289), 1.0), 234);s.store_sub_mixed_ai(762, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(101), 1.0), s.ad_value(289), 1.0), 234);s.store_div_from_scalar_offset_input(763, 1.0, 105, 1.0);s.store_div_from_scalar_offset_input(764, 1.0, 106, 1.0);s.store_mul(765, 109, 289);s.store_mul_scaled_offset_ad_rhs(0, 765, 2.0, A::sqrt(A::offset(A::div(s.ad_value(759), s.ad_value(765)), 1.0)), (-1.0));s.store_mul(766, 107, 0);s.store_mul(767, 108, 0);s.store_add_scaled_product_mixed_iai(768, 760, 1.0, A::add(s.ad_value(761), s.ad_value(766)), 763, 1.0);s.store_add_scaled_product_mixed_iai(769, 760, 1.0, A::add(s.ad_value(762), s.ad_value(767)), 764, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(770, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(771, A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_div(772, 246, 763);s.store_div(773, 247, 764);s.store_div_from_scalar(774, 1.0, 772);s.store_div_from_scalar(775, 1.0, 773);s.store_div_from_scalar_add_ad(776, 1.0, A::offset(s.ad_value(774), 1.0), s.ad_value(775));s.store_div_square_rhs(777, 290, 390);s.store_mul_sub_rhs(778, 776, 770, 771);}
        s.b[1311] = ((((s.v[771] - s.v[770])) as f64).abs() <= 1e-12);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1311]) {s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(776), s.ad_value(774)), 1.0, 776, 775, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
    ) {
        if (s.b[1310] && s.b[1311]) {s.store_mul_add_scaled_inputs4_rhs_mixed_iaaa(3, 778, 775, 1.0, A::mul3_scaled_output(s.ad_value(774), s.ad_value(776), s.ad_value(774), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(775), s.ad_value(776), s.ad_value(775), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(776)), -1.0);s.store_div_scaled_product_mixed_aii(4, A::sub(s.ad_value(2), s.ad_value(3)), 777, 0.5, 776, 1.0);}
        if (s.b[1310] && (!s.b[1311])) {s.store_exp_mul_scaled_lhs_indices(2, 774, -1.0, 778);s.store_exp_ad(3, A::mul(A::sub(s.ad_value(775), A::div_from_scalar(1.0, s.ad_value(776))), s.ad_value(778)));s.store_div_scaled_product_mixed_iai(4, 777, A::sub(s.ad_value(2), s.ad_value(3)), 1.0, 778, 2.0);}
        if s.b[1310] {s.copy_ad(779, 4);}
        s.b[1312] = (s.v[770] < 80.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1312]) {s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(s.ad_value(770))), 1.0));s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
        s.b[1313] = (s.v[770] < 0.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });s.b[1314] = (s.v[770] > (-80.0));s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((s.b[1310] && (!s.b[1312])) && s.b[1313]) && s.b[1314]) {s.store_exp(784, 770);}
        if (((s.b[1310] && (!s.b[1312])) && s.b[1313]) && (!s.b[1314])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(784, 1.80485e-35, A::neg(s.ad_value(770)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1310] && (!s.b[1312])) && s.b[1313]) {s.store_mul(0, 779, 784);}
        if ((s.b[1310] && (!s.b[1312])) && (!s.b[1313])) {s.store_add_ln_lhs(784, 779, 770);s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
        if s.b[1310] {s.copy_ad(780, 0);}
        s.b[1315] = ((s.v[770] - s.v[411]) < 80.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1315]) {s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(A::sub(s.ad_value(770), s.ad_value(411)))), 1.0));s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
        s.b[1316] = ((s.v[770] - s.v[411]) < 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });s.b[1317] = ((s.v[770] - s.v[411]) > (-80.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && s.b[1317]) {s.store_exp_sub(784, 770, 411);}
        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(784, 1.80485e-35, A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1310] && (!s.b[1315])) && s.b[1316]) {s.store_mul(0, 779, 784);}
        if ((s.b[1310] && (!s.b[1315])) && (!s.b[1316])) {s.store_add_scaled_inputs3_mixed_aii(784, A::ln(s.ad_value(779)), 1.0, 770, 1.0, 411, (-1.0));}
    }
}
