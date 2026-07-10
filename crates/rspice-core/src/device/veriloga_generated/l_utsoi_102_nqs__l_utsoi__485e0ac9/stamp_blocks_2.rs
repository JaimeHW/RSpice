#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
    ) {
        if (s.b[1107] && s.b[1108]) {s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);}
        s.b[1110] = (s.v[804] < (-0.005));s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if ((s.b[1107] && s.b[1108]) && s.b[1110]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1111] = (s.v[804] > 0.005);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (((s.b[1107] && s.b[1108]) && (!s.b[1110])) && s.b[1111]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if (((s.b[1107] && s.b[1108]) && (!s.b[1110])) && (!s.b[1111])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
    ) {
        if (((s.b[1107] && s.b[1108]) && (!s.b[1110])) && (!s.b[1111])) {s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1112] = (s.v[804] > 0.005);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if ((s.b[1107] && s.b[1108]) && s.b[1112]) {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1113] = (s.v[804] < (-0.005));s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (((s.b[1107] && s.b[1108]) && (!s.b[1112])) && s.b[1113]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if (((s.b[1107] && s.b[1108]) && (!s.b[1112])) && (!s.b[1113])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1114] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if ((s.b[1107] && s.b[1108]) && s.b[1114]) {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if ((s.b[1107] && s.b[1108]) && (!s.b[1114])) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1115] = (s.v[816] > 0.0);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if ((s.b[1107] && s.b[1108]) && s.b[1115]) {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if ((s.b[1107] && s.b[1108]) && (!s.b[1115])) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 916);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        if (s.b[1107] && s.b[1108]) {s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 916, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if (s.b[1107] && s.b[1108]) {s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(916, 916, 831);}
        s.store_mul(918, 911, 916);s.b[1116] = ((s.v[909] - s.v[916]) < 80.0);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if s.b[1116] {s.store_exp_sub(793, 909, 916);}
        if (!s.b[1116]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(922, 884, 793);s.store_sub_square_lhs(921, 918, 922);s.b[1117] = (s.v[922] <= 0.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if s.b[1117] {s.store_scalar(917, 1e-80);s.store_sub(919, 917, 918);s.store_div(920, 919, 912);}
        s.b[1118] = (s.v[921] < (-0.005));s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if ((!s.b[1117]) && s.b[1118]) {s.store_sqrt_abs_ad(807, s.ad_value(921));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));}
        s.b[1119] = (s.v[921] > 0.005);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if (((!s.b[1117]) && (!s.b[1118])) && s.b[1119]) {s.store_sqrt_abs_ad(807, s.ad_value(921));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);}
        if (((!s.b[1117]) && (!s.b[1118])) && (!s.b[1119])) {s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1120] = (((1.01 * s.v[918]) + s.v[808]) > 0.0);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        if ((!s.b[1117]) && s.b[1120]) {s.store_add(793, 918, 808);}
        s.b[1121] = ((s.v[922] * s.v[918]) < (((0.9 * s.v[918]) * s.v[918]) * s.v[793]));s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (((!s.b[1117]) && s.b[1120]) && s.b[1121]) {s.store_offset_div(917, 922, 793, 1e-80);s.store_sub(919, 917, 918);s.store_div(920, 919, 912);}
        s.b[1122] = (s.v[921] > 0.005);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if ((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && s.b[1122]) {s.store_sub_mixed_ai(794, A::ln(A::div_scaled_inputs(s.ad_value(921), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0)), 807);}
        s.b[1123] = (s.v[921] < (-0.005));s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if (((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && (!s.b[1122])) && s.b[1123]) {s.store_sin_scaled_input(795, 807, 0.5);s.store_ln_div_scaled_input_square_denominator(794, 921, -1.0, 795, 1.0);}
        if (((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && (!s.b[1122])) && (!s.b[1123])) {s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(920, 910, 1.0, 909, (-1.0), 916, 1.0, A::ln(s.ad_value(793)), 2.0, 794);s.store_mul(919, 912, 920);s.store_add(917, 918, 919);}
        s.b[1124] = (s.v[921] > 0.005);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        s.b[1125] = (((s.v[916] - s.v[909]) - s.v[807]) < 80.0);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if ((((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) && s.b[1125]) {s.store_exp_ad(795, A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0));}
        if ((((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) && (!s.b[1125])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(795, A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) {s.store_div(794, 795, 884);s.store_div_scaled_product_mixed_iia(793, 921, 794, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);}
        s.b[1126] = (s.v[921] < (-0.005));s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if ((((!s.b[1117]) && (!s.b[1120])) && (!s.b[1124])) && s.b[1126]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_value_by_product_mixed_iai(793, 921, -1.0, A::square(s.ad_value(794)), 922, 1.0);}
        if ((((!s.b[1117]) && (!s.b[1120])) && (!s.b[1124])) && (!s.b[1126])) {s.store_div_mixed_ai(793, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0396825396825397), 0.05), 0.3333333333333)), 922);}
        if ((!s.b[1117]) && (!s.b[1120])) {s.store_offset_div_scaled_inputs2_mixed_iia(917, 918, 1.0, 808, (-1.0), A::sub_from_scalar(1.0, s.ad_value(793)), 1.0, 1e-80);s.store_sub(919, 917, 918);s.store_div(920, 919, 912);}
        s.b[1127] = ((s.v[910] - s.v[920]) < 80.0);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if s.b[1127] {s.store_exp_sub(793, 910, 920);}
        if (!s.b[1127]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(910), s.ad_value(920)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(923, 884, 793);s.store_scalar(926, 0.0);s.store_scalar(927, 0.0);s.store_scalar(924, 0.0);s.store_scalar(925, 0.0);s.store_scalar(928, 0.0);s.store_scalar(929, 0.0);s.b[1128] = (s.v[917] > 1e-6);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if s.b[1128] {s.store_mul(924, 922, 885);s.store_mul(925, 923, 886);s.store_add_scaled_inputs(926, 924, 1.0, 918, 2.0);s.store_add_scaled_inputs(927, 925, 1.0, 919, 2.0);s.store_add_scaled_inputs3_indices(928, 917, 2.0, 924, 1.0, 925, 1.0);}
        s.b[1129] = (((s.v[921]) as f64).abs() > 0.005);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if (s.b[1128] && s.b[1129]) {s.store_add_scaled_products3_mixed_iiaiai(2, 926, 927, 1.0, A::offset(s.ad_value(916), 2.0), 927, 2.0, A::offset(s.ad_value(920), 2.0), 926, 2.0);s.store_div_scaled_product_by_product_indices(929, 921, 928, (-4.0), 917, 2, 1.0);}
        if (s.b[1128] && (!s.b[1129])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 921, 1.0, 921, 1.0, 921, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 926, 922, 1.0, 927, 923, 1.0, A::mul3(s.ad_value(926), s.ad_value(927), s.ad_value(917)), A::offset(A::mul(s.ad_value(917), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(929, 922, 923, 928, 1.0, 917, 3, 1.0);}
        s.store_ln(930, 917);s.b[1130] = ((s.v[918] / 2.0) < 80.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.store_ln_one_plus_exp_scaled_input(2, 918, 0.5);}
        if (!s.b[1130]) {s.store_scale(2, 918, 0.5);}
        s.store_scale(931, 2, 2.0);s.b[1131] = ((s.v[919] / 2.0) < 80.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_ln_one_plus_exp_scaled_input(3, 919, 0.5);}
        if (!s.b[1131]) {s.store_scale(3, 919, 0.5);}
        s.store_scale(932, 3, 2.0);s.store_sub(933, 932, 919);s.store_sub(934, 931, 918);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_products_indices(935, 270, 931, 1.0, 271, 933, 1.0);s.store_add_scaled_products_indices(936, 270, 932, 1.0, 271, 934, 1.0);s.store_div_add_scaled_inputs_rhs_indices(0, 917, 931, 1.0, 932, 1.0);s.store_mul(937, 931, 0);s.store_mul(938, 932, 0);s.store_mul_ad_product_rhs_mixed_ia(939, 931, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_mul_ad_product_rhs_mixed_ia(940, 932, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_mul_add_scaled_product_rhs_indices(2, 50, 933, 1.0, 51, 934, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(941, 3, 4);s.store_mul_ad_product_rhs(942, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(933)), 1.0), 1.0, s.ad_value(42), s.ad_value(934), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(937), s.ad_value(268)), 1.0), 1.0, s.ad_value(938), s.ad_value(269), 1.0)))));s.b[1132] = (s.v[56] == 0.0);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if s.b[1132] {s.store_scalar(4, 1.0);}
        s.b[1133] = (s.v[56] < 0.0);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if ((!s.b[1132]) && s.b[1133]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1132]) && (!s.b[1133])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_ad_affine_product_rhs(943, 272, s.ad_value(898), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879))), A::sqrt_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879))), 0.01)), 0.5, 0.0);s.store_mul_add_scaled_product_rhs_indices(944, 943, 54, 1.0, 917, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(945, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(935)), 1e-6)))), 1.0), 1.0, 942, 1.0, 38, 944, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_inputs_product_mixed_aiii(946, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(936)), 1e-6)))), 1.0), 1.0, 942, 1.0, 39, 944, 1.0);s.store_div_scaled_product_mixed_iaa(947, 941, A::add(s.ad_value(939), s.ad_value(940)), 1.0, A::add(A::div(s.ad_value(939), s.ad_value(945)), A::div(s.ad_value(940), s.ad_value(946))), 1.0);s.b[1134] = (((s.v[914]) as f64).abs() > 0.007);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });s.b[1135] = (s.v[914] > 0.0);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if (s.b[1134] && s.b[1135]) {s.store_exp_neg_input(0, 914);s.store_div_mixed_ia(948, 914, A::sub_from_scalar(1.0, s.ad_value(0)));s.store_mul(949, 0, 948);s.store_add_offset_lhs_mixed_ai(950, A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(948)))), (-0.6931471805599), 887);}
        if (s.b[1134] && (!s.b[1135])) {s.store_exp(0, 914);s.store_div_scaled_value_offset_denominator(949, s.ad_value(914), 1.0, s.ad_value(0), (-1.0), 1.0);s.store_mul(948, 0, 949);s.store_add_offset_lhs_mixed_ai(950, A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(949)))), (-0.6931471805599), 888);}
        if s.b[1134] {s.store_div_scaled_inputs_mixed_ia(951, 914, -1.0, A::mul(s.ad_value(913), A::add_scaled_sub_value_product(1.0, s.ad_value(948), 1.0, s.ad_value(914), s.ad_value(886), (-1.0))), 1.0);s.store_div_scaled_value_by_product_mixed_iia(952, 914, 1.0, 913, A::add_scaled_sub_value_product(1.0, s.ad_value(949), 1.0, s.ad_value(914), s.ad_value(885), 1.0), 1.0);s.store_div_add_scaled_inputs_rhs_ad(953, 914, A::div_scaled_offset_numerator(A::mul(s.ad_value(949), s.ad_value(886)), 1.0, 0.5, s.ad_value(952), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(948), s.ad_value(885)), 1.0, 0.5, s.ad_value(951), 1.0), -1.0);}
        if (!s.b[1134]) {s.store_scale(0, 915, (0.5 * 0.1666666666667));s.store_scale(2, 914, 0.5);s.store_add_offset_lhs(948, 2, 1.0, 0);s.store_add_mixed_ai(949, A::sub_from_scalar(1.0, s.ad_value(2)), 0);s.store_scale(3, 2, 0.1666666666667);s.store_div_scalar_by_product_mixed_ia(951, 1.0, 913, A::add(A::offset(s.ad_value(886), 0.5), s.ad_value(3)), 1.0);s.store_div_scalar_by_product_mixed_ia(952, 1.0, 913, A::sub(A::offset(s.ad_value(885), 0.5), s.ad_value(3)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if s.b[1148] {s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1149] = (s.v[804] > 0.005);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if ((!s.b[1148]) && s.b[1149]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1148]) && (!s.b[1149])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
    ) {
        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.store_mul(802, 911, 980);s.store_mul(832, 912, 981);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.store_sub_square_lhs(837, 802, 804);s.b[1154] = (s.v[837] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 979, 1.0, 909, -1.0, 980, 1.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);}
        let (t2,) = {
    if s.b[1154] {
        let t0: f64 = (s.v[909] - s.v[980]);let t1: f64 = (t0 - s.v[796]);
        (t1,)
    } else {
        (s.v[838],)
    }
};
        s.store_scalar(838, t2);s.b[1155] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (s.b[1154] && s.b[1155]) {s.store_sub_div_rhs_indices(980, 980, 828, 829);}
        s.store_mul(802, 911, 980);s.store_mul(832, 912, 981);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.b[1156] = (s.v[804] < (-0.005));s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        s.b[1157] = (s.v[804] > 0.005);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if ((!s.b[1156]) && s.b[1157]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        if ((!s.b[1156]) && (!s.b[1157])) {s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(809, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(804, 804, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(825), s.ad_value(808), 1.0, s.ad_value(802), s.ad_value(832), 1.0), 1.0, s.ad_value(804), 1.0, A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0), 1.0));s.store_sub_square_lhs(837, 802, 804);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
    ) {
        s.b[1158] = (s.v[837] > 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 979, 1.0, 909, -1.0, 980, 1.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);}
        let (t5,) = {
    if s.b[1158] {
        let t3: f64 = (s.v[909] - s.v[980]);let t4: f64 = (t3 - s.v[796]);
        (t4,)
    } else {
        (s.v[838],)
    }
};
        s.store_scalar(838, t5);s.b[1159] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if (s.b[1158] && s.b[1159]) {s.store_sub_div_rhs_indices(980, 980, 828, 829);}
        s.store_mul(802, 911, 980);s.b[1160] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1160]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1161] = (s.v[804] < (-0.005));s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1162] = (s.v[804] > 0.005);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if ((!s.b[1161]) && s.b[1162]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
    ) {
        if ((!s.b[1161]) && s.b[1162]) {s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1161]) && (!s.b[1162])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1163] = (s.v[804] > 0.005);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1164] = (s.v[804] < (-0.005));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((!s.b[1163]) && s.b[1164]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1163]) && (!s.b[1164])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1165] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1165]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
    ) {
        if (!s.b[1165]) {s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1166] = (s.v[816] > 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1166]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.store_mul(802, 911, 980);s.b[1167] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if (!s.b[1167]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1168] = (s.v[804] < (-0.005));s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
    ) {
        if s.b[1168] {s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1169] = (s.v[804] > 0.005);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((!s.b[1168]) && s.b[1169]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1168]) && (!s.b[1169])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1170] = (s.v[804] > 0.005);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);}
    }
}
