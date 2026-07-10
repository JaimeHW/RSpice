#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        s.b[1201] = (s.v[56] < 0.0);s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if ((!s.b[1200]) && s.b[1201]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((!s.b[1200]) && (!s.b[1201])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        s.store_mul_add_scaled_product_rhs_indices(1019, 943, 54, 1.0, 995, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1020, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1012)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 38, 1019, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1021, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1013)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 39, 1019, 1.0);s.store_div_scaled_product_add_scaled_denominator(1022, 1017, 1016, 1.0, A::div(s.ad_value(1014), s.ad_value(1020)), 1.0, A::div(s.ad_value(1015), s.ad_value(1021)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1023, 1.0, 995, 4.0);s.b[1202] = (s.v[65] > 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1009, 1.0);}
        if (!s.b[1202]) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1009, 1.0);}
        s.store_mul3_lhs(1024, 995, 1023, 0);s.store_mul_ln_mixed_ia(1025, 1024, A::offset(A::div_scaled_inputs2(s.ad_value(339), 1.0, s.ad_value(979), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(995), s.ad_value(995)), 1.0, s.ad_value(66), s.ad_value(227), 1.0), 1.0), 1.0));s.store_mul(1026, 877, 1025);s.store_div_from_scalar_offset_ad(1027, 1.0, A::mul_offset_rhs(s.ad_value(1026), s.ad_value(1026), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(955, s.ad_value(1006), 100.0, s.ad_value(1006), 100.0, 1.0);s.b[1203] = (s.v[61] < 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
        if s.b[1203] {s.store_div_from_scalar_sub_from_scalar_ad(956, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(955)));}
        if (!s.b[1203]) {s.store_offset_mul(956, 61, 955, 1.0);}
        s.store_div_scaled_value_offset_denominator(957, s.ad_value(1007), 100.0, s.ad_value(1007), 100.0, 1.0);s.b[1204] = (s.v[62] < 0.0);s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if s.b[1204] {s.store_div_from_scalar_sub_from_scalar_ad(958, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(957)));}
        if (!s.b[1204]) {s.store_offset_mul(958, 62, 957, 1.0);}
        s.store_mul_ad_affine_product_rhs(1028, 875, s.ad_value(996), A::add(s.ad_value(956), s.ad_value(958)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1029, 1028, 1.0, 1022, 1027, 1.0);s.store_square(1030, 1029);s.store_sqrt_offset_input(1031, 1030, 1.0);s.store_div_scaled_offset_numerator_indices(1032, 1030, 1.5, 1.0, 1031, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1205] = (p.p13 > 0.0);s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if s.b[1205] {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 258, 0.6, 1006, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 258, 0.6, 1007, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1033, A::mul(s.ad_value(911), s.ad_value(2)), 1.0, 1.0, 892, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1034, A::mul(s.ad_value(912), s.ad_value(3)), 1.0, 1.0, 893, 1.0);}
        if (!s.b[1205]) {s.store_scalar(1033, 1.0);s.store_scalar(1034, 1.0);}
        s.b[1206] = (s.v[917] > 1e-6);s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });s.b[1207] = (s.v[982] > 1e-6);s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });s.b[1208] = (((s.v[991]) as f64).abs() < 0.01);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if ((s.b[1206] && s.b[1207]) && s.b[1208]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(980), 2.0), 1.0, 990, 0.5, A::offset(s.ad_value(981), 2.0), 990, 1.0);s.store_mul(2, 0, 991);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 984, 1.0, A::mul3_scaled_output(s.ad_value(985), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(990))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(981), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1035, A::div_scaled_add_product(s.ad_value(986), (-1.0), s.ad_value(993), s.ad_value(982), 1.0, s.ad_value(990), 1.0), 1.0, 2, (-1.0), 982, 1.0);s.store_div_scaled_product_offset_denominator_indices(1036, 1035, 982, 1.0, 1035, 1.0, 1.0);}
        if ((s.b[1206] && s.b[1207]) && (!s.b[1208])) {s.store_sub_ad(1035, A::div_scaled_product_by_product(s.ad_value(993), s.ad_value(992), 1.0, s.ad_value(990), s.ad_value(991), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(986), s.ad_value(990)), 1.0, A::div(s.ad_value(987), s.ad_value(991)), 1.0, s.ad_value(982), 1.0));s.store_div_scaled_product_offset_denominator_indices(1036, 1035, 982, 1.0, 1035, 1.0, 1.0);}
        if (s.b[1206] && (!s.b[1207])) {s.copy_ad(1036, 953);}
        if s.b[1206] {s.store_sub(2, 1036, 960);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1209] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if (s.b[1206] && s.b[1209]) {s.store_sub(4, 982, 917);s.store_add_scaled_product_indices(1037, 4, 1.0, 1036, 996, (-1.0));s.store_add_scaled_product_indices(1038, 4, 1.0, 960, 996, (-1.0));s.store_sqrt_square_add(1039, 1037, 3);s.store_sqrt_square_add(1040, 1038, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1041, 0.25, 2, A::add_scaled_products3(s.ad_value(1040), s.ad_value(1037), 1.0, s.ad_value(1039), s.ad_value(1038), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(1040), 1.0, A::add(s.ad_value(1037), s.ad_value(1039)), 1.0)), 1.0));}
        if (s.b[1206] && (!s.b[1209])) {s.store_mul(4, 996, 2);s.store_div_scaled_product3_mixed_iiia(1041, 996, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (!s.b[1206]) {s.copy_ad(1036, 953);s.store_scalar(1041, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs3_mixed_aii(1042, A::add_scaled_product(s.ad_value(1041), 1.0, s.ad_value(995), s.ad_value(996), 1.0), 1.0, 917, 1.0, 982, -1.0);s.b[1210] = (s.v[917] > 1e-6);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });s.b[1211] = (s.v[1042] > 1e-30);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if (s.b[1210] && s.b[1211]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 926, A::div(s.ad_value(922), s.ad_value(917)), 1.0, 929, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1044, 990, A::div(s.ad_value(986), s.ad_value(982)), 1.0, 993, -1.0);s.store_div_scaled_inputs2_indices(1045, 1043, 1.0, 1044, (-1.0), 1042, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1046, 927, A::div(s.ad_value(923), s.ad_value(917)), 1.0, 929, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1047, 991, A::div(s.ad_value(987), s.ad_value(982)), 1.0, 993, -1.0);s.store_div_scaled_inputs2_indices(1048, 1046, 1.0, 1047, (-1.0), 1042, 1.0);}
        if (s.b[1210] && (!s.b[1211])) {s.store_scalar(1045, 0.0);s.store_scalar(1048, 0.0);}
        if (!s.b[1210]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1049, 948, A::div(s.ad_value(885), s.ad_value(951)), (-2.0), 954, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1050, 949, A::div(s.ad_value(886), s.ad_value(952)), (-2.0), 954, (-2.0));s.store_mul_sub_lhs(0, 1050, 1049, 954);s.store_mul(2, 1049, 885);s.store_mul(3, 1050, 886);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(948), s.ad_value(885), 2.0, s.ad_value(949), s.ad_value(886), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1051, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(951)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1052, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(952)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1045, 951, 954, -1.0, 1051, 951, -1.0);s.store_mul_add_scaled_product_rhs_indices(1048, 952, 954, -1.0, 1052, 952, -1.0);}
        s.store_mul(1053, 1045, 1032);s.store_mul(1054, 1048, 1032);s.store_scaled_sub(1055, 983, 918, 0.5);s.store_scaled_sub(1056, 984, 919, 0.5);s.store_mul(1057, 1055, 1053);s.store_mul(1058, 1056, 1054);s.copy_ad(383, 879);s.copy_ad(384, 883);s.copy_ad(385, 884);s.copy_ad(386, 885);s.copy_ad(387, 886);s.copy_ad(388, 913);s.copy_ad(389, 914);s.copy_ad(390, 898);s.copy_ad(391, 897);s.copy_ad(392, 916);s.copy_ad(393, 901);s.copy_ad(394, 902);s.copy_ad(395, 903);s.copy_ad(396, 904);s.copy_ad(397, 905);s.copy_ad(398, 908);s.copy_ad(399, 910);s.copy_ad(400, 909);s.copy_ad(401, 911);s.copy_ad(402, 912);s.copy_ad(403, 917);s.copy_ad(404, 918);s.copy_ad(405, 919);s.copy_ad(406, 930);s.copy_ad(407, 960);s.copy_ad(408, 983);s.copy_ad(409, 984);s.copy_ad(411, 979);s.copy_ad(412, 980);s.copy_ad(413, 982);s.copy_ad(414, 994);s.copy_ad(415, 995);s.copy_ad(416, 999);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(417, 1006);s.copy_ad(418, 1007);s.copy_ad(419, 1008);s.copy_ad(420, 1009);s.copy_ad(421, 1016);s.copy_ad(422, 1022);s.copy_ad(423, 1023);s.copy_ad(424, 1025);s.copy_ad(425, 1027);s.copy_ad(426, 1031);s.copy_ad(428, 1030);s.copy_ad(429, 1032);s.copy_ad(430, 1033);s.copy_ad(431, 1034);s.copy_ad(432, 1036);s.copy_ad(433, 1042);s.copy_ad(435, 1045);s.copy_ad(436, 1055);s.copy_ad(437, 1056);s.copy_ad(438, 1057);s.copy_ad(439, 1058);s.store_div_scaled_inputs_mixed_ia(342, 421, p.p35, A::add(s.ad_value(417), s.ad_value(418)), 1.0);s.store_mul_add_scaled_product_rhs_indices(343, 424, 63, 1.0, 275, 423, 1.0);s.store_mul_scale_offset_mixed_ia(344, 425, A::mul_offset_rhs(s.ad_value(343), s.ad_value(343), 1.0), 1.0, 1.0);s.store_mul3_lhs(345, 422, 425, 426);s.b[1212] = (p.p13 > 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if s.b[1212] {s.store_div_scaled_inputs2_mixed_iia(346, 417, 1.0, 418, 1.0, A::add(A::div(s.ad_value(417), s.ad_value(430)), A::div(s.ad_value(418), s.ad_value(431))), 1.0);}
        if (!s.b[1212]) {s.store_scalar(346, 1.0);}
        s.store_mul_square_lhs(347, 226, 342);s.store_div_scaled_product_by_product_mixed_aiii(348, A::mul3(s.ad_value(347), s.ad_value(390), s.ad_value(433)), 344, 1.0, 345, 346, 1.0);s.store_mul_scale_offset_indices(704, 224, 330, -1.0, 0.0);s.store_mul_scale_offset_indices(705, 224, 332, -1.0, 0.0);s.store_add_scaled_product_indices(0, 234, 1.0, 163, 224, p.p14);s.store_add(706, 704, 0);s.store_add(707, 705, 0);s.store_scalar(714, 0.0);s.store_scalar(715, 0.0);s.store_scalar(716, 0.0);s.store_scalar(717, 0.0);s.store_div_mixed_ai(708, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);s.store_square(709, 708);s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);s.store_scale(711, 710, 1e-5);s.store_div_from_scalar(712, 1.0, 710);s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);s.b[1213] = (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0)));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });s.b[1214] = (((s.v[704]) as f64).abs() <= s.v[711]);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (s.b[1213] && s.b[1214]) {s.store_mul_scale_offset_indices(714, 712, 704, -1.0, 0.0);}
        s.b[1215] = (s.v[704] < (-s.v[711]));s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_neg(683, 704);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(686, A::sub(s.ad_value(683), s.ad_value(685)), 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1216] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((s.b[1213] && (!s.b[1214])) && s.b[1215]) && s.b[1216]) {s.store_exp(693, 692);}
        s.b[1217] = (s.v[692] < (-80.0));s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(714, 692, 697);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {s.store_add_scaled_inputs3_mixed_iia(694, 704, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_square_product_mixed_aia(695, A::sub(s.ad_value(704), s.ad_value(700)), 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(714, 700, 701);}
        if (s.b[1213] && (!s.b[1214])) {s.store_neg(714, 714);}
        s.b[1222] = (s.v[159] > 0.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = (((s.v[706]) as f64).abs() <= s.v[711]);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1223]) {s.store_mul_scale_offset_indices(716, 712, 706, -1.0, 0.0);}
        s.b[1224] = (s.v[706] < (-s.v[711]));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_neg(683, 706);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(686, A::sub(s.ad_value(683), s.ad_value(685)), 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1225] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (((s.b[1222] && (!s.b[1223])) && s.b[1224]) && s.b[1225]) {s.store_exp(693, 692);}
        s.b[1226] = (s.v[692] < (-80.0));s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && s.b[1226]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && (!s.b[1226])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(716, 692, 697);}
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
        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {s.store_add_scaled_inputs3_mixed_iia(694, 706, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_square_product_mixed_aia(695, A::sub(s.ad_value(706), s.ad_value(700)), 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(716, 700, 701);}
        if (s.b[1222] && (!s.b[1223])) {s.store_neg(716, 716);}
        s.store_div_mixed_ai(708, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);s.store_square(709, 708);s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);s.store_scale(711, 710, 1e-5);s.store_div_from_scalar(712, 1.0, 710);s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);s.b[1231] = (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0)));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });s.b[1232] = (((s.v[705]) as f64).abs() <= s.v[711]);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (s.b[1231] && s.b[1232]) {s.store_mul_scale_offset_indices(715, 712, 705, -1.0, 0.0);}
        s.b[1233] = (s.v[705] < (-s.v[711]));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {s.store_neg(683, 705);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {s.store_add_scaled_square_product_mixed_aia(686, A::sub(s.ad_value(683), s.ad_value(685)), 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1234] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if (((s.b[1231] && (!s.b[1232])) && s.b[1233]) && s.b[1234]) {s.store_exp(693, 692);}
        s.b[1235] = (s.v[692] < (-80.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && s.b[1235]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && (!s.b[1235])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && s.b[1239]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && (!s.b[1239])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {s.store_add_scaled_inputs3_mixed_iia(694, 705, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_square_product_mixed_aia(695, A::sub(s.ad_value(705), s.ad_value(700)), 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(715, 700, 701);}
        if (s.b[1231] && (!s.b[1232])) {s.store_neg(715, 715);}
        s.b[1240] = (s.v[160] > 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });s.b[1241] = (((s.v[707]) as f64).abs() <= s.v[711]);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if (s.b[1240] && s.b[1241]) {s.store_mul_scale_offset_indices(717, 712, 707, -1.0, 0.0);}
        s.b[1242] = (s.v[707] < (-s.v[711]));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_neg(683, 707);s.store_scaled_mul(684, 683, 712, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(685, 684, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(686, A::sub(s.ad_value(683), s.ad_value(685)), 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);s.store_sub_ln_div_lhs(688, 686, 709, 685);s.store_add(689, 686, 687);s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(691, 690, A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), 687, A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0), 1.0);s.store_add_mixed_ia(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));}
        s.b[1243] = (((s.v[692]) as f64).abs() < 80.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((s.b[1240] && (!s.b[1241])) && s.b[1242]) && s.b[1243]) {s.store_exp(693, 692);}
        s.b[1244] = (s.v[692] < (-80.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && s.b[1244]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(s.ad_value(692)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && (!s.b[1244])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(693, 692, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_sub(691, 683, 692);s.store_add_scaled_offset_product_rhs(694, 691, 2.0, 709, 693, (-1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {s.store_add_scaled_square_product_mixed_iia(695, 691, 1.0, 709, A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0);s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_neg_add(717, 692, 697);}
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
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(693, 1.80485e-35, A::neg(A::neg(s.ad_value(700))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && (!s.b[1248])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(693, A::neg(s.ad_value(700)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {s.store_add_scaled_inputs3_mixed_iia(694, 707, 2.0, 700, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0);s.store_add_scaled_square_product_mixed_aia(695, A::sub(s.ad_value(707), s.ad_value(700)), 1.0, 709, A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0));s.store_sub_from_scalar_scaled_mul(696, 1.0, 709, 693, 0.5);s.store_add_scaled_square_product_indices(691, 694, 1.0, 696, 695, (-4.0));s.store_div_scaled_inputs_mixed_ia(701, 695, 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0);s.store_add(717, 700, 701);}
        if (s.b[1240] && (!s.b[1241])) {s.store_neg(717, 717);}
        s.store_mul_add_scaled_inputs_rhs_indices(718, 223, 704, -1.0, 714, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(719, 223, 705, -1.0, 715, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(349, 223, 706, -1.0, 716, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(350, 223, 707, -1.0, 717, -1.0);s.b[1249] = (p.p3 > 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });s.b[1250] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1250]) {s.store_add(720, 718, 285);s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(718)), 0.0001));}
        s.b[1251] = ((((0.5 * s.v[704])) as f64).abs() < 80.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1250]) && s.b[1251]) {s.store_exp_scaled_input(0, 704, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
    ) {
        s.b[1252] = ((0.5 * s.v[704]) < (-80.0));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(704), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1250]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(723, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(724, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(725, 282, 2, 1.0, 281, 3, 1.0);s.store_mul_div_scaled_inputs_indices(2, 279, 81, (-1.0), 722, 1.0);}
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
        if (s.b[1249] && s.b[1250]) {s.store_mul_scale_offset_mixed_ia(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), 1.0, (-1.5));s.store_div_scaled_offset_numerator_mixed_ia(0, 729, 1.0, 1.0, A::offset(s.ad_value(730), 1.0), 1.0);}
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
        s.b[1267] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1267]) {s.store_add(720, 719, 285);s.store_scaled_sub_mixed_ia(721, 720, A::sqrt_square_offset(A::neg(s.ad_value(720)), 0.01), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1249] && s.b[1267]) {s.store_mul_sqrt_mixed_ia(722, 276, A::offset(A::square(s.ad_value(719)), 0.0001));}
        s.b[1268] = ((((0.5 * s.v[705])) as f64).abs() < 80.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1267]) && s.b[1268]) {s.store_exp_scaled_input(0, 705, 0.5);}
        s.b[1269] = ((0.5 * s.v[705]) < (-80.0));s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && s.b[1269]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && (!s.b[1269])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(705), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1249] && s.b[1267]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(723, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(724, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(725, 282, 2, 1.0, 281, 3, 1.0);s.store_mul_div_scaled_inputs_indices(2, 279, 81, (-1.0), 722, 1.0);}
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
        if (s.b[1249] && s.b[1267]) {s.store_mul_scale_offset_mixed_ia(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), 1.0, (-1.5));s.store_div_scaled_offset_numerator_mixed_ia(0, 729, 1.0, 1.0, A::offset(s.ad_value(730), 1.0), 1.0);}
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
        s.b[1284] = (s.v[68] > 0.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if (s.b[1249] && s.b[1284]) {s.store_mul_scale_offset_indices(735, 386, 436, -1.0, 0.0);}
        s.b[1285] = (((((2.0 * s.v[735]) - s.v[411])) as f64).abs() < 80.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
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
        if (s.b[1249] && s.b[1284]) {s.store_mul(730, 729, 0);s.store_mul_scale_offset_mixed_ia(0, 278, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(722), 1.0)), 1.0, (-1.5));}
        s.b[1295] = ((s.v[740] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((s.b[1249] && s.b[1284]) && (!s.b[1295])) {s.store_add_scaled_product_indices(0, 78, 1.0, 79, 722, 2.0);s.store_mul_div_mixed_iia(744, 227, 87, A::mul(s.ad_value(0), s.ad_value(278)));s.store_div(745, 735, 744);}
        s.b[1296] = (s.v[745] < 0.001);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });s.b[1297] = (((s.v[745]) as f64).abs() < 80.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if ((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && s.b[1297]) {s.store_exp(751, 745);}
        s.b[1298] = (s.v[745] < (-80.0));s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && s.b[1298]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(751, 1.80485e-35, A::neg(s.ad_value(745)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && (!s.b[1298])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(751, 745, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_div_from_scalar(752, 1.0, 751);s.store_sub(0, 751, 752);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {s.store_add(3, 751, 752);}
        s.b[1300] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[718] < 0.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
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
        s.b[1305] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[719] < 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
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
        s.store_scalar(356, 0.0);s.b[1310] = (p.p12 > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if s.b[1310] {s.store_mul(758, 336, 289);s.store_mul_scale_offset_mixed_ia(759, 289, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(760, 758, 759, 0.5);s.store_sub_mixed_ai(761, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub(s.ad_value(335), s.ad_value(100)), s.ad_value(289), 1.0), 234);s.store_sub_mixed_ai(762, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(101), 1.0), s.ad_value(289), 1.0), 234);s.store_div_from_scalar_offset_input(763, 1.0, 105, 1.0);s.store_div_from_scalar_offset_input(764, 1.0, 106, 1.0);s.store_mul(765, 109, 289);s.store_mul_scaled_offset_ad_rhs(0, 765, 2.0, A::sqrt(A::offset(A::div(s.ad_value(759), s.ad_value(765)), 1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1310] {s.store_mul(766, 107, 0);s.store_mul(767, 108, 0);s.store_add_scaled_product_mixed_iai(768, 760, 1.0, A::add(s.ad_value(761), s.ad_value(766)), 763, 1.0);s.store_add_scaled_product_mixed_iai(769, 760, 1.0, A::add(s.ad_value(762), s.ad_value(767)), 764, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(770, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(771, A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), s.ad_value(225))), 0.01), (-0.5));s.store_div(772, 246, 763);s.store_div(773, 247, 764);s.store_div_from_scalar(774, 1.0, 772);s.store_div_from_scalar(775, 1.0, 773);s.store_div_from_scalar_add_ad(776, 1.0, A::offset(s.ad_value(774), 1.0), s.ad_value(775));s.store_div_square_rhs(777, 290, 390);s.store_mul_sub_rhs(778, 776, 770, 771);}
        s.b[1311] = ((((s.v[771] - s.v[770])) as f64).abs() <= 1e-12);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1311]) {s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(776), s.ad_value(774)), 1.0, 776, 775, (-1.0));s.store_mul_add_scaled_inputs4_rhs_mixed_iaaa(3, 778, 775, 1.0, A::mul3_scaled_output(s.ad_value(774), s.ad_value(776), s.ad_value(774), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(775), s.ad_value(776), s.ad_value(775), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(776)), -1.0);s.store_div_scaled_product_mixed_aii(4, A::sub(s.ad_value(2), s.ad_value(3)), 777, 0.5, 776, 1.0);}
        if (s.b[1310] && (!s.b[1311])) {s.store_exp_mul_scaled_lhs_indices(2, 774, -1.0, 778);s.store_exp_ad(3, A::mul(A::sub(s.ad_value(775), A::div_from_scalar(1.0, s.ad_value(776))), s.ad_value(778)));s.store_div_scaled_product_mixed_iai(4, 777, A::sub(s.ad_value(2), s.ad_value(3)), 1.0, 778, 2.0);}
        if s.b[1310] {s.copy_ad(779, 4);}
        s.b[1312] = (s.v[770] < 80.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (s.b[1310] && s.b[1312]) {s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(s.ad_value(770))), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1310] && s.b[1312]) {s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
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
        if ((s.b[1310] && (!s.b[1315])) && (!s.b[1316])) {s.store_add_scaled_inputs3_mixed_aii(784, A::ln(s.ad_value(779)), 1.0, 770, 1.0, 411, (-1.0));s.store_mul_scale_offset_mixed_ia(0, 784, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)), -1.0, 1.0);}
        if s.b[1310] {s.copy_ad(781, 0);s.store_mul_scale_offset(782, A::sub(s.ad_value(780), s.ad_value(781)), A::add_scaled_inputs(s.ad_value(780), 0.5, s.ad_value(781), 0.5), 1.0, 1.0);s.store_mul_square_lhs(783, 288, 110);s.store_div_scaled_product3_indices(356, 783, 241, 782, 1.0, 422, 1.0);}
        s.b[1318] = (p.p8 != 0.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {s.store_div_scaled_add_product_indices(757, 339, 1.0, 115, 411, (-1.0), 227, 1.0);}
        s.b[1319] = (s.v[757] > 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (s.b[1318] && s.b[1319]) {s.store_div_scaled_value_offset_denominator(3, s.ad_value(113), (-1.0), s.ad_value(757), 1e-30, 1.0);}
        s.b[1320] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((s.b[1318] && s.b[1319]) && s.b[1320]) {s.store_exp(0, 3);}
        s.b[1321] = (s.v[3] < (-80.0));s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && s.b[1321]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && (!s.b[1321])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1322] = (s.v[6] > 0.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
    }
}
