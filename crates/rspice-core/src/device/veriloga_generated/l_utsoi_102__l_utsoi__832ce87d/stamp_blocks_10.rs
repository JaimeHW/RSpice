#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1132] && s.b[1135]) {s.store_sqrt_offset_ad(966, A::add(A::square(s.ad_value(963)), A::mul3_scaled_output(s.ad_value(962), s.ad_value(962), s.ad_value(962), 0.148148148148)), 1e-20);s.store_sqrt_offset_ad(967, A::add(A::square(s.ad_value(965)), A::mul3_scaled_output(s.ad_value(964), s.ad_value(964), s.ad_value(964), 0.148148148148)), 1e-20);s.store_sub_ad(968, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(966), s.ad_value(963)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(966), s.ad_value(963)), 0.5), 0.3333333333333));s.store_sub_ad(969, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(967), s.ad_value(965)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(967), s.ad_value(965)), 0.5), 0.3333333333333));}
        if (s.b[1132] && (!s.b[1135])) {s.copy_ad(968, 959);s.copy_ad(969, 960);}
        if s.b[1132] {s.store_square(4, 2);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(970, 968, (0.94 * 0.5), 969, (0.94 * 0.5), A::add_scaled_inputs(A::square(A::sub(s.ad_value(968), s.ad_value(969))), 1.0, s.ad_value(4), 10.0), (0.94 * 0.5));s.store_add_scaled_product_indices(971, 913, 1.0, 956, 970, 1.0);s.store_mul_sub_rhs(972, 949, 970, 946);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(973, 971, 0.5, 972, 0.5, A::add_scaled_inputs(A::square(A::sub(s.ad_value(971), s.ad_value(972))), 1.0, s.ad_value(4), 36.0), 0.5);}
        if (!s.b[1132]) {s.copy_ad(956, 949);s.store_scaled_offset(970, 946, 1.0, 0.94);s.store_add_scaled_product_mixed_iia(973, 913, 0.5, 949, A::sub_scaled_inputs(s.ad_value(970), 1.0, s.ad_value(946), 0.5), 1.0);}
        s.b[1136] = ((s.v[973] - 0.5) < 80.0);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });
        if s.b[1136] {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(973), (-0.5)));}
        if (!s.b[1136]) {s.store_offset(2, 973, (-0.5));}
        s.store_offset(3, 2, 0.5);s.store_add_mixed_ia(4, 970, A::ln(A::div(s.ad_value(913), s.ad_value(3))));s.b[1137] = ((s.v[4] - 6.0) < 80.0);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if s.b[1137] {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));}
        if (!s.b[1137]) {s.store_offset(2, 4, (-6.0));}
        s.store_offset(4, 2, 6.0);s.b[1138] = ((s.v[221] - s.v[4]) < 80.0);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if s.b[1138] {s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(221), s.ad_value(4)));}
        if (!s.b[1138]) {s.store_sub(2, 221, 4);}
        s.store_sub(974, 221, 2);s.store_div(2, 335, 974);s.store_square(3, 2);s.store_square(4, 3);s.store_square(5, 4);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(872), s.ad_value(4)), 1.0)), 2.666666666667);s.store_mul_mixed_ia(975, 335, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));s.store_div_from_scalar_offset_input(789, 1.0, 907, 1.0);s.store_div_from_scalar_offset_input(790, 1.0, 908, 1.0);s.store_offset_add_ad(792, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(908), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0)), s.ad_value(975), 3.0);s.store_offset_add_ad(793, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(907), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0)), s.ad_value(975), 3.0);s.b[1139] = (((s.v[792] - s.v[883]) * 0.3333333333333) < 80.0);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if s.b[1139] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(883), 0.3333333333333));}
        if (!s.b[1139]) {s.store_scaled_sub(791, 792, 883, 0.3333333333333);}
        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);s.b[1140] = (((s.v[793] - s.v[884]) * 0.3333333333333) < 80.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(884), 0.3333333333333));}
        if (!s.b[1140]) {s.store_scaled_sub(791, 793, 884, 0.3333333333333);}
        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);s.store_mul_add_scaled_product_rhs_indices(794, 789, 797, 1.0, 907, 905, 1.0);s.store_mul_add_scaled_product_rhs_indices(795, 790, 796, 1.0, 908, 906, 1.0);s.b[1141] = (((s.v[792] - s.v[794]) * 0.3333333333333) < 80.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(794), 0.3333333333333));}
        if (!s.b[1141]) {s.store_scaled_sub(791, 792, 794, 0.3333333333333);}
        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);s.b[1142] = (((s.v[793] - s.v[795]) * 0.3333333333333) < 80.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(795), 0.3333333333333));}
        if (!s.b[1142]) {s.store_scaled_sub(791, 793, 795, 0.3333333333333);}
        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);s.store_sub(976, 905, 796);s.store_sub(977, 906, 797);s.store_scalar(803, 0.0);s.store_scalar(806, 0.0);s.store_mul(798, 907, 976);s.b[1143] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if (!s.b[1143]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1144] = (s.v[800] < (-0.005));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_sqrt_abs_ad(803, s.ad_value(800));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1144] {s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1145] = (s.v[800] > 0.005);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if ((!s.b[1144]) && s.b[1145]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if ((!s.b[1144]) && (!s.b[1145])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
    ) {
        s.b[1146] = (s.v[800] > 0.005);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1147] = (s.v[800] < (-0.005));s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if ((!s.b[1146]) && s.b[1147]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1146]) && (!s.b[1147])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1148] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1148]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1149] = (s.v[812] > 0.0);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1149]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 976);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
    ) {
        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);s.store_mul(798, 907, 976);s.store_mul(828, 908, 977);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.store_sub_square_lhs(833, 798, 800);s.b[1150] = (s.v[833] > 0.0);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if s.b[1150] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 975, 1.0, 905, -1.0, 976, 1.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);s.store_add_scaled_inputs3_indices(834, 905, 1.0, 976, (-1.0), 792, -1.0);}
        s.b[1151] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (s.b[1150] && s.b[1151]) {s.store_sub_div_rhs_indices(976, 976, 824, 825);}
        s.store_mul(798, 907, 976);s.store_mul(828, 908, 977);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.b[1152] = (s.v[800] < (-0.005));s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        s.b[1153] = (s.v[800] > 0.005);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if ((!s.b[1152]) && s.b[1153]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        if ((!s.b[1152]) && (!s.b[1153])) {s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(805, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(800, 800, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(821), s.ad_value(804), 1.0, s.ad_value(798), s.ad_value(828), 1.0), 1.0, s.ad_value(800), 1.0, A::offset(A::mul(s.ad_value(821), s.ad_value(805)), 1.0), 1.0));s.store_sub_square_lhs(833, 798, 800);s.b[1154] = (s.v[833] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1154] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 975, 1.0, 905, -1.0, 976, 1.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);s.store_add_scaled_inputs3_indices(834, 905, 1.0, 976, (-1.0), 792, -1.0);}
        s.b[1155] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (s.b[1154] && s.b[1155]) {s.store_sub_div_rhs_indices(976, 976, 824, 825);}
        s.store_mul(798, 907, 976);s.b[1156] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if (!s.b[1156]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1157] = (s.v[800] < (-0.005));s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1158] = (s.v[800] > 0.005);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if ((!s.b[1157]) && s.b[1158]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1157]) && (!s.b[1158])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1159] = (s.v[800] > 0.005);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1160] = (s.v[800] < (-0.005));s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if ((!s.b[1159]) && s.b[1160]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1159]) && (!s.b[1160])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1161] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1161]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1162] = (s.v[812] > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if s.b[1162] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1162]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 976);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);s.store_mul(798, 907, 976);s.b[1163] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if (!s.b[1163]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1164] = (s.v[800] < (-0.005));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1165] = (s.v[800] > 0.005);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if ((!s.b[1164]) && s.b[1165]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1164]) && s.b[1165]) {s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if ((!s.b[1164]) && (!s.b[1165])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1166] = (s.v[800] > 0.005);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1167] = (s.v[800] < (-0.005));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if ((!s.b[1166]) && s.b[1167]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1166]) && (!s.b[1167])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1168] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1168]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1168]) {s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1169] = (s.v[812] > 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1169]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 976);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);s.b[1170] = (p.p10 == 1.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });s.b[1171] = (((s.v[827]) as f64).abs() > 0.01);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if (s.b[1170] && s.b[1171]) {s.store_mul(798, 907, 976);}
        s.b[1172] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1172]) {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if ((s.b[1170] && s.b[1171]) && (!s.b[1172])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1170] && s.b[1171]) {s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);}
        s.b[1173] = (s.v[800] < (-0.005));s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1173]) {s.store_sqrt_abs_ad(803, s.ad_value(800));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1170] && s.b[1171]) && s.b[1173]) {s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1174] = (s.v[800] > 0.005);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && s.b[1174]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && (!s.b[1174])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        s.b[1175] = (s.v[800] > 0.005);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1175]) {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1176] = (s.v[800] < (-0.005));s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (((s.b[1170] && s.b[1171]) && (!s.b[1175])) && s.b[1176]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if (((s.b[1170] && s.b[1171]) && (!s.b[1175])) && (!s.b[1176])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1177] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1177]) {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if ((s.b[1170] && s.b[1171]) && (!s.b[1177])) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1178] = (s.v[812] > 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1178]) {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if ((s.b[1170] && s.b[1171]) && (!s.b[1178])) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 976);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        if (s.b[1170] && s.b[1171]) {s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1170] && s.b[1171]) {s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);}
        s.store_mul(979, 907, 976);s.b[1179] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if s.b[1179] {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if (!s.b[1179]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(982, 880, 789);s.store_sub_square_lhs(981, 979, 982);s.b[1180] = (s.v[982] <= 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if s.b[1180] {s.store_scalar(978, 1e-80);s.store_sub(980, 978, 979);s.store_div(977, 980, 908);}
        s.b[1181] = (s.v[981] < (-0.005));s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if ((!s.b[1180]) && s.b[1181]) {s.store_sqrt_abs_ad(803, s.ad_value(981));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));}
        s.b[1182] = (s.v[981] > 0.005);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (((!s.b[1180]) && (!s.b[1181])) && s.b[1182]) {s.store_sqrt_abs_ad(803, s.ad_value(981));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);}
        if (((!s.b[1180]) && (!s.b[1181])) && (!s.b[1182])) {s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1183] = (((1.01 * s.v[979]) + s.v[804]) > 0.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if ((!s.b[1180]) && s.b[1183]) {s.store_add(789, 979, 804);}
        s.b[1184] = ((s.v[982] * s.v[979]) < (((0.9 * s.v[979]) * s.v[979]) * s.v[789]));s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if (((!s.b[1180]) && s.b[1183]) && s.b[1184]) {s.store_offset_div(978, 982, 789, 1e-80);s.store_sub(980, 978, 979);s.store_div(977, 980, 908);}
        s.b[1185] = (s.v[981] > 0.005);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if ((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && s.b[1185]) {s.store_sub_mixed_ai(790, A::ln(A::div_scaled_inputs(s.ad_value(981), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0)), 803);}
        s.b[1186] = (s.v[981] < (-0.005));s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && s.b[1186]) {s.store_sin_scaled_input(791, 803, 0.5);s.store_ln_div_scaled_input_square_denominator(790, 981, -1.0, 791, 1.0);}
        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && (!s.b[1186])) {s.store_ln_ad(790, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(977, 906, 1.0, 905, (-1.0), 976, 1.0, A::ln(s.ad_value(789)), 2.0, 790);s.store_mul(980, 908, 977);s.store_add(978, 979, 980);}
        s.b[1187] = (s.v[981] > 0.005);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });s.b[1188] = ((((s.v[976] + s.v[975]) - s.v[905]) - s.v[803]) < 80.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) && s.b[1188]) {s.store_exp_ad(791, A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0));}
        if ((((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) && (!s.b[1188])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(791, A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) {s.store_div(790, 791, 880);s.store_div_scaled_product_mixed_iia(789, 981, 790, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);}
        s.b[1189] = (s.v[981] < (-0.005));s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if ((((!s.b[1180]) && (!s.b[1183])) && (!s.b[1187])) && s.b[1189]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_value_by_product_mixed_iai(789, 981, -1.0, A::square(s.ad_value(790)), 982, 1.0);}
        if ((((!s.b[1180]) && (!s.b[1183])) && (!s.b[1187])) && (!s.b[1189])) {s.store_div_mixed_ai(789, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0396825396825397), 0.05), 0.3333333333333)), 982);}
        if ((!s.b[1180]) && (!s.b[1183])) {s.store_offset_div_scaled_inputs2_mixed_iia(978, 979, 1.0, 804, (-1.0), A::sub_from_scalar(1.0, s.ad_value(789)), 1.0, 1e-80);s.store_sub(980, 978, 979);s.store_div(977, 980, 908);}
        s.b[1190] = (((s.v[906] - s.v[977]) - s.v[975]) < 80.0);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if s.b[1190] {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0));}
        if (!s.b[1190]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(983, 880, 789);s.store_scalar(986, 0.0);s.store_scalar(987, 0.0);s.store_scalar(984, 0.0);s.store_scalar(985, 0.0);s.store_scalar(988, 0.0);s.store_scalar(989, 0.0);s.b[1191] = (s.v[913] > 1e-6);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if s.b[1191] {s.store_mul(984, 982, 881);s.store_mul(985, 983, 882);s.store_add_scaled_inputs(986, 984, 1.0, 979, 2.0);s.store_add_scaled_inputs(987, 985, 1.0, 980, 2.0);s.store_add_scaled_inputs3_indices(988, 978, 2.0, 984, 1.0, 985, 1.0);}
        s.b[1192] = (((s.v[981]) as f64).abs() > 0.005);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
        if (s.b[1191] && s.b[1192]) {s.store_add_scaled_products3_mixed_iiaiai(2, 986, 987, 1.0, A::offset(s.ad_value(976), 2.0), 987, 2.0, A::offset(s.ad_value(977), 2.0), 986, 2.0);s.store_div_scaled_product_by_product_indices(989, 981, 988, (-4.0), 978, 2, 1.0);}
        if (s.b[1191] && (!s.b[1192])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 981, 1.0, 981, 1.0, 981, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 986, 982, 1.0, 987, 983, 1.0, A::mul3(s.ad_value(986), s.ad_value(987), s.ad_value(978)), A::offset(A::mul(s.ad_value(978), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(989, 982, 983, 988, 1.0, 978, 3, 1.0);}
        s.store_add_mixed_ia(990, 975, A::ln(s.ad_value(978)));s.store_scaled_add(991, 913, 978, 0.5);s.store_sub(992, 990, 926);s.store_scalar(995, 1.0);s.b[1193] = (p.p9 > 0.0);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1193] {s.store_div_scaled_inputs2_indices(993, 914, 0.5, 979, 0.5, 907, 1.0);s.store_scaled_add_offset_sqrt_square_offset(993, 993, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(993), s.ad_value(223)), 1.0, s.ad_value(246), s.ad_value(246), 0.25)), 1.0, 246, 0.5);s.store_mul_square_lhs(994, 1, 223);s.store_sub_from_scalar_div_indices(995, 1.0, 994, 993);}
        s.b[1194] = ((s.v[979] / 2.0) < 80.0);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_ln_one_plus_exp_scaled_input(2, 979, 0.5);}
        if (!s.b[1194]) {s.store_scale(2, 979, 0.5);}
        s.store_scale(996, 2, 2.0);s.b[1195] = ((s.v[980] / 2.0) < 80.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.store_ln_one_plus_exp_scaled_input(3, 980, 0.5);}
        if (!s.b[1195]) {s.store_scale(3, 980, 0.5);}
        s.store_scale(997, 3, 2.0);s.store_sub(998, 997, 980);s.store_sub(999, 996, 979);s.store_add_scaled_products_indices(1000, 266, 996, 1.0, 267, 998, 1.0);s.store_add_scaled_products_indices(1001, 266, 997, 1.0, 267, 999, 1.0);s.store_scaled_add(1002, 927, 996, 0.5);s.store_scaled_add(1003, 928, 997, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1002), s.ad_value(1003));s.store_mul3_lhs(1004, 991, 1002, 0);s.store_mul3_lhs(1005, 991, 1003, 0);s.store_scaled_add(1006, 929, 998, 0.5);s.store_scaled_add(1007, 930, 999, 0.5);s.store_scaled_add(1008, 931, 1000, 0.5);s.store_scaled_add(1009, 932, 1001, 0.5);s.store_mul_product3_mixed_iiia(1010, 995, 1002, 187, A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1011, 1003, 188, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_add(1012, 1010, 1011);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1006, 1.0, 51, 1007, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1013, 3, 4);s.store_mul_ad_product_rhs(1014, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1006)), 1.0), 1.0, s.ad_value(42), s.ad_value(1007), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1004), s.ad_value(264)), 1.0), 1.0, s.ad_value(1005), s.ad_value(265), 1.0)))));s.b[1196] = (s.v[56] == 0.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if s.b[1196] {s.store_scalar(4, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        s.b[1197] = (s.v[56] < 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if ((!s.b[1196]) && s.b[1197]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1196]) && (!s.b[1197])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_add_scaled_product_rhs_indices(1015, 939, 54, 1.0, 991, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1016, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1008)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 38, 1015, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1017, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1009)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 39, 1015, 1.0);s.store_div_scaled_product_add_scaled_denominator(1018, 1013, 1012, 1.0, A::div(s.ad_value(1010), s.ad_value(1016)), 1.0, A::div(s.ad_value(1011), s.ad_value(1017)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1019, 1.0, 991, 4.0);s.b[1198] = (s.v[65] > 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1005, 1.0);}
        if (!s.b[1198]) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1005, 1.0);}
        s.store_mul3_lhs(1020, 991, 1019, 0);s.store_mul_ln_mixed_ia(1021, 1020, A::offset(A::div_scaled_inputs2(s.ad_value(335), 1.0, s.ad_value(975), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(991), s.ad_value(991)), 1.0, s.ad_value(66), s.ad_value(223), 1.0), 1.0), 1.0));s.store_mul(1022, 873, 1021);s.store_div_from_scalar_offset_ad(1023, 1.0, A::mul_offset_rhs(s.ad_value(1022), s.ad_value(1022), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(951, s.ad_value(1002), 100.0, s.ad_value(1002), 100.0, 1.0);s.b[1199] = (s.v[61] < 0.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_div_from_scalar_sub_from_scalar_ad(952, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(951)));}
        if (!s.b[1199]) {s.store_offset_mul(952, 61, 951, 1.0);}
        s.store_div_scaled_value_offset_denominator(953, s.ad_value(1003), 100.0, s.ad_value(1003), 100.0, 1.0);s.b[1200] = (s.v[62] < 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_div_from_scalar_sub_from_scalar_ad(954, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(953)));}
        if (!s.b[1200]) {s.store_offset_mul(954, 62, 953, 1.0);}
        s.store_mul_ad_affine_product_rhs(1024, 871, s.ad_value(992), A::add(s.ad_value(952), s.ad_value(954)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1025, 1024, 1.0, 1018, 1023, 1.0);s.store_square(1026, 1025);s.store_sqrt_offset_input(1027, 1026, 1.0);s.store_div_scaled_offset_numerator_indices(1028, 1026, 1.5, 1.0, 1027, 1.0);
    }
}
