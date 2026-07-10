#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1166] {s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1167] = (s.v[800] < (-0.005));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if ((!s.b[1166]) && s.b[1167]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1166]) && (!s.b[1167])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1168] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1168]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1169] = (s.v[812] > 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1169]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 976);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);s.b[1170] = (p.p10 == 1.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });s.b[1171] = (((s.v[827]) as f64).abs() > 0.01);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        if (s.b[1170] && s.b[1171]) {s.store_mul(798, 907, 976);}
        s.b[1172] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1172]) {s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));}
        if ((s.b[1170] && s.b[1171]) && (!s.b[1172])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1170] && s.b[1171]) {s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);}
        s.b[1173] = (s.v[800] < (-0.005));s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if ((s.b[1170] && s.b[1171]) && s.b[1173]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1174] = (s.v[800] > 0.005);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && s.b[1174]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && (!s.b[1174])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
    ) {
        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && (!s.b[1174])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        if (s.b[1170] && s.b[1171]) {s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 976, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(976, 976, 827);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && s.b[1185]) {s.store_sub_mixed_ai(790, A::ln(A::div_scaled_inputs(s.ad_value(981), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0)), 803);}
        s.b[1186] = (s.v[981] < (-0.005));s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && s.b[1186]) {s.store_sin_scaled_input(791, 803, 0.5);s.store_ln_div_scaled_input_square_denominator(790, 981, -1.0, 791, 1.0);}
        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && (!s.b[1186])) {s.store_ln_ad(790, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(977, 906, 1.0, 905, (-1.0), 976, 1.0, A::ln(s.ad_value(789)), 2.0, 790);s.store_mul(980, 908, 977);s.store_add(978, 979, 980);}
        s.b[1187] = (s.v[981] > 0.005);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });s.b[1188] = ((((s.v[976] + s.v[975]) - s.v[905]) - s.v[803]) < 80.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1192] = (((s.v[981]) as f64).abs() > 0.005);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
        if (s.b[1191] && s.b[1192]) {s.store_add_scaled_products3_mixed_iiaiai(2, 986, 987, 1.0, A::offset(s.ad_value(976), 2.0), 987, 2.0, A::offset(s.ad_value(977), 2.0), 986, 2.0);s.store_div_scaled_product_by_product_indices(989, 981, 988, (-4.0), 978, 2, 1.0);}
        if (s.b[1191] && (!s.b[1192])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 981, 1.0, 981, 1.0, 981, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 986, 982, 1.0, 987, 983, 1.0, A::mul3(s.ad_value(986), s.ad_value(987), s.ad_value(978)), A::offset(A::mul(s.ad_value(978), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(989, 982, 983, 988, 1.0, 978, 3, 1.0);}
        s.store_add_mixed_ia(990, 975, A::ln(s.ad_value(978)));s.store_scaled_add(991, 913, 978, 0.5);s.store_sub(992, 990, 926);s.store_scalar(995, 1.0);s.b[1193] = (p.p9 > 0.0);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if s.b[1193] {s.store_div_scaled_inputs2_indices(993, 914, 0.5, 979, 0.5, 907, 1.0);s.store_scaled_add_offset_sqrt_square_offset(993, 993, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(993), s.ad_value(223)), 1.0, s.ad_value(246), s.ad_value(246), 0.25)), 1.0, 246, 0.5);s.store_mul_square_lhs(994, 1, 223);s.store_sub_from_scalar_div_indices(995, 1.0, 994, 993);}
        s.b[1194] = ((s.v[979] / 2.0) < 80.0);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_ln_one_plus_exp_scaled_input(2, 979, 0.5);}
        if (!s.b[1194]) {s.store_scale(2, 979, 0.5);}
        s.store_scale(996, 2, 2.0);s.b[1195] = ((s.v[980] / 2.0) < 80.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.store_ln_one_plus_exp_scaled_input(3, 980, 0.5);}
        if (!s.b[1195]) {s.store_scale(3, 980, 0.5);}
        s.store_scale(997, 3, 2.0);s.store_sub(998, 997, 980);s.store_sub(999, 996, 979);s.store_add_scaled_products_indices(1000, 266, 996, 1.0, 267, 998, 1.0);s.store_add_scaled_products_indices(1001, 266, 997, 1.0, 267, 999, 1.0);s.store_scaled_add(1002, 927, 996, 0.5);s.store_scaled_add(1003, 928, 997, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1002), s.ad_value(1003));s.store_mul3_lhs(1004, 991, 1002, 0);s.store_mul3_lhs(1005, 991, 1003, 0);s.store_scaled_add(1006, 929, 998, 0.5);s.store_scaled_add(1007, 930, 999, 0.5);s.store_scaled_add(1008, 931, 1000, 0.5);s.store_scaled_add(1009, 932, 1001, 0.5);s.store_mul_product3_mixed_iiia(1010, 995, 1002, 187, A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1011, 1003, 188, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_add(1012, 1010, 1011);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1006, 1.0, 51, 1007, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
    ) {
        s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1013, 3, 4);s.store_mul_ad_product_rhs(1014, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1006)), 1.0), 1.0, s.ad_value(42), s.ad_value(1007), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1004), s.ad_value(264)), 1.0), 1.0, s.ad_value(1005), s.ad_value(265), 1.0)))));s.b[1196] = (s.v[56] == 0.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if s.b[1196] {s.store_scalar(4, 1.0);}
        s.b[1197] = (s.v[56] < 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if ((!s.b[1196]) && s.b[1197]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1196]) && (!s.b[1197])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_add_scaled_product_rhs_indices(1015, 939, 54, 1.0, 991, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1016, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1008)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 38, 1015, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1017, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1009)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 39, 1015, 1.0);s.store_div_scaled_product_add_scaled_denominator(1018, 1013, 1012, 1.0, A::div(s.ad_value(1010), s.ad_value(1016)), 1.0, A::div(s.ad_value(1011), s.ad_value(1017)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1019, 1.0, 991, 4.0);s.b[1198] = (s.v[65] > 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1005, 1.0);}
        if (!s.b[1198]) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1005, 1.0);}
        s.store_mul3_lhs(1020, 991, 1019, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_ln_mixed_ia(1021, 1020, A::offset(A::div_scaled_inputs2(s.ad_value(335), 1.0, s.ad_value(975), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(991), s.ad_value(991)), 1.0, s.ad_value(66), s.ad_value(223), 1.0), 1.0), 1.0));s.store_mul(1022, 873, 1021);s.store_div_from_scalar_offset_ad(1023, 1.0, A::mul_offset_rhs(s.ad_value(1022), s.ad_value(1022), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(951, s.ad_value(1002), 100.0, s.ad_value(1002), 100.0, 1.0);s.b[1199] = (s.v[61] < 0.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_div_from_scalar_sub_from_scalar_ad(952, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(951)));}
        if (!s.b[1199]) {s.store_offset_mul(952, 61, 951, 1.0);}
        s.store_div_scaled_value_offset_denominator(953, s.ad_value(1003), 100.0, s.ad_value(1003), 100.0, 1.0);s.b[1200] = (s.v[62] < 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_div_from_scalar_sub_from_scalar_ad(954, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(953)));}
        if (!s.b[1200]) {s.store_offset_mul(954, 62, 953, 1.0);}
        s.store_mul_ad_affine_product_rhs(1024, 871, s.ad_value(992), A::add(s.ad_value(952), s.ad_value(954)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1025, 1024, 1.0, 1018, 1023, 1.0);s.store_square(1026, 1025);s.store_sqrt_offset_input(1027, 1026, 1.0);s.store_div_scaled_offset_numerator_indices(1028, 1026, 1.5, 1.0, 1027, 1.0);s.b[1201] = (p.p13 > 0.0);s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 254, 0.6, 1002, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 254, 0.6, 1003, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1029, A::mul(s.ad_value(907), s.ad_value(2)), 1.0, 1.0, 888, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1030, A::mul(s.ad_value(908), s.ad_value(3)), 1.0, 1.0, 889, 1.0);}
        if (!s.b[1201]) {s.store_scalar(1029, 1.0);s.store_scalar(1030, 1.0);}
        s.b[1202] = (s.v[913] > 1e-6);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });s.b[1203] = (s.v[978] > 1e-6);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });s.b[1204] = (((s.v[987]) as f64).abs() < 0.01);s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if ((s.b[1202] && s.b[1203]) && s.b[1204]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(976), 2.0), 1.0, 986, 0.5, A::offset(s.ad_value(977), 2.0), 986, 1.0);s.store_mul(2, 0, 987);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 980, 1.0, A::mul3_scaled_output(s.ad_value(981), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(986))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(977), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1031, A::div_scaled_add_product(s.ad_value(982), (-1.0), s.ad_value(989), s.ad_value(978), 1.0, s.ad_value(986), 1.0), 1.0, 2, (-1.0), 978, 1.0);s.store_div_scaled_product_offset_denominator_indices(1032, 1031, 978, 1.0, 1031, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
    ) {
        if ((s.b[1202] && s.b[1203]) && (!s.b[1204])) {s.store_sub_ad(1031, A::div_scaled_product_by_product(s.ad_value(989), s.ad_value(988), 1.0, s.ad_value(986), s.ad_value(987), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(982), s.ad_value(986)), 1.0, A::div(s.ad_value(983), s.ad_value(987)), 1.0, s.ad_value(978), 1.0));s.store_div_scaled_product_offset_denominator_indices(1032, 1031, 978, 1.0, 1031, 1.0, 1.0);}
        if (s.b[1202] && (!s.b[1203])) {s.copy_ad(1032, 949);}
        if s.b[1202] {s.store_sub(2, 1032, 956);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1205] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if (s.b[1202] && s.b[1205]) {s.store_sub(4, 978, 913);s.store_add_scaled_product_indices(1033, 4, 1.0, 1032, 992, (-1.0));s.store_add_scaled_product_indices(1034, 4, 1.0, 956, 992, (-1.0));s.store_sqrt_square_add(1035, 1033, 3);s.store_sqrt_square_add(1036, 1034, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1037, 0.25, 2, A::add_scaled_products3(s.ad_value(1036), s.ad_value(1033), 1.0, s.ad_value(1035), s.ad_value(1034), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1034), 1.0, s.ad_value(1036), 1.0, A::add(s.ad_value(1033), s.ad_value(1035)), 1.0)), 1.0));}
        if (s.b[1202] && (!s.b[1205])) {s.store_mul(4, 992, 2);s.store_div_scaled_product3_mixed_iiia(1037, 992, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (!s.b[1202]) {s.copy_ad(1032, 949);s.store_scalar(1037, 0.0);}
        s.store_add_scaled_inputs3_mixed_aii(1038, A::add_scaled_product(s.ad_value(1037), 1.0, s.ad_value(991), s.ad_value(992), 1.0), 1.0, 913, 1.0, 978, -1.0);s.b[1206] = (s.v[913] > 1e-6);s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });s.b[1207] = (s.v[1038] > 1e-30);s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });
        if (s.b[1206] && s.b[1207]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1039, 922, A::div(s.ad_value(918), s.ad_value(913)), 1.0, 925, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1040, 986, A::div(s.ad_value(982), s.ad_value(978)), 1.0, 989, -1.0);s.store_div_scaled_inputs2_indices(1041, 1039, 1.0, 1040, (-1.0), 1038, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1042, 923, A::div(s.ad_value(919), s.ad_value(913)), 1.0, 925, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 987, A::div(s.ad_value(983), s.ad_value(978)), 1.0, 989, -1.0);s.store_div_scaled_inputs2_indices(1044, 1042, 1.0, 1043, (-1.0), 1038, 1.0);}
        if (s.b[1206] && (!s.b[1207])) {s.store_scalar(1041, 0.0);s.store_scalar(1044, 0.0);}
        if (!s.b[1206]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1045, 944, A::div(s.ad_value(881), s.ad_value(947)), (-2.0), 950, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1046, 945, A::div(s.ad_value(882), s.ad_value(948)), (-2.0), 950, (-2.0));s.store_mul_sub_lhs(0, 1046, 1045, 950);s.store_mul(2, 1045, 881);s.store_mul(3, 1046, 882);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(944), s.ad_value(881), 2.0, s.ad_value(945), s.ad_value(882), 2.0), 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1206]) {s.store_div_scaled_inputs3_mixed_iiai(1047, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(947)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1048, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(948)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1041, 947, 950, -1.0, 1047, 947, -1.0);s.store_mul_add_scaled_product_rhs_indices(1044, 948, 950, -1.0, 1048, 948, -1.0);}
        s.store_mul(1049, 1041, 1028);s.store_mul(1050, 1044, 1028);s.store_scaled_sub(1051, 979, 914, 0.5);s.store_scaled_sub(1052, 980, 915, 0.5);s.store_mul(1053, 1051, 1049);s.store_mul(1054, 1052, 1050);s.copy_ad(379, 875);s.copy_ad(380, 879);s.copy_ad(381, 880);s.copy_ad(382, 881);s.copy_ad(383, 882);s.copy_ad(384, 909);s.copy_ad(385, 910);s.copy_ad(386, 894);s.copy_ad(387, 893);s.copy_ad(388, 912);s.copy_ad(389, 897);s.copy_ad(390, 898);s.copy_ad(391, 899);s.copy_ad(392, 900);s.copy_ad(393, 901);s.copy_ad(394, 904);s.copy_ad(395, 906);s.copy_ad(396, 905);s.copy_ad(397, 907);s.copy_ad(398, 908);s.copy_ad(399, 913);s.copy_ad(400, 914);s.copy_ad(401, 915);s.copy_ad(402, 926);s.copy_ad(403, 956);s.copy_ad(404, 979);s.copy_ad(405, 980);s.copy_ad(407, 975);s.copy_ad(408, 976);s.copy_ad(409, 978);s.copy_ad(410, 990);s.copy_ad(411, 991);s.copy_ad(412, 995);s.copy_ad(413, 1002);s.copy_ad(414, 1003);s.copy_ad(415, 1004);s.copy_ad(416, 1005);s.copy_ad(417, 1012);s.copy_ad(418, 1018);s.copy_ad(419, 1019);s.copy_ad(420, 1021);s.copy_ad(421, 1023);s.copy_ad(422, 1027);s.store_scalar(423, s.v[1024]);s.copy_ad(424, 1026);s.copy_ad(425, 1028);s.copy_ad(426, 1029);s.copy_ad(427, 1030);s.copy_ad(428, 1032);s.copy_ad(429, 1038);s.copy_ad(430, 1049);s.copy_ad(431, 1041);s.copy_ad(432, 1051);s.copy_ad(433, 1052);s.copy_ad(434, 1053);s.copy_ad(435, 1054);s.store_div_scaled_inputs_mixed_ia(338, 417, p.p35, A::add(s.ad_value(413), s.ad_value(414)), 1.0);s.store_mul_add_scaled_product_rhs_indices(339, 420, 63, 1.0, 271, 419, 1.0);s.store_mul_scale_offset_mixed_ia(340, 421, A::mul_offset_rhs(s.ad_value(339), s.ad_value(339), 1.0), 1.0, 1.0);s.store_mul3_lhs(341, 418, 421, 422);s.b[1208] = (p.p13 > 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if s.b[1208] {s.store_div_scaled_inputs2_mixed_iia(342, 413, 1.0, 414, 1.0, A::add(A::div(s.ad_value(413), s.ad_value(426)), A::div(s.ad_value(414), s.ad_value(427))), 1.0);}
        if (!s.b[1208]) {s.store_scalar(342, 1.0);}
        s.store_mul_square_lhs(343, 222, 338);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_product_by_product_mixed_aiii(344, A::mul3(s.ad_value(343), s.ad_value(386), s.ad_value(429)), 340, 1.0, 341, 342, 1.0);s.store_mul_scale_offset_indices(700, 220, 326, -1.0, 0.0);s.store_mul_scale_offset_indices(701, 220, 328, -1.0, 0.0);s.store_add_scaled_product_indices(0, 230, 1.0, 163, 220, p.p14);s.store_add(702, 700, 0);s.store_add(703, 701, 0);s.store_scalar(710, 0.0);s.store_scalar(711, 0.0);s.store_scalar(712, 0.0);s.store_scalar(713, 0.0);s.store_div_mixed_ai(704, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);s.store_square(705, 704);s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);let t0: f64 = (1e-5 * s.v[706]);s.store_scalar(707, t0);s.store_div_from_scalar(708, 1.0, 706);s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);s.b[1209] = (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0)));s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });s.b[1210] = (((s.v[700]) as f64).abs() <= s.v[707]);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if (s.b[1209] && s.b[1210]) {s.store_mul_scale_offset_indices(710, 708, 700, -1.0, 0.0);}
        s.b[1211] = (s.v[700] < (-s.v[707]));s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_neg(679, 700);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1212] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if (((s.b[1209] && (!s.b[1210])) && s.b[1211]) && s.b[1212]) {s.store_exp(689, 688);}
        s.b[1213] = (s.v[688] < (-80.0));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && s.b[1213]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(710, 688, 693);}
        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {s.store_mul_scale_offset_mixed_ia(694, 709, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(695, 700, 708, A::offset(A::mul(s.ad_value(694), s.ad_value(700)), 1.0));}
        s.b[1214] = ((((-s.v[695])) as f64).abs() < 80.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && s.b[1214]) {s.store_exp_neg_input(687, 695);}
        s.b[1215] = ((-s.v[695]) < (-80.0));s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1214])) && s.b[1215]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1214])) && (!s.b[1215])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(687, A::neg(s.ad_value(695)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {s.store_sub_from_scalar(693, 1.0, 687);s.store_add_scaled_inputs_product_mixed_iiia(696, 700, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));}
        s.b[1216] = ((((-s.v[696])) as f64).abs() < 80.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && s.b[1216]) {s.store_exp_neg_input(689, 696);}
        s.b[1217] = ((-s.v[696]) < (-80.0));s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1216])) && s.b[1217]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1216])) && (!s.b[1217])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {s.store_add_scaled_inputs3_mixed_iia(690, 700, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(700), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(710, 696, 697);}
        if (s.b[1209] && (!s.b[1210])) {s.store_neg(710, 710);}
        s.b[1218] = (s.v[159] > 0.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });s.b[1219] = (((s.v[702]) as f64).abs() <= s.v[707]);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if (s.b[1218] && s.b[1219]) {s.store_mul_scale_offset_indices(712, 708, 702, -1.0, 0.0);}
        s.b[1220] = (s.v[702] < (-s.v[707]));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_neg(679, 702);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
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
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {s.store_add_scaled_inputs3_mixed_iia(690, 702, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(702), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(712, 696, 697);}
        if (s.b[1218] && (!s.b[1219])) {s.store_neg(712, 712);}
        s.store_div_mixed_ai(704, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);s.store_square(705, 704);s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);let t1: f64 = (1e-5 * s.v[706]);s.store_scalar(707, t1);s.store_div_from_scalar(708, 1.0, 706);s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);s.b[1227] = (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0)));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });s.b[1228] = (((s.v[701]) as f64).abs() <= s.v[707]);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
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
    pub(super) fn stamp_transient_block_62(
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
    pub(super) fn stamp_transient_block_63(
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
}
