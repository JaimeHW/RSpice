#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1019] {s.store_div_scaled_product_by_product_mixed_aaai(229, A::mul3_scaled_output(s.ad_value(270), s.ad_value(86), s.ad_value(158), s.v[466]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 3.0, 1.0), 1.0, s.ad_value(278), 6.0), s.ad_value(230), s.ad_value(230)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 4.0, 3.0), 1.0, s.ad_value(278), 3.0), s.ad_value(230), s.ad_value(158)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(85), 3.0, 6.0), s.ad_value(278)), s.ad_value(158), s.ad_value(158)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(386), A::offset(s.ad_value(85), 1.0), s.ad_value(228), 15.0), 228, 1.0);}
        if (!s.b[1019]) {s.store_scalar(229, 0.0);}
        s.b[1022] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if s.b[1022] {s.store_sqrt(235, 233);s.store_add(280, 86, 235);s.store_square(281, 231);s.store_square(282, 233);s.store_scaled_mul(283, 231, 233, 42.0);s.store_add_scaled_inputs3_indices(283, 283, 1.0, 281, 4.0, 282, 4.0);s.store_add_product3_rhs_mixed_iia(283, 283, 235, 86, A::add(s.ad_value(231), s.ad_value(233)), 20.0);s.store_square(288, 280);s.store_div_scaled_value_by_product_mixed_iai(236, 283, 1.0, A::square(s.ad_value(288)), 280, 1.0);s.store_mul_ad_product_lhs_mixed_ai(237, A::div_from_scalar(s.v[466], s.ad_value(386)), 158, 270);s.store_add_mixed_ai(285, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 4.0), 233);}
        s.store_add(94, 94, 193);
        if s.b[517] {s.store_scalar(200, ((-p.p172) * s.v[277]));s.store_mul_sub_rhs(201, 200, 42, 40);}
        if (!s.b[517]) {s.store_scalar(200, 0.0);s.store_scalar(201, 0.0);}
        s.store_scalar(215, (((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[513]) * (((1.0 + (p.p171 / s.v[272]))) as f64).ln()));s.store_scaled_sub(216, 42, 41, s.v[215]);s.store_scale(217, 42, s.v[215]);s.store_add(197, 197, 216);s.store_add(196, 196, 217);s.store_scale(0, 94, s.v[394]);s.store_scale(279, 123, (-s.v[513]));s.store_scaled_add(280, 523, 576, (-0.5));s.store_scaled_add(281, 531, 585, (-0.5));s.store_scaled_mul(444, 279, 40, (0.1 * s.v[294]));s.store_mul_sub_scaled_inputs_rhs_indices(443, 279, 40, (0.1 * s.v[294]), 41, (0.1 * s.v[294]));s.store_mul(441, 279, 280);s.store_mul(442, 279, 281);
        if (p.p303 != 0.0) {s.store_scalar(336, 0.0);s.copy_ad(92, 91);}
        if (p.p303 == 0.0) {s.store_add_scaled_inputs3_indices(92, 91, 1.0, 441, 1.0, 442, 1.0);}
        s.store_scale(93, 92, s.v[385]);
        if (s.v[38] != 0.0) {s.store_scalar(15, 0.0);s.store_scalar(14, 0.0);s.store_scalar(492, 0.0);s.store_scale(556, 336, s.v[394]);s.store_scale(555, 92, s.v[394]);}
        if (s.v[38] == 0.0) {s.store_sub_scaled_inputs(14, 336, (-s.v[394]), 92, s.v[394]);s.store_scaled_add(15, 93, 443, s.v[394]);s.store_add_scaled_inputs3_indices(16, 92, s.v[394], 93, ((-1.0) * s.v[394]), 444, s.v[394]);}
        s.b[1023] = (p.p45 == 0.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if s.b[1023] {s.store_scalar(219, 0.0);}
        if (!s.b[1023]) {s.store_add_scaled_product_indices(218, 56, 1.0, 261, 123, 1.0);}
        s.b[1024] = (s.v[218] > s.v[260]);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[1023]) && s.b[1024]) {s.copy_ad(218, 260);}
        if (!s.b[1023]) {s.store_add_scaled_inputs3_indices(279, 51, s.v[264], 56, s.v[264], 218, (1.0 - s.v[264]));s.store_sqrt_div_from_scalar_ad(288, (2.0 * 1.034943e-10), s.ad_value(126));s.store_scale(281, 288, 1.3);s.store_scale(280, 281, (1.034943e-10 * s.v[513]));s.store_mul_add_scaled_inputs4_indices_rhs(219, 280, 56, 1.0 / (p.p45), 51, 1.0 / (p.p45), 279, (-1.0 / (p.p45)), 261, -1.0);}
        s.b[1025] = (p.p46 != 0.0);s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if s.b[1025] {s.store_add_scaled_inputs(219, 219, 1.0, 50, s.v[490]);}
        s.b[1026] = (p.p14 == 1.0);s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if s.b[1026] {s.store_add_mixed_ia(14, 14, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(197), 1.0, s.ad_value(196), 1.0, s.ad_value(201), -1.0, s.ad_value(219), -1.0), s.ad_value(398)), s.v[394], s.ad_value(397), s.v[394]));s.store_add_scaled_inputs4_indices(15, 15, 1.0, 219, s.v[394], 197, ((-1.0) * s.v[394]), 405, s.v[394]);s.store_add_scaled_inputs3_indices(16, 16, 1.0, 406, s.v[394], 196, (-s.v[394]));}
        s.store_scale(494, 185, s.v[394]);s.b[1027] = (s.v[575] == 1.0);s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if (!s.b[1027]) {s.store_sub_from_scalar(279, 1.0, 256);}
        s.b[1028] = (s.v[575] == 1.0);s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if s.b[1028] {s.store_sub_from_scalar(279, 1.0, 256);}
        s.store_scale(573, 374, (4.0 * 1.3806226e-23));s.store_scale(564, 229, s.v[394]);s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));s.store_scale(18, 18, p.p33);s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));s.store_scale(19, 19, p.p33);
        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }
        s.b[1029] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if s.b[1029] {s.store_scaled_mul(278, 270, 123, (1e-6 * s.v[513]));s.store_scale(288, 493, 1.0 / (s.v[394]));s.store_div_scaled_product3_indices(241, 122, 288, 288, (0.1185185185185185 * 1.6021918e-19), 237, 1.0);}
        s.b[1030] = ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if (s.b[1029] && s.b[1030]) {s.store_div(242, 159, 158);s.store_div_scaled_inputs2_mixed_aii(243, A::div(s.ad_value(159), s.ad_value(230)), 1.0, 242, (-1.0), 51, 1.0);s.store_add_mixed_ia(244, 242, A::div_scaled_product(s.ad_value(243), A::add(A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 1.0), s.ad_value(233)), 0.6666666666666667, A::add(s.ad_value(86), s.ad_value(235)), 1.0));}
        if (s.b[1029] && (!s.b[1030])) {s.store_div(244, 159, 230);}
        if s.b[1029] {s.store_mul3_affine_lhs(495, 241, 236, s.v[394], 0.0, 244);}
        if s.b[1029] {
            if (s.v[495] < 0.0) {
                s.store_scalar(495, 0.0);
            } else {
            }
        }
        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(495, 0.0);
            }
        }
        if (!s.b[1029]) {s.store_scalar(495, 0.0);}
        s.store_mul(608, 573, 564);
        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_div(610, 495, 608);
        } else {
            s.store_scalar(610, 0.0);
        }
        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }
        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }
        s.b[1031] = (p.p312 == 1.0);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if s.b[1031] {s.store_scalar(1035, p.p317);s.store_scalar(1036, p.p319);s.store_scalar(1037, p.p324);s.store_scalar(1041, p.p311);s.store_scaled_voltage(1039, ctx, nodes, Some(12), Some(2), p.p33);s.store_primal_scale(1035, 1035, 0.0001);s.store_primal_scale(1036, 1036, 0.01);s.store_scale(1040, 374, 1.0 / (s.v[445]));s.store_powf(279, 1040, p.p320);s.store_div(1043, 1035, 279);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1031] {s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1040), 0.4, 1.8), 1.0, s.ad_value(1040), s.ad_value(1040), 0.1), A::scale_offset(s.ad_value(1040), (-p.p321), p.p321));s.store_div(1044, 1036, 278);s.store_add_mixed_ia(1037, 1037, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));s.store_mul(1043, 1043, 1032);s.store_offset_product3(1044, s.ad_value(1044), s.ad_value(1033), s.ad_value(1034), 1.0, 1e-50);s.store_div(1045, 1039, 1041);s.store_mul(1046, 1043, 1045);}
        s.b[1051] = (s.v[1039] >= 0.0);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1051]) {s.store_div(279, 1046, 1044);}
        if (s.b[1031] && (!s.b[1051])) {s.store_div_scaled_inputs_indices(279, 1046, -1.0, 1044, 1.0);}
        s.b[1052] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1052]) {s.store_scalar(281, 1.0);}
        s.b[1053] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if ((s.b[1031] && (!s.b[1052])) && s.b[1053]) {s.copy_ad(281, 279);}
        if ((s.b[1031] && (!s.b[1052])) && (!s.b[1053])) {s.store_pow_offset_rhs(281, 279, 1037, (-1.0));}
        if s.b[1031] {s.store_mul(280, 279, 281);s.store_offset(282, 280, 1.0);}
        s.b[1054] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1054]) {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[1055] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if ((s.b[1031] && (!s.b[1054])) && s.b[1055]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((s.b[1031] && (!s.b[1054])) && (!s.b[1055])) {s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0)));s.store_mul(283, 282, 284);}
        if s.b[1031] {s.store_div_from_scalar(279, 1.6021918e-19, 1041);}
        s.b[1058] = (p.p313 == 1.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if s.b[1058] {s.store_scalar(1062, p.p316);s.store_scalar(1063, p.p318);s.store_scalar(1064, p.p323);s.store_scalar(1068, p.p310);s.store_scaled_voltage(1066, ctx, nodes, Some(0), Some(11), p.p33);s.store_primal_scale(1062, 1062, 0.0001);s.store_primal_scale(1063, 1063, 0.01);s.store_scale(1067, 374, 1.0 / (s.v[445]));s.store_powf(279, 1067, p.p320);s.store_div(1070, 1062, 279);s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1067), 0.4, 1.8), 1.0, s.ad_value(1067), s.ad_value(1067), 0.1), A::scale_offset(s.ad_value(1067), (-p.p321), p.p321));s.store_div(1071, 1063, 278);s.store_add_mixed_ia(1064, 1064, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));s.store_mul(1070, 1070, 1059);s.store_offset_product3(1071, s.ad_value(1071), s.ad_value(1060), s.ad_value(1061), 1.0, 1e-50);s.store_div(1072, 1066, 1068);s.store_mul(1073, 1070, 1072);}
        s.b[1078] = (s.v[1066] >= 0.0);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1078]) {s.store_div(279, 1073, 1071);}
        if (s.b[1058] && (!s.b[1078])) {s.store_div_scaled_inputs_indices(279, 1073, -1.0, 1071, 1.0);}
        s.b[1079] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1079]) {s.store_scalar(281, 1.0);}
        s.b[1080] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if ((s.b[1058] && (!s.b[1079])) && s.b[1080]) {s.copy_ad(281, 279);}
        if ((s.b[1058] && (!s.b[1079])) && (!s.b[1080])) {s.store_pow_offset_rhs(281, 279, 1064, (-1.0));}
        if s.b[1058] {s.store_mul(280, 279, 281);s.store_offset(282, 280, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1081] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1081]) {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[1082] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if ((s.b[1058] && (!s.b[1081])) && s.b[1082]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((s.b[1058] && (!s.b[1081])) && (!s.b[1082])) {s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0)));s.store_mul(283, 282, 284);}
        if s.b[1058] {s.store_div_from_scalar(279, 1.6021918e-19, 1068);}
        s.b[1085] = (s.v[221] < 1e-18);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1085]) {s.store_scalar(221, 1e-18);}
        s.b[1086] = (s.v[222] < 1e-18);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1086]) {s.store_scalar(222, 1e-18);}
        if (s.v[38] != 0.0) {s.store_div_scaled_inputs2_indices(549, 551, 1.0, 555, (-1.0), 221, 1.0);s.store_div_scaled_inputs2_indices(550, 548, 1.0, 556, (-1.0), 222, 1.0);s.store_sub_scaled_inputs(554, 551, -1.0, 548, 1.0);s.store_scale(552, 551, s.v[385]);s.store_scale(553, 551, (1.0 - s.v[385]));}
        if (s.v[38] == 0.0) {s.store_scalar(549, 0.0);s.store_scalar(550, 0.0);s.store_scalar(552, 0.0);s.store_scalar(553, 0.0);s.store_scalar(554, 0.0);s.store_scalar(548, 0.0);}
        s.b[1087] = (s.v[575] == 1.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if s.b[1087] {s.copy_ad(94, 0);s.copy_ad(185, 494);s.copy_ad(561, 14);s.copy_ad(93, 15);s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));s.copy_ad(90, 492);}
        if (!s.b[1087]) {s.store_neg(94, 0);s.store_scalar(185, 0.0);s.copy_ad(561, 14);s.copy_ad(93, 16);s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));s.copy_ad(90, 492);s.copy_ad(16, 15);s.copy_ad(15, 93);}
        if ((!s.b[1087]) && (s.v[38] != 0.0)) {s.copy_ad(279, 552);s.copy_ad(552, 553);s.copy_ad(553, 279);}
        s.b[1088] = ((p.p28 != 0.0) && (p.p237 > 0.0));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if s.b[1088] {s.store_mul(547, 0, 51);s.store_scalar(516, s.v[468]);s.store_scalar(557, (1.0 / s.v[467]));}
        if (!s.b[1088]) {s.store_scalar(547, 0.0);s.store_scalar(516, 0.0);s.store_scalar(557, 0.0);}
        s.copy_ad(0, 94);s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));s.store_scale(18, 18, p.p33);s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));s.store_scale(19, 19, p.p33);s.b[1094] = ((p.p28 != 0.0) && (p.p237 > 0.0));s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });s.b[1095] = (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0));s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq0_e348: f64 = (p.p33 * s.v[94]);let eq0_value: f64 = eq0_e348;
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            &s.dn[94],
            &s.db[94],
            (multiplicity) * (p.p33),
        );let eq1_e352: f64 = (s.v[257] + s.v[185]);let eq1_e352_d_n0: f64 = (s.dn[257][0] + s.dn[185][0]);let eq1_e352_d_n1: f64 = (s.dn[257][1] + s.dn[185][1]);let eq1_e352_d_n2: f64 = (s.dn[257][2] + s.dn[185][2]);let eq1_e352_d_n3: f64 = (s.dn[257][3] + s.dn[185][3]);let eq1_e352_d_n4: f64 = (s.dn[257][4] + s.dn[185][4]);let eq1_e352_d_n5: f64 = (s.dn[257][5] + s.dn[185][5]);let eq1_e352_d_n6: f64 = (s.dn[257][6] + s.dn[185][6]);let eq1_e352_d_n7: f64 = (s.dn[257][7] + s.dn[185][7]);let eq1_e352_d_n8: f64 = (s.dn[257][8] + s.dn[185][8]);let eq1_e352_d_n9: f64 = (s.dn[257][9] + s.dn[185][9]);let eq1_e352_d_n10: f64 = (s.dn[257][10] + s.dn[185][10]);let eq1_e352_d_n11: f64 = (s.dn[257][11] + s.dn[185][11]);let eq1_e352_d_n12: f64 = (s.dn[257][12] + s.dn[185][12]);let eq1_e352_d_b0: f64 = (s.db[257][0] + s.db[185][0]);let eq1_e352_d_b1: f64 = (s.db[257][1] + s.db[185][1]);let eq1_e352_d_b2: f64 = (s.db[257][2] + s.db[185][2]);let eq1_e352_d_b3: f64 = (s.db[257][3] + s.db[185][3]);let eq1_e352_d_b4: f64 = (s.db[257][4] + s.db[185][4]);let eq1_e352_d_b5: f64 = (s.db[257][5] + s.db[185][5]);let eq1_e352_d_b6: f64 = (s.db[257][6] + s.db[185][6]);let eq1_e352_d_b7: f64 = (s.db[257][7] + s.db[185][7]);let eq1_e353: f64 = (p.p33 * eq1_e352);let eq1_e353_d_n0: f64 = (p.p33 * eq1_e352_d_n0);let eq1_e353_d_n1: f64 = (p.p33 * eq1_e352_d_n1);let eq1_e353_d_n2: f64 = (p.p33 * eq1_e352_d_n2);let eq1_e353_d_n3: f64 = (p.p33 * eq1_e352_d_n3);let eq1_e353_d_n4: f64 = (p.p33 * eq1_e352_d_n4);let eq1_e353_d_n5: f64 = (p.p33 * eq1_e352_d_n5);let eq1_e353_d_n6: f64 = (p.p33 * eq1_e352_d_n6);let eq1_e353_d_n7: f64 = (p.p33 * eq1_e352_d_n7);let eq1_e353_d_n8: f64 = (p.p33 * eq1_e352_d_n8);let eq1_e353_d_n9: f64 = (p.p33 * eq1_e352_d_n9);let eq1_e353_d_n10: f64 = (p.p33 * eq1_e352_d_n10);let eq1_e353_d_n11: f64 = (p.p33 * eq1_e352_d_n11);let eq1_e353_d_n12: f64 = (p.p33 * eq1_e352_d_n12);let eq1_e353_d_b0: f64 = (p.p33 * eq1_e352_d_b0);let eq1_e353_d_b1: f64 = (p.p33 * eq1_e352_d_b1);let eq1_e353_d_b2: f64 = (p.p33 * eq1_e352_d_b2);let eq1_e353_d_b3: f64 = (p.p33 * eq1_e352_d_b3);let eq1_e353_d_b4: f64 = (p.p33 * eq1_e352_d_b4);let eq1_e353_d_b5: f64 = (p.p33 * eq1_e352_d_b5);let eq1_e353_d_b6: f64 = (p.p33 * eq1_e352_d_b6);let eq1_e353_d_b7: f64 = (p.p33 * eq1_e352_d_b7);let eq1_value: f64 = eq1_e353;let eq1_node_derivatives: [f64; 13] = [eq1_e353_d_n0, eq1_e353_d_n1, eq1_e353_d_n2, eq1_e353_d_n3, eq1_e353_d_n4, eq1_e353_d_n5, eq1_e353_d_n6, eq1_e353_d_n7, eq1_e353_d_n8, eq1_e353_d_n9, eq1_e353_d_n10, eq1_e353_d_n11, eq1_e353_d_n12];let eq1_branch_derivatives: [f64; 8] = [eq1_e353_d_b0, eq1_e353_d_b1, eq1_e353_d_b2, eq1_e353_d_b3, eq1_e353_d_b4, eq1_e353_d_b5, eq1_e353_d_b6, eq1_e353_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv12 = ctx.node_voltage(nodes[12]);let eq2_e357: f64 = (s.v[258] + s.v[546]);let eq2_e357_d_n0: f64 = (s.dn[258][0] + s.dn[546][0]);let eq2_e357_d_n1: f64 = (s.dn[258][1] + s.dn[546][1]);let eq2_e357_d_n2: f64 = (s.dn[258][2] + s.dn[546][2]);let eq2_e357_d_n3: f64 = (s.dn[258][3] + s.dn[546][3]);let eq2_e357_d_n4: f64 = (s.dn[258][4] + s.dn[546][4]);let eq2_e357_d_n5: f64 = (s.dn[258][5] + s.dn[546][5]);let eq2_e357_d_n6: f64 = (s.dn[258][6] + s.dn[546][6]);let eq2_e357_d_n7: f64 = (s.dn[258][7] + s.dn[546][7]);let eq2_e357_d_n8: f64 = (s.dn[258][8] + s.dn[546][8]);let eq2_e357_d_n9: f64 = (s.dn[258][9] + s.dn[546][9]);let eq2_e357_d_n10: f64 = (s.dn[258][10] + s.dn[546][10]);let eq2_e357_d_n11: f64 = (s.dn[258][11] + s.dn[546][11]);let eq2_e357_d_n12: f64 = (s.dn[258][12] + s.dn[546][12]);let eq2_e357_d_b0: f64 = (s.db[258][0] + s.db[546][0]);let eq2_e357_d_b1: f64 = (s.db[258][1] + s.db[546][1]);let eq2_e357_d_b2: f64 = (s.db[258][2] + s.db[546][2]);let eq2_e357_d_b3: f64 = (s.db[258][3] + s.db[546][3]);let eq2_e357_d_b4: f64 = (s.db[258][4] + s.db[546][4]);let eq2_e357_d_b5: f64 = (s.db[258][5] + s.db[546][5]);let eq2_e357_d_b6: f64 = (s.db[258][6] + s.db[546][6]);let eq2_e357_d_b7: f64 = (s.db[258][7] + s.db[546][7]);let eq2_e358: f64 = (p.p33 * eq2_e357);let eq2_e358_d_n0: f64 = (p.p33 * eq2_e357_d_n0);let eq2_e358_d_n1: f64 = (p.p33 * eq2_e357_d_n1);let eq2_e358_d_n2: f64 = (p.p33 * eq2_e357_d_n2);let eq2_e358_d_n3: f64 = (p.p33 * eq2_e357_d_n3);let eq2_e358_d_n4: f64 = (p.p33 * eq2_e357_d_n4);let eq2_e358_d_n5: f64 = (p.p33 * eq2_e357_d_n5);let eq2_e358_d_n6: f64 = (p.p33 * eq2_e357_d_n6);let eq2_e358_d_n7: f64 = (p.p33 * eq2_e357_d_n7);let eq2_e358_d_n8: f64 = (p.p33 * eq2_e357_d_n8);let eq2_e358_d_n9: f64 = (p.p33 * eq2_e357_d_n9);let eq2_e358_d_n10: f64 = (p.p33 * eq2_e357_d_n10);let eq2_e358_d_n11: f64 = (p.p33 * eq2_e357_d_n11);let eq2_e358_d_n12: f64 = (p.p33 * eq2_e357_d_n12);let eq2_e358_d_b0: f64 = (p.p33 * eq2_e357_d_b0);let eq2_e358_d_b1: f64 = (p.p33 * eq2_e357_d_b1);let eq2_e358_d_b2: f64 = (p.p33 * eq2_e357_d_b2);let eq2_e358_d_b3: f64 = (p.p33 * eq2_e357_d_b3);let eq2_e358_d_b4: f64 = (p.p33 * eq2_e357_d_b4);let eq2_e358_d_b5: f64 = (p.p33 * eq2_e357_d_b5);let eq2_e358_d_b6: f64 = (p.p33 * eq2_e357_d_b6);let eq2_e358_d_b7: f64 = (p.p33 * eq2_e357_d_b7);let eq2_value: f64 = eq2_e358;let eq2_node_derivatives: [f64; 13] = [eq2_e358_d_n0, eq2_e358_d_n1, eq2_e358_d_n2, eq2_e358_d_n3, eq2_e358_d_n4, eq2_e358_d_n5, eq2_e358_d_n6, eq2_e358_d_n7, eq2_e358_d_n8, eq2_e358_d_n9, eq2_e358_d_n10, eq2_e358_d_n11, eq2_e358_d_n12];let eq2_branch_derivatives: [f64; 8] = [eq2_e358_d_b0, eq2_e358_d_b1, eq2_e358_d_b2, eq2_e358_d_b3, eq2_e358_d_b4, eq2_e358_d_b5, eq2_e358_d_b6, eq2_e358_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );let eq3_e361: f64 = (p.p33 * s.v[250]);let eq3_value: f64 = eq3_e361;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            &s.dn[250],
            &s.db[250],
            (multiplicity) * (p.p33),
        );let eq4_e364: f64 = (p.p33 * s.v[251]);let eq4_value: f64 = eq4_e364;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            &s.dn[251],
            &s.db[251],
            (multiplicity) * (p.p33),
        );let eq5_e367: f64 = (p.p33 * s.v[254]);let eq5_value: f64 = eq5_e367;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            &s.dn[254],
            &s.db[254],
            (multiplicity) * (p.p33),
        );
        let (eq6_e373, eq6_e373_d_n0, eq6_e373_d_n1, eq6_e373_d_n2, eq6_e373_d_n3, eq6_e373_d_n4, eq6_e373_d_n5, eq6_e373_d_n6, eq6_e373_d_n7, eq6_e373_d_n8, eq6_e373_d_n9, eq6_e373_d_n10, eq6_e373_d_n11, eq6_e373_d_n12, eq6_e373_d_b0, eq6_e373_d_b1, eq6_e373_d_b2, eq6_e373_d_b3, eq6_e373_d_b4, eq6_e373_d_b5, eq6_e373_d_b6, eq6_e373_d_b7,) = {
    if (p.p312 != 0.0) {
        let eq6_e371: f64 = ((nv12 - nv2) / s.v[27]);let eq6_e371_d_n0: f64 = (-(((nv12 - nv2) * s.dn[27][0]) / (s.v[27] * s.v[27])));let eq6_e371_d_n1: f64 = (-(((nv12 - nv2) * s.dn[27][1]) / (s.v[27] * s.v[27])));let eq6_e371_d_n2: f64 = (((-s.v[27]) - ((nv12 - nv2) * s.dn[27][2])) / (s.v[27] * s.v[27]));let eq6_e371_d_n3: f64 = (-(((nv12 - nv2) * s.dn[27][3]) / (s.v[27] * s.v[27])));let eq6_e371_d_n4: f64 = (-(((nv12 - nv2) * s.dn[27][4]) / (s.v[27] * s.v[27])));let eq6_e371_d_n5: f64 = (-(((nv12 - nv2) * s.dn[27][5]) / (s.v[27] * s.v[27])));let eq6_e371_d_n6: f64 = (-(((nv12 - nv2) * s.dn[27][6]) / (s.v[27] * s.v[27])));let eq6_e371_d_n7: f64 = (-(((nv12 - nv2) * s.dn[27][7]) / (s.v[27] * s.v[27])));let eq6_e371_d_n8: f64 = (-(((nv12 - nv2) * s.dn[27][8]) / (s.v[27] * s.v[27])));let eq6_e371_d_n9: f64 = (-(((nv12 - nv2) * s.dn[27][9]) / (s.v[27] * s.v[27])));let eq6_e371_d_n10: f64 = (-(((nv12 - nv2) * s.dn[27][10]) / (s.v[27] * s.v[27])));let eq6_e371_d_n11: f64 = (-(((nv12 - nv2) * s.dn[27][11]) / (s.v[27] * s.v[27])));let eq6_e371_d_n12: f64 = ((s.v[27] - ((nv12 - nv2) * s.dn[27][12])) / (s.v[27] * s.v[27]));let eq6_e371_d_b0: f64 = (-(((nv12 - nv2) * s.db[27][0]) / (s.v[27] * s.v[27])));let eq6_e371_d_b1: f64 = (-(((nv12 - nv2) * s.db[27][1]) / (s.v[27] * s.v[27])));let eq6_e371_d_b2: f64 = (-(((nv12 - nv2) * s.db[27][2]) / (s.v[27] * s.v[27])));let eq6_e371_d_b3: f64 = (-(((nv12 - nv2) * s.db[27][3]) / (s.v[27] * s.v[27])));let eq6_e371_d_b4: f64 = (-(((nv12 - nv2) * s.db[27][4]) / (s.v[27] * s.v[27])));let eq6_e371_d_b5: f64 = (-(((nv12 - nv2) * s.db[27][5]) / (s.v[27] * s.v[27])));let eq6_e371_d_b6: f64 = (-(((nv12 - nv2) * s.db[27][6]) / (s.v[27] * s.v[27])));let eq6_e371_d_b7: f64 = (-(((nv12 - nv2) * s.db[27][7]) / (s.v[27] * s.v[27])));
        (eq6_e371, eq6_e371_d_n0, eq6_e371_d_n1, eq6_e371_d_n2, eq6_e371_d_n3, eq6_e371_d_n4, eq6_e371_d_n5, eq6_e371_d_n6, eq6_e371_d_n7, eq6_e371_d_n8, eq6_e371_d_n9, eq6_e371_d_n10, eq6_e371_d_n11, eq6_e371_d_n12, eq6_e371_d_b0, eq6_e371_d_b1, eq6_e371_d_b2, eq6_e371_d_b3, eq6_e371_d_b4, eq6_e371_d_b5, eq6_e371_d_b6, eq6_e371_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e373;let eq6_node_derivatives: [f64; 13] = [eq6_e373_d_n0, eq6_e373_d_n1, eq6_e373_d_n2, eq6_e373_d_n3, eq6_e373_d_n4, eq6_e373_d_n5, eq6_e373_d_n6, eq6_e373_d_n7, eq6_e373_d_n8, eq6_e373_d_n9, eq6_e373_d_n10, eq6_e373_d_n11, eq6_e373_d_n12];let eq6_branch_derivatives: [f64; 8] = [eq6_e373_d_b0, eq6_e373_d_b1, eq6_e373_d_b2, eq6_e373_d_b3, eq6_e373_d_b4, eq6_e373_d_b5, eq6_e373_d_b6, eq6_e373_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e378,) = {
    if (p.p312 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e378;
        stamper.stamp_potential_const_local(
            0,
            eq7_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq8_e384, eq8_e384_d_n0, eq8_e384_d_n1, eq8_e384_d_n2, eq8_e384_d_n3, eq8_e384_d_n4, eq8_e384_d_n5, eq8_e384_d_n6, eq8_e384_d_n7, eq8_e384_d_n8, eq8_e384_d_n9, eq8_e384_d_n10, eq8_e384_d_n11, eq8_e384_d_n12, eq8_e384_d_b0, eq8_e384_d_b1, eq8_e384_d_b2, eq8_e384_d_b3, eq8_e384_d_b4, eq8_e384_d_b5, eq8_e384_d_b6, eq8_e384_d_b7,) = {
    if (p.p313 != 0.0) {
        let eq8_e382: f64 = ((nv0 - nv11) / s.v[26]);let eq8_e382_d_n0: f64 = ((s.v[26] - ((nv0 - nv11) * s.dn[26][0])) / (s.v[26] * s.v[26]));let eq8_e382_d_n1: f64 = (-(((nv0 - nv11) * s.dn[26][1]) / (s.v[26] * s.v[26])));let eq8_e382_d_n2: f64 = (-(((nv0 - nv11) * s.dn[26][2]) / (s.v[26] * s.v[26])));let eq8_e382_d_n3: f64 = (-(((nv0 - nv11) * s.dn[26][3]) / (s.v[26] * s.v[26])));let eq8_e382_d_n4: f64 = (-(((nv0 - nv11) * s.dn[26][4]) / (s.v[26] * s.v[26])));let eq8_e382_d_n5: f64 = (-(((nv0 - nv11) * s.dn[26][5]) / (s.v[26] * s.v[26])));let eq8_e382_d_n6: f64 = (-(((nv0 - nv11) * s.dn[26][6]) / (s.v[26] * s.v[26])));let eq8_e382_d_n7: f64 = (-(((nv0 - nv11) * s.dn[26][7]) / (s.v[26] * s.v[26])));let eq8_e382_d_n8: f64 = (-(((nv0 - nv11) * s.dn[26][8]) / (s.v[26] * s.v[26])));let eq8_e382_d_n9: f64 = (-(((nv0 - nv11) * s.dn[26][9]) / (s.v[26] * s.v[26])));let eq8_e382_d_n10: f64 = (-(((nv0 - nv11) * s.dn[26][10]) / (s.v[26] * s.v[26])));let eq8_e382_d_n11: f64 = (((-s.v[26]) - ((nv0 - nv11) * s.dn[26][11])) / (s.v[26] * s.v[26]));let eq8_e382_d_n12: f64 = (-(((nv0 - nv11) * s.dn[26][12]) / (s.v[26] * s.v[26])));let eq8_e382_d_b0: f64 = (-(((nv0 - nv11) * s.db[26][0]) / (s.v[26] * s.v[26])));let eq8_e382_d_b1: f64 = (-(((nv0 - nv11) * s.db[26][1]) / (s.v[26] * s.v[26])));let eq8_e382_d_b2: f64 = (-(((nv0 - nv11) * s.db[26][2]) / (s.v[26] * s.v[26])));let eq8_e382_d_b3: f64 = (-(((nv0 - nv11) * s.db[26][3]) / (s.v[26] * s.v[26])));let eq8_e382_d_b4: f64 = (-(((nv0 - nv11) * s.db[26][4]) / (s.v[26] * s.v[26])));let eq8_e382_d_b5: f64 = (-(((nv0 - nv11) * s.db[26][5]) / (s.v[26] * s.v[26])));let eq8_e382_d_b6: f64 = (-(((nv0 - nv11) * s.db[26][6]) / (s.v[26] * s.v[26])));let eq8_e382_d_b7: f64 = (-(((nv0 - nv11) * s.db[26][7]) / (s.v[26] * s.v[26])));
        (eq8_e382, eq8_e382_d_n0, eq8_e382_d_n1, eq8_e382_d_n2, eq8_e382_d_n3, eq8_e382_d_n4, eq8_e382_d_n5, eq8_e382_d_n6, eq8_e382_d_n7, eq8_e382_d_n8, eq8_e382_d_n9, eq8_e382_d_n10, eq8_e382_d_n11, eq8_e382_d_n12, eq8_e382_d_b0, eq8_e382_d_b1, eq8_e382_d_b2, eq8_e382_d_b3, eq8_e382_d_b4, eq8_e382_d_b5, eq8_e382_d_b6, eq8_e382_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e384;let eq8_node_derivatives: [f64; 13] = [eq8_e384_d_n0, eq8_e384_d_n1, eq8_e384_d_n2, eq8_e384_d_n3, eq8_e384_d_n4, eq8_e384_d_n5, eq8_e384_d_n6, eq8_e384_d_n7, eq8_e384_d_n8, eq8_e384_d_n9, eq8_e384_d_n10, eq8_e384_d_n11, eq8_e384_d_n12];let eq8_branch_derivatives: [f64; 8] = [eq8_e384_d_b0, eq8_e384_d_b1, eq8_e384_d_b2, eq8_e384_d_b3, eq8_e384_d_b4, eq8_e384_d_b5, eq8_e384_d_b6, eq8_e384_d_b7];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e389,) = {
    if (p.p313 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e389;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );let eq10_e393: f64 = (s.v[561] + s.v[554]);let eq10_e393_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);let eq10_e393_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);let eq10_e393_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);let eq10_e393_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);let eq10_e393_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);let eq10_e393_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);let eq10_e393_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);let eq10_e393_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);let eq10_e393_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);let eq10_e393_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);let eq10_e393_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);let eq10_e393_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);let eq10_e393_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);let eq10_e393_d_b0: f64 = (s.db[561][0] + s.db[554][0]);let eq10_e393_d_b1: f64 = (s.db[561][1] + s.db[554][1]);let eq10_e393_d_b2: f64 = (s.db[561][2] + s.db[554][2]);let eq10_e393_d_b3: f64 = (s.db[561][3] + s.db[554][3]);let eq10_e393_d_b4: f64 = (s.db[561][4] + s.db[554][4]);let eq10_e393_d_b5: f64 = (s.db[561][5] + s.db[554][5]);let eq10_e393_d_b6: f64 = (s.db[561][6] + s.db[554][6]);let eq10_e393_d_b7: f64 = (s.db[561][7] + s.db[554][7]);let eq10_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e393);let eq10_e395: f64 = (p.p33 * eq10_e394);let eq10_e395_d_n0: f64 = (p.p33 * (eq10_e393_d_n0 * ddt_scale));let eq10_e395_d_n1: f64 = (p.p33 * (eq10_e393_d_n1 * ddt_scale));let eq10_e395_d_n2: f64 = (p.p33 * (eq10_e393_d_n2 * ddt_scale));let eq10_e395_d_n3: f64 = (p.p33 * (eq10_e393_d_n3 * ddt_scale));let eq10_e395_d_n4: f64 = (p.p33 * (eq10_e393_d_n4 * ddt_scale));let eq10_e395_d_n5: f64 = (p.p33 * (eq10_e393_d_n5 * ddt_scale));let eq10_e395_d_n6: f64 = (p.p33 * (eq10_e393_d_n6 * ddt_scale));let eq10_e395_d_n7: f64 = (p.p33 * (eq10_e393_d_n7 * ddt_scale));let eq10_e395_d_n8: f64 = (p.p33 * (eq10_e393_d_n8 * ddt_scale));let eq10_e395_d_n9: f64 = (p.p33 * (eq10_e393_d_n9 * ddt_scale));let eq10_e395_d_n10: f64 = (p.p33 * (eq10_e393_d_n10 * ddt_scale));let eq10_e395_d_n11: f64 = (p.p33 * (eq10_e393_d_n11 * ddt_scale));let eq10_e395_d_n12: f64 = (p.p33 * (eq10_e393_d_n12 * ddt_scale));let eq10_e395_d_b0: f64 = (p.p33 * (eq10_e393_d_b0 * ddt_scale));let eq10_e395_d_b1: f64 = (p.p33 * (eq10_e393_d_b1 * ddt_scale));let eq10_e395_d_b2: f64 = (p.p33 * (eq10_e393_d_b2 * ddt_scale));let eq10_e395_d_b3: f64 = (p.p33 * (eq10_e393_d_b3 * ddt_scale));let eq10_e395_d_b4: f64 = (p.p33 * (eq10_e393_d_b4 * ddt_scale));let eq10_e395_d_b5: f64 = (p.p33 * (eq10_e393_d_b5 * ddt_scale));let eq10_e395_d_b6: f64 = (p.p33 * (eq10_e393_d_b6 * ddt_scale));let eq10_e395_d_b7: f64 = (p.p33 * (eq10_e393_d_b7 * ddt_scale));let eq10_value: f64 = eq10_e395;let eq10_node_derivatives: [f64; 13] = [eq10_e395_d_n0, eq10_e395_d_n1, eq10_e395_d_n2, eq10_e395_d_n3, eq10_e395_d_n4, eq10_e395_d_n5, eq10_e395_d_n6, eq10_e395_d_n7, eq10_e395_d_n8, eq10_e395_d_n9, eq10_e395_d_n10, eq10_e395_d_n11, eq10_e395_d_n12];let eq10_branch_derivatives: [f64; 8] = [eq10_e395_d_b0, eq10_e395_d_b1, eq10_e395_d_b2, eq10_e395_d_b3, eq10_e395_d_b4, eq10_e395_d_b5, eq10_e395_d_b6, eq10_e395_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq11_e399: f64 = (s.v[93] + s.v[552]);let eq11_e399_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);let eq11_e399_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);let eq11_e399_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);let eq11_e399_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);let eq11_e399_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);let eq11_e399_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);let eq11_e399_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);let eq11_e399_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);let eq11_e399_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);let eq11_e399_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);let eq11_e399_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);let eq11_e399_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);let eq11_e399_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);let eq11_e399_d_b0: f64 = (s.db[93][0] + s.db[552][0]);let eq11_e399_d_b1: f64 = (s.db[93][1] + s.db[552][1]);let eq11_e399_d_b2: f64 = (s.db[93][2] + s.db[552][2]);let eq11_e399_d_b3: f64 = (s.db[93][3] + s.db[552][3]);let eq11_e399_d_b4: f64 = (s.db[93][4] + s.db[552][4]);let eq11_e399_d_b5: f64 = (s.db[93][5] + s.db[552][5]);let eq11_e399_d_b6: f64 = (s.db[93][6] + s.db[552][6]);let eq11_e399_d_b7: f64 = (s.db[93][7] + s.db[552][7]);let eq11_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e399);let eq11_e401: f64 = (p.p33 * eq11_e400);let eq11_e401_d_n0: f64 = (p.p33 * (eq11_e399_d_n0 * ddt_scale));let eq11_e401_d_n1: f64 = (p.p33 * (eq11_e399_d_n1 * ddt_scale));let eq11_e401_d_n2: f64 = (p.p33 * (eq11_e399_d_n2 * ddt_scale));let eq11_e401_d_n3: f64 = (p.p33 * (eq11_e399_d_n3 * ddt_scale));let eq11_e401_d_n4: f64 = (p.p33 * (eq11_e399_d_n4 * ddt_scale));let eq11_e401_d_n5: f64 = (p.p33 * (eq11_e399_d_n5 * ddt_scale));let eq11_e401_d_n6: f64 = (p.p33 * (eq11_e399_d_n6 * ddt_scale));let eq11_e401_d_n7: f64 = (p.p33 * (eq11_e399_d_n7 * ddt_scale));let eq11_e401_d_n8: f64 = (p.p33 * (eq11_e399_d_n8 * ddt_scale));let eq11_e401_d_n9: f64 = (p.p33 * (eq11_e399_d_n9 * ddt_scale));let eq11_e401_d_n10: f64 = (p.p33 * (eq11_e399_d_n10 * ddt_scale));let eq11_e401_d_n11: f64 = (p.p33 * (eq11_e399_d_n11 * ddt_scale));let eq11_e401_d_n12: f64 = (p.p33 * (eq11_e399_d_n12 * ddt_scale));let eq11_e401_d_b0: f64 = (p.p33 * (eq11_e399_d_b0 * ddt_scale));let eq11_e401_d_b1: f64 = (p.p33 * (eq11_e399_d_b1 * ddt_scale));let eq11_e401_d_b2: f64 = (p.p33 * (eq11_e399_d_b2 * ddt_scale));let eq11_e401_d_b3: f64 = (p.p33 * (eq11_e399_d_b3 * ddt_scale));let eq11_e401_d_b4: f64 = (p.p33 * (eq11_e399_d_b4 * ddt_scale));let eq11_e401_d_b5: f64 = (p.p33 * (eq11_e399_d_b5 * ddt_scale));let eq11_e401_d_b6: f64 = (p.p33 * (eq11_e399_d_b6 * ddt_scale));let eq11_e401_d_b7: f64 = (p.p33 * (eq11_e399_d_b7 * ddt_scale));let eq11_value: f64 = eq11_e401;let eq11_node_derivatives: [f64; 13] = [eq11_e401_d_n0, eq11_e401_d_n1, eq11_e401_d_n2, eq11_e401_d_n3, eq11_e401_d_n4, eq11_e401_d_n5, eq11_e401_d_n6, eq11_e401_d_n7, eq11_e401_d_n8, eq11_e401_d_n9, eq11_e401_d_n10, eq11_e401_d_n11, eq11_e401_d_n12];let eq11_branch_derivatives: [f64; 8] = [eq11_e401_d_b0, eq11_e401_d_b1, eq11_e401_d_b2, eq11_e401_d_b3, eq11_e401_d_b4, eq11_e401_d_b5, eq11_e401_d_b6, eq11_e401_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let eq12_e405: f64 = (s.v[90] + s.v[548]);let eq12_e405_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);let eq12_e405_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);let eq12_e405_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);let eq12_e405_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);let eq12_e405_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);let eq12_e405_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);let eq12_e405_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);let eq12_e405_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);let eq12_e405_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);let eq12_e405_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);let eq12_e405_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);let eq12_e405_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);let eq12_e405_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);let eq12_e405_d_b0: f64 = (s.db[90][0] + s.db[548][0]);let eq12_e405_d_b1: f64 = (s.db[90][1] + s.db[548][1]);let eq12_e405_d_b2: f64 = (s.db[90][2] + s.db[548][2]);let eq12_e405_d_b3: f64 = (s.db[90][3] + s.db[548][3]);let eq12_e405_d_b4: f64 = (s.db[90][4] + s.db[548][4]);let eq12_e405_d_b5: f64 = (s.db[90][5] + s.db[548][5]);let eq12_e405_d_b6: f64 = (s.db[90][6] + s.db[548][6]);let eq12_e405_d_b7: f64 = (s.db[90][7] + s.db[548][7]);let eq12_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e405);let eq12_e407: f64 = (p.p33 * eq12_e406);let eq12_e407_d_n0: f64 = (p.p33 * (eq12_e405_d_n0 * ddt_scale));let eq12_e407_d_n1: f64 = (p.p33 * (eq12_e405_d_n1 * ddt_scale));let eq12_e407_d_n2: f64 = (p.p33 * (eq12_e405_d_n2 * ddt_scale));let eq12_e407_d_n3: f64 = (p.p33 * (eq12_e405_d_n3 * ddt_scale));let eq12_e407_d_n4: f64 = (p.p33 * (eq12_e405_d_n4 * ddt_scale));let eq12_e407_d_n5: f64 = (p.p33 * (eq12_e405_d_n5 * ddt_scale));let eq12_e407_d_n6: f64 = (p.p33 * (eq12_e405_d_n6 * ddt_scale));let eq12_e407_d_n7: f64 = (p.p33 * (eq12_e405_d_n7 * ddt_scale));let eq12_e407_d_n8: f64 = (p.p33 * (eq12_e405_d_n8 * ddt_scale));let eq12_e407_d_n9: f64 = (p.p33 * (eq12_e405_d_n9 * ddt_scale));let eq12_e407_d_n10: f64 = (p.p33 * (eq12_e405_d_n10 * ddt_scale));let eq12_e407_d_n11: f64 = (p.p33 * (eq12_e405_d_n11 * ddt_scale));let eq12_e407_d_n12: f64 = (p.p33 * (eq12_e405_d_n12 * ddt_scale));let eq12_e407_d_b0: f64 = (p.p33 * (eq12_e405_d_b0 * ddt_scale));let eq12_e407_d_b1: f64 = (p.p33 * (eq12_e405_d_b1 * ddt_scale));let eq12_e407_d_b2: f64 = (p.p33 * (eq12_e405_d_b2 * ddt_scale));let eq12_e407_d_b3: f64 = (p.p33 * (eq12_e405_d_b3 * ddt_scale));let eq12_e407_d_b4: f64 = (p.p33 * (eq12_e405_d_b4 * ddt_scale));let eq12_e407_d_b5: f64 = (p.p33 * (eq12_e405_d_b5 * ddt_scale));let eq12_e407_d_b6: f64 = (p.p33 * (eq12_e405_d_b6 * ddt_scale));let eq12_e407_d_b7: f64 = (p.p33 * (eq12_e405_d_b7 * ddt_scale));let eq12_value: f64 = eq12_e407;let eq12_node_derivatives: [f64; 13] = [eq12_e407_d_n0, eq12_e407_d_n1, eq12_e407_d_n2, eq12_e407_d_n3, eq12_e407_d_n4, eq12_e407_d_n5, eq12_e407_d_n6, eq12_e407_d_n7, eq12_e407_d_n8, eq12_e407_d_n9, eq12_e407_d_n10, eq12_e407_d_n11, eq12_e407_d_n12];let eq12_branch_derivatives: [f64; 8] = [eq12_e407_d_b0, eq12_e407_d_b1, eq12_e407_d_b2, eq12_e407_d_b3, eq12_e407_d_b4, eq12_e407_d_b5, eq12_e407_d_b6, eq12_e407_d_b7];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );let eq14_e418: f64 = (nv7 - 0.0);let eq14_value: f64 = eq14_e418;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (1.0),
        );let eq17_e433: f64 = (s.v[609] * (nv7 - 0.0));let eq17_e433_d_n0: f64 = (s.dn[609][0] * (nv7 - 0.0));let eq17_e433_d_n1: f64 = (s.dn[609][1] * (nv7 - 0.0));let eq17_e433_d_n2: f64 = (s.dn[609][2] * (nv7 - 0.0));let eq17_e433_d_n3: f64 = (s.dn[609][3] * (nv7 - 0.0));let eq17_e433_d_n4: f64 = (s.dn[609][4] * (nv7 - 0.0));let eq17_e433_d_n5: f64 = (s.dn[609][5] * (nv7 - 0.0));let eq17_e433_d_n6: f64 = (s.dn[609][6] * (nv7 - 0.0));let eq17_e433_d_n7: f64 = ((s.dn[609][7] * (nv7 - 0.0)) + s.v[609]);let eq17_e433_d_n8: f64 = (s.dn[609][8] * (nv7 - 0.0));let eq17_e433_d_n9: f64 = (s.dn[609][9] * (nv7 - 0.0));let eq17_e433_d_n10: f64 = (s.dn[609][10] * (nv7 - 0.0));let eq17_e433_d_n11: f64 = (s.dn[609][11] * (nv7 - 0.0));let eq17_e433_d_n12: f64 = (s.dn[609][12] * (nv7 - 0.0));let eq17_e433_d_b0: f64 = (s.db[609][0] * (nv7 - 0.0));let eq17_e433_d_b1: f64 = (s.db[609][1] * (nv7 - 0.0));let eq17_e433_d_b2: f64 = (s.db[609][2] * (nv7 - 0.0));let eq17_e433_d_b3: f64 = (s.db[609][3] * (nv7 - 0.0));let eq17_e433_d_b4: f64 = (s.db[609][4] * (nv7 - 0.0));let eq17_e433_d_b5: f64 = (s.db[609][5] * (nv7 - 0.0));let eq17_e433_d_b6: f64 = (s.db[609][6] * (nv7 - 0.0));let eq17_e433_d_b7: f64 = (s.db[609][7] * (nv7 - 0.0));let eq17_value: f64 = eq17_e433;let eq17_node_derivatives: [f64; 13] = [eq17_e433_d_n0, eq17_e433_d_n1, eq17_e433_d_n2, eq17_e433_d_n3, eq17_e433_d_n4, eq17_e433_d_n5, eq17_e433_d_n6, eq17_e433_d_n7, eq17_e433_d_n8, eq17_e433_d_n9, eq17_e433_d_n10, eq17_e433_d_n11, eq17_e433_d_n12];let eq17_branch_derivatives: [f64; 8] = [eq17_e433_d_b0, eq17_e433_d_b1, eq17_e433_d_b2, eq17_e433_d_b3, eq17_e433_d_b4, eq17_e433_d_b5, eq17_e433_d_b6, eq17_e433_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let eq18_e436: f64 = ((nv7 - 0.0) * s.v[611]);let eq18_e436_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);let eq18_e436_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);let eq18_e436_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);let eq18_e436_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);let eq18_e436_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);let eq18_e436_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);let eq18_e436_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);let eq18_e436_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));let eq18_e436_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);let eq18_e436_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);let eq18_e436_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);let eq18_e436_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);let eq18_e436_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);let eq18_e436_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);let eq18_e436_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);let eq18_e436_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);let eq18_e436_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);let eq18_e436_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);let eq18_e436_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);let eq18_e436_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);let eq18_e436_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);let eq18_e437: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e436);let eq18_value: f64 = eq18_e437;let eq18_node_derivatives: [f64; 13] = [(eq18_e436_d_n0 * ddt_scale), (eq18_e436_d_n1 * ddt_scale), (eq18_e436_d_n2 * ddt_scale), (eq18_e436_d_n3 * ddt_scale), (eq18_e436_d_n4 * ddt_scale), (eq18_e436_d_n5 * ddt_scale), (eq18_e436_d_n6 * ddt_scale), (eq18_e436_d_n7 * ddt_scale), (eq18_e436_d_n8 * ddt_scale), (eq18_e436_d_n9 * ddt_scale), (eq18_e436_d_n10 * ddt_scale), (eq18_e436_d_n11 * ddt_scale), (eq18_e436_d_n12 * ddt_scale)];let eq18_branch_derivatives: [f64; 8] = [(eq18_e436_d_b0 * ddt_scale), (eq18_e436_d_b1 * ddt_scale), (eq18_e436_d_b2 * ddt_scale), (eq18_e436_d_b3 * ddt_scale), (eq18_e436_d_b4 * ddt_scale), (eq18_e436_d_b5 * ddt_scale), (eq18_e436_d_b6 * ddt_scale), (eq18_e436_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );let eq19_e440: f64 = ((nv7 - 0.0) * s.v[612]);let eq19_e440_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);let eq19_e440_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);let eq19_e440_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);let eq19_e440_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);let eq19_e440_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);let eq19_e440_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);let eq19_e440_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);let eq19_e440_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));let eq19_e440_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);let eq19_e440_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);let eq19_e440_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);let eq19_e440_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);let eq19_e440_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);let eq19_e440_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);let eq19_e440_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);let eq19_e440_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);let eq19_e440_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);let eq19_e440_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);let eq19_e440_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);let eq19_e440_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);let eq19_e440_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);let eq19_e441: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e440);let eq19_value: f64 = eq19_e441;let eq19_node_derivatives: [f64; 13] = [(eq19_e440_d_n0 * ddt_scale), (eq19_e440_d_n1 * ddt_scale), (eq19_e440_d_n2 * ddt_scale), (eq19_e440_d_n3 * ddt_scale), (eq19_e440_d_n4 * ddt_scale), (eq19_e440_d_n5 * ddt_scale), (eq19_e440_d_n6 * ddt_scale), (eq19_e440_d_n7 * ddt_scale), (eq19_e440_d_n8 * ddt_scale), (eq19_e440_d_n9 * ddt_scale), (eq19_e440_d_n10 * ddt_scale), (eq19_e440_d_n11 * ddt_scale), (eq19_e440_d_n12 * ddt_scale)];let eq19_branch_derivatives: [f64; 8] = [(eq19_e440_d_b0 * ddt_scale), (eq19_e440_d_b1 * ddt_scale), (eq19_e440_d_b2 * ddt_scale), (eq19_e440_d_b3 * ddt_scale), (eq19_e440_d_b4 * ddt_scale), (eq19_e440_d_b5 * ddt_scale), (eq19_e440_d_b6 * ddt_scale), (eq19_e440_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e484, eq25_e484_d_n0, eq25_e484_d_n1, eq25_e484_d_n2, eq25_e484_d_n3, eq25_e484_d_n4, eq25_e484_d_n5, eq25_e484_d_n6, eq25_e484_d_n7, eq25_e484_d_n8, eq25_e484_d_n9, eq25_e484_d_n10, eq25_e484_d_n11, eq25_e484_d_n12, eq25_e484_d_b0, eq25_e484_d_b1, eq25_e484_d_b2, eq25_e484_d_b3, eq25_e484_d_b4, eq25_e484_d_b5, eq25_e484_d_b6, eq25_e484_d_b7,) = {
    if (p.p25 != 0.0) {
        let eq25_e482: f64 = (s.v[484] * (nv1 - nv5));let eq25_e482_d_n0: f64 = (s.dn[484][0] * (nv1 - nv5));let eq25_e482_d_n1: f64 = ((s.dn[484][1] * (nv1 - nv5)) + s.v[484]);let eq25_e482_d_n2: f64 = (s.dn[484][2] * (nv1 - nv5));let eq25_e482_d_n3: f64 = (s.dn[484][3] * (nv1 - nv5));let eq25_e482_d_n4: f64 = (s.dn[484][4] * (nv1 - nv5));let eq25_e482_d_n5: f64 = ((s.dn[484][5] * (nv1 - nv5)) + (-s.v[484]));let eq25_e482_d_n6: f64 = (s.dn[484][6] * (nv1 - nv5));let eq25_e482_d_n7: f64 = (s.dn[484][7] * (nv1 - nv5));let eq25_e482_d_n8: f64 = (s.dn[484][8] * (nv1 - nv5));let eq25_e482_d_n9: f64 = (s.dn[484][9] * (nv1 - nv5));let eq25_e482_d_n10: f64 = (s.dn[484][10] * (nv1 - nv5));let eq25_e482_d_n11: f64 = (s.dn[484][11] * (nv1 - nv5));let eq25_e482_d_n12: f64 = (s.dn[484][12] * (nv1 - nv5));let eq25_e482_d_b0: f64 = (s.db[484][0] * (nv1 - nv5));let eq25_e482_d_b1: f64 = (s.db[484][1] * (nv1 - nv5));let eq25_e482_d_b2: f64 = (s.db[484][2] * (nv1 - nv5));let eq25_e482_d_b3: f64 = (s.db[484][3] * (nv1 - nv5));let eq25_e482_d_b4: f64 = (s.db[484][4] * (nv1 - nv5));let eq25_e482_d_b5: f64 = (s.db[484][5] * (nv1 - nv5));let eq25_e482_d_b6: f64 = (s.db[484][6] * (nv1 - nv5));let eq25_e482_d_b7: f64 = (s.db[484][7] * (nv1 - nv5));
        (eq25_e482, eq25_e482_d_n0, eq25_e482_d_n1, eq25_e482_d_n2, eq25_e482_d_n3, eq25_e482_d_n4, eq25_e482_d_n5, eq25_e482_d_n6, eq25_e482_d_n7, eq25_e482_d_n8, eq25_e482_d_n9, eq25_e482_d_n10, eq25_e482_d_n11, eq25_e482_d_n12, eq25_e482_d_b0, eq25_e482_d_b1, eq25_e482_d_b2, eq25_e482_d_b3, eq25_e482_d_b4, eq25_e482_d_b5, eq25_e482_d_b6, eq25_e482_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e484;let eq25_node_derivatives: [f64; 13] = [eq25_e484_d_n0, eq25_e484_d_n1, eq25_e484_d_n2, eq25_e484_d_n3, eq25_e484_d_n4, eq25_e484_d_n5, eq25_e484_d_n6, eq25_e484_d_n7, eq25_e484_d_n8, eq25_e484_d_n9, eq25_e484_d_n10, eq25_e484_d_n11, eq25_e484_d_n12];let eq25_branch_derivatives: [f64; 8] = [eq25_e484_d_b0, eq25_e484_d_b1, eq25_e484_d_b2, eq25_e484_d_b3, eq25_e484_d_b4, eq25_e484_d_b5, eq25_e484_d_b6, eq25_e484_d_b7];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e489,) = {
    if (p.p25 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e489;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );let eq27_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq28_e504, eq28_e504_d_n0, eq28_e504_d_n1, eq28_e504_d_n2, eq28_e504_d_n3, eq28_e504_d_n4, eq28_e504_d_n5, eq28_e504_d_n6, eq28_e504_d_n7, eq28_e504_d_n8, eq28_e504_d_n9, eq28_e504_d_n10, eq28_e504_d_n11, eq28_e504_d_n12, eq28_e504_d_b0, eq28_e504_d_b1, eq28_e504_d_b2, eq28_e504_d_b3, eq28_e504_d_b4, eq28_e504_d_b5, eq28_e504_d_b6, eq28_e504_d_b7,) = {
    if s.b[1094] {
        let eq28_e493: f64 = (-s.v[547]);let eq28_e496: f64 = (s.v[516] * (nv4 - 0.0));let eq28_e496_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));let eq28_e496_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));let eq28_e496_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));let eq28_e496_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));let eq28_e496_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);let eq28_e496_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));let eq28_e496_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));let eq28_e496_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));let eq28_e496_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));let eq28_e496_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));let eq28_e496_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));let eq28_e496_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));let eq28_e496_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));let eq28_e496_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));let eq28_e496_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));let eq28_e496_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));let eq28_e496_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));let eq28_e496_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));let eq28_e496_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));let eq28_e496_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));let eq28_e496_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));let eq28_e497: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e496);let eq28_e498: f64 = (eq28_e493 + eq28_e497);let eq28_e498_d_n0: f64 = ((-s.dn[547][0]) + (eq28_e496_d_n0 * ddt_scale));let eq28_e498_d_n1: f64 = ((-s.dn[547][1]) + (eq28_e496_d_n1 * ddt_scale));let eq28_e498_d_n2: f64 = ((-s.dn[547][2]) + (eq28_e496_d_n2 * ddt_scale));let eq28_e498_d_n3: f64 = ((-s.dn[547][3]) + (eq28_e496_d_n3 * ddt_scale));let eq28_e498_d_n4: f64 = ((-s.dn[547][4]) + (eq28_e496_d_n4 * ddt_scale));let eq28_e498_d_n5: f64 = ((-s.dn[547][5]) + (eq28_e496_d_n5 * ddt_scale));let eq28_e498_d_n6: f64 = ((-s.dn[547][6]) + (eq28_e496_d_n6 * ddt_scale));let eq28_e498_d_n7: f64 = ((-s.dn[547][7]) + (eq28_e496_d_n7 * ddt_scale));let eq28_e498_d_n8: f64 = ((-s.dn[547][8]) + (eq28_e496_d_n8 * ddt_scale));let eq28_e498_d_n9: f64 = ((-s.dn[547][9]) + (eq28_e496_d_n9 * ddt_scale));let eq28_e498_d_n10: f64 = ((-s.dn[547][10]) + (eq28_e496_d_n10 * ddt_scale));let eq28_e498_d_n11: f64 = ((-s.dn[547][11]) + (eq28_e496_d_n11 * ddt_scale));let eq28_e498_d_n12: f64 = ((-s.dn[547][12]) + (eq28_e496_d_n12 * ddt_scale));let eq28_e498_d_b0: f64 = ((-s.db[547][0]) + (eq28_e496_d_b0 * ddt_scale));let eq28_e498_d_b1: f64 = ((-s.db[547][1]) + (eq28_e496_d_b1 * ddt_scale));let eq28_e498_d_b2: f64 = ((-s.db[547][2]) + (eq28_e496_d_b2 * ddt_scale));let eq28_e498_d_b3: f64 = ((-s.db[547][3]) + (eq28_e496_d_b3 * ddt_scale));let eq28_e498_d_b4: f64 = ((-s.db[547][4]) + (eq28_e496_d_b4 * ddt_scale));let eq28_e498_d_b5: f64 = ((-s.db[547][5]) + (eq28_e496_d_b5 * ddt_scale));let eq28_e498_d_b6: f64 = ((-s.db[547][6]) + (eq28_e496_d_b6 * ddt_scale));let eq28_e498_d_b7: f64 = ((-s.db[547][7]) + (eq28_e496_d_b7 * ddt_scale));let eq28_e501: f64 = ((nv4 - 0.0) * s.v[557]);let eq28_e501_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);let eq28_e501_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);let eq28_e501_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);let eq28_e501_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);let eq28_e501_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));let eq28_e501_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);let eq28_e501_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);let eq28_e501_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);let eq28_e501_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);let eq28_e501_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);let eq28_e501_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);let eq28_e501_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);let eq28_e501_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);let eq28_e501_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);let eq28_e501_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e501_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);let eq28_e501_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);let eq28_e501_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);let eq28_e501_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);let eq28_e501_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);let eq28_e501_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);let eq28_e502: f64 = (eq28_e498 + eq28_e501);let eq28_e502_d_n0: f64 = (eq28_e498_d_n0 + eq28_e501_d_n0);let eq28_e502_d_n1: f64 = (eq28_e498_d_n1 + eq28_e501_d_n1);let eq28_e502_d_n2: f64 = (eq28_e498_d_n2 + eq28_e501_d_n2);let eq28_e502_d_n3: f64 = (eq28_e498_d_n3 + eq28_e501_d_n3);let eq28_e502_d_n4: f64 = (eq28_e498_d_n4 + eq28_e501_d_n4);let eq28_e502_d_n5: f64 = (eq28_e498_d_n5 + eq28_e501_d_n5);let eq28_e502_d_n6: f64 = (eq28_e498_d_n6 + eq28_e501_d_n6);let eq28_e502_d_n7: f64 = (eq28_e498_d_n7 + eq28_e501_d_n7);let eq28_e502_d_n8: f64 = (eq28_e498_d_n8 + eq28_e501_d_n8);let eq28_e502_d_n9: f64 = (eq28_e498_d_n9 + eq28_e501_d_n9);let eq28_e502_d_n10: f64 = (eq28_e498_d_n10 + eq28_e501_d_n10);let eq28_e502_d_n11: f64 = (eq28_e498_d_n11 + eq28_e501_d_n11);let eq28_e502_d_n12: f64 = (eq28_e498_d_n12 + eq28_e501_d_n12);let eq28_e502_d_b0: f64 = (eq28_e498_d_b0 + eq28_e501_d_b0);let eq28_e502_d_b1: f64 = (eq28_e498_d_b1 + eq28_e501_d_b1);let eq28_e502_d_b2: f64 = (eq28_e498_d_b2 + eq28_e501_d_b2);let eq28_e502_d_b3: f64 = (eq28_e498_d_b3 + eq28_e501_d_b3);let eq28_e502_d_b4: f64 = (eq28_e498_d_b4 + eq28_e501_d_b4);let eq28_e502_d_b5: f64 = (eq28_e498_d_b5 + eq28_e501_d_b5);let eq28_e502_d_b6: f64 = (eq28_e498_d_b6 + eq28_e501_d_b6);let eq28_e502_d_b7: f64 = (eq28_e498_d_b7 + eq28_e501_d_b7);
        (eq28_e502, eq28_e502_d_n0, eq28_e502_d_n1, eq28_e502_d_n2, eq28_e502_d_n3, eq28_e502_d_n4, eq28_e502_d_n5, eq28_e502_d_n6, eq28_e502_d_n7, eq28_e502_d_n8, eq28_e502_d_n9, eq28_e502_d_n10, eq28_e502_d_n11, eq28_e502_d_n12, eq28_e502_d_b0, eq28_e502_d_b1, eq28_e502_d_b2, eq28_e502_d_b3, eq28_e502_d_b4, eq28_e502_d_b5, eq28_e502_d_b6, eq28_e502_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e504;let eq28_node_derivatives: [f64; 13] = [eq28_e504_d_n0, eq28_e504_d_n1, eq28_e504_d_n2, eq28_e504_d_n3, eq28_e504_d_n4, eq28_e504_d_n5, eq28_e504_d_n6, eq28_e504_d_n7, eq28_e504_d_n8, eq28_e504_d_n9, eq28_e504_d_n10, eq28_e504_d_n11, eq28_e504_d_n12];let eq28_branch_derivatives: [f64; 8] = [eq28_e504_d_b0, eq28_e504_d_b1, eq28_e504_d_b2, eq28_e504_d_b3, eq28_e504_d_b4, eq28_e504_d_b5, eq28_e504_d_b6, eq28_e504_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq29_e509,) = {
    if (!s.b[1094]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e509;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e518, eq30_e518_d_n0, eq30_e518_d_n1, eq30_e518_d_n2, eq30_e518_d_n3, eq30_e518_d_n4, eq30_e518_d_n5, eq30_e518_d_n6, eq30_e518_d_n7, eq30_e518_d_n8, eq30_e518_d_n9, eq30_e518_d_n10, eq30_e518_d_n11, eq30_e518_d_n12, eq30_e518_d_b0, eq30_e518_d_b1, eq30_e518_d_b2, eq30_e518_d_b3, eq30_e518_d_b4, eq30_e518_d_b5, eq30_e518_d_b6, eq30_e518_d_b7,) = {
    if s.b[1095] {
        let eq30_e514: f64 = (1e-9 * (nv10 - 0.0));let eq30_e515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e514);let eq30_e516: f64 = (s.v[558] + eq30_e515);let eq30_e516_d_n10: f64 = (s.dn[558][10] + (1e-9 * ddt_scale));
        (eq30_e516, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e516_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e518;let eq30_node_derivatives: [f64; 13] = [eq30_e518_d_n0, eq30_e518_d_n1, eq30_e518_d_n2, eq30_e518_d_n3, eq30_e518_d_n4, eq30_e518_d_n5, eq30_e518_d_n6, eq30_e518_d_n7, eq30_e518_d_n8, eq30_e518_d_n9, eq30_e518_d_n10, eq30_e518_d_n11, eq30_e518_d_n12];let eq30_branch_derivatives: [f64; 8] = [eq30_e518_d_b0, eq30_e518_d_b1, eq30_e518_d_b2, eq30_e518_d_b3, eq30_e518_d_b4, eq30_e518_d_b5, eq30_e518_d_b6, eq30_e518_d_b7];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e523,) = {
    if (!s.b[1095]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e523;
        stamper.stamp_potential_const_local(
            5,
            eq31_value,
        );
        let (eq32_e532, eq32_e532_d_n0, eq32_e532_d_n1, eq32_e532_d_n2, eq32_e532_d_n3, eq32_e532_d_n4, eq32_e532_d_n5, eq32_e532_d_n6, eq32_e532_d_n7, eq32_e532_d_n8, eq32_e532_d_n9, eq32_e532_d_n10, eq32_e532_d_n11, eq32_e532_d_n12, eq32_e532_d_b0, eq32_e532_d_b1, eq32_e532_d_b2, eq32_e532_d_b3, eq32_e532_d_b4, eq32_e532_d_b5, eq32_e532_d_b6, eq32_e532_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq32_e528: f64 = (1e-9 * (nv8 - 0.0));let eq32_e529: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e528);let eq32_e530: f64 = (s.v[549] + eq32_e529);let eq32_e530_d_n8: f64 = (s.dn[549][8] + (1e-9 * ddt_scale));
        (eq32_e530, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e530_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e532;let eq32_node_derivatives: [f64; 13] = [eq32_e532_d_n0, eq32_e532_d_n1, eq32_e532_d_n2, eq32_e532_d_n3, eq32_e532_d_n4, eq32_e532_d_n5, eq32_e532_d_n6, eq32_e532_d_n7, eq32_e532_d_n8, eq32_e532_d_n9, eq32_e532_d_n10, eq32_e532_d_n11, eq32_e532_d_n12];let eq32_branch_derivatives: [f64; 8] = [eq32_e532_d_b0, eq32_e532_d_b1, eq32_e532_d_b2, eq32_e532_d_b3, eq32_e532_d_b4, eq32_e532_d_b5, eq32_e532_d_b6, eq32_e532_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e541, eq33_e541_d_n0, eq33_e541_d_n1, eq33_e541_d_n2, eq33_e541_d_n3, eq33_e541_d_n4, eq33_e541_d_n5, eq33_e541_d_n6, eq33_e541_d_n7, eq33_e541_d_n8, eq33_e541_d_n9, eq33_e541_d_n10, eq33_e541_d_n11, eq33_e541_d_n12, eq33_e541_d_b0, eq33_e541_d_b1, eq33_e541_d_b2, eq33_e541_d_b3, eq33_e541_d_b4, eq33_e541_d_b5, eq33_e541_d_b6, eq33_e541_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq33_e537: f64 = (1e-9 * (nv9 - 0.0));let eq33_e538: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e537);let eq33_e539: f64 = (s.v[550] + eq33_e538);let eq33_e539_d_n9: f64 = (s.dn[550][9] + (1e-9 * ddt_scale));
        (eq33_e539, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e539_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e541;let eq33_node_derivatives: [f64; 13] = [eq33_e541_d_n0, eq33_e541_d_n1, eq33_e541_d_n2, eq33_e541_d_n3, eq33_e541_d_n4, eq33_e541_d_n5, eq33_e541_d_n6, eq33_e541_d_n7, eq33_e541_d_n8, eq33_e541_d_n9, eq33_e541_d_n10, eq33_e541_d_n11, eq33_e541_d_n12];let eq33_branch_derivatives: [f64; 8] = [eq33_e541_d_b0, eq33_e541_d_b1, eq33_e541_d_b2, eq33_e541_d_b3, eq33_e541_d_b4, eq33_e541_d_b5, eq33_e541_d_b6, eq33_e541_d_b7];
        stamper.stamp_current_dense_local(
            Some(9),
            None,
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e546,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e546;
        stamper.stamp_potential_const_local(
            6,
            eq34_value,
        );
        let (eq35_e551,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e551;
        stamper.stamp_potential_const_local(
            7,
            eq35_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq10_e393: f64 = (s.v[561] + s.v[554]);let eq10_e393_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);let eq10_e393_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);let eq10_e393_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);let eq10_e393_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);let eq10_e393_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);let eq10_e393_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);let eq10_e393_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);let eq10_e393_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);let eq10_e393_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);let eq10_e393_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);let eq10_e393_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);let eq10_e393_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);let eq10_e393_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);let eq10_e393_d_b0: f64 = (s.db[561][0] + s.db[554][0]);let eq10_e393_d_b1: f64 = (s.db[561][1] + s.db[554][1]);let eq10_e393_d_b2: f64 = (s.db[561][2] + s.db[554][2]);let eq10_e393_d_b3: f64 = (s.db[561][3] + s.db[554][3]);let eq10_e393_d_b4: f64 = (s.db[561][4] + s.db[554][4]);let eq10_e393_d_b5: f64 = (s.db[561][5] + s.db[554][5]);let eq10_e393_d_b6: f64 = (s.db[561][6] + s.db[554][6]);let eq10_e393_d_b7: f64 = (s.db[561][7] + s.db[554][7]);let eq10_e394_q: f64 = eq10_e393;let eq10_e395: f64 = (p.p33 * eq10_e393);let eq10_e395_d_n0: f64 = (p.p33 * eq10_e393_d_n0);let eq10_e395_d_n1: f64 = (p.p33 * eq10_e393_d_n1);let eq10_e395_d_n2: f64 = (p.p33 * eq10_e393_d_n2);let eq10_e395_d_n3: f64 = (p.p33 * eq10_e393_d_n3);let eq10_e395_d_n4: f64 = (p.p33 * eq10_e393_d_n4);let eq10_e395_d_n5: f64 = (p.p33 * eq10_e393_d_n5);let eq10_e395_d_n6: f64 = (p.p33 * eq10_e393_d_n6);let eq10_e395_d_n7: f64 = (p.p33 * eq10_e393_d_n7);let eq10_e395_d_n8: f64 = (p.p33 * eq10_e393_d_n8);let eq10_e395_d_n9: f64 = (p.p33 * eq10_e393_d_n9);let eq10_e395_d_n10: f64 = (p.p33 * eq10_e393_d_n10);let eq10_e395_d_n11: f64 = (p.p33 * eq10_e393_d_n11);let eq10_e395_d_n12: f64 = (p.p33 * eq10_e393_d_n12);let eq10_e395_d_b0: f64 = (p.p33 * eq10_e393_d_b0);let eq10_e395_d_b1: f64 = (p.p33 * eq10_e393_d_b1);let eq10_e395_d_b2: f64 = (p.p33 * eq10_e393_d_b2);let eq10_e395_d_b3: f64 = (p.p33 * eq10_e393_d_b3);let eq10_e395_d_b4: f64 = (p.p33 * eq10_e393_d_b4);let eq10_e395_d_b5: f64 = (p.p33 * eq10_e393_d_b5);let eq10_e395_d_b6: f64 = (p.p33 * eq10_e393_d_b6);let eq10_e395_d_b7: f64 = (p.p33 * eq10_e393_d_b7);let eq10_e395_q: f64 = (p.p33 * eq10_e394_q);let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e395_d_n0, eq10_e395_d_n1, eq10_e395_d_n2, eq10_e395_d_n3, eq10_e395_d_n4, eq10_e395_d_n5, eq10_e395_d_n6, eq10_e395_d_n7, eq10_e395_d_n8, eq10_e395_d_n9, eq10_e395_d_n10, eq10_e395_d_n11, eq10_e395_d_n12];let eq10_reactive_branch_derivatives: [f64; 8] = [eq10_e395_d_b0, eq10_e395_d_b1, eq10_e395_d_b2, eq10_e395_d_b3, eq10_e395_d_b4, eq10_e395_d_b5, eq10_e395_d_b6, eq10_e395_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(12),
            &eq10_reactive_node_derivatives,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq11_e399: f64 = (s.v[93] + s.v[552]);let eq11_e399_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);let eq11_e399_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);let eq11_e399_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);let eq11_e399_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);let eq11_e399_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);let eq11_e399_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);let eq11_e399_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);let eq11_e399_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);let eq11_e399_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);let eq11_e399_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);let eq11_e399_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);let eq11_e399_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);let eq11_e399_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);let eq11_e399_d_b0: f64 = (s.db[93][0] + s.db[552][0]);let eq11_e399_d_b1: f64 = (s.db[93][1] + s.db[552][1]);let eq11_e399_d_b2: f64 = (s.db[93][2] + s.db[552][2]);let eq11_e399_d_b3: f64 = (s.db[93][3] + s.db[552][3]);let eq11_e399_d_b4: f64 = (s.db[93][4] + s.db[552][4]);let eq11_e399_d_b5: f64 = (s.db[93][5] + s.db[552][5]);let eq11_e399_d_b6: f64 = (s.db[93][6] + s.db[552][6]);let eq11_e399_d_b7: f64 = (s.db[93][7] + s.db[552][7]);let eq11_e400_q: f64 = eq11_e399;let eq11_e401: f64 = (p.p33 * eq11_e399);let eq11_e401_d_n0: f64 = (p.p33 * eq11_e399_d_n0);let eq11_e401_d_n1: f64 = (p.p33 * eq11_e399_d_n1);let eq11_e401_d_n2: f64 = (p.p33 * eq11_e399_d_n2);let eq11_e401_d_n3: f64 = (p.p33 * eq11_e399_d_n3);let eq11_e401_d_n4: f64 = (p.p33 * eq11_e399_d_n4);let eq11_e401_d_n5: f64 = (p.p33 * eq11_e399_d_n5);let eq11_e401_d_n6: f64 = (p.p33 * eq11_e399_d_n6);let eq11_e401_d_n7: f64 = (p.p33 * eq11_e399_d_n7);let eq11_e401_d_n8: f64 = (p.p33 * eq11_e399_d_n8);let eq11_e401_d_n9: f64 = (p.p33 * eq11_e399_d_n9);let eq11_e401_d_n10: f64 = (p.p33 * eq11_e399_d_n10);let eq11_e401_d_n11: f64 = (p.p33 * eq11_e399_d_n11);let eq11_e401_d_n12: f64 = (p.p33 * eq11_e399_d_n12);let eq11_e401_d_b0: f64 = (p.p33 * eq11_e399_d_b0);let eq11_e401_d_b1: f64 = (p.p33 * eq11_e399_d_b1);let eq11_e401_d_b2: f64 = (p.p33 * eq11_e399_d_b2);let eq11_e401_d_b3: f64 = (p.p33 * eq11_e399_d_b3);let eq11_e401_d_b4: f64 = (p.p33 * eq11_e399_d_b4);let eq11_e401_d_b5: f64 = (p.p33 * eq11_e399_d_b5);let eq11_e401_d_b6: f64 = (p.p33 * eq11_e399_d_b6);let eq11_e401_d_b7: f64 = (p.p33 * eq11_e399_d_b7);let eq11_e401_q: f64 = (p.p33 * eq11_e400_q);let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e401_d_n0, eq11_e401_d_n1, eq11_e401_d_n2, eq11_e401_d_n3, eq11_e401_d_n4, eq11_e401_d_n5, eq11_e401_d_n6, eq11_e401_d_n7, eq11_e401_d_n8, eq11_e401_d_n9, eq11_e401_d_n10, eq11_e401_d_n11, eq11_e401_d_n12];let eq11_reactive_branch_derivatives: [f64; 8] = [eq11_e401_d_b0, eq11_e401_d_b1, eq11_e401_d_b2, eq11_e401_d_b3, eq11_e401_d_b4, eq11_e401_d_b5, eq11_e401_d_b6, eq11_e401_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(12),
            &eq11_reactive_node_derivatives,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let eq12_e405: f64 = (s.v[90] + s.v[548]);let eq12_e405_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);let eq12_e405_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);let eq12_e405_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);let eq12_e405_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);let eq12_e405_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);let eq12_e405_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);let eq12_e405_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);let eq12_e405_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);let eq12_e405_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);let eq12_e405_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);let eq12_e405_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);let eq12_e405_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);let eq12_e405_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);let eq12_e405_d_b0: f64 = (s.db[90][0] + s.db[548][0]);let eq12_e405_d_b1: f64 = (s.db[90][1] + s.db[548][1]);let eq12_e405_d_b2: f64 = (s.db[90][2] + s.db[548][2]);let eq12_e405_d_b3: f64 = (s.db[90][3] + s.db[548][3]);let eq12_e405_d_b4: f64 = (s.db[90][4] + s.db[548][4]);let eq12_e405_d_b5: f64 = (s.db[90][5] + s.db[548][5]);let eq12_e405_d_b6: f64 = (s.db[90][6] + s.db[548][6]);let eq12_e405_d_b7: f64 = (s.db[90][7] + s.db[548][7]);let eq12_e406_q: f64 = eq12_e405;let eq12_e407: f64 = (p.p33 * eq12_e405);let eq12_e407_d_n0: f64 = (p.p33 * eq12_e405_d_n0);let eq12_e407_d_n1: f64 = (p.p33 * eq12_e405_d_n1);let eq12_e407_d_n2: f64 = (p.p33 * eq12_e405_d_n2);let eq12_e407_d_n3: f64 = (p.p33 * eq12_e405_d_n3);let eq12_e407_d_n4: f64 = (p.p33 * eq12_e405_d_n4);let eq12_e407_d_n5: f64 = (p.p33 * eq12_e405_d_n5);let eq12_e407_d_n6: f64 = (p.p33 * eq12_e405_d_n6);let eq12_e407_d_n7: f64 = (p.p33 * eq12_e405_d_n7);let eq12_e407_d_n8: f64 = (p.p33 * eq12_e405_d_n8);let eq12_e407_d_n9: f64 = (p.p33 * eq12_e405_d_n9);let eq12_e407_d_n10: f64 = (p.p33 * eq12_e405_d_n10);let eq12_e407_d_n11: f64 = (p.p33 * eq12_e405_d_n11);let eq12_e407_d_n12: f64 = (p.p33 * eq12_e405_d_n12);let eq12_e407_d_b0: f64 = (p.p33 * eq12_e405_d_b0);let eq12_e407_d_b1: f64 = (p.p33 * eq12_e405_d_b1);let eq12_e407_d_b2: f64 = (p.p33 * eq12_e405_d_b2);let eq12_e407_d_b3: f64 = (p.p33 * eq12_e405_d_b3);let eq12_e407_d_b4: f64 = (p.p33 * eq12_e405_d_b4);let eq12_e407_d_b5: f64 = (p.p33 * eq12_e405_d_b5);let eq12_e407_d_b6: f64 = (p.p33 * eq12_e405_d_b6);let eq12_e407_d_b7: f64 = (p.p33 * eq12_e405_d_b7);let eq12_e407_q: f64 = (p.p33 * eq12_e406_q);let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e407_d_n0, eq12_e407_d_n1, eq12_e407_d_n2, eq12_e407_d_n3, eq12_e407_d_n4, eq12_e407_d_n5, eq12_e407_d_n6, eq12_e407_d_n7, eq12_e407_d_n8, eq12_e407_d_n9, eq12_e407_d_n10, eq12_e407_d_n11, eq12_e407_d_n12];let eq12_reactive_branch_derivatives: [f64; 8] = [eq12_e407_d_b0, eq12_e407_d_b1, eq12_e407_d_b2, eq12_e407_d_b3, eq12_e407_d_b4, eq12_e407_d_b5, eq12_e407_d_b6, eq12_e407_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(12),
            &eq12_reactive_node_derivatives,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );let eq18_e436: f64 = ((nv7 - 0.0) * s.v[611]);let eq18_e436_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);let eq18_e436_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);let eq18_e436_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);let eq18_e436_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);let eq18_e436_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);let eq18_e436_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);let eq18_e436_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);let eq18_e436_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));let eq18_e436_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);let eq18_e436_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);let eq18_e436_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);let eq18_e436_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);let eq18_e436_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);let eq18_e436_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);let eq18_e436_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);let eq18_e436_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);let eq18_e436_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);let eq18_e436_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);let eq18_e436_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);let eq18_e436_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);let eq18_e436_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);let eq18_e437_q: f64 = eq18_e436;let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e436_d_n0, eq18_e436_d_n1, eq18_e436_d_n2, eq18_e436_d_n3, eq18_e436_d_n4, eq18_e436_d_n5, eq18_e436_d_n6, eq18_e436_d_n7, eq18_e436_d_n8, eq18_e436_d_n9, eq18_e436_d_n10, eq18_e436_d_n11, eq18_e436_d_n12];let eq18_reactive_branch_derivatives: [f64; 8] = [eq18_e436_d_b0, eq18_e436_d_b1, eq18_e436_d_b2, eq18_e436_d_b3, eq18_e436_d_b4, eq18_e436_d_b5, eq18_e436_d_b6, eq18_e436_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(12),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let eq19_e440: f64 = ((nv7 - 0.0) * s.v[612]);let eq19_e440_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);let eq19_e440_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);let eq19_e440_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);let eq19_e440_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);let eq19_e440_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);let eq19_e440_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);let eq19_e440_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);let eq19_e440_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));let eq19_e440_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);let eq19_e440_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);let eq19_e440_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);let eq19_e440_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);let eq19_e440_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);let eq19_e440_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);let eq19_e440_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);let eq19_e440_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);let eq19_e440_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);let eq19_e440_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);let eq19_e440_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);let eq19_e440_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);let eq19_e440_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);let eq19_e441_q: f64 = eq19_e440;let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e440_d_n0, eq19_e440_d_n1, eq19_e440_d_n2, eq19_e440_d_n3, eq19_e440_d_n4, eq19_e440_d_n5, eq19_e440_d_n6, eq19_e440_d_n7, eq19_e440_d_n8, eq19_e440_d_n9, eq19_e440_d_n10, eq19_e440_d_n11, eq19_e440_d_n12];let eq19_reactive_branch_derivatives: [f64; 8] = [eq19_e440_d_b0, eq19_e440_d_b1, eq19_e440_d_b2, eq19_e440_d_b3, eq19_e440_d_b4, eq19_e440_d_b5, eq19_e440_d_b6, eq19_e440_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(11),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
