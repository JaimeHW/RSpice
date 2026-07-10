#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && (!s.b[1248])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {s.store_add_scaled_inputs3_mixed_iia(694, 707, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_square_product_mixed_aia(695, A::sub(s.ad_value(707), s.ad_value(700)), 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(717, 700, 701);}
        if (s.b[1240] && (!s.b[1241])) {s.store_neg(717, 717);}
        s.store_mul_add_scaled_inputs_rhs_indices(718, 223, 704, -1.0, 714, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(719, 223, 705, -1.0, 715, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(349, 223, 706, -1.0, 716, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(350, 223, 707, -1.0, 717, -1.0);s.store_scalar(733, 0.0);s.store_scalar(734, 0.0);s.store_scalar(351, 0.0);s.store_scalar(352, 0.0);s.store_scalar(353, 0.0);s.store_scalar(753, 0.0);s.store_scalar(754, 0.0);s.b[1249] = (p.p3 > 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });s.b[1250] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
    ) {
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
        if (s.b[1249] && s.b[1267]) {s.store_add(720, 719, 285);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
    ) {
        if (s.b[1249] && s.b[1267]) {s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(719)), 0.0001));}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
    ) {
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
    ) {
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
        if (s.b[1249] && s.b[1284]) {s.store_mul(730, 729, 0);s.store_mul_scale_offset_mixed_ia(0, 278, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(722), 1.0)), 1.0, (-1.5));}
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
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_div_from_scalar(752, 1.0, 751);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_sub(0, 751, 752);s.store_add(3, 751, 752);s.store_add_scaled_products_mixed_aiii(742, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(746), s.ad_value(0)), 750, 0.5, 746, 3, 0.5);s.store_scaled_sub_ad(743, A::add_scaled_product(s.ad_value(742), 1.0, s.ad_value(0), A::sub(s.ad_value(748), A::mul3(s.ad_value(747), s.ad_value(750), s.ad_value(750))), (-1.0)), A::mul3(s.ad_value(747), s.ad_value(3), s.ad_value(750)), 0.5);}
        if (s.b[1249] && s.b[1284]) {s.store_mul(351, 741, 742);s.store_mul(754, 741, 743);s.store_sub(753, 351, 754);}
        s.b[1299] = (s.v[334] < 0.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1299]) {s.store_add(352, 754, 733);s.store_add(353, 753, 734);}
        if (s.b[1249] && (!s.b[1299])) {s.store_add(352, 753, 733);s.store_add(353, 754, 734);}
        s.store_scalar(355, 0.0);s.b[1300] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[718] < 0.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
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
        s.store_scalar(354, 0.0);s.b[1305] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[719] < 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if s.b[1305] {s.store_sqrt_offset_ad(756, A::add(A::square(s.ad_value(719)), A::mul3(A::square(s.ad_value(96)), s.ad_value(333), s.ad_value(333))), 1e-6);s.store_div_scaled_inputs_indices(0, 92, -1.0, 756, 1.0);}
        s.b[1306] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1305] && s.b[1306]) {s.store_exp(3, 0);}
        s.b[1307] = (s.v[0] < (-80.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1305] && (!s.b[1306])) && s.b[1307]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1305] && (!s.b[1306])) && (!s.b[1307])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1305] {s.store_mul(4, 98, 702);}
        s.b[1308] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1305] && s.b[1308]) {s.store_exp(5, 4);}
        s.b[1309] = (s.v[4] < (-80.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1305] && (!s.b[1308])) && s.b[1309]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1305] && (!s.b[1308])) && (!s.b[1309])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1305] {s.store_scaled_mul_ad(354, A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(702), s.ad_value(719), -1.0), s.ad_value(756), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);}
        s.store_scalar(356, 0.0);s.b[1310] = (p.p12 > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if s.b[1310] {s.store_mul(758, 336, 289);s.store_mul_scale_offset_mixed_ia(759, 289, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(760, 758, 759, 0.5);s.store_sub_mixed_ai(761, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub(s.ad_value(335), s.ad_value(100)), s.ad_value(289), 1.0), 234);s.store_sub_mixed_ai(762, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(101), 1.0), s.ad_value(289), 1.0), 234);s.store_div_from_scalar_offset_input(763, 1.0, 105, 1.0);s.store_div_from_scalar_offset_input(764, 1.0, 106, 1.0);s.store_mul(765, 109, 289);s.store_mul_scaled_offset_ad_rhs(0, 765, 2.0, A::sqrt(A::offset(A::div(s.ad_value(759), s.ad_value(765)), 1.0)), (-1.0));s.store_mul(766, 107, 0);s.store_mul(767, 108, 0);s.store_add_scaled_product_mixed_iai(768, 760, 1.0, A::add(s.ad_value(761), s.ad_value(766)), 763, 1.0);s.store_add_scaled_product_mixed_iai(769, 760, 1.0, A::add(s.ad_value(762), s.ad_value(767)), 764, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(770, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(771, A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_div(772, 246, 763);s.store_div(773, 247, 764);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
    ) {
        if s.b[1310] {s.store_div_from_scalar(774, 1.0, 772);s.store_div_from_scalar(775, 1.0, 773);s.store_div_from_scalar_add_ad(776, 1.0, A::offset(s.ad_value(774), 1.0), s.ad_value(775));s.store_div_square_rhs(777, 290, 390);s.store_mul_sub_rhs(778, 776, 770, 771);}
        s.b[1311] = ((((s.v[771] - s.v[770])) as f64).abs() <= 1e-12);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1311]) {s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(776), s.ad_value(774)), 1.0, 776, 775, (-1.0));s.store_mul_add_scaled_inputs4_rhs_mixed_iaaa(3, 778, 775, 1.0, A::mul3_scaled_output(s.ad_value(774), s.ad_value(776), s.ad_value(774), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(775), s.ad_value(776), s.ad_value(775), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(776)), -1.0);s.store_div_scaled_product_mixed_aii(4, A::sub(s.ad_value(2), s.ad_value(3)), 777, 0.5, 776, 1.0);}
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
        s.b[1316] = ((s.v[770] - s.v[411]) < 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1317] = ((s.v[770] - s.v[411]) > (-80.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && s.b[1317]) {s.store_exp_sub(784, 770, 411);}
        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(784, 1.80485e-35, A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1310] && (!s.b[1315])) && s.b[1316]) {s.store_mul(0, 779, 784);}
        if ((s.b[1310] && (!s.b[1315])) && (!s.b[1316])) {s.store_add_scaled_inputs3_mixed_aii(784, A::ln(s.ad_value(779)), 1.0, 770, 1.0, 411, (-1.0));s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
        if s.b[1310] {s.copy_ad(781, 0);s.store_mul_scale_offset(782, A::sub(s.ad_value(780), s.ad_value(781)), A::add_scaled_inputs(s.ad_value(780), 0.5, s.ad_value(781), 0.5), 1.0, 1.0);s.store_mul_square_lhs(783, 288, 110);s.store_div_scaled_product3_indices(356, 783, 241, 782, 1.0, 422, 1.0);}
        s.store_scalar(357, 0.0);s.store_scalar(358, 0.0);s.b[1318] = (p.p8 != 0.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {s.store_div_scaled_add_product_indices(757, 339, 1.0, 115, 411, (-1.0), 227, 1.0);}
        s.b[1319] = (s.v[757] > 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (s.b[1318] && s.b[1319]) {s.store_div_scaled_value_offset_denominator(3, s.ad_value(113), (-1.0), s.ad_value(757), 1e-30, 1.0);}
        s.b[1320] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((s.b[1318] && s.b[1319]) && s.b[1320]) {s.store_exp(0, 3);}
        s.b[1321] = (s.v[3] < (-80.0));s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && s.b[1321]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && (!s.b[1321])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1318] && s.b[1319]) {s.store_mul3_lhs(357, 112, 757, 0);s.store_mul_add_rhs(358, 357, 348, 356);}
        s.b[1322] = (s.v[6] > 0.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if s.b[1322] {s.store_mul_abs_mixed_ia(0, 168, A::mul(A::add(s.ad_value(348), s.ad_value(356)), s.ad_value(336)));}
        s.b[1323] = (s.v[0] > (100000000.0 * p.p16));s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if (s.b[1322] && s.b[1323]) {s.store_div_from_scalar(359, (-(p.p16 + (0.25 / p.p16))), 168);}
        if (s.b[1322] && (!s.b[1323])) {s.store_div_scaled_inputs_mixed_ai(359, A::offset(A::sub_scaled_inputs(A::offset(s.ad_value(0), p.p16), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(0), (-p.p16)), 1.0), 0.5), (0.25 / p.p16)), -1.0, 168, 1.0);}
        if s.b[1322] {s.store_div(360, 219, 168);}
        if (!s.b[1322]) {s.store_scalar(359, 0.0);s.store_scaled_voltage(360, ctx, nodes, Some(4), None, 0.001);}
        s.b[1608] = (p.p11 > 0.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if s.b[1608] {s.copy_ad(1418, 130);s.copy_ad(1419, 131);s.copy_ad(1420, 135);s.copy_ad(1421, 136);s.copy_ad(1422, 140);s.copy_ad(1423, 141);s.copy_ad(1424, 274);s.copy_ad(1425, 216);s.copy_ad(1426, 158);s.store_sub_mixed_ai(1427, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub(s.ad_value(335), s.ad_value(1418)), s.ad_value(227), 1.0), 234);s.store_add_scaled_product_mixed_iai(1428, 341, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1419), 1.0), 227, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1608] {s.store_sub(1429, 1428, 234);}
        s.b[1609] = (p.p2 > 0.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1609]) {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(1430, 246, 1.0, 1.0, A::offset(s.ad_value(247), 1.0), 1.0);s.store_ln(1431, 1430);}
        s.b[1610] = (s.v[1431] > 1e-8);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1609]) && s.b[1610]) {s.store_div_scaled_product_offset_denominator_mixed_iai(1432, 1431, A::offset(s.ad_value(1430), 1.0), 2.0, 1430, (-1.0), 1.0);}
        if ((s.b[1608] && s.b[1609]) && (!s.b[1610])) {s.store_scaled_offset(1432, 1431, 2.0, 2.0);}
        if (s.b[1608] && s.b[1609]) {s.store_div_square_rhs(1433, 253, 245);s.store_div_from_scalar(1434, 1.0, 246);s.store_div_from_scalar(1435, 1.0, 247);s.store_div_from_scalar_add_ad(1462, 1.0, A::offset(s.ad_value(1434), 1.0), s.ad_value(1435));s.store_mul_sub_rhs(1463, 1462, 1427, 1429);s.store_add_scaled_product_indices(1436, 1427, 1.0, 1463, 1434, (-1.0));s.store_add_scaled_product_indices(1437, 1429, 1.0, 1463, 1435, 1.0);s.store_div_from_scalar_offset_input(1342, 1.0, 246, 1.0);s.store_div_from_scalar_offset_input(1343, 1.0, 247, 1.0);s.store_offset_ln_ad(1345, A::div_scaled_product(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 1.5);s.store_offset_ln_ad(1346, A::div_scaled_product(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(246), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 1.5);}
        s.b[1611] = (((s.v[1345] - s.v[1436]) / 1.5) < 80.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1609]) && s.b[1611]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.6666666666666666, s.ad_value(1436), 0.6666666666666666));}
        if ((s.b[1608] && s.b[1609]) && (!s.b[1611])) {s.store_scaled_sub(1344, 1345, 1436, 0.6666666666666666);}
        if (s.b[1608] && s.b[1609]) {s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 1.5);s.store_mul_add_scaled_product_rhs_indices(1348, 1343, 1349, 1.0, 247, 1429, 1.0);}
        s.b[1612] = (((s.v[1346] - s.v[1348]) / 1.5) < 80.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1609]) && s.b[1612]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.6666666666666666, s.ad_value(1348), 0.6666666666666666));}
        if ((s.b[1608] && s.b[1609]) && (!s.b[1612])) {s.store_scaled_sub(1344, 1346, 1348, 0.6666666666666666);}
        if (s.b[1608] && s.b[1609]) {s.store_sub_scaled_inputs(1, 1346, 1.0, 1344, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 1429);s.store_sub(1394, 2, 3);}
        s.b[1613] = ((((-s.v[266])) as f64).abs() < 80.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1609]) && s.b[1613]) {s.store_exp_neg_input(1395, 266);}
        s.b[1614] = ((-s.v[266]) < (-80.0));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1609]) && (!s.b[1613])) && s.b[1614]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1395, 1.80485e-35, A::neg(A::neg(s.ad_value(266))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1608] && s.b[1609]) && (!s.b[1613])) && (!s.b[1614])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1395, A::neg(s.ad_value(266)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1615] = (((s.v[1394]) as f64).abs() <= s.v[265]);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1609]) && s.b[1615]) {s.store_scaled_square(1392, 264, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 1394, 264, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(1394), 1.0, s.ad_value(1395)), s.ad_value(260), s.ad_value(1392)), 1.0));}
        s.b[1616] = (s.v[1394] < (-s.v[265]));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {s.store_neg(1396, 1394);s.store_scaled_mul(1397, 1396, 264, 1.25);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {s.store_scaled_sub_offset_sqrt_square_offset(1398, 1397, 10.0, (-6.0), 64.0, 0.5);s.store_sub(1391, 1396, 1398);s.store_add_scaled_square_product_mixed_iia(1399, 1391, 1.0, 261, A::offset(s.ad_value(1398), 1.0), 1.0);s.store_sub_scaled_inputs(1401, 1391, 2.0, 261, 1.0);s.store_sub_ln_mul_lhs(1402, 1399, 262, 1398);s.store_add(1389, 1399, 1401);s.store_add_scaled_square_product_mixed_iia(1390, 1389, 1.0, 1402, A::add_scaled_product(s.ad_value(1399), (-1.0), s.ad_value(1401), s.ad_value(1401), 0.5), 1.0);s.store_add_mixed_ia(1403, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::sub_scaled_inputs(A::square(s.ad_value(1401)), 0.3333333333333, s.ad_value(1399), 1.0))), 1.0));}
        s.b[1617] = (s.v[1403] < 80.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && s.b[1617]) {s.store_exp(1404, 1403);}
        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && (!s.b[1617])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(1404, 1403, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {s.store_div_from_scalar(1405, 1.0, 1404);s.store_div_from_scalar_offset_square(1391, 1.0, 1403, 2.0);s.store_mul_square_lhs(1406, 1403, 1391);s.store_mul3_affine_lhs(1407, 1403, 1391, 4.0, 0.0, 1391);s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), 1391, 1391);s.store_sub(1391, 1396, 1403);s.store_mul(1392, 1395, 1405);s.store_add_scaled_product_mixed_iia(1409, 1391, 2.0, 261, A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1392), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(1395), 1.0, s.ad_value(1407)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(1410, 1391, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1403), (-1.0), s.ad_value(1392), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::sub(A::offset(s.ad_value(1403), (-1.0)), s.ad_value(1406)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(1391, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(1404), 1.0, s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(1391, 1409, 1.0, 1410, 1391, (-2.0));s.store_sub_scaled_inputs_mixed_ia(4, 1403, -1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0);}
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {s.store_div_from_scalar_offset_scaled_input(1411, 1.0, 260, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(1412, 1411, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(1411)), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {s.store_mul_ad_product_rhs_mixed_ia(1413, 1394, 264, A::offset(A::mul(s.ad_value(1412), s.ad_value(1394)), 1.0));}
        s.b[1618] = ((-s.v[1413]) > (-80.0));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1618]) {s.store_exp_neg_input(1391, 1413);}
        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1618])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1391, 1.80485e-35, A::neg(A::neg(s.ad_value(1413))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {s.store_sub_from_scalar(1414, 1.0, 1391);s.store_add_scaled_inputs_product_mixed_iiia(1415, 1394, 1.0, 261, 0.5, 260, A::sqrt(A::add_scaled_inputs3(s.ad_value(1394), 1.0, s.ad_value(261), 0.25, s.ad_value(1414), -1.0)), (-1.0));s.store_offset(1416, 266, 3.0);s.store_sub_ad(1398, A::add_scaled_inputs3(s.ad_value(1415), 0.5, s.ad_value(1416), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1415), s.ad_value(1416)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(1416), 0.5, A::sqrt_square_offset(s.ad_value(1416), 5.0), 0.5));s.store_sub(1391, 1394, 1398);s.store_exp_neg_input(1392, 1398);s.store_div_from_scalar_offset_square(1393, 1.0, 1398, 2.0);s.store_mul_square_lhs(1406, 1398, 1393);s.store_mul3_affine_lhs(1407, 1398, 1393, 4.0, 0.0, 1393);s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1393), 8.0, s.ad_value(1406), 12.0), 1393, 1393);s.store_max_from_scalar_ad(1399, 1e-40, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(1392), s.ad_value(1398)), (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1398), 1.0), s.ad_value(1406)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(1400, 1.0, 261, A::add_scaled_product(s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(1401, 1391, 2.0, 261, A::add_scaled_sub_value_product(1.0, s.ad_value(1392), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(1402, 266, 1.0, 1398, (-1.0), A::ln(A::div(s.ad_value(1399), s.ad_value(261))), 1.0);s.store_add(1389, 1399, 1401);s.store_add_scaled_square_product_mixed_iia(1390, 1389, 1.0, 1402, A::add_scaled_products(s.ad_value(1401), s.ad_value(1401), 0.5, s.ad_value(1399), s.ad_value(1400), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {s.store_add_mixed_ia(1417, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::add_scaled_square_product(s.ad_value(1401), 0.3333333333333, s.ad_value(1399), s.ad_value(1400), (-1.0)))), 1.0));}
        s.b[1619] = (s.v[1417] < 80.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1619]) {s.store_exp(1404, 1417);s.store_div_from_scalar(1405, 1.0, 1404);s.store_mul(1404, 1395, 1404);}
        s.b[1620] = (s.v[1417] > (s.v[266] - 80.0));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && s.b[1620]) {s.store_exp_sub(1404, 1417, 266);s.store_div(1405, 1395, 1404);}
        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && (!s.b[1620])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1404, 1.80485e-35, A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(1405, 1.80485e-35, 1417, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {s.store_div_from_scalar_offset_square(1391, 1.0, 1417, 2.0);s.store_mul_square_lhs(1406, 1417, 1391);s.store_mul3_affine_lhs(1407, 1417, 1391, 4.0, 0.0, 1391);s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), 1391, 1391);s.store_sub(1391, 1394, 1417);s.store_add_scaled_product_mixed_iia(1409, 1391, 2.0, 261, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(1405)), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(1410, 1391, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1405), 1.0, s.ad_value(1417), 1.0, s.ad_value(1404), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1417), 1.0), s.ad_value(1406)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(1391, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(1405), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(1391, 1409, 1.0, 1410, 1391, (-2.0));s.store_add_scaled_inputs_mixed_ia(4, 1417, 1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0);}
        if (s.b[1608] && s.b[1609]) {s.store_mul_add_rhs(1438, 0, 4, 3);}
        if (s.b[1608] && (!s.b[1609])) {s.copy_ad(1438, 1429);}
        if s.b[1608] {s.store_mul_sub_rhs(0, 248, 1427, 1438);}
        s.b[1621] = (p.p13 > 0.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1621]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1439, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1440, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && s.b[1621]) {s.store_mul_mixed_ia(2, 258, A::exp_scaled_input(A::ln(s.ad_value(1439)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 258, A::exp_scaled_input(A::ln(s.ad_value(1440)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div(1447, 245, 4);s.store_offset_mul(1441, 246, 2, 1.0);s.store_offset_mul(1442, 247, 3, 1.0);s.store_div_scaled_product_indices(1443, 246, 4, 1.0, 1441, 1.0);s.store_div_scaled_product_indices(1444, 247, 4, 1.0, 1442, 1.0);s.store_div_from_scalar_add_ad(1445, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1443)), 1.0), A::div_from_scalar(1.0, s.ad_value(1444)));s.store_offset_mul(1441, 1443, 2, 1.0);s.store_offset_mul(1442, 1444, 3, 1.0);}
        if (s.b[1608] && (!s.b[1621])) {s.copy_ad(1447, 245);s.copy_ad(1443, 246);s.copy_ad(1444, 247);s.copy_ad(1445, 248);s.store_scalar(1441, 1.0);s.store_scalar(1442, 1.0);}
        if s.b[1608] {s.store_mul_sub_rhs(1446, 1445, 1427, 1438);}
        s.b[1622] = (s.v[1446] > 0.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });s.b[1623] = ((-s.v[1446]) < 80.0);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1622]) && s.b[1623]) {s.store_ln_one_plus_exp_neg_input(0, 1446);}
        if ((s.b[1608] && s.b[1622]) && (!s.b[1623])) {s.store_neg(0, 1446);}
        if (s.b[1608] && s.b[1622]) {s.store_add_scaled_inputs3_offset_mixed_iai(1448, 1427, 1.0, A::div(s.ad_value(1446), s.ad_value(1443)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1624] = (s.v[1446] < 80.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1622])) && s.b[1624]) {s.store_ln_one_plus_exp(0, 1446);}
        if ((s.b[1608] && (!s.b[1622])) && (!s.b[1624])) {s.copy_ad(0, 1446);}
        if (s.b[1608] && (!s.b[1622])) {s.store_add_scaled_inputs3_offset_mixed_iai(1448, 1438, 1.0, A::div(s.ad_value(1446), s.ad_value(1444)), 1.0, 0, 1.0, (-0.6931471805599));}
        if s.b[1608] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1449, 1448, 0.5, 254, 0.5, 1448, 254, 4.0, (-0.5));s.store_offset_sqrt_ad(1450, A::offset(A::div_scaled_inputs2(s.ad_value(254), 2.0, s.ad_value(1449), (-2.0), s.ad_value(255), 1.0), 1.0), (-1.0));s.store_add_scaled_product_indices(1451, 1449, 1.0, 255, 1450, 1.0);s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1428)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_div_from_scalar_offset_product(1452, 1.0, 1420, 0, 1.0);s.store_div_from_scalar_offset_product(1453, 1.0, 1421, 0, 1.0);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(329), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1450)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1428)), 1.0, 1.0);s.store_mul(1454, 1422, 0);s.store_mul(1455, 1423, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_add_mixed_ai(1456, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1427), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1454), 1.0), s.ad_value(1452), 1.0), 341);s.store_add_mixed_ai(1457, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1438), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1455), 1.0), s.ad_value(1453), 1.0), 341);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(1458, A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(1459, A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_div(1460, 1443, 1452);s.store_div(1461, 1444, 1453);s.store_div_from_scalar(1434, 1.0, 1460);s.store_div_from_scalar(1435, 1.0, 1461);s.store_div_from_scalar_add_ad(1462, 1.0, A::offset(s.ad_value(1434), 1.0), s.ad_value(1435));s.store_div_square_rhs(1433, 253, 1447);s.store_div_scaled_offset_numerator_mixed_ia(1430, 1460, 1.0, 1.0, A::offset(s.ad_value(1461), 1.0), 1.0);s.store_ln(1431, 1430);}
        s.b[1625] = (s.v[1431] > 1e-8);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1625]) {s.store_div_scaled_product_offset_denominator_mixed_iai(1432, 1431, A::offset(s.ad_value(1430), 1.0), 2.0, 1430, (-1.0), 1.0);}
        if (s.b[1608] && (!s.b[1625])) {s.store_scaled_offset(1432, 1431, 2.0, 2.0);}
        if s.b[1608] {s.store_mul_sub_rhs(1463, 1462, 1458, 1459);s.store_square(1464, 1463);s.store_add_scaled_product_indices(1436, 1458, 1.0, 1463, 1434, (-1.0));s.store_add_scaled_product_indices(1437, 1459, 1.0, 1463, 1435, 1.0);s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);s.store_offset_ln_ad(1345, A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);s.store_offset_ln_ad(1346, A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);}
        s.b[1626] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && s.b[1626]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1626])) {s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);}
        s.b[1627] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1627]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1627])) {s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);s.store_mul_add_scaled_product_rhs_indices(1347, 1342, 1350, 1.0, 1460, 1458, 1.0);s.store_mul_add_scaled_product_rhs_indices(1348, 1343, 1349, 1.0, 1461, 1459, 1.0);}
        s.b[1628] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1628]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1628])) {s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);}
        s.b[1629] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1629]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1629])) {s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);s.store_sub(1465, 1458, 1349);s.store_sub(1469, 1459, 1350);s.store_scalar(1356, 0.0);s.store_scalar(1359, 0.0);s.store_mul(1351, 1460, 1465);}
        s.b[1630] = ((s.v[1458] - s.v[1465]) < 80.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1630]) {s.store_exp_sub(1342, 1458, 1465);}
        if (s.b[1608] && (!s.b[1630])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1631] = (s.v[1353] < (-0.005));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1631]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1632] = (s.v[1353] > 0.005);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1631])) && s.b[1632]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));}
    }
}
