#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs3_sqrt_third_mixed_aia(906, A::add_scaled_product(s.ad_value(903), 1.0, s.ad_value(26), A::sub(s.ad_value(904), s.ad_value(903)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(903), 1.0, s.ad_value(26), A::sub(s.ad_value(904), s.ad_value(903)), 1.0), s.ad_value(221))), 0.01), (-0.5));s.store_div(907, 890, 899);s.store_div(908, 891, 900);s.store_div_from_scalar(881, 1.0, 907);s.store_div_from_scalar(882, 1.0, 908);s.store_div_from_scalar_add_ad(909, 1.0, A::offset(s.ad_value(881), 1.0), s.ad_value(882));s.store_div_square_rhs(880, 249, 894);s.store_div_scaled_offset_numerator_mixed_ia(877, 907, 1.0, 1.0, A::offset(s.ad_value(908), 1.0), 1.0);s.store_ln(878, 877);s.b[1071] = (s.v[878] > 1e-8);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if s.b[1071] {s.store_div_scaled_product_offset_denominator_mixed_iai(879, 878, A::offset(s.ad_value(877), 1.0), 2.0, 877, (-1.0), 1.0);}
        if (!s.b[1071]) {s.store_scaled_offset(879, 878, 2.0, 2.0);}
        s.store_mul_sub_rhs(910, 909, 905, 906);s.store_square(911, 910);s.store_add_scaled_product_indices(883, 905, 1.0, 910, 881, (-1.0));s.store_add_scaled_product_indices(884, 906, 1.0, 910, 882, 1.0);s.store_div_from_scalar_offset_input(789, 1.0, 907, 1.0);s.store_div_from_scalar_offset_input(790, 1.0, 908, 1.0);s.store_offset_ln_ad(792, A::div_scaled_product(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(908), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 3.0);s.store_offset_ln_ad(793, A::div_scaled_product(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(907), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 3.0);s.b[1072] = (((s.v[792] - s.v[883]) * 0.3333333333333) < 80.0);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if s.b[1072] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(883), 0.3333333333333));}
        if (!s.b[1072]) {s.store_scaled_sub(791, 792, 883, 0.3333333333333);}
        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);s.b[1073] = (((s.v[793] - s.v[884]) * 0.3333333333333) < 80.0);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if s.b[1073] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(884), 0.3333333333333));}
        if (!s.b[1073]) {s.store_scaled_sub(791, 793, 884, 0.3333333333333);}
        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);s.store_mul_add_scaled_product_rhs_indices(794, 789, 797, 1.0, 907, 905, 1.0);s.store_mul_add_scaled_product_rhs_indices(795, 790, 796, 1.0, 908, 906, 1.0);s.b[1074] = (((s.v[792] - s.v[794]) * 0.3333333333333) < 80.0);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if s.b[1074] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(794), 0.3333333333333));}
        if (!s.b[1074]) {s.store_scaled_sub(791, 792, 794, 0.3333333333333);}
        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);s.b[1075] = (((s.v[793] - s.v[795]) * 0.3333333333333) < 80.0);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if s.b[1075] {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(795), 0.3333333333333));}
        if (!s.b[1075]) {s.store_scaled_sub(791, 793, 795, 0.3333333333333);}
        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);s.store_sub(912, 905, 796);s.store_sub(916, 906, 797);s.store_scalar(803, 0.0);s.store_scalar(806, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        s.store_mul(798, 907, 912);s.b[1076] = ((s.v[905] - s.v[912]) < 80.0);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if s.b[1076] {s.store_exp_sub(789, 905, 912);}
        if (!s.b[1076]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1077] = (s.v[800] < (-0.005));s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if s.b[1077] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1078] = (s.v[800] > 0.005);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if ((!s.b[1077]) && s.b[1078]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if ((!s.b[1077]) && (!s.b[1078])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1077]) && (!s.b[1078])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1079] = (s.v[800] > 0.005);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if s.b[1079] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1080] = (s.v[800] < (-0.005));s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if ((!s.b[1079]) && s.b[1080]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1079]) && (!s.b[1080])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1081] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if s.b[1081] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1081]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1082] = (s.v[812] > 0.0);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if s.b[1082] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1082]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 912);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 912, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(912, 912, 827);s.store_mul(798, 907, 912);s.store_mul(828, 908, 916);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.store_sub_square_lhs(833, 798, 800);s.b[1083] = (s.v[833] > 0.0);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if s.b[1083] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 905, (-1.0), 912, 1.0, 0.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);s.store_add_scaled_inputs3_indices(834, 905, 1.0, 912, (-1.0), 792, -1.0);}
        s.b[1084] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1083] && s.b[1084]) {s.store_sub_div_rhs_indices(912, 912, 824, 825);}
        s.store_mul(798, 907, 912);s.store_mul(828, 908, 916);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.b[1085] = (s.v[800] < (-0.005));s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if s.b[1085] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        s.b[1086] = (s.v[800] > 0.005);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((!s.b[1085]) && s.b[1086]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        if ((!s.b[1085]) && (!s.b[1086])) {s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1085]) && (!s.b[1086])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(805, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(800, 800, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(821), s.ad_value(804), 1.0, s.ad_value(798), s.ad_value(828), 1.0), 1.0, s.ad_value(800), 1.0, A::offset(A::mul(s.ad_value(821), s.ad_value(805)), 1.0), 1.0));s.store_sub_square_lhs(833, 798, 800);s.b[1087] = (s.v[833] > 0.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if s.b[1087] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 905, (-1.0), 912, 1.0, 0.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);s.store_add_scaled_inputs3_indices(834, 905, 1.0, 912, (-1.0), 792, -1.0);}
        s.b[1088] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (s.b[1087] && s.b[1088]) {s.store_sub_div_rhs_indices(912, 912, 824, 825);}
        s.store_mul(798, 907, 912);s.b[1089] = ((s.v[905] - s.v[912]) < 80.0);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if s.b[1089] {s.store_exp_sub(789, 905, 912);}
        if (!s.b[1089]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1090] = (s.v[800] < (-0.005));s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if s.b[1090] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1091] = (s.v[800] > 0.005);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if ((!s.b[1090]) && s.b[1091]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1090]) && s.b[1091]) {s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if ((!s.b[1090]) && (!s.b[1091])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1092] = (s.v[800] > 0.005);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if s.b[1092] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1093] = (s.v[800] < (-0.005));s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if ((!s.b[1092]) && s.b[1093]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if ((!s.b[1092]) && (!s.b[1093])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1094] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if s.b[1094] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1094]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1094]) {s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1095] = (s.v[812] > 0.0);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if s.b[1095] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1095]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 912);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 912, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(912, 912, 827);s.store_mul(798, 907, 912);s.b[1096] = ((s.v[905] - s.v[912]) < 80.0);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if s.b[1096] {s.store_exp_sub(789, 905, 912);}
        if (!s.b[1096]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);s.b[1097] = (s.v[800] < (-0.005));s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if s.b[1097] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1097] {s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1098] = (s.v[800] > 0.005);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if ((!s.b[1097]) && s.b[1098]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if ((!s.b[1097]) && (!s.b[1098])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1099] = (s.v[800] > 0.005);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if s.b[1099] {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1100] = (s.v[800] < (-0.005));s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((!s.b[1099]) && s.b[1100]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1099]) && (!s.b[1100])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1101] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if s.b[1101] {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if (!s.b[1101]) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1102] = (s.v[812] > 0.0);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if s.b[1102] {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if (!s.b[1102]) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 912);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 912, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(912, 912, 827);s.b[1103] = (p.p10 == 1.0);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });s.b[1104] = (((s.v[827]) as f64).abs() > 0.01);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if (s.b[1103] && s.b[1104]) {s.store_mul(798, 907, 912);}
        s.b[1105] = ((s.v[905] - s.v[912]) < 80.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((s.b[1103] && s.b[1104]) && s.b[1105]) {s.store_exp_sub(789, 905, 912);}
        if ((s.b[1103] && s.b[1104]) && (!s.b[1105])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1103] && s.b[1104]) {s.store_mul(799, 880, 789);s.store_sub_square_lhs(800, 798, 799);s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);}
        s.b[1106] = (s.v[800] < (-0.005));s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((s.b[1103] && s.b[1104]) && s.b[1106]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        s.b[1107] = (s.v[800] > 0.005);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if (((s.b[1103] && s.b[1104]) && (!s.b[1106])) && s.b[1107]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);s.store_mul_add_mixed_iia(805, 789, 800, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)));s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);s.store_mul_div_lhs(810, 801, 800, 790);s.store_div_mixed_ai(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);}
        if (((s.b[1103] && s.b[1104]) && (!s.b[1106])) && (!s.b[1107])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(791, 800, 1.0, 800, 1.0, 800, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(804, 800, 791, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(789, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(805, 801, 789);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(790, 800, 1.0, 800, 1.0, 800, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1103] && s.b[1104]) && (!s.b[1106])) && (!s.b[1107])) {s.store_scaled_mul(810, 801, 791, (-0.5));s.store_add_scaled_product_mixed_aii(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));}
        s.b[1108] = (s.v[800] > 0.005);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if ((s.b[1103] && s.b[1104]) && s.b[1108]) {s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);s.store_mul(808, 790, 806);s.store_sub_ln_lhs(809, 790, 803);}
        s.b[1109] = (s.v[800] < (-0.005));s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (((s.b[1103] && s.b[1104]) && (!s.b[1108])) && s.b[1109]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_inputs_square_rhs(808, 800, -1.0, 790, 1.0);s.store_ln(809, 808);}
        if (((s.b[1103] && s.b[1104]) && (!s.b[1108])) && (!s.b[1109])) {s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(809, 808);}
        s.b[1110] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if ((s.b[1103] && s.b[1104]) && s.b[1110]) {s.store_add(812, 798, 804);s.store_add(813, 907, 805);s.copy_ad(814, 807);}
        if ((s.b[1103] && s.b[1104]) && (!s.b[1110])) {s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));s.store_sub(791, 805, 907);s.store_mul_sub_lhs(812, 799, 808, 790);s.store_mul_mixed_ai(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);s.store_mul_mixed_ai(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);}
        s.b[1111] = (s.v[812] > 0.0);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if ((s.b[1103] && s.b[1104]) && s.b[1111]) {s.store_ln(815, 812);s.store_div_from_scalar(789, 1.0, 812);s.store_mul(816, 813, 789);s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);}
        if ((s.b[1103] && s.b[1104]) && (!s.b[1111])) {s.store_add_offset_lhs_mixed_ia(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));s.store_div_from_scalar(789, 1.0, 912);s.store_add(816, 907, 789);s.store_mul_scale_offset_indices(817, 789, 789, -1.0, 0.0);}
        if (s.b[1103] && s.b[1104]) {s.store_sub_add_scaled_inputs4_lhs_indices(818, 906, 1.0, 905, (-1.0), 912, 1.0, 815, 2.0, 809);s.store_sub_mixed_ai(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1103] && s.b[1104]) {s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(912, 912, 827);}
        s.store_mul(914, 907, 912);s.b[1112] = ((s.v[905] - s.v[912]) < 80.0);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if s.b[1112] {s.store_exp_sub(789, 905, 912);}
        if (!s.b[1112]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(918, 880, 789);s.store_sub_square_lhs(917, 914, 918);s.b[1113] = (s.v[918] <= 0.0);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if s.b[1113] {s.store_scalar(913, 1e-80);s.store_sub(915, 913, 914);s.store_div(916, 915, 908);}
        s.b[1114] = (s.v[917] < (-0.005));s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if ((!s.b[1113]) && s.b[1114]) {s.store_sqrt_abs_ad(803, s.ad_value(917));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));}
        s.b[1115] = (s.v[917] > 0.005);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (((!s.b[1113]) && (!s.b[1114])) && s.b[1115]) {s.store_sqrt_abs_ad(803, s.ad_value(917));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);}
        if (((!s.b[1113]) && (!s.b[1114])) && (!s.b[1115])) {s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1116] = (((1.01 * s.v[914]) + s.v[804]) > 0.0);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if ((!s.b[1113]) && s.b[1116]) {s.store_add(789, 914, 804);}
        s.b[1117] = ((s.v[918] * s.v[914]) < (((0.9 * s.v[914]) * s.v[914]) * s.v[789]));s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (((!s.b[1113]) && s.b[1116]) && s.b[1117]) {s.store_offset_div(913, 918, 789, 1e-80);s.store_sub(915, 913, 914);s.store_div(916, 915, 908);}
        s.b[1118] = (s.v[917] > 0.005);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if ((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && s.b[1118]) {s.store_sub_mixed_ai(790, A::ln(A::div_scaled_inputs(s.ad_value(917), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0)), 803);}
        s.b[1119] = (s.v[917] < (-0.005));s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if (((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && (!s.b[1118])) && s.b[1119]) {s.store_sin_scaled_input(791, 803, 0.5);s.store_ln_div_scaled_input_square_denominator(790, 917, -1.0, 791, 1.0);}
        if (((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && (!s.b[1118])) && (!s.b[1119])) {s.store_ln_ad(790, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(916, 906, 1.0, 905, (-1.0), 912, 1.0, A::ln(s.ad_value(789)), 2.0, 790);s.store_mul(915, 908, 916);s.store_add(913, 914, 915);}
        s.b[1120] = (s.v[917] > 0.005);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        s.b[1121] = (((s.v[912] - s.v[905]) - s.v[803]) < 80.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if ((((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) && s.b[1121]) {s.store_exp_ad(791, A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0));}
        if ((((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) && (!s.b[1121])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(791, A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) {s.store_div(790, 791, 880);s.store_div_scaled_product_mixed_iia(789, 917, 790, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);}
        s.b[1122] = (s.v[917] < (-0.005));s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if ((((!s.b[1113]) && (!s.b[1116])) && (!s.b[1120])) && s.b[1122]) {s.store_sin_scaled_input(790, 803, 0.5);s.store_div_scaled_value_by_product_mixed_iai(789, 917, -1.0, A::square(s.ad_value(790)), 918, 1.0);}
        if ((((!s.b[1113]) && (!s.b[1116])) && (!s.b[1120])) && (!s.b[1122])) {s.store_div_mixed_ai(789, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0396825396825397), 0.05), 0.3333333333333)), 918);}
        if ((!s.b[1113]) && (!s.b[1116])) {s.store_offset_div_scaled_inputs2_mixed_iia(913, 914, 1.0, 804, (-1.0), A::sub_from_scalar(1.0, s.ad_value(789)), 1.0, 1e-80);s.store_sub(915, 913, 914);s.store_div(916, 915, 908);}
        s.b[1123] = ((s.v[906] - s.v[916]) < 80.0);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if s.b[1123] {s.store_exp_sub(789, 906, 916);}
        if (!s.b[1123]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(789, A::sub(s.ad_value(906), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(919, 880, 789);s.store_scalar(922, 0.0);s.store_scalar(923, 0.0);s.store_scalar(920, 0.0);s.store_scalar(921, 0.0);s.store_scalar(924, 0.0);s.store_scalar(925, 0.0);s.b[1124] = (s.v[913] > 1e-6);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if s.b[1124] {s.store_mul(920, 918, 881);s.store_mul(921, 919, 882);s.store_add_scaled_inputs(922, 920, 1.0, 914, 2.0);s.store_add_scaled_inputs(923, 921, 1.0, 915, 2.0);s.store_add_scaled_inputs3_indices(924, 913, 2.0, 920, 1.0, 921, 1.0);}
        s.b[1125] = (((s.v[917]) as f64).abs() > 0.005);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (s.b[1124] && s.b[1125]) {s.store_add_scaled_products3_mixed_iiaiai(2, 922, 923, 1.0, A::offset(s.ad_value(912), 2.0), 923, 2.0, A::offset(s.ad_value(916), 2.0), 922, 2.0);s.store_div_scaled_product_by_product_indices(925, 917, 924, (-4.0), 913, 2, 1.0);}
        if (s.b[1124] && (!s.b[1125])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 917, 1.0, 917, 1.0, 917, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 922, 918, 1.0, 923, 919, 1.0, A::mul3(s.ad_value(922), s.ad_value(923), s.ad_value(913)), A::offset(A::mul(s.ad_value(913), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(925, 918, 919, 924, 1.0, 913, 3, 1.0);}
        s.store_ln(926, 913);s.b[1126] = ((s.v[914] / 2.0) < 80.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if s.b[1126] {s.store_ln_one_plus_exp_scaled_input(2, 914, 0.5);}
        if (!s.b[1126]) {s.store_scale(2, 914, 0.5);}
        s.store_scale(927, 2, 2.0);s.b[1127] = ((s.v[915] / 2.0) < 80.0);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if s.b[1127] {s.store_ln_one_plus_exp_scaled_input(3, 915, 0.5);}
        if (!s.b[1127]) {s.store_scale(3, 915, 0.5);}
        s.store_scale(928, 3, 2.0);s.store_sub(929, 928, 915);s.store_sub(930, 927, 914);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_products_indices(931, 266, 927, 1.0, 267, 929, 1.0);s.store_add_scaled_products_indices(932, 266, 928, 1.0, 267, 930, 1.0);s.store_div_add_scaled_inputs_rhs_indices(0, 913, 927, 1.0, 928, 1.0);s.store_mul(933, 927, 0);s.store_mul(934, 928, 0);s.store_mul_ad_product_rhs_mixed_ia(935, 927, 187, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_mul_ad_product_rhs_mixed_ia(936, 928, 188, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_mul_add_scaled_product_rhs_indices(2, 50, 929, 1.0, 51, 930, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(937, 3, 4);s.store_mul_ad_product_rhs(938, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(929)), 1.0), 1.0, s.ad_value(42), s.ad_value(930), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(933), s.ad_value(264)), 1.0), 1.0, s.ad_value(934), s.ad_value(265), 1.0)))));s.b[1128] = (s.v[56] == 0.0);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if s.b[1128] {s.store_scalar(4, 1.0);}
        s.b[1129] = (s.v[56] < 0.0);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if ((!s.b[1128]) && s.b[1129]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(913), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1128]) && (!s.b[1129])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(913), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_ad_affine_product_rhs(939, 268, s.ad_value(894), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(875))), A::sqrt_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(875))), 0.01)), 0.5, 0.0);s.store_mul_add_scaled_product_rhs_indices(940, 939, 54, 1.0, 913, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(941, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(931)), 1e-6)))), 1.0), 1.0, 938, 1.0, 38, 940, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs_product_mixed_aiii(942, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(932)), 1e-6)))), 1.0), 1.0, 938, 1.0, 39, 940, 1.0);s.store_div_scaled_product_mixed_iaa(943, 937, A::add(s.ad_value(935), s.ad_value(936)), 1.0, A::add(A::div(s.ad_value(935), s.ad_value(941)), A::div(s.ad_value(936), s.ad_value(942))), 1.0);s.b[1130] = (((s.v[910]) as f64).abs() > 0.007);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });s.b[1131] = (s.v[910] > 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if (s.b[1130] && s.b[1131]) {s.store_exp_neg_input(0, 910);s.store_div_mixed_ia(944, 910, A::sub_from_scalar(1.0, s.ad_value(0)));s.store_mul(945, 0, 944);s.store_add_offset_lhs_mixed_ai(946, A::ln(A::div(s.ad_value(880), A::mul(s.ad_value(913), s.ad_value(944)))), (-0.6931471805599), 883);}
        if (s.b[1130] && (!s.b[1131])) {s.store_exp(0, 910);s.store_div_scaled_value_offset_denominator(945, s.ad_value(910), 1.0, s.ad_value(0), (-1.0), 1.0);s.store_mul(944, 0, 945);s.store_add_offset_lhs_mixed_ai(946, A::ln(A::div(s.ad_value(880), A::mul(s.ad_value(913), s.ad_value(945)))), (-0.6931471805599), 884);}
        if s.b[1130] {s.store_div_scaled_inputs_mixed_ia(947, 910, -1.0, A::mul(s.ad_value(909), A::add_scaled_sub_value_product(1.0, s.ad_value(944), 1.0, s.ad_value(910), s.ad_value(882), (-1.0))), 1.0);s.store_div_scaled_value_by_product_mixed_iia(948, 910, 1.0, 909, A::add_scaled_sub_value_product(1.0, s.ad_value(945), 1.0, s.ad_value(910), s.ad_value(881), 1.0), 1.0);s.store_div_add_scaled_inputs_rhs_ad(949, 910, A::div_scaled_offset_numerator(A::mul(s.ad_value(945), s.ad_value(882)), 1.0, 0.5, s.ad_value(948), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(944), s.ad_value(881)), 1.0, 0.5, s.ad_value(947), 1.0), -1.0);}
        if (!s.b[1130]) {s.store_scale(0, 911, (0.5 * 0.1666666666667));s.store_scale(2, 910, 0.5);s.store_add_offset_lhs(944, 2, 1.0, 0);s.store_add_mixed_ai(945, A::sub_from_scalar(1.0, s.ad_value(2)), 0);s.store_scale(3, 2, 0.1666666666667);s.store_div_scalar_by_product_mixed_ia(947, 1.0, 909, A::add(A::offset(s.ad_value(882), 0.5), s.ad_value(3)), 1.0);s.store_div_scalar_by_product_mixed_ia(948, 1.0, 909, A::sub(A::offset(s.ad_value(881), 0.5), s.ad_value(3)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1130]) {s.store_add_scaled_inputs3_offset_mixed_aii(946, A::ln(A::div(s.ad_value(880), A::mul_sub_from_scalar_rhs(s.ad_value(913), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 883, 0.5, 884, 0.5, (-0.6931471805599));s.store_div_from_scalar_ad(949, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(909), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(909), 12.0, A::mul(s.ad_value(907), s.ad_value(908)), 1.0), 1.0, A::mul3(s.ad_value(909), A::sub(s.ad_value(881), s.ad_value(882)), s.ad_value(910)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(909), 0.25), s.ad_value(911), 0.3333333333333), 1.0, 4.0));}
        s.store_div_from_scalar(950, 1.0, 949);s.b[1132] = (s.v[913] > 1e-6);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if s.b[1132] {s.store_div_scaled_value_offset_denominator(951, s.ad_value(927), 100.0, s.ad_value(927), 100.0, 1.0);}
        s.b[1133] = (s.v[61] < 0.0);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (s.b[1132] && s.b[1133]) {s.store_div_from_scalar_sub_from_scalar_ad(952, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(951)));}
        if (s.b[1132] && (!s.b[1133])) {s.store_offset_mul(952, 61, 951, 1.0);}
        if s.b[1132] {s.store_div_scaled_value_offset_denominator(953, s.ad_value(928), 100.0, s.ad_value(928), 100.0, 1.0);}
        s.b[1134] = (s.v[62] < 0.0);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if (s.b[1132] && s.b[1134]) {s.store_div_from_scalar_sub_from_scalar_ad(954, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(953)));}
        if (s.b[1132] && (!s.b[1134])) {s.store_offset_mul(954, 62, 953, 1.0);}
        if s.b[1132] {s.store_sub_ad(955, A::div_scaled_product_by_product(s.ad_value(925), s.ad_value(924), 1.0, s.ad_value(922), s.ad_value(923), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(918), s.ad_value(922)), 1.0, A::div(s.ad_value(919), s.ad_value(923)), 1.0, s.ad_value(913), 1.0));s.store_div_scaled_product_offset_denominator_indices(956, 955, 913, 1.0, 955, 1.0, 1.0);s.store_sub(2, 949, 956);s.store_div_scaled_add_product_indices(957, 913, 1.0, 949, 946, 1.0, 2, 1.0);s.store_scaled_add_mixed_ia(957, 957, A::sqrt_square_offset(s.ad_value(957), 1e-6), 0.5);s.store_scaled_mul_ad(958, A::div(s.ad_value(871), s.ad_value(943)), A::add(s.ad_value(952), s.ad_value(954)), 0.5);s.store_sub_from_scalar_div_indices(959, 1.0, 913, 956);s.store_offset(960, 946, 1.0);s.store_mul_sub_mixed_iai(961, 957, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(956), 2.0, s.ad_value(913), 1.0), s.ad_value(950)), (-2.0)), 946);}
        s.b[1135] = (s.v[958] > 1e-14);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if (s.b[1132] && s.b[1135]) {s.store_div_from_scalar_square_ad(962, 2.0, s.ad_value(958));s.store_mul(963, 962, 959);s.store_add(964, 962, 961);s.store_mul(965, 962, 960);}
    }
}
