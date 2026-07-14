#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_exp_scaled_input(752, 215, s.v[354]);s.store_primal_mul(714, 214, 752);s.store_primal_mul(715, 253, 752);s.store_primal_mul_mixed_ia(706, 225, A::exp_scaled_input(s.ad_value(226), (-s.v[354])));s.store_primal_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));s.b[1143] = ((p.p46 != 0.0) && (s.v[282] > 0.0));s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
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
        s.store_primal_pow_from_scalar_ad(797, s.v[346], s.ad_value(234));s.store_primal_mul(231, 231, 797);s.store_primal_mul(232, 232, 797);s.store_primal_mul(233, 233, 797);
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
        s.store_primal_mul(705, 245, 790);s.store_primal_scaled_mul(801, 705, 188, 500000000.0);s.store_scalar(802, 0.0);s.b[1147] = (s.v[267] > 1e-10);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_primal_div_from_scalar(802, 0.75, 267);}
        s.store_primal_square(803, 268);s.store_primal_scale(20, 2, s.v[640]);s.store_primal_scale(21, 2, s.v[641]);s.store_primal_scale(22, 2, s.v[642]);s.store_primal_scale(23, 2, s.v[667]);s.store_primal_scale(24, 2, s.v[668]);s.store_primal_scale(25, 2, s.v[669]);s.store_scalar(26, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1155] = (p.p43 == 3.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
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
        s.store_scalar(650, 0.0);s.store_scalar(677, 0.0);s.store_scalar(652, 0.0);s.store_scalar(679, 0.0);s.store_scalar(651, 0.0);s.store_scalar(678, 0.0);s.store_scalar(653, 0.0);s.store_scalar(680, 0.0);s.store_scalar(648, 0.0);s.store_scalar(675, 0.0);s.store_scalar(649, 0.0);s.store_scalar(676, 0.0);s.store_scalar(645, 1.0);s.store_scalar(672, 1.0);s.store_scalar(646, 1.0);s.store_scalar(673, 1.0);s.store_scalar(647, 1.0);s.store_scalar(674, 1.0);s.store_scalar(495, 0.0);s.b[1159] = (p.p43 > 0.0);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = ((s.v[381] * s.v[640]) > 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {s.store_primal_scaled_offset_ad(649, A::mul_offset_rhs(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(648), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1159] {s.store_scalar(390, s.v[387]);s.store_scalar(391, s.v[388]);s.store_scalar(392, s.v[389]);s.store_scalar(393, p.p824);s.store_scalar(394, p.p825);s.store_scalar(395, p.p826);s.store_scalar(396, p.p821);s.store_scalar(397, p.p822);s.store_scalar(398, p.p823);}
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
        if s.b[1159] {s.copy_ad(390, 563);s.copy_ad(391, 564);s.copy_ad(392, 565);s.copy_ad(393, 505);s.copy_ad(394, 506);s.copy_ad(395, 507);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1159] {s.copy_ad(396, 502);s.copy_ad(397, 503);s.copy_ad(398, 504);}
        s.b[1173] = (s.v[667] == 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1173]) {s.store_primal_add(390, 564, 565);s.store_primal_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);s.store_primal_add(396, 503, 504);}
        s.b[1174] = (s.v[668] == 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1174]) {s.store_primal_add(391, 563, 565);s.store_primal_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);s.store_primal_add(397, 502, 504);}
        s.b[1175] = (s.v[669] == 0.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1175]) {s.store_primal_add(392, 563, 564);s.store_primal_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);s.store_primal_add(398, 502, 503);}
        if s.b[1159] {s.store_min3(677, 390, 391, 392);s.store_primal_scale(678, 677, 0.1);s.store_max3(371, 393, 394, 395);s.store_primal_mul_scale_offset_mixed_ia(679, 677, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))), -1.0, 1.0);s.store_primal_offset_min_ad(680, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));}
        s.b[1176] = (s.v[468] == 1.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1176]) {s.store_primal_add_scaled_inputs3_indices(495, 640, (s.v[408] * p.p922), 641, (s.v[409] * p.p922), 642, (s.v[410] * p.p922));}
        s.b[1511] = ((s.v[640] * s.v[408]) <= s.v[495]);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1511]) {s.store_scalar(645, 0.0);}
        s.b[1512] = ((s.v[641] * s.v[409]) <= s.v[495]);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1512]) {s.store_scalar(646, 0.0);}
        s.b[1513] = ((s.v[642] * s.v[410]) <= s.v[495]);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1513]) {s.store_scalar(647, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_primal_mul_mixed_ia(495, 547, A::add_scaled_products3(s.ad_value(667), s.ad_value(575), 1.0, s.ad_value(668), s.ad_value(576), 1.0, s.ad_value(669), s.ad_value(577), 1.0));}
        s.b[1801] = ((s.v[667] * s.v[575]) <= s.v[495]);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1801]) {s.store_scalar(672, 0.0);}
        s.b[1802] = ((s.v[668] * s.v[576]) <= s.v[495]);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1802]) {s.store_scalar(673, 0.0);}
        s.b[1803] = ((s.v[669] * s.v[577]) <= s.v[495]);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1803]) {s.store_scalar(674, 0.0);}
        s.store_scalar(1919, 0.0);s.store_scalar(1920, 0.0);s.store_scalar(1921, 0.0);s.b[1994] = (s.v[0] == 1.0);s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });
        if s.b[1994] {s.store_voltage(819, ctx, nodes, Some(5), Some(6));s.store_voltage(820, ctx, nodes, Some(7), Some(6));s.store_voltage(821, ctx, nodes, Some(6), Some(8));s.store_scaled_voltage(826, ctx, nodes, Some(6), Some(10), -1.0);s.store_scaled_voltage(827, ctx, nodes, Some(7), Some(11), -1.0);}
        if (!s.b[1994]) {s.store_scaled_voltage(819, ctx, nodes, Some(5), Some(6), -1.0);s.store_scaled_voltage(820, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(821, ctx, nodes, Some(6), Some(8), -1.0);s.store_voltage(826, ctx, nodes, Some(6), Some(10));s.store_voltage(827, ctx, nodes, Some(7), Some(11));}
        s.store_add(823, 819, 821);s.copy_ad(828, 819);s.copy_ad(829, 821);s.store_add(830, 820, 821);s.store_sub(831, 819, 820);s.store_scale(1805, 828, (-s.v[349]));s.store_scale(1806, 831, (-s.v[349]));s.store_scaled_sub(1807, 823, 694, (-s.v[349]));s.store_scalar(825, 1.0);s.b[1995] = (s.v[820] < 0.0);s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });
        if s.b[1995] {s.store_scalar(825, (-1.0));s.store_sub(819, 819, 820);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1995] {s.store_add(821, 821, 820);s.store_neg(820, 820);}
        s.store_add(822, 820, 821);s.store_div_scaled_product_offset_denominator_mixed_iia(824, 820, 820, 1.0, A::sqrt_square_offset(s.ad_value(820), 0.01), 0.1, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1999, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(733))), (-0.5), 731, 1.0);s.copy_ad(1808, 1999);s.store_add_scaled_inputs4_mixed_iiai(1922, 821, 1.0, 1999, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1999)), s.ad_value(732))), (-(-0.5)), 735, 1.0);s.copy_ad(1809, 1922);s.store_scalar(1923, 0.0);s.b[2155] = ((p.p45 != 0.0) && (s.v[179] != 1.0));s.store_scalar(2155, if s.b[2155] { 1.0 } else { 0.0 });
        if s.b[2155] {s.store_add_scaled_inputs3_indices(1924, 1922, 1.0, 820, 0.5, 824, (-0.5));s.store_sub_mixed_ai(1925, A::sqrt(A::add(s.ad_value(1924), s.ad_value(722))), 730);s.store_offset_div_scaled_inputs2_indices(1919, 1925, 2.0, 737, (-2.0), 738, 1.0, (-1.0));s.store_add_scaled_product_mixed_iaa(1926, 1925, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(179), s.ad_value(738), 0.25), A::add(s.ad_value(1919), A::sqrt_square_offset(s.ad_value(1919), 0.4804530139182)), (-1.0));s.store_add_scaled_square_product_indices(1927, 1926, 1.0, 730, 1926, 2.0);s.store_add_scaled_inputs3_indices(1922, 1927, 1.0, 820, (-0.5), 824, (-(-0.5)));s.store_sub(1923, 1809, 1922);}
        s.copy_ad(1996, 722);s.copy_ad(1997, 732);s.copy_ad(1998, 723);s.copy_ad(2000, 1922);s.copy_ad(2004, 1923);s.copy_ad(2001, 714);s.copy_ad(2002, 771);s.store_add_scaled_inputs3_indices(2003, 823, 1.0, 2004, (-1.0), 694, -1.0);s.store_add_scaled_inputs3_indices(2005, 2000, 1.0, 820, 0.5, 824, (-0.5));s.store_scalar(2017, 1.0);s.b[2156] = (s.v[185] > 0.0);s.store_scalar(2156, if s.b[2156] { 1.0 } else { 0.0 });
        if s.b[2156] {s.store_primal_scale(2008, 1996, s.v[355]);s.store_scale(2009, 2005, s.v[355]);s.store_scale(2010, 2003, s.v[355]);s.store_offset_div_scaled_inputs_sqrt_rhs(1920, 1998, 0.5, 2008, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(1921, 2008, 1.0, 1998, A::sqrt(s.ad_value(2008)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2011, A::div_scaled_inputs2(s.ad_value(2010), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2008, 0.5, A::offset(s.ad_value(186), 1.0), 2009, (-1.0));s.store_primal_offset_scaled(2012, 2008, 0.5, 2.0);s.store_add(2013, 2008, 2009);s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2010), 1.0, s.ad_value(2013), (-1.0), s.ad_value(1998), A::sqrt(s.ad_value(2013)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2008), s.ad_value(1998)), A::sqrt(s.ad_value(2008)))), 2.0);s.store_add_scaled_inputs(2014, 1920, 2.0, 2012, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2011, 0.5, 2014, 0.5, 2011, 2014, 20.0, 0.5);s.store_add_scaled_inputs3_indices(1921, 2010, 2.0, 2009, (-2.0), 2012, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2015, 1920, 0.5, 1921, 0.5, 1920, 1921, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2015, 0.5, 2012, 0.5, 2015, 2012, 5.0, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2156] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2016, 1920, 0.5, 2012, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2012), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(1921, 696, A::div(s.ad_value(2016), s.ad_value(2012)), 1.0, 1.0);}
        s.b[2157] = (s.v[1921] > (-230.25850929940458));s.store_scalar(2157, if s.b[2157] { 1.0 } else { 0.0 });
        if (s.b[2156] && s.b[2157]) {s.store_exp(2017, 1921);}
        if (s.b[2156] && (!s.b[2157])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2017, 1e-100, (-230.25850929940458), 1921, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_offset_mul(2018, 695, 2017, 1.0);s.store_scale(2019, 2018, s.v[709]);s.store_mul_ad_product_rhs(2020, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2005)), 1.0));s.store_mul_scale_offset_indices(2021, 2019, 2020, 1.0, 1.0);s.store_div_from_scalar(2022, 1.0, 2021);s.store_mul_mixed_ia(2006, 1998, A::sqrt_scaled_input(s.ad_value(2022), s.v[709]));s.store_square(2007, 2006);s.store_div_from_scalar(2023, 1.0, 2007);s.store_mul(2024, 2000, 2022);s.store_mul(2025, 2003, 2022);s.store_div_scaled_value_offset_denominator(2026, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(192), s.ad_value(824), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2027, 191, 2026, A::offset(A::mul(s.ad_value(193), s.ad_value(2005)), 1.0));s.store_mul(2028, 1996, 2022);s.store_sqrt_square_add(1920, 1999, 1997);s.store_sqrt_add_ad(1921, A::square(A::sub(s.ad_value(1999), s.ad_value(2027))), s.ad_value(1997));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2029, 2022, 2027, 0.5, 1920, 0.5, 1921, ((-1.0) * (0.5)), 0.0);s.store_add(2030, 2028, 2024);s.store_sub(2031, 2030, 2029);s.b[2158] = (p.p45 > 0.0);s.store_scalar(2158, if s.b[2158] { 1.0 } else { 0.0 });s.b[2159] = (((s.v[2031]) as f64).abs() < 1e-5);s.store_scalar(2159, if s.b[2159] { 1.0 } else { 0.0 });
        if (s.b[2158] && s.b[2159]) {s.store_offset_ad(2032, A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2031), 1.0, A::scale(s.ad_value(2031), 0.3125), 0.5)), 1.0);}
        s.b[2160] = (s.v[2031] < 460.51701859880916);s.store_scalar(2160, if s.b[2160] { 1.0 } else { 0.0 });
        if ((s.b[2158] && (!s.b[2159])) && s.b[2160]) {s.store_exp_neg_input(2046, 2031);}
        if ((s.b[2158] && (!s.b[2159])) && (!s.b[2160])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2158] && (!s.b[2159])) {s.store_scalar(1919, (if (s.v[2031] > 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[2158] && (!s.b[2159])) {s.store_offset_ad(2032, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2006), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2031))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2031), 1.0, s.ad_value(2046))), 2.0), 1.0);}
        if (!s.b[2158]) {s.store_offset_div_scaled_inputs_sqrt_rhs(2032, 2006, 0.5, 2031, 1.0, 1.0);}
        s.store_add_scaled_value_products_mixed_iiaia(2033, 2031, 1.0, 2006, A::sqrt(s.ad_value(2031)), 1.0, 2032, A::ln(A::offset(s.ad_value(2032), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2034, 2025, 1.0, 2033, (-1.0), 2032, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2007)), 1.0)), (-1.0));s.store_scalar(2039, 0.0);s.store_scalar(2041, 1.0);s.b[2161] = (s.v[2034] > (-30.0));s.store_scalar(2161, if s.b[2161] { 1.0 } else { 0.0 });
        if s.b[2161] {s.store_offset_mul(2035, 2032, 2034, (-1.0));s.store_scaled_add_mixed_ia(1919, 2035, A::sqrt_square_offset(s.ad_value(2035), 10.0), 0.5);s.store_sub_mixed_ia(2036, 2034, A::ln(s.ad_value(1919)));s.store_scaled_add_mixed_ia(2037, 2036, A::sqrt_square_offset(s.ad_value(2036), 2.0), 0.5);}
        s.b[2162] = ((s.v[2034] - s.v[2037]) < 230.25850929940458);s.store_scalar(2162, if s.b[2162] { 1.0 } else { 0.0 });
        if (s.b[2161] && s.b[2162]) {s.store_exp_sub(1919, 2034, 2037);}
        if (s.b[2161] && (!s.b[2162])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if s.b[2161] {s.store_div(2038, 1919, 2032);s.store_sub_mixed_ai(1919, A::scaled_offset(s.ad_value(2037), 1.0, 2.0), 2038);}
        s.b[2163] = (s.v[2038] > 1e-6);s.store_scalar(2163, if s.b[2163] { 1.0 } else { 0.0 });
        if (s.b[2161] && s.b[2163]) {s.store_mul_scale_offset_mixed_ia(2039, 2032, A::sub(s.ad_value(2037), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2038), s.ad_value(1919), 1.0), 1.0, (-1.0), s.ad_value(2038), 1.0)), 1.0, 1.0);}
        if (s.b[2161] && (!s.b[2163])) {s.store_mul_ad_affine_product_rhs(2039, 2032, s.ad_value(2038), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);}
        if s.b[2161] {s.store_add_scaled_inputs3_offset_mixed_iia(1919, 2025, 0.5, 2039, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2007)), s.ad_value(1919), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2041, 2040, 2040, 1.0, 2039, 1.0);s.store_add_scaled_product_indices(2031, 2030, 1.0, 2041, 2029, (-1.0));}
        s.store_offset_scaled(2042, 2006, 0.7071067811865475, 1.0);s.store_scale(2043, 2042, 1e-5);s.store_div_from_scalar(2044, 1.0, 2042);s.store_scalar(2151, 0.0);s.store_scalar(2045, 0.0);s.b[2164] = (s.v[2031] < 460.51701859880916);s.store_scalar(2164, if s.b[2164] { 1.0 } else { 0.0 });
        if s.b[2164] {s.store_exp_neg_input(2046, 2031);}
        if (!s.b[2164]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2165] = (((s.v[2025]) as f64).abs() <= s.v[2043]);s.store_scalar(2165, if s.b[2165] { 1.0 } else { 0.0 });
        if s.b[2165] {s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2045, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2046)), s.ad_value(2006), s.ad_value(2131)), 1.0));}
        s.b[2166] = (s.v[2025] < (-s.v[2043]));s.store_scalar(2166, if s.b[2166] { 1.0 } else { 0.0 });
        if ((!s.b[2165]) && s.b[2166]) {s.store_neg(2133, 2025);s.store_scaled_mul(2134, 2133, 2044, 1.25);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2165]) && s.b[2166]) {s.store_scaled_sub_offset_sqrt_square_offset(2135, 2134, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2130, 2133, 2135);s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::offset(s.ad_value(2135), 1.0), 1.0);s.store_sub_scaled_inputs(2137, 2130, 2.0, 2007, 1.0);s.store_sub_ln_mul_lhs(2138, 2136, 2023, 2135);s.store_add(818, 2136, 2137);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.5, s.ad_value(2136), 1.0), 1.0);s.store_add_mixed_ia(2139, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.3333333333333333, s.ad_value(2136), 1.0))), 1.0));}
        s.b[2167] = (s.v[2139] < 230.25850929940458);s.store_scalar(2167, if s.b[2167] { 1.0 } else { 0.0 });
        if (((!s.b[2165]) && s.b[2166]) && s.b[2167]) {s.store_exp(2140, 2139);}
        if (((!s.b[2165]) && s.b[2166]) && (!s.b[2167])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2140, 2139, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((!s.b[2165]) && s.b[2166]) {s.store_div_from_scalar(2141, 1.0, 2140);s.store_div_from_scalar_offset_square(2130, 1.0, 2139, 2.0);s.store_mul_square_lhs(2142, 2139, 2130);s.store_mul3_affine_lhs(2143, 2139, 2130, 4.0, 0.0, 2130);s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);s.store_sub(2130, 2133, 2139);s.store_mul(2131, 2046, 2141);s.store_add_scaled_product_mixed_iia(2145, 2130, 2.0, 2007, A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2131), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2143)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2139), (-1.0), s.ad_value(2131), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::sub(A::offset(s.ad_value(2139), (-1.0)), s.ad_value(2142)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2140), 1.0, s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2045, 2139, -1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);}
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_div_from_scalar_offset_scaled_input(2147, 1.0, 2006, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2148, 2147, A::mul_scaled_lhs(s.ad_value(2042), 1.25, s.ad_value(2147)), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_mul_ad_product_rhs_mixed_ia(2149, 2025, 2044, A::offset(A::mul(s.ad_value(2148), s.ad_value(2025)), 1.0));}
        s.b[2168] = ((-s.v[2149]) > (-230.25850929940458));s.store_scalar(2168, if s.b[2168] { 1.0 } else { 0.0 });
        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2168]) {s.store_exp_neg_input(2130, 2149);}
        if (((!s.b[2165]) && (!s.b[2166])) && (!s.b[2168])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2130, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2149)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_sub_from_scalar(2150, 1.0, 2130);s.store_add_scaled_inputs_product_mixed_iiia(2151, 2025, 1.0, 2007, 0.5, 2006, A::sqrt(A::add_scaled_inputs3(s.ad_value(2025), 1.0, s.ad_value(2007), 0.25, s.ad_value(2150), -1.0)), (-1.0));s.store_offset(2152, 2031, 3.0);s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));s.store_sub(2130, 2025, 2135);s.store_exp_neg_input(2131, 2135);s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);s.store_mul_square_lhs(2142, 2135, 2132);s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);}
        if ((!s.b[2165]) && (!s.b[2166])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2046] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2138, 2031, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);s.store_add(818, 2136, 2137);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_add_mixed_ia(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));}
        s.b[2169] = (s.v[2154] < 230.25850929940458);s.store_scalar(2169, if s.b[2169] { 1.0 } else { 0.0 });
        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2169]) {s.store_exp(2140, 2154);s.store_div_from_scalar(2141, 1.0, 2140);s.store_mul(2140, 2046, 2140);}
        s.b[2170] = (s.v[2154] > (s.v[2031] - 230.25850929940458));s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });
        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && s.b[2170]) {s.store_exp_sub(2140, 2154, 2031);s.store_div(2141, 2046, 2140);}
        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && (!s.b[2170])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((!s.b[2165]) && (!s.b[2166])) {s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);s.store_mul_square_lhs(2142, 2154, 2130);s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);s.store_sub(2130, 2025, 2154);s.store_add_scaled_product_mixed_iia(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));s.store_add_scaled_inputs_mixed_ia(2045, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);}
        s.store_scalar(2048, 0.0);s.store_scalar(2049, 0.0);s.store_scalar(2050, 0.0);s.store_scalar(2051, 0.0);s.store_scalar(2052, 0.0);s.store_scalar(2053, 0.0);s.store_scalar(2054, 0.0);s.store_scalar(2055, 1.0);s.store_scalar(2056, 1.0);s.store_sub(2057, 2025, 2045);s.store_scalar(2058, 0.0);s.store_mul(2059, 2021, 2057);s.store_scalar(2060, 1.0);s.store_scalar(2061, 1.0);s.store_scalar(2065, 1.0);s.store_scalar(2066, 1.0);s.store_scalar(2068, 1.0);s.b[2171] = (s.v[2025] > 0.0);s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });
        if s.b[2171] {s.store_div_from_scalar_offset_square(1919, 1.0, 2045, 2.0);s.store_mul_square_lhs(2047, 2045, 1919);s.store_mul3_affine_lhs(2048, 2045, 1919, 4.0, 0.0, 1919);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2171] {s.store_mul_ad_product_lhs_mixed_ai(2049, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2047), 12.0), 1919, 1919);s.store_scalar(2050, 0.0);}
        s.b[2172] = (s.v[2045] < 230.25850929940458);s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });
        if (s.b[2171] && s.b[2172]) {s.store_exp(2050, 2045);s.store_div_from_scalar(2051, 1.0, 2050);s.store_mul(2050, 2046, 2050);}
        s.b[2173] = (s.v[2045] > (s.v[2031] - 230.25850929940458));s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });
        if ((s.b[2171] && (!s.b[2172])) && s.b[2173]) {s.store_exp_sub(2050, 2045, 2031);s.store_div(2051, 2046, 2050);}
        if ((s.b[2171] && (!s.b[2172])) && (!s.b[2173])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2050, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2051, 1e-100, 2045, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2171] {s.store_add_scaled_product_mixed_iia(2052, 2050, 1.0, 2046, A::add(A::offset(s.ad_value(2045), 1.0), s.ad_value(2047)), (-1.0));}
        s.b[2174] = (s.v[2045] < 1e-5);s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });
        if (s.b[2171] && s.b[2174]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2053, 2045, 1.0, 2045, 1.0, 2045, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2052, A::mul3(s.ad_value(2046), s.ad_value(2045), s.ad_value(2045)), 2045, A::scale_offset(s.ad_value(2045), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.25), 0.3333333333333333));s.store_scaled_mul(2054, 2045, 1919, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2055, 2006, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.5)), 1.0, A::square(s.ad_value(2045)), 0.16666666666666666), 0.7071067811865475, 1919, 1.0, 1.0);}
        if (s.b[2171] && (!s.b[2174])) {s.store_add_offset_lhs(2053, 2045, (-1.0), 2051);s.store_sqrt(2054, 2053);s.store_offset_scaled_ad(2055, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2051)), s.ad_value(2054)), 0.5, 1.0);}
        if s.b[2171] {s.store_div_scaled_offset_numerator(2056, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2005)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2005)), 1.0), 1.0);}
        s.b[2175] = (s.v[2052] > 1e-100);s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });
        if (s.b[2171] && s.b[2175]) {s.store_mul_sqrt_mixed_ia(2057, 2006, A::add(s.ad_value(2053), s.ad_value(2052)));s.store_div_scaled_product3_mixed_iiia(2058, 2007, 2052, 2021, 1.0, A::add_scaled_product(s.ad_value(2057), 1.0, s.ad_value(2006), s.ad_value(2054), 1.0), 1.0);s.store_mul3_lhs(2059, 2054, 2006, 2021);}
        s.b[2176] = (s.v[212] < 0.0);s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });
        if ((s.b[2171] && s.b[2175]) && s.b[2176]) {s.store_div_from_scalar_sub_from_scalar_ad(2060, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2005)));}
        if ((s.b[2171] && s.b[2175]) && (!s.b[2176])) {s.store_offset_mul(2060, 212, 2005, 1.0);}
        s.b[2177] = (s.v[213] < 0.0);s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });
        if ((s.b[2171] && s.b[2175]) && s.b[2177]) {s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2058, 1.0);}
        if ((s.b[2171] && s.b[2175]) && (!s.b[2177])) {s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2058, 1.0);}
        if (s.b[2171] && s.b[2175]) {s.store_mul_product3_indices(2062, 2058, 751, 2060, 2061, 1.0);s.store_mul_add_scaled_product_rhs_indices(2063, 768, 2059, 1.0, 769, 2058, 1.0);s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2053), 1.0, A::add(s.ad_value(2053), s.ad_value(2052)), 1e-14, 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2171] && s.b[2175]) {s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2063), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);s.store_mul_add_mixed_iai(2065, 2056, A::offset(s.ad_value(2064), 1.0), 2062);}
        s.b[2178] = (s.v[216] < 0.0);s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });
        if ((s.b[2171] && s.b[2175]) && s.b[2178]) {s.store_div_from_scalar_sub_from_scalar_ad(2066, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2005)));}
        if ((s.b[2171] && s.b[2175]) && (!s.b[2178])) {s.store_offset_mul(2066, 216, 2005, 1.0);}
        if (s.b[2171] && s.b[2175]) {s.store_mul(1921, 2058, 2066);s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);}
        s.b[2179] = (s.v[217] < 0.0);s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });
        if ((s.b[2171] && s.b[2175]) && s.b[2179]) {s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));}
        if ((s.b[2171] && s.b[2175]) && (!s.b[2179])) {s.store_offset_mul(2068, 217, 2067, 1.0);}
        s.copy_ad(1810, 2003);s.copy_ad(1811, 2005);s.copy_ad(1812, 2021);s.copy_ad(1813, 2022);s.copy_ad(1814, 2006);s.copy_ad(1815, 2007);s.copy_ad(1816, 2023);s.copy_ad(1817, 2025);s.copy_ad(1818, 2030);s.copy_ad(1819, 2031);s.copy_ad(1820, 2042);s.copy_ad(1821, 2043);s.copy_ad(1822, 2044);s.copy_ad(1823, 2151);s.copy_ad(1824, 2046);s.copy_ad(1825, 2045);s.copy_ad(1826, 2048);s.copy_ad(1827, 2049);s.copy_ad(1828, 2050);s.copy_ad(1829, 2051);s.copy_ad(1830, 2053);s.copy_ad(1831, 2052);s.copy_ad(1832, 2054);s.copy_ad(1833, 2055);s.copy_ad(1834, 2056);s.copy_ad(1835, 2057);s.copy_ad(1836, 2058);s.copy_ad(1837, 2059);s.copy_ad(1838, 2060);s.copy_ad(1839, 2061);s.copy_ad(1840, 2065);s.copy_ad(1841, 2066);s.copy_ad(1842, 2068);s.store_scalar(2070, 0.0);s.store_scale(2069, 2021, 4.60517018598809);s.copy_ad(2086, 2069);s.copy_ad(2087, 820);s.store_mul(2088, 820, 2022);s.copy_ad(2092, 2045);s.store_scalar(2093, 0.0);s.store_scalar(2096, 0.0);s.copy_ad(2098, 2051);s.copy_ad(2099, 2053);s.copy_ad(2101, 2052);s.copy_ad(2102, 2059);s.copy_ad(2103, 2045);s.copy_ad(2104, 2051);s.copy_ad(2106, 2052);s.copy_ad(2107, 2053);s.store_sub(2108, 2025, 2045);s.store_scalar(2109, 1.0);s.store_scalar(2111, 1.0);s.store_scalar(2110, 0.0);s.copy_ad(2120, 2058);s.store_mul(2124, 2108, 2021);s.store_scalar(2121, 0.0);s.copy_ad(2122, 2059);s.store_scalar(2127, 0.0);s.store_scalar(2126, 1.0);s.copy_ad(2129, 2001);s.copy_ad(2128, 2124);s.b[2180] = (s.v[2025] > 0.0);s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });s.b[2181] = (s.v[2052] > 1e-100);s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2181]) {s.store_mul(2129, 2001, 2068);s.store_div(2070, 2129, 2065);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && s.b[2181]) {s.store_add_scaled_inputs(2071, 2057, 1.0, 2007, 0.5);s.store_div_scaled_product_by_product_indices(1919, 2007, 2050, 1.0, 2071, 2071, 1.0);}
        s.b[2182] = (s.v[1919] > 0.0001);s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });
        if ((s.b[2180] && s.b[2181]) && s.b[2182]) {s.store_sub_from_scalar(1920, 1.0, 1919);}
        s.b[2183] = (s.v[1920] < 1e-10);s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });
        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && s.b[2183]) {s.store_scalar(1921, 1.0);}
        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && (!s.b[2183])) {s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));}
        if ((s.b[2180] && s.b[2181]) && (!s.b[2182])) {s.store_scale(1921, 1919, 0.5);}
        if (s.b[2180] && s.b[2181]) {s.store_mul(2072, 1921, 2071);}
        s.b[2184] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });
        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {s.store_scaled_mul(2073, 2021, 2072, 0.475);s.store_add_scaled_product_indices(1919, 2058, 1.0, 2055, 2073, (-1.0));s.store_scaled_add_mixed_ia(2074, 1919, A::sqrt_square_offset(s.ad_value(1919), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2075, 2058, (-1.0), 2021, 2057, 1.0, A::offset(s.ad_value(2055), (-1.0)), 2073, 1.0);s.store_offset_div_scaled_product_indices(2076, 2007, 2021, 0.5, 2075, 1.0, 1.0);s.store_add_scaled_product_indices(1919, 2075, 1.0, 769, 2074, 1.0);s.store_pow_ad(2077, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));s.store_mul_mixed_ai(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2076), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2077);s.store_div(1919, 2074, 2075);s.store_mul_pow_mixed_iaa(2078, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));s.store_mul_div_scaled_product_mixed_iiai(1921, 2078, 701, A::add(A::offset(s.ad_value(2076), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, 2075, 1.0);s.store_mul_product3_indices(2079, 2074, 751, 2060, 2061, 1.0);s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), s.ad_value(2076), (-1.0), s.ad_value(1921), 1.0), 1.0);}
        s.b[2185] = (s.v[1919] < 230.25850929940458);s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });
        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && s.b[2185]) {s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);}
        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && (!s.b[2185])) {s.copy_ad(1920, 1919);}
        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {s.store_div_scaled_product3_mixed_iiia(2080, 2073, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2077), 1.0, s.ad_value(2078), 1.0, s.ad_value(2079), 1.0, 1.0), 1.0);s.store_mul_scale_offset_mixed_ia(2081, 2072, A::div_scaled_value_offset_denominator(s.ad_value(2080), 1.0, A::sqrt_square_offset(s.ad_value(2080), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if ((s.b[2180] && s.b[2181]) && (!s.b[2184])) {s.copy_ad(2081, 2072);}
        if (s.b[2180] && s.b[2181]) {s.store_mul3_affine_lhs(2082, 2021, 2070, 0.7071067811865475, 0.0, 2081);}
        s.b[2186] = (s.v[0] == (-1.0));s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });
        if ((s.b[2180] && s.b[2181]) && s.b[2186]) {s.store_div_mixed_ia(2082, 2082, A::sqrt(A::offset(s.ad_value(2082), 1.0)));}
        if (s.b[2180] && s.b[2181]) {s.store_div_from_scalar_offset_ad(2083, 2.0, A::sqrt(A::scale_offset(s.ad_value(2082), 4.0, 1.0)), 1.0);s.store_mul(1919, 2083, 2082);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && s.b[2181]) {s.store_mul_ad_product_rhs_mixed_ia(2084, 2081, 2083, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2083)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2083), 4.0), 1.0)), 1.0));s.store_scale(2085, 2084, 0.99);s.store_div_scaled_product3_mixed_iaii(1919, 2085, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2071), 2.0), 2023, 1.0, 2052, 1.0);}
        if (s.b[2180] && s.b[2181]) {
            s.store_mul_sub_mixed_iia(2086, 2021, 2085, A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if (s.b[2180] && (!s.b[2181])) {s.copy_ad(2086, 2069);}
        if s.b[2180] {s.store_offset(1919, 2002, 1.0);s.store_div_scaled_product_mixed_aii(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2086, 1.0);s.store_add_mixed_ai(1921, A::square(s.ad_value(1920)), 1919);s.store_scale(1919, 1920, 2.0);s.store_div_scaled_product_add_scaled_denominator(2087, 2086, 1919, 1.0, A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), 1.0, A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919))), 1.0, 1.0);s.store_mul(2088, 2087, 2022);s.store_add(2089, 2031, 2088);}
        s.b[2187] = (s.v[2088] < 460.51701859880916);s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2187]) {s.store_exp_neg_input(2090, 2088);}
        if (s.b[2180] && (!s.b[2187])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2090, 1e-200, 2088, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2180] {s.store_mul(2091, 2046, 2090);}
        s.b[2188] = (((s.v[2025]) as f64).abs() <= s.v[2043]);s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2188]) {s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2092, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2091)), s.ad_value(2006), s.ad_value(2131)), 1.0));}
        if (s.b[2180] && (!s.b[2188])) {s.store_offset(2152, 2089, 3.0);s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));s.store_sub(2130, 2025, 2135);s.store_exp_neg_input(2131, 2135);s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);s.store_mul_square_lhs(2142, 2135, 2132);s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && (!s.b[2188])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2091] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }
        if (s.b[2180] && (!s.b[2188])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2138, 2089, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);s.store_add(818, 2136, 2137);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);s.store_add_mixed_ia(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));}
        s.b[2189] = (s.v[2154] < 230.25850929940458);s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });
        if ((s.b[2180] && (!s.b[2188])) && s.b[2189]) {s.store_exp(2140, 2154);s.store_div_from_scalar(2141, 1.0, 2140);s.store_mul(2140, 2091, 2140);}
        s.b[2190] = (s.v[2154] > (s.v[2089] - 230.25850929940458));s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });
        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && s.b[2190]) {s.store_exp_sub(2140, 2154, 2089);s.store_div(2141, 2091, 2140);}
        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && (!s.b[2190])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2180] && (!s.b[2188])) {s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);s.store_mul_square_lhs(2142, 2154, 2130);s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);s.store_sub(2130, 2025, 2154);s.store_add_scaled_product_mixed_iia(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && (!s.b[2188])) {s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));s.store_add_scaled_inputs_mixed_ia(2092, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);}
        if s.b[2180] {s.store_sub(2093, 2092, 2045);}
        s.b[2191] = (s.v[2093] < 1e-10);s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2191]) {s.store_add_scaled_inputs_product_mixed_iiia(2094, 2025, 2.0, 2045, (-2.0), 2007, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0), 1.0, s.ad_value(2091), s.ad_value(2048), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2095, A::mul_sub_from_scalar_rhs(s.ad_value(2007), 1.0, s.ad_value(2090)), 2052);s.store_sub_from_scalar_scaled_mul_mixed_ia(1919, 2.0, 2007, A::add_scaled_value_products(s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0, s.ad_value(2091), s.ad_value(2049), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(1919, 2094, 1.0, 1919, 2095, (-2.0));s.store_scaled_div_mixed_ia(2093, 2095, A::add(s.ad_value(2094), A::sqrt(s.ad_value(1919))), 2.0);s.store_add(2092, 2045, 2093);}
        if s.b[2180] {s.store_mul(2096, 2093, 2021);s.store_div_scaled_product_offset_denominator_mixed_iia(2097, 2092, 2092, 1.0, A::square(s.ad_value(2092)), 2.0, 1.0);}
        s.b[2192] = (s.v[2092] < 230.25850929940458);s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2192]) {s.store_exp_neg_input(2098, 2092);}
        s.b[2193] = (s.v[2092] < 1e-5);s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });
        if ((s.b[2180] && s.b[2192]) && s.b[2193]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2099, 2092, 1.0, 2092, 1.0, 2092, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2092), 1.0, A::scale(s.ad_value(2092), 0.25), 0.3333333333333333));s.store_scaled_mul(2100, 2092, 1919, 0.7071067811865475);s.store_mul3_ad_middle(2101, A::mul3_scaled_output(s.ad_value(2091), s.ad_value(2092), s.ad_value(2092), 0.16666666666666666), 2092, A::scale_offset(s.ad_value(2092), 1.75, 1.0));}
        if ((s.b[2180] && s.b[2192]) && (!s.b[2193])) {s.store_add_offset_lhs(2099, 2092, (-1.0), 2098);s.store_sqrt(2100, 2099);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2101, 2091, A::div_from_scalar(1.0, s.ad_value(2098)), 1.0, 2092, (-1.0), 2097, -1.0, (-1.0));}
        s.b[2194] = (s.v[2092] > (s.v[2089] - 230.25850929940458));s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });
        if ((s.b[2180] && (!s.b[2192])) && s.b[2194]) {s.store_exp_sub(1919, 2092, 2089);s.store_div(2098, 2091, 1919);s.store_add_scaled_product_mixed_iia(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));}
        if ((s.b[2180] && (!s.b[2192])) && (!s.b[2194])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2098, 1e-100, 2092, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1919, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
    }
}
