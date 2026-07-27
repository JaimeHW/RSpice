#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
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
    pub(super) fn stamp_transient_block_65(
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
    pub(super) fn stamp_transient_block_66(
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
    pub(super) fn stamp_transient_block_67(
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
    pub(super) fn stamp_transient_block_68(
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
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
    ) {
        if s.b[1168] {s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1169] = (s.v[804] > 0.005);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((!s.b[1168]) && s.b[1169]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1168]) && (!s.b[1169])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1170] = (s.v[804] > 0.005);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1170] {s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1171] = (s.v[804] < (-0.005));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((!s.b[1170]) && s.b[1171]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1170]) && (!s.b[1171])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1172] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if s.b[1172] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1172]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1173] = (s.v[816] > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if s.b[1173] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1173]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 980);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);s.b[1174] = (p[10] == 1.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });s.b[1175] = (((s.v[831]) as f64).abs() > 0.01);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
    ) {
        if (s.b[1174] && s.b[1175]) {s.store_mul(802, 911, 980);}
        s.b[1176] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1176]) {s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));}
        if ((s.b[1174] && s.b[1175]) && (!s.b[1176])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1174] && s.b[1175]) {s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);}
        s.b[1177] = (s.v[804] < (-0.005));s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if ((s.b[1174] && s.b[1175]) && s.b[1177]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1178] = (s.v[804] > 0.005);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && s.b[1178]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && (!s.b[1178])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
    ) {
        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && (!s.b[1178])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
    ) {
        if (s.b[1174] && s.b[1175]) {s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 980, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(980, 980, 831);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && s.b[1189]) {s.store_sub_mixed_ai(794, A::ln(A::div_scaled_inputs(s.ad_value(985), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0)), 807);}
        s.b[1190] = (s.v[985] < (-0.005));s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && s.b[1190]) {s.store_sin_scaled_input(795, 807, 0.5);s.store_ln_div_scaled_input_square_denominator(794, 985, -1.0, 795, 1.0);}
        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && (!s.b[1190])) {s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(981, 910, 1.0, 909, (-1.0), 980, 1.0, A::ln(s.ad_value(793)), 2.0, 794);s.store_mul(984, 912, 981);s.store_add(982, 983, 984);}
        s.b[1191] = (s.v[985] > 0.005);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });s.b[1192] = ((((s.v[980] + s.v[979]) - s.v[909]) - s.v[807]) < 80.0);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1196] = (((s.v[985]) as f64).abs() > 0.005);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1195] && s.b[1196]) {s.store_add_scaled_products3_mixed_iiaiai(2, 990, 991, 1.0, A::offset(s.ad_value(980), 2.0), 991, 2.0, A::offset(s.ad_value(981), 2.0), 990, 2.0);s.store_div_scaled_product_by_product_indices(993, 985, 992, (-4.0), 982, 2, 1.0);}
        if (s.b[1195] && (!s.b[1196])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 985, 1.0, 985, 1.0, 985, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 990, 986, 1.0, 991, 987, 1.0, A::mul3(s.ad_value(990), s.ad_value(991), s.ad_value(982)), A::offset(A::mul(s.ad_value(982), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(993, 986, 987, 992, 1.0, 982, 3, 1.0);}
        s.store_add_mixed_ia(994, 979, A::ln(s.ad_value(982)));s.store_scaled_add(995, 917, 982, 0.5);s.store_sub(996, 994, 930);s.store_scalar(999, 1.0);s.b[1197] = (p[9] > 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if s.b[1197] {s.store_div_scaled_inputs2_indices(997, 918, 0.5, 983, 0.5, 911, 1.0);s.store_scaled_add_offset_sqrt_square_offset(997, 997, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(997), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 1.0, 250, 0.5);s.store_mul_square_lhs(998, 1, 227);s.store_sub_from_scalar_div_indices(999, 1.0, 998, 997);}
        s.b[1198] = ((s.v[983] / 2.0) < 80.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.store_ln_one_plus_exp_scaled_input(2, 983, 0.5);}
        if (!s.b[1198]) {s.store_scale(2, 983, 0.5);}
        s.store_scale(1000, 2, 2.0);s.b[1199] = ((s.v[984] / 2.0) < 80.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_ln_one_plus_exp_scaled_input(3, 984, 0.5);}
        if (!s.b[1199]) {s.store_scale(3, 984, 0.5);}
        s.store_scale(1001, 3, 2.0);s.store_sub(1002, 1001, 984);s.store_sub(1003, 1000, 983);s.store_add_scaled_products_indices(1004, 270, 1000, 1.0, 271, 1002, 1.0);s.store_add_scaled_products_indices(1005, 270, 1001, 1.0, 271, 1003, 1.0);s.store_scaled_add(1006, 931, 1000, 0.5);s.store_scaled_add(1007, 932, 1001, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1006), s.ad_value(1007));s.store_mul3_lhs(1008, 995, 1006, 0);s.store_mul3_lhs(1009, 995, 1007, 0);s.store_scaled_add(1010, 933, 1002, 0.5);s.store_scaled_add(1011, 934, 1003, 0.5);s.store_scaled_add(1012, 935, 1004, 0.5);s.store_scaled_add(1013, 936, 1005, 0.5);s.store_mul_product3_mixed_iiia(1014, 999, 1006, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1015, 1007, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_add(1016, 1014, 1015);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1010, 1.0, 51, 1011, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
    ) {
        s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1017, 3, 4);s.store_mul_ad_product_rhs(1018, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1010)), 1.0), 1.0, s.ad_value(42), s.ad_value(1011), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1008), s.ad_value(268)), 1.0), 1.0, s.ad_value(1009), s.ad_value(269), 1.0)))));s.b[1200] = (s.v[56] == 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_scalar(4, 1.0);}
        s.b[1201] = (s.v[56] < 0.0);s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if ((!s.b[1200]) && s.b[1201]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1200]) && (!s.b[1201])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_add_scaled_product_rhs_indices(1019, 943, 54, 1.0, 995, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1020, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1012)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 38, 1019, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1021, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1013)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 39, 1019, 1.0);s.store_div_scaled_product_add_scaled_denominator(1022, 1017, 1016, 1.0, A::div(s.ad_value(1014), s.ad_value(1020)), 1.0, A::div(s.ad_value(1015), s.ad_value(1021)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1023, 1.0, 995, 4.0);s.b[1202] = (s.v[65] > 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1009, 1.0);}
        if (!s.b[1202]) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1009, 1.0);}
        s.store_mul3_lhs(1024, 995, 1023, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_ln_mixed_ia(1025, 1024, A::offset(A::div_scaled_inputs2(s.ad_value(339), 1.0, s.ad_value(979), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(995), s.ad_value(995)), 1.0, s.ad_value(66), s.ad_value(227), 1.0), 1.0), 1.0));s.store_mul(1026, 877, 1025);s.store_div_from_scalar_offset_ad(1027, 1.0, A::mul_offset_rhs(s.ad_value(1026), s.ad_value(1026), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(955, s.ad_value(1006), 100.0, s.ad_value(1006), 100.0, 1.0);s.b[1203] = (s.v[61] < 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
        if s.b[1203] {s.store_div_from_scalar_sub_from_scalar_ad(956, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(955)));}
        if (!s.b[1203]) {s.store_offset_mul(956, 61, 955, 1.0);}
        s.store_div_scaled_value_offset_denominator(957, s.ad_value(1007), 100.0, s.ad_value(1007), 100.0, 1.0);s.b[1204] = (s.v[62] < 0.0);s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if s.b[1204] {s.store_div_from_scalar_sub_from_scalar_ad(958, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(957)));}
        if (!s.b[1204]) {s.store_offset_mul(958, 62, 957, 1.0);}
        s.store_mul_ad_affine_product_rhs(1028, 875, s.ad_value(996), A::add(s.ad_value(956), s.ad_value(958)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1029, 1028, 1.0, 1022, 1027, 1.0);s.store_square(1030, 1029);s.store_sqrt_offset_input(1031, 1030, 1.0);s.store_div_scaled_offset_numerator_indices(1032, 1030, 1.5, 1.0, 1031, 1.0);s.b[1205] = (p[13] > 0.0);s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if s.b[1205] {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 258, 0.6, 1006, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 258, 0.6, 1007, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1033, A::mul(s.ad_value(911), s.ad_value(2)), 1.0, 1.0, 892, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1034, A::mul(s.ad_value(912), s.ad_value(3)), 1.0, 1.0, 893, 1.0);}
        if (!s.b[1205]) {s.store_scalar(1033, 1.0);s.store_scalar(1034, 1.0);}
        s.b[1206] = (s.v[917] > 1e-6);s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });s.b[1207] = (s.v[982] > 1e-6);s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });s.b[1208] = (((s.v[991]) as f64).abs() < 0.01);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if ((s.b[1206] && s.b[1207]) && s.b[1208]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(980), 2.0), 1.0, 990, 0.5, A::offset(s.ad_value(981), 2.0), 990, 1.0);s.store_mul(2, 0, 991);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 984, 1.0, A::mul3_scaled_output(s.ad_value(985), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(990))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(981), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1035, A::div_scaled_add_product(s.ad_value(986), (-1.0), s.ad_value(993), s.ad_value(982), 1.0, s.ad_value(990), 1.0), 1.0, 2, (-1.0), 982, 1.0);s.store_div_scaled_product_offset_denominator_indices(1036, 1035, 982, 1.0, 1035, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
    ) {
        if ((s.b[1206] && s.b[1207]) && (!s.b[1208])) {s.store_sub_ad(1035, A::div_scaled_product_by_product(s.ad_value(993), s.ad_value(992), 1.0, s.ad_value(990), s.ad_value(991), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(986), s.ad_value(990)), 1.0, A::div(s.ad_value(987), s.ad_value(991)), 1.0, s.ad_value(982), 1.0));s.store_div_scaled_product_offset_denominator_indices(1036, 1035, 982, 1.0, 1035, 1.0, 1.0);}
        if (s.b[1206] && (!s.b[1207])) {s.copy_ad(1036, 953);}
        if s.b[1206] {s.store_sub(2, 1036, 960);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1209] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if (s.b[1206] && s.b[1209]) {s.store_sub(4, 982, 917);s.store_add_scaled_product_indices(1037, 4, 1.0, 1036, 996, (-1.0));s.store_add_scaled_product_indices(1038, 4, 1.0, 960, 996, (-1.0));s.store_sqrt_square_add(1039, 1037, 3);s.store_sqrt_square_add(1040, 1038, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1041, 0.25, 2, A::add_scaled_products3(s.ad_value(1040), s.ad_value(1037), 1.0, s.ad_value(1039), s.ad_value(1038), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(1040), 1.0, A::add(s.ad_value(1037), s.ad_value(1039)), 1.0)), 1.0));}
        if (s.b[1206] && (!s.b[1209])) {s.store_mul(4, 996, 2);s.store_div_scaled_product3_mixed_iiia(1041, 996, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (!s.b[1206]) {s.copy_ad(1036, 953);s.store_scalar(1041, 0.0);}
        s.store_add_scaled_inputs3_mixed_aii(1042, A::add_scaled_product(s.ad_value(1041), 1.0, s.ad_value(995), s.ad_value(996), 1.0), 1.0, 917, 1.0, 982, -1.0);s.b[1210] = (s.v[917] > 1e-6);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });s.b[1211] = (s.v[1042] > 1e-30);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if (s.b[1210] && s.b[1211]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 926, A::div(s.ad_value(922), s.ad_value(917)), 1.0, 929, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1044, 990, A::div(s.ad_value(986), s.ad_value(982)), 1.0, 993, -1.0);s.store_div_scaled_inputs2_indices(1045, 1043, 1.0, 1044, (-1.0), 1042, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1046, 927, A::div(s.ad_value(923), s.ad_value(917)), 1.0, 929, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1047, 991, A::div(s.ad_value(987), s.ad_value(982)), 1.0, 993, -1.0);s.store_div_scaled_inputs2_indices(1048, 1046, 1.0, 1047, (-1.0), 1042, 1.0);}
        if (s.b[1210] && (!s.b[1211])) {s.store_scalar(1045, 0.0);s.store_scalar(1048, 0.0);}
        if (!s.b[1210]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1049, 948, A::div(s.ad_value(885), s.ad_value(951)), (-2.0), 954, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1050, 949, A::div(s.ad_value(886), s.ad_value(952)), (-2.0), 954, (-2.0));s.store_mul_sub_lhs(0, 1050, 1049, 954);s.store_mul(2, 1049, 885);s.store_mul(3, 1050, 886);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(948), s.ad_value(885), 2.0, s.ad_value(949), s.ad_value(886), 2.0), 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1210]) {s.store_div_scaled_inputs3_mixed_iiai(1051, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(951)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1052, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(952)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1045, 951, 954, -1.0, 1051, 951, -1.0);s.store_mul_add_scaled_product_rhs_indices(1048, 952, 954, -1.0, 1052, 952, -1.0);}
        s.store_mul(1053, 1045, 1032);s.store_mul(1054, 1048, 1032);s.store_scaled_sub(1055, 983, 918, 0.5);s.store_scaled_sub(1056, 984, 919, 0.5);s.store_mul(1057, 1055, 1053);s.store_mul(1058, 1056, 1054);s.copy_ad(383, 879);s.copy_ad(384, 883);s.copy_ad(385, 884);s.copy_ad(386, 885);s.copy_ad(387, 886);s.copy_ad(388, 913);s.copy_ad(389, 914);s.copy_ad(390, 898);s.copy_ad(391, 897);s.copy_ad(392, 916);s.copy_ad(393, 901);s.copy_ad(394, 902);s.copy_ad(395, 903);s.copy_ad(396, 904);s.copy_ad(397, 905);s.copy_ad(398, 908);s.copy_ad(399, 910);s.copy_ad(400, 909);s.copy_ad(401, 911);s.copy_ad(402, 912);s.copy_ad(403, 917);s.copy_ad(404, 918);s.copy_ad(405, 919);s.copy_ad(406, 930);s.copy_ad(407, 960);s.copy_ad(408, 983);s.copy_ad(409, 984);s.copy_ad(411, 979);s.copy_ad(412, 980);s.copy_ad(413, 982);s.copy_ad(414, 994);s.copy_ad(415, 995);s.copy_ad(416, 999);s.copy_ad(417, 1006);s.copy_ad(418, 1007);s.copy_ad(419, 1008);s.copy_ad(420, 1009);s.copy_ad(421, 1016);s.copy_ad(422, 1022);s.copy_ad(423, 1023);s.copy_ad(424, 1025);s.copy_ad(425, 1027);s.copy_ad(426, 1031);s.store_scalar(427, s.v[1028]);s.copy_ad(428, 1030);s.copy_ad(429, 1032);s.copy_ad(430, 1033);s.copy_ad(431, 1034);s.copy_ad(432, 1036);s.copy_ad(433, 1042);s.copy_ad(434, 1053);s.copy_ad(435, 1045);s.copy_ad(436, 1055);s.copy_ad(437, 1056);s.copy_ad(438, 1057);s.copy_ad(439, 1058);s.store_div_scaled_inputs_mixed_ia(342, 421, p[35], A::add(s.ad_value(417), s.ad_value(418)), 1.0);s.store_mul_add_scaled_product_rhs_indices(343, 424, 63, 1.0, 275, 423, 1.0);s.store_mul_scale_offset_mixed_ia(344, 425, A::mul_offset_rhs(s.ad_value(343), s.ad_value(343), 1.0), 1.0, 1.0);s.store_mul3_lhs(345, 422, 425, 426);s.b[1212] = (p[13] > 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if s.b[1212] {s.store_div_scaled_inputs2_mixed_iia(346, 417, 1.0, 418, 1.0, A::add(A::div(s.ad_value(417), s.ad_value(430)), A::div(s.ad_value(418), s.ad_value(431))), 1.0);}
        if (!s.b[1212]) {s.store_scalar(346, 1.0);}
        s.store_mul_square_lhs(347, 226, 342);
    }
}
