#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }

        s.store_scalar(773, 0.0);

        s.b[1143] = (p.p52 > 0.0);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.b[1144] = (s.v[0] == (-1.0));
        s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });

        if (s.b[1143] && s.b[1144]) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));

        s.store_scale(775, 214, 0.5);

        s.store_scalar(776, 0.5);

        s.b[1145] = (s.v[0] == (-1.0));
        s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });

        if s.b[1145] {
            s.store_scale(775, 214, 0.3333333333333333);
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(777, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(778, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(779, 1.0, 228);

        s.store_div(780, 768, 192);

        s.store_div(781, 768, 193);

        s.store_div_ad_lhs(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);

        s.store_div_ad_lhs(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);

        s.store_square(784, 782);

        s.store_square(785, 783);

        s.store_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);

        s.store_add_ad_lhs(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);

        s.store_div_from_scalar(820, 1.0, 782);

        s.store_offset_scaled(821, 782, 3.1, 8.5);

        s.store_square(789, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1146] = (s.v[820] < 0.06);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if s.b[1146] {
            s.store_scale(790, 820, 64.0);
        }

        s.b[1147] = (s.v[820] <= 0.45);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if ((!s.b[1146]) && s.b[1147]) {
            s.store_offset_scaled(790, 820, 22.0, 3.0);
        }

        s.b[1148] = (s.v[820] <= 1.6);
        s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });

        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {
            s.store_offset_scaled(790, 820, (-7.2), 15.5);
        }

        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {
            s.copy_ad(790, 782);
        }

        s.store_add_scaled_inputs_product_right_ad(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));

        s.store_div_from_scalar(820, 1.0, 783);

        s.store_offset_scaled(821, 783, 3.1, 8.5);

        s.store_square(792, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1149] = (s.v[820] < 0.06);
        s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });

        if s.b[1149] {
            s.store_scale(793, 820, 64.0);
        }

        s.b[1150] = (s.v[820] <= 0.45);
        s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });

        if ((!s.b[1149]) && s.b[1150]) {
            s.store_offset_scaled(793, 820, 22.0, 3.0);
        }

        s.b[1151] = (s.v[820] <= 1.6);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {
            s.store_offset_scaled(793, 820, (-7.2), 15.5);
        }

        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {
            s.copy_ad(793, 783);
        }

        s.store_add_scaled_inputs_product_right_ad(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));

        if (!(s.v[728] > 0.05)) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.store_scalar(730, 0.0);

        s.store_scalar(731, 0.0);

        s.b[1152] = (s.v[188] > 0.0);
        s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });

        if s.b[1152] {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

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

        if s.b[1152] {
            s.store_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));
        }

        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));

        s.b[1153] = (p.p52 > 0.0);
        s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });

        if s.b[1153] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(728, 728, 735);
            s.store_mul_offset_ad_rhs(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_scaled_mul(738, 728, 728, 0.0025);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);

        s.store_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);

        if (!(s.v[745] > 0.05)) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.b[1154] = (p.p52 > 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if s.b[1154] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(745, 745, 735);
            s.store_mul_offset_ad_rhs(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_scale(747, 745, 0.95);

        s.store_scaled_mul(748, 745, 745, 0.0025);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));

        s.store_offset_add_scaled_product(700, s.ad_value(177), 1.0, s.ad_value(178), A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);

        s.store_exp_scaled_input(751, 180, s.v[360]);

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_scaled_input(752, 203, s.v[360]);

        s.store_mul(703, 202, 752);

        s.store_scaled_mul(716, 703, 769, s.v[20]);

        s.store_mul_ad_rhs(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));

        s.store_exp_scaled_input(753, 205, s.v[360]);

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));

        s.store_exp_scaled_input(754, 209, s.v[360]);

        s.store_mul(706, 208, 754);

        s.store_exp_scaled_input(755, 213, s.v[360]);

        s.store_mul(708, 212, 755);

        s.store_exp_scaled_input(756, 216, s.v[360]);

        s.store_mul(709, 215, 756);

        s.store_scaled_mul(757, 716, 709, 2.0);

        s.store_exp_scaled_input(758, 220, s.v[360]);

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if s.b[1155] {
            s.store_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);
            s.store_exp_scaled_input(759, 288, s.v[360]);
            s.store_mul(714, 287, 759);
            s.store_scaled_mul(717, 714, 769, s.v[22]);
            s.store_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);
            s.store_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }

        if s.b[1155] {
            s.store_div_ad_lhs(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
            s.store_square(724, 761);
            s.store_ln(725, 724);
            s.store_scale(762, 760, 0.95);
            s.store_scaled_mul(763, 760, 760, 0.0025);
            s.copy_ad(764, 763);
            s.store_scaled_sqrt(765, 764, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));
        }

        if (!s.b[1155]) {
            s.store_scalar(713, 0.0);
            s.store_scalar(759, 1.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(723, s.v[715]);
            s.store_scalar(760, 0.0);
            s.store_scalar(761, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.store_scalar(800, 0.0);

        s.b[1156] = (s.v[241] < 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if s.b[1156] {
            s.store_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);
        }

        s.store_scalar(801, 0.0);

        s.b[1157] = (s.v[243] < 0.0);
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if s.b[1157] {
            s.store_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);
        }

        s.b[1158] = (s.v[245] < 0.0);
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if s.b[1158] {
            s.store_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);
        }

        s.store_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));

        s.store_mul(236, 236, 803);

        s.store_mul(237, 237, 803);

        s.store_mul(238, 238, 803);

        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(710, 249, 796);

        s.store_scaled_mul(806, 710, 192, 500000000.0);

        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(711, 250, 796);

        s.store_scaled_mul(807, 711, 193, 500000000.0);

        s.store_scalar(808, 0.0);

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1159] = (s.v[272] > 1e-10);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if s.b[1159] {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.store_scalar(30, 0.0);

        s.b[1167] = (p.p43 == 3.0);
        s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });

        if s.b[1167] {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.b[1168] = (p.p39 == 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if s.b[1168] {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.b[1169] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if s.b[1169] {
            s.store_scale(24, 6, s.v[649]);
            s.store_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));
            s.copy_ad(26, 31);
            s.store_scale(27, 6, s.v[676]);
            s.store_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));
            s.copy_ad(29, 31);
        }

        s.b[1170] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

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

        if (!s.b[1170]) {
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
        }

        s.store_scalar(656, 0.0);

        s.store_scalar(683, 0.0);

        s.store_scalar(658, 0.0);

        s.store_scalar(685, 0.0);

        s.store_scalar(657, 0.0);

        s.store_scalar(684, 0.0);

        s.store_scalar(659, 0.0);

        s.store_scalar(686, 0.0);

        s.store_scalar(654, 0.0);

        s.store_scalar(681, 0.0);

        s.store_scalar(655, 0.0);

        s.store_scalar(682, 0.0);

        s.store_scalar(651, 1.0);

        s.store_scalar(678, 1.0);

        s.store_scalar(652, 1.0);

        s.store_scalar(679, 1.0);

        s.store_scalar(653, 1.0);

        s.store_scalar(680, 1.0);

        s.store_scalar(501, 0.0);

        s.b[1171] = (p.p43 > 0.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1172]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1172])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1173]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1173])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1174]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1174])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(654, 454, 455, 456);
        }

        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1175]) {
            s.store_exp_scaled_input(655, 654, s.v[371]);
        }

        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.store_scalar(396, s.v[393]);
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, p.p831);
            s.store_scalar(400, p.p832);
            s.store_scalar(401, p.p833);
            s.store_scalar(402, p.p828);
            s.store_scalar(403, p.p829);
            s.store_scalar(404, p.p830);
        }

        s.b[1177] = (s.v[646] == 0.0);
        s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1177]) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.b[1178] = (s.v[647] == 0.0);
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1178]) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.b[1179] = (s.v[648] == 0.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1179]) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if s.b[1171] {
            s.store_min3(656, 396, 397, 398);
            s.store_scale(657, 656, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(658, 656, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1180]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1180])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1181]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1181])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1182]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1182])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(681, 454, 455, 456);
        }

        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1183]) {
            s.store_exp_scaled_input(682, 681, s.v[371]);
        }

        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.copy_ad(396, 569);
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 511);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 508);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
        }

        s.b[1185] = (s.v[673] == 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1185]) {
            s.store_add(396, 570, 571);
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(402, 509, 510);
        }

        s.b[1186] = (s.v[674] == 0.0);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1186]) {
            s.store_add(397, 569, 571);
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
            s.store_add(403, 508, 510);
        }

        s.b[1187] = (s.v[675] == 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1187]) {
            s.store_add(398, 569, 570);
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
            s.store_add(404, 508, 509);
        }

        if s.b[1171] {
            s.store_min3(683, 396, 397, 398);
            s.store_scale(684, 683, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(685, 683, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1188] = (s.v[474] == 1.0);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_inputs3_indices(501, 646, (s.v[414] * p.p929), 647, (s.v[415] * p.p929), 648, (s.v[416] * p.p929));
        }

        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);
        s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1523]) {
            s.store_scalar(651, 0.0);
        }

        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);
        s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1524]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);
        s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1525]) {
            s.store_scalar(653, 0.0);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_mul_ad_rhs(501, 553, A::add_scaled_products3(s.ad_value(673), s.ad_value(581), 1.0, s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0));
        }

        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);
        s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1813]) {
            s.store_scalar(678, 0.0);
        }

        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);
        s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1814]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);
        s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1815]) {
            s.store_scalar(680, 0.0);
        }

        s.store_scalar(2027, 0.0);

        s.store_scalar(2028, 0.0);

        s.store_scalar(2029, 0.0);

        s.store_scalar(1937, 1.0);

        s.store_scalar(1936, 0.0);

        s.b[2102] = (s.v[0] == 1.0);
        s.store_scalar(2102, if s.b[2102] { 1.0 } else { 0.0 });

        if s.b[2102] {
            s.store_voltage(825, ctx, nodes, Some(5), Some(6));
            s.store_voltage(826, ctx, nodes, Some(7), Some(6));
            s.store_voltage(827, ctx, nodes, Some(6), Some(8));
            s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[2102]) {
            s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(832, ctx, nodes, Some(6), Some(10));
            s.store_voltage(833, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(829, 825, 827);

        s.copy_ad(834, 825);

        s.copy_ad(835, 827);

        s.store_add(836, 826, 827);

        s.store_sub(837, 825, 826);

        s.store_scale(1817, 834, (-s.v[355]));

        s.store_scale(1818, 837, (-s.v[355]));

        s.store_scaled_sub(1819, 829, 700, (-s.v[355]));

        s.store_scalar(831, 1.0);

        s.b[2103] = (s.v[826] < 0.0);
        s.store_scalar(2103, if s.b[2103] { 1.0 } else { 0.0 });

        if s.b[2103] {
            s.store_scalar(831, (-1.0));
            s.store_sub(825, 825, 826);
            s.store_add(827, 827, 826);
            s.store_neg(826, 826);
        }

        s.store_add(828, 826, 827);

        s.store_div_scaled_product_offset_denominator(830, s.ad_value(826), s.ad_value(826), 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739))), (-0.5), 737, 1.0);

        s.copy_ad(1820, 2107);

        s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2107)), s.ad_value(738))), (-(-0.5)), 741, 1.0);

        s.copy_ad(1821, 2030);

        s.store_scalar(2031, 0.0);

        s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));
        s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });

        if s.b[2263] {
            s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));
            s.store_sub_ad_lhs(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);
            s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);
            s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));
            s.store_sub(2031, 1821, 2030);
        }

        s.copy_ad(2104, 728);

        s.copy_ad(2105, 738);

        s.copy_ad(2106, 729);

        s.copy_ad(2108, 2030);

        s.copy_ad(2112, 2031);

        s.copy_ad(2109, 720);

        s.copy_ad(2110, 777);

        s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);

        s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));

        s.store_scalar(2125, 1.0);

        s.b[2264] = (s.v[190] > 0.0);
        s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });

        if s.b[2264] {
            s.store_scale(2116, 2104, s.v[361]);
            s.store_scale(2117, 2113, s.v[361]);
            s.store_scale(2118, 2111, s.v[361]);
            s.store_offset_div_scaled_inputs_mixed_ia(2028, 2106, 0.5, A::sqrt(s.ad_value(2116)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));
            s.store_offset_scaled(2120, 2116, 0.5, 2.0);
            s.store_add(2121, 2116, 2117);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);
            s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2119, 0.5, 2122, 0.5, 2119, 2122, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2123, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2123, 0.5, 2120, 0.5, 2123, 2120, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0);
        }

        s.b[2265] = (s.v[2029] > (-230.25850929940458));
        s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });

        if (s.b[2264] && s.b[2265]) {
            s.store_exp(2125, 2029);
        }

        if (s.b[2264] && (!s.b[2265])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2126, 701, 2125, 1.0);

        s.store_scale(2127, 2126, s.v[715]);

        s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));

        s.store_mul_offset_rhs(2129, 2127, 2128, 1.0);

        s.store_div_from_scalar(2130, 1.0, 2129);

        s.store_mul_ad_rhs(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));

        s.store_square(2115, 2114);

        s.store_div_from_scalar(2131, 1.0, 2115);

        s.store_mul(2132, 2108, 2130);

        s.store_mul(2133, 2111, 2130);

        s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs_mixed_ia(2135, 196, 2134, A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));

        s.store_mul(2136, 2104, 2130);

        s.store_sqrt_square_add(2028, 2107, 2105);

        s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105));

        s.store_mul_add_scaled_inputs3_offset_rhs(2137, 2130, s.ad_value(2135), 0.5, s.ad_value(2028), 0.5, s.ad_value(2029), ((-1.0) * (0.5)), 0.0);

        s.store_add(2138, 2136, 2132);

        s.store_sub(2139, 2138, 2137);

        s.b[2266] = (p.p45 > 0.0);
        s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });

        s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);
        s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });

        if (s.b[2266] && s.b[2267]) {
            s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);
        }

        s.b[2268] = (s.v[2139] < 460.51701859880916);
        s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });

        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {
            s.store_exp_neg_input(2154, 2139);
        }

        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);
        }

        if (!s.b[2266]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2140, 2114, 0.5, A::sqrt(s.ad_value(2139)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2141, s.ad_value(2139), 1.0, s.ad_value(2114), A::sqrt(s.ad_value(2139)), 1.0, s.ad_value(2140), A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));

        s.store_scalar(2147, 0.0);

        s.store_scalar(2149, 1.0);

        s.b[2269] = (s.v[2142] > (-30.0));
        s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });

        if s.b[2269] {
            s.store_offset_mul(2143, 2140, 2142, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2143, 2143, 10.0, 0.5);
            s.store_sub_ad_rhs(2144, 2142, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2145, 2144, 2144, 2.0, 0.5);
        }

        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);
        s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });

        if (s.b[2269] && s.b[2270]) {
            s.store_exp_sub(2027, 2142, 2145);
        }

        if (s.b[2269] && (!s.b[2270])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if s.b[2269] {
            s.store_div(2146, 2027, 2140);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);
        }

        s.b[2271] = (s.v[2146] > 1e-6);
        s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });

        if (s.b[2269] && s.b[2271]) {
            s.store_mul_offset_ad_rhs(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0);
        }

        if (s.b[2269] && (!s.b[2271])) {
            s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if s.b[2269] {
            s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);
            s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));
        }

        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);

        s.store_scale(2151, 2150, 1e-5);

        s.store_div_from_scalar(2152, 1.0, 2150);

        s.store_scalar(2259, 0.0);

        s.store_scalar(2153, 0.0);

        s.b[2272] = (s.v[2139] < 460.51701859880916);
        s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });

        if s.b[2272] {
            s.store_exp_neg_input(2154, 2139);
        }

        if (!s.b[2272]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });

        if s.b[2273] {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2153, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        s.b[2274] = (s.v[2133] < (-s.v[2151]));
        s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_neg(2241, 2133);
            s.store_scaled_mul(2242, 2241, 2152, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2238, 2241, 2243);
            s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);
            s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);
            s.store_sub_ln_mul_lhs(2246, 2244, 2131, 2243);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);
            s.store_add_ad_rhs(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));
        }

        s.b[2275] = (s.v[2247] < 230.25850929940458);
        s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });

        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {
            s.store_exp(2248, 2247);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_div_from_scalar_offset_square(2238, 1.0, 2247, 2.0);
            s.store_mul_square_lhs(2250, 2247, 2238);
            s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);
            s.store_sub(2238, 2241, 2247);
            s.store_mul(2239, 2154, 2249);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2256, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), (-1.0), 2255);
            s.store_mul_ad_product_rhs_mixed_ia(2257, 2133, 2152, A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));
        }

        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));
        s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {
            s.store_exp_neg_input(2238, 2257);
        }

        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar(2258, 1.0, 2238);
            s.store_add_scaled_inputs_product_right_ad(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));
            s.store_offset(2260, 2139, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2246, 2139, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2277] = (s.v[2262] < 230.25850929940458);
        s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2154, 2248);
        }

        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));
        s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {
            s.store_exp_sub(2248, 2262, 2139);
            s.store_div(2249, 2154, 2248);
        }

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        s.store_scalar(2156, 0.0);

        s.store_scalar(2157, 0.0);

        s.store_scalar(2158, 0.0);

        s.store_scalar(2159, 0.0);

        s.store_scalar(2160, 0.0);

        s.store_scalar(2161, 0.0);

        s.store_scalar(2162, 0.0);

        s.store_scalar(2163, 1.0);

        s.store_scalar(2164, 1.0);

        s.store_sub(2165, 2133, 2153);

        s.store_scalar(2166, 0.0);

        s.store_mul(2167, 2129, 2165);

        s.store_scalar(2168, 1.0);

        s.store_scalar(2169, 1.0);

        s.store_scalar(2173, 1.0);

        s.store_scalar(2174, 1.0);

        s.store_scalar(2176, 1.0);

        s.b[2279] = (s.v[2133] > 0.0);
        s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });

        if s.b[2279] {
            s.store_div_from_scalar_offset_square(2027, 1.0, 2153, 2.0);
            s.store_mul_square_lhs(2155, 2153, 2027);
            s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs_mixed_ai(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), 2027, 2027);
            s.store_scalar(2158, 0.0);
        }

        s.b[2280] = (s.v[2153] < 230.25850929940458);
        s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });

        if (s.b[2279] && s.b[2280]) {
            s.store_exp(2158, 2153);
            s.store_div_from_scalar(2159, 1.0, 2158);
            s.store_mul(2158, 2154, 2158);
        }

        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));
        s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });

        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {
            s.store_exp_sub(2158, 2153, 2139);
            s.store_div(2159, 2154, 2158);
        }

        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2279] {
            s.store_add_scaled_product_right_ad(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));
        }

        s.b[2282] = (s.v[2153] < 1e-5);
        s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });

        if (s.b[2279] && s.b[2282]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);
            s.store_offset_div_scaled_product(2163, s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0, 1.0);
        }

        if (s.b[2279] && (!s.b[2282])) {
            s.store_add_offset_lhs(2161, 2153, (-1.0), 2159);
            s.store_sqrt(2162, 2161);
            s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);
        }

        if s.b[2279] {
            s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);
        }

        s.b[2283] = (s.v[2160] > 1e-100);
        s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_sqrt_ad_rhs(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));
            s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);
            s.store_mul3_lhs(2167, 2162, 2114, 2129);
        }

        s.b[2284] = (s.v[217] < 0.0);
        s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });

        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {
            s.store_offset_mul(2168, 217, 2113, 1.0);
        }

        s.b[2285] = (s.v[218] < 0.0);
        s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });

        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {
            s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2166, 1.0);
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {
            s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2166, 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_product3_indices(2170, 2166, 757, 2168, 2169, 1.0);
            s.store_mul_add_scaled_product_rhs(2171, 774, s.ad_value(2167), 1.0, s.ad_value(775), s.ad_value(2166), 1.0);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2173, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
        }

        s.b[2286] = (s.v[221] < 0.0);
        s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });

        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {
            s.store_offset_mul(2174, 221, 2113, 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul(2029, 2166, 2174);
            s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2287] = (s.v[222] < 0.0);
        s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });

        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        s.copy_ad(1822, 2111);

        s.copy_ad(1823, 2113);

        s.copy_ad(1824, 2129);

        s.copy_ad(1825, 2130);

        s.copy_ad(1826, 2114);

        s.copy_ad(1827, 2115);

        s.copy_ad(1828, 2131);

        s.copy_ad(1829, 2133);

        s.copy_ad(1830, 2138);

        s.copy_ad(1831, 2139);

        s.copy_ad(1832, 2150);

        s.copy_ad(1833, 2151);

        s.copy_ad(1834, 2152);

        s.copy_ad(1835, 2259);

        s.copy_ad(1836, 2154);

        s.copy_ad(1837, 2153);

        s.copy_ad(1838, 2156);

        s.copy_ad(1839, 2157);

        s.copy_ad(1840, 2158);

        s.copy_ad(1841, 2159);

        s.copy_ad(1842, 2161);

        s.copy_ad(1843, 2160);

        s.copy_ad(1844, 2162);

        s.copy_ad(1845, 2163);

        s.copy_ad(1846, 2164);

        s.copy_ad(1847, 2165);

        s.copy_ad(1848, 2166);

        s.copy_ad(1849, 2167);

        s.copy_ad(1850, 2168);

        s.copy_ad(1851, 2169);

        s.copy_ad(1852, 2173);

        s.copy_ad(1853, 2174);

        s.copy_ad(1854, 2176);

        s.store_scalar(2178, 0.0);

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        s.store_scale(2177, 2129, 4.60517018598809);

        s.copy_ad(2194, 2177);

        s.copy_ad(2195, 826);

        s.store_mul(2196, 826, 2130);

        s.copy_ad(2200, 2153);

        s.store_scalar(2201, 0.0);

        s.store_scalar(2204, 0.0);

        s.copy_ad(2206, 2159);

        s.copy_ad(2207, 2161);

        s.copy_ad(2209, 2160);

        s.copy_ad(2210, 2167);

        s.copy_ad(2211, 2153);

        s.copy_ad(2212, 2159);

        s.copy_ad(2214, 2160);

        s.copy_ad(2215, 2161);

        s.store_sub(2216, 2133, 2153);

        s.store_scalar(2217, 1.0);

        s.store_scalar(2219, 1.0);

        s.store_scalar(2218, 0.0);

        s.copy_ad(2228, 2166);

        s.store_mul(2232, 2216, 2129);

        s.store_scalar(2229, 0.0);

        s.copy_ad(2230, 2167);

        s.store_scalar(2235, 0.0);

        s.store_scalar(2234, 1.0);

        s.copy_ad(2237, 2109);

        s.copy_ad(2236, 2232);

        s.b[2288] = (s.v[2133] > 0.0);
        s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });

        s.b[2289] = (s.v[2160] > 1e-100);
        s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2237, 2109, 2176);
            s.store_div(2178, 2237, 2173);
            s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);
            s.store_div_scaled_product_by_product(2027, s.ad_value(2115), s.ad_value(2158), 1.0, s.ad_value(2179), s.ad_value(2179), 1.0);
        }

        s.b[2290] = (s.v[2027] > 0.0001);
        s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });

        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2291] = (s.v[2028] < 1e-10);
        s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {
            s.store_scalar(2029, 1.0);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2180, 2029, 2179);
        }

        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_scaled_mul(2181, 2129, 2180, 0.475);
            s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2182, 2027, 2027, 1e-12, 0.5);
            s.store_add_scaled_value_products(2183, s.ad_value(2166), (-1.0), s.ad_value(2129), s.ad_value(2165), 1.0, A::offset(s.ad_value(2163), (-1.0)), s.ad_value(2181), 1.0);
            s.store_offset_div_scaled_product(2184, s.ad_value(2115), s.ad_value(2129), 0.5, s.ad_value(2183), 1.0, 1.0);
            s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);
            s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);
            s.store_div(2027, 2182, 2183);
            s.store_mul_pow_ad_rhs(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_div_scaled_product_mixed_iiai(2029, 2186, 707, A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2183, 1.0);
            s.store_mul_product3_indices(2187, 2182, 757, 2168, 2169, 1.0);
            s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);
        }

        s.b[2293] = (s.v[2027] < 230.25850929940458);
        s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {
            s.copy_ad(2028, 2027);
        }

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt_square_offset(s.ad_value(2188), 1.0), 1.0, 1.0), 1.0);
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {
            s.copy_ad(2189, 2180);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);
        }

        s.b[2294] = (s.v[0] == (-1.0));
        s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });

        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {
            s.store_div_ad_rhs(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2191, 2190);
            s.store_mul_ad_product_rhs_mixed_ia(2192, 2189, 2191, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));
            s.store_scale(2193, 2192, 0.99);
            s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_ad_rhs(2194, 2129, s.ad_value(2193), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2288] && (!s.b[2289])) {
            s.copy_ad(2194, 2177);
        }

        if s.b[2288] {
            s.store_offset(2027, 2110, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);
            s.store_mul(2196, 2195, 2130);
            s.store_add(2197, 2139, 2196);
        }

        s.b[2295] = (s.v[2196] < 460.51701859880916);
        s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2295]) {
            s.store_exp_neg_input(2198, 2196);
        }

        if (s.b[2288] && (!s.b[2295])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2288] {
            s.store_mul(2199, 2154, 2198);
        }

        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2296]) {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2200, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_offset(2260, 2197, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);
        }

        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2246, 2197, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2297] = (s.v[2262] < 230.25850929940458);
        s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });

        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2199, 2248);
        }

        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));
        s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {
            s.store_exp_sub(2248, 2262, 2197);
            s.store_div(2249, 2199, 2248);
        }

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if s.b[2288] {
            s.store_sub(2201, 2200, 2153);
        }

        s.b[2299] = (s.v[2201] < 1e-10);
        s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2299]) {
            s.store_add_scaled_inputs_product_right_ad(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2027, 2.0, 2115, A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));
            s.store_scaled_div_ad_rhs(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2200, 2153, 2201);
        }

        if s.b[2288] {
            s.store_mul(2204, 2201, 2129);
            s.store_div_scaled_product_offset_denominator(2205, s.ad_value(2200), s.ad_value(2200), 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);
        }

        s.b[2300] = (s.v[2200] < 230.25850929940458);
        s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2300]) {
            s.store_exp_neg_input(2206, 2200);
        }

        s.b[2301] = (s.v[2200] < 1e-5);
        s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });

        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);
            s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));
        }

        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {
            s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);
            s.store_sqrt(2208, 2207);
            s.store_mul_add_scaled_inputs3_offset_rhs(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, s.ad_value(2200), (-1.0), s.ad_value(2205), -1.0, (-1.0));
        }

        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));
        s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });

        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {
            s.store_exp_sub(2027, 2200, 2197);
            s.store_div(2206, 2199, 2027);
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

        if (s.b[2288] && (!s.b[2300])) {
            s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);
            s.store_sqrt(2208, 2207);
        }

        if s.b[2288] {
            s.store_mul3_lhs(2210, 2208, 2114, 2129);
            s.store_scaled_add(2211, 2153, 2200, 0.5);
            s.store_scalar(2212, 0.0);
            s.store_mul(2027, 2206, 2159);
        }

        s.b[2303] = (s.v[2027] > 0.0);
        s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2303]) {
            s.store_sqrt(2212, 2027);
        }

        if s.b[2288] {
            s.store_scaled_add(2213, 2160, 2209, 0.5);
            s.store_add_scaled_product_mixed_iaa(2214, 2213, 1.0, A::square(s.ad_value(2201)), A::sub_scaled_inputs(s.ad_value(2212), 1.0, s.ad_value(2131), 2.0), 0.125);
        }

        s.b[2304] = (s.v[2211] < 1e-5);
        s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2304]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2215, 2211, 1.0, 2211, 1.0, 2211, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2305] = (s.v[730] > 0.0);
        s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });

        if ((s.b[2288] && s.b[2304]) && s.b[2305]) {
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
        }

        if (s.b[2288] && s.b[2304]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2211), 1.0, A::scale(s.ad_value(2211), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2219, 2217, A::div_scaled_product(s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), 1.0, A::square(s.ad_value(2211)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2306] = (s.v[730] > 0.0);
        s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });

        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {
            s.store_add_scaled_sub_value_product_indices(2220, 1.0, 2212, 1.0, 2216, 2131, 2.0);
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
            s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2217), 1.0, s.ad_value(2217), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2221, 730, A::square(s.ad_value(2027)), 2115, 2214, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2222, 2216, 2.0, 2221, (-2.0), 2115, A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2223, 2221, s.ad_value(2221), 1.0, s.ad_value(2216), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2224, 1.0, 2115, A::add(s.ad_value(2212), s.ad_value(2214)), 0.5);
            s.store_div_scaled_product_denominator_ad(2225, 2223, 2222, 1.0, A::add_scaled_square_product(s.ad_value(2222), 1.0, s.ad_value(2224), s.ad_value(2223), (-1.0)), 1.0);
            s.store_add(2211, 2211, 2225);
            s.store_exp(2226, 2225);
            s.store_div(2212, 2212, 2226);
            s.store_mul(2214, 2214, 2226);
            s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
            s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::mul3_scaled_output(s.ad_value(2216), s.ad_value(2217), s.ad_value(2131), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2201, 2201, 2226, A::add(s.ad_value(2220), s.ad_value(2213)), 1.0, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2226), s.ad_value(2213), 1.0), 1.0);
            s.store_mul(2204, 2201, 2129);
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_sqrt(2218, 2215);
            s.store_add_scaled_inputs_ad_rhs(2219, 2217, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2212)), s.ad_value(2218)), 0.5);
        }

        if s.b[2288] {
            s.store_mul_div_scaled_product_mixed_iiia(2228, 2129, 2115, 2214, 1.0, A::add_scaled_product(s.ad_value(2216), 1.0, s.ad_value(2114), s.ad_value(2218), 1.0), 1.0);
            s.store_add_scaled_product_indices(2229, 2228, 1.0, 2129, 2219, 1.0);
            s.store_mul3_lhs(2230, 2218, 2114, 2129);
        }

        s.b[2307] = (s.v[218] < 0.0);
        s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2307]) {
            s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2228, 1.0);
        }

        if (s.b[2288] && (!s.b[2307])) {
            s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2228, 1.0);
        }

        if s.b[2288] {
            s.store_mul_product3_indices(2170, 2228, 757, 2168, 2169, 1.0);
            s.store_add_scaled_product_indices(2231, 2230, 1.0, 775, 2228, 1.0);
            s.store_add_scaled_product_indices(2232, 2230, 1.0, 776, 2228, 1.0);
            s.store_mul(2233, 774, 2231);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2215), 1.0, A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2234, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
            s.store_ln_ad(2235, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0), 1.0));
            s.store_mul(2029, 2228, 2174);
            s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2308] = (s.v[222] < 0.0);
        s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });

        if (s.b[2288] && s.b[2308]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if (s.b[2288] && (!s.b[2308])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        if s.b[2288] {
            s.store_mul(2237, 2109, 2176);
            s.store_mul(2236, 2216, 2129);
        }

        s.copy_ad(1855, 2177);

        s.copy_ad(1857, 2195);

        s.copy_ad(1858, 2196);

        s.copy_ad(1859, 2201);

        s.copy_ad(1860, 2204);

        s.copy_ad(1862, 2211);

        s.copy_ad(1861, 2210);

        s.copy_ad(1863, 2217);

        s.copy_ad(1864, 2219);

        s.copy_ad(1865, 2228);

        s.copy_ad(1866, 2229);

        s.copy_ad(1867, 2230);

        s.copy_ad(1868, 2232);

        s.copy_ad(1869, 2234);

        s.copy_ad(1871, 2235);

        s.copy_ad(1870, 2237);

        s.copy_ad(1872, 2236);

        s.copy_ad(1931, 2216);

        s.store_scalar(1873, 1.0);

        s.store_scalar(1874, 1.0);

        s.store_scalar(1876, 1.0);

        s.store_scalar(1877, 1.0);

        s.store_scalar(838, 0.0);

        s.b[2309] = (s.v[1829] > 0.0);
        s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });

        if s.b[2309] {
            s.store_ln_ad(2037, A::offset(A::mul(s.ad_value(830), s.ad_value(779)), 1.0));
            s.store_div_scaled_product_indices(2027, 1824, 1864, 1.0, 1866, 1.0);
            s.store_add_scaled_product_mixed_aai(2036, A::mul3(A::mul3(s.ad_value(227), s.ad_value(1867), s.ad_value(2027)), s.ad_value(2027), s.ad_value(2037)), 1.0, A::div_scaled_product(A::add(s.ad_value(225), A::div(s.ad_value(226), s.ad_value(1866))), s.ad_value(1865), 1.0, s.ad_value(1866), 1.0), 1871, 1.0);
            s.store_div_from_scalar_add_ad(1873, 1.0, A::offset(s.ad_value(2036), 1.0), A::square(s.ad_value(2036)));
            s.store_mul(1874, 1869, 1873);
            s.store_div(1875, 1870, 1874);
            s.store_mul_ad_product_lhs_mixed_ai(2038, A::square(s.ad_value(1875)), 1860, 1860);
        }

        s.b[2310] = (s.v[0] == (-1.0));
        s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });

        if (s.b[2309] && s.b[2310]) {
            s.store_div_scaled_value_offset_denominator(2038, s.ad_value(2038), 1.0, A::mul(s.ad_value(1875), s.ad_value(1860)), 1.0, 1.0);
        }

        if s.b[2309] {
            s.store_mul_offset_rhs_scaled_ad_rhs(2039, 1874, A::sqrt(A::scale_offset(s.ad_value(2038), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1876, 1.0, 2039);
            s.store_mul(2027, 1874, 1876);
            s.store_mul_offset_ad_rhs(2040, 1864, A::mul3_scaled_output(s.ad_value(2038), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_div_scaled_product_indices(1877, 2027, 1866, 1.0, 2040, 1.0);
            s.store_mul_product3_indices(838, 1876, 716, 1866, 1860, 1.0);
        }

        s.store_scalar(2042, 0.0);

        s.store_scalar(2043, 0.0);

        s.store_scalar(1878, 0.0);

        s.store_scalar(1879, 0.0);

        s.b[2311] = (((((p.p40 != 0.0) && ((s.v[237] > 0.0) || (s.v[238] > 0.0))) || ((p.p42 != 0.0) && ((s.v[247] > 0.0) || (s.v[248] > 0.0)))) || (s.v[262] > 0.0)) || (s.v[263] > 0.0));
        s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });

        if s.b[2311] {
            s.store_scaled_add_ad_rhs(2041, 1817, A::sqrt(A::add(A::square(s.ad_value(1817)), s.ad_value(789))), 0.5);
            s.store_add_ad_lhs(2042, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(784), (-0.5), s.ad_value(782), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), 1.0), 791);
            s.store_scaled_add_ad_rhs(2041, 1818, A::sqrt(A::add(A::square(s.ad_value(1818)), s.ad_value(792))), 0.5);
            s.store_add_ad_lhs(2043, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(785), (-0.5), s.ad_value(783), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), 1.0), 794);
            s.store_scaled_add(1878, 1817, 2042, (-s.v[354]));
            s.store_scaled_add(1879, 1818, 2043, (-s.v[354]));
        }

        s.b[2312] = (p.p40 != 0.0);
        s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });

        s.b[2313] = (s.v[237] > 0.0);
        s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1878)), 1e-6), 795);
        }

        s.b[2314] = (s.v[243] < 0.0);
        s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2313]) && s.b[2314]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 801, 0.5, 2044, 801, 1e-6, (-0.5));
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_offset_ad_rhs(2027, 798, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(2044), 1.0)), (-1.5));
            s.store_offset(2046, 2042, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 834, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(2027, 2.0, 818, A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
        }

        s.b[2317] = (s.v[238] > 0.0);
        s.store_scalar(2317, if s.b[2317] { 1.0 } else { 0.0 });

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1879)), 1e-6), 795);
        }

        s.b[2318] = (s.v[245] < 0.0);
        s.store_scalar(2318, if s.b[2318] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2317]) && s.b[2318]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 802, 0.5, 2044, 802, 1e-6, (-0.5));
        }

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_offset_ad_rhs(2027, 799, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(245), s.ad_value(2044), 1.0)), (-1.5));
            s.store_offset(2046, 2043, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 837, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(2027, 2.0, 818, A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
        }

        s.b[2321] = (s.v[236] > 0.0);
        s.store_scalar(2321, if s.b[2321] { 1.0 } else { 0.0 });

        s.b[2322] = (s.v[1829] <= 0.0);
        s.store_scalar(2322, if s.b[2322] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {
            s.store_offset(2027, 777, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 1855, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1858, 1855, 1825, 2027, 1.0, A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))), 1.0);
        }

        s.b[2323] = ((s.v[1859] - s.v[1858]) > (-230.25850929940458));
        s.store_scalar(2323, if s.b[2323] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2321]) && s.b[2323]) {
            s.store_exp_sub(2027, 1859, 1858);
        }

        if ((s.b[2312] && s.b[2321]) && (!s.b[2323])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_add_scaled_product_right_ad(2050, 2030, 1.0, 1824, A::sub_scaled_inputs(s.ad_value(1859), 0.5, A::ln_scaled_input(A::offset(s.ad_value(2027), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(2051, 235, 1824);
            s.store_add(2052, 1872, 2051);
            s.store_scaled_sub_ad_rhs(2053, 2052, A::sqrt_square_offset(A::neg(s.ad_value(2052)), 0.01), 0.5);
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1872)), 1e-6), 795);
        }

        s.b[2324] = (s.v[241] < 0.0);
        s.store_scalar(2324, if s.b[2324] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2321]) && s.b[2324]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 800, 0.5, 2044, 800, 1e-6, (-0.5));
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_add_scaled_product_left_ad(2054, 1862, 1.0, A::add_scaled_inputs3(s.ad_value(2053), 1.0, s.ad_value(742), (-1.0), s.ad_value(2050), -1.0), 1825, 1.0);
            s.store_mul_neg_ad_lhs(2054, A::add_scaled_inputs3(s.ad_value(825), 1.0, s.ad_value(2030), 1.0, s.ad_value(2050), -1.0), 1825);
        }

        s.b[2327] = (((s.v[2054]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2321]) && s.b[2327]) {
            s.store_exp(2027, 2054);
        }

        s.b[2328] = (s.v[2054] < 0.0);
        s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && s.b[2328]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2027, 1e-100, (-230.25850929940458), 2054, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && (!s.b[2328])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2027, 2054, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_mul_offset_ad_rhs(2027, 797, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2331] = ((s.v[1829] <= 0.0) || ((s.v[240] == 0.0) && (s.v[241] == 0.0)));
        s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });

        if ((s.b[2312] && s.b[2321]) && (!s.b[2331])) {
            s.store_add_scaled_product_indices(2027, 240, 1.0, 241, 2044, 2.0);
            s.store_div_ad_rhs(2058, 246, A::mul(s.ad_value(2027), s.ad_value(797)));
            s.store_scaled_div(2059, 1860, 2058, 0.5);
        }

        s.b[2332] = (s.v[2059] < 0.001);
        s.store_scalar(2332, if s.b[2332] { 1.0 } else { 0.0 });

        s.b[2333] = (((s.v[2059]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });

        if ((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {
            s.store_exp(2067, 2059);
        }

        s.b[2334] = (s.v[2059] < 0.0);
        s.store_scalar(2334, if s.b[2334] { 1.0 } else { 0.0 });

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && s.b[2334]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2067, 1e-100, (-230.25850929940458), 2059, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && (!s.b[2334])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2067, 2059, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) {
            s.store_div_from_scalar(2068, 1.0, 2067);
            s.store_sub(2027, 2067, 2068);
            s.store_add(2029, 2067, 2068);
        }

        s.b[2335] = (p.p42 != 0.0);
        s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });

        s.b[2336] = ((s.v[248] > 0.0) && (s.v[1879] < 0.0));
        s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });

        if (s.b[2335] && s.b[2336]) {
            s.store_sqrt_offset_ad(2071, A::add_scaled_square_product(s.ad_value(1879), 1.0, A::square(s.ad_value(254)), A::square(s.ad_value(836)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(2027, 807, -1.0, 2071, 1.0);
        }

        s.b[2337] = (s.v[2027] > (-230.25850929940458));
        s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });

        if ((s.b[2335] && s.b[2336]) && s.b[2337]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2336]) && (!s.b[2337])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2338] = ((s.v[247] > 0.0) && (s.v[1878] < 0.0));
        s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });

        if (s.b[2335] && s.b[2338]) {
            s.store_sqrt_offset_ad(2072, A::add_scaled_square_product(s.ad_value(1878), 1.0, A::square(s.ad_value(253)), A::square(s.ad_value(835)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(2027, 806, -1.0, 2072, 1.0);
        }

        s.b[2339] = (s.v[2027] > (-230.25850929940458));
        s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });

        if ((s.b[2335] && s.b[2338]) && s.b[2339]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2338]) && (!s.b[2339])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_scalar(2076, s.v[715]);

        s.store_scalar(1880, 0.0);

        s.store_scalar(1881, 0.0);

        s.store_scalar(1882, 0.0);

        s.store_scalar(1883, 1e-40);

        s.store_scalar(1884, 1.0);

        s.store_scalar(846, 0.0);

        s.b[2340] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });

        if s.b[2340] {
            s.store_add_scaled_inputs4_mixed_iiai(2027, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(764))), (-0.5), 762, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(2073, 827, 1.0, 2027, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2027)), s.ad_value(763))), (-(-0.5)), 766, 1.0);
            s.store_add_scaled_inputs3_indices(2074, 2073, 1.0, 826, 0.5, 830, (-0.5));
            s.store_mul_ad_product_rhs(2075, 289, A::offset(A::mul(s.ad_value(291), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(290), s.ad_value(2074)), 1.0));
            s.store_mul_offset_rhs(2076, 723, 2075, 1.0);
            s.store_div_from_scalar(2077, 1.0, 2076);
            s.store_div_scaled_value_offset_denominator(2078, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(293), s.ad_value(830), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(2079, 292, 2078, A::offset(A::mul(s.ad_value(294), s.ad_value(2074)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1880, 2077, s.ad_value(829), 1.0, s.ad_value(2079), 1.0, s.ad_value(713), -1.0, 0.0);
            s.store_mul(2080, 2077, 760);
            s.store_scaled_ln_ad(2081, A::add(A::div(s.ad_value(2080), s.ad_value(761)), A::sqrt(s.ad_value(2080))), 2.0);
            s.store_mul(2082, 2077, 2073);
            s.store_add(2087, 2080, 2082);
            s.store_add_scaled_product_right_ad(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);
            s.store_add(2089, 2088, 2081);
            s.store_offset_div_scaled_inputs_mixed_ia(2090, 761, 1.0, A::sqrt(s.ad_value(2087)), 2.0, 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2341] = (s.v[2092] > (-12.0));
        s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });

        if (s.b[2340] && s.b[2341]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2094, 2093, 2093, 10.0, 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_sqrt_square_offset_rhs(2096, 2095, 2095, 2.0, 0.5);
        }

        s.b[2342] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });

        if ((s.b[2340] && s.b[2341]) && s.b[2342]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if ((s.b[2340] && s.b[2341]) && (!s.b[2342])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2340] && s.b[2341]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_indices(2099, 2098, 2091);
            s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);
            s.store_mul_offset_ad_rhs(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), (-1.0));
            s.store_sub(2083, 2096, 2101);
        }

        s.b[2343] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });

        if ((s.b[2340] && (!s.b[2341])) && s.b[2343]) {
            s.store_exp_ad(2083, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if ((s.b[2340] && (!s.b[2341])) && (!s.b[2343])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2083, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if s.b[2340] {
            s.store_mul_add_rhs(2084, 2077, 1857, 2073);
        }

        s.b[2344] = ((s.v[2083] < 0.001) && (s.v[1857] < 1e-6));
        s.store_scalar(2344, if s.b[2344] { 1.0 } else { 0.0 });

        s.b[2345] = (((-s.v[2084]) + s.v[2082]) > (-230.25850929940458));
        s.store_scalar(2345, if s.b[2345] { 1.0 } else { 0.0 });

        if ((s.b[2340] && s.b[2344]) && s.b[2345]) {
            s.store_exp_sub(2027, 2082, 2084);
        }

        if ((s.b[2340] && s.b[2344]) && (!s.b[2345])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2340] && s.b[2344]) {
            s.store_mul_offset_rhs(1881, 2083, 2027, (-1.0));
            s.store_add(2085, 1881, 2083);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_add(2087, 2080, 2084);
            s.store_add_scaled_product_right_ad(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);
            s.store_add(2089, 2088, 2081);
            s.store_offset_div_scaled_inputs_mixed_ia(2090, 761, 1.0, A::sqrt(s.ad_value(2087)), 2.0, 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2346] = (s.v[2092] > (-12.0));
        s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2094, 2093, 2093, 10.0, 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_sqrt_square_offset_rhs(2096, 2095, 2095, 2.0, 0.5);
        }

        s.b[2347] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && s.b[2347]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && (!s.b[2347])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_indices(2099, 2098, 2091);
            s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);
            s.store_mul_offset_ad_rhs(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), (-1.0));
            s.store_sub(2085, 2096, 2101);
        }

        s.b[2348] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.store_scalar(2348, if s.b[2348] { 1.0 } else { 0.0 });

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && s.b[2348]) {
            s.store_exp_ad(2085, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && (!s.b[2348])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2085, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_sub(1881, 2085, 2083);
        }

        if s.b[2340] {
            s.store_scaled_add(1882, 2085, 2083, 0.5);
        }

        if s.b[2340] {
            if ((s.v[1880] - s.v[1882]) > 1e-40) {
                s.store_sub(1883, 1880, 1882);
            } else {
                s.store_scalar(1883, 1e-40);
            }
        }

        if s.b[2340] {
            s.store_sub_from_scalar_ad(1884, 1.0, A::div_scaled_inputs(s.ad_value(761), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1883), 1.0, s.ad_value(724), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(846, A::mul3_scaled_output(s.ad_value(717), s.ad_value(2076), s.ad_value(2076), -1.0), A::offset(A::mul(s.ad_value(1884), s.ad_value(1882)), 1.0), 1881, 1.0, 1869, 1.0);
        }

        s.store_scalar(1885, 0.0);

        s.store_scalar(847, 0.0);

        s.b[2349] = ((s.v[1829] > 0.0) && (p.p41 != 0.0));
        s.store_scalar(2349, if s.b[2349] { 1.0 } else { 0.0 });

        if s.b[2349] {
            s.store_add_scaled_product_indices(2086, 826, 1.0, 232, 1860, (-1.0));
        }

        s.b[2350] = (s.v[2086] > 0.0);
        s.store_scalar(2350, if s.b[2350] { 1.0 } else { 0.0 });

        if (s.b[2349] && s.b[2350]) {
            s.store_mul_div_scaled_offset_numerator_rhs(2029, 712, A::mul(s.ad_value(233), A::sub(A::sqrt(A::add(s.ad_value(728), s.ad_value(2030))), s.ad_value(736))), 1.0, 1.0, A::offset(s.ad_value(2086), 1e-30), 1.0);
        }

        s.b[2351] = ((((-s.v[2029])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2351, if s.b[2351] { 1.0 } else { 0.0 });

        if ((s.b[2349] && s.b[2350]) && s.b[2351]) {
            s.store_exp_neg_input(2027, 2029);
        }

        s.b[2352] = ((-s.v[2029]) < 0.0);
        s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && s.b[2352]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2029)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && (!s.b[2352])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2029)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2349] && s.b[2350]) {
            s.store_mul3_lhs(1885, 229, 2086, 2027);
            s.store_mul_add_rhs(847, 1885, 838, 846);
        }

        s.b[2353] = (s.v[847] > (0.5 * s.v[234]));
        s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2349] && s.b[2350]) && s.b[2353]) {
            s.store_offset_div_scaled_inputs_indices(2027, 847, 2.0, 234, 1.0, (-1.0));
            s.store_mul_scaled_offset_ad_rhs(847, 234, 0.5, A::div(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 1.0)), 1.0);
        }

        s.b[2547] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.store_scalar(2547, if s.b[2547] { 1.0 } else { 0.0 });

        s.b[2548] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.store_scalar(2548, if s.b[2548] { 1.0 } else { 0.0 });

        if (s.b[2547] && s.b[2548]) {
            s.copy_ad(2388, 728);
            s.copy_ad(2389, 738);
            s.copy_ad(2390, 729);
            s.copy_ad(2391, 1820);
            s.copy_ad(2392, 1821);
            s.store_scalar(2396, 0.0);
        }

        s.b[2549] = (p.p47 > 0.0);
        s.store_scalar(2549, if s.b[2549] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2549]) {
            s.store_add_scaled_inputs4_mixed_iiai(2391, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(749))), (-0.5), 747, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1886, 827, 1.0, 2391, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2391)), s.ad_value(748))), (-(-0.5)), 750, 1.0);
            s.copy_ad(2392, 1886);
            s.copy_ad(2388, 745);
            s.copy_ad(2389, 748);
            s.copy_ad(2390, 746);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_inputs3_indices(2395, 829, 1.0, 2396, (-1.0), 700, -1.0);
            s.store_add_scaled_inputs3_indices(2397, 2392, 1.0, 826, 0.5, 830, (-0.5));
            s.store_scalar(2409, 1.0);
        }

        s.b[2550] = (s.v[190] > 0.0);
        s.store_scalar(2550, if s.b[2550] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {
            s.store_scale(2400, 2388, s.v[361]);
            s.store_scale(2401, 2397, s.v[361]);
            s.store_scale(2402, 2395, s.v[361]);
            s.store_offset_div_scaled_inputs_mixed_ia(2028, 2390, 0.5, A::sqrt(s.ad_value(2400)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2400, 1.0, 2390, A::sqrt(s.ad_value(2400)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2403, A::div_scaled_inputs2(s.ad_value(2402), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2400, 0.5, A::offset(s.ad_value(191), 1.0), 2401, (-1.0));
            s.store_offset_scaled(2404, 2400, 0.5, 2.0);
            s.store_add(2405, 2400, 2401);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2402), 1.0, s.ad_value(2405), (-1.0), s.ad_value(2390), A::sqrt(s.ad_value(2405)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0);
            s.store_add_scaled_inputs(2406, 2028, 2.0, 2404, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2403, 0.5, 2406, 0.5, 2403, 2406, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(2029, 2402, 2.0, 2401, (-2.0), 2404, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2407, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2407, 0.5, 2404, 0.5, 2407, 2404, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2408, 2028, 0.5, 2404, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2408), s.ad_value(2404)), 1.0);
        }

        s.b[2551] = (s.v[2029] > (-230.25850929940458));
        s.store_scalar(2551, if s.b[2551] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && s.b[2551]) {
            s.store_exp(2409, 2029);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && (!s.b[2551])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2409, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_mul(2410, 701, 2409, 1.0);
            s.store_scale(2411, 2410, s.v[715]);
            s.store_mul_ad_product_rhs(2412, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));
            s.store_mul_offset_rhs(2413, 2411, 2412, 1.0);
            s.store_div_from_scalar(2414, 1.0, 2413);
            s.store_mul_ad_rhs(2398, 2390, A::sqrt_scaled_input(s.ad_value(2414), s.v[715]));
            s.store_square(2399, 2398);
            s.store_div_from_scalar(2415, 1.0, 2399);
            s.store_mul(2416, 2392, 2414);
            s.store_mul(2417, 2395, 2414);
            s.store_div_scaled_value_offset_denominator(2418, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(2419, 196, 2418, A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));
            s.store_mul(2420, 2388, 2414);
            s.store_sqrt_square_add(2028, 2391, 2389);
            s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2391), s.ad_value(2419))), s.ad_value(2389));
            s.store_mul_add_scaled_inputs3_offset_rhs(2421, 2414, s.ad_value(2419), 0.5, s.ad_value(2028), 0.5, s.ad_value(2029), ((-1.0) * (0.5)), 0.0);
            s.store_add(2422, 2420, 2416);
            s.store_sub(2423, 2422, 2421);
        }

        s.b[2552] = (p.p45 > 0.0);
        s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });

        s.b[2553] = (((s.v[2423]) as f64).abs() < 1e-5);
        s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && s.b[2553]) {
            s.store_offset_ad(2424, A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2423), 1.0, A::scale(s.ad_value(2423), 0.3125), 0.5)), 1.0);
        }

        s.b[2554] = (s.v[2423] < 460.51701859880916);
        s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && s.b[2554]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && (!s.b[2554])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_offset_ad(2424, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2398), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2423))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2423), 1.0, s.ad_value(2438))), 2.0), 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2552])) {
            s.store_offset_div_scaled_inputs_mixed_ia(2424, 2398, 0.5, A::sqrt(s.ad_value(2423)), 1.0, 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_value_products(2425, s.ad_value(2423), 1.0, s.ad_value(2398), A::sqrt(s.ad_value(2423)), 1.0, s.ad_value(2424), A::ln(A::offset(s.ad_value(2424), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2_indices(2426, 2417, 1.0, 2425, (-1.0), 2424, 1.0);
            s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0));
            s.store_scalar(2431, 0.0);
            s.store_scalar(2433, 1.0);
        }

        s.b[2555] = (s.v[2426] > (-30.0));
        s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_offset_mul(2427, 2424, 2426, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2427, 2427, 10.0, 0.5);
            s.store_sub_ad_rhs(2428, 2426, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2429, 2428, 2428, 2.0, 0.5);
        }

        s.b[2556] = ((s.v[2426] - s.v[2429]) < 230.25850929940458);
        s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2556]) {
            s.store_exp_sub(2027, 2426, 2429);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2556])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_div(2430, 2027, 2424);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2429), 1.0, 2.0), 2430);
        }

        s.b[2557] = (s.v[2430] > 1e-6);
        s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2557]) {
            s.store_mul_offset_ad_rhs(2431, 2424, A::sub(s.ad_value(2429), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2430), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2430), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2557])) {
            s.store_mul_ad_affine_product_rhs(2431, 2424, s.ad_value(2430), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2417, 0.5, 2431, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2433, 2432, 2432, 1.0, 2431, 1.0);
            s.store_add_scaled_product_indices(2423, 2422, 1.0, 2433, 2421, (-1.0));
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);
            s.store_scale(2435, 2434, 1e-5);
            s.store_div_from_scalar(2436, 1.0, 2434);
            s.store_scalar(2543, 0.0);
            s.store_scalar(2437, 0.0);
        }

        s.b[2558] = (s.v[2423] < 460.51701859880916);
        s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2558]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2558])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2559] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2559]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2437, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2438)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        s.b[2560] = (s.v[2417] < (-s.v[2435]));
        s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_neg(2525, 2417);
            s.store_scaled_mul(2526, 2525, 2436, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2527, 2526, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2522, 2525, 2527);
            s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::offset(s.ad_value(2527), 1.0), 1.0);
            s.store_sub_scaled_inputs(2529, 2522, 2.0, 2399, 1.0);
            s.store_sub_ln_mul_lhs(2530, 2528, 2415, 2527);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.5, s.ad_value(2528), 1.0), 1.0);
            s.store_add_ad_rhs(2531, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.3333333333333333, s.ad_value(2528), 1.0))), 1.0));
        }

        s.b[2561] = (s.v[2531] < 230.25850929940458);
        s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && s.b[2561]) {
            s.store_exp(2532, 2531);
        }

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && (!s.b[2561])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2532, 2531, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_div_from_scalar_offset_square(2522, 1.0, 2531, 2.0);
            s.store_mul_square_lhs(2534, 2531, 2522);
            s.store_mul3_affine_lhs(2535, 2531, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);
            s.store_sub(2522, 2525, 2531);
            s.store_mul(2523, 2438, 2533);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2523), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2535)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2531), (-1.0), s.ad_value(2523), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2532), 1.0, s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2437, 2531, -1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_scaled_input(2539, 1.0, 2398, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2540, A::mul_scaled_lhs(s.ad_value(2434), 1.25, s.ad_value(2539)), (-1.0), 2539);
            s.store_mul_ad_product_rhs_mixed_ia(2541, 2417, 2436, A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));
        }

        s.b[2562] = ((-s.v[2541]) > (-230.25850929940458));
        s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2562]) {
            s.store_exp_neg_input(2522, 2541);
        }

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2562])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2522, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2541)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar(2542, 1.0, 2522);
            s.store_add_scaled_inputs_product_right_ad(2543, 2417, 1.0, 2399, 0.5, 2398, A::sqrt(A::add_scaled_inputs3(s.ad_value(2417), 1.0, s.ad_value(2399), 0.25, s.ad_value(2542), -1.0)), (-1.0));
            s.store_offset(2544, 2423, 3.0);
            s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2543), s.ad_value(2544)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt_square_offset(s.ad_value(2544), 5.0), 0.5));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_square(2524, 1.0, 2527, 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), 2524, 2524);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2545, 1.0, 2399, A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2530, 2423, 1.0, 2527, (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2563] = (s.v[2546] < 230.25850929940458);
        s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2563]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2438, 2532);
        }

        s.b[2564] = (s.v[2546] > (s.v[2423] - 230.25850929940458));
        s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && s.b[2564]) {
            s.store_exp_sub(2532, 2546, 2423);
            s.store_div(2533, 2438, 2532);
        }

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && (!s.b[2564])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2437, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_scalar(2440, 0.0);
            s.store_scalar(2441, 0.0);
            s.store_scalar(2442, 0.0);
            s.store_scalar(2443, 0.0);
            s.store_scalar(2444, 0.0);
            s.store_scalar(2445, 0.0);
            s.store_scalar(2446, 0.0);
            s.store_scalar(2447, 1.0);
            s.store_scalar(2448, 1.0);
            s.store_sub(2449, 2417, 2437);
            s.store_scalar(2450, 0.0);
            s.store_mul(2451, 2413, 2449);
            s.store_scalar(2452, 1.0);
            s.store_scalar(2453, 1.0);
            s.store_scalar(2457, 1.0);
            s.store_scalar(2458, 1.0);
            s.store_scalar(2460, 1.0);
        }

        s.b[2565] = (s.v[2417] > 0.0);
        s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_from_scalar_offset_square(2027, 1.0, 2437, 2.0);
            s.store_mul_square_lhs(2439, 2437, 2027);
            s.store_mul3_affine_lhs(2440, 2437, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs_mixed_ai(2441, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2439), 12.0), 2027, 2027);
            s.store_scalar(2442, 0.0);
        }

        s.b[2566] = (s.v[2437] < 230.25850929940458);
        s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2566]) {
            s.store_exp(2442, 2437);
            s.store_div_from_scalar(2443, 1.0, 2442);
            s.store_mul(2442, 2438, 2442);
        }

        s.b[2567] = (s.v[2437] > (s.v[2423] - 230.25850929940458));
        s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {
            s.store_exp_sub(2442, 2437, 2423);
            s.store_div(2443, 2438, 2442);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2442, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2443, 1e-100, 2437, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_add_scaled_product_right_ad(2444, 2442, 1.0, 2438, A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439)), (-1.0));
        }

        s.b[2568] = (s.v[2437] < 1e-5);
        s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2568]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2445, 2437, 1.0, 2437, 1.0, 2437, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2444, A::mul3(s.ad_value(2438), s.ad_value(2437), s.ad_value(2437)), 2437, A::scale_offset(s.ad_value(2437), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2437), 1.0, A::scale(s.ad_value(2437), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);
            s.store_offset_div_scaled_product(2447, s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), 1.0, A::square(s.ad_value(2437)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2568])) {
            s.store_add_offset_lhs(2445, 2437, (-1.0), 2443);
            s.store_sqrt(2446, 2445);
            s.store_offset_scaled_ad(2447, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2443)), s.ad_value(2446)), 0.5, 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_scaled_offset_numerator(2448, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2397)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0), 1.0);
        }

        s.b[2569] = (s.v[2444] > 1e-100);
        s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_sqrt_ad_rhs(2449, 2398, A::add(s.ad_value(2445), s.ad_value(2444)));
            s.store_div_scaled_product3_mixed_iiia(2450, 2399, 2444, 2413, 1.0, A::add_scaled_product(s.ad_value(2449), 1.0, s.ad_value(2398), s.ad_value(2446), 1.0), 1.0);
            s.store_mul3_lhs(2451, 2446, 2398, 2413);
        }

        s.b[2570] = (s.v[217] < 0.0);
        s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2570]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2452, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2570])) {
            s.store_offset_mul(2452, 217, 2397, 1.0);
        }

        s.b[2571] = (s.v[218] < 0.0);
        s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2571]) {
            s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2450, 1.0);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2571])) {
            s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2450, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_product3_indices(2454, 2450, 757, 2452, 2453, 1.0);
            s.store_mul_add_scaled_product_rhs(2455, 774, s.ad_value(2451), 1.0, s.ad_value(775), s.ad_value(2450), 1.0);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2445), 1.0, A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2457, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
        }

        s.b[2572] = (s.v[221] < 0.0);
        s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2572]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2458, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2572])) {
            s.store_offset_mul(2458, 221, 2397, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul(2029, 2450, 2458);
            s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2573] = (s.v[222] < 0.0);
        s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2573]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2573])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && (!s.b[2548])) {
            s.copy_ad(2395, 1822);
            s.copy_ad(2397, 1823);
            s.copy_ad(2413, 1824);
            s.copy_ad(2414, 1825);
            s.copy_ad(2398, 1826);
            s.copy_ad(2399, 1827);
            s.copy_ad(2415, 1828);
            s.copy_ad(2417, 1829);
            s.copy_ad(2422, 1830);
            s.copy_ad(2423, 1831);
            s.copy_ad(2434, 1832);
            s.copy_ad(2435, 1833);
            s.copy_ad(2436, 1834);
            s.copy_ad(2543, 1835);
            s.copy_ad(2438, 1836);
            s.copy_ad(2437, 1837);
            s.copy_ad(2440, 1838);
            s.copy_ad(2441, 1839);
            s.copy_ad(2442, 1840);
            s.copy_ad(2443, 1841);
            s.copy_ad(2445, 1842);
            s.copy_ad(2444, 1843);
            s.copy_ad(2446, 1844);
            s.copy_ad(2447, 1845);
            s.copy_ad(2448, 1846);
            s.copy_ad(2449, 1847);
            s.copy_ad(2450, 1848);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2547] && (!s.b[2548])) {
            s.copy_ad(2451, 1849);
            s.copy_ad(2452, 1850);
            s.copy_ad(2453, 1851);
            s.copy_ad(2457, 1852);
            s.copy_ad(2458, 1853);
            s.copy_ad(2460, 1854);
        }

        if s.b[2547] {
            s.copy_ad(2393, 720);
            s.copy_ad(2394, 777);
        }

        s.b[2574] = (p.p48 != 0.0);
        s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });

        if (s.b[2547] && s.b[2574]) {
            s.copy_ad(2393, 721);
            s.copy_ad(2394, 778);
        }

        if s.b[2547] {
            s.store_scalar(2462, 0.0);
            s.store_scale(2461, 2413, 4.60517018598809);
            s.copy_ad(2478, 2461);
            s.copy_ad(2479, 826);
            s.store_mul(2480, 826, 2414);
            s.copy_ad(2484, 2437);
            s.store_scalar(2485, 0.0);
            s.store_scalar(2488, 0.0);
            s.copy_ad(2490, 2443);
            s.copy_ad(2491, 2445);
            s.copy_ad(2493, 2444);
            s.copy_ad(2494, 2451);
            s.copy_ad(2495, 2437);
            s.copy_ad(2496, 2443);
            s.copy_ad(2498, 2444);
            s.copy_ad(2499, 2445);
            s.store_sub(2500, 2417, 2437);
            s.store_scalar(2501, 1.0);
            s.store_scalar(2503, 1.0);
            s.store_scalar(2502, 0.0);
            s.copy_ad(2512, 2450);
            s.store_mul(2516, 2500, 2413);
            s.store_scalar(2513, 0.0);
            s.copy_ad(2514, 2451);
            s.store_scalar(2519, 0.0);
            s.store_scalar(2518, 1.0);
            s.copy_ad(2521, 2393);
            s.copy_ad(2520, 2516);
        }

        s.b[2575] = (s.v[2417] > 0.0);
        s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });

        s.b[2576] = (s.v[2444] > 1e-100);
        s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2521, 2393, 2460);
            s.store_div(2462, 2521, 2457);
            s.store_add_scaled_inputs(2463, 2449, 1.0, 2399, 0.5);
            s.store_div_scaled_product_by_product(2027, s.ad_value(2399), s.ad_value(2442), 1.0, s.ad_value(2463), s.ad_value(2463), 1.0);
        }

        s.b[2577] = (s.v[2027] > 0.0001);
        s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2578] = (s.v[2028] < 1e-10);
        s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_scalar(2029, 1.0);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2577])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2464, 2029, 2463);
        }

        s.b[2579] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_scaled_mul(2465, 2413, 2464, 0.475);
            s.store_add_scaled_product_indices(2027, 2450, 1.0, 2447, 2465, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2466, 2027, 2027, 1e-12, 0.5);
            s.store_add_scaled_value_products(2467, s.ad_value(2450), (-1.0), s.ad_value(2413), s.ad_value(2449), 1.0, A::offset(s.ad_value(2447), (-1.0)), s.ad_value(2465), 1.0);
            s.store_offset_div_scaled_product(2468, s.ad_value(2399), s.ad_value(2413), 0.5, s.ad_value(2467), 1.0, 1.0);
            s.store_add_scaled_product_indices(2027, 2467, 1.0, 775, 2466, 1.0);
            s.store_pow_ad(2469, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2468), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2469);
            s.store_div(2027, 2466, 2467);
            s.store_mul_pow_ad_rhs(2470, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_div_scaled_product_mixed_iiai(2029, 2470, 707, A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2467, 1.0);
            s.store_mul_product3_indices(2471, 2466, 757, 2452, 2453, 1.0);
            s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), s.ad_value(2468), (-1.0), s.ad_value(2029), 1.0), 1.0);
        }

        s.b[2580] = (s.v[2027] < 230.25850929940458);
        s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && s.b[2580]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && (!s.b[2580])) {
            s.copy_ad(2028, 2027);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_div_scaled_product3_mixed_iiia(2472, 2465, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2469), 1.0, s.ad_value(2470), 1.0, s.ad_value(2471), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2473, 2464, A::div_scaled_value_offset_denominator(s.ad_value(2472), 1.0, A::sqrt_square_offset(s.ad_value(2472), 1.0), 1.0, 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2579])) {
            s.copy_ad(2473, 2464);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul3_affine_lhs(2474, 2413, 2462, 0.7071067811865475, 0.0, 2473);
        }

        s.b[2581] = (s.v[0] == (-1.0));
        s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2581]) {
            s.store_div_ad_rhs(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar_offset_ad(2475, 2.0, A::sqrt(A::scale_offset(s.ad_value(2474), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2475, 2474);
            s.store_mul_ad_product_rhs_mixed_ia(2476, 2473, 2475, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2475)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2475), 4.0), 1.0)), 1.0));
            s.store_scale(2477, 2476, 0.99);
            s.store_div_scaled_product3_mixed_iaii(2027, 2477, A::sub_scaled_inputs(s.ad_value(2477), 1.0, s.ad_value(2463), 2.0), 2415, 1.0, 2444, 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul_sub_ad_rhs(2478, 2413, s.ad_value(2477), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2576])) {
            s.copy_ad(2478, 2461);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_offset(2027, 2394, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2478, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2479, 2478, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);
            s.store_mul(2480, 2479, 2414);
            s.store_add(2481, 2423, 2480);
        }

        s.b[2582] = (s.v[2480] < 460.51701859880916);
        s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2582]) {
            s.store_exp_neg_input(2482, 2480);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2582])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2482, 1e-200, 2480, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2483, 2438, 2482);
        }

        s.b[2583] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2583]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2484, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2483)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_offset(2544, 2481, 3.0);
            s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2543), s.ad_value(2544)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt_square_offset(s.ad_value(2544), 5.0), 0.5));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_square(2524, 1.0, 2527, 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), 2524, 2524);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2545, 1.0, 2399, A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2530, 2481, 1.0, 2527, (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2584] = (s.v[2546] < 230.25850929940458);
        s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2483, 2532);
        }

        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));
        s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {
            s.store_exp_sub(2532, 2546, 2481);
            s.store_div(2533, 2483, 2532);
        }

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2484, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_sub(2485, 2484, 2437);
        }

        s.b[2586] = (s.v[2485] < 1e-10);
        s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {
            s.store_add_scaled_inputs_product_right_ad(2486, 2417, 2.0, 2437, (-2.0), 2399, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2440), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2027, 2.0, 2399, A::add_scaled_value_products(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2027, 2486, 1.0, 2027, 2487, (-2.0));
            s.store_scaled_div_ad_rhs(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2484, 2437, 2485);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2488, 2485, 2413);
            s.store_div_scaled_product_offset_denominator(2489, s.ad_value(2484), s.ad_value(2484), 1.0, A::square(s.ad_value(2484)), 2.0, 1.0);
        }

        s.b[2587] = (s.v[2484] < 230.25850929940458);
        s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {
            s.store_exp_neg_input(2490, 2484);
        }

        s.b[2588] = (s.v[2484] < 1e-5);
        s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2491, 2484, 1.0, 2484, 1.0, 2484, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);
            s.store_mul3_ad_middle(2493, A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), 2484, A::scale_offset(s.ad_value(2484), 1.75, 1.0));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {
            s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);
            s.store_sqrt(2492, 2491);
            s.store_mul_add_scaled_inputs3_offset_rhs(2493, 2483, A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, s.ad_value(2484), (-1.0), s.ad_value(2489), -1.0, (-1.0));
        }

        s.b[2589] = (s.v[2484] > (s.v[2481] - 230.25850929940458));
        s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && s.b[2589]) {
            s.store_exp_sub(2027, 2484, 2481);
            s.store_div(2490, 2483, 2027);
            s.store_add_scaled_product_right_ad(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));
        }

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && (!s.b[2589])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2490, 1e-100, 2484, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2587])) {
            s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);
            s.store_sqrt(2492, 2491);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul3_lhs(2494, 2492, 2398, 2413);
            s.store_scaled_add(2495, 2437, 2484, 0.5);
            s.store_scalar(2496, 0.0);
            s.store_mul(2027, 2490, 2443);
        }

        s.b[2590] = (s.v[2027] > 0.0);
        s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2590]) {
            s.store_sqrt(2496, 2027);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_scaled_add(2497, 2444, 2493, 0.5);
            s.store_add_scaled_product_mixed_iaa(2498, 2497, 1.0, A::square(s.ad_value(2485)), A::sub_scaled_inputs(s.ad_value(2496), 1.0, s.ad_value(2415), 2.0), 0.125);
        }

        s.b[2591] = (s.v[2495] < 1e-5);
        s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2499, 2495, 1.0, 2495, 1.0, 2495, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2592] = (s.v[730] > 0.0);
        s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && s.b[2591]) && s.b[2592]) {
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2593] = (s.v[730] > 0.0);
        s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });

        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {
            s.store_add_scaled_sub_value_product_indices(2504, 1.0, 2496, 1.0, 2500, 2415, 2.0);
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
            s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2501), 1.0, s.ad_value(2501), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2505, 730, A::square(s.ad_value(2027)), 2399, 2498, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2506, 2500, 2.0, 2505, (-2.0), 2399, A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2507, 2505, s.ad_value(2505), 1.0, s.ad_value(2500), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2508, 1.0, 2399, A::add(s.ad_value(2496), s.ad_value(2498)), 0.5);
            s.store_div_scaled_product_denominator_ad(2509, 2507, 2506, 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0);
            s.store_add(2495, 2495, 2509);
            s.store_exp(2510, 2509);
            s.store_div(2496, 2496, 2510);
            s.store_mul(2498, 2498, 2510);
            s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
            s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2485, 2485, 2510, A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0);
            s.store_mul(2488, 2485, 2413);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_sqrt(2502, 2499);
            s.store_add_scaled_inputs_ad_rhs(2503, 2501, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_div_scaled_product_mixed_iiia(2512, 2413, 2399, 2498, 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0);
            s.store_add_scaled_product_indices(2513, 2512, 1.0, 2413, 2503, 1.0);
            s.store_mul3_lhs(2514, 2502, 2398, 2413);
        }

        s.b[2594] = (s.v[218] < 0.0);
        s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {
            s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2512, 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {
            s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2512, 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_product3_indices(2454, 2512, 757, 2452, 2453, 1.0);
            s.store_add_scaled_product_indices(2515, 2514, 1.0, 775, 2512, 1.0);
            s.store_add_scaled_product_indices(2516, 2514, 1.0, 776, 2512, 1.0);
            s.store_mul(2517, 774, 2515);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2499), 1.0, A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2518, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
            s.store_ln_ad(2519, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0), 1.0));
            s.store_mul(2029, 2512, 2458);
            s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2595] = (s.v[222] < 0.0);
        s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });

        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2521, 2393, 2460);
            s.store_mul(2520, 2500, 2413);
        }

        if s.b[2547] {
            s.copy_ad(1887, 2395);
            s.copy_ad(1888, 2413);
            s.copy_ad(1889, 2398);
            s.copy_ad(1890, 2417);
            s.copy_ad(1891, 2422);
            s.copy_ad(1892, 2451);
            s.copy_ad(1893, 2488);
            s.copy_ad(1894, 2494);
            s.copy_ad(1895, 2501);
            s.copy_ad(1896, 2503);
            s.copy_ad(1897, 2512);
            s.copy_ad(1898, 2513);
            s.copy_ad(1899, 2516);
            s.copy_ad(1900, 2518);
            s.copy_ad(1901, 2519);
            s.copy_ad(1902, 2521);
            s.copy_ad(1903, 2520);
            s.copy_ad(1932, 2414);
            s.copy_ad(1933, 2435);
            s.copy_ad(1934, 2495);
            s.copy_ad(1935, 2500);
        }

        if (!s.b[2547]) {
            s.copy_ad(745, 728);
            s.copy_ad(1887, 1822);
            s.copy_ad(1888, 1824);
            s.copy_ad(1889, 1826);
            s.copy_ad(1890, 1829);
            s.copy_ad(1891, 1830);
            s.copy_ad(1892, 1849);
            s.copy_ad(1893, 1860);
            s.copy_ad(1894, 1861);
            s.copy_ad(1895, 1863);
            s.copy_ad(1896, 1864);
            s.copy_ad(1897, 1865);
            s.copy_ad(1898, 1866);
            s.copy_ad(1899, 1868);
            s.copy_ad(1900, 1869);
            s.copy_ad(1901, 1871);
            s.copy_ad(1902, 1870);
            s.copy_ad(1903, 1872);
            s.copy_ad(1932, 1825);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[2547]) {
            s.copy_ad(1933, 1833);
            s.copy_ad(1934, 1862);
            s.copy_ad(1935, 1931);
        }

        s.copy_ad(1904, 255);

        s.b[2596] = (s.v[773] > 0.0);
        s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });

        if s.b[2596] {
            s.store_div_scaled_value_offset_denominator(1904, s.ad_value(255), 1.0, A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.store_scalar(1905, 1.0);

        s.store_scalar(1906, 1.0);

        s.store_scalar(1907, 0.0);

        s.store_scalar(1908, 1.0);

        s.store_scalar(1909, 1.0);

        s.copy_ad(2359, 1903);

        s.store_scalar(2362, 0.0);

        s.store_scalar(2361, 0.0);

        s.copy_ad(2363, 2359);

        s.b[2597] = (s.v[1890] > 0.0);
        s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });

        if s.b[2597] {
            s.store_mul_div_scaled_product_mixed_iaii(2354, 1901, A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), 1897, 1.0, 1898, 1.0);
        }

        s.b[2598] = (s.v[2354] > 0.0);
        s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });

        if (s.b[2597] && s.b[2598]) {
            s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));
        }

        if (s.b[2597] && (!s.b[2598])) {
            s.store_sub_from_scalar(1905, 1.0, 2354);
        }

        if s.b[2597] {
            s.store_mul(1906, 1900, 1905);
            s.store_div(1907, 1902, 1906);
            s.store_mul_ad_product_lhs_mixed_ai(2355, A::square(s.ad_value(1907)), 1893, 1893);
        }

        s.b[2599] = (s.v[0] == (-1.0));
        s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });

        if (s.b[2597] && s.b[2599]) {
            s.store_div_scaled_value_offset_denominator(2355, s.ad_value(2355), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2597] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1908, 1906, A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(2027, 1906, 1908);
            s.store_mul_offset_ad_rhs(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_div_scaled_product_indices(1909, 2027, 1898, 1.0, 2356, 1.0);
            s.store_scaled_div(2357, 1893, 1909, 0.5);
            s.store_square(2358, 2357);
            s.store_add_product3_rhs_mixed_iia(2359, 1903, 1895, 1893, A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5);
            s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);
        }

        s.b[2600] = (p.p49 == 1.0);
        s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });

        if (s.b[2597] && s.b[2600]) {
            s.store_scalar(2360, 0.0);
            s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);
        }

        if (s.b[2597] && (!s.b[2600])) {
            s.store_mul_sub_from_scalar_lhs_ad_rhs(2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5)));
            s.store_add_scaled_products_mixed_aaia(2361, A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, 2360, A::offset(s.ad_value(1905), 1.0), 0.5);
        }

        if s.b[2597] {
            s.store_add_scaled_product_right_ad(2362, 2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0);
            s.store_sub(2363, 2359, 2362);
        }

        s.store_mul(851, 2359, 1904);

        s.store_mul_neg_lhs(853, 2361, 1904);

        s.store_mul_neg_lhs(852, 2363, 1904);

        s.store_scalar(2379, 0.0);

        s.store_scalar(2380, 0.0);

        s.store_scalar(2378, 0.0);

        s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));
        s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });

        if s.b[2601] {
            s.store_scalar(2368, 1.0);
            s.copy_ad(2367, 1887);
        }

        s.b[2602] = (s.v[272] > 1e-10);
        s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2602]) {
            s.store_add_scaled_inputs3_indices(2364, 1887, 1.0, 270, (-1.0), 808, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2027, 2364, 0.5, 808, 0.5, A::add(A::square(A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)), 0.5);
            s.store_mul_add_scaled_inputs3_offset_rhs(2028, 2027, s.ad_value(2027), 2.0, s.ad_value(808), (-1.0), s.ad_value(2364), -1.0, 0.0);
            s.store_div(2029, 808, 2027);
            s.store_mul(2365, 2364, 2029);
            s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));
            s.store_add_scaled_inputs3_mixed_aii(2367, A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, 2364, 1.0, 2365, -1.0);
            s.store_offset_ad(2368, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add_scaled_product(s.ad_value(2028), 1.0, s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027)), 1.0), s.ad_value(2029), 1.0, s.ad_value(2028), 1.0), 1.0);
        }

        if s.b[2601] {
            s.store_scalar(2370, 1.0);
            s.store_scalar(2371, 0.0);
        }

        s.b[2603] = (s.v[271] > 0.0);
        s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2603]) {
            s.store_add_scaled_product_right_ad(2027, 745, 0.5, 1888, A::scale_offset(s.ad_value(1889), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2369, 1887, 2027);
        }

        s.b[2604] = (((s.v[2369]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });

        if ((s.b[2601] && s.b[2603]) && s.b[2604]) {
            s.store_div_from_scalar_offset_ad(2370, 1.0, A::exp_scaled_input(s.ad_value(2369), -1.0), 1.0);
        }

        s.b[2605] = (s.v[2369] < 0.0);
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if (((s.b[2601] && s.b[2603]) && (!s.b[2604])) && s.b[2605]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2370, 1e-100, 2369, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2606] = (s.v[2369] < 230.25850929940458);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        if ((s.b[2601] && s.b[2603]) && s.b[2606]) {
            s.store_ln_one_plus_exp(2028, 2369);
        }

        if ((s.b[2601] && s.b[2603]) && (!s.b[2606])) {
            s.copy_ad(2028, 2369);
        }

        if (s.b[2601] && s.b[2603]) {
            s.store_mul(2371, 2027, 2028);
        }

        if s.b[2601] {
            s.store_add_scaled_product_right_sub(2372, 2368, 1.0, 271, 2370, 2368, 1.0);
            s.store_add_scaled_product_right_sub(2373, 2367, 1.0, 271, 2371, 2367, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(2374, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1888), s.ad_value(1891), (-1.0)), 1.0, 1903, (-1.0), 1893, (-0.5));
            s.store_add_scaled_inputs3_indices(2375, 1887, 1.0, 2374, (-1.0), 1892, -1.0);
            s.store_add_scaled_inputs3_indices(2376, 1893, 1.0, 2374, 1.0, 826, -1.0);
            s.store_add_scaled_inputs3_indices(2377, 1887, 1.0, 2376, (-1.0), 1894, -1.0);
        }

        s.b[2607] = (s.v[831] > 0.0);
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2607]) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(269), s.ad_value(2376), 1.0, s.ad_value(268), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 268, 2375, 2373);
            s.store_mul_sub_rhs(2380, 269, 2377, 2373);
        }

        if (s.b[2601] && (!s.b[2607])) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(268), s.ad_value(2376), 1.0, s.ad_value(269), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 269, 2375, 2373);
            s.store_mul_sub_rhs(2380, 268, 2377, 2373);
        }

        if s.b[2601] {
            s.store_add(851, 851, 2378);
            s.store_add(853, 853, 2380);
            s.store_add_scaled_inputs4_indices(852, 852, 1.0, 2378, (-1.0), 2380, -1.0, 2379, -1.0);
        }

        s.store_mul(1910, 262, 1878);

        s.store_mul(1911, 263, 1879);

        s.store_scalar(2383, 0.0);

        s.store_scalar(2381, 0.0);

        s.b[2608] = ((s.v[262] > 0.0) && (s.v[264] > 0.0));
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        if s.b[2608] {
            s.store_mul_add_scaled_inputs_rhs(2027, 266, s.ad_value(1819), 0.5, s.ad_value(787), 1.0);
        }

        s.b[2609] = (s.v[2027] < 230.25850929940458);
        s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });

        s.b[2610] = (s.v[2027] > (-230.25850929940458));
        s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });

        if ((s.b[2608] && s.b[2609]) && s.b[2610]) {
            s.store_exp(2381, 2027);
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2610])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2381, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2611] = (s.v[2381] > 1e-10);
        s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });

        if ((s.b[2608] && s.b[2609]) && s.b[2611]) {
            s.store_ln_offset_input(2382, 2381, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2611])) {
            s.copy_ad(2382, 2381);
            s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2382), 2.0, s.ad_value(2382), 2.0, 1.0);
        }

        if (s.b[2608] && (!s.b[2609])) {
            s.copy_ad(2382, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if s.b[2608] {
            s.store_mul_ad_affine_product_lhs(2383, A::div_scaled_inputs(s.ad_value(264), (-2.0), s.ad_value(266), 1.0), s.ad_value(262), s.v[354], 0.0, 2028);
        }

        s.store_scalar(2386, 0.0);

        s.store_scalar(2384, 0.0);

        s.b[2612] = ((s.v[263] > 0.0) && (s.v[265] > 0.0));
        s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });

        if s.b[2612] {
            s.store_mul_add_scaled_inputs_rhs(2027, 266, s.ad_value(1819), 0.5, s.ad_value(788), 1.0);
        }

        s.b[2613] = (s.v[2027] < 230.25850929940458);
        s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });

        s.b[2614] = (s.v[2027] > (-230.25850929940458));
        s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });

        if ((s.b[2612] && s.b[2613]) && s.b[2614]) {
            s.store_exp(2384, 2027);
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2614])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2384, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2615] = (s.v[2384] > 1e-10);
        s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });

        if ((s.b[2612] && s.b[2613]) && s.b[2615]) {
            s.store_ln_offset_input(2385, 2384, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2615])) {
            s.copy_ad(2385, 2384);
            s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2385), 2.0, s.ad_value(2385), 2.0, 1.0);
        }

        if (s.b[2612] && (!s.b[2613])) {
            s.copy_ad(2385, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if s.b[2612] {
            s.store_mul_ad_affine_product_lhs(2386, A::div_scaled_inputs(s.ad_value(265), (-2.0), s.ad_value(266), 1.0), s.ad_value(263), s.v[354], 0.0, 2028);
        }

        s.store_add(2387, 2383, 2386);

        s.store_add_scaled_product_indices(856, 2387, 1.0, 267, 829, 1.0);

        s.store_mul(854, 274, 834);

        s.store_mul(855, 275, 837);

        s.store_scalar(1938, 0.0);

        s.store_scalar(1939, 0.0);

        s.store_scalar(1940, 0.0);

        s.store_scalar(1941, 0.0);

        s.b[2616] = (s.v[1] != 0.0);
        s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });

        s.b[2617] = (s.v[1890] <= 0.0);
        s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });

        if (s.b[2616] && s.b[2617]) {
            s.store_scalar(1936, 0.5);
            s.store_scalar(1937, 1.0);
            s.copy_ad(1938, 1889);
        }

        if (s.b[2616] && (!s.b[2617])) {
            s.store_offset_scaled_div(1936, 1893, 1909, ((0.25) * (0.5)), 0.5);
            s.store_div_add_scaled_inputs_rhs_indices(1937, 1935, 1890, 1.0, 1934, -1.0);
            s.store_div(1938, 1889, 1937);
        }

        if s.b[2616] {
            s.store_square(1939, 1938);
            s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);
            s.store_scale(1941, 1940, 1e-5);
        }

        s.store_scalar(2618, 0.0);

        s.store_scalar(2621, 0.0);

        s.store_scalar(2622, 0.0);

        s.store_scalar(2623, 0.0);

        s.store_scalar(2624, 0.0);

        s.store_scalar(2625, 0.0);

        s.store_scalar(2626, 0.0);

        s.store_scalar(2627, 0.0);

        s.store_scalar(2628, 0.0);

        s.store_scalar(2629, 0.0);

        s.store_scalar(2630, 0.0);

        s.store_scalar(2631, 0.0);

        s.store_scalar(2632, 0.0);

        s.store_scalar(2633, 0.0);

        s.store_scalar(2634, 0.0);

        s.store_scalar(2635, 0.0);

        s.store_scalar(2636, 0.0);

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(2639, 0.0);

        s.store_scalar(2643, 0.0);

        s.store_scalar(2646, 0.0);

        s.store_scalar(2647, 0.0);

        s.store_scalar(2648, 0.0);

        s.store_scalar(2649, 0.0);

        s.store_scalar(2650, 0.0);

        s.store_scalar(2651, 0.0);

        s.store_scalar(2654, 0.0);

        s.store_scalar(2655, 0.0);

        s.store_scalar(2656, 0.0);

        s.store_scalar(2657, 0.0);

        s.store_scalar(2661, 0.0);

        s.store_scalar(2663, 0.0);

        s.store_scalar(2664, 0.0);

        s.store_scalar(857, 0.0);

        s.store_scalar(1918, 0.0);

        s.store_scalar(1919, 0.0);

        s.store_scalar(1920, 0.0);

        s.store_scalar(858, 0.0);

        s.store_scalar(1921, 0.0);

        s.store_scalar(1922, 0.0);

        s.store_scalar(1923, 0.0);

        s.b[2665] = (p.p43 > 0.0);
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        s.b[2666] = (s.v[474] == 1.0);
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        if (s.b[2665] && s.b[2666]) {
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2670, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2671] = (s.v[651] > 0.5);
        s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });

        s.b[2672] = (s.v[408] == 0.5);
        s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && s.b[2672]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[405]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && (!s.b[2672])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2671]) {
            s.store_add_scaled_inputs3_offset_indices(1918, 2669, (-s.v[417]), 832, s.v[420], 2670, (-s.v[420]), s.v[417]);
        }

        s.b[2673] = (s.v[652] > 0.5);
        s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });

        s.b[2674] = (s.v[409] == 0.5);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && s.b[2674]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[406]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && (!s.b[2674])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2673]) {
            s.store_add_scaled_inputs3_offset_indices(1919, 2669, (-s.v[418]), 832, s.v[421], 2670, (-s.v[421]), s.v[418]);
        }

        s.b[2675] = (s.v[653] > 0.5);
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        s.b[2676] = (s.v[410] == 0.5);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && s.b[2676]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[407]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && (!s.b[2676])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2675]) {
            s.store_add_scaled_inputs3_offset_indices(1920, 2669, (-s.v[419]), 832, s.v[422], 2670, (-s.v[422]), s.v[419]);
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2670, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2677] = (s.v[678] > 0.5);
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        s.b[2678] = (s.v[575] == 0.5);
        s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && s.b[2678]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(572)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 572, 575);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2677]) {
            s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2669)), 1.0, 587, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2679] = (s.v[679] > 0.5);
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        s.b[2680] = (s.v[576] == 0.5);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && s.b[2680]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(573)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && (!s.b[2680])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 573, 576);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2679]) {
            s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2669)), 1.0, 588, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2681] = (s.v[680] > 0.5);
        s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });

        s.b[2682] = (s.v[577] == 0.5);
        s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && s.b[2682]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(574)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && (!s.b[2682])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 574, 577);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2681]) {
            s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2669)), 1.0, 589, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2683] = (p.p872 > 0.0);
        s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2683]) {
            s.store_scaled_offset_ad(642, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873)), p.p872);
            s.store_offset(640, 642, p.p862);
            s.store_div_from_scalar(450, 1.0, 640);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2683])) {
            s.store_scalar(640, p.p862);
        }

        s.b[2684] = (p.p874 > 0.0);
        s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {
            s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875)), p.p874);
            s.store_mul_offset_rhs(443, 443, 644, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2685] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2628, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2686] = (s.v[832] < s.v[654]);
        s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });

        s.b[2687] = (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
            s.store_exp_scaled_input(2629, 832, (s.v[371] * (-0.5)));
        }

        s.b[2688] = (((-0.5) * (s.v[832] * s.v[371])) < 0.0);
        s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && s.b[2688]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2629, 1e-100, (-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && (!s.b[2688])) {
            s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(832), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2686])) {
            s.store_mul_offset_ad_lhs(2627, A::sub_scaled_inputs(s.ad_value(832), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2689] = (s.v[832] > 0.0);
        s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2689]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2689])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 832);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_sub(2632, 656, 2631);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2633, 832, 0.5, 2632, 0.5, 832, 2632, ((4.0 * s.v[370]) * s.v[370]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2634, 832, 0.5, 659, 0.5, 832, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2635, 832, 832, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2690] = (s.v[646] == 0.0);
        s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2690]) {
            s.store_scalar(1918, 0.0);
        }

        s.b[2691] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_sub_from_scalar(2639, s.v[393], 2633);
        }

        s.b[2693] = (p.p831 == 0.5);
        s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {
            s.store_powf_scaled_input(2636, 2639, s.v[429], p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_scale(2643, 2636, s.v[423]);
        }

        s.b[2694] = (p.p845 == 0.0);
        s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_div_scaled_inputs_indices(2646, 2643, (s.v[408] * s.v[438]), 2639, 1.0);
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, s.ad_value(2649), (-s.v[435]), s.ad_value(2647), s.ad_value(2650), s.v[435], s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2697] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2697]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2697])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2698] = (s.v[2657] > 0.0);
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

        s.b[2699] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2700] = (p.p851 == 0.0);
        s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });

        s.b[2701] = (p.p831 == 0.5);
        s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {
            s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {
            s.store_powf_scale_offset_input(2636, 2634, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {
            s.store_div_scaled_offset_numerator(2661, s.ad_value(2634), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(2636), 1.0);
        }

        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);
        s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 441, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 441, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2704] = (p.p860 > 1000.0);
        s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });

        s.b[2705] = (s.v[2635] > ((-s.v[444]) * p.p860));
        s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });

        s.b[2706] = (p.p863 == 4.0);
        s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && s.b[2706]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[448] * s.v[448]) * s.v[448])), 2635, s.v[448]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && (!s.b[2706])) {
            s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[448]), p.p863);
        }

        s.b[2707] = (s.v[408] == 0.5);
        s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2707]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[405]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2707])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {
            s.store_add_scaled_inputs3_offset_indices(1918, 2636, ((-s.v[417]) * p.p30), 832, (s.v[420] * p.p30), 2628, ((-s.v[420]) * p.p30), (s.v[417] * p.p30));
        }

        s.b[2708] = (s.v[647] == 0.0);
        s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2708]) {
            s.store_scalar(1919, 0.0);
        }

        s.b[2709] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_sub_from_scalar(2639, s.v[394], 2633);
        }

        s.b[2711] = (p.p832 == 0.5);
        s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {
            s.store_powf_scaled_input(2636, 2639, s.v[430], p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_scale(2643, 2636, s.v[424]);
        }

        s.b[2712] = (p.p846 == 0.0);
        s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_div_scaled_inputs_indices(2646, 2643, (s.v[409] * s.v[439]), 2639, 1.0);
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, s.ad_value(2649), (-s.v[436]), s.ad_value(2647), s.ad_value(2650), s.v[436], s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2715] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2715]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2715])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2716] = (s.v[2657] > 0.0);
        s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });

        s.b[2717] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && s.b[2717]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && (!s.b[2717])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2718] = (p.p852 == 0.0);
        s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });

        s.b[2719] = (p.p832 == 0.5);
        s.store_scalar(2719, if s.b[2719] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2719]) {
            s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2719])) {
            s.store_powf_scale_offset_input(2636, 2634, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {
            s.store_div_scaled_offset_numerator(2661, s.ad_value(2634), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(2636), 1.0);
        }

        s.b[2720] = (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2720]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2721] = (((-s.v[442]) / s.v[2661]) < 0.0);
        s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && s.b[2721]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 442, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && (!s.b[2721])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 442, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2722] = (p.p861 > 1000.0);
        s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });

        s.b[2723] = (s.v[2635] > ((-s.v[444]) * p.p861));
        s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });

        s.b[2724] = (p.p864 == 4.0);
        s.store_scalar(2724, if s.b[2724] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && s.b[2724]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[449] * s.v[449]) * s.v[449])), 2635, s.v[449]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && (!s.b[2724])) {
            s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[449]), p.p864);
        }

        s.b[2725] = (s.v[409] == 0.5);
        s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2725]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[406]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2725])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {
            s.store_add_scaled_inputs3_offset_indices(1919, 2636, ((-s.v[418]) * p.p30), 832, (s.v[421] * p.p30), 2628, ((-s.v[421]) * p.p30), (s.v[418] * p.p30));
        }

        s.b[2726] = (s.v[648] == 0.0);
        s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2726]) {
            s.store_scalar(1920, 0.0);
        }

        s.b[2727] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_sub_from_scalar(2639, s.v[395], 2633);
        }

        s.b[2729] = (p.p833 == 0.5);
        s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2729]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2729])) {
            s.store_powf_scaled_input(2636, 2639, s.v[431], p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_scale(2643, 2636, s.v[425]);
        }

        s.b[2730] = (p.p847 == 0.0);
        s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_div_scaled_inputs_indices(2646, 2643, (s.v[410] * s.v[440]), 2639, 1.0);
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, s.ad_value(2649), (-s.v[437]), s.ad_value(2647), s.ad_value(2650), s.v[437], s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2733] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2733]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2733])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2734] = (s.v[2657] > 0.0);
        s.store_scalar(2734, if s.b[2734] { 1.0 } else { 0.0 });

        s.b[2735] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && s.b[2735]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && (!s.b[2735])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2736] = (p.p853 == 0.0);
        s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });

        s.b[2737] = (p.p833 == 0.5);
        s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2737]) {
            s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2737])) {
            s.store_powf_scale_offset_input(2636, 2634, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {
            s.store_div_scaled_offset_numerator(2661, s.ad_value(2634), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(2636), 1.0);
        }

        s.b[2738] = (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2738]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2739] = (((-s.v[443]) / s.v[2661]) < 0.0);
        s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && s.b[2739]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 443, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && (!s.b[2739])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 443, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2740] = (s.v[640] > 1000.0);
        s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });

        s.b[2741] = (s.v[2635] > ((-s.v[444]) * s.v[640]));
        s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });

        s.b[2742] = (p.p865 == 4.0);
        s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && s.b[2742]) {
            s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(450))), s.ad_value(2635), s.ad_value(450)), 2635, 450);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && (!s.b[2742])) {
            s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);
        }

        s.b[2743] = (s.v[473] == 1.0);
        s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            if (s.v[832] < p.p870) {
                if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                    s.store_scalar(2663, p.p870);
                } else {
                    s.store_offset_scaled_ad(2663, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(832), (-p.p870), 1.0 / (p.p871))), p.p871, p.p870);
                }
            } else {
                if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                    s.copy_ad(2663, 832);
                } else {
                    s.store_add_scaled_inputs_ad_rhs(2663, 832, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(832), (-1.0 / (p.p871)), ((p.p870) * (1.0 / (p.p871))))), p.p871);
                }
            }
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2744] = (s.v[410] == 0.5);
        s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2744]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2664), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2744])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p.p30), 2663, (s.v[422] * p.p30), 2664, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));
            s.store_sub_offset_lhs(2663, 832, p.p870, 2663);
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2745] = (s.v[467] == 0.5);
        s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2745]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(466)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2745])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 466, 467);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, s.ad_value(2636)), p.p30, 471, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);
            s.store_add(1920, 1920, 472);
        }

        s.b[2746] = (s.v[410] == 0.5);
        s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && s.b[2746]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && (!s.b[2746])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) {
            s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p.p30), 832, (s.v[422] * p.p30), 2628, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));
        }

        s.b[2747] = (s.v[636] > 0.0);
        s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2747]) {
            s.store_mul_sub_ad_rhs(643, 636, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637)));
            s.store_add(641, 542, 643);
            s.store_div_from_scalar(616, 1.0, 641);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2747])) {
            s.copy_ad(641, 542);
        }

        s.b[2748] = (s.v[638] > 0.0);
        s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2665] && (!s.b[2666])) && s.b[2748]) {
            s.store_mul_sub_ad_rhs(645, 638, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639)));
            s.store_mul_offset_rhs(610, 610, 645, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2749] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2628, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2750] = (s.v[833] < s.v[681]);
        s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });

        s.b[2751] = (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && s.b[2751]) {
            s.store_exp_scaled_input(2629, 833, (s.v[371] * (-0.5)));
        }

        s.b[2752] = (((-0.5) * (s.v[833] * s.v[371])) < 0.0);
        s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && s.b[2752]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2629, 1e-100, (-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && (!s.b[2752])) {
            s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(833), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2750])) {
            s.store_mul_offset_ad_lhs(2627, A::sub_scaled_inputs(s.ad_value(833), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2753] = (s.v[833] > 0.0);
        s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2753]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2753])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 833);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_sub(2632, 683, 2631);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2633, 833, 0.5, 2632, 0.5, 833, 2632, ((4.0 * s.v[370]) * s.v[370]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2634, 833, 0.5, 686, 0.5, 833, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2635, 833, 833, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2754] = (s.v[673] == 0.0);
        s.store_scalar(2754, if s.b[2754] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2754]) {
            s.store_scalar(1921, 0.0);
        }

        s.b[2755] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_sub(2639, 569, 2633);
        }

        s.b[2757] = (s.v[511] == 0.5);
        s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && s.b[2757]) {
            s.store_sqrt_mul(2636, 2639, 596);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && (!s.b[2757])) {
            s.store_pow_mul_base_indices(2636, 2639, 596, 511);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_mul(2643, 590, 2636);
        }

        s.b[2758] = (s.v[525] == 0.0);
        s.store_scalar(2758, if s.b[2758] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_mul_div_scaled_product_indices(2646, 605, 2643, 575, 1.0, 2639, 1.0);
            s.store_div_scaled_inputs_indices(2647, 602, 0.666666666666667, 2646, 1.0);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, A::mul3(s.ad_value(602), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(602), s.ad_value(2649), (-1.0), s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2761] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2761, if s.b[2761] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2761]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2761])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2762] = (s.v[2657] > 0.0);
        s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });

        s.b[2763] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2763, if s.b[2763] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && s.b[2763]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && (!s.b[2763])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2764] = (s.v[531] == 0.0);
        s.store_scalar(2764, if s.b[2764] { 1.0 } else { 0.0 });

        s.b[2765] = (s.v[511] == 0.5);
        s.store_scalar(2765, if s.b[2765] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2765]) {
            s.store_sqrt_mul_sub_lhs(2636, 508, 2634, 596);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2765])) {
            s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(508), s.ad_value(2634)), 596, 511);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) {
            s.store_mul_div_scaled_product_mixed_iaii(2661, 578, A::sub(s.ad_value(508), s.ad_value(2634)), 593, 1.0, 2636, 1.0);
        }

        s.b[2766] = (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2766, if s.b[2766] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2766]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2767] = (((-s.v[608]) / s.v[2661]) < 0.0);
        s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && s.b[2767]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 608, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && (!s.b[2767])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 608, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2768] = (s.v[540] > 1000.0);
        s.store_scalar(2768, if s.b[2768] { 1.0 } else { 0.0 });

        s.b[2769] = (s.v[2635] > ((-s.v[444]) * s.v[540]));
        s.store_scalar(2769, if s.b[2769] { 1.0 } else { 0.0 });

        s.b[2770] = (s.v[543] == 4.0);
        s.store_scalar(2770, if s.b[2770] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && s.b[2770]) {
            s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(2635), s.ad_value(614)), 2635, 614);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && (!s.b[2770])) {
            s.store_pow_abs_mul_base_indices(2636, 2635, 614, 543);
        }

        s.b[2771] = (s.v[575] == 0.5);
        s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2771]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(572)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2771])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 572, 575);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {
            s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2636)), p.p30, 587, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);
        }

        s.b[2772] = (s.v[674] == 0.0);
        s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2772]) {
            s.store_scalar(1922, 0.0);
        }

        s.b[2773] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_sub(2639, 570, 2633);
        }

        s.b[2775] = (s.v[512] == 0.5);
        s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && s.b[2775]) {
            s.store_sqrt_mul(2636, 2639, 597);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && (!s.b[2775])) {
            s.store_pow_mul_base_indices(2636, 2639, 597, 512);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_mul(2643, 591, 2636);
        }

        s.b[2776] = (s.v[526] == 0.0);
        s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_mul_div_scaled_product_indices(2646, 606, 2643, 576, 1.0, 2639, 1.0);
            s.store_div_scaled_inputs_indices(2647, 603, 0.666666666666667, 2646, 1.0);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, A::mul3(s.ad_value(603), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(603), s.ad_value(2649), (-1.0), s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2779] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2779]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2779])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2780] = (s.v[2657] > 0.0);
        s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });

        s.b[2781] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && s.b[2781]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && (!s.b[2781])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2782] = (s.v[532] == 0.0);
        s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });

        s.b[2783] = (s.v[512] == 0.5);
        s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2783]) {
            s.store_sqrt_mul_sub_lhs(2636, 509, 2634, 597);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2783])) {
            s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(509), s.ad_value(2634)), 597, 512);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {
            s.store_mul_div_scaled_product_mixed_iaii(2661, 579, A::sub(s.ad_value(509), s.ad_value(2634)), 594, 1.0, 2636, 1.0);
        }

        s.b[2784] = (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2784]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2785] = (((-s.v[609]) / s.v[2661]) < 0.0);
        s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && s.b[2785]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 609, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && (!s.b[2785])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 609, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2786] = (s.v[541] > 1000.0);
        s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });

        s.b[2787] = (s.v[2635] > ((-s.v[444]) * s.v[541]));
        s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });

        s.b[2788] = (s.v[544] == 4.0);
        s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && s.b[2788]) {
            s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(2635), s.ad_value(615)), 2635, 615);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && (!s.b[2788])) {
            s.store_pow_abs_mul_base_indices(2636, 2635, 615, 544);
        }

        s.b[2789] = (s.v[576] == 0.5);
        s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2789]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(573)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2789])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 573, 576);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {
            s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2636)), p.p30, 588, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);
        }

        s.b[2790] = (s.v[675] == 0.0);
        s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });

        if ((s.b[2665] && (!s.b[2666])) && s.b[2790]) {
            s.store_scalar(1923, 0.0);
        }

        s.b[2791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_sub(2639, 571, 2633);
        }

        s.b[2793] = (s.v[513] == 0.5);
        s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2793]) {
            s.store_sqrt_mul(2636, 2639, 598);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2793])) {
            s.store_pow_mul_base_indices(2636, 2639, 598, 513);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_mul(2643, 592, 2636);
        }

        s.b[2794] = (s.v[527] == 0.0);
        s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_mul_div_scaled_product_indices(2646, 607, 2643, 577, 1.0, 2639, 1.0);
            s.store_div_scaled_inputs_indices(2647, 604, 0.666666666666667, 2646, 1.0);
            s.store_square(2648, 2647);
            s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);
            s.store_sqrt(2650, 2649);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, A::mul3(s.ad_value(604), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(604), s.ad_value(2649), (-1.0), s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2797] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2797]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2797])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2798] = (s.v[2657] > 0.0);
        s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });

        s.b[2799] = (s.v[2656] > (-230.25850929940458));
        s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && s.b[2799]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && (!s.b[2799])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2800] = (s.v[533] == 0.0);
        s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });

        s.b[2801] = (s.v[513] == 0.5);
        s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2801]) {
            s.store_sqrt_mul_sub_lhs(2636, 510, 2634, 598);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2801])) {
            s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(510), s.ad_value(2634)), 598, 513);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {
            s.store_mul_div_scaled_product_mixed_iaii(2661, 580, A::sub(s.ad_value(510), s.ad_value(2634)), 595, 1.0, 2636, 1.0);
        }

        s.b[2802] = (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2802]) {
            s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2803] = (((-s.v[610]) / s.v[2661]) < 0.0);
        s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && s.b[2803]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 610, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && (!s.b[2803])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 610, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2804] = (s.v[641] > 1000.0);
        s.store_scalar(2804, if s.b[2804] { 1.0 } else { 0.0 });

        s.b[2805] = (s.v[2635] > ((-s.v[444]) * s.v[641]));
        s.store_scalar(2805, if s.b[2805] { 1.0 } else { 0.0 });

        s.b[2806] = (s.v[545] == 4.0);
        s.store_scalar(2806, if s.b[2806] { 1.0 } else { 0.0 });

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && s.b[2806]) {
            s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(2635), s.ad_value(616)), 2635, 616);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && (!s.b[2806])) {
            s.store_pow_abs_mul_base_indices(2636, 2635, 616, 545);
        }

        s.b[2807] = (s.v[635] == 1.0);
        s.store_scalar(2807, if s.b[2807] { 1.0 } else { 0.0 });

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            if (s.v[833] < s.v[550]) {
                if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                    s.copy_ad(2663, 550);
                } else {
                    s.store_add_scaled_product_left_ad(2663, 550, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(833), 1.0, s.ad_value(550), (-1.0), s.ad_value(551), 1.0)), 551, 1.0);
                }
            } else {
                if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                    s.copy_ad(2663, 833);
                } else {
                    s.store_add_scaled_product_left_ad(2663, 833, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(550), 1.0, s.ad_value(833), (-1.0), s.ad_value(551), 1.0)), 551, 1.0);
                }
            }
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_scaled_product_indices(2623, 2663, 1.0, 684, 2622, 1.0);
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2808] = (s.v[577] == 0.5);
        s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2808]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2808])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 574, 577);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, 589, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);
            s.store_add_scaled_inputs3_indices(2663, 833, 1.0, 550, 1.0, 2663, -1.0);
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_scaled_product_indices(2623, 2663, 1.0, 684, 2622, 1.0);
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2809] = (s.v[630] == 0.5);
        s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2809]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(629)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2809])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 629, 630);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(633), 1.0, s.ad_value(2636)), p.p30, 634, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);
            s.store_add(1923, 1923, 472);
        }

        s.b[2810] = (s.v[577] == 0.5);
        s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && s.b[2810]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && (!s.b[2810])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 574, 577);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) {
            s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, 589, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);
        }

        s.store_scalar(1942, 0.0);

        s.store_scalar(1943, 0.0);

        s.store_scalar(1944, 0.0);

        s.store_scalar(1945, 0.0);

        s.store_scalar(1946, 0.0);

        s.store_scalar(1947, 0.0);

        s.store_scalar(1948, 0.0);

        s.store_scalar(1949, 0.0);

        s.store_scalar(1950, 0.0);

        s.store_scalar(1951, 0.0);

        s.store_scalar(1952, 0.0);

        s.store_scalar(1953, 0.0);

        s.store_scalar(1954, 0.0);

        s.store_scalar(1955, 0.0);

        s.store_scalar(1956, 0.0);

        s.store_scalar(1957, 0.0);

        s.store_scalar(1958, 0.0);

        s.store_scalar(1959, 0.0);

        s.b[2811] = (s.v[1] != 0.0);
        s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });

        if s.b[2811] {
            s.store_scalar(1988, 0.0);
            s.store_scalar(1992, 0.0);
            s.store_scalar(1986, 0.0);
            s.store_scalar(1987, 0.0);
            s.store_scalar(1993, 0.0);
            s.store_scalar(1969, 0.0);
            s.store_scalar(1970, 0.0);
            s.store_scalar(1971, 0.0);
            s.store_scalar(1972, 0.0);
            s.store_scalar(1973, 0.0);
            s.store_scalar(1974, 0.0);
            s.store_scalar(1975, 0.0);
            s.store_scalar(1976, 0.0);
            s.store_scalar(1977, 0.0);
            s.store_scalar(1960, 0.0);
            s.store_scalar(1961, 0.0);
            s.store_scalar(1962, 0.0);
            s.store_scalar(1963, 0.0);
            s.store_scalar(1964, 0.0);
            s.store_scalar(1965, 0.0);
            s.store_scalar(1966, 0.0);
            s.store_scalar(1967, 0.0);
            s.store_scalar(1968, 0.0);
        }

        s.b[2812] = (s.v[1890] > 0.0);
        s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });

        s.b[2813] = (s.v[1] == 1.0);
        s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_add_scaled_product_left_ad(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2814] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && s.b[2814]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2815] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2815]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2816] = ((-s.v[1960]) < 0.0);
        s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && s.b[2816]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && (!s.b[2816])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2817] = (s.v[1960] > s.v[1933]);
        s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2817]) {
            s.store_neg(1996, 1996);
        }

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);
        }

        s.b[2818] = (s.v[1] == 2.0);
        s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_add_scaled_product_left_ad(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3333333333333333, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2819] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2819]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2820] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2821] = ((-s.v[1960]) < 0.0);
        s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2822] = (s.v[1960] > s.v[1933]);
        s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2822]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);
            s.store_add_scaled_product_left_ad(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6666666666666666, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2823] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2823]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2824] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2824]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2825] = ((-s.v[1961]) < 0.0);
        s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && s.b[2825]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && (!s.b[2825])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2826] = (s.v[1961] > s.v[1933]);
        s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2826]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);
        }

        s.b[2827] = (s.v[831] < 0.0);
        s.store_scalar(2827, if s.b[2827] { 1.0 } else { 0.0 });

    }
}
