#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(730, 0.0);s.store_scalar(731, 0.0);s.b[1152] = (s.v[188] > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
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
        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));s.b[1153] = (p[52] > 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
    ) {
        if s.b[1153] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(728, 728, 735);s.store_primal_mul_scale_offset_mixed_ia(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_sqrt(736, 728);s.store_primal_scale(737, 728, 0.95);s.store_primal_scaled_mul(738, 728, 728, 0.0025);s.copy_ad(739, 738);s.store_primal_scaled_sqrt(740, 739, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));s.store_primal_scaled_offset(742, 728, s.v[362], 0.5);s.store_primal_sub_mixed_ai(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);s.store_primal_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);s.store_primal_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);
        if (!(s.v[745] > 0.05)) {s.store_scalar(745, 0.05);}
        s.store_primal_div_mixed_ai(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1154] = (p[52] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        if s.b[1154] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(745, 745, 735);s.store_primal_mul_scale_offset_mixed_ia(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_scale(747, 745, 0.95);s.store_primal_scaled_mul(748, 745, 745, 0.0025);s.copy_ad(749, 748);s.store_primal_scaled_sqrt(740, 749, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        s.store_primal_offset_add_scaled_product_mixed_iia(700, 177, 1.0, 178, A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);s.store_primal_exp_scaled_input(751, 180, s.v[360]);s.store_primal_mul(701, 189, 751);s.store_primal_scale(702, 190, 1.0 / (s.v[359]));s.store_primal_exp_scaled_input(752, 203, s.v[360]);s.store_primal_mul(703, 202, 752);s.store_primal_scaled_mul(716, 703, 769, s.v[20]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
    ) {
        s.store_primal_mul_mixed_ia(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));s.store_primal_exp_scaled_input(753, 205, s.v[360]);s.store_primal_mul(704, 204, 753);s.store_primal_mul_mixed_ia(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));s.store_primal_exp_scaled_input(754, 209, s.v[360]);s.store_primal_mul(706, 208, 754);s.store_primal_exp_scaled_input(755, 213, s.v[360]);s.store_primal_mul(708, 212, 755);s.store_primal_exp_scaled_input(756, 216, s.v[360]);s.store_primal_mul(709, 215, 756);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_scaled_mul(757, 716, 709, 2.0);s.store_primal_exp_scaled_input(758, 220, s.v[360]);s.store_primal_mul(720, 219, 758);s.store_primal_mul(721, 258, 758);s.store_primal_mul_mixed_ia(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));s.store_primal_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));s.b[1155] = ((p[46] != 0.0) && (s.v[287] > 0.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_primal_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);s.store_primal_exp_scaled_input(759, 288, s.v[360]);s.store_primal_mul(714, 287, 759);s.store_primal_scaled_mul(717, 714, 769, s.v[22]);s.store_primal_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);s.store_primal_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);}
        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }
        if s.b[1155] {s.store_primal_div_mixed_ai(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_primal_square(724, 761);s.store_primal_ln(725, 724);s.store_primal_scale(762, 760, 0.95);s.store_primal_scaled_mul(763, 760, 760, 0.0025);s.copy_ad(764, 763);s.store_primal_scaled_sqrt(765, 764, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));}
        if (!s.b[1155]) {s.store_scalar(713, 0.0);s.store_scalar(759, 1.0);s.store_scalar(714, 0.0);s.store_scalar(717, 0.0);s.store_scalar(723, s.v[715]);s.store_scalar(760, 0.0);s.store_scalar(761, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 0.0);s.store_scalar(762, 0.0);s.store_scalar(763, 0.0);s.store_scalar(764, 0.0);s.store_scalar(765, 0.0);s.store_scalar(766, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
    ) {
        s.store_primal_div_from_scalar(795, 1.0, 246);s.store_primal_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));s.store_primal_mul(797, 796, 181);s.store_primal_mul(798, 796, 192);s.store_primal_mul(799, 796, 193);s.store_scalar(800, 0.0);s.b[1156] = (s.v[241] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_primal_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);}
        s.store_scalar(801, 0.0);s.b[1157] = (s.v[243] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_primal_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);}
        s.b[1158] = (s.v[245] < 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_primal_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
    ) {
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
        s.store_primal_mul(711, 250, 796);s.store_primal_scaled_mul(807, 711, 193, 500000000.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(808, 0.0);s.b[1159] = (s.v[272] > 1e-10);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
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
        s.store_primal_scale(24, 6, s.v[646]);s.store_primal_scale(25, 6, s.v[647]);s.store_primal_scale(26, 6, s.v[648]);s.store_primal_scale(27, 6, s.v[673]);s.store_primal_scale(28, 6, s.v[674]);s.store_primal_scale(29, 6, s.v[675]);s.store_scalar(30, 0.0);s.b[1167] = (p[43] == 3.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(30, 1.0);}
        s.copy_ad(31, 313);s.b[1168] = (p[39] == 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));}
        s.b[1169] = ((p[43] == 2.0) || (p[43] == 3.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_primal_scale(24, 6, s.v[649]);s.store_primal_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));s.copy_ad(26, 31);s.store_primal_scale(27, 6, s.v[676]);s.store_primal_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));s.copy_ad(29, 31);}
        s.b[1170] = (((p[43] == 1.0) || (p[43] == 2.0)) || (p[43] == 3.0));s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
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
        s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(654, 0.0);s.store_scalar(681, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(667, 0.0);s.store_scalar(694, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(668, 1.0);s.store_scalar(695, 1.0);s.store_scalar(669, 0.0);s.store_scalar(696, 0.0);s.store_scalar(670, 1.0);s.store_scalar(697, 1.0);s.store_scalar(671, 0.0);s.store_scalar(698, 0.0);s.store_scalar(672, 1.0);s.store_scalar(699, 1.0);s.store_scalar(666, 0.0);s.store_scalar(693, 0.0);s.store_scalar(660, 0.0);s.store_scalar(687, 0.0);s.store_scalar(661, 0.0);s.store_scalar(688, 0.0);s.store_scalar(662, 0.0);s.store_scalar(689, 0.0);s.store_scalar(663, 0.0);s.store_scalar(690, 0.0);s.store_scalar(664, 0.0);s.store_scalar(691, 0.0);s.store_scalar(665, 0.0);s.store_scalar(692, 0.0);s.store_scalar(651, 1.0);s.store_scalar(678, 1.0);s.store_scalar(652, 1.0);s.store_scalar(679, 1.0);s.store_scalar(653, 1.0);s.store_scalar(680, 1.0);s.store_scalar(491, 0.0);s.store_scalar(492, 0.0);s.store_scalar(480, 0.0);s.store_scalar(481, 0.0);s.store_scalar(482, 0.0);s.store_scalar(483, 0.0);s.store_scalar(484, 0.0);s.store_scalar(493, 0.0);s.store_scalar(494, 0.0);s.store_scalar(495, 0.0);s.store_scalar(501, 0.0);s.store_scalar(490, 0.0);s.b[1171] = (p[43] > 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1172]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1172])) {s.store_scalar(454, 100000000.0);}
        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1173]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1173])) {s.store_scalar(455, 100000000.0);}
        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1174]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1174])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(654, 454, 455, 456);}
        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1175]) {s.store_primal_exp_scaled_input(655, 654, s.v[371]);}
        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {s.store_primal_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.store_scalar(396, s.v[393]);s.store_scalar(397, s.v[394]);s.store_scalar(398, s.v[395]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1171] {s.store_scalar(399, p[831]);s.store_scalar(400, p[832]);s.store_scalar(401, p[833]);s.store_scalar(402, p[828]);s.store_scalar(403, p[829]);s.store_scalar(404, p[830]);}
        s.b[1177] = (s.v[646] == 0.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1177]) {s.store_scalar(396, (s.v[394] + s.v[395]));s.store_scalar(399, (0.9 * (p[832]).min(p[833])));s.store_scalar(402, (p[829] + p[830]));}
        s.b[1178] = (s.v[647] == 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1178]) {s.store_scalar(397, (s.v[393] + s.v[395]));s.store_scalar(400, (0.9 * (p[831]).min(p[833])));s.store_scalar(403, (p[828] + p[830]));}
        s.b[1179] = (s.v[648] == 0.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1179]) {s.store_scalar(398, (s.v[393] + s.v[394]));s.store_scalar(401, (0.9 * (p[831]).min(p[832])));s.store_scalar(404, (p[828] + p[829]));}
        if s.b[1171] {s.store_min3(656, 396, 397, 398);s.store_primal_scale(657, 656, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(658, 656, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);s.store_primal_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1180]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1180])) {s.store_scalar(454, 100000000.0);}
        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1181]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1181])) {s.store_scalar(455, 100000000.0);}
        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1182]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1182])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(681, 454, 455, 456);}
        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1183]) {s.store_primal_exp_scaled_input(682, 681, s.v[371]);}
        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {s.store_primal_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.copy_ad(396, 569);s.copy_ad(397, 570);s.copy_ad(398, 571);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
    ) {
        if s.b[1171] {s.copy_ad(399, 511);s.copy_ad(400, 512);s.copy_ad(401, 513);s.copy_ad(402, 508);s.copy_ad(403, 509);s.copy_ad(404, 510);}
        s.b[1185] = (s.v[673] == 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1185]) {s.store_primal_add(396, 570, 571);s.store_primal_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);s.store_primal_add(402, 509, 510);}
        s.b[1186] = (s.v[674] == 0.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1186]) {s.store_primal_add(397, 569, 571);s.store_primal_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);s.store_primal_add(403, 508, 510);}
        s.b[1187] = (s.v[675] == 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1187]) {s.store_primal_add(398, 569, 570);s.store_primal_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);s.store_primal_add(404, 508, 509);}
        if s.b[1171] {s.store_min3(683, 396, 397, 398);s.store_primal_scale(684, 683, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(685, 683, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1171] {s.store_primal_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
        s.b[1188] = (s.v[474] == 1.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1188]) {s.store_scalar(1189, 0.0);s.store_scalar(1190, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1198, 0.0);s.store_scalar(1200, 0.0);s.store_scalar(1201, 0.0);s.store_scalar(1202, 0.0);s.store_scalar(1203, 0.0);s.store_scalar(1204, 0.0);s.store_scalar(1205, 0.0);s.store_scalar(1206, 0.0);s.store_scalar(1207, 0.0);s.store_scalar(1208, 0.0);s.store_scalar(1209, 0.0);s.store_scalar(1210, 0.0);s.store_scalar(1211, 0.0);s.store_scalar(1212, 0.0);s.store_scalar(1213, 0.0);s.store_scalar(1214, 0.0);s.store_scalar(1215, 0.0);s.store_scalar(1216, 0.0);s.store_scalar(1217, 0.0);s.store_scalar(1218, 0.0);s.store_scalar(1219, 0.0);s.store_scalar(1220, 0.0);s.store_scalar(1221, 0.0);s.store_scalar(1222, 0.0);s.store_scalar(1223, 0.0);s.store_scalar(1224, 0.0);s.store_scalar(1225, 0.0);s.store_scalar(1226, 0.0);s.store_scalar(1227, 0.0);s.store_scalar(1228, 0.0);s.store_scalar(1229, 0.0);s.store_scalar(1230, 0.0);s.store_scalar(1231, 0.0);s.store_scalar(1232, 0.0);s.store_scalar(1233, 0.0);s.store_scalar(498, 0.4);s.store_scalar(499, 0.65);s.store_scalar(500, 0.8);s.store_primal_scale(485, 498, (-p[928]));s.store_primal_scale(486, 499, (-p[928]));s.store_primal_scale(487, 500, (-p[928]));s.store_scalar(488, 0.1);s.store_scalar(489, 0.2);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1237] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
    }
}
