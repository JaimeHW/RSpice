#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);s.store_primal_mul(154, 154, 1013);s.store_primal_div_scaled_inputs_indices(1013, 1012, p.p803, 1010, 1.0);s.store_primal_add(44, 44, 1013);s.store_primal_add(149, 149, 1013);s.store_primal_div_scaled_inputs_mixed_ia(1013, 1012, p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);s.store_primal_add(66, 66, 1013);s.store_primal_add(159, 159, 1013);}
        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {s.store_primal_offset(1012, 8, s.v[12]);s.store_scalar(1013, (1.0 / p.p811));s.store_primal_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);s.store_primal_div_scaled_add_product_mixed_aaai(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), 8, 1.0);s.store_primal_div_scaled_add_product_mixed_aaai(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), 8, 1.0);}
        if (s.b[1030] && s.b[1140]) {s.store_primal_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p.p812, 17, p.p813);s.store_primal_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);}
        s.copy_ad(177, 44);s.copy_ad(178, 45);s.copy_ad(179, 46);s.copy_ad(181, 47);s.copy_ad(182, 48);
        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }
        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }
        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }
        s.copy_ad(186, 52);s.copy_ad(187, 53);
        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }
        s.copy_ad(192, 59);s.copy_ad(193, 60);
        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }
        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }
        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }
        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }
        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }
        s.copy_ad(180, 58);
        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
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
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
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
        s.copy_ad(276, 143);s.copy_ad(282, 149);s.copy_ad(283, 150);s.copy_ad(284, 151);
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
        s.store_primal_scale(768, 182, 8.8541878176e-12);s.store_primal_div(769, 768, 181);s.store_primal_square(770, 181);s.store_primal_scale(771, 769, 6.241449993689894e18);s.store_primal_mul(772, 257, 183);
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }
        s.store_scalar(773, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1143] = (p.p52 > 0.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
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
        s.b[1150] = (s.v[820] <= 0.45);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if ((!s.b[1149]) && s.b[1150]) {s.store_primal_offset_scaled(793, 820, 22.0, 3.0);}
        s.b[1151] = (s.v[820] <= 1.6);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {s.store_primal_offset_scaled(793, 820, (-7.2), 15.5);}
        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {s.copy_ad(793, 783);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if s.b[1154] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(745, 745, 735);s.store_primal_mul_scale_offset_mixed_ia(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_scale(747, 745, 0.95);s.store_primal_scaled_mul(748, 745, 745, 0.0025);s.copy_ad(749, 748);s.store_primal_scaled_sqrt(740, 749, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));s.store_primal_offset_add_scaled_product_mixed_iia(700, 177, 1.0, 178, A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);s.store_primal_exp_scaled_input(751, 180, s.v[360]);s.store_primal_mul(701, 189, 751);s.store_primal_scale(702, 190, 1.0 / (s.v[359]));s.store_primal_exp_scaled_input(752, 203, s.v[360]);s.store_primal_mul(703, 202, 752);s.store_primal_scaled_mul(716, 703, 769, s.v[20]);s.store_primal_mul_mixed_ia(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));s.store_primal_exp_scaled_input(753, 205, s.v[360]);s.store_primal_mul(704, 204, 753);s.store_primal_mul_mixed_ia(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));s.store_primal_exp_scaled_input(754, 209, s.v[360]);s.store_primal_mul(706, 208, 754);s.store_primal_exp_scaled_input(755, 213, s.v[360]);s.store_primal_mul(708, 212, 755);s.store_primal_exp_scaled_input(756, 216, s.v[360]);s.store_primal_mul(709, 215, 756);s.store_primal_scaled_mul(757, 716, 709, 2.0);s.store_primal_exp_scaled_input(758, 220, s.v[360]);s.store_primal_mul(720, 219, 758);s.store_primal_mul(721, 258, 758);s.store_primal_mul_mixed_ia(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));s.store_primal_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_primal_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);s.store_primal_exp_scaled_input(759, 288, s.v[360]);s.store_primal_mul(714, 287, 759);s.store_primal_scaled_mul(717, 714, 769, s.v[22]);s.store_primal_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);s.store_primal_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);}
        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }
        if s.b[1155] {s.store_primal_div_mixed_ai(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_primal_square(724, 761);s.store_primal_ln(725, 724);s.store_primal_scale(762, 760, 0.95);s.store_primal_scaled_mul(763, 760, 760, 0.0025);s.copy_ad(764, 763);s.store_primal_scaled_sqrt(765, 764, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));}
        if (!s.b[1155]) {s.store_scalar(713, 0.0);s.store_scalar(759, 1.0);s.store_scalar(714, 0.0);s.store_scalar(717, 0.0);s.store_scalar(723, s.v[715]);s.store_scalar(760, 0.0);s.store_scalar(761, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 0.0);s.store_scalar(762, 0.0);s.store_scalar(763, 0.0);s.store_scalar(764, 0.0);s.store_scalar(765, 0.0);s.store_scalar(766, 0.0);}
        s.store_primal_div_from_scalar(795, 1.0, 246);s.store_primal_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));s.store_primal_mul(797, 796, 181);s.store_primal_mul(798, 796, 192);s.store_primal_mul(799, 796, 193);s.store_scalar(800, 0.0);s.b[1156] = (s.v[241] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_primal_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);}
        s.store_scalar(801, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1157] = (s.v[243] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_primal_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);}
        s.b[1158] = (s.v[245] < 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_primal_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);}
        s.store_primal_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));s.store_primal_mul(236, 236, 803);s.store_primal_mul(237, 237, 803);s.store_primal_mul(238, 238, 803);
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
        s.store_primal_square(809, 273);s.store_primal_scale(24, 6, s.v[646]);s.store_primal_scale(25, 6, s.v[647]);s.store_primal_scale(26, 6, s.v[648]);s.store_primal_scale(27, 6, s.v[673]);s.store_primal_scale(28, 6, s.v[674]);s.store_primal_scale(29, 6, s.v[675]);s.store_scalar(30, 0.0);s.b[1167] = (p.p43 == 3.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
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
        s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(654, 0.0);s.store_scalar(681, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(651, 1.0);s.store_scalar(678, 1.0);s.store_scalar(652, 1.0);s.store_scalar(679, 1.0);s.store_scalar(653, 1.0);s.store_scalar(680, 1.0);s.store_scalar(501, 0.0);s.b[1171] = (p.p43 > 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1172]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1172])) {s.store_scalar(454, 100000000.0);}
        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1173]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1173])) {s.store_scalar(455, 100000000.0);}
        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1174]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1182]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1182])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(681, 454, 455, 456);}
        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1183]) {s.store_primal_exp_scaled_input(682, 681, s.v[371]);}
        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
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
        if (s.b[1171] && s.b[1188]) {s.store_primal_add_scaled_inputs3_indices(501, 646, (s.v[414] * p.p929), 647, (s.v[415] * p.p929), 648, (s.v[416] * p.p929));}
        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1523]) {s.store_scalar(651, 0.0);}
        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1524]) {s.store_scalar(652, 0.0);}
        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1525]) {s.store_scalar(653, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_primal_mul_mixed_ia(501, 553, A::add_scaled_products3(s.ad_value(673), s.ad_value(581), 1.0, s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0));}
        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1813]) {s.store_scalar(678, 0.0);}
        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1814]) {s.store_scalar(679, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1815]) {s.store_scalar(680, 0.0);}
        s.store_scalar(2027, 0.0);s.store_scalar(2028, 0.0);s.store_scalar(2029, 0.0);s.store_scalar(1937, 1.0);s.store_scalar(1936, 0.0);s.b[2102] = (s.v[0] == 1.0);s.store_scalar(2102, if s.b[2102] { 1.0 } else { 0.0 });
        if s.b[2102] {s.store_voltage(825, ctx, nodes, Some(5), Some(6));s.store_voltage(826, ctx, nodes, Some(7), Some(6));s.store_voltage(827, ctx, nodes, Some(6), Some(8));s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);}
        if (!s.b[2102]) {s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);s.store_voltage(832, ctx, nodes, Some(6), Some(10));s.store_voltage(833, ctx, nodes, Some(7), Some(11));}
        s.store_add(829, 825, 827);s.copy_ad(834, 825);s.copy_ad(835, 827);s.store_add(836, 826, 827);s.store_sub(837, 825, 826);s.store_scale(1817, 834, (-s.v[355]));s.store_scale(1818, 837, (-s.v[355]));s.store_scaled_sub(1819, 829, 700, (-s.v[355]));s.store_scalar(831, 1.0);s.b[2103] = (s.v[826] < 0.0);s.store_scalar(2103, if s.b[2103] { 1.0 } else { 0.0 });
        if s.b[2103] {s.store_scalar(831, (-1.0));s.store_sub(825, 825, 826);s.store_add(827, 827, 826);s.store_neg(826, 826);}
        s.store_add(828, 826, 827);s.store_div_scaled_product_offset_denominator_mixed_iia(830, 826, 826, 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739))), (-0.5), 737, 1.0);s.copy_ad(1820, 2107);s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2107)), s.ad_value(738))), (-(-0.5)), 741, 1.0);s.copy_ad(1821, 2030);s.store_scalar(2031, 0.0);s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
        if s.b[2263] {s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));s.store_sub_mixed_ai(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));s.store_sub(2031, 1821, 2030);}
        s.copy_ad(2104, 728);s.copy_ad(2105, 738);s.copy_ad(2106, 729);s.copy_ad(2108, 2030);s.copy_ad(2112, 2031);s.copy_ad(2109, 720);s.copy_ad(2110, 777);s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));s.store_scalar(2125, 1.0);s.b[2264] = (s.v[190] > 0.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
        if s.b[2264] {s.store_primal_scale(2116, 2104, s.v[361]);s.store_scale(2117, 2113, s.v[361]);s.store_scale(2118, 2111, s.v[361]);s.store_offset_div_scaled_inputs_sqrt_rhs(2028, 2106, 0.5, 2116, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2264] {s.store_add_scaled_product_mixed_iia(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));s.store_primal_offset_scaled(2120, 2116, 0.5, 2.0);s.store_add(2121, 2116, 2117);s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2119, 0.5, 2122, 0.5, 2119, 2122, 20.0, 0.5);s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2123, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2123, 0.5, 2120, 0.5, 2123, 2120, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0, 1.0);}
        s.b[2265] = (s.v[2029] > (-230.25850929940458));s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
        if (s.b[2264] && s.b[2265]) {s.store_exp(2125, 2029);}
        if (s.b[2264] && (!s.b[2265])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_offset_mul(2126, 701, 2125, 1.0);s.store_scale(2127, 2126, s.v[715]);s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));s.store_mul_scale_offset_indices(2129, 2127, 2128, 1.0, 1.0);s.store_div_from_scalar(2130, 1.0, 2129);s.store_mul_mixed_ia(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));s.store_square(2115, 2114);s.store_div_from_scalar(2131, 1.0, 2115);s.store_mul(2132, 2108, 2130);s.store_mul(2133, 2111, 2130);s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2135, 196, 2134, A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));s.store_mul(2136, 2104, 2130);s.store_sqrt_square_add(2028, 2107, 2105);s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2137, 2130, 2135, 0.5, 2028, 0.5, 2029, ((-1.0) * (0.5)), 0.0);s.store_add(2138, 2136, 2132);s.store_sub(2139, 2138, 2137);s.b[2266] = (p.p45 > 0.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2266] && s.b[2267]) {s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);}
        s.b[2268] = (s.v[2139] < 460.51701859880916);s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {s.store_exp_neg_input(2154, 2139);}
        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2266] && (!s.b[2267])) {s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[2266] && (!s.b[2267])) {s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);}
        if (!s.b[2266]) {s.store_offset_div_scaled_inputs_sqrt_rhs(2140, 2114, 0.5, 2139, 1.0, 1.0);}
        s.store_add_scaled_value_products_mixed_iiaia(2141, 2139, 1.0, 2114, A::sqrt(s.ad_value(2139)), 1.0, 2140, A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));s.store_scalar(2147, 0.0);s.store_scalar(2149, 1.0);s.b[2269] = (s.v[2142] > (-30.0));s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
        if s.b[2269] {s.store_offset_mul(2143, 2140, 2142, (-1.0));s.store_scaled_add_mixed_ia(2027, 2143, A::sqrt_square_offset(s.ad_value(2143), 10.0), 0.5);s.store_sub_mixed_ia(2144, 2142, A::ln(s.ad_value(2027)));s.store_scaled_add_mixed_ia(2145, 2144, A::sqrt_square_offset(s.ad_value(2144), 2.0), 0.5);}
        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2270]) {s.store_exp_sub(2027, 2142, 2145);}
        if (s.b[2269] && (!s.b[2270])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if s.b[2269] {s.store_div(2146, 2027, 2140);s.store_sub_mixed_ai(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);}
        s.b[2271] = (s.v[2146] > 1e-6);s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2271]) {s.store_mul_scale_offset_mixed_ia(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0, 1.0);}
        if (s.b[2269] && (!s.b[2271])) {s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);}
        if s.b[2269] {s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2269] {s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));}
        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);s.store_scale(2151, 2150, 1e-5);s.store_div_from_scalar(2152, 1.0, 2150);s.store_scalar(2259, 0.0);s.store_scalar(2153, 0.0);s.b[2272] = (s.v[2139] < 460.51701859880916);s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });
        if s.b[2272] {s.store_exp_neg_input(2154, 2139);}
        if (!s.b[2272]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });
        if s.b[2273] {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2153, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        s.b[2274] = (s.v[2133] < (-s.v[2151]));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((!s.b[2273]) && s.b[2274]) {s.store_neg(2241, 2133);s.store_scaled_mul(2242, 2241, 2152, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2238, 2241, 2243);s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);s.store_sub_ln_mul_lhs(2246, 2244, 2131, 2243);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);s.store_add_mixed_ia(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));}
        s.b[2275] = (s.v[2247] < 230.25850929940458);s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_exp(2248, 2247);}
        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((!s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(2249, 1.0, 2248);s.store_div_from_scalar_offset_square(2238, 1.0, 2247, 2.0);s.store_mul_square_lhs(2250, 2247, 2238);s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2241, 2247);s.store_mul(2239, 2154, 2249);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2273]) && s.b[2274]) {s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2256, 2255, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2257, 2133, 2152, A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));}
        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {s.store_exp_neg_input(2238, 2257);}
        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar(2258, 1.0, 2238);s.store_add_scaled_inputs_product_mixed_iiia(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));s.store_offset(2260, 2139, 3.0);s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2139, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2277] = (s.v[2262] < 230.25850929940458);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2154, 2248);}
        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {s.store_exp_sub(2248, 2262, 2139);s.store_div(2249, 2154, 2248);}
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(2156, 0.0);s.store_scalar(2157, 0.0);s.store_scalar(2158, 0.0);s.store_scalar(2159, 0.0);s.store_scalar(2160, 0.0);s.store_scalar(2161, 0.0);s.store_scalar(2162, 0.0);s.store_scalar(2163, 1.0);s.store_scalar(2164, 1.0);s.store_sub(2165, 2133, 2153);s.store_scalar(2166, 0.0);s.store_mul(2167, 2129, 2165);s.store_scalar(2168, 1.0);s.store_scalar(2169, 1.0);s.store_scalar(2173, 1.0);s.store_scalar(2174, 1.0);s.store_scalar(2176, 1.0);s.b[2279] = (s.v[2133] > 0.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        if s.b[2279] {s.store_div_from_scalar_offset_square(2027, 1.0, 2153, 2.0);s.store_mul_square_lhs(2155, 2153, 2027);s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);s.store_mul_ad_product_lhs_mixed_ai(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), 2027, 2027);s.store_scalar(2158, 0.0);}
        s.b[2280] = (s.v[2153] < 230.25850929940458);s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2280]) {s.store_exp(2158, 2153);s.store_div_from_scalar(2159, 1.0, 2158);s.store_mul(2158, 2154, 2158);}
        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });
        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {s.store_exp_sub(2158, 2153, 2139);s.store_div(2159, 2154, 2158);}
        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2279] {s.store_add_scaled_product_mixed_iia(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));}
        s.b[2282] = (s.v[2153] < 1e-5);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2282]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2163, 2114, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, 2027, 1.0, 1.0);}
        if (s.b[2279] && (!s.b[2282])) {s.store_add_offset_lhs(2161, 2153, (-1.0), 2159);s.store_sqrt(2162, 2161);s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);}
        if s.b[2279] {s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);}
        s.b[2283] = (s.v[2160] > 1e-100);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2283]) {s.store_mul_sqrt_mixed_ia(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);}
    }
}
