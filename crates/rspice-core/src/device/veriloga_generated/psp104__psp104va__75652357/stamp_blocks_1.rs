#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }
        s.copy_ad(206, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }
        s.copy_ad(208, 76);s.copy_ad(209, 77);
        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 79);
        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(212, 80);
            } else {
                s.store_scalar(212, 1.0);
            }
        } else {
            s.store_scalar(212, (-0.5));
        }
        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }
        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }
        s.copy_ad(215, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(216, 84);
            } else {
                s.store_scalar(216, 1.0);
            }
        } else {
            s.store_scalar(216, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }
        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }
        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }
        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }
        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }
        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }
        s.copy_ad(223, 91);
        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }
        s.copy_ad(225, 93);s.copy_ad(226, 94);
        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }
        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }
        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }
        s.copy_ad(230, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }
        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }
        s.copy_ad(234, 102);s.copy_ad(235, 103);s.copy_ad(236, 104);s.copy_ad(237, 105);s.copy_ad(238, 106);s.copy_ad(239, 107);s.copy_ad(240, 108);s.copy_ad(241, 109);
        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }
        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }
        s.copy_ad(244, 112);s.copy_ad(245, 113);s.copy_ad(246, 114);s.copy_ad(247, 115);s.copy_ad(248, 116);s.copy_ad(249, 117);
        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }
        s.copy_ad(251, 119);
        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }
        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }
        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }
        s.copy_ad(255, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }
        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }
        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }
        s.copy_ad(259, 127);s.copy_ad(260, 128);s.copy_ad(261, 129);
        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }
        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }
        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.copy_ad(265, 133);s.copy_ad(266, 134);s.copy_ad(267, 135);s.copy_ad(268, 136);
        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }
        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }
        s.copy_ad(271, 139);
        if (s.v[140] > 0.0) {
            s.copy_ad(272, 140);
        } else {
            s.store_scalar(272, 0.0);
        }
        s.copy_ad(277, 145);s.copy_ad(278, 146);s.copy_ad(279, 147);
        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(280, 148);
            } else {
                s.store_scalar(280, 1e26);
            }
        } else {
            s.store_scalar(280, 1e20);
        }
        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }
        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }
        s.copy_ad(283, 151);
        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }
        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(285, 153);
            } else {
                s.store_scalar(285, 1.0);
            }
        } else {
            s.store_scalar(285, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }
        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(289, 157);
            } else {
                s.store_scalar(289, 1.0);
            }
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }
        if (s.v[163] > 0.0) {
            s.copy_ad(295, 163);
        } else {
            s.store_scalar(295, 0.0);
        }
        s.copy_ad(296, 166);s.copy_ad(297, 167);s.copy_ad(298, 169);s.copy_ad(299, 170);s.copy_ad(300, 171);s.copy_ad(301, 168);
        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_primal_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }
        s.store_scalar(16, p.p16);s.store_scalar(17, p.p15);s.store_scalar(18, p.p18);s.store_scalar(19, p.p17);s.b[1130] = (p.p44 == 0.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.copy_ad(188, 187);s.copy_ad(190, 189);s.copy_ad(243, 242);s.copy_ad(245, 244);s.copy_ad(247, 246);s.copy_ad(249, 248);s.copy_ad(233, 232);s.copy_ad(239, 237);s.copy_ad(240, 238);s.copy_ad(258, 257);s.copy_ad(260, 259);s.copy_ad(264, 263);s.copy_ad(270, 269);}
        s.store_primal_scale(762, 177, 8.8541878176e-12);s.store_primal_div(763, 762, 176);s.store_primal_square(764, 176);s.store_primal_scale(765, 763, 6.241449993689894e18);s.store_primal_mul(766, 252, 178);
        if (s.v[766] > 1e20) {
            if (s.v[766] < 1e26) {
            } else {
                s.store_scalar(766, 1e26);
            }
        } else {
            s.store_scalar(766, 1e20);
        }
        s.store_scalar(767, 0.0);s.b[1131] = (p.p51 > 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_primal_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));}
        s.b[1132] = (s.v[0] == (-1.0));s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1132]) {s.store_primal_scale(767, 767, (7.448711 / 5.951993));}
        s.store_primal_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));s.store_primal_scale(769, 209, 0.5);s.store_scalar(770, 0.5);s.b[1133] = (s.v[0] == (-1.0));s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if s.b[1133] {s.store_primal_scale(769, 209, 0.3333333333333333);s.store_scalar(770, 0.3333333333333333);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        s.store_primal_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(771, 1000, (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(772, 1000, (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(773, 1.0, 223);s.store_primal_div(774, 762, 187);s.store_primal_div(775, 762, 188);s.store_primal_div_mixed_ai(776, A::sqrt_scaled_input(s.ad_value(189), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 774);s.store_primal_div_mixed_ai(777, A::sqrt_scaled_input(s.ad_value(190), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 775);s.store_primal_square(778, 776);s.store_primal_square(779, 777);s.store_primal_offset_div_ad(780, A::ln(A::offset(A::exp_scaled_input(s.ad_value(261), (0.005 * s.v[349])), (-1.0))), s.ad_value(261), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(781, A::ln_scaled_input(s.ad_value(776), 0.5), 780);s.store_primal_add_mixed_ai(782, A::ln_scaled_input(s.ad_value(777), 0.5), 780);s.store_primal_div_from_scalar(814, 1.0, 776);s.store_primal_offset_scaled(815, 776, 3.1, 8.5);s.store_primal_square(783, 815);s.store_primal_scale(816, 815, 0.5);s.b[1134] = (s.v[814] < 0.06);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if s.b[1134] {s.store_primal_scale(784, 814, 64.0);}
        s.b[1135] = (s.v[814] <= 0.45);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if ((!s.b[1134]) && s.b[1135]) {s.store_primal_offset_scaled(784, 814, 22.0, 3.0);}
        s.b[1136] = (s.v[814] <= 1.6);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });
        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {s.store_primal_offset_scaled(784, 814, (-7.2), 15.5);}
        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {s.copy_ad(784, 776);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(785, 816, 1.0, 778, 0.5, 776, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), (-1.0));s.store_primal_div_from_scalar(814, 1.0, 777);s.store_primal_offset_scaled(815, 777, 3.1, 8.5);s.store_primal_square(786, 815);s.store_primal_scale(816, 815, 0.5);s.b[1137] = (s.v[814] < 0.06);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if s.b[1137] {s.store_primal_scale(787, 814, 64.0);}
        s.b[1138] = (s.v[814] <= 0.45);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if ((!s.b[1137]) && s.b[1138]) {s.store_primal_offset_scaled(787, 814, 22.0, 3.0);}
        s.b[1139] = (s.v[814] <= 1.6);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {s.store_primal_offset_scaled(787, 814, (-7.2), 15.5);}
        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {s.copy_ad(787, 777);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(788, 816, 1.0, 779, 0.5, 777, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), (-1.0));s.store_primal_add_scaled_inputs_ad(722, A::offset(s.ad_value(182), s.v[356]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]));
        if (!(s.v[722] > 0.05)) {s.store_scalar(722, 0.05);}
        s.store_primal_div_mixed_ai(723, A::sqrt_scaled_input(s.ad_value(178), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);s.store_scalar(724, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(725, 0.0);s.b[1140] = (s.v[183] > 0.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_primal_div_from_scalar(726, 80000000.0, 764);}
        if s.b[1140] {
            if (s.v[183] > s.v[726]) {
                s.copy_ad(725, 183);
            } else {
                s.copy_ad(725, 726);
            }
        }
        if s.b[1140] {
            if (5e24 > s.v[725]) {
                s.store_scalar(725, 5e24);
            } else {
            }
        }
        if s.b[1140] {s.store_primal_div_scaled_product_indices(724, 763, 763, (2.0 * s.v[709]), 725, (1.6021918e-19 * s.v[761]));}
        s.store_scalar(727, ((100.0 * s.v[709]) * s.v[709]));s.b[1141] = (p.p51 > 0.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_primal_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(723), s.ad_value(723), s.ad_value(722), s.v[709]));s.store_primal_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);s.store_primal_add(722, 722, 729);s.store_primal_mul_scale_offset_mixed_ia(723, 723, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0, 1.0);}
        s.store_primal_sqrt(730, 722);s.store_primal_scale(731, 722, 0.95);s.store_primal_scaled_mul(732, 722, 722, 0.0025);s.copy_ad(733, 732);s.store_primal_scaled_sqrt(734, 733, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(735, 731, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(731), s.ad_value(734))), s.ad_value(732)), (-0.5));s.store_primal_scaled_offset(736, 722, s.v[356], 0.5);s.store_primal_sub_mixed_ai(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);s.store_primal_add_scaled_inputs3_sqrt_first_mixed_aii(738, A::add_scaled_inputs3(s.ad_value(180), 1.0, s.ad_value(181), 1.0, s.ad_value(722), 1.0), 1.0, 730, (-1.0), 737, -1.0);s.store_primal_add_scaled_inputs3_offset_mixed_iia(739, 182, 1.0, 251, 1.0, A::ln_scaled_input(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]), s.v[356]);
        if (!(s.v[739] > 0.05)) {s.store_scalar(739, 0.05);}
        s.store_primal_div_mixed_ai(740, A::sqrt_scaled_input(s.ad_value(766), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);s.b[1142] = (p.p51 > 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_primal_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(740), s.ad_value(740), s.ad_value(739), s.v[709]));s.store_primal_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);s.store_primal_add(739, 739, 729);s.store_primal_mul_scale_offset_mixed_ia(740, 740, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0, 1.0);}
        s.store_primal_scale(741, 739, 0.95);s.store_primal_scaled_mul(742, 739, 739, 0.0025);s.copy_ad(743, 742);s.store_primal_scaled_sqrt(734, 743, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(744, 741, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(741), s.ad_value(734))), s.ad_value(742)), (-0.5));s.store_primal_offset_add_scaled_product_mixed_iia(694, 172, 1.0, 173, A::scale_offset(s.ad_value(174), s.v[352], 1.0), s.v[352], s.v[17]);s.store_primal_exp_scaled_input(745, 175, s.v[354]);s.store_primal_mul(695, 184, 745);s.store_primal_scale(696, 185, 1.0 / (s.v[353]));s.store_primal_exp_scaled_input(746, 198, s.v[354]);s.store_primal_mul(697, 197, 746);s.store_primal_scaled_mul(710, 697, 763, s.v[16]);s.store_primal_mul_mixed_ia(699, 201, A::exp_scaled_input(s.ad_value(202), s.v[354]));s.store_primal_exp_scaled_input(747, 200, s.v[354]);s.store_primal_mul(698, 199, 747);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_mul_mixed_ia(701, 205, A::exp_scaled_input(s.ad_value(206), s.v[354]));s.store_primal_exp_scaled_input(748, 204, s.v[354]);s.store_primal_mul(700, 203, 748);s.store_primal_exp_scaled_input(749, 208, s.v[354]);s.store_primal_mul(702, 207, 749);s.store_primal_exp_scaled_input(750, 211, s.v[354]);s.store_primal_mul(703, 210, 750);s.store_primal_scaled_mul(751, 710, 703, 2.0);s.store_primal_exp_scaled_input(752, 215, s.v[354]);s.store_primal_mul(714, 214, 752);s.store_primal_mul(715, 253, 752);s.store_primal_mul_mixed_ia(706, 225, A::exp_scaled_input(s.ad_value(226), (-s.v[354])));s.store_primal_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));s.b[1143] = ((p.p46 != 0.0) && (s.v[282] > 0.0));s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_primal_offset_add_scaled_inputs_indices(707, 277, 1.0, 278, s.v[352], s.v[19]);s.store_primal_exp_scaled_input(753, 283, s.v[354]);s.store_primal_mul(708, 282, 753);s.store_primal_scaled_mul(711, 708, 763, s.v[18]);s.store_primal_offset_scaled(717, 281, ((s.v[353]) * (s.v[709])), s.v[709]);s.store_primal_add_scaled_product_mixed_aia(754, A::offset(s.ad_value(279), s.v[356]), 1.0, 717, A::ln_scaled_input(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26), 2.0);}
        if s.b[1143] {
            if (s.v[754] > 0.05) {
            } else {
                s.store_scalar(754, 0.05);
            }
        }
        if s.b[1143] {s.store_primal_div_mixed_ai(755, A::sqrt_scaled_input(s.ad_value(280), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);s.store_primal_square(718, 755);s.store_primal_ln(719, 718);s.store_primal_scale(756, 754, 0.95);s.store_primal_scaled_mul(757, 754, 754, 0.0025);s.copy_ad(758, 757);s.store_primal_scaled_sqrt(759, 758, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(760, 756, 0.5, 759, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(756), s.ad_value(759))), s.ad_value(757)), (-0.5));}
        if (!s.b[1143]) {s.store_scalar(707, 0.0);s.store_scalar(753, 1.0);s.store_scalar(708, 0.0);s.store_scalar(711, 0.0);s.store_scalar(717, s.v[709]);s.store_scalar(754, 0.0);s.store_scalar(755, 1.0);s.store_scalar(718, 1.0);s.store_scalar(719, 0.0);s.store_scalar(756, 0.0);s.store_scalar(757, 0.0);s.store_scalar(758, 0.0);s.store_scalar(759, 0.0);s.store_scalar(760, 0.0);}
        s.store_primal_div_from_scalar(789, 1.0, 241);s.store_primal_scaled_sqrt_scaled_input(790, 241, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));s.store_primal_mul(791, 790, 176);s.store_primal_mul(792, 790, 187);s.store_primal_mul(793, 790, 188);s.store_scalar(794, 0.0);s.b[1144] = (s.v[236] < 0.0);s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_primal_div_scaled_inputs_indices(794, 235, (-0.495), 236, 1.0);}
        s.store_scalar(795, 0.0);s.b[1145] = (s.v[238] < 0.0);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_primal_div_scaled_inputs_indices(795, 237, (-0.495), 238, 1.0);}
        s.b[1146] = (s.v[240] < 0.0);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_primal_div_scaled_inputs_indices(796, 239, (-0.495), 240, 1.0);}
        s.store_primal_pow_from_scalar_ad(797, s.v[346], s.ad_value(234));s.store_primal_mul(231, 231, 797);s.store_primal_mul(232, 232, 797);s.store_primal_mul(233, 233, 797);s.store_primal_div_scaled_inputs_square_rhs(798, 242, 4e-18, 187, 1.0);s.store_primal_div_scaled_inputs_square_rhs(799, 243, 4e-18, 188, 1.0);
        if ((1.0 + (s.v[246] * s.v[347])) > 0.0) {
            s.store_primal_offset_scaled(790, 246, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }
        s.store_primal_mul(704, 244, 790);s.store_primal_scaled_mul(800, 704, 187, 500000000.0);
        if ((1.0 + (s.v[247] * s.v[347])) > 0.0) {
            s.store_primal_offset_scaled(790, 247, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }
        s.store_primal_mul(705, 245, 790);s.store_primal_scaled_mul(801, 705, 188, 500000000.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(802, 0.0);s.b[1147] = (s.v[267] > 1e-10);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_primal_div_from_scalar(802, 0.75, 267);}
        s.store_primal_square(803, 268);s.store_primal_scale(804, 272, (9.1093826e-31 * 1000000000.0));s.b[1148] = (s.v[295] > 0.0);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_primal_div_from_scalar(805, 1.0, 295);}
        if (!s.b[1148]) {s.store_scalar(805, 0.0);}
        s.b[1149] = (s.v[296] > 0.0);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_primal_div_from_scalar(806, 1.0, 296);}
        if (!s.b[1149]) {s.store_scalar(806, 0.0);}
        s.b[1150] = (s.v[297] > 0.0);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if s.b[1150] {s.store_primal_div_from_scalar(807, 1.0, 297);}
        if (!s.b[1150]) {s.store_scalar(807, 0.0);}
        s.b[1151] = (s.v[298] > 0.0);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if s.b[1151] {s.store_primal_div_from_scalar(808, 1.0, 298);}
        if (!s.b[1151]) {s.store_scalar(808, 0.0);}
        s.b[1152] = (s.v[299] > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_primal_div_from_scalar(809, 1.0, 299);}
        if (!s.b[1152]) {s.store_scalar(809, 0.0);}
        s.b[1153] = (s.v[300] > 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_primal_div_from_scalar(810, 1.0, 300);}
        if (!s.b[1153]) {s.store_scalar(810, 0.0);}
        s.b[1154] = (s.v[301] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_primal_div_from_scalar(811, 1.0, 301);}
        if (!s.b[1154]) {s.store_scalar(811, 0.0);}
        s.store_primal_scale(20, 2, s.v[640]);s.store_primal_scale(21, 2, s.v[641]);s.store_primal_scale(22, 2, s.v[642]);s.store_primal_scale(23, 2, s.v[667]);s.store_primal_scale(24, 2, s.v[668]);s.store_primal_scale(25, 2, s.v[669]);s.store_scalar(26, 0.0);s.b[1155] = (p.p43 == 3.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_scalar(26, 1.0);}
        s.copy_ad(27, 307);s.b[1156] = (p.p39 == 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));}
        s.b[1157] = ((p.p43 == 2.0) || (p.p43 == 3.0));s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_primal_scale(20, 2, s.v[643]);s.store_primal_add_scaled_product_indices(21, 2, s.v[644], 26, 27, (-1.0));s.copy_ad(22, 27);s.store_primal_scale(23, 2, s.v[670]);s.store_primal_add_scaled_product_indices(24, 2, s.v[671], 26, 27, (-1.0));s.copy_ad(25, 27);}
        s.b[1158] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {
            if (s.v[20] > 0.0) {
                s.copy_ad(640, 20);
            } else {
                s.store_scalar(640, 0.0);
            }
        }
        if s.b[1158] {
            if (s.v[21] > 0.0) {
                s.copy_ad(641, 21);
            } else {
                s.store_scalar(641, 0.0);
            }
        }
        if s.b[1158] {
            if (s.v[22] > 0.0) {
                s.copy_ad(642, 22);
            } else {
                s.store_scalar(642, 0.0);
            }
        }
        if s.b[1158] {
            if (s.v[23] > 0.0) {
                s.copy_ad(667, 23);
            } else {
                s.store_scalar(667, 0.0);
            }
        }
        if s.b[1158] {
            if (s.v[24] > 0.0) {
                s.copy_ad(668, 24);
            } else {
                s.store_scalar(668, 0.0);
            }
        }
        if s.b[1158] {
            if (s.v[25] > 0.0) {
                s.copy_ad(669, 25);
            } else {
                s.store_scalar(669, 0.0);
            }
        }
        if (!s.b[1158]) {s.store_scalar(640, 0.0);s.store_scalar(641, 0.0);s.store_scalar(642, 0.0);s.store_scalar(667, 0.0);s.store_scalar(668, 0.0);s.store_scalar(669, 0.0);}
        s.store_scalar(650, 0.0);s.store_scalar(677, 0.0);s.store_scalar(652, 0.0);s.store_scalar(679, 0.0);s.store_scalar(651, 0.0);s.store_scalar(678, 0.0);s.store_scalar(653, 0.0);s.store_scalar(680, 0.0);s.store_scalar(648, 0.0);s.store_scalar(675, 0.0);s.store_scalar(649, 0.0);s.store_scalar(676, 0.0);s.store_scalar(661, 0.0);s.store_scalar(688, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(662, 1.0);s.store_scalar(689, 1.0);s.store_scalar(663, 0.0);s.store_scalar(690, 0.0);s.store_scalar(664, 1.0);s.store_scalar(691, 1.0);s.store_scalar(665, 0.0);s.store_scalar(692, 0.0);s.store_scalar(666, 1.0);s.store_scalar(693, 1.0);s.store_scalar(660, 0.0);s.store_scalar(687, 0.0);s.store_scalar(654, 0.0);s.store_scalar(681, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(645, 1.0);s.store_scalar(672, 1.0);s.store_scalar(646, 1.0);s.store_scalar(673, 1.0);s.store_scalar(647, 1.0);s.store_scalar(674, 1.0);s.store_scalar(485, 0.0);s.store_scalar(486, 0.0);s.store_scalar(474, 0.0);s.store_scalar(475, 0.0);s.store_scalar(476, 0.0);s.store_scalar(477, 0.0);s.store_scalar(478, 0.0);s.store_scalar(487, 0.0);s.store_scalar(488, 0.0);s.store_scalar(489, 0.0);s.store_scalar(495, 0.0);s.store_scalar(484, 0.0);s.b[1159] = (p.p43 > 0.0);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = ((s.v[381] * s.v[640]) > 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1160]) {s.store_primal_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1160])) {s.store_scalar(448, 100000000.0);}
        s.b[1161] = ((s.v[382] * s.v[641]) > 0.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1161]) {s.store_primal_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1161])) {s.store_scalar(449, 100000000.0);}
        s.b[1162] = ((s.v[383] * s.v[642]) > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1162]) {s.store_primal_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1162])) {s.store_scalar(450, 100000000.0);}
        if s.b[1159] {s.store_min3(648, 448, 449, 450);}
        s.b[1163] = ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1163]) {s.store_primal_exp_scaled_input(649, 648, s.v[365]);}
        s.b[1164] = ((s.v[648] * s.v[365]) < 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(649, 1e-100, (-230.25850929940458), A::scale(s.ad_value(648), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {s.store_primal_scaled_offset_ad(649, A::mul_offset_rhs(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(648), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1159] {s.store_scalar(390, s.v[387]);s.store_scalar(391, s.v[388]);s.store_scalar(392, s.v[389]);s.store_scalar(393, p.p824);s.store_scalar(394, p.p825);s.store_scalar(395, p.p826);s.store_scalar(396, p.p821);s.store_scalar(397, p.p822);s.store_scalar(398, p.p823);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1165] = (s.v[640] == 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1165]) {s.store_scalar(390, (s.v[388] + s.v[389]));s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));s.store_scalar(396, (p.p822 + p.p823));}
        s.b[1166] = (s.v[641] == 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1166]) {s.store_scalar(391, (s.v[387] + s.v[389]));s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));s.store_scalar(397, (p.p821 + p.p823));}
        s.b[1167] = (s.v[642] == 0.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1167]) {s.store_scalar(392, (s.v[387] + s.v[388]));s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));s.store_scalar(398, (p.p821 + p.p822));}
        if s.b[1159] {s.store_min3(650, 390, 391, 392);s.store_primal_scale(651, 650, 0.1);s.store_max3(371, 393, 394, 395);s.store_primal_mul_scale_offset_mixed_ia(652, 650, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))), -1.0, 1.0);s.store_primal_offset_min_ad(653, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));}
        s.b[1168] = ((s.v[557] * s.v[667]) > 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1168]) {s.store_primal_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1168])) {s.store_scalar(448, 100000000.0);}
        s.b[1169] = ((s.v[558] * s.v[668]) > 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1169]) {s.store_primal_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1169])) {s.store_scalar(449, 100000000.0);}
        s.b[1170] = ((s.v[559] * s.v[669]) > 0.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1170]) {s.store_primal_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0), s.v[364]);}
        if (s.b[1159] && (!s.b[1170])) {s.store_scalar(450, 100000000.0);}
        if s.b[1159] {s.store_min3(675, 448, 449, 450);}
        s.b[1171] = ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1171]) {s.store_primal_exp_scaled_input(676, 675, s.v[365]);}
        s.b[1172] = ((s.v[675] * s.v[365]) < 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1171])) && s.b[1172]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(676, 1e-100, (-230.25850929940458), A::scale(s.ad_value(675), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1159] && (!s.b[1171])) && (!s.b[1172])) {s.store_primal_scaled_offset_ad(676, A::mul_offset_rhs(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(675), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1159] {s.copy_ad(390, 563);s.copy_ad(391, 564);s.copy_ad(392, 565);s.copy_ad(393, 505);s.copy_ad(394, 506);s.copy_ad(395, 507);s.copy_ad(396, 502);s.copy_ad(397, 503);s.copy_ad(398, 504);}
        s.b[1173] = (s.v[667] == 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1173]) {s.store_primal_add(390, 564, 565);s.store_primal_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);s.store_primal_add(396, 503, 504);}
        s.b[1174] = (s.v[668] == 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1174]) {s.store_primal_add(391, 563, 565);s.store_primal_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1159] && s.b[1174]) {s.store_primal_add(397, 502, 504);}
        s.b[1175] = (s.v[669] == 0.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1175]) {s.store_primal_add(392, 563, 564);s.store_primal_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);s.store_primal_add(398, 502, 503);}
        if s.b[1159] {s.store_min3(677, 390, 391, 392);s.store_primal_scale(678, 677, 0.1);s.store_max3(371, 393, 394, 395);s.store_primal_mul_scale_offset_mixed_ia(679, 677, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))), -1.0, 1.0);s.store_primal_offset_min_ad(680, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));}
        s.b[1176] = (s.v[468] == 1.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1176]) {s.store_scalar(1177, 0.0);s.store_scalar(1178, 0.0);s.store_scalar(1179, 0.0);s.store_scalar(1186, 0.0);s.store_scalar(1188, 0.0);s.store_scalar(1189, 0.0);s.store_scalar(1190, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1192, 0.0);s.store_scalar(1193, 0.0);s.store_scalar(1194, 0.0);s.store_scalar(1195, 0.0);s.store_scalar(1196, 0.0);s.store_scalar(1197, 0.0);s.store_scalar(1198, 0.0);s.store_scalar(1199, 0.0);s.store_scalar(1200, 0.0);s.store_scalar(1201, 0.0);s.store_scalar(1202, 0.0);s.store_scalar(1203, 0.0);s.store_scalar(1204, 0.0);s.store_scalar(1205, 0.0);s.store_scalar(1206, 0.0);s.store_scalar(1207, 0.0);s.store_scalar(1208, 0.0);s.store_scalar(1209, 0.0);s.store_scalar(1210, 0.0);s.store_scalar(1211, 0.0);s.store_scalar(1212, 0.0);s.store_scalar(1213, 0.0);s.store_scalar(1214, 0.0);s.store_scalar(1215, 0.0);s.store_scalar(1216, 0.0);s.store_scalar(1217, 0.0);s.store_scalar(1218, 0.0);s.store_scalar(1219, 0.0);s.store_scalar(1220, 0.0);s.store_scalar(1221, 0.0);s.store_scalar(492, 0.4);s.store_scalar(493, 0.65);s.store_scalar(494, 0.8);s.store_primal_scale(479, 492, (-p.p921));s.store_primal_scale(480, 493, (-p.p921));s.store_primal_scale(481, 494, (-p.p921));s.store_scalar(482, 0.1);s.store_scalar(483, 0.2);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1225] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });s.b[1226] = (s.v[479] < s.v[648]);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });s.b[1227] = (((((-0.5) * (s.v[479] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && s.b[1227]) {s.store_primal_exp_scaled_input(1188, 479, (s.v[365] * (-0.5)));}
        s.b[1228] = (((-0.5) * (s.v[479] * s.v[365])) < 0.0);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && (!s.b[1227])) && s.b[1228]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && (!s.b[1227])) && (!s.b[1228])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(479), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && (!s.b[1226])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 649, A::sub_scaled_inputs(s.ad_value(479), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1225]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1229] = (s.v[479] > 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1229]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && (!s.b[1229])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 479);}
        if ((s.b[1159] && s.b[1176]) && s.b[1225]) {s.store_primal_sub(1191, 650, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 479, 0.5, 1191, 0.5, 479, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 479, 0.5, 653, 0.5, 479, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 479, A::sqrt_square_offset(s.ad_value(479), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1230] = (s.v[640] == 0.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1230]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1230])) {s.store_primal_scale(1196, 1186, s.v[381]);}
        s.b[1231] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1231]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {s.store_primal_sub_from_scalar(1198, s.v[387], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1232] = (p.p824 == 0.5);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && s.b[1232]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && (!s.b[1232])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1233] = (p.p824 == 0.5);s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && s.b[1233]) {s.store_sqrt_scaled_input(1195, 1198, s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && (!s.b[1233])) {s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {s.store_scale(1202, 1195, s.v[417]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[378], ((-1.0)) * (s.v[378]));s.store_scaled_mul(1197, 1203, 1201, p.p833);}
        s.b[1234] = (p.p838 == 0.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1234]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1235] = (((-p.p824) * s.v[405]) == (-1.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1235]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1235])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[429]), 1206, 1209, s.v[429], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1236] = (s.v[1216] > 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1236]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1236])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1237] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1237]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1237])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1238] = (s.v[1216] > 0.0);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1238]) {s.copy_ad(1217, 1179);}
        s.b[1239] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) && s.b[1239]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) && (!s.b[1239])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);}
        s.b[1240] = (p.p844 == 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1240]) {s.store_scalar(1219, 0.0);}
        s.b[1241] = (p.p824 == 0.5);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && s.b[1241]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1241])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 1195, 1.0);}
        s.b[1242] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && s.b[1242]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));}
        s.b[1243] = (((-s.v[435]) / s.v[1220]) < 0.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1242])) && s.b[1243]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1242])) && (!s.b[1243])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844, 0.0);}
        s.b[1244] = (p.p853 > 1000.0);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1244]) {s.store_scalar(1221, 1.0);}
        s.b[1245] = (s.v[1194] > ((-s.v[438]) * p.p853));s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });s.b[1246] = (p.p856 == 4.0);s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) && s.b[1246]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) && (!s.b[1246])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && (!s.b[1245])) {s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1230])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1247] = (s.v[641] == 0.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1247]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1247])) {s.store_primal_scale(1196, 1186, s.v[382]);}
        s.b[1248] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1248]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {s.store_primal_sub_from_scalar(1198, s.v[388], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1249] = (p.p825 == 0.5);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && s.b[1249]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && (!s.b[1249])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1250] = (p.p825 == 0.5);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && s.b[1250]) {s.store_sqrt_scaled_input(1195, 1198, s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && (!s.b[1250])) {s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {s.store_scale(1202, 1195, s.v[418]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[379], ((-1.0)) * (s.v[379]));s.store_scaled_mul(1197, 1203, 1201, p.p834);}
        s.b[1251] = (p.p839 == 0.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1251]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1252] = (((-p.p825) * s.v[406]) == (-1.0));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1252]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1252])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[430]), 1206, 1209, s.v[430], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1253] = (s.v[1216] > 0.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1253]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1253])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1254] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1254]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1254])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1255] = (s.v[1216] > 0.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1255]) {s.copy_ad(1217, 1179);}
        s.b[1256] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) && s.b[1256]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) && (!s.b[1256])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);}
        s.b[1257] = (p.p845 == 0.0);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1257]) {s.store_scalar(1219, 0.0);}
        s.b[1258] = (p.p825 == 0.5);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && s.b[1258]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1258])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 1195, 1.0);}
        s.b[1259] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && s.b[1259]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));}
        s.b[1260] = (((-s.v[436]) / s.v[1220]) < 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1259])) && s.b[1260]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1259])) && (!s.b[1260])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845, 0.0);}
        s.b[1261] = (p.p854 > 1000.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1261]) {s.store_scalar(1221, 1.0);}
        s.b[1262] = (s.v[1194] > ((-s.v[438]) * p.p854));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });s.b[1263] = (p.p857 == 4.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) && s.b[1263]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) && (!s.b[1263])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && (!s.b[1262])) {s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1247])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1264] = (s.v[642] == 0.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1264]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1264])) {s.store_primal_scale(1196, 1186, s.v[383]);}
        s.b[1265] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1265]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {s.store_primal_sub_from_scalar(1198, s.v[389], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1266] = (p.p826 == 0.5);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && (!s.b[1266])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1267] = (p.p826 == 0.5);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && s.b[1267]) {s.store_sqrt_scaled_input(1195, 1198, s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && (!s.b[1267])) {s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {s.store_scale(1202, 1195, s.v[419]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(1197, 1203, 1201, p.p835);}
        s.b[1268] = (p.p840 == 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1268]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1269] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1269]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1269])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[431]), 1206, 1209, s.v[431], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1270] = (s.v[1216] > 0.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1270]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1270])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1271] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1271]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1271])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1272] = (s.v[1216] > 0.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1272]) {s.copy_ad(1217, 1179);}
        s.b[1273] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) && s.b[1273]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) && (!s.b[1273])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);}
        s.b[1274] = (p.p846 == 0.0);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1274]) {s.store_scalar(1219, 0.0);}
        s.b[1275] = (p.p826 == 0.5);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && s.b[1275]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1275])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 1195, 1.0);}
        s.b[1276] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && s.b[1276]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));}
        s.b[1277] = (((-s.v[437]) / s.v[1220]) < 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1276])) && s.b[1277]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1276])) && (!s.b[1277])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846, 0.0);}
        s.b[1278] = (p.p855 > 1000.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1278]) {s.store_scalar(1221, 1.0);}
        s.b[1279] = (s.v[1194] > ((-s.v[438]) * p.p855));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });s.b[1280] = (p.p858 == 4.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) && s.b[1280]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) && (!s.b[1280])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && (!s.b[1279])) {s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1264])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(469, 640, 1222, 1.0, 641, 1223, 1.0, 642, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1281] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });s.b[1282] = (s.v[480] < s.v[648]);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });s.b[1283] = (((((-0.5) * (s.v[480] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && s.b[1283]) {s.store_primal_exp_scaled_input(1188, 480, (s.v[365] * (-0.5)));}
        s.b[1284] = (((-0.5) * (s.v[480] * s.v[365])) < 0.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && (!s.b[1283])) && s.b[1284]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && (!s.b[1283])) && (!s.b[1284])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(480), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && (!s.b[1282])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 649, A::sub_scaled_inputs(s.ad_value(480), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1281]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1285] = (s.v[480] > 0.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1285]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && (!s.b[1285])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 480);}
        if ((s.b[1159] && s.b[1176]) && s.b[1281]) {s.store_primal_sub(1191, 650, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 480, 0.5, 1191, 0.5, 480, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 480, 0.5, 653, 0.5, 480, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 480, A::sqrt_square_offset(s.ad_value(480), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1286] = (s.v[640] == 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1286]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1286])) {s.store_primal_scale(1196, 1186, s.v[381]);}
        s.b[1287] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1287]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {s.store_primal_sub_from_scalar(1198, s.v[387], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1288] = (p.p824 == 0.5);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && s.b[1288]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && (!s.b[1288])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1289] = (p.p824 == 0.5);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && s.b[1289]) {s.store_sqrt_scaled_input(1195, 1198, s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && (!s.b[1289])) {s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {s.store_scale(1202, 1195, s.v[417]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[378], ((-1.0)) * (s.v[378]));s.store_scaled_mul(1197, 1203, 1201, p.p833);}
        s.b[1290] = (p.p838 == 0.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1290]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1291] = (((-p.p824) * s.v[405]) == (-1.0));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1291]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1291])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[429]), 1206, 1209, s.v[429], 1205, 1210, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1292] = (s.v[1216] > 0.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1292]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1292])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1293] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1293]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1293])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1294] = (s.v[1216] > 0.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1294]) {s.copy_ad(1217, 1179);}
        s.b[1295] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) && s.b[1295]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) && (!s.b[1295])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);}
        s.b[1296] = (p.p844 == 0.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1296]) {s.store_scalar(1219, 0.0);}
        s.b[1297] = (p.p824 == 0.5);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && s.b[1297]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1297])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 1195, 1.0);}
        s.b[1298] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && s.b[1298]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));}
        s.b[1299] = (((-s.v[435]) / s.v[1220]) < 0.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1298])) && s.b[1299]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1298])) && (!s.b[1299])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844, 0.0);}
        s.b[1300] = (p.p853 > 1000.0);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1300]) {s.store_scalar(1221, 1.0);}
        s.b[1301] = (s.v[1194] > ((-s.v[438]) * p.p853));s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });s.b[1302] = (p.p856 == 4.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) && s.b[1302]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) && (!s.b[1302])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && (!s.b[1301])) {s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1286])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1303] = (s.v[641] == 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1303]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1303])) {s.store_primal_scale(1196, 1186, s.v[382]);}
        s.b[1304] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1304]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {s.store_primal_sub_from_scalar(1198, s.v[388], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
    }
}
