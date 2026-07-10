#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[781] && s.b[784]) {s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        if s.b[781] {s.store_scalar(254, 0.0);}
        s.b[785] = (p.p13 > 0.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });s.b[786] = (p.p14 == 1.0);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if ((s.b[781] && s.b[785]) && s.b[786]) {s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if ((s.b[781] && s.b[785]) && (!s.b[786])) {s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        if s.b[781] {s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 245);s.store_add_scaled_inputs4_indices(21, 179, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 180, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 181, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 182, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_ln(291, 218);s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p.p35);s.store_mul(38, 187, 292);s.store_mul(39, 188, 292);s.store_exp_mul(293, 48, 291);s.store_mul(46, 189, 293);s.store_exp_mul(294, 49, 291);s.store_mul(47, 190, 294);s.store_exp_mul(295, 43, 291);s.store_mul(33, 191, 295);s.store_exp_mul(296, 45, 291);s.store_mul(44, 192, 296);s.store_exp_mul(297, 52, 291);s.store_mul(50, 193, 297);s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);s.store_mul(263, 0, 46);s.store_exp_mul(298, 55, 291);s.store_mul(53, 194, 298);s.store_scaled_mul(268, 53, 222, 2.0);s.store_exp_mul(299, 60, 291);s.store_mul3_lhs(59, 195, 299, 292);s.store_mul(269, 59, 222);s.store_mul3_lhs(147, 196, 299, 292);s.store_mul(270, 147, 222);s.store_mul(271, 64, 223);s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);s.store_mul(68, 197, 300);s.store_mul(69, 198, 300);s.store_mul(70, 199, 300);s.store_mul(71, 200, 300);s.store_mul(72, 201, 300);s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);s.store_mul(73, 202, 300);s.store_mul(74, 203, 300);s.store_scale(279, 229, 0.5);s.store_mul(280, 75, 222);s.store_mul(281, 75, 219);s.store_div_from_scalar_offset_product(282, 1.0, 88, 232, 1.0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 204, 273, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[781] {s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 205, 273, 0);s.store_mul_exp_mixed_ia(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));s.store_mul(283, 116, 222);s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);s.store_ln_div(288, 118, 248);s.store_scaled_mul(289, 119, 222, 1.25e-6);s.store_exp_mul(301, 169, 291);s.store_mul(168, 210, 301);}
        let (t8,) = {
    if s.b[781] {
        let t6: f64 = (4.0 * 1.3806488e-23);let t7: f64 = (t6 * s.v[213]);
        (t7,)
    } else {
        (s.v[302],)
    }
};
        s.store_scalar(302, t8);
        let (ta,) = {
    if s.b[781] {
        let t9: f64 = (s.v[171] * s.v[302]);
        (t9,)
    } else {
        (s.v[303],)
    }
};
        s.store_scalar(303, ta);s.b[787] = (p.p14 == 1.0);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if s.b[787] {s.store_voltage(326, ctx, nodes, Some(9), Some(6));s.store_voltage(698, ctx, nodes, Some(7), Some(6));s.store_voltage(327, ctx, nodes, Some(6), Some(8));}
        if (!s.b[787]) {s.store_scaled_voltage(326, ctx, nodes, Some(9), Some(6), -1.0);s.store_scaled_voltage(698, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(327, ctx, nodes, Some(6), Some(8), -1.0);}
        s.store_neg(699, 698);s.store_add(328, 326, 699);s.store_add(329, 698, 327);s.b[788] = (s.v[698] < 0.0);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if s.b[788] {s.store_scalar(330, (-1.0));s.copy_ad(332, 699);s.copy_ad(331, 328);s.copy_ad(333, 329);}
        if (!s.b[788]) {s.store_scalar(330, 1.0);s.copy_ad(332, 698);s.copy_ad(331, 326);s.copy_ad(333, 327);}
        s.store_add(334, 331, 333);s.store_mul(335, 332, 223);s.store_mul_scale_offset_mixed_ia(336, 223, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(337, 335, 336, 0.5);s.copy_ad(865, 21);s.copy_ad(866, 22);s.copy_ad(867, 27);s.copy_ad(868, 28);s.copy_ad(869, 31);s.copy_ad(870, 32);s.copy_ad(871, 269);s.copy_ad(872, 211);s.copy_ad(873, 63);s.store_sub_mixed_ai(874, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(865)), s.ad_value(223), 1.0), 230);s.store_add_scaled_product_mixed_iai(875, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(866), 1.0), 223, 1.0);s.store_sub(876, 875, 230);s.b[1055] = (p.p2 > 0.0);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if s.b[1055] {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(877, 242, 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);s.store_ln(878, 877);}
        s.b[1056] = (s.v[878] > 1e-8);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1056]) {s.store_div_scaled_product_offset_denominator_mixed_iai(879, 878, A::offset(s.ad_value(877), 1.0), 2.0, 877, (-1.0), 1.0);}
        if (s.b[1055] && (!s.b[1056])) {s.store_scaled_offset(879, 878, 2.0, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        if s.b[1055] {s.store_div_square_rhs(880, 249, 241);s.store_div_from_scalar(881, 1.0, 242);s.store_div_from_scalar(882, 1.0, 243);s.store_div_from_scalar_add_ad(909, 1.0, A::offset(s.ad_value(881), 1.0), s.ad_value(882));s.store_mul_sub_rhs(910, 909, 874, 876);s.store_add_scaled_product_indices(883, 874, 1.0, 910, 881, (-1.0));s.store_add_scaled_product_indices(884, 876, 1.0, 910, 882, 1.0);s.store_div_from_scalar_offset_input(789, 1.0, 242, 1.0);s.store_div_from_scalar_offset_input(790, 1.0, 243, 1.0);s.store_offset_ln_ad(792, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);s.store_offset_ln_ad(793, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);}
        s.b[1057] = (((s.v[792] - s.v[883]) / 1.5) < 80.0);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1057]) {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.6666666666666666, s.ad_value(883), 0.6666666666666666));}
        if (s.b[1055] && (!s.b[1057])) {s.store_scaled_sub(791, 792, 883, 0.6666666666666666);}
        if s.b[1055] {s.store_sub_scaled_inputs(796, 792, 1.0, 791, 1.5);s.store_mul_add_scaled_product_rhs_indices(795, 790, 796, 1.0, 243, 876, 1.0);}
        s.b[1058] = (((s.v[793] - s.v[795]) / 1.5) < 80.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1058]) {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.6666666666666666, s.ad_value(795), 0.6666666666666666));}
        if (s.b[1055] && (!s.b[1058])) {s.store_scaled_sub(791, 793, 795, 0.6666666666666666);}
        if s.b[1055] {s.store_sub_scaled_inputs(1, 793, 1.0, 791, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 876);s.store_sub(841, 2, 3);}
        s.b[1059] = ((((-s.v[262])) as f64).abs() < 80.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1059]) {s.store_exp_neg_input(842, 262);}
        s.b[1060] = ((-s.v[262]) < (-80.0));s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if ((s.b[1055] && (!s.b[1059])) && s.b[1060]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(262))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1059])) && (!s.b[1060])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(842, A::neg(s.ad_value(262)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1061] = (((s.v[841]) as f64).abs() <= s.v[261]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1061]) {s.store_scaled_square(839, 260, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 841, 260, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(841), 1.0, s.ad_value(842)), s.ad_value(256), s.ad_value(839)), 1.0));}
        s.b[1062] = (s.v[841] < (-s.v[261]));s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_neg(843, 841);s.store_scaled_mul(844, 843, 260, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(845, 844, 10.0, (-6.0), 64.0, 0.5);s.store_sub(838, 843, 845);s.store_add_scaled_square_product_mixed_iia(846, 838, 1.0, 257, A::offset(s.ad_value(845), 1.0), 1.0);s.store_sub_scaled_inputs(848, 838, 2.0, 257, 1.0);s.store_sub_ln_mul_lhs(849, 846, 258, 845);s.store_add(836, 846, 848);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_product(s.ad_value(846), (-1.0), s.ad_value(848), s.ad_value(848), 0.5), 1.0);s.store_add_mixed_ia(850, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::sub_scaled_inputs(A::square(s.ad_value(848)), 0.3333333333333, s.ad_value(846), 1.0))), 1.0));}
        s.b[1063] = (s.v[850] < 80.0);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && s.b[1063]) {s.store_exp(851, 850);}
        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && (!s.b[1063])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(851, 850, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_div_from_scalar(852, 1.0, 851);s.store_div_from_scalar_offset_square(838, 1.0, 850, 2.0);s.store_mul_square_lhs(853, 850, 838);s.store_mul3_affine_lhs(854, 850, 838, 4.0, 0.0, 838);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), 838, 838);s.store_sub(838, 843, 850);s.store_mul(839, 842, 852);s.store_add_scaled_product_mixed_iia(856, 838, 2.0, 257, A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(839), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(842), 1.0, s.ad_value(854)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(850), (-1.0), s.ad_value(839), 1.0, (-1.0)), 1.0, s.ad_value(842), A::sub(A::offset(s.ad_value(850), (-1.0)), s.ad_value(853)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(851), 1.0, s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));s.store_sub_scaled_inputs_mixed_ia(4, 850, -1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_div_from_scalar_offset_scaled_input(858, 1.0, 256, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(859, 858, A::mul_scaled_lhs(s.ad_value(259), 1.25, s.ad_value(858)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(860, 841, 260, A::offset(A::mul(s.ad_value(859), s.ad_value(841)), 1.0));}
        s.b[1064] = ((-s.v[860]) > (-80.0));s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1064]) {s.store_exp_neg_input(838, 860);}
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1064])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(838, 1.80485e-35, A::neg(A::neg(s.ad_value(860))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_sub_from_scalar(861, 1.0, 838);s.store_add_scaled_inputs_product_mixed_iiia(862, 841, 1.0, 257, 0.5, 256, A::sqrt(A::add_scaled_inputs3(s.ad_value(841), 1.0, s.ad_value(257), 0.25, s.ad_value(861), -1.0)), (-1.0));s.store_offset(863, 262, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_sub_ad(845, A::add_scaled_inputs3(s.ad_value(862), 0.5, s.ad_value(863), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(862), s.ad_value(863)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(863), 0.5, A::sqrt_square_offset(s.ad_value(863), 5.0), 0.5));s.store_sub(838, 841, 845);s.store_exp_neg_input(839, 845);s.store_div_from_scalar_offset_square(840, 1.0, 845, 2.0);s.store_mul_square_lhs(853, 845, 840);s.store_mul3_affine_lhs(854, 845, 840, 4.0, 0.0, 840);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(840), 8.0, s.ad_value(853), 12.0), 840, 840);s.store_max_from_scalar_ad(846, 1e-40, A::add_scaled_square_product(s.ad_value(838), 1.0, s.ad_value(257), A::add_scaled_product(A::offset(A::add(s.ad_value(839), s.ad_value(845)), (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(845), 1.0), s.ad_value(853)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(847, 1.0, 257, A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(848, 838, 2.0, 257, A::add_scaled_sub_value_product(1.0, s.ad_value(839), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(849, 262, 1.0, 845, (-1.0), A::ln(A::div(s.ad_value(846), s.ad_value(257))), 1.0);s.store_add(836, 846, 848);s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_products(s.ad_value(848), s.ad_value(848), 0.5, s.ad_value(846), s.ad_value(847), (-1.0)), 1.0);s.store_add_mixed_ia(864, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::add_scaled_square_product(s.ad_value(848), 0.3333333333333, s.ad_value(846), s.ad_value(847), (-1.0)))), 1.0));}
        s.b[1065] = (s.v[864] < 80.0);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1065]) {s.store_exp(851, 864);s.store_div_from_scalar(852, 1.0, 851);s.store_mul(851, 842, 851);}
        s.b[1066] = (s.v[864] > (s.v[262] - 80.0));s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && s.b[1066]) {s.store_exp_sub(851, 864, 262);s.store_div(852, 842, 851);}
        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(851, 1.80485e-35, A::sub(s.ad_value(262), s.ad_value(864)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(852, 1.80485e-35, 864, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_div_from_scalar_offset_square(838, 1.0, 864, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_mul_square_lhs(853, 864, 838);s.store_mul3_affine_lhs(854, 864, 838, 4.0, 0.0, 838);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), 838, 838);s.store_sub(838, 841, 864);s.store_add_scaled_product_mixed_iia(856, 838, 2.0, 257, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(852)), 1.0, s.ad_value(851), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(852), 1.0, s.ad_value(864), 1.0, s.ad_value(851), 1.0, (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(864), 1.0), s.ad_value(853)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(852), 1.0, s.ad_value(851), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));s.store_add_scaled_inputs_mixed_ia(4, 864, 1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);}
        if s.b[1055] {s.store_mul_add_rhs(885, 0, 4, 3);}
        if (!s.b[1055]) {s.copy_ad(885, 876);}
        s.store_mul_sub_rhs(0, 244, 874, 885);s.b[1067] = (p.p13 > 0.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if s.b[1067] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(886, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(887, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 254, A::exp_scaled_input(A::ln(s.ad_value(886)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 254, A::exp_scaled_input(A::ln(s.ad_value(887)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div(894, 241, 4);s.store_offset_mul(888, 242, 2, 1.0);s.store_offset_mul(889, 243, 3, 1.0);s.store_div_scaled_product_indices(890, 242, 4, 1.0, 888, 1.0);s.store_div_scaled_product_indices(891, 243, 4, 1.0, 889, 1.0);s.store_div_from_scalar_add_ad(892, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(890)), 1.0), A::div_from_scalar(1.0, s.ad_value(891)));s.store_offset_mul(888, 890, 2, 1.0);s.store_offset_mul(889, 891, 3, 1.0);}
        if (!s.b[1067]) {s.copy_ad(894, 241);s.copy_ad(890, 242);s.copy_ad(891, 243);s.copy_ad(892, 244);s.store_scalar(888, 1.0);s.store_scalar(889, 1.0);}
        s.store_mul_sub_rhs(893, 892, 874, 885);s.b[1068] = (s.v[893] > 0.0);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });s.b[1069] = ((-s.v[893]) < 80.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (s.b[1068] && s.b[1069]) {s.store_ln_one_plus_exp_neg_input(0, 893);}
        if (s.b[1068] && (!s.b[1069])) {s.store_neg(0, 893);}
        if s.b[1068] {s.store_add_scaled_inputs3_offset_mixed_iai(895, 874, 1.0, A::div(s.ad_value(893), s.ad_value(890)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1070] = (s.v[893] < 80.0);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if ((!s.b[1068]) && s.b[1070]) {s.store_ln_one_plus_exp(0, 893);}
        if ((!s.b[1068]) && (!s.b[1070])) {s.copy_ad(0, 893);}
        if (!s.b[1068]) {s.store_add_scaled_inputs3_offset_mixed_iai(895, 885, 1.0, A::div(s.ad_value(893), s.ad_value(891)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(896, 895, 0.5, 250, 0.5, 895, 250, 4.0, (-0.5));s.store_offset_sqrt_ad(897, A::offset(A::div_scaled_inputs2(s.ad_value(250), 2.0, s.ad_value(896), (-2.0), s.ad_value(251), 1.0), 1.0), (-1.0));s.store_add_scaled_product_indices(898, 896, 1.0, 251, 897, 1.0);s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(875)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_div_from_scalar_offset_product(899, 1.0, 867, 0, 1.0);s.store_div_from_scalar_offset_product(900, 1.0, 868, 0, 1.0);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(325), A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(897)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(875)), 1.0, 1.0);s.store_mul(901, 869, 0);s.store_mul(902, 870, 0);s.store_add_mixed_ai(903, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(874), 1.0, s.ad_value(898), (-1.0), s.ad_value(901), 1.0), s.ad_value(899), 1.0), 337);s.store_add_mixed_ai(904, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(885), 1.0, s.ad_value(898), (-1.0), s.ad_value(902), 1.0), s.ad_value(900), 1.0), 337);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(905, A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), s.ad_value(221))), 0.01), (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);s.store_mul(823, 908, 820);s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);s.store_add_mixed_ai(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);s.store_sub_mixed_ai(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);s.store_add(912, 912, 827);s.store_mul(798, 907, 912);s.store_mul(828, 908, 916);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.store_sub_square_lhs(833, 798, 800);s.b[1083] = (s.v[833] > 0.0);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if s.b[1083] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 905, (-1.0), 912, 1.0, 0.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);}
        let (t2,) = {
    if s.b[1083] {
        let t0: f64 = (s.v[905] - s.v[912]);let t1: f64 = (t0 - s.v[792]);
        (t1,)
    } else {
        (s.v[834],)
    }
};
        s.store_scalar(834, t2);s.b[1084] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1083] && s.b[1084]) {s.store_sub_div_rhs_indices(912, 912, 824, 825);}
        s.store_mul(798, 907, 912);s.store_mul(828, 908, 916);s.store_add(821, 798, 828);s.store_offset_scaled(829, 821, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);s.store_sqrt_add_scaled_square_product(832, 830, 1.0, 829, 831, (-4.0));s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);s.b[1085] = (s.v[800] < (-0.005));s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if s.b[1085] {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_div_mixed_ia(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        s.b[1086] = (s.v[800] > 0.005);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((!s.b[1085]) && s.b[1086]) {s.store_sqrt_abs_ad(803, s.ad_value(800));s.store_exp_neg_input(806, 803);s.store_div_scaled_product_offset_rhs_mixed_iia(804, 803, 806, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);}
        if ((!s.b[1085]) && (!s.b[1086])) {s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        if ((!s.b[1085]) && (!s.b[1086])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(805, 800, 1.0, 800, 1.0, 800, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        s.store_sub_mixed_ia(800, 800, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(821), s.ad_value(804), 1.0, s.ad_value(798), s.ad_value(828), 1.0), 1.0, s.ad_value(800), 1.0, A::offset(A::mul(s.ad_value(821), s.ad_value(805)), 1.0), 1.0));s.store_sub_square_lhs(833, 798, 800);s.b[1087] = (s.v[833] > 0.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if s.b[1087] {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, 905, (-1.0), 912, 1.0, 0.0);s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);}
        let (t5,) = {
    if s.b[1087] {
        let t3: f64 = (s.v[905] - s.v[912]);let t4: f64 = (t3 - s.v[792]);
        (t4,)
    } else {
        (s.v[834],)
    }
};
        s.store_scalar(834, t5);s.b[1088] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
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
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
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
}
