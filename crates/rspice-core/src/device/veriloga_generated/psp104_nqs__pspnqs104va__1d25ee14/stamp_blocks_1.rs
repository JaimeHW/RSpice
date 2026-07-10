#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }
        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }
        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }
        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }
        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }
        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }
        s.copy_ad(203, 70);
        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }
        s.copy_ad(205, 72);
        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }
        s.copy_ad(207, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }
        s.copy_ad(209, 76);
        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 78);
        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }
        s.copy_ad(213, 80);s.copy_ad(214, 81);
        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }
        s.copy_ad(216, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }
        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }
        s.copy_ad(220, 87);
        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }
        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }
        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }
        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }
        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }
        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }
        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }
        s.copy_ad(228, 95);
        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }
        s.copy_ad(230, 97);s.copy_ad(231, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }
        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }
        s.copy_ad(235, 102);
        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }
        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }
        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }
        s.copy_ad(239, 106);s.copy_ad(240, 107);s.copy_ad(241, 108);s.copy_ad(242, 109);s.copy_ad(243, 110);s.copy_ad(244, 111);s.copy_ad(245, 112);s.copy_ad(246, 113);
        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }
        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }
        s.copy_ad(249, 116);s.copy_ad(250, 117);s.copy_ad(251, 118);s.copy_ad(252, 119);s.copy_ad(253, 120);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.copy_ad(254, 121);
        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }
        s.copy_ad(256, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }
        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }
        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }
        s.copy_ad(260, 127);
        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }
        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }
        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }
        s.copy_ad(264, 131);s.copy_ad(265, 132);s.copy_ad(266, 133);
        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }
        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }
        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }
        s.copy_ad(270, 137);s.copy_ad(271, 138);s.copy_ad(272, 139);s.copy_ad(273, 140);
        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }
        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }
        s.copy_ad(276, 143);
        if (s.v[144] > 0.0) {
            s.copy_ad(277, 144);
        } else {
            s.store_scalar(277, 0.0);
        }
        s.copy_ad(282, 149);s.copy_ad(283, 150);s.copy_ad(284, 151);
        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }
        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }
        s.copy_ad(288, 155);
        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }
        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }
        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }
        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }
        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }
        if (s.v[167] > 0.0) {
            s.copy_ad(300, 167);
        } else {
            s.store_scalar(300, 0.0);
        }
        s.copy_ad(301, 170);s.copy_ad(302, 171);s.copy_ad(303, 173);s.copy_ad(304, 174);s.copy_ad(305, 175);s.copy_ad(306, 172);
        if ((p.p31 * s.v[5]) > 0.0) {
            s.store_primal_scale(19, 5, p.p31);
        } else {
            s.store_scalar(19, 0.0);
        }
        s.store_scalar(20, p.p16);s.store_scalar(21, p.p15);s.store_scalar(22, p.p18);s.store_scalar(23, p.p17);
        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }
        s.b[1142] = (p.p44 == 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.copy_ad(193, 192);s.copy_ad(195, 194);s.copy_ad(248, 247);s.copy_ad(250, 249);s.copy_ad(252, 251);s.copy_ad(254, 253);s.copy_ad(238, 237);s.copy_ad(244, 242);s.copy_ad(245, 243);s.copy_ad(263, 262);s.copy_ad(265, 264);s.copy_ad(269, 268);s.copy_ad(275, 274);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_scale(768, 182, 8.8541878176e-12);s.store_primal_div(769, 768, 181);s.store_primal_square(770, 181);s.store_primal_scale(771, 769, 6.241449993689894e18);s.store_primal_mul(772, 257, 183);
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }
        s.store_scalar(773, 0.0);s.b[1143] = (p.p52 > 0.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_primal_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));}
        s.b[1144] = (s.v[0] == (-1.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if (s.b[1143] && s.b[1144]) {s.store_primal_scale(773, 773, (7.448711 / 5.951993));}
        s.store_primal_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));s.store_primal_scale(775, 214, 0.5);s.store_scalar(776, 0.5);s.b[1145] = (s.v[0] == (-1.0));s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_primal_scale(775, 214, 0.3333333333333333);s.store_scalar(776, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(777, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(778, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(779, 1.0, 228);s.store_primal_div(780, 768, 192);s.store_primal_div(781, 768, 193);s.store_primal_div_mixed_ai(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);s.store_primal_div_mixed_ai(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);s.store_primal_square(784, 782);s.store_primal_square(785, 783);s.store_primal_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);s.store_primal_add_mixed_ai(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);s.store_primal_div_from_scalar(820, 1.0, 782);s.store_primal_offset_scaled(821, 782, 3.1, 8.5);s.store_primal_square(789, 821);s.store_primal_scale(822, 821, 0.5);s.b[1146] = (s.v[820] < 0.06);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_primal_scale(790, 820, 64.0);}
        s.b[1147] = (s.v[820] <= 0.45);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if ((!s.b[1146]) && s.b[1147]) {s.store_primal_offset_scaled(790, 820, 22.0, 3.0);}
        s.b[1148] = (s.v[820] <= 1.6);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {s.store_primal_offset_scaled(790, 820, (-7.2), 15.5);}
        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {s.copy_ad(790, 782);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));s.store_primal_div_from_scalar(820, 1.0, 783);s.store_primal_offset_scaled(821, 783, 3.1, 8.5);s.store_primal_square(792, 821);s.store_primal_scale(822, 821, 0.5);s.b[1149] = (s.v[820] < 0.06);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_primal_scale(793, 820, 64.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1150] = (s.v[820] <= 0.45);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if ((!s.b[1149]) && s.b[1150]) {s.store_primal_offset_scaled(793, 820, 22.0, 3.0);}
        s.b[1151] = (s.v[820] <= 1.6);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {s.store_primal_offset_scaled(793, 820, (-7.2), 15.5);}
        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {s.copy_ad(793, 783);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));s.store_primal_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));
        if (!(s.v[728] > 0.05)) {s.store_scalar(728, 0.05);}
        s.store_primal_div_mixed_ai(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_scalar(730, 0.0);s.store_scalar(731, 0.0);s.b[1152] = (s.v[188] > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_primal_div_from_scalar(732, 80000000.0, 770);}
        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }
        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }
        if s.b[1152] {s.store_primal_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));}
        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));s.b[1153] = (p.p52 > 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(728, 728, 735);s.store_primal_mul_scale_offset_mixed_ia(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_sqrt(736, 728);s.store_primal_scale(737, 728, 0.95);s.store_primal_scaled_mul(738, 728, 728, 0.0025);s.copy_ad(739, 738);s.store_primal_scaled_sqrt(740, 739, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));s.store_primal_scaled_offset(742, 728, s.v[362], 0.5);s.store_primal_sub_mixed_ai(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);s.store_primal_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);s.store_primal_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);
        if (!(s.v[745] > 0.05)) {s.store_scalar(745, 0.05);}
        s.store_primal_div_mixed_ai(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.b[1154] = (p.p52 > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(745, 745, 735);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1154] {s.store_primal_mul_scale_offset_mixed_ia(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_scale(747, 745, 0.95);s.store_primal_scaled_mul(748, 745, 745, 0.0025);s.copy_ad(749, 748);s.store_primal_scaled_sqrt(740, 749, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));s.store_primal_offset_add_scaled_product_mixed_iia(700, 177, 1.0, 178, A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);s.store_primal_exp_scaled_input(751, 180, s.v[360]);s.store_primal_mul(701, 189, 751);s.store_primal_scale(702, 190, 1.0 / (s.v[359]));s.store_primal_exp_scaled_input(752, 203, s.v[360]);s.store_primal_mul(703, 202, 752);s.store_primal_scaled_mul(716, 703, 769, s.v[20]);s.store_primal_mul_mixed_ia(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));s.store_primal_exp_scaled_input(753, 205, s.v[360]);s.store_primal_mul(704, 204, 753);s.store_primal_mul_mixed_ia(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));s.store_primal_exp_scaled_input(754, 209, s.v[360]);s.store_primal_mul(706, 208, 754);s.store_primal_exp_scaled_input(755, 213, s.v[360]);s.store_primal_mul(708, 212, 755);s.store_primal_exp_scaled_input(756, 216, s.v[360]);s.store_primal_mul(709, 215, 756);s.store_primal_scaled_mul(757, 716, 709, 2.0);s.store_primal_exp_scaled_input(758, 220, s.v[360]);s.store_primal_mul(720, 219, 758);s.store_primal_mul(721, 258, 758);s.store_primal_mul_mixed_ia(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));s.store_primal_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_primal_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);s.store_primal_exp_scaled_input(759, 288, s.v[360]);s.store_primal_mul(714, 287, 759);s.store_primal_scaled_mul(717, 714, 769, s.v[22]);s.store_primal_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);s.store_primal_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);}
        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }
        if s.b[1155] {s.store_primal_div_mixed_ai(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_primal_square(724, 761);s.store_primal_ln(725, 724);s.store_primal_scale(762, 760, 0.95);s.store_primal_scaled_mul(763, 760, 760, 0.0025);s.copy_ad(764, 763);s.store_primal_scaled_sqrt(765, 764, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));}
        if (!s.b[1155]) {s.store_scalar(713, 0.0);s.store_scalar(759, 1.0);s.store_scalar(714, 0.0);s.store_scalar(717, 0.0);s.store_scalar(723, s.v[715]);s.store_scalar(760, 0.0);s.store_scalar(761, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 0.0);s.store_scalar(762, 0.0);s.store_scalar(763, 0.0);s.store_scalar(764, 0.0);s.store_scalar(765, 0.0);s.store_scalar(766, 0.0);}
        s.store_primal_div_from_scalar(795, 1.0, 246);s.store_primal_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_mul(797, 796, 181);s.store_primal_mul(798, 796, 192);s.store_primal_mul(799, 796, 193);s.store_scalar(800, 0.0);s.b[1156] = (s.v[241] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_primal_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);}
        s.store_scalar(801, 0.0);s.b[1157] = (s.v[243] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_primal_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);}
        s.b[1158] = (s.v[245] < 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_primal_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);}
        s.store_primal_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));s.store_primal_mul(236, 236, 803);s.store_primal_mul(237, 237, 803);s.store_primal_mul(238, 238, 803);s.store_primal_div_scaled_inputs_square_rhs(804, 247, 4e-18, 192, 1.0);s.store_primal_div_scaled_inputs_square_rhs(805, 248, 4e-18, 193, 1.0);
        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_primal_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }
        s.store_primal_mul(710, 249, 796);s.store_primal_scaled_mul(806, 710, 192, 500000000.0);
        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_primal_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }
        s.store_primal_mul(711, 250, 796);s.store_primal_scaled_mul(807, 711, 193, 500000000.0);s.store_scalar(808, 0.0);s.b[1159] = (s.v[272] > 1e-10);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_primal_div_from_scalar(808, 0.75, 272);}
        s.store_primal_square(809, 273);s.store_primal_scale(810, 277, (9.1093826e-31 * 1000000000.0));s.b[1160] = (s.v[300] > 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_primal_div_from_scalar(811, 1.0, 300);}
        if (!s.b[1160]) {s.store_scalar(811, 0.0);}
        s.b[1161] = (s.v[301] > 0.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_primal_div_from_scalar(812, 1.0, 301);}
        if (!s.b[1161]) {s.store_scalar(812, 0.0);}
        s.b[1162] = (s.v[302] > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if s.b[1162] {s.store_primal_div_from_scalar(813, 1.0, 302);}
        if (!s.b[1162]) {s.store_scalar(813, 0.0);}
        s.b[1163] = (s.v[303] > 0.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_primal_div_from_scalar(814, 1.0, 303);}
        if (!s.b[1163]) {s.store_scalar(814, 0.0);}
        s.b[1164] = (s.v[304] > 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_primal_div_from_scalar(815, 1.0, 304);}
        if (!s.b[1164]) {s.store_scalar(815, 0.0);}
        s.b[1165] = (s.v[305] > 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_primal_div_from_scalar(816, 1.0, 305);}
        if (!s.b[1165]) {s.store_scalar(816, 0.0);}
        s.b[1166] = (s.v[306] > 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_primal_div_from_scalar(817, 1.0, 306);}
        if (!s.b[1166]) {s.store_scalar(817, 0.0);}
        s.store_primal_scale(24, 6, s.v[646]);s.store_primal_scale(25, 6, s.v[647]);s.store_primal_scale(26, 6, s.v[648]);s.store_primal_scale(27, 6, s.v[673]);s.store_primal_scale(28, 6, s.v[674]);s.store_primal_scale(29, 6, s.v[675]);s.store_scalar(30, 0.0);s.b[1167] = (p.p43 == 3.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(30, 1.0);}
        s.copy_ad(31, 313);s.b[1168] = (p.p39 == 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));}
        s.b[1169] = ((p.p43 == 2.0) || (p.p43 == 3.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_primal_scale(24, 6, s.v[649]);s.store_primal_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));s.copy_ad(26, 31);s.store_primal_scale(27, 6, s.v[676]);s.store_primal_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));s.copy_ad(29, 31);}
        s.b[1170] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {
            if (s.v[24] > 0.0) {
                s.copy_ad(646, 24);
            } else {
                s.store_scalar(646, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[25] > 0.0) {
                s.copy_ad(647, 25);
            } else {
                s.store_scalar(647, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[26] > 0.0) {
                s.copy_ad(648, 26);
            } else {
                s.store_scalar(648, 0.0);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1170] {
            if (s.v[27] > 0.0) {
                s.copy_ad(673, 27);
            } else {
                s.store_scalar(673, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[28] > 0.0) {
                s.copy_ad(674, 28);
            } else {
                s.store_scalar(674, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[29] > 0.0) {
                s.copy_ad(675, 29);
            } else {
                s.store_scalar(675, 0.0);
            }
        }
        if (!s.b[1170]) {s.store_scalar(646, 0.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(673, 0.0);s.store_scalar(674, 0.0);s.store_scalar(675, 0.0);}
        s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(654, 0.0);s.store_scalar(681, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(667, 0.0);s.store_scalar(694, 0.0);s.store_scalar(668, 1.0);s.store_scalar(695, 1.0);s.store_scalar(669, 0.0);s.store_scalar(696, 0.0);s.store_scalar(670, 1.0);s.store_scalar(697, 1.0);s.store_scalar(671, 0.0);s.store_scalar(698, 0.0);s.store_scalar(672, 1.0);s.store_scalar(699, 1.0);s.store_scalar(666, 0.0);s.store_scalar(693, 0.0);s.store_scalar(660, 0.0);s.store_scalar(687, 0.0);s.store_scalar(661, 0.0);s.store_scalar(688, 0.0);s.store_scalar(662, 0.0);s.store_scalar(689, 0.0);s.store_scalar(663, 0.0);s.store_scalar(690, 0.0);s.store_scalar(664, 0.0);s.store_scalar(691, 0.0);s.store_scalar(665, 0.0);s.store_scalar(692, 0.0);s.store_scalar(651, 1.0);s.store_scalar(678, 1.0);s.store_scalar(652, 1.0);s.store_scalar(679, 1.0);s.store_scalar(653, 1.0);s.store_scalar(680, 1.0);s.store_scalar(491, 0.0);s.store_scalar(492, 0.0);s.store_scalar(480, 0.0);s.store_scalar(481, 0.0);s.store_scalar(482, 0.0);s.store_scalar(483, 0.0);s.store_scalar(484, 0.0);s.store_scalar(493, 0.0);s.store_scalar(494, 0.0);s.store_scalar(495, 0.0);s.store_scalar(501, 0.0);s.store_scalar(490, 0.0);s.b[1171] = (p.p43 > 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1172]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1172])) {s.store_scalar(454, 100000000.0);}
        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1173]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1173])) {s.store_scalar(455, 100000000.0);}
        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1174]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && (!s.b[1174])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(654, 454, 455, 456);}
        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1175]) {s.store_primal_exp_scaled_input(655, 654, s.v[371]);}
        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {s.store_primal_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.store_scalar(396, s.v[393]);s.store_scalar(397, s.v[394]);s.store_scalar(398, s.v[395]);s.store_scalar(399, p.p831);s.store_scalar(400, p.p832);s.store_scalar(401, p.p833);s.store_scalar(402, p.p828);s.store_scalar(403, p.p829);s.store_scalar(404, p.p830);}
        s.b[1177] = (s.v[646] == 0.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1177]) {s.store_scalar(396, (s.v[394] + s.v[395]));s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));s.store_scalar(402, (p.p829 + p.p830));}
        s.b[1178] = (s.v[647] == 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1178]) {s.store_scalar(397, (s.v[393] + s.v[395]));s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));s.store_scalar(403, (p.p828 + p.p830));}
        s.b[1179] = (s.v[648] == 0.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1179]) {s.store_scalar(398, (s.v[393] + s.v[394]));s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));s.store_scalar(404, (p.p828 + p.p829));}
        if s.b[1171] {s.store_min3(656, 396, 397, 398);s.store_primal_scale(657, 656, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(658, 656, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);s.store_primal_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1180]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1180])) {s.store_scalar(454, 100000000.0);}
        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1181]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1181])) {s.store_scalar(455, 100000000.0);}
        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1182]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1182])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(681, 454, 455, 456);}
        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1183]) {s.store_primal_exp_scaled_input(682, 681, s.v[371]);}
        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {s.store_primal_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.copy_ad(396, 569);s.copy_ad(397, 570);s.copy_ad(398, 571);s.copy_ad(399, 511);s.copy_ad(400, 512);s.copy_ad(401, 513);s.copy_ad(402, 508);s.copy_ad(403, 509);s.copy_ad(404, 510);}
        s.b[1185] = (s.v[673] == 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1185]) {s.store_primal_add(396, 570, 571);s.store_primal_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);s.store_primal_add(402, 509, 510);}
        s.b[1186] = (s.v[674] == 0.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1186]) {s.store_primal_add(397, 569, 571);s.store_primal_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);s.store_primal_add(403, 508, 510);}
        s.b[1187] = (s.v[675] == 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1187]) {s.store_primal_add(398, 569, 570);s.store_primal_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);s.store_primal_add(404, 508, 509);}
        if s.b[1171] {s.store_min3(683, 396, 397, 398);s.store_primal_scale(684, 683, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(685, 683, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);s.store_primal_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
        s.b[1188] = (s.v[474] == 1.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1188]) {s.store_scalar(1189, 0.0);s.store_scalar(1190, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1198, 0.0);s.store_scalar(1200, 0.0);s.store_scalar(1201, 0.0);s.store_scalar(1202, 0.0);s.store_scalar(1203, 0.0);s.store_scalar(1204, 0.0);s.store_scalar(1205, 0.0);s.store_scalar(1206, 0.0);s.store_scalar(1207, 0.0);s.store_scalar(1208, 0.0);s.store_scalar(1209, 0.0);s.store_scalar(1210, 0.0);s.store_scalar(1211, 0.0);s.store_scalar(1212, 0.0);s.store_scalar(1213, 0.0);s.store_scalar(1214, 0.0);s.store_scalar(1215, 0.0);s.store_scalar(1216, 0.0);s.store_scalar(1217, 0.0);s.store_scalar(1218, 0.0);s.store_scalar(1219, 0.0);s.store_scalar(1220, 0.0);s.store_scalar(1221, 0.0);s.store_scalar(1222, 0.0);s.store_scalar(1223, 0.0);s.store_scalar(1224, 0.0);s.store_scalar(1225, 0.0);s.store_scalar(1226, 0.0);s.store_scalar(1227, 0.0);s.store_scalar(1228, 0.0);s.store_scalar(1229, 0.0);s.store_scalar(1230, 0.0);s.store_scalar(1231, 0.0);s.store_scalar(1232, 0.0);s.store_scalar(1233, 0.0);s.store_scalar(498, 0.4);s.store_scalar(499, 0.65);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1188]) {s.store_scalar(500, 0.8);s.store_primal_scale(485, 498, (-p.p928));s.store_primal_scale(486, 499, (-p.p928));s.store_primal_scale(487, 500, (-p.p928));s.store_scalar(488, 0.1);s.store_scalar(489, 0.2);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1237] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });s.b[1238] = (s.v[485] < s.v[654]);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });s.b[1239] = (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && s.b[1239]) {s.store_primal_exp_scaled_input(1200, 485, (s.v[371] * (-0.5)));}
        s.b[1240] = (((-0.5) * (s.v[485] * s.v[371])) < 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(485), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1238])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(485), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1241] = (s.v[485] > 0.0);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1241]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1241])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 485);}
        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 485, 0.5, 1203, 0.5, 485, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 485, 0.5, 659, 0.5, 485, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 485, A::sqrt_square_offset(s.ad_value(485), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1242] = (s.v[646] == 0.0);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1242]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {s.store_primal_scale(1208, 1198, s.v[387]);}
        s.b[1243] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1243]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1244] = (p.p831 == 0.5);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1244]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1244])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1245] = (p.p831 == 0.5);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1245]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1245])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p.p840);}
        s.b[1246] = (p.p845 == 0.0);s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1246]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1247] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1247]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1247])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1248] = (s.v[1228] > 0.0);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1248]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1248])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1249] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1249]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1249])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1250] = (s.v[1228] > 0.0);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1250]) {s.copy_ad(1229, 1191);}
        s.b[1251] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && s.b[1251]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && (!s.b[1251])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);}
        s.b[1252] = (p.p851 == 0.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1252]) {s.store_scalar(1231, 0.0);}
        s.b[1253] = (p.p831 == 0.5);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1253]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1253])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 1207, 1.0);}
        s.b[1254] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1254]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1255] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && s.b[1255]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && (!s.b[1255])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851, 0.0);}
        s.b[1256] = (p.p860 > 1000.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1256]) {s.store_scalar(1233, 1.0);}
        s.b[1257] = (s.v[1206] > ((-s.v[444]) * p.p860));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });s.b[1258] = (p.p863 == 4.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && s.b[1258]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && (!s.b[1258])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && (!s.b[1257])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1259] = (s.v[647] == 0.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1259]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1260] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1260]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1261] = (p.p832 == 0.5);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1261]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1261])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1262] = (p.p832 == 0.5);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1262]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1262])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p.p841);}
        s.b[1263] = (p.p846 == 0.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1263]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1264] = (((-p.p832) * s.v[412]) == (-1.0));s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1264]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1264])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1265] = (s.v[1228] > 0.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1265]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1265])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1266] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1266]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1266])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1267] = (s.v[1228] > 0.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1267]) {s.copy_ad(1229, 1191);}
        s.b[1268] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && s.b[1268]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && (!s.b[1268])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);}
        s.b[1269] = (p.p852 == 0.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1269]) {s.store_scalar(1231, 0.0);}
        s.b[1270] = (p.p832 == 0.5);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1270]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1270])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1271] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1271]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1272] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && s.b[1272]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && (!s.b[1272])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852, 0.0);}
        s.b[1273] = (p.p861 > 1000.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1273]) {s.store_scalar(1233, 1.0);}
        s.b[1274] = (s.v[1206] > ((-s.v[444]) * p.p861));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });s.b[1275] = (p.p864 == 4.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && s.b[1275]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && (!s.b[1275])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && (!s.b[1274])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1276] = (s.v[648] == 0.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1276]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1277] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1277]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1278] = (p.p833 == 0.5);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1278]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1278])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1279] = (p.p833 == 0.5);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1279]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1279])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p.p842);}
        s.b[1280] = (p.p847 == 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1280]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1281] = (((-p.p833) * s.v[413]) == (-1.0));s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1281]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1281])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1282] = (s.v[1228] > 0.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1282]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1282])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1283] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1283]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1283])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1284] = (s.v[1228] > 0.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1284]) {s.copy_ad(1229, 1191);}
        s.b[1285] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && s.b[1285]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && (!s.b[1285])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);}
        s.b[1286] = (p.p853 == 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1286]) {s.store_scalar(1231, 0.0);}
        s.b[1287] = (p.p833 == 0.5);s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1287]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1287])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1288] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1288]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1289] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && s.b[1289]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && (!s.b[1289])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853, 0.0);}
        s.b[1290] = (p.p862 > 1000.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1290]) {s.store_scalar(1233, 1.0);}
        s.b[1291] = (s.v[1206] > ((-s.v[444]) * p.p862));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });s.b[1292] = (p.p865 == 4.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && s.b[1292]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && (!s.b[1292])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && (!s.b[1291])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(475, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1293] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });s.b[1294] = (s.v[486] < s.v[654]);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });s.b[1295] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && s.b[1295]) {s.store_primal_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));}
        s.b[1296] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && s.b[1296]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && (!s.b[1296])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1294])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1297] = (s.v[486] > 0.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1297]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1297])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);}
        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 486, 0.5, 1203, 0.5, 486, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 486, 0.5, 659, 0.5, 486, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 486, A::sqrt_square_offset(s.ad_value(486), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1298] = (s.v[646] == 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1298]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {s.store_primal_scale(1208, 1198, s.v[387]);}
        s.b[1299] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1299]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1300] = (p.p831 == 0.5);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1300]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1300])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1301] = (p.p831 == 0.5);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1301]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1301])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p.p840);}
        s.b[1302] = (p.p845 == 0.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1302]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1303] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1303]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1303])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1304] = (s.v[1228] > 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1304]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1304])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1305] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1305]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1305])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1306] = (s.v[1228] > 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1306]) {s.copy_ad(1229, 1191);}
        s.b[1307] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && s.b[1307]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && (!s.b[1307])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);}
        s.b[1308] = (p.p851 == 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1308]) {s.store_scalar(1231, 0.0);}
        s.b[1309] = (p.p831 == 0.5);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1309]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1309])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 1207, 1.0);}
        s.b[1310] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1310]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1311] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && s.b[1311]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && (!s.b[1311])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
}
