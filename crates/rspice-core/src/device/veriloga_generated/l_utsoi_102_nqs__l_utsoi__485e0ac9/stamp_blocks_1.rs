#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[787] = (p.p9 > 0.0);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[787]) {s.store_mul_add_mixed_iai(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), 235);}
        s.b[788] = (p.p10 == 1.0);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[788]) {s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        if s.b[785] {s.store_scalar(258, 0.0);}
        s.b[789] = (p.p13 > 0.0);s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });s.b[790] = (p.p14 == 1.0);s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if ((s.b[785] && s.b[789]) && s.b[790]) {s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if ((s.b[785] && s.b[789]) && (!s.b[790])) {s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        if s.b[785] {s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 249);s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_ln(295, 222);s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);s.store_mul(38, 191, 296);s.store_mul(39, 192, 296);s.store_exp_mul(297, 48, 295);s.store_mul(46, 193, 297);s.store_exp_mul(298, 49, 295);s.store_mul(47, 194, 298);s.store_exp_mul(299, 43, 295);s.store_mul(33, 195, 299);s.store_exp_mul(300, 45, 295);s.store_mul(44, 196, 300);s.store_exp_mul(301, 52, 295);s.store_mul(50, 197, 301);s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);s.store_mul(267, 0, 46);s.store_exp_mul(302, 55, 295);s.store_mul(53, 198, 302);s.store_scaled_mul(272, 53, 226, 2.0);s.store_exp_mul(303, 60, 295);s.store_mul3_lhs(59, 199, 303, 296);s.store_mul(273, 59, 226);s.store_mul3_lhs(147, 200, 303, 296);s.store_mul(274, 147, 226);s.store_mul(275, 64, 227);s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);s.store_mul(68, 201, 304);s.store_mul(69, 202, 304);s.store_mul(70, 203, 304);s.store_mul(71, 204, 304);s.store_mul(72, 205, 304);s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);s.store_mul(73, 206, 304);s.store_mul(74, 207, 304);s.store_scale(283, 233, 0.5);s.store_mul(284, 75, 226);s.store_mul(285, 75, 223);s.store_div_from_scalar_offset_product(286, 1.0, 88, 236, 1.0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), 0.01, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[785] {s.store_mul3_lhs(91, 208, 277, 0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 209, 277, 0);s.store_mul_exp_mixed_ia(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));s.store_mul(287, 116, 226);s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);s.store_ln_div(292, 118, 252);s.store_scaled_mul(293, 119, 226, 1.25e-6);s.store_exp_mul(305, 169, 295);s.store_mul(168, 214, 305);}
        let (t8,) = {
    if s.b[785] {
        let t6: f64 = (4.0 * 1.3806488e-23);let t7: f64 = (t6 * s.v[217]);
        (t7,)
    } else {
        (s.v[306],)
    }
};
        s.store_scalar(306, t8);
        let (ta,) = {
    if s.b[785] {
        let t9: f64 = (s.v[171] * s.v[306]);
        (t9,)
    } else {
        (s.v[307],)
    }
};
        s.store_scalar(307, ta);s.b[791] = (p.p14 == 1.0);s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });
        if s.b[791] {s.store_voltage(330, ctx, nodes, Some(9), Some(6));s.store_voltage(702, ctx, nodes, Some(7), Some(6));s.store_voltage(331, ctx, nodes, Some(6), Some(8));}
        if (!s.b[791]) {s.store_scaled_voltage(330, ctx, nodes, Some(9), Some(6), -1.0);s.store_scaled_voltage(702, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(331, ctx, nodes, Some(6), Some(8), -1.0);}
        s.store_neg(703, 702);s.store_add(332, 330, 703);s.store_add(333, 702, 331);s.b[792] = (s.v[702] < 0.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        if s.b[792] {s.store_scalar(334, (-1.0));s.copy_ad(336, 703);s.copy_ad(335, 332);s.copy_ad(337, 333);}
        if (!s.b[792]) {s.store_scalar(334, 1.0);s.copy_ad(336, 702);s.copy_ad(335, 330);s.copy_ad(337, 331);}
        s.store_add(338, 335, 337);s.store_mul(339, 336, 227);s.store_mul_scale_offset_mixed_ia(340, 227, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(341, 339, 340, 0.5);s.copy_ad(869, 21);s.copy_ad(870, 22);s.copy_ad(871, 27);s.copy_ad(872, 28);s.copy_ad(873, 31);s.copy_ad(874, 32);s.copy_ad(875, 273);s.copy_ad(876, 215);s.copy_ad(877, 63);s.store_sub_mixed_ai(878, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub(s.ad_value(335), s.ad_value(869)), s.ad_value(227), 1.0), 234);s.store_add_scaled_product_mixed_iai(879, 341, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(870), 1.0), 227, 1.0);s.store_sub(880, 879, 234);s.b[1059] = (p.p2 > 0.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if s.b[1059] {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(881, 246, 1.0, 1.0, A::offset(s.ad_value(247), 1.0), 1.0);s.store_ln(882, 881);}
        s.b[1060] = (s.v[882] > 1e-8);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1060]) {s.store_div_scaled_product_offset_denominator_mixed_iai(883, 882, A::offset(s.ad_value(881), 1.0), 2.0, 881, (-1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        if (s.b[1059] && (!s.b[1060])) {s.store_scaled_offset(883, 882, 2.0, 2.0);}
        if s.b[1059] {s.store_div_square_rhs(884, 253, 245);s.store_div_from_scalar(885, 1.0, 246);s.store_div_from_scalar(886, 1.0, 247);s.store_div_from_scalar_add_ad(913, 1.0, A::offset(s.ad_value(885), 1.0), s.ad_value(886));s.store_mul_sub_rhs(914, 913, 878, 880);s.store_add_scaled_product_indices(887, 878, 1.0, 914, 885, (-1.0));s.store_add_scaled_product_indices(888, 880, 1.0, 914, 886, 1.0);s.store_div_from_scalar_offset_input(793, 1.0, 246, 1.0);s.store_div_from_scalar_offset_input(794, 1.0, 247, 1.0);s.store_offset_ln_ad(796, A::div_scaled_product(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);s.store_offset_ln_ad(797, A::div_scaled_product(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(246), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);}
        s.b[1061] = (((s.v[796] - s.v[887]) / 1.5) < 80.0);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1061]) {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.6666666666666666, s.ad_value(887), 0.6666666666666666));}
        if (s.b[1059] && (!s.b[1061])) {s.store_scaled_sub(795, 796, 887, 0.6666666666666666);}
        if s.b[1059] {s.store_sub_scaled_inputs(800, 796, 1.0, 795, 1.5);s.store_mul_add_scaled_product_rhs_indices(799, 794, 800, 1.0, 247, 880, 1.0);}
        s.b[1062] = (((s.v[797] - s.v[799]) / 1.5) < 80.0);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1062]) {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.6666666666666666, s.ad_value(799), 0.6666666666666666));}
        if (s.b[1059] && (!s.b[1062])) {s.store_scaled_sub(795, 797, 799, 0.6666666666666666);}
        if s.b[1059] {s.store_sub_scaled_inputs(1, 797, 1.0, 795, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 880);s.store_sub(845, 2, 3);}
        s.b[1063] = ((((-s.v[266])) as f64).abs() < 80.0);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1063]) {s.store_exp_neg_input(846, 266);}
        s.b[1064] = ((-s.v[266]) < (-80.0));s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if ((s.b[1059] && (!s.b[1063])) && s.b[1064]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(846, 1.80485e-35, A::neg(A::neg(s.ad_value(266))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1063])) && (!s.b[1064])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(846, A::neg(s.ad_value(266)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1065] = (((s.v[845]) as f64).abs() <= s.v[265]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1065]) {s.store_scaled_square(843, 264, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 845, 264, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(845), 1.0, s.ad_value(846)), s.ad_value(260), s.ad_value(843)), 1.0));}
        s.b[1066] = (s.v[845] < (-s.v[265]));s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_neg(847, 845);s.store_scaled_mul(848, 847, 264, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(849, 848, 10.0, (-6.0), 64.0, 0.5);s.store_sub(842, 847, 849);s.store_add_scaled_square_product_mixed_iia(850, 842, 1.0, 261, A::offset(s.ad_value(849), 1.0), 1.0);s.store_sub_scaled_inputs(852, 842, 2.0, 261, 1.0);s.store_sub_ln_mul_lhs(853, 850, 262, 849);s.store_add(840, 850, 852);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_product(s.ad_value(850), (-1.0), s.ad_value(852), s.ad_value(852), 0.5), 1.0);s.store_add_mixed_ia(854, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::sub_scaled_inputs(A::square(s.ad_value(852)), 0.3333333333333, s.ad_value(850), 1.0))), 1.0));}
        s.b[1067] = (s.v[854] < 80.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && s.b[1067]) {s.store_exp(855, 854);}
        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && (!s.b[1067])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(855, 854, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_div_from_scalar(856, 1.0, 855);s.store_div_from_scalar_offset_square(842, 1.0, 854, 2.0);s.store_mul_square_lhs(857, 854, 842);s.store_mul3_affine_lhs(858, 854, 842, 4.0, 0.0, 842);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), 842, 842);s.store_sub(842, 847, 854);s.store_mul(843, 846, 856);s.store_add_scaled_product_mixed_iia(860, 842, 2.0, 261, A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(843), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(846), 1.0, s.ad_value(858)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(854), (-1.0), s.ad_value(843), 1.0, (-1.0)), 1.0, s.ad_value(846), A::sub(A::offset(s.ad_value(854), (-1.0)), s.ad_value(857)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(855), 1.0, s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));s.store_sub_scaled_inputs_mixed_ia(4, 854, -1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_offset_scaled_input(862, 1.0, 260, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(863, 862, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(862)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(864, 845, 264, A::offset(A::mul(s.ad_value(863), s.ad_value(845)), 1.0));}
        s.b[1068] = ((-s.v[864]) > (-80.0));s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1068]) {s.store_exp_neg_input(842, 864);}
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1068])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(864))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_sub_from_scalar(865, 1.0, 842);s.store_add_scaled_inputs_product_mixed_iiia(866, 845, 1.0, 261, 0.5, 260, A::sqrt(A::add_scaled_inputs3(s.ad_value(845), 1.0, s.ad_value(261), 0.25, s.ad_value(865), -1.0)), (-1.0));s.store_offset(867, 266, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_sub_ad(849, A::add_scaled_inputs3(s.ad_value(866), 0.5, s.ad_value(867), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(866), s.ad_value(867)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(867), 0.5, A::sqrt_square_offset(s.ad_value(867), 5.0), 0.5));s.store_sub(842, 845, 849);s.store_exp_neg_input(843, 849);s.store_div_from_scalar_offset_square(844, 1.0, 849, 2.0);s.store_mul_square_lhs(857, 849, 844);s.store_mul3_affine_lhs(858, 849, 844, 4.0, 0.0, 844);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(844), 8.0, s.ad_value(857), 12.0), 844, 844);s.store_max_from_scalar_ad(850, 1e-40, A::add_scaled_square_product(s.ad_value(842), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(843), s.ad_value(849)), (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(849), 1.0), s.ad_value(857)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(851, 1.0, 261, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(852, 842, 2.0, 261, A::add_scaled_sub_value_product(1.0, s.ad_value(843), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(853, 266, 1.0, 849, (-1.0), A::ln(A::div(s.ad_value(850), s.ad_value(261))), 1.0);s.store_add(840, 850, 852);s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_products(s.ad_value(852), s.ad_value(852), 0.5, s.ad_value(850), s.ad_value(851), (-1.0)), 1.0);s.store_add_mixed_ia(868, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::add_scaled_square_product(s.ad_value(852), 0.3333333333333, s.ad_value(850), s.ad_value(851), (-1.0)))), 1.0));}
        s.b[1069] = (s.v[868] < 80.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1069]) {s.store_exp(855, 868);s.store_div_from_scalar(856, 1.0, 855);s.store_mul(855, 846, 855);}
        s.b[1070] = (s.v[868] > (s.v[266] - 80.0));s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && s.b[1070]) {s.store_exp_sub(855, 868, 266);s.store_div(856, 846, 855);}
        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && (!s.b[1070])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(855, 1.80485e-35, A::sub(s.ad_value(266), s.ad_value(868)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(856, 1.80485e-35, 868, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_offset_square(842, 1.0, 868, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_mul_square_lhs(857, 868, 842);s.store_mul3_affine_lhs(858, 868, 842, 4.0, 0.0, 842);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), 842, 842);s.store_sub(842, 845, 868);s.store_add_scaled_product_mixed_iia(860, 842, 2.0, 261, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(856)), 1.0, s.ad_value(855), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(856), 1.0, s.ad_value(868), 1.0, s.ad_value(855), 1.0, (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(868), 1.0), s.ad_value(857)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(856), 1.0, s.ad_value(855), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));s.store_add_scaled_inputs_mixed_ia(4, 868, 1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);}
        if s.b[1059] {s.store_mul_add_rhs(889, 0, 4, 3);}
        if (!s.b[1059]) {s.copy_ad(889, 880);}
        s.store_mul_sub_rhs(0, 248, 878, 889);s.b[1071] = (p.p13 > 0.0);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if s.b[1071] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(890, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(891, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 258, A::exp_scaled_input(A::ln(s.ad_value(890)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 258, A::exp_scaled_input(A::ln(s.ad_value(891)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div(898, 245, 4);s.store_offset_mul(892, 246, 2, 1.0);s.store_offset_mul(893, 247, 3, 1.0);s.store_div_scaled_product_indices(894, 246, 4, 1.0, 892, 1.0);s.store_div_scaled_product_indices(895, 247, 4, 1.0, 893, 1.0);s.store_div_from_scalar_add_ad(896, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(894)), 1.0), A::div_from_scalar(1.0, s.ad_value(895)));s.store_offset_mul(892, 894, 2, 1.0);s.store_offset_mul(893, 895, 3, 1.0);}
        if (!s.b[1071]) {s.copy_ad(898, 245);s.copy_ad(894, 246);s.copy_ad(895, 247);s.copy_ad(896, 248);s.store_scalar(892, 1.0);s.store_scalar(893, 1.0);}
        s.store_mul_sub_rhs(897, 896, 878, 889);s.b[1072] = (s.v[897] > 0.0);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });s.b[1073] = ((-s.v[897]) < 80.0);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (s.b[1072] && s.b[1073]) {s.store_ln_one_plus_exp_neg_input(0, 897);}
        if (s.b[1072] && (!s.b[1073])) {s.store_neg(0, 897);}
        if s.b[1072] {s.store_add_scaled_inputs3_offset_mixed_iai(899, 878, 1.0, A::div(s.ad_value(897), s.ad_value(894)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1074] = (s.v[897] < 80.0);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if ((!s.b[1072]) && s.b[1074]) {s.store_ln_one_plus_exp(0, 897);}
        if ((!s.b[1072]) && (!s.b[1074])) {s.copy_ad(0, 897);}
        if (!s.b[1072]) {s.store_add_scaled_inputs3_offset_mixed_iai(899, 889, 1.0, A::div(s.ad_value(897), s.ad_value(895)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(900, 899, 0.5, 254, 0.5, 899, 254, 4.0, (-0.5));s.store_offset_sqrt_ad(901, A::offset(A::div_scaled_inputs2(s.ad_value(254), 2.0, s.ad_value(900), (-2.0), s.ad_value(255), 1.0), 1.0), (-1.0));s.store_add_scaled_product_indices(902, 900, 1.0, 255, 901, 1.0);s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(879)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_div_from_scalar_offset_product(903, 1.0, 871, 0, 1.0);s.store_div_from_scalar_offset_product(904, 1.0, 872, 0, 1.0);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(329), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(901)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(879)), 1.0, 1.0);s.store_mul(905, 873, 0);s.store_mul(906, 874, 0);s.store_add_mixed_ai(907, A::add_scaled_product(s.ad_value(902), 1.0, A::add_scaled_inputs3(s.ad_value(878), 1.0, s.ad_value(902), (-1.0), s.ad_value(905), 1.0), s.ad_value(903), 1.0), 341);s.store_add_mixed_ai(908, A::add_scaled_product(s.ad_value(902), 1.0, A::add_scaled_inputs3(s.ad_value(889), 1.0, s.ad_value(902), (-1.0), s.ad_value(906), 1.0), s.ad_value(904), 1.0), 341);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(909, A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)), 1.0), s.ad_value(225))), 0.01), (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_inputs3_sqrt_third_mixed_aia(910, A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_div(911, 894, 903);s.store_div(912, 895, 904);s.store_div_from_scalar(885, 1.0, 911);s.store_div_from_scalar(886, 1.0, 912);s.store_div_from_scalar_add_ad(913, 1.0, A::offset(s.ad_value(885), 1.0), s.ad_value(886));s.store_div_square_rhs(884, 253, 898);s.store_div_scaled_offset_numerator_mixed_ia(881, 911, 1.0, 1.0, A::offset(s.ad_value(912), 1.0), 1.0);s.store_ln(882, 881);s.b[1075] = (s.v[882] > 1e-8);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if s.b[1075] {s.store_div_scaled_product_offset_denominator_mixed_iai(883, 882, A::offset(s.ad_value(881), 1.0), 2.0, 881, (-1.0), 1.0);}
        if (!s.b[1075]) {s.store_scaled_offset(883, 882, 2.0, 2.0);}
        s.store_mul_sub_rhs(914, 913, 909, 910);s.store_square(915, 914);s.store_add_scaled_product_indices(887, 909, 1.0, 914, 885, (-1.0));s.store_add_scaled_product_indices(888, 910, 1.0, 914, 886, 1.0);s.store_div_from_scalar_offset_input(793, 1.0, 911, 1.0);s.store_div_from_scalar_offset_input(794, 1.0, 912, 1.0);s.store_offset_ln_ad(796, A::div_scaled_product(A::add_scaled_product(s.ad_value(911), 1.0, s.ad_value(912), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 3.0);s.store_offset_ln_ad(797, A::div_scaled_product(A::add_scaled_product(s.ad_value(912), 1.0, s.ad_value(911), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 3.0);s.b[1076] = (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if s.b[1076] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(887), 0.3333333333333));}
        if (!s.b[1076]) {s.store_scaled_sub(795, 796, 887, 0.3333333333333);}
        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);s.b[1077] = (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if s.b[1077] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(888), 0.3333333333333));}
        if (!s.b[1077]) {s.store_scaled_sub(795, 797, 888, 0.3333333333333);}
        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);s.store_mul_add_scaled_product_rhs_indices(798, 793, 801, 1.0, 911, 909, 1.0);s.store_mul_add_scaled_product_rhs_indices(799, 794, 800, 1.0, 912, 910, 1.0);s.b[1078] = (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if s.b[1078] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(798), 0.3333333333333));}
        if (!s.b[1078]) {s.store_scaled_sub(795, 796, 798, 0.3333333333333);}
        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);s.b[1079] = (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if s.b[1079] {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(799), 0.3333333333333));}
        if (!s.b[1079]) {s.store_scaled_sub(795, 797, 799, 0.3333333333333);}
        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);s.store_sub(916, 909, 800);s.store_sub(920, 910, 801);s.store_scalar(807, 0.0);s.store_scalar(810, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        s.store_mul(802, 911, 916);s.b[1080] = ((s.v[909] - s.v[916]) < 80.0);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if s.b[1080] {s.store_exp_sub(793, 909, 916);}
        if (!s.b[1080]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1081] = (s.v[804] < (-0.005));s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if s.b[1081] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1082] = (s.v[804] > 0.005);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if ((!s.b[1081]) && s.b[1082]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1081]) && (!s.b[1082])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if ((!s.b[1081]) && (!s.b[1082])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1083] = (s.v[804] > 0.005);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if s.b[1083] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1084] = (s.v[804] < (-0.005));s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if ((!s.b[1083]) && s.b[1084]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1083]) && (!s.b[1084])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1085] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if s.b[1085] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1085]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1086] = (s.v[816] > 0.0);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if s.b[1086] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1086]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 916);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 916, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(916, 916, 831);s.store_mul(802, 911, 916);s.store_mul(832, 912, 920);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.store_sub_square_lhs(837, 802, 804);s.b[1087] = (s.v[837] > 0.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if s.b[1087] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 909, (-1.0), 916, 1.0, 0.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);}
        let (t2,) = {
    if s.b[1087] {
        let t0: f64 = (s.v[909] - s.v[916]);let t1: f64 = (t0 - s.v[796]);
        (t1,)
    } else {
        (s.v[838],)
    }
};
        s.store_scalar(838, t2);s.b[1088] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (s.b[1087] && s.b[1088]) {s.store_sub_div_rhs_indices(916, 916, 828, 829);}
        s.store_mul(802, 911, 916);s.store_mul(832, 912, 920);s.store_add(825, 802, 832);s.store_offset_scaled(833, 825, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);s.store_sqrt_add_scaled_square_product(836, 834, 1.0, 833, 835, (-4.0));s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);s.b[1089] = (s.v[804] < (-0.005));s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if s.b[1089] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        s.b[1090] = (s.v[804] > 0.005);s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if ((!s.b[1089]) && s.b[1090]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);}
        if ((!s.b[1089]) && (!s.b[1090])) {s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        if ((!s.b[1089]) && (!s.b[1090])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(809, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(804, 804, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(825), s.ad_value(808), 1.0, s.ad_value(802), s.ad_value(832), 1.0), 1.0, s.ad_value(804), 1.0, A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0), 1.0));s.store_sub_square_lhs(837, 802, 804);s.b[1091] = (s.v[837] > 0.0);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if s.b[1091] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, 909, (-1.0), 916, 1.0, 0.0);s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);}
        let (t5,) = {
    if s.b[1091] {
        let t3: f64 = (s.v[909] - s.v[916]);let t4: f64 = (t3 - s.v[796]);
        (t4,)
    } else {
        (s.v[838],)
    }
};
        s.store_scalar(838, t5);s.b[1092] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if (s.b[1091] && s.b[1092]) {s.store_sub_div_rhs_indices(916, 916, 828, 829);}
        s.store_mul(802, 911, 916);s.b[1093] = ((s.v[909] - s.v[916]) < 80.0);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if s.b[1093] {s.store_exp_sub(793, 909, 916);}
        if (!s.b[1093]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1094] = (s.v[804] < (-0.005));s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if s.b[1094] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1095] = (s.v[804] > 0.005);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if ((!s.b[1094]) && s.b[1095]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if ((!s.b[1094]) && s.b[1095]) {s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1094]) && (!s.b[1095])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1096] = (s.v[804] > 0.005);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if s.b[1096] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1097] = (s.v[804] < (-0.005));s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if ((!s.b[1096]) && s.b[1097]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
        if ((!s.b[1096]) && (!s.b[1097])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1098] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if s.b[1098] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1098]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if (!s.b[1098]) {s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1099] = (s.v[816] > 0.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if s.b[1099] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1099]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 916);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 916, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(916, 916, 831);s.store_mul(802, 911, 916);s.b[1100] = ((s.v[909] - s.v[916]) < 80.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if s.b[1100] {s.store_exp_sub(793, 909, 916);}
        if (!s.b[1100]) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_mul(803, 884, 793);s.store_sub_square_lhs(804, 802, 803);s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);s.b[1101] = (s.v[804] < (-0.005));s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if s.b[1101] {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_div_mixed_ia(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if s.b[1101] {s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        s.b[1102] = (s.v[804] > 0.005);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((!s.b[1101]) && s.b[1102]) {s.store_sqrt_abs_ad(807, s.ad_value(804));s.store_exp_neg_input(810, 807);s.store_div_scaled_product_offset_rhs_mixed_iia(808, 807, 810, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);s.store_mul_add_mixed_iia(809, 793, 804, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)));s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);s.store_mul_div_lhs(814, 805, 804, 794);s.store_div_mixed_ai(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);}
        if ((!s.b[1101]) && (!s.b[1102])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(795, 804, 1.0, 804, 1.0, 804, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(808, 804, 795, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(793, 804, 1.0, 804, 1.0, 804, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(809, 805, 793);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(794, 804, 1.0, 804, 1.0, 804, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));s.store_scaled_mul(814, 805, 795, (-0.5));s.store_add_scaled_product_mixed_aii(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));}
        s.b[1103] = (s.v[804] > 0.005);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if s.b[1103] {s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);s.store_mul(812, 794, 810);s.store_sub_ln_lhs(813, 794, 807);}
        s.b[1104] = (s.v[804] < (-0.005));s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if ((!s.b[1103]) && s.b[1104]) {s.store_sin_scaled_input(794, 807, 0.5);s.store_div_scaled_inputs_square_rhs(812, 804, -1.0, 794, 1.0);s.store_ln(813, 812);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1103]) && (!s.b[1104])) {s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(813, 812);}
        s.b[1105] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if s.b[1105] {s.store_add(816, 802, 808);s.store_add(817, 911, 809);s.copy_ad(818, 811);}
        if (!s.b[1105]) {s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));s.store_sub(795, 809, 911);s.store_mul_sub_lhs(816, 803, 812, 794);s.store_mul_mixed_ai(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);s.store_mul_mixed_ai(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);}
        s.b[1106] = (s.v[816] > 0.0);s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if s.b[1106] {s.store_ln(819, 816);s.store_div_from_scalar(793, 1.0, 816);s.store_mul(820, 817, 793);s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);}
        if (!s.b[1106]) {s.store_add_offset_lhs_mixed_ia(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));s.store_div_from_scalar(793, 1.0, 916);s.store_add(820, 911, 793);s.store_mul_scale_offset_indices(821, 793, 793, -1.0, 0.0);}
        s.store_sub_add_scaled_inputs4_lhs_indices(822, 910, 1.0, 909, (-1.0), 916, 1.0, 819, 2.0, 813);s.store_sub_mixed_ai(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);s.store_mul(827, 912, 824);s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);s.store_add_mixed_ai(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);s.store_sub_mixed_ai(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);s.store_add(916, 916, 831);s.b[1107] = (p.p10 == 1.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });s.b[1108] = (((s.v[831]) as f64).abs() > 0.01);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (s.b[1107] && s.b[1108]) {s.store_mul(802, 911, 916);}
        s.b[1109] = ((s.v[909] - s.v[916]) < 80.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if ((s.b[1107] && s.b[1108]) && s.b[1109]) {s.store_exp_sub(793, 909, 916);}
        if ((s.b[1107] && s.b[1108]) && (!s.b[1109])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
    }
}
