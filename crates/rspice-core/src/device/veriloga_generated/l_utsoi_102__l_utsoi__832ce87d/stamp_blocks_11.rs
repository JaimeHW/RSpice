#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1201] = (p.p13 > 0.0);s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 254, 0.6, 1002, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 254, 0.6, 1003, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1029, A::mul(s.ad_value(907), s.ad_value(2)), 1.0, 1.0, 888, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1030, A::mul(s.ad_value(908), s.ad_value(3)), 1.0, 1.0, 889, 1.0);}
        if (!s.b[1201]) {s.store_scalar(1029, 1.0);s.store_scalar(1030, 1.0);}
        s.b[1202] = (s.v[913] > 1e-6);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });s.b[1203] = (s.v[978] > 1e-6);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });s.b[1204] = (((s.v[987]) as f64).abs() < 0.01);s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if ((s.b[1202] && s.b[1203]) && s.b[1204]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(976), 2.0), 1.0, 986, 0.5, A::offset(s.ad_value(977), 2.0), 986, 1.0);s.store_mul(2, 0, 987);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 980, 1.0, A::mul3_scaled_output(s.ad_value(981), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(986))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(977), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1031, A::div_scaled_add_product(s.ad_value(982), (-1.0), s.ad_value(989), s.ad_value(978), 1.0, s.ad_value(986), 1.0), 1.0, 2, (-1.0), 978, 1.0);s.store_div_scaled_product_offset_denominator_indices(1032, 1031, 978, 1.0, 1031, 1.0, 1.0);}
        if ((s.b[1202] && s.b[1203]) && (!s.b[1204])) {s.store_sub_ad(1031, A::div_scaled_product_by_product(s.ad_value(989), s.ad_value(988), 1.0, s.ad_value(986), s.ad_value(987), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(982), s.ad_value(986)), 1.0, A::div(s.ad_value(983), s.ad_value(987)), 1.0, s.ad_value(978), 1.0));s.store_div_scaled_product_offset_denominator_indices(1032, 1031, 978, 1.0, 1031, 1.0, 1.0);}
        if (s.b[1202] && (!s.b[1203])) {s.copy_ad(1032, 949);}
        if s.b[1202] {s.store_sub(2, 1032, 956);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1205] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if (s.b[1202] && s.b[1205]) {s.store_sub(4, 978, 913);s.store_add_scaled_product_indices(1033, 4, 1.0, 1032, 992, (-1.0));s.store_add_scaled_product_indices(1034, 4, 1.0, 956, 992, (-1.0));s.store_sqrt_square_add(1035, 1033, 3);s.store_sqrt_square_add(1036, 1034, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1037, 0.25, 2, A::add_scaled_products3(s.ad_value(1036), s.ad_value(1033), 1.0, s.ad_value(1035), s.ad_value(1034), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1034), 1.0, s.ad_value(1036), 1.0, A::add(s.ad_value(1033), s.ad_value(1035)), 1.0)), 1.0));}
        if (s.b[1202] && (!s.b[1205])) {s.store_mul(4, 992, 2);s.store_div_scaled_product3_mixed_iiia(1037, 992, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (!s.b[1202]) {s.copy_ad(1032, 949);s.store_scalar(1037, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs3_mixed_aii(1038, A::add_scaled_product(s.ad_value(1037), 1.0, s.ad_value(991), s.ad_value(992), 1.0), 1.0, 913, 1.0, 978, -1.0);s.b[1206] = (s.v[913] > 1e-6);s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });s.b[1207] = (s.v[1038] > 1e-30);s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });
        if (s.b[1206] && s.b[1207]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1039, 922, A::div(s.ad_value(918), s.ad_value(913)), 1.0, 925, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1040, 986, A::div(s.ad_value(982), s.ad_value(978)), 1.0, 989, -1.0);s.store_div_scaled_inputs2_indices(1041, 1039, 1.0, 1040, (-1.0), 1038, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1042, 923, A::div(s.ad_value(919), s.ad_value(913)), 1.0, 925, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 987, A::div(s.ad_value(983), s.ad_value(978)), 1.0, 989, -1.0);s.store_div_scaled_inputs2_indices(1044, 1042, 1.0, 1043, (-1.0), 1038, 1.0);}
        if (s.b[1206] && (!s.b[1207])) {s.store_scalar(1041, 0.0);s.store_scalar(1044, 0.0);}
        if (!s.b[1206]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1045, 944, A::div(s.ad_value(881), s.ad_value(947)), (-2.0), 950, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1046, 945, A::div(s.ad_value(882), s.ad_value(948)), (-2.0), 950, (-2.0));s.store_mul_sub_lhs(0, 1046, 1045, 950);s.store_mul(2, 1045, 881);s.store_mul(3, 1046, 882);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(944), s.ad_value(881), 2.0, s.ad_value(945), s.ad_value(882), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1047, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(947)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1048, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(948)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1041, 947, 950, -1.0, 1047, 947, -1.0);s.store_mul_add_scaled_product_rhs_indices(1044, 948, 950, -1.0, 1048, 948, -1.0);}
        s.store_mul(1049, 1041, 1028);s.store_mul(1050, 1044, 1028);s.store_scaled_sub(1051, 979, 914, 0.5);s.store_scaled_sub(1052, 980, 915, 0.5);s.store_mul(1053, 1051, 1049);s.store_mul(1054, 1052, 1050);s.copy_ad(379, 875);s.copy_ad(380, 879);s.copy_ad(381, 880);s.copy_ad(382, 881);s.copy_ad(383, 882);s.copy_ad(384, 909);s.copy_ad(385, 910);s.copy_ad(386, 894);s.copy_ad(387, 893);s.copy_ad(388, 912);s.copy_ad(389, 897);s.copy_ad(390, 898);s.copy_ad(391, 899);s.copy_ad(392, 900);s.copy_ad(393, 901);s.copy_ad(394, 904);s.copy_ad(395, 906);s.copy_ad(396, 905);s.copy_ad(397, 907);s.copy_ad(398, 908);s.copy_ad(399, 913);s.copy_ad(400, 914);s.copy_ad(401, 915);s.copy_ad(402, 926);s.copy_ad(403, 956);s.copy_ad(404, 979);s.copy_ad(405, 980);s.copy_ad(407, 975);s.copy_ad(408, 976);s.copy_ad(409, 978);s.copy_ad(410, 990);s.copy_ad(411, 991);s.copy_ad(412, 995);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(413, 1002);s.copy_ad(414, 1003);s.copy_ad(415, 1004);s.copy_ad(416, 1005);s.copy_ad(417, 1012);s.copy_ad(418, 1018);s.copy_ad(419, 1019);s.copy_ad(420, 1021);s.copy_ad(421, 1023);s.copy_ad(422, 1027);s.copy_ad(424, 1026);s.copy_ad(425, 1028);s.copy_ad(426, 1029);s.copy_ad(427, 1030);s.copy_ad(428, 1032);s.copy_ad(429, 1038);s.copy_ad(431, 1041);s.copy_ad(432, 1051);s.copy_ad(433, 1052);s.copy_ad(434, 1053);s.copy_ad(435, 1054);s.store_div_scaled_inputs_mixed_ia(338, 417, p.p35, A::add(s.ad_value(413), s.ad_value(414)), 1.0);s.store_mul_add_scaled_product_rhs_indices(339, 420, 63, 1.0, 271, 419, 1.0);s.store_mul_scale_offset_mixed_ia(340, 421, A::mul_offset_rhs(s.ad_value(339), s.ad_value(339), 1.0), 1.0, 1.0);s.store_mul3_lhs(341, 418, 421, 422);s.b[1208] = (p.p13 > 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if s.b[1208] {s.store_div_scaled_inputs2_mixed_iia(342, 413, 1.0, 414, 1.0, A::add(A::div(s.ad_value(413), s.ad_value(426)), A::div(s.ad_value(414), s.ad_value(427))), 1.0);}
        if (!s.b[1208]) {s.store_scalar(342, 1.0);}
        s.store_mul_square_lhs(343, 222, 338);s.store_div_scaled_product_by_product_mixed_aiii(344, A::mul3(s.ad_value(343), s.ad_value(386), s.ad_value(429)), 340, 1.0, 341, 342, 1.0);s.store_mul_scale_offset_indices(700, 220, 326, -1.0, 0.0);s.store_mul_scale_offset_indices(701, 220, 328, -1.0, 0.0);s.store_add_scaled_product_indices(0, 230, 1.0, 163, 220, p.p14);s.store_add(702, 700, 0);s.store_add(703, 701, 0);s.store_scalar(710, 0.0);s.store_scalar(711, 0.0);s.store_scalar(712, 0.0);s.store_scalar(713, 0.0);s.store_div_mixed_ai(704, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);s.store_square(705, 704);s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);s.store_scale(707, 706, 1e-5);s.store_div_from_scalar(708, 1.0, 706);s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);s.b[1209] = (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0)));s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });s.b[1210] = (((s.v[700]) as f64).abs() <= s.v[707]);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if (s.b[1209] && s.b[1210]) {s.store_mul_scale_offset_indices(710, 708, 700, -1.0, 0.0);}
        s.b[1211] = (s.v[700] < (-s.v[707]));s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_neg(679, 700);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1212] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if (((s.b[1209] && (!s.b[1210])) && s.b[1211]) && s.b[1212]) {s.store_exp(689, 688);}
        s.b[1213] = (s.v[688] < (-80.0));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && s.b[1213]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(710, 688, 693);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {s.store_add_scaled_inputs3_mixed_iia(690, 700, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(700), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(710, 696, 697);}
        if (s.b[1209] && (!s.b[1210])) {s.store_neg(710, 710);}
        s.b[1218] = (s.v[159] > 0.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });s.b[1219] = (((s.v[702]) as f64).abs() <= s.v[707]);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if (s.b[1218] && s.b[1219]) {s.store_mul_scale_offset_indices(712, 708, 702, -1.0, 0.0);}
        s.b[1220] = (s.v[702] < (-s.v[707]));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_neg(679, 702);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1221] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if (((s.b[1218] && (!s.b[1219])) && s.b[1220]) && s.b[1221]) {s.store_exp(689, 688);}
        s.b[1222] = (s.v[688] < (-80.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && s.b[1222]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && (!s.b[1222])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(712, 688, 693);}
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
        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {s.store_add_scaled_inputs3_mixed_iia(690, 702, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(702), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(712, 696, 697);}
        if (s.b[1218] && (!s.b[1219])) {s.store_neg(712, 712);}
        s.store_div_mixed_ai(704, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);s.store_square(705, 704);s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);s.store_scale(707, 706, 1e-5);s.store_div_from_scalar(708, 1.0, 706);s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);s.b[1227] = (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0)));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });s.b[1228] = (((s.v[701]) as f64).abs() <= s.v[707]);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if (s.b[1227] && s.b[1228]) {s.store_mul_scale_offset_indices(711, 708, 701, -1.0, 0.0);}
        s.b[1229] = (s.v[701] < (-s.v[707]));s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {s.store_neg(679, 701);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1230] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (((s.b[1227] && (!s.b[1228])) && s.b[1229]) && s.b[1230]) {s.store_exp(689, 688);}
        s.b[1231] = (s.v[688] < (-80.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && s.b[1231]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && (!s.b[1231])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && s.b[1235]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && (!s.b[1235])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {s.store_add_scaled_inputs3_mixed_iia(690, 701, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(701), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(711, 696, 697);}
        if (s.b[1227] && (!s.b[1228])) {s.store_neg(711, 711);}
        s.b[1236] = (s.v[160] > 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });s.b[1237] = (((s.v[703]) as f64).abs() <= s.v[707]);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if (s.b[1236] && s.b[1237]) {s.store_mul_scale_offset_indices(713, 708, 703, -1.0, 0.0);}
        s.b[1238] = (s.v[703] < (-s.v[707]));s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_neg(679, 703);s.store_scaled_mul(680, 679, 708, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(681, 680, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(682, A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);s.store_sub_ln_div_lhs(684, 682, 705, 681);s.store_add(685, 682, 683);s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);s.store_add_product3_rhs_mixed_aia(687, 686, A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), 683, A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0), 1.0);s.store_add_mixed_ia(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));}
        s.b[1239] = (((s.v[688]) as f64).abs() < 80.0);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if (((s.b[1236] && (!s.b[1237])) && s.b[1238]) && s.b[1239]) {s.store_exp(689, 688);}
        s.b[1240] = (s.v[688] < (-80.0));s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(689, 688, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_sub(687, 679, 688);s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_neg_add(713, 688, 693);}
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
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && s.b[1244]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && (!s.b[1244])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(689, A::neg(s.ad_value(696)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {s.store_add_scaled_inputs3_mixed_iia(690, 703, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);s.store_add_scaled_square_product_mixed_aia(691, A::sub(s.ad_value(703), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);s.store_add(713, 696, 697);}
        if (s.b[1236] && (!s.b[1237])) {s.store_neg(713, 713);}
        s.store_mul_add_scaled_inputs_rhs_indices(714, 219, 700, -1.0, 710, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(715, 219, 701, -1.0, 711, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(345, 219, 702, -1.0, 712, -1.0);s.store_mul_add_scaled_inputs_rhs_indices(346, 219, 703, -1.0, 713, -1.0);s.b[1245] = (p.p3 > 0.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });s.b[1246] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1246]) {s.store_add(716, 714, 281);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(714)), 0.0001));}
        s.b[1247] = ((((0.5 * s.v[700])) as f64).abs() < 80.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1247]) {s.store_exp_scaled_input(0, 700, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
    ) {
        s.b[1248] = ((0.5 * s.v[700]) < (-80.0));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && (!s.b[1248])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(700), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);s.store_mul_div_scaled_inputs_indices(2, 275, 81, (-1.0), 718, 1.0);}
        s.b[1249] = (s.v[720] < 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1249]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 721, 0.5, 718, 721, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1246]) {s.store_add_scaled_product_mixed_aii(724, A::offset(s.ad_value(710), 3.0), 1.0, 717, 220, 1.0);}
        s.b[1250] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1250]) {s.store_exp(725, 724);}
        s.b[1251] = (s.v[724] < (-80.0));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && s.b[1251]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && (!s.b[1251])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_add_mixed_ai(724, A::add_scaled_product(A::offset(s.ad_value(710), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 700);}
        s.b[1252] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1252]) {s.store_exp(726, 724);}
        s.b[1253] = (s.v[724] < (-80.0));s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && s.b[1253]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && (!s.b[1253])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(726, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_mul_scale_offset_mixed_ia(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), 1.0, (-1.5));s.store_div_scaled_offset_numerator_mixed_ia(0, 725, 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);}
        s.b[1258] = (s.v[0] < 1e-80);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1258]) {s.store_scalar(0, 1e-80);}
        if (s.b[1245] && s.b[1246]) {s.store_mul_sub_rhs(2, 85, 328, 86);}
        s.b[1259] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1259]) {s.store_exp(3, 2);}
        s.b[1260] = (s.v[2] < (-80.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && s.b[1260]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && (!s.b[1260])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1246]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 699, 1.0);}
        s.b[1261] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1246]) && s.b[1261]) {s.store_exp(5, 4);}
        s.b[1262] = (s.v[4] < (-80.0));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && s.b[1262]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && (!s.b[1262])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1263] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1263]) {s.store_add(716, 715, 281);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1245] && s.b[1263]) {s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(715)), 0.0001));}
        s.b[1264] = ((((0.5 * s.v[701])) as f64).abs() < 80.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1264]) {s.store_exp_scaled_input(0, 701, 0.5);}
        s.b[1265] = ((0.5 * s.v[701]) < (-80.0));s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && s.b[1265]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && (!s.b[1265])) {s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(701), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);s.store_sub_from_scalar(3, 1.0, 2);s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);s.store_mul_div_scaled_inputs_indices(2, 275, 81, (-1.0), 718, 1.0);}
        s.b[1266] = (s.v[720] < 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1266]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 721, 0.5, 718, 721, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1263]) {s.store_add_scaled_product_mixed_aii(724, A::offset(s.ad_value(711), 3.0), 1.0, 717, 220, 1.0);}
        s.b[1267] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1267]) {s.store_exp(725, 724);}
        s.b[1268] = (s.v[724] < (-80.0));s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && s.b[1268]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && (!s.b[1268])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_add_mixed_ai(724, A::add_scaled_product(A::offset(s.ad_value(711), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 701);}
        s.b[1269] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1269]) {s.store_exp(726, 724);}
        s.b[1270] = (s.v[724] < (-80.0));s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && s.b[1270]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && (!s.b[1270])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(726, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_mul_scale_offset_mixed_ia(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), 1.0, (-1.5));s.store_div_scaled_offset_numerator_mixed_ia(0, 725, 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);}
        s.b[1275] = (s.v[0] < 1e-80);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1275]) {s.store_scalar(0, 1e-80);}
        if (s.b[1245] && s.b[1263]) {s.store_mul_sub_rhs(2, 85, 326, 86);}
        s.b[1276] = (((s.v[2]) as f64).abs() < 80.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1276]) {s.store_exp(3, 2);}
        s.b[1277] = (s.v[2] < (-80.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && s.b[1277]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && (!s.b[1277])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 2, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1263]) {s.store_add_scaled_product_indices(4, 2, 1.0, 85, 698, 1.0);}
        s.b[1278] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1263]) && s.b[1278]) {s.store_exp(5, 4);}
        s.b[1279] = (s.v[4] < (-80.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && s.b[1279]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && (!s.b[1279])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1280] = (s.v[68] > 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (s.b[1245] && s.b[1280]) {s.store_mul_scale_offset_indices(731, 382, 432, -1.0, 0.0);}
        s.b[1281] = (((((2.0 * s.v[731]) - s.v[407])) as f64).abs() < 80.0);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1245] && s.b[1280]) && s.b[1281]) {s.store_exp_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0));}
        s.b[1282] = (((2.0 * s.v[731]) - s.v[407]) < (-80.0));s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && s.b[1282]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && (!s.b[1282])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul_sub_mixed_iaa(732, 222, A::offset(s.ad_value(731), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0)));s.store_scaled_add(733, 388, 408, 0.5);s.store_mul(734, 222, 733);s.store_add(716, 734, 280);s.store_scaled_sub_mixed_ia(717, 716, A::sqrt_square_offset(A::neg(s.ad_value(716)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(718, 272, A::offset(A::square(s.ad_value(734)), 0.0001));}
        s.b[1283] = (s.v[79] < 0.0);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1283]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(718, 718, 0.5, 276, 0.5, 718, 276, 1e-6, (-0.5));}
        if (s.b[1245] && s.b[1280]) {s.store_add(736, 396, 230);s.store_sub(735, 736, 733);s.store_mul_add_scaled_product_rhs_mixed_iai(724, 282, 735, 1.0, A::add_scaled_inputs3(s.ad_value(717), 1.0, s.ad_value(279), (-1.0), s.ad_value(732), -1.0), 223, 1.0);}
        s.b[1284] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1284]) {s.store_exp(725, 724);}
        s.b[1285] = (s.v[724] < (-80.0));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && s.b[1285]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && (!s.b[1285])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(725, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul_ad_affine_product_lhs(724, A::sub(s.ad_value(331), s.ad_value(732)), s.ad_value(223), -1.0, 0.0, 282);}
        s.b[1286] = (((s.v[724]) as f64).abs() < 80.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && s.b[1286]) {s.store_exp(0, 724);}
        s.b[1287] = (s.v[724] < (-80.0));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && s.b[1287]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && (!s.b[1287])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 724, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (s.b[1245] && s.b[1280]) {s.store_mul(726, 725, 0);s.store_mul_scale_offset_mixed_ia(0, 274, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(718), 1.0)), 1.0, (-1.5));}
        s.b[1291] = ((s.v[736] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if ((s.b[1245] && s.b[1280]) && (!s.b[1291])) {s.store_add_scaled_product_indices(0, 78, 1.0, 79, 718, 2.0);s.store_mul_div_mixed_iia(740, 223, 87, A::mul(s.ad_value(0), s.ad_value(274)));s.store_div(741, 731, 740);}
        s.b[1292] = (s.v[741] < 0.001);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });s.b[1293] = (((s.v[741]) as f64).abs() < 80.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {s.store_exp(747, 741);}
        s.b[1294] = (s.v[741] < (-80.0));s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(747, 1.80485e-35, A::neg(s.ad_value(741)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(747, 741, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {s.store_div_from_scalar(748, 1.0, 747);s.store_sub(0, 747, 748);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {s.store_add(3, 747, 748);}
        s.b[1296] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[714] < 0.0));s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if s.b[1296] {s.store_sqrt_offset_ad(751, A::add(A::square(s.ad_value(714)), A::mul3(A::square(s.ad_value(95)), s.ad_value(327), s.ad_value(327))), 1e-6);s.store_div_scaled_inputs_indices(0, 91, -1.0, 751, 1.0);}
        s.b[1297] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if (s.b[1296] && s.b[1297]) {s.store_exp(3, 0);}
        s.b[1298] = (s.v[0] < (-80.0));s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if ((s.b[1296] && (!s.b[1297])) && s.b[1298]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1296] && (!s.b[1297])) && (!s.b[1298])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1296] {s.store_mul(4, 97, 699);}
        s.b[1299] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1296] && s.b[1299]) {s.store_exp(5, 4);}
        s.b[1300] = (s.v[4] < (-80.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if ((s.b[1296] && (!s.b[1299])) && s.b[1300]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1296] && (!s.b[1299])) && (!s.b[1300])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1301] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[715] < 0.0));s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if s.b[1301] {s.store_sqrt_offset_ad(752, A::add(A::square(s.ad_value(715)), A::mul3(A::square(s.ad_value(96)), s.ad_value(329), s.ad_value(329))), 1e-6);s.store_div_scaled_inputs_indices(0, 92, -1.0, 752, 1.0);}
        s.b[1302] = (((s.v[0]) as f64).abs() < 80.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (s.b[1301] && s.b[1302]) {s.store_exp(3, 0);}
        s.b[1303] = (s.v[0] < (-80.0));s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if ((s.b[1301] && (!s.b[1302])) && s.b[1303]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1301] && (!s.b[1302])) && (!s.b[1303])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(3, 0, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1301] {s.store_mul(4, 98, 698);}
        s.b[1304] = (((s.v[4]) as f64).abs() < 80.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if (s.b[1301] && s.b[1304]) {s.store_exp(5, 4);}
        s.b[1305] = (s.v[4] < (-80.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1301] && (!s.b[1304])) && s.b[1305]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1301] && (!s.b[1304])) && (!s.b[1305])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(5, 4, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.store_scalar(352, 0.0);s.b[1306] = (p.p12 > 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if s.b[1306] {s.store_mul(754, 332, 285);s.store_mul_scale_offset_mixed_ia(755, 285, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(756, 754, 755, 0.5);s.store_sub_mixed_ai(757, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub(s.ad_value(331), s.ad_value(100)), s.ad_value(285), 1.0), 230);s.store_sub_mixed_ai(758, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(101), 1.0), s.ad_value(285), 1.0), 230);s.store_div_from_scalar_offset_input(759, 1.0, 105, 1.0);s.store_div_from_scalar_offset_input(760, 1.0, 106, 1.0);s.store_mul(761, 109, 285);s.store_mul_scaled_offset_ad_rhs(0, 761, 2.0, A::sqrt(A::offset(A::div(s.ad_value(755), s.ad_value(761)), 1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1306] {s.store_mul(762, 107, 0);s.store_mul(763, 108, 0);s.store_add_scaled_product_mixed_iai(764, 756, 1.0, A::add(s.ad_value(757), s.ad_value(762)), 759, 1.0);s.store_add_scaled_product_mixed_iai(765, 756, 1.0, A::add(s.ad_value(758), s.ad_value(763)), 760, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(766, A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), s.ad_value(221))), 0.01), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_aia(767, A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), s.ad_value(221))), 0.01), (-0.5));s.store_div(768, 242, 759);s.store_div(769, 243, 760);s.store_div_from_scalar(770, 1.0, 768);s.store_div_from_scalar(771, 1.0, 769);s.store_div_from_scalar_add_ad(772, 1.0, A::offset(s.ad_value(770), 1.0), s.ad_value(771));s.store_div_square_rhs(773, 286, 386);s.store_mul_sub_rhs(774, 772, 766, 767);}
        s.b[1307] = ((((s.v[767] - s.v[766])) as f64).abs() <= 1e-12);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1307]) {s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(772), s.ad_value(770)), 1.0, 772, 771, (-1.0));s.store_mul_add_scaled_inputs4_rhs_mixed_iaaa(3, 774, 771, 1.0, A::mul3_scaled_output(s.ad_value(770), s.ad_value(772), s.ad_value(770), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(771), s.ad_value(772), s.ad_value(771), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(772)), -1.0);s.store_div_scaled_product_mixed_aii(4, A::sub(s.ad_value(2), s.ad_value(3)), 773, 0.5, 772, 1.0);}
        if (s.b[1306] && (!s.b[1307])) {s.store_exp_mul_scaled_lhs_indices(2, 770, -1.0, 774);s.store_exp_ad(3, A::mul(A::sub(s.ad_value(771), A::div_from_scalar(1.0, s.ad_value(772))), s.ad_value(774)));s.store_div_scaled_product_mixed_iai(4, 773, A::sub(s.ad_value(2), s.ad_value(3)), 1.0, 774, 2.0);}
        if s.b[1306] {s.copy_ad(775, 4);}
        s.b[1308] = (s.v[766] < 80.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1308]) {s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(s.ad_value(766))), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1306] && s.b[1308]) {s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        s.b[1309] = (s.v[766] < 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });s.b[1310] = (s.v[766] > (-80.0));s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && s.b[1310]) {s.store_exp(780, 766);}
        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && (!s.b[1310])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(780, 1.80485e-35, A::neg(s.ad_value(766)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1306] && (!s.b[1308])) && s.b[1309]) {s.store_mul(0, 775, 780);}
        if ((s.b[1306] && (!s.b[1308])) && (!s.b[1309])) {s.store_add_ln_lhs(780, 775, 766);s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        if s.b[1306] {s.copy_ad(776, 0);}
        s.b[1311] = ((s.v[766] - s.v[407]) < 80.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1306] && s.b[1311]) {s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(A::sub(s.ad_value(766), s.ad_value(407)))), 1.0));s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        s.b[1312] = ((s.v[766] - s.v[407]) < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });s.b[1313] = ((s.v[766] - s.v[407]) > (-80.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && s.b[1313]) {s.store_exp_sub(780, 766, 407);}
        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && (!s.b[1313])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(780, 1.80485e-35, A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1306] && (!s.b[1311])) && s.b[1312]) {s.store_mul(0, 775, 780);}
        if ((s.b[1306] && (!s.b[1311])) && (!s.b[1312])) {s.store_add_scaled_inputs3_mixed_aii(780, A::ln(s.ad_value(775)), 1.0, 766, 1.0, 407, (-1.0));s.store_mul_scale_offset_mixed_ia(0, 780, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)), -1.0, 1.0);}
        if s.b[1306] {s.copy_ad(777, 0);s.store_mul_scale_offset(778, A::sub(s.ad_value(776), s.ad_value(777)), A::add_scaled_inputs(s.ad_value(776), 0.5, s.ad_value(777), 0.5), 1.0, 1.0);s.store_mul_square_lhs(779, 284, 110);s.store_div_scaled_product3_indices(352, 779, 237, 778, 1.0, 418, 1.0);}
        s.b[1314] = (p.p8 != 0.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if s.b[1314] {s.store_div_scaled_add_product_indices(753, 335, 1.0, 115, 407, (-1.0), 223, 1.0);}
        s.b[1315] = (s.v[753] > 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (s.b[1314] && s.b[1315]) {s.store_div_scaled_value_offset_denominator(3, s.ad_value(113), (-1.0), s.ad_value(753), 1e-30, 1.0);}
        s.b[1316] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if ((s.b[1314] && s.b[1315]) && s.b[1316]) {s.store_exp(0, 3);}
        s.b[1317] = (s.v[3] < (-80.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && s.b[1317]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(0, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && (!s.b[1317])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(0, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1318] = (s.v[6] > 0.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1318] {s.store_mul_abs_mixed_ia(0, 168, A::mul(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(332)));}
        s.b[1604] = (p.p11 > 0.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if s.b[1604] {s.copy_ad(1414, 130);s.copy_ad(1415, 131);s.copy_ad(1416, 135);s.copy_ad(1417, 136);s.copy_ad(1418, 140);s.copy_ad(1419, 141);s.copy_ad(1420, 270);s.copy_ad(1421, 212);s.copy_ad(1422, 158);s.store_sub_mixed_ai(1423, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(1414)), s.ad_value(223), 1.0), 230);s.store_add_scaled_product_mixed_iai(1424, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1415), 1.0), 223, 1.0);s.store_sub(1425, 1424, 230);}
        s.b[1605] = (p.p2 > 0.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1605]) {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(1426, 242, 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);s.store_ln(1427, 1426);}
        s.b[1606] = (s.v[1427] > 1e-8);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1606]) {s.store_div_scaled_product_offset_denominator_mixed_iai(1428, 1427, A::offset(s.ad_value(1426), 1.0), 2.0, 1426, (-1.0), 1.0);}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1606])) {s.store_scaled_offset(1428, 1427, 2.0, 2.0);}
        if (s.b[1604] && s.b[1605]) {s.store_div_square_rhs(1429, 249, 241);s.store_div_from_scalar(1430, 1.0, 242);s.store_div_from_scalar(1431, 1.0, 243);s.store_div_from_scalar_add_ad(1458, 1.0, A::offset(s.ad_value(1430), 1.0), s.ad_value(1431));s.store_mul_sub_rhs(1459, 1458, 1423, 1425);s.store_add_scaled_product_indices(1432, 1423, 1.0, 1459, 1430, (-1.0));s.store_add_scaled_product_indices(1433, 1425, 1.0, 1459, 1431, 1.0);s.store_div_from_scalar_offset_input(1338, 1.0, 242, 1.0);s.store_div_from_scalar_offset_input(1339, 1.0, 243, 1.0);s.store_offset_ln_ad(1341, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);s.store_offset_ln_ad(1342, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);}
        s.b[1607] = (((s.v[1341] - s.v[1432]) / 1.5) < 80.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1607]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.6666666666666666, s.ad_value(1432), 0.6666666666666666));}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1607])) {s.store_scaled_sub(1340, 1341, 1432, 0.6666666666666666);}
        if (s.b[1604] && s.b[1605]) {s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 1.5);s.store_mul_add_scaled_product_rhs_indices(1344, 1339, 1345, 1.0, 243, 1425, 1.0);}
        s.b[1608] = (((s.v[1342] - s.v[1344]) / 1.5) < 80.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1608]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.6666666666666666, s.ad_value(1344), 0.6666666666666666));}
        if ((s.b[1604] && s.b[1605]) && (!s.b[1608])) {s.store_scaled_sub(1340, 1342, 1344, 0.6666666666666666);}
        if (s.b[1604] && s.b[1605]) {s.store_sub_scaled_inputs(1, 1342, 1.0, 1340, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 1425);s.store_sub(1390, 2, 3);}
        s.b[1609] = ((((-s.v[262])) as f64).abs() < 80.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1605]) && s.b[1609]) {s.store_exp_neg_input(1391, 262);}
        s.b[1610] = ((-s.v[262]) < (-80.0));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
    }
}
