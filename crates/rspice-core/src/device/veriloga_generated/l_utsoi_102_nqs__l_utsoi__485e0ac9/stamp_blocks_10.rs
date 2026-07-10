#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1134]) {s.store_add_scaled_inputs3_offset_mixed_aii(950, A::ln(A::div(s.ad_value(884), A::mul_sub_from_scalar_rhs(s.ad_value(917), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 887, 0.5, 888, 0.5, (-0.6931471805599));s.store_div_from_scalar_ad(953, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(913), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(913), 12.0, A::mul(s.ad_value(911), s.ad_value(912)), 1.0), 1.0, A::mul3(s.ad_value(913), A::sub(s.ad_value(885), s.ad_value(886)), s.ad_value(914)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(913), 0.25), s.ad_value(915), 0.3333333333333), 1.0, 4.0));}
        s.store_div_from_scalar(954, 1.0, 953);s.b[1136] = (s.v[917] > 1e-6);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });
        if s.b[1136] {s.store_div_scaled_value_offset_denominator(955, s.ad_value(931), 100.0, s.ad_value(931), 100.0, 1.0);}
        s.b[1137] = (s.v[61] < 0.0);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (s.b[1136] && s.b[1137]) {s.store_div_from_scalar_sub_from_scalar_ad(956, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(955)));}
        if (s.b[1136] && (!s.b[1137])) {s.store_offset_mul(956, 61, 955, 1.0);}
        if s.b[1136] {s.store_div_scaled_value_offset_denominator(957, s.ad_value(932), 100.0, s.ad_value(932), 100.0, 1.0);}
        s.b[1138] = (s.v[62] < 0.0);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if (s.b[1136] && s.b[1138]) {s.store_div_from_scalar_sub_from_scalar_ad(958, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(957)));}
        if (s.b[1136] && (!s.b[1138])) {s.store_offset_mul(958, 62, 957, 1.0);}
        if s.b[1136] {s.store_sub_ad(959, A::div_scaled_product_by_product(s.ad_value(929), s.ad_value(928), 1.0, s.ad_value(926), s.ad_value(927), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(922), s.ad_value(926)), 1.0, A::div(s.ad_value(923), s.ad_value(927)), 1.0, s.ad_value(917), 1.0));s.store_div_scaled_product_offset_denominator_indices(960, 959, 917, 1.0, 959, 1.0, 1.0);s.store_sub(2, 953, 960);s.store_div_scaled_add_product_indices(961, 917, 1.0, 953, 950, 1.0, 2, 1.0);s.store_scaled_add_mixed_ia(961, 961, A::sqrt_square_offset(s.ad_value(961), 1e-6), 0.5);s.store_scaled_mul_ad(962, A::div(s.ad_value(875), s.ad_value(947)), A::add(s.ad_value(956), s.ad_value(958)), 0.5);s.store_sub_from_scalar_div_indices(963, 1.0, 917, 960);s.store_offset(964, 950, 1.0);s.store_mul_sub_mixed_iai(965, 961, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(960), 2.0, s.ad_value(917), 1.0), s.ad_value(954)), (-2.0)), 950);}
        s.b[1139] = (s.v[962] > 1e-14);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (s.b[1136] && s.b[1139]) {s.store_div_from_scalar_square_ad(966, 2.0, s.ad_value(962));s.store_mul(967, 966, 963);s.store_add(968, 966, 965);s.store_mul(969, 966, 964);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1136] && s.b[1139]) {s.store_sqrt_offset_ad(970, A::add(A::square(s.ad_value(967)), A::mul3_scaled_output(s.ad_value(966), s.ad_value(966), s.ad_value(966), 0.148148148148)), 1e-20);s.store_sqrt_offset_ad(971, A::add(A::square(s.ad_value(969)), A::mul3_scaled_output(s.ad_value(968), s.ad_value(968), s.ad_value(968), 0.148148148148)), 1e-20);s.store_sub_ad(972, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(970), s.ad_value(967)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(970), s.ad_value(967)), 0.5), 0.3333333333333));s.store_sub_ad(973, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(971), s.ad_value(969)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(971), s.ad_value(969)), 0.5), 0.3333333333333));}
        if (s.b[1136] && (!s.b[1139])) {s.copy_ad(972, 963);s.copy_ad(973, 964);}
        if s.b[1136] {s.store_square(4, 2);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(974, 972, (0.94 * 0.5), 973, (0.94 * 0.5), A::add_scaled_inputs(A::square(A::sub(s.ad_value(972), s.ad_value(973))), 1.0, s.ad_value(4), 10.0), (0.94 * 0.5));s.store_add_scaled_product_indices(975, 917, 1.0, 960, 974, 1.0);s.store_mul_sub_rhs(976, 953, 974, 950);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(977, 975, 0.5, 976, 0.5, A::add_scaled_inputs(A::square(A::sub(s.ad_value(975), s.ad_value(976))), 1.0, s.ad_value(4), 36.0), 0.5);}
        if (!s.b[1136]) {s.copy_ad(960, 953);s.store_scaled_offset(974, 950, 1.0, 0.94);s.store_add_scaled_product_mixed_iia(977, 917, 0.5, 953, A::sub_scaled_inputs(s.ad_value(974), 1.0, s.ad_value(950), 0.5), 1.0);}
        s.b[1140] = ((s.v[977] - 0.5) < 80.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(977), (-0.5)));}
        if (!s.b[1140]) {s.store_offset(2, 977, (-0.5));}
        s.store_offset(3, 2, 0.5);s.store_add_mixed_ia(4, 974, A::ln(A::div(s.ad_value(917), s.ad_value(3))));s.b[1141] = ((s.v[4] - 6.0) < 80.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));}
        if (!s.b[1141]) {s.store_offset(2, 4, (-6.0));}
        s.store_offset(4, 2, 6.0);s.b[1142] = ((s.v[225] - s.v[4]) < 80.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(225), s.ad_value(4)));}
        if (!s.b[1142]) {s.store_sub(2, 225, 4);}
        s.store_sub(978, 225, 2);s.store_div(2, 339, 978);s.store_square(3, 2);s.store_square(4, 3);s.store_square(5, 4);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(876), s.ad_value(4)), 1.0)), 2.666666666667);s.store_mul_mixed_ia(979, 339, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));s.store_div_from_scalar_offset_input(793, 1.0, 911, 1.0);s.store_div_from_scalar_offset_input(794, 1.0, 912, 1.0);s.store_offset_add_ad(796, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(911), 1.0, s.ad_value(912), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0)), s.ad_value(979), 3.0);s.store_offset_add_ad(797, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(912), 1.0, s.ad_value(911), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0)), s.ad_value(979), 3.0);s.b[1143] = (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(887), 0.3333333333333));}
        if (!s.b[1143]) {s.store_scaled_sub(795, 796, 887, 0.3333333333333);}
        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);s.b[1144] = (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0);s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(888), 0.3333333333333));}
        if (!s.b[1144]) {s.store_scaled_sub(795, 797, 888, 0.3333333333333);}
        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);s.store_mul_add_scaled_product_rhs_indices(798, 793, 801, 1.0, 911, 909, 1.0);s.store_mul_add_scaled_product_rhs_indices(799, 794, 800, 1.0, 912, 910, 1.0);s.b[1145] = (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(798), 0.3333333333333));}
        if (!s.b[1145]) {s.store_scaled_sub(795, 796, 798, 0.3333333333333);}
        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);s.b[1146] = (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(799), 0.3333333333333));}
        if (!s.b[1146]) {s.store_scaled_sub(795, 797, 799, 0.3333333333333);}
        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);s.store_sub(980, 909, 800);s.store_sub(981, 910, 801);s.store_scalar(807, 0.0);s.store_scalar(810, 0.0);s.store_mul(802, 911, 980);s.b[1147] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1147]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1148] = (s.v[804] < (-0.005));s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_sqrt_abs_ad(807, s.ad_value(804));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1148] {s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1149] = (s.v[804] > 0.005);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if ((!s.b[1148]) && s.b[1149]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1148]) && (!s.b[1149])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
    ) {
        s.b[1150] = (s.v[804] > 0.005);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if s.b[1150] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1151] = (s.v[804] < (-0.005));s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if ((!s.b[1150]) && s.b[1151]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1150]) && (!s.b[1151])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1152] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1152]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1153] = (s.v[816] > 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1153]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
    ) {
        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.store_mul(802, 911, 980);s.store_mul(832, 912, 981);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.store_sub_square_lhs(837, 802, 804);s.b[1154] = (s.v[837] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 979, 1.0, 909, -1.0, 980, 1.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);s.store_add_scaled_inputs3_indices(838, 909, 1.0, 980, (-1.0), 796, -1.0);}
        s.b[1155] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (s.b[1154] && s.b[1155]) {s.store_sub_div_rhs_indices(980, 980, 828, 829);}
        s.store_mul(802, 911, 980);s.store_mul(832, 912, 981);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.b[1156] = (s.v[804] < (-0.005));s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        s.b[1157] = (s.v[804] > 0.005);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if ((!s.b[1156]) && s.b[1157]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        if ((!s.b[1156]) && (!s.b[1157])) {s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(809, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(804, 804, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(825), s.ad_value(808), 1.0, s.ad_value(802), s.ad_value(832), 1.0), 1.0, s.ad_value(804), 1.0, A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0), 1.0));s.store_sub_square_lhs(837, 802, 804);s.b[1158] = (s.v[837] > 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1158] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 979, 1.0, 909, -1.0, 980, 1.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);s.store_add_scaled_inputs3_indices(838, 909, 1.0, 980, (-1.0), 796, -1.0);}
        s.b[1159] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if (s.b[1158] && s.b[1159]) {s.store_sub_div_rhs_indices(980, 980, 828, 829);}
        s.store_mul(802, 911, 980);s.b[1160] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1160]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1161] = (s.v[804] < (-0.005));s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1162] = (s.v[804] > 0.005);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if ((!s.b[1161]) && s.b[1162]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1161]) && (!s.b[1162])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1163] = (s.v[804] > 0.005);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1164] = (s.v[804] < (-0.005));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((!s.b[1163]) && s.b[1164]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1163]) && (!s.b[1164])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1165] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1165]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1166] = (s.v[816] > 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1166]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.store_mul(802, 911, 980);s.b[1167] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1167]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1168] = (s.v[804] < (-0.005));s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1169] = (s.v[804] > 0.005);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((!s.b[1168]) && s.b[1169]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1168]) && s.b[1169]) {s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1168]) && (!s.b[1169])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1170] = (s.v[804] > 0.005);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1171] = (s.v[804] < (-0.005));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((!s.b[1170]) && s.b[1171]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1170]) && (!s.b[1171])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1172] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if s.b[1172] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1172]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1172]) {s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1173] = (s.v[816] > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if s.b[1173] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1173]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.b[1174] = (p.p10 == 1.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });s.b[1175] = (((s.v[831]) as f64).abs() > 0.01);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1174] && s.b[1175]) {s.store_mul(802, 911, 980);}
        s.b[1176] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1176]) {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if ((s.b[1174] && s.b[1175]) && (!s.b[1176])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1174] && s.b[1175]) {s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);}
        s.b[1177] = (s.v[804] < (-0.005));s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1177]) {s.store_sqrt_abs_ad(807, s.ad_value(804));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1174] && s.b[1175]) && s.b[1177]) {s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1178] = (s.v[804] > 0.005);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && s.b[1178]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && (!s.b[1178])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        s.b[1179] = (s.v[804] > 0.005);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1179]) {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1180] = (s.v[804] < (-0.005));s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (((s.b[1174] && s.b[1175]) && (!s.b[1179])) && s.b[1180]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if (((s.b[1174] && s.b[1175]) && (!s.b[1179])) && (!s.b[1180])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1181] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1181]) {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if ((s.b[1174] && s.b[1175]) && (!s.b[1181])) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1182] = (s.v[816] > 0.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1182]) {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if ((s.b[1174] && s.b[1175]) && (!s.b[1182])) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        if (s.b[1174] && s.b[1175]) {s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1174] && s.b[1175]) {s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);}
        s.store_mul(983, 911, 980);s.b[1183] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if s.b[1183] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1183]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(986, 884, 793);s.store_sub_square_lhs(985, 983, 986);s.b[1184] = (s.v[986] <= 0.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if s.b[1184] {s.store_scalar(982, 1e-80);s.store_sub(984, 982, 983);s.store_div(981, 984, 912);}
        s.b[1185] = (s.v[985] < (-0.005));s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if ((!s.b[1184]) && s.b[1185]) {s.store_sqrt_abs_ad(807, s.ad_value(985));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));}
        s.b[1186] = (s.v[985] > 0.005);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((!s.b[1184]) && (!s.b[1185])) && s.b[1186]) {s.store_sqrt_abs_ad(807, s.ad_value(985));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);}
        if (((!s.b[1184]) && (!s.b[1185])) && (!s.b[1186])) {s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1187] = (((1.01 * s.v[983]) + s.v[808]) > 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if ((!s.b[1184]) && s.b[1187]) {s.store_add(793, 983, 808);}
        s.b[1188] = ((s.v[986] * s.v[983]) < (((0.9 * s.v[983]) * s.v[983]) * s.v[793]));s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (((!s.b[1184]) && s.b[1187]) && s.b[1188]) {s.store_offset_div(982, 986, 793, 1e-80);s.store_sub(984, 982, 983);s.store_div(981, 984, 912);}
        s.b[1189] = (s.v[985] > 0.005);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if ((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && s.b[1189]) {s.store_sub_mixed_ai(794, A::ln(A::div_scaled_inputs(s.ad_value(985), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0)), 807);}
        s.b[1190] = (s.v[985] < (-0.005));s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && s.b[1190]) {s.store_sin_scaled_input(795, 807, 0.5);s.store_ln_div_scaled_input_square_denominator(794, 985, -1.0, 795, 1.0);}
        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && (!s.b[1190])) {s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(981, 910, 1.0, 909, (-1.0), 980, 1.0, A::ln(s.ad_value(793)), 2.0, 794);s.store_mul(984, 912, 981);s.store_add(982, 983, 984);}
        s.b[1191] = (s.v[985] > 0.005);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });s.b[1192] = ((((s.v[980] + s.v[979]) - s.v[909]) - s.v[807]) < 80.0);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) && s.b[1192]) {s.store_exp_ad(795, A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0));}
        if ((((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) && (!s.b[1192])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(795, A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) {s.store_div(794, 795, 884);s.store_div_scaled_product_mixed_iia(793, 985, 794, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);}
        s.b[1193] = (s.v[985] < (-0.005));s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if ((((!s.b[1184]) && (!s.b[1187])) && (!s.b[1191])) && s.b[1193]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_value_by_product_mixed_iai(793, 985, -1.0, A::square(s.ad_value(794)), 986, 1.0);}
        if ((((!s.b[1184]) && (!s.b[1187])) && (!s.b[1191])) && (!s.b[1193])) {s.store_div_mixed_ai(793, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0396825396825397), 0.05), 0.3333333333333)), 986);}
        if ((!s.b[1184]) && (!s.b[1187])) {s.store_offset_div_scaled_inputs2_mixed_iia(982, 983, 1.0, 808, (-1.0), A::sub_from_scalar(1.0, s.ad_value(793)), 1.0, 1e-80);s.store_sub(984, 982, 983);s.store_div(981, 984, 912);}
        s.b[1194] = (((s.v[910] - s.v[981]) - s.v[979]) < 80.0);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1194]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(987, 884, 793);s.store_scalar(990, 0.0);s.store_scalar(991, 0.0);s.store_scalar(988, 0.0);s.store_scalar(989, 0.0);s.store_scalar(992, 0.0);s.store_scalar(993, 0.0);s.b[1195] = (s.v[917] > 1e-6);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.store_mul(988, 986, 885);s.store_mul(989, 987, 886);s.store_add_scaled_inputs(990, 988, 1.0, 983, 2.0);s.store_add_scaled_inputs(991, 989, 1.0, 984, 2.0);s.store_add_scaled_inputs3_indices(992, 982, 2.0, 988, 1.0, 989, 1.0);}
        s.b[1196] = (((s.v[985]) as f64).abs() > 0.005);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1195] && s.b[1196]) {s.store_add_scaled_products3_mixed_iiaiai(2, 990, 991, 1.0, A::offset(s.ad_value(980), 2.0), 991, 2.0, A::offset(s.ad_value(981), 2.0), 990, 2.0);s.store_div_scaled_product_by_product_indices(993, 985, 992, (-4.0), 982, 2, 1.0);}
        if (s.b[1195] && (!s.b[1196])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 985, 1.0, 985, 1.0, 985, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 990, 986, 1.0, 991, 987, 1.0, A::mul3(s.ad_value(990), s.ad_value(991), s.ad_value(982)), A::offset(A::mul(s.ad_value(982), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(993, 986, 987, 992, 1.0, 982, 3, 1.0);}
        s.store_add_mixed_ia(994, 979, A::ln(s.ad_value(982)));s.store_scaled_add(995, 917, 982, 0.5);s.store_sub(996, 994, 930);s.store_scalar(999, 1.0);s.b[1197] = (p.p9 > 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1197] {s.store_div_scaled_inputs2_indices(997, 918, 0.5, 983, 0.5, 911, 1.0);s.store_scaled_add_offset_sqrt_square_offset(997, 997, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(997), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 1.0, 250, 0.5);s.store_mul_square_lhs(998, 1, 227);s.store_sub_from_scalar_div_indices(999, 1.0, 998, 997);}
        s.b[1198] = ((s.v[983] / 2.0) < 80.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.store_ln_one_plus_exp_scaled_input(2, 983, 0.5);}
        if (!s.b[1198]) {s.store_scale(2, 983, 0.5);}
        s.store_scale(1000, 2, 2.0);s.b[1199] = ((s.v[984] / 2.0) < 80.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_ln_one_plus_exp_scaled_input(3, 984, 0.5);}
        if (!s.b[1199]) {s.store_scale(3, 984, 0.5);}
        s.store_scale(1001, 3, 2.0);s.store_sub(1002, 1001, 984);s.store_sub(1003, 1000, 983);s.store_add_scaled_products_indices(1004, 270, 1000, 1.0, 271, 1002, 1.0);s.store_add_scaled_products_indices(1005, 270, 1001, 1.0, 271, 1003, 1.0);s.store_scaled_add(1006, 931, 1000, 0.5);s.store_scaled_add(1007, 932, 1001, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1006), s.ad_value(1007));s.store_mul3_lhs(1008, 995, 1006, 0);s.store_mul3_lhs(1009, 995, 1007, 0);s.store_scaled_add(1010, 933, 1002, 0.5);s.store_scaled_add(1011, 934, 1003, 0.5);s.store_scaled_add(1012, 935, 1004, 0.5);s.store_scaled_add(1013, 936, 1005, 0.5);s.store_mul_product3_mixed_iiia(1014, 999, 1006, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1015, 1007, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_add(1016, 1014, 1015);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1010, 1.0, 51, 1011, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1017, 3, 4);s.store_mul_ad_product_rhs(1018, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1010)), 1.0), 1.0, s.ad_value(42), s.ad_value(1011), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1008), s.ad_value(268)), 1.0), 1.0, s.ad_value(1009), s.ad_value(269), 1.0)))));s.b[1200] = (s.v[56] == 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_scalar(4, 1.0);}
    }
}
