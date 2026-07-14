#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if (s.v[65] > 0.0) {
            s.copy_ad(200, 65);
        } else {
            s.store_scalar(200, 0.0);
        }
        s.copy_ad(201, 66);
        if (s.v[67] > 0.0) {
            s.copy_ad(202, 67);
        } else {
            s.store_scalar(202, 0.0);
        }
        s.copy_ad(203, 68);
        if (s.v[69] > 0.0) {
            s.copy_ad(204, 69);
        } else {
            s.store_scalar(204, 0.0);
        }
        s.copy_ad(205, 70);
        if (s.v[71] > 0.0) {
            s.copy_ad(206, 71);
        } else {
            s.store_scalar(206, 0.0);
        }
        s.copy_ad(207, 72);
        if (s.v[73] > 0.0) {
            s.copy_ad(208, 73);
        } else {
            s.store_scalar(208, 0.0);
        }
        s.copy_ad(209, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(210, 75);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 76);s.copy_ad(212, 77);
        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }
        s.copy_ad(214, 79);
        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(215, 80);
            } else {
                s.store_scalar(215, 1.0);
            }
        } else {
            s.store_scalar(215, (-0.5));
        }
        if (s.v[81] > (-0.5)) {
            s.copy_ad(216, 81);
        } else {
            s.store_scalar(216, (-0.5));
        }
        if (s.v[82] > 0.0) {
            s.copy_ad(217, 82);
        } else {
            s.store_scalar(217, 0.0);
        }
        s.copy_ad(218, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(219, 84);
            } else {
                s.store_scalar(219, 1.0);
            }
        } else {
            s.store_scalar(219, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(220, 85);
        } else {
            s.store_scalar(220, (-0.5));
        }
        if (s.v[86] > 0.01) {
            s.copy_ad(221, 86);
        } else {
            s.store_scalar(221, 0.01);
        }
        if (s.v[87] > 2.0) {
            s.copy_ad(222, 87);
        } else {
            s.store_scalar(222, 2.0);
        }
        if (s.v[88] > 0.0) {
            s.copy_ad(223, 88);
        } else {
            s.store_scalar(223, 0.0);
        }
        if (s.v[89] > 0.0) {
            s.copy_ad(224, 89);
        } else {
            s.store_scalar(224, 0.0);
        }
        if (s.v[90] > 0.0) {
            s.copy_ad(225, 90);
        } else {
            s.store_scalar(225, 0.0);
        }
        s.copy_ad(226, 91);
        if (s.v[92] > 0.0) {
            s.copy_ad(227, 92);
        } else {
            s.store_scalar(227, 0.0);
        }
        s.copy_ad(228, 93);s.copy_ad(229, 94);
        if (s.v[95] > 0.0) {
            s.copy_ad(230, 95);
        } else {
            s.store_scalar(230, 0.0);
        }
        if (s.v[96] > 0.0) {
            s.copy_ad(231, 96);
        } else {
            s.store_scalar(231, 0.0);
        }
        if (s.v[97] > 1e-12) {
            s.copy_ad(232, 97);
        } else {
            s.store_scalar(232, 1e-12);
        }
        s.copy_ad(233, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(234, 99);
        } else {
            s.store_scalar(234, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(235, 100);
        } else {
            s.store_scalar(235, 0.0);
        }
        if (s.v[101] > 0.0) {
            s.copy_ad(236, 101);
        } else {
            s.store_scalar(236, 0.0);
        }
        s.copy_ad(237, 102);s.copy_ad(238, 103);s.copy_ad(239, 104);s.copy_ad(240, 105);s.copy_ad(241, 106);s.copy_ad(242, 107);s.copy_ad(243, 108);s.copy_ad(244, 109);
        if (s.v[110] > 0.0) {
            s.copy_ad(245, 110);
        } else {
            s.store_scalar(245, 0.0);
        }
        if (s.v[111] > 0.0) {
            s.copy_ad(246, 111);
        } else {
            s.store_scalar(246, 0.0);
        }
        s.copy_ad(247, 112);s.copy_ad(248, 113);s.copy_ad(249, 114);s.copy_ad(250, 115);s.copy_ad(251, 116);s.copy_ad(252, 117);
        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }
        s.copy_ad(254, 119);
        if (s.v[120] > 0.0) {
            s.copy_ad(255, 120);
        } else {
            s.store_scalar(255, 0.0);
        }
        if (s.v[121] > 0.0) {
            s.copy_ad(256, 121);
        } else {
            s.store_scalar(256, 0.0);
        }
        if (s.v[122] > 2.0) {
            s.copy_ad(257, 122);
        } else {
            s.store_scalar(257, 2.0);
        }
        s.copy_ad(258, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(259, 124);
        } else {
            s.store_scalar(259, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[125] > 0.0) {
            s.copy_ad(260, 125);
        } else {
            s.store_scalar(260, 0.0);
        }
        if (s.v[126] > 0.0) {
            s.copy_ad(261, 126);
        } else {
            s.store_scalar(261, 0.0);
        }
        s.copy_ad(262, 127);s.copy_ad(263, 128);s.copy_ad(264, 129);
        if (s.v[130] > 0.0) {
            s.copy_ad(265, 130);
        } else {
            s.store_scalar(265, 0.0);
        }
        if (s.v[131] > 0.0) {
            s.copy_ad(266, 131);
        } else {
            s.store_scalar(266, 0.0);
        }
        if (s.v[132] > 0.0) {
            s.copy_ad(267, 132);
        } else {
            s.store_scalar(267, 0.0);
        }
        s.copy_ad(268, 133);s.copy_ad(269, 134);s.copy_ad(270, 135);s.copy_ad(271, 136);
        if (s.v[137] > 0.0) {
            s.copy_ad(272, 137);
        } else {
            s.store_scalar(272, 0.0);
        }
        if (s.v[138] > 0.0) {
            s.copy_ad(273, 138);
        } else {
            s.store_scalar(273, 0.0);
        }
        s.copy_ad(274, 139);
        if (s.v[140] > 0.0) {
            s.copy_ad(275, 140);
        } else {
            s.store_scalar(275, 0.0);
        }
        s.copy_ad(280, 145);s.copy_ad(281, 146);s.copy_ad(282, 147);
        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(283, 148);
            } else {
                s.store_scalar(283, 1e26);
            }
        } else {
            s.store_scalar(283, 1e20);
        }
        if (s.v[149] > 0.0) {
            s.copy_ad(284, 149);
        } else {
            s.store_scalar(284, 0.0);
        }
        if (s.v[150] > 0.0) {
            s.copy_ad(285, 150);
        } else {
            s.store_scalar(285, 0.0);
        }
        s.copy_ad(286, 151);
        if (s.v[152] > 0.0) {
            s.copy_ad(287, 152);
        } else {
            s.store_scalar(287, 0.0);
        }
        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(288, 153);
            } else {
                s.store_scalar(288, 1.0);
            }
        } else {
            s.store_scalar(288, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(289, 154);
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(292, 157);
            } else {
                s.store_scalar(292, 1.0);
            }
        } else {
            s.store_scalar(292, 0.0);
        }
        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }
        if (s.v[163] > 0.0) {
            s.copy_ad(298, 163);
        } else {
            s.store_scalar(298, 0.0);
        }
        s.copy_ad(299, 166);s.copy_ad(300, 167);s.copy_ad(301, 169);s.copy_ad(302, 170);s.copy_ad(303, 171);s.copy_ad(304, 168);
        if (s.v[172] > 0.0001) {
            s.copy_ad(305, 172);
        } else {
            s.store_scalar(305, 0.0001);
        }
        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
        }
        s.copy_ad(307, 174);
        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_primal_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }
        s.store_scalar(16, p.p16);s.store_scalar(17, p.p15);s.store_scalar(18, p.p18);s.store_scalar(19, p.p17);s.b[1130] = (p.p44 == 0.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.copy_ad(191, 190);s.copy_ad(193, 192);s.copy_ad(246, 245);s.copy_ad(248, 247);s.copy_ad(250, 249);s.copy_ad(252, 251);s.copy_ad(236, 235);s.copy_ad(242, 240);s.copy_ad(243, 241);s.copy_ad(261, 260);s.copy_ad(263, 262);s.copy_ad(267, 266);s.copy_ad(273, 272);}
        s.store_primal_scale(757, 180, 8.8541878176e-12);s.store_primal_div(758, 757, 179);s.store_primal_square(759, 179);s.store_primal_scale(760, 758, 6.241449993689894e18);s.store_primal_mul(761, 255, 181);
        if (s.v[761] > 1e20) {
            if (s.v[761] < 1e26) {
            } else {
                s.store_scalar(761, 1e26);
            }
        } else {
            s.store_scalar(761, 1e20);
        }
        s.store_scalar(762, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1131] = (p.p51 > 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_primal_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));}
        s.b[1132] = (s.v[0] == (-1.0));s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1132]) {s.store_primal_scale(762, 762, (7.448711 / 5.951993));}
        s.store_primal_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));s.store_primal_scale(764, 212, 0.5);s.store_scalar(765, 0.5);s.b[1133] = (s.v[0] == (-1.0));s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if s.b[1133] {s.store_primal_scale(764, 212, 0.3333333333333333);s.store_scalar(765, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(766, 997, (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(767, 997, (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(768, 1.0, 226);s.store_primal_div(769, 757, 190);s.store_primal_div(770, 757, 191);s.store_primal_div_mixed_ai(771, A::sqrt_scaled_input(s.ad_value(192), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 769);s.store_primal_div_mixed_ai(772, A::sqrt_scaled_input(s.ad_value(193), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 770);s.store_primal_square(773, 771);s.store_primal_square(774, 772);s.store_primal_offset_div_ad(775, A::ln(A::offset(A::exp_scaled_input(s.ad_value(264), (0.005 * s.v[356])), (-1.0))), s.ad_value(264), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(776, A::ln_scaled_input(s.ad_value(771), 0.5), 775);s.store_primal_add_mixed_ai(777, A::ln_scaled_input(s.ad_value(772), 0.5), 775);s.store_primal_div_from_scalar(809, 1.0, 771);s.store_primal_offset_scaled(810, 771, 3.1, 8.5);s.store_primal_square(778, 810);s.store_primal_scale(811, 810, 0.5);s.b[1134] = (s.v[809] < 0.06);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if s.b[1134] {s.store_primal_scale(779, 809, 64.0);}
        s.b[1135] = (s.v[809] <= 0.45);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if ((!s.b[1134]) && s.b[1135]) {s.store_primal_offset_scaled(779, 809, 22.0, 3.0);}
        s.b[1136] = (s.v[809] <= 1.6);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });
        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {s.store_primal_offset_scaled(779, 809, (-7.2), 15.5);}
        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {s.copy_ad(779, 771);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(780, 811, 1.0, 773, 0.5, 771, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), (-1.0));s.store_primal_div_from_scalar(809, 1.0, 772);s.store_primal_offset_scaled(810, 772, 3.1, 8.5);s.store_primal_square(781, 810);s.store_primal_scale(811, 810, 0.5);s.b[1137] = (s.v[809] < 0.06);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if s.b[1137] {s.store_primal_scale(782, 809, 64.0);}
        s.b[1138] = (s.v[809] <= 0.45);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if ((!s.b[1137]) && s.b[1138]) {s.store_primal_offset_scaled(782, 809, 22.0, 3.0);}
        s.b[1139] = (s.v[809] <= 1.6);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {s.store_primal_offset_scaled(782, 809, (-7.2), 15.5);}
        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {s.copy_ad(782, 772);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs_product_mixed_iiia(783, 811, 1.0, 774, 0.5, 772, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), (-1.0));s.store_primal_div_from_scalar(784, 1.0, 244);s.store_primal_scaled_sqrt_scaled_input(785, 244, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));s.store_primal_mul(786, 785, 179);s.store_primal_mul(787, 785, 190);s.store_primal_mul(788, 785, 191);s.store_scalar(789, 0.0);s.b[1140] = (s.v[239] < 0.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_primal_div_scaled_inputs_indices(789, 238, (-0.495), 239, 1.0);}
        s.store_scalar(790, 0.0);s.b[1141] = (s.v[241] < 0.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_primal_div_scaled_inputs_indices(790, 240, (-0.495), 241, 1.0);}
        s.b[1142] = (s.v[243] < 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_primal_div_scaled_inputs_indices(791, 242, (-0.495), 243, 1.0);}
        s.store_primal_pow_from_scalar_ad(792, s.v[353], s.ad_value(237));s.store_primal_mul(234, 234, 792);s.store_primal_mul(235, 235, 792);s.store_primal_mul(236, 236, 792);s.store_primal_div_scaled_inputs_square_rhs(793, 245, 4e-18, 190, 1.0);s.store_primal_div_scaled_inputs_square_rhs(794, 246, 4e-18, 191, 1.0);
        if ((1.0 + (s.v[249] * s.v[354])) > 0.0) {
            s.store_primal_offset_scaled(785, 249, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }
        s.store_primal_mul(711, 247, 785);s.store_primal_scaled_mul(795, 711, 190, 500000000.0);
        if ((1.0 + (s.v[250] * s.v[354])) > 0.0) {
            s.store_primal_offset_scaled(785, 250, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }
        s.store_primal_mul(712, 248, 785);s.store_primal_scaled_mul(796, 712, 191, 500000000.0);s.store_scalar(797, 0.0);s.b[1143] = (s.v[270] > 1e-10);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_primal_div_from_scalar(797, 0.75, 270);}
        s.store_primal_square(798, 271);s.store_primal_mul_mixed_ia(716, 305, A::pow_from_scalar(s.v[353], s.ad_value(307)));s.store_primal_scale(799, 275, (9.1093826e-31 * 1000000000.0));s.b[1144] = (s.v[298] > 0.0);s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_primal_div_from_scalar(800, 1.0, 298);}
        if (!s.b[1144]) {s.store_scalar(800, 0.0);}
        s.b[1145] = (s.v[299] > 0.0);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_primal_div_from_scalar(801, 1.0, 299);}
        if (!s.b[1145]) {s.store_scalar(801, 0.0);}
        s.b[1146] = (s.v[300] > 0.0);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_primal_div_from_scalar(802, 1.0, 300);}
        if (!s.b[1146]) {s.store_scalar(802, 0.0);}
        s.b[1147] = (s.v[301] > 0.0);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_primal_div_from_scalar(803, 1.0, 301);}
        if (!s.b[1147]) {s.store_scalar(803, 0.0);}
        s.b[1148] = (s.v[302] > 0.0);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_primal_div_from_scalar(804, 1.0, 302);}
        if (!s.b[1148]) {s.store_scalar(804, 0.0);}
        s.b[1149] = (s.v[303] > 0.0);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_primal_div_from_scalar(805, 1.0, 303);}
        if (!s.b[1149]) {s.store_scalar(805, 0.0);}
        s.b[1150] = (s.v[304] > 0.0);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if s.b[1150] {s.store_primal_div_from_scalar(806, 1.0, 304);}
        if (!s.b[1150]) {s.store_scalar(806, 0.0);}
        s.store_primal_scale(20, 2, s.v[647]);s.store_primal_scale(21, 2, s.v[648]);s.store_primal_scale(22, 2, s.v[649]);s.store_primal_scale(23, 2, s.v[674]);s.store_primal_scale(24, 2, s.v[675]);s.store_primal_scale(25, 2, s.v[676]);s.store_scalar(26, 0.0);s.b[1151] = (p.p43 == 3.0);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if s.b[1151] {s.store_scalar(26, 1.0);}
        s.copy_ad(27, 313);s.b[1152] = (p.p39 == 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));}
        s.b[1153] = ((p.p43 == 2.0) || (p.p43 == 3.0));s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_primal_scale(20, 2, s.v[650]);s.store_primal_add_scaled_product_indices(21, 2, s.v[651], 26, 27, (-1.0));s.copy_ad(22, 27);s.store_primal_scale(23, 2, s.v[677]);s.store_primal_add_scaled_product_indices(24, 2, s.v[678], 26, 27, (-1.0));s.copy_ad(25, 27);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1154] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {
            if (s.v[20] > 0.0) {
                s.copy_ad(647, 20);
            } else {
                s.store_scalar(647, 0.0);
            }
        }
        if s.b[1154] {
            if (s.v[21] > 0.0) {
                s.copy_ad(648, 21);
            } else {
                s.store_scalar(648, 0.0);
            }
        }
        if s.b[1154] {
            if (s.v[22] > 0.0) {
                s.copy_ad(649, 22);
            } else {
                s.store_scalar(649, 0.0);
            }
        }
        if s.b[1154] {
            if (s.v[23] > 0.0) {
                s.copy_ad(674, 23);
            } else {
                s.store_scalar(674, 0.0);
            }
        }
        if s.b[1154] {
            if (s.v[24] > 0.0) {
                s.copy_ad(675, 24);
            } else {
                s.store_scalar(675, 0.0);
            }
        }
        if s.b[1154] {
            if (s.v[25] > 0.0) {
                s.copy_ad(676, 25);
            } else {
                s.store_scalar(676, 0.0);
            }
        }
        if (!s.b[1154]) {s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(649, 0.0);s.store_scalar(674, 0.0);s.store_scalar(675, 0.0);s.store_scalar(676, 0.0);}
        s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(660, 0.0);s.store_scalar(687, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(668, 0.0);s.store_scalar(695, 0.0);s.store_scalar(669, 1.0);s.store_scalar(696, 1.0);s.store_scalar(670, 0.0);s.store_scalar(697, 0.0);s.store_scalar(671, 1.0);s.store_scalar(698, 1.0);s.store_scalar(672, 0.0);s.store_scalar(699, 0.0);s.store_scalar(673, 1.0);s.store_scalar(700, 1.0);s.store_scalar(667, 0.0);s.store_scalar(694, 0.0);s.store_scalar(661, 0.0);s.store_scalar(688, 0.0);s.store_scalar(662, 0.0);s.store_scalar(689, 0.0);s.store_scalar(663, 0.0);s.store_scalar(690, 0.0);s.store_scalar(664, 0.0);s.store_scalar(691, 0.0);s.store_scalar(665, 0.0);s.store_scalar(692, 0.0);s.store_scalar(666, 0.0);s.store_scalar(693, 0.0);s.store_scalar(652, 1.0);s.store_scalar(679, 1.0);s.store_scalar(653, 1.0);s.store_scalar(680, 1.0);s.store_scalar(654, 1.0);s.store_scalar(681, 1.0);s.store_scalar(492, 0.0);s.store_scalar(493, 0.0);s.store_scalar(481, 0.0);s.store_scalar(482, 0.0);s.store_scalar(483, 0.0);s.store_scalar(484, 0.0);s.store_scalar(485, 0.0);s.store_scalar(494, 0.0);s.store_scalar(495, 0.0);s.store_scalar(496, 0.0);s.store_scalar(502, 0.0);s.store_scalar(491, 0.0);s.b[1155] = (p.p43 > 0.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });s.b[1156] = ((s.v[388] * s.v[647]) > 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1156]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1156])) {s.store_scalar(455, 100000000.0);}
        s.b[1157] = ((s.v[389] * s.v[648]) > 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1155] && s.b[1157]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1157])) {s.store_scalar(456, 100000000.0);}
        s.b[1158] = ((s.v[390] * s.v[649]) > 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1158]) {s.store_primal_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1158])) {s.store_scalar(457, 100000000.0);}
        if s.b[1155] {s.store_min3(655, 455, 456, 457);}
        s.b[1159] = ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1159]) {s.store_primal_exp_scaled_input(656, 655, s.v[372]);}
        s.b[1160] = ((s.v[655] * s.v[372]) < 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if ((s.b[1155] && (!s.b[1159])) && s.b[1160]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(656, 1e-100, (-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1155] && (!s.b[1159])) && (!s.b[1160])) {s.store_primal_scaled_offset_ad(656, A::mul_offset_rhs(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(655), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1155] {s.store_scalar(397, s.v[394]);s.store_scalar(398, s.v[395]);s.store_scalar(399, s.v[396]);s.store_scalar(400, p.p848);s.store_scalar(401, p.p849);s.store_scalar(402, p.p850);s.store_scalar(403, p.p845);s.store_scalar(404, p.p846);s.store_scalar(405, p.p847);}
        s.b[1161] = (s.v[647] == 0.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1161]) {s.store_scalar(397, (s.v[395] + s.v[396]));s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));s.store_scalar(403, (p.p846 + p.p847));}
        s.b[1162] = (s.v[648] == 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1162]) {s.store_scalar(398, (s.v[394] + s.v[396]));s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));s.store_scalar(404, (p.p845 + p.p847));}
        s.b[1163] = (s.v[649] == 0.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1163]) {s.store_scalar(399, (s.v[394] + s.v[395]));s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));s.store_scalar(405, (p.p845 + p.p846));}
        if s.b[1155] {s.store_min3(657, 397, 398, 399);s.store_primal_scale(658, 657, 0.1);s.store_max3(378, 400, 401, 402);s.store_primal_mul_scale_offset_mixed_ia(659, 657, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))), -1.0, 1.0);s.store_primal_offset_min_ad(660, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));}
        s.b[1164] = ((s.v[564] * s.v[674]) > 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1164]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1164])) {s.store_scalar(455, 100000000.0);}
        s.b[1165] = ((s.v[565] * s.v[675]) > 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1165]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1165])) {s.store_scalar(456, 100000000.0);}
        s.b[1166] = ((s.v[566] * s.v[676]) > 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1166]) {s.store_primal_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0), s.v[371]);}
        if (s.b[1155] && (!s.b[1166])) {s.store_scalar(457, 100000000.0);}
        if s.b[1155] {s.store_min3(682, 455, 456, 457);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        s.b[1167] = ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1167]) {s.store_primal_exp_scaled_input(683, 682, s.v[372]);}
        s.b[1168] = ((s.v[682] * s.v[372]) < 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if ((s.b[1155] && (!s.b[1167])) && s.b[1168]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(683, 1e-100, (-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1155] && (!s.b[1167])) && (!s.b[1168])) {s.store_primal_scaled_offset_ad(683, A::mul_offset_rhs(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(682), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1155] {s.copy_ad(397, 570);s.copy_ad(398, 571);s.copy_ad(399, 572);s.copy_ad(400, 512);s.copy_ad(401, 513);s.copy_ad(402, 514);s.copy_ad(403, 509);s.copy_ad(404, 510);s.copy_ad(405, 511);}
        s.b[1169] = (s.v[674] == 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1169]) {s.store_primal_add(397, 571, 572);s.store_primal_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);s.store_primal_add(403, 510, 511);}
        s.b[1170] = (s.v[675] == 0.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1170]) {s.store_primal_add(398, 570, 572);s.store_primal_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);s.store_primal_add(404, 509, 511);}
        s.b[1171] = (s.v[676] == 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1171]) {s.store_primal_add(399, 570, 571);s.store_primal_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);s.store_primal_add(405, 509, 510);}
        if s.b[1155] {s.store_min3(684, 397, 398, 399);s.store_primal_scale(685, 684, 0.1);s.store_max3(378, 400, 401, 402);s.store_primal_mul_scale_offset_mixed_ia(686, 684, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))), -1.0, 1.0);s.store_primal_offset_min_ad(687, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));}
        s.b[1172] = (s.v[475] == 1.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1155] && s.b[1172]) {s.store_scalar(1173, 0.0);s.store_scalar(1174, 0.0);s.store_scalar(1175, 0.0);s.store_scalar(1182, 0.0);s.store_scalar(1184, 0.0);s.store_scalar(1185, 0.0);s.store_scalar(1186, 0.0);s.store_scalar(1187, 0.0);s.store_scalar(1188, 0.0);s.store_scalar(1189, 0.0);s.store_scalar(1190, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1192, 0.0);s.store_scalar(1193, 0.0);s.store_scalar(1194, 0.0);s.store_scalar(1195, 0.0);s.store_scalar(1196, 0.0);s.store_scalar(1197, 0.0);s.store_scalar(1198, 0.0);s.store_scalar(1199, 0.0);s.store_scalar(1200, 0.0);s.store_scalar(1201, 0.0);s.store_scalar(1202, 0.0);s.store_scalar(1203, 0.0);s.store_scalar(1204, 0.0);s.store_scalar(1205, 0.0);s.store_scalar(1206, 0.0);s.store_scalar(1207, 0.0);s.store_scalar(1208, 0.0);s.store_scalar(1209, 0.0);s.store_scalar(1210, 0.0);s.store_scalar(1211, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1155] && s.b[1172]) {s.store_scalar(1212, 0.0);s.store_scalar(1213, 0.0);s.store_scalar(1214, 0.0);s.store_scalar(1215, 0.0);s.store_scalar(1216, 0.0);s.store_scalar(1217, 0.0);s.store_scalar(499, 0.4);s.store_scalar(500, 0.65);s.store_scalar(501, 0.8);s.store_primal_scale(486, 499, (-p.p945));s.store_primal_scale(487, 500, (-p.p945));s.store_primal_scale(488, 501, (-p.p945));s.store_scalar(489, 0.1);s.store_scalar(490, 0.2);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1221] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });s.b[1222] = (s.v[486] < s.v[655]);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = (((((-0.5) * (s.v[486] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && s.b[1223]) {s.store_primal_exp_scaled_input(1184, 486, (s.v[372] * (-0.5)));}
        s.b[1224] = (((-0.5) * (s.v[486] * s.v[372])) < 0.0);s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && (!s.b[1223])) && s.b[1224]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && (!s.b[1223])) && (!s.b[1224])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && (!s.b[1222])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 656, A::sub_scaled_inputs(s.ad_value(486), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1221]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1225] = (s.v[486] > 0.0);s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1225]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && (!s.b[1225])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 486);}
        if ((s.b[1155] && s.b[1172]) && s.b[1221]) {s.store_primal_sub(1187, 657, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 486, 0.5, 1187, 0.5, 486, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 486, 0.5, 660, 0.5, 486, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 486, A::sqrt_square_offset(s.ad_value(486), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1226] = (s.v[647] == 0.0);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1226]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1226])) {s.store_primal_scale(1192, 1182, s.v[388]);}
        s.b[1227] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1227]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {s.store_primal_sub_from_scalar(1194, s.v[394], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1228] = (p.p848 == 0.5);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && s.b[1228]) {s.store_scalar(1196, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && (!s.b[1228])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1229] = (p.p848 == 0.5);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && s.b[1229]) {s.store_sqrt_scaled_input(1191, 1194, s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && (!s.b[1229])) {s.store_powf_scaled_input(1191, 1194, s.v[430], p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {s.store_scale(1198, 1191, s.v[424]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1193, 1199, 1197, p.p857);}
        s.b[1230] = (p.p862 == 0.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1230]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[409] * s.v[439]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1231] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1231]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1231])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[436]), 1202, 1205, s.v[436], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1232] = (s.v[1212] > 0.0);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1232]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1232])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1233] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1233]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1233])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1234] = (s.v[1212] > 0.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1234]) {s.copy_ad(1213, 1175);}
        s.b[1235] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) && s.b[1235]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1235])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[436] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);}
        s.b[1236] = (p.p868 == 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1236]) {s.store_scalar(1215, 0.0);}
        s.b[1237] = (p.p848 == 0.5);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && s.b[1237]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1237])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 1191, 1.0);}
        s.b[1238] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && s.b[1238]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));}
        s.b[1239] = (((-s.v[442]) / s.v[1216]) < 0.0);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1238])) && s.b[1239]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 442, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1238])) && (!s.b[1239])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 442, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868, 0.0);}
        s.b[1240] = (p.p877 > 1000.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1240]) {s.store_scalar(1217, 1.0);}
        s.b[1241] = (s.v[1190] > ((-s.v[445]) * p.p877));s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });s.b[1242] = (p.p880 == 4.0);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) && s.b[1242]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) && (!s.b[1242])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && (!s.b[1241])) {s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1226])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1243] = (s.v[648] == 0.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1243]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1243])) {s.store_primal_scale(1192, 1182, s.v[389]);}
        s.b[1244] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1244]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {s.store_primal_sub_from_scalar(1194, s.v[395], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1245] = (p.p849 == 0.5);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && s.b[1245]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && (!s.b[1245])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1246] = (p.p849 == 0.5);s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && s.b[1246]) {s.store_sqrt_scaled_input(1191, 1194, s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && (!s.b[1246])) {s.store_powf_scaled_input(1191, 1194, s.v[431], p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {s.store_scale(1198, 1191, s.v[425]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1193, 1199, 1197, p.p858);}
        s.b[1247] = (p.p863 == 0.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1247]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[410] * s.v[440]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1248] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1248]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1248])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[437]), 1202, 1205, s.v[437], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1249] = (s.v[1212] > 0.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1249]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1249])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1250] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1250]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1250])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1251] = (s.v[1212] > 0.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1251]) {s.copy_ad(1213, 1175);}
        s.b[1252] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) && s.b[1252]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1252])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[437] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);}
        s.b[1253] = (p.p869 == 0.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1253]) {s.store_scalar(1215, 0.0);}
        s.b[1254] = (p.p849 == 0.5);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && s.b[1254]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1254])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 1191, 1.0);}
        s.b[1255] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && s.b[1255]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));}
        s.b[1256] = (((-s.v[443]) / s.v[1216]) < 0.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1255])) && s.b[1256]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 443, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1255])) && (!s.b[1256])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 443, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869, 0.0);}
        s.b[1257] = (p.p878 > 1000.0);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1257]) {s.store_scalar(1217, 1.0);}
        s.b[1258] = (s.v[1190] > ((-s.v[445]) * p.p878));s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });s.b[1259] = (p.p881 == 4.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) && (!s.b[1259])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && (!s.b[1258])) {s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1243])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1260] = (s.v[649] == 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1260]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1260])) {s.store_primal_scale(1192, 1182, s.v[390]);}
        s.b[1261] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1261]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {s.store_primal_sub_from_scalar(1194, s.v[396], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1262] = (p.p850 == 0.5);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && s.b[1262]) {s.store_scalar(1196, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && (!s.b[1262])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1263] = (p.p850 == 0.5);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && s.b[1263]) {s.store_sqrt_scaled_input(1191, 1194, s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && (!s.b[1263])) {s.store_powf_scaled_input(1191, 1194, s.v[432], p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {s.store_scale(1198, 1191, s.v[426]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(1193, 1199, 1197, p.p859);}
        s.b[1264] = (p.p864 == 0.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1264]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[411] * s.v[441]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1265] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1265]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1265])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[438]), 1202, 1205, s.v[438], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1266] = (s.v[1212] > 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1266]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1266])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1267] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1267]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1267])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1268] = (s.v[1212] > 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1268]) {s.copy_ad(1213, 1175);}
        s.b[1269] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) && s.b[1269]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1269])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[438] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);}
        s.b[1270] = (p.p870 == 0.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1270]) {s.store_scalar(1215, 0.0);}
        s.b[1271] = (p.p850 == 0.5);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && s.b[1271]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1271])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 1191, 1.0);}
        s.b[1272] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && s.b[1272]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));}
        s.b[1273] = (((-s.v[444]) / s.v[1216]) < 0.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1272])) && s.b[1273]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 444, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1272])) && (!s.b[1273])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 444, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870, 0.0);}
        s.b[1274] = (p.p879 > 1000.0);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1274]) {s.store_scalar(1217, 1.0);}
        s.b[1275] = (s.v[1190] > ((-s.v[445]) * p.p879));s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });s.b[1276] = (p.p882 == 4.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) && s.b[1276]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) && (!s.b[1276])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && (!s.b[1275])) {s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1260])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(476, 647, 1218, 1.0, 648, 1219, 1.0, 649, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1277] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });s.b[1278] = (s.v[487] < s.v[655]);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });s.b[1279] = (((((-0.5) * (s.v[487] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && s.b[1279]) {s.store_primal_exp_scaled_input(1184, 487, (s.v[372] * (-0.5)));}
        s.b[1280] = (((-0.5) * (s.v[487] * s.v[372])) < 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && (!s.b[1279])) && s.b[1280]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && (!s.b[1279])) && (!s.b[1280])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && (!s.b[1278])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 656, A::sub_scaled_inputs(s.ad_value(487), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1277]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1281] = (s.v[487] > 0.0);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1281]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && (!s.b[1281])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 487);}
        if ((s.b[1155] && s.b[1172]) && s.b[1277]) {s.store_primal_sub(1187, 657, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 487, 0.5, 1187, 0.5, 487, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 487, 0.5, 660, 0.5, 487, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 487, A::sqrt_square_offset(s.ad_value(487), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1282] = (s.v[647] == 0.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1282]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1282])) {s.store_primal_scale(1192, 1182, s.v[388]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1283] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1283]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {s.store_primal_sub_from_scalar(1194, s.v[394], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1284] = (p.p848 == 0.5);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && s.b[1284]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && (!s.b[1284])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1285] = (p.p848 == 0.5);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && s.b[1285]) {s.store_sqrt_scaled_input(1191, 1194, s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && (!s.b[1285])) {s.store_powf_scaled_input(1191, 1194, s.v[430], p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {s.store_scale(1198, 1191, s.v[424]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1193, 1199, 1197, p.p857);}
        s.b[1286] = (p.p862 == 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1286]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[409] * s.v[439]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1287] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1287]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1287])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[436]), 1202, 1205, s.v[436], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1288] = (s.v[1212] > 0.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1288]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1288])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1289] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1289]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1289])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1290] = (s.v[1212] > 0.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1290]) {s.copy_ad(1213, 1175);}
        s.b[1291] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) && s.b[1291]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1291])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[436] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);}
        s.b[1292] = (p.p868 == 0.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1292]) {s.store_scalar(1215, 0.0);}
        s.b[1293] = (p.p848 == 0.5);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && s.b[1293]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1293])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 1191, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1294] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && s.b[1294]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));}
        s.b[1295] = (((-s.v[442]) / s.v[1216]) < 0.0);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1294])) && s.b[1295]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 442, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1294])) && (!s.b[1295])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 442, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868, 0.0);}
        s.b[1296] = (p.p877 > 1000.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1296]) {s.store_scalar(1217, 1.0);}
        s.b[1297] = (s.v[1190] > ((-s.v[445]) * p.p877));s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });s.b[1298] = (p.p880 == 4.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) && s.b[1298]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) && (!s.b[1298])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && (!s.b[1297])) {s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1282])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1299] = (s.v[648] == 0.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1299]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1299])) {s.store_primal_scale(1192, 1182, s.v[389]);}
        s.b[1300] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1300]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {s.store_primal_sub_from_scalar(1194, s.v[395], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1301] = (p.p849 == 0.5);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && s.b[1301]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && (!s.b[1301])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1302] = (p.p849 == 0.5);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && s.b[1302]) {s.store_sqrt_scaled_input(1191, 1194, s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && (!s.b[1302])) {s.store_powf_scaled_input(1191, 1194, s.v[431], p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {s.store_scale(1198, 1191, s.v[425]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1193, 1199, 1197, p.p858);}
        s.b[1303] = (p.p863 == 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1303]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[410] * s.v[440]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1304] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1304]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1304])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[437]), 1202, 1205, s.v[437], 1201, 1206, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1305] = (s.v[1212] > 0.0);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1305]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1305])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1306] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1306]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1306])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1307] = (s.v[1212] > 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1307]) {s.copy_ad(1213, 1175);}
        s.b[1308] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) && s.b[1308]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1308])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[437] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);}
        s.b[1309] = (p.p869 == 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1309]) {s.store_scalar(1215, 0.0);}
        s.b[1310] = (p.p849 == 0.5);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && s.b[1310]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1310])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 1191, 1.0);}
        s.b[1311] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && s.b[1311]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));}
        s.b[1312] = (((-s.v[443]) / s.v[1216]) < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1311])) && s.b[1312]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 443, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1311])) && (!s.b[1312])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 443, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869, 0.0);}
        s.b[1313] = (p.p878 > 1000.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1313]) {s.store_scalar(1217, 1.0);}
        s.b[1314] = (s.v[1190] > ((-s.v[445]) * p.p878));s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });s.b[1315] = (p.p881 == 4.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) && s.b[1315]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) && (!s.b[1315])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && (!s.b[1314])) {s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1299])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1316] = (s.v[649] == 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1316]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1316])) {s.store_primal_scale(1192, 1182, s.v[390]);}
        s.b[1317] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1317]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {s.store_primal_sub_from_scalar(1194, s.v[396], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
    }
}
