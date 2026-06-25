#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[1149] != 0.0)) && (!(s.v[1150] != 0.0))) && (!(s.v[1151] != 0.0))) {
            s.copy_ad(793, 783);
        }

        s.store_sub_ad(794, A::add(s.ad_value(822), A::scale(s.ad_value(785), 0.5)), A::mul(s.ad_value(783), A::sqrt(A::add(A::add(s.ad_value(822), A::scale(s.ad_value(785), 0.25)), s.ad_value(793)))));

        s.store_add_ad(728, A::offset(s.ad_value(187), s.v[362]), A::scale(A::ln(A::scale(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26)), (2.0 * s.v[715])));

        if !(s.v[728] > 0.05) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt(A::scale(s.ad_value(183), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);

        s.v[730] = 0.0;

        s.v[731] = 0.0;

        s.v[1152] = if (s.v[188] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if (s.v[1152] != 0.0) {
            s.store_ad(731, &{
                if (s.v[188] > s.v[732]) {
                    s.ad_value(188)
                } else {
                    s.ad_value(732)
                }
            });
        }

        if (s.v[1152] != 0.0) {
            s.store_ad(731, &{
                if (5e24 > s.v[731]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(731)
                }
            });
        }

        if (s.v[1152] != 0.0) {
            s.store_div_ad(730, A::scale(A::mul(A::scale(s.ad_value(769), 2.0), s.ad_value(769)), s.v[715]), A::scale(s.ad_value(731), (1.6021918e-19 * s.v[767])));
        }

        s.v[733] = ((100.0 * s.v[715]) * s.v[715]);

        s.v[1153] = if (p.p52 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_sqrt_ad(734, A::mul(A::mul(A::scale(s.ad_value(729), s.v[715]), s.ad_value(729)), s.ad_value(728)));
        }

        if (s.v[1153] != 0.0) {
            s.store_mul_ad(735, A::scale(s.ad_value(773), 0.75), A::powf(s.ad_value(734), 0.6666666666666666));
        }

        if (s.v[1153] != 0.0) {
            s.store_add(728, 728, 735);
        }

        if (s.v[1153] != 0.0) {
            s.store_mul_ad_rhs(729, 729, A::offset(A::div(A::scale(s.ad_value(735), (2.0 * 0.6666666666666666)), s.ad_value(734)), 1.0));
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_mul_ad_lhs(738, A::scale(s.ad_value(728), 0.0025), 728);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_scale_ad(741, A::sub(A::sub(s.ad_value(737), s.ad_value(740)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(737), s.ad_value(740)), A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)))), 0.5);

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_sub_ad_lhs(744, A::sub(A::sqrt(A::add(A::add(s.ad_value(185), s.ad_value(186)), s.ad_value(728))), s.ad_value(736)), 743);

        s.store_add_ad(745, A::add(A::offset(s.ad_value(187), s.v[362]), s.ad_value(256)), A::scale(A::ln(A::scale(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26)), (2.0 * s.v[715])));

        if !(s.v[745] > 0.05) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt(A::scale(s.ad_value(772), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);

        s.v[1154] = if (p.p52 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_sqrt_ad(734, A::mul(A::mul(A::scale(s.ad_value(746), s.v[715]), s.ad_value(746)), s.ad_value(745)));
        }

        if (s.v[1154] != 0.0) {
            s.store_mul_ad(735, A::scale(s.ad_value(773), 0.75), A::powf(s.ad_value(734), 0.6666666666666666));
        }

        if (s.v[1154] != 0.0) {
            s.store_add(745, 745, 735);
        }

        if (s.v[1154] != 0.0) {
            s.store_mul_ad_rhs(746, 746, A::offset(A::div(A::scale(s.ad_value(735), (2.0 * 0.6666666666666666)), s.ad_value(734)), 1.0));
        }

        s.store_scale(747, 745, 0.95);

        s.store_mul_ad_lhs(748, A::scale(s.ad_value(745), 0.0025), 745);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_scale_ad(750, A::sub(A::sub(s.ad_value(747), s.ad_value(740)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(747), s.ad_value(740)), A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)))), 0.5);

        s.store_offset_ad(700, A::add(s.ad_value(177), A::mul(A::scale(s.ad_value(178), s.v[358]), A::offset(A::scale(s.ad_value(179), s.v[358]), 1.0))), s.v[21]);

        s.store_exp_ad(751, A::scale(s.ad_value(180), s.v[360]));

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_ad(752, A::scale(s.ad_value(203), s.v[360]));

        s.store_mul(703, 202, 752);

        s.store_mul_ad_lhs(716, A::scale(s.ad_value(703), s.v[20]), 769);

        s.store_mul_ad_rhs(705, 206, A::exp(A::scale(s.ad_value(207), s.v[360])));

        s.store_exp_ad(753, A::scale(s.ad_value(205), s.v[360]));

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp(A::scale(s.ad_value(211), s.v[360])));

        s.store_exp_ad(754, A::scale(s.ad_value(209), s.v[360]));

        s.store_mul(706, 208, 754);

        s.store_exp_ad(755, A::scale(s.ad_value(213), s.v[360]));

        s.store_mul(708, 212, 755);

        s.store_exp_ad(756, A::scale(s.ad_value(216), s.v[360]));

        s.store_mul(709, 215, 756);

        s.store_mul_ad_lhs(757, A::scale(s.ad_value(716), 2.0), 709);

        s.store_exp_ad(758, A::scale(s.ad_value(220), s.v[360]));

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp(A::scale(A::neg(s.ad_value(231)), s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.v[1155] = if ((p.p46 != 0.0) && (s.v[287] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(713, A::add(s.ad_value(282), A::scale(s.ad_value(283), s.v[358])), s.v[23]);
        }

        if (s.v[1155] != 0.0) {
            s.store_exp_ad(759, A::scale(s.ad_value(288), s.v[360]));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul(714, 287, 759);
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_lhs(717, A::scale(s.ad_value(714), s.v[22]), 769);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale_ad(723, A::offset(A::scale(s.ad_value(286), s.v[359]), 1.0), s.v[715]);
        }

        if (s.v[1155] != 0.0) {
            s.store_add_ad(760, A::offset(s.ad_value(284), s.v[362]), A::mul(A::scale(s.ad_value(723), 2.0), A::ln(A::scale(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26))));
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(760, &{
                if (s.v[760] > 0.05) {
                    s.ad_value(760)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[1155] != 0.0) {
            s.store_div_ad_lhs(761, A::sqrt(A::scale(s.ad_value(285), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);
        }

        if (s.v[1155] != 0.0) {
            s.store_square(724, 761);
        }

        if (s.v[1155] != 0.0) {
            s.store_ln(725, 724);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(762, 760, 0.95);
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_lhs(763, A::scale(s.ad_value(760), 0.0025), 760);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(764, 763);
        }

        if (s.v[1155] != 0.0) {
            s.store_scaled_sqrt(765, 764, 0.5);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale_ad(766, A::sub(A::sub(s.ad_value(762), s.ad_value(765)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(762), s.ad_value(765)), A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)))), 0.5);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(713, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(759, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(714, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(717, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(723, s.v[715]);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(760, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(761, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(725, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(762, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(763, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(764, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(765, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(766, 0.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scale_ad(796, A::sqrt(A::scale(s.ad_value(246), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.v[800] = 0.0;

        s.v[1156] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_div_ad_lhs(800, A::scale(s.ad_value(240), (-0.495)), 241);
        }

        s.v[801] = 0.0;

        s.v[1157] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_div_ad_lhs(801, A::scale(s.ad_value(242), (-0.495)), 243);
        }

        s.v[1158] = if (s.v[245] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_div_ad_lhs(802, A::scale(s.ad_value(244), (-0.495)), 245);
        }

        s.store_ad(803, &A::pow_from_scalar(s.v[352], s.ad_value(239)));

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

        s.v[808] = 0.0;

        s.v[1159] = if (s.v[272] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1159] != 0.0) {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.v[30] = 0.0;

        s.v[1167] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1167] != 0.0) {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.v[1168] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1168] != 0.0) {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.v[1169] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1169] != 0.0) {
            s.store_scale(24, 6, s.v[649]);
        }

        if (s.v[1169] != 0.0) {
            s.store_sub_ad(25, A::scale(s.ad_value(6), s.v[650]), A::mul(s.ad_value(30), s.ad_value(31)));
        }

        if (s.v[1169] != 0.0) {
            s.copy_ad(26, 31);
        }

        if (s.v[1169] != 0.0) {
            s.store_scale(27, 6, s.v[676]);
        }

        if (s.v[1169] != 0.0) {
            s.store_sub_ad(28, A::scale(s.ad_value(6), s.v[677]), A::mul(s.ad_value(30), s.ad_value(31)));
        }

        if (s.v[1169] != 0.0) {
            s.copy_ad(29, 31);
        }

        s.v[1170] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1170] != 0.0) {
            s.store_ad(646, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(647, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(648, &{
                if (s.v[26] > 0.0) {
                    s.ad_value(26)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(673, &{
                if (s.v[27] > 0.0) {
                    s.ad_value(27)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(674, &{
                if (s.v[28] > 0.0) {
                    s.ad_value(28)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(675, &{
                if (s.v[29] > 0.0) {
                    s.ad_value(29)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(673, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(674, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(675, 0.0);
        }

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[657] = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[654] = 0.0;

        s.v[681] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[651] = 1.0;

        s.v[678] = 1.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[501] = 0.0;

        s.v[1171] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1172] = if ((s.v[387] * s.v[646]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(454, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1172] != 0.0))) {
            s.store_scalar(454, 100000000.0);
        }

        s.v[1173] = if ((s.v[388] * s.v[647]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1173] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1174] = if ((s.v[389] * s.v[648]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1174] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(654, &A::min(A::min(s.ad_value(454), s.ad_value(455)), s.ad_value(456)));
        }

        s.v[1175] = if ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_exp_ad(655, A::scale(s.ad_value(654), s.v[371]));
        }

        s.v[1176] = if ((s.v[654] * s.v[371]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (!(s.v[1175] != 0.0))) && (s.v[1176] != 0.0)) {
            s.store_div_from_scalar_ad(655, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1171] != 0.0) && (!(s.v[1175] != 0.0))) && (!(s.v[1176] != 0.0))) {
            s.store_scale_ad(655, A::offset(A::mul(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(396, s.v[393]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(397, s.v[394]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(398, s.v[395]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(399, p.p831);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(400, p.p832);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(401, p.p833);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(402, p.p828);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(403, p.p829);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(404, p.p830);
        }

        s.v[1177] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.v[1178] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.v[1179] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(656, &A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)));
        }

        if (s.v[1171] != 0.0) {
            s.store_scale(657, 656, 0.1);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(377, &A::max(A::max(s.ad_value(399), s.ad_value(400)), s.ad_value(401)));
        }

        if (s.v[1171] != 0.0) {
            s.store_mul_ad_rhs(658, 656, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377)))));
        }

        if (s.v[1171] != 0.0) {
            s.store_offset_ad(659, A::min(A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404)), (-0.05));
        }

        s.v[1180] = if ((s.v[563] * s.v[673]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1180] != 0.0)) {
            s.store_scale_ad(454, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1180] != 0.0))) {
            s.store_scalar(454, 100000000.0);
        }

        s.v[1181] = if ((s.v[564] * s.v[674]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1181] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1182] = if ((s.v[565] * s.v[675]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(681, &A::min(A::min(s.ad_value(454), s.ad_value(455)), s.ad_value(456)));
        }

        s.v[1183] = if ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1183] != 0.0)) {
            s.store_exp_ad(682, A::scale(s.ad_value(681), s.v[371]));
        }

        s.v[1184] = if ((s.v[681] * s.v[371]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (!(s.v[1183] != 0.0))) && (s.v[1184] != 0.0)) {
            s.store_div_from_scalar_ad(682, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1171] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_scale_ad(682, A::offset(A::mul(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(396, 569);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(397, 570);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(398, 571);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(399, 511);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(400, 512);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(401, 513);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(402, 508);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(403, 509);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(404, 510);
        }

        s.v[1185] = if (s.v[673] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_add(396, 570, 571);
        }

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_add(402, 509, 510);
        }

        s.v[1186] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_add(397, 569, 571);
        }

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_add(403, 508, 510);
        }

        s.v[1187] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_add(398, 569, 570);
        }

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_add(404, 508, 509);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(683, &A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)));
        }

        if (s.v[1171] != 0.0) {
            s.store_scale(684, 683, 0.1);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(377, &A::max(A::max(s.ad_value(399), s.ad_value(400)), s.ad_value(401)));
        }

        if (s.v[1171] != 0.0) {
            s.store_mul_ad_rhs(685, 683, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377)))));
        }

        if (s.v[1171] != 0.0) {
            s.store_offset_ad(686, A::min(A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404)), (-0.05));
        }

        s.v[1188] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scale_ad(501, A::add(A::add(A::scale(s.ad_value(646), s.v[414]), A::scale(s.ad_value(647), s.v[415])), A::scale(s.ad_value(648), s.v[416])), p.p929);
        }

        s.v[1523] = if ((s.v[646] * s.v[414]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1523] != 0.0)) {
            s.store_scalar(651, 0.0);
        }

        s.v[1524] = if ((s.v[647] * s.v[415]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1524] != 0.0)) {
            s.store_scalar(652, 0.0);
        }

        s.v[1525] = if ((s.v[648] * s.v[416]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1525] != 0.0)) {
            s.store_scalar(653, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_mul_ad_rhs(501, 553, A::add(A::add(A::mul(s.ad_value(673), s.ad_value(581)), A::mul(s.ad_value(674), s.ad_value(582))), A::mul(s.ad_value(675), s.ad_value(583))));
        }

        s.v[1813] = if ((s.v[673] * s.v[581]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1813] != 0.0)) {
            s.store_scalar(678, 0.0);
        }

        s.v[1814] = if ((s.v[674] * s.v[582]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1814] != 0.0)) {
            s.store_scalar(679, 0.0);
        }

        s.v[1815] = if ((s.v[675] * s.v[583]) <= s.v[501]) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1815] != 0.0)) {
            s.store_scalar(680, 0.0);
        }

        s.v[2027] = 0.0;

        s.v[2028] = 0.0;

        s.v[2029] = 0.0;

        s.v[1937] = 1.0;

        s.v[1936] = 0.0;

        s.v[2102] = if (s.v[0] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[2102] != 0.0) {
            s.store_ad(825, &A::voltage(ctx, &nodes, Some(5), Some(6)));
        }

        if (s.v[2102] != 0.0) {
            s.store_ad(826, &A::voltage(ctx, &nodes, Some(7), Some(6)));
        }

        if (s.v[2102] != 0.0) {
            s.store_ad(827, &A::voltage(ctx, &nodes, Some(6), Some(8)));
        }

        if (s.v[2102] != 0.0) {
            s.store_ad(832, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(10))));
        }

        if (s.v[2102] != 0.0) {
            s.store_ad(833, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(11))));
        }

        if (!(s.v[2102] != 0.0)) {
            s.store_ad(825, &A::neg(A::voltage(ctx, &nodes, Some(5), Some(6))));
        }

        if (!(s.v[2102] != 0.0)) {
            s.store_ad(826, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(6))));
        }

        if (!(s.v[2102] != 0.0)) {
            s.store_ad(827, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(8))));
        }

        if (!(s.v[2102] != 0.0)) {
            s.store_ad(832, &A::voltage(ctx, &nodes, Some(6), Some(10)));
        }

        if (!(s.v[2102] != 0.0)) {
            s.store_ad(833, &A::voltage(ctx, &nodes, Some(7), Some(11)));
        }

        s.store_add(829, 825, 827);

        s.copy_ad(834, 825);

        s.copy_ad(835, 827);

        s.store_add(836, 826, 827);

        s.store_sub(837, 825, 826);

        s.store_scale_ad(1817, A::neg(s.ad_value(834)), s.v[355]);

        s.store_scale_ad(1818, A::neg(s.ad_value(837)), s.v[355]);

        s.store_scale_ad(1819, A::neg(A::sub(s.ad_value(829), s.ad_value(700))), s.v[355]);

        s.v[831] = 1.0;

        s.v[2103] = if (s.v[826] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2103] != 0.0) {
            s.store_scalar(831, (-1.0));
        }

        if (s.v[2103] != 0.0) {
            s.store_sub(825, 825, 826);
        }

        if (s.v[2103] != 0.0) {
            s.store_add(827, 827, 826);
        }

        if (s.v[2103] != 0.0) {
            s.store_neg(826, 826);
        }

        s.store_add(828, 826, 827);

        s.store_div_ad(830, A::square(s.ad_value(826)), A::offset(A::sqrt(A::offset(A::square(s.ad_value(826)), 0.01)), 0.1));

        s.store_add_ad_lhs(2107, A::scale(A::sub(A::add(s.ad_value(828), s.ad_value(827)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739)))), 0.5), 737);

        s.copy_ad(1820, 2107);

        s.store_add_ad_lhs(2030, A::sub(s.ad_value(827), A::scale(A::sub(s.ad_value(2107), A::sqrt(A::add(A::mul(s.ad_value(2107), s.ad_value(2107)), s.ad_value(738)))), 0.5)), 741);

        s.copy_ad(1821, 2030);

        s.v[2031] = 0.0;

        s.v[2263] = if ((p.p45 != 0.0) && (s.v[184] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[2263] != 0.0) {
            s.store_add_ad_rhs(2032, 2030, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));
        }

        if (s.v[2263] != 0.0) {
            s.store_sub_ad_lhs(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);
        }

        if (s.v[2263] != 0.0) {
            s.store_offset_ad(2027, A::div(A::scale(A::sub(s.ad_value(2033), s.ad_value(743)), 2.0), s.ad_value(744)), (-1.0));
        }

        if (s.v[2263] != 0.0) {
            s.store_sub_ad_rhs(2034, 2033, A::mul(A::mul(A::scale(A::sub_from_scalar(1.0, s.ad_value(184)), 0.25), s.ad_value(744)), A::add(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 0.4804530139182)))));
        }

        if (s.v[2263] != 0.0) {
            s.store_add_ad(2035, A::square(s.ad_value(2034)), A::mul(A::scale(s.ad_value(736), 2.0), s.ad_value(2034)));
        }

        if (s.v[2263] != 0.0) {
            s.store_sub_ad_rhs(2030, 2035, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));
        }

        if (s.v[2263] != 0.0) {
            s.store_sub(2031, 1821, 2030);
        }

        s.copy_ad(2104, 728);

        s.copy_ad(2105, 738);

        s.copy_ad(2106, 729);

        s.copy_ad(2108, 2030);

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.copy_ad(2112, 2031);

        s.copy_ad(2109, 720);

        s.copy_ad(2110, 777);

        s.store_sub_ad_lhs(2111, A::sub(s.ad_value(829), s.ad_value(2112)), 700);

        s.store_add_ad_rhs(2113, 2108, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));

        s.v[2125] = 1.0;

        s.v[2264] = if (s.v[190] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2264] != 0.0) {
            s.store_scale(2116, 2104, s.v[361]);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale(2117, 2113, s.v[361]);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale(2118, 2111, s.v[361]);
        }

        if (s.v[2264] != 0.0) {
            s.store_offset_ad(2028, A::div(A::scale(s.ad_value(2106), 0.5), A::sqrt(s.ad_value(2116))), 1.0);
        }

        if (s.v[2264] != 0.0) {
            s.store_add_ad_rhs(2029, 2116, A::mul(s.ad_value(2106), A::sqrt(s.ad_value(2116))));
        }

        if (s.v[2264] != 0.0) {
            s.store_sub_ad(2119, A::add(A::div(A::sub(s.ad_value(2118), s.ad_value(2029)), s.ad_value(2028)), A::scale(s.ad_value(2116), 0.5)), A::mul(A::offset(s.ad_value(191), 1.0), s.ad_value(2117)));
        }

        if (s.v[2264] != 0.0) {
            s.store_offset_scaled(2120, 2116, 0.5, 2.0);
        }

        if (s.v[2264] != 0.0) {
            s.store_add(2121, 2116, 2117);
        }

        if (s.v[2264] != 0.0) {
            s.store_sub_ad(2028, A::sub(A::sub(s.ad_value(2118), s.ad_value(2121)), A::mul(s.ad_value(2106), A::sqrt(s.ad_value(2121)))), A::scale(A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0));
        }

        if (s.v[2264] != 0.0) {
            s.store_add_ad_lhs(2122, A::scale(s.ad_value(2028), 2.0), 2120);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale_ad(2028, A::add(A::add(s.ad_value(2119), s.ad_value(2122)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2119), s.ad_value(2122)), A::sub(s.ad_value(2119), s.ad_value(2122))), 20.0))), 0.5);
        }

        if (s.v[2264] != 0.0) {
            s.store_sub_ad_lhs(2029, A::scale(A::sub(s.ad_value(2118), s.ad_value(2117)), 2.0), 2120);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale_ad(2123, A::sub(A::add(s.ad_value(2028), s.ad_value(2029)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0))), 0.5);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale_ad(2028, A::sub(A::add(s.ad_value(2123), s.ad_value(2120)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2123), s.ad_value(2120)), A::sub(s.ad_value(2123), s.ad_value(2120))), 5.0))), 0.5);
        }

        if (s.v[2264] != 0.0) {
            s.store_scale_ad(2124, A::add(A::sub(s.ad_value(2028), s.ad_value(2120)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), A::neg(s.ad_value(2120))), A::sub(s.ad_value(2028), A::neg(s.ad_value(2120)))), 20.0))), 0.5);
        }

        if (s.v[2264] != 0.0) {
            s.store_mul_ad_rhs(2029, 702, A::offset(A::div(s.ad_value(2124), s.ad_value(2120)), 1.0));
        }

        s.v[2265] = if (s.v[2029] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((s.v[2264] != 0.0) && (s.v[2265] != 0.0)) {
            s.store_exp(2125, 2029);
        }

        if ((s.v[2264] != 0.0) && (!(s.v[2265] != 0.0))) {
            s.store_div_from_scalar_ad(2125, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.store_offset_ad(2126, A::mul(s.ad_value(701), s.ad_value(2125)), 1.0);

        s.store_scale(2127, 2126, s.v[715]);

        s.store_mul_ad(2128, A::mul(s.ad_value(199), A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0)), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));

        s.store_mul_ad_rhs(2129, 2127, A::offset(s.ad_value(2128), 1.0));

        s.store_div_from_scalar(2130, 1.0, 2129);

        s.store_mul_ad_rhs(2114, 2106, A::sqrt(A::scale(s.ad_value(2130), s.v[715])));

        s.store_square(2115, 2114);

        s.store_div_from_scalar(2131, 1.0, 2115);

        s.store_mul(2132, 2108, 2130);

        s.store_mul(2133, 2111, 2130);

        s.store_div_ad(2134, A::scale(s.ad_value(830), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0));

        s.store_mul_ad(2135, A::mul(s.ad_value(196), s.ad_value(2134)), A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));

        s.store_mul(2136, 2104, 2130);

        s.store_sqrt_ad(2028, A::add(A::square(s.ad_value(2107)), s.ad_value(2105)));

        s.store_sqrt_ad(2029, A::add(A::mul(A::sub(s.ad_value(2107), s.ad_value(2135)), A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105)));

        s.store_mul_ad(2137, A::scale(s.ad_value(2130), 0.5), A::sub(A::add(s.ad_value(2135), s.ad_value(2028)), s.ad_value(2029)));

        s.store_add(2138, 2136, 2132);

        s.store_sub(2139, 2138, 2137);

        s.v[2266] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2267] = if (((s.v[2139]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2266] != 0.0) && (s.v[2267] != 0.0)) {
            s.store_offset_ad(2140, A::mul(s.ad_value(2114), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2139), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2139), 0.3125))))), 1.0);
        }

        s.v[2268] = if (s.v[2139] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2266] != 0.0) && (!(s.v[2267] != 0.0))) && (s.v[2268] != 0.0)) {
            s.store_exp_ad(2154, A::neg(s.ad_value(2139)));
        }

        if (((s.v[2266] != 0.0) && (!(s.v[2267] != 0.0))) && (!(s.v[2268] != 0.0))) {
            s.store_div_from_scalar_ad(2154, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2139), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2139), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2139), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2266] != 0.0) && (!(s.v[2267] != 0.0))) {
            s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[2266] != 0.0) && (!(s.v[2267] != 0.0))) {
            s.store_offset_ad(2140, A::div(A::mul(A::mul(s.ad_value(2027), s.ad_value(2114)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2154), A::sub_from_scalar(1.0, s.ad_value(2139))))), A::scale(A::sqrt(A::mul(s.ad_value(2139), A::sub_from_scalar(1.0, s.ad_value(2154)))), 2.0)), 1.0);
        }

        if (!(s.v[2266] != 0.0)) {
            s.store_offset_ad(2140, A::div(A::scale(s.ad_value(2114), 0.5), A::sqrt(s.ad_value(2139))), 1.0);
        }

        s.store_sub_ad(2141, A::add(s.ad_value(2139), A::mul(s.ad_value(2114), A::sqrt(s.ad_value(2139)))), A::mul(s.ad_value(2140), A::ln(A::offset(s.ad_value(2140), (-1.0)))));

        s.store_div_ad_lhs(2142, A::sub(s.ad_value(2133), s.ad_value(2141)), 2140);

        s.store_mul_ad(2148, A::scale(s.ad_value(2115), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0)));

        s.v[2147] = 0.0;

        s.v[2149] = 1.0;

        s.v[2269] = if (s.v[2142] > (-30.0)) { 1.0 } else { 0.0 };

        if (s.v[2269] != 0.0) {
            s.store_offset_ad(2143, A::mul(s.ad_value(2140), s.ad_value(2142)), (-1.0));
        }

        if (s.v[2269] != 0.0) {
            s.store_scale_ad(2027, A::add(s.ad_value(2143), A::sqrt(A::offset(A::square(s.ad_value(2143)), 10.0))), 0.5);
        }

        if (s.v[2269] != 0.0) {
            s.store_sub_ad_rhs(2144, 2142, A::ln(s.ad_value(2027)));
        }

        if (s.v[2269] != 0.0) {
            s.store_scale_ad(2145, A::add(s.ad_value(2144), A::sqrt(A::offset(A::square(s.ad_value(2144)), 2.0))), 0.5);
        }

        s.v[2270] = if ((s.v[2142] - s.v[2145]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2269] != 0.0) && (s.v[2270] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)));
        }

        if ((s.v[2269] != 0.0) && (!(s.v[2270] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[2269] != 0.0) {
            s.store_div(2146, 2027, 2140);
        }

        if (s.v[2269] != 0.0) {
            s.store_sub_ad_lhs(2027, A::scale(A::offset(s.ad_value(2145), 1.0), 2.0), 2146);
        }

        s.v[2271] = if (s.v[2146] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[2269] != 0.0) && (s.v[2271] != 0.0)) {
            s.store_mul_ad_rhs(2147, 2140, A::offset(A::sub(s.ad_value(2145), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2146), s.ad_value(2027)), 1.0)), (-1.0)), s.ad_value(2146))), 1.0));
        }

        if ((s.v[2269] != 0.0) && (!(s.v[2271] != 0.0))) {
            s.store_mul_ad(2147, A::mul(A::scale(s.ad_value(2140), 0.5), s.ad_value(2146)), A::offset(A::mul(A::scale(s.ad_value(2027), 0.25), s.ad_value(2027)), 1.0));
        }

        if (s.v[2269] != 0.0) {
            s.store_scale_ad(2027, A::add(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0))), 1.0))), 0.5);
        }

        if (s.v[2269] != 0.0) {
            s.store_mul_ad(2148, A::scale(s.ad_value(2115), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027)), 1.0)), (-1.0)));
        }

        if (s.v[2269] != 0.0) {
            s.store_div_ad_rhs(2149, 2148, A::add(s.ad_value(2148), s.ad_value(2147)));
        }

        if (s.v[2269] != 0.0) {
            s.store_sub_ad_rhs(2139, 2138, A::mul(s.ad_value(2149), s.ad_value(2137)));
        }

        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);

        s.store_scale(2151, 2150, 1e-5);

        s.store_div_from_scalar(2152, 1.0, 2150);

        s.v[2259] = 0.0;

        s.v[2153] = 0.0;

        s.v[2272] = if (s.v[2139] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[2272] != 0.0) {
            s.store_exp_ad(2154, A::neg(s.ad_value(2139)));
        }

        if (!(s.v[2272] != 0.0)) {
            s.store_div_from_scalar_ad(2154, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2139), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2139), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2139), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2273] = if (((s.v[2133]) as f64).abs() <= s.v[2151]) { 1.0 } else { 0.0 };

        if (s.v[2273] != 0.0) {
            s.store_scale_ad(2239, A::square(s.ad_value(2152)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (s.v[2273] != 0.0) {
            s.store_mul_ad(2153, A::mul(s.ad_value(2133), s.ad_value(2152)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2133), A::sub_from_scalar(1.0, s.ad_value(2154))), s.ad_value(2114)), s.ad_value(2239)), 1.0));
        }

        s.v[2274] = if (s.v[2133] < (-s.v[2151])) { 1.0 } else { 0.0 };

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_neg(2241, 2133);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_scaled_mul(2242, 2241, 2152, 1.25);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_scale_ad(2243, A::sub(A::offset(s.ad_value(2242), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2242), (-6.0)), A::offset(s.ad_value(2242), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub(2238, 2241, 2243);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_add_ad(2244, A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::offset(s.ad_value(2243), 1.0)));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_ad_lhs(2245, A::scale(s.ad_value(2238), 2.0), 2115);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_ad_lhs(2246, A::ln(A::mul(s.ad_value(2244), s.ad_value(2131))), 2243);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_add(824, 2244, 2245);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2246), A::sub(A::scale(A::square(s.ad_value(2245)), 0.5), s.ad_value(2244))));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_add_ad_rhs(2247, 2243, A::div(A::mul(A::mul(s.ad_value(2244), s.ad_value(824)), s.ad_value(2246)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246)), s.ad_value(2246)), s.ad_value(2245)), A::sub(A::scale(A::square(s.ad_value(2245)), 0.3333333333333333), s.ad_value(2244))))));
        }

        s.v[2275] = if (s.v[2247] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) && (s.v[2275] != 0.0)) {
            s.store_exp(2248, 2247);
        }

        if (((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) && (!(s.v[2275] != 0.0))) {
            s.store_scale_ad(2248, A::offset(A::mul(A::offset(s.ad_value(2247), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2247), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2247), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_div_from_scalar(2249, 1.0, 2248);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_div_from_scalar_ad(2238, 1.0, A::offset(A::square(s.ad_value(2247)), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_mul_ad_lhs(2250, A::square(s.ad_value(2247)), 2238);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_scale_ad(2251, A::mul(A::mul(s.ad_value(2247), s.ad_value(2238)), s.ad_value(2238)), 4.0);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_mul_ad_lhs(2252, A::mul(A::sub(A::scale(s.ad_value(2238), 8.0), A::scale(s.ad_value(2250), 12.0)), s.ad_value(2238)), 2238);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub(2238, 2241, 2247);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_mul(2239, 2154, 2249);
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_add_ad(2253, A::scale(s.ad_value(2238), 2.0), A::mul(s.ad_value(2115), A::add(A::sub(A::offset(s.ad_value(2248), (-1.0)), s.ad_value(2239)), A::mul(s.ad_value(2154), A::sub_from_scalar(1.0, s.ad_value(2251))))));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_ad(2254, A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::add(A::add(A::offset(A::sub(s.ad_value(2248), s.ad_value(2247)), (-1.0)), s.ad_value(2239)), A::mul(s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250))))));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::sub(A::add(s.ad_value(2248), s.ad_value(2239)), A::mul(s.ad_value(2154), s.ad_value(2252)))));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_ad(2238, A::square(s.ad_value(2253)), A::scale(A::mul(s.ad_value(2254), s.ad_value(2238)), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (s.v[2274] != 0.0)) {
            s.store_sub_ad(2153, A::neg(s.ad_value(2247)), A::scale(A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_div_from_scalar_ad(2255, 1.0, A::offset(A::scale(s.ad_value(2114), 0.7324648775608221), 1.25));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad_lhs(2256, A::offset(A::mul(A::scale(s.ad_value(2150), 1.25), s.ad_value(2255)), (-1.0)), 2255);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad(2257, A::mul(s.ad_value(2133), s.ad_value(2152)), A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));
        }

        s.v[2276] = if ((-s.v[2257]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (s.v[2276] != 0.0)) {
            s.store_exp_ad(2238, A::neg(s.ad_value(2257)));
        }

        if (((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (!(s.v[2276] != 0.0))) {
            s.store_div_from_scalar_ad(2238, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2257))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2257))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2257))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_from_scalar(2258, 1.0, 2238);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_ad(2259, A::add(s.ad_value(2133), A::scale(s.ad_value(2115), 0.5)), A::mul(s.ad_value(2114), A::sqrt(A::sub(A::add(s.ad_value(2133), A::scale(s.ad_value(2115), 0.25)), s.ad_value(2258)))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_offset(2260, 2139, 3.0);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_ad(2243, A::scale(A::sub(A::add(s.ad_value(2259), s.ad_value(2260)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2259), s.ad_value(2260)), A::sub(s.ad_value(2259), s.ad_value(2260))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2260), A::sqrt(A::offset(A::square(s.ad_value(2260)), 5.0))), 0.5));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub(2238, 2133, 2243);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_exp_ad(2239, A::neg(s.ad_value(2243)));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_div_from_scalar_ad(2240, 1.0, A::offset(A::square(s.ad_value(2243)), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad_lhs(2250, A::square(s.ad_value(2243)), 2240);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_scale_ad(2251, A::mul(A::mul(s.ad_value(2243), s.ad_value(2240)), s.ad_value(2240)), 4.0);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad_lhs(2252, A::mul(A::sub(A::scale(s.ad_value(2240), 8.0), A::scale(s.ad_value(2250), 12.0)), s.ad_value(2240)), 2240);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            let assign42250_ad_e55460: A = {
                if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::sub(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), A::mul(s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250))))))
                }
            };
            s.store_ad(2244, &assign42250_ad_e55460);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_from_scalar_ad(2261, 1.0, A::scale(A::mul(s.ad_value(2115), A::sub(s.ad_value(2239), A::mul(s.ad_value(2154), s.ad_value(2252)))), 0.5));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add_ad(2245, A::scale(s.ad_value(2238), 2.0), A::mul(s.ad_value(2115), A::sub(A::sub_from_scalar(1.0, s.ad_value(2239)), A::mul(s.ad_value(2154), A::offset(s.ad_value(2251), 1.0)))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add_ad(2246, A::sub(s.ad_value(2139), s.ad_value(2243)), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add(824, 2244, 2245);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2246), A::sub(A::scale(A::square(s.ad_value(2245)), 0.5), A::mul(s.ad_value(2244), s.ad_value(2261)))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            let assign42310_ad_e55583: A = A::add(s.ad_value(2243), A::div(A::mul(A::mul(s.ad_value(2244), s.ad_value(824)), s.ad_value(2246)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246)), s.ad_value(2246)), s.ad_value(2245)), A::sub(A::scale(A::square(s.ad_value(2245)), 0.3333333333333333), A::mul(s.ad_value(2244), s.ad_value(2261)))))));
            s.store_ad(2262, &assign42310_ad_e55583);
        }

        s.v[2277] = if (s.v[2262] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (s.v[2277] != 0.0)) {
            s.store_exp(2248, 2262);
        }

        if (((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (s.v[2277] != 0.0)) {
            s.store_div_from_scalar(2249, 1.0, 2248);
        }

        if (((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (s.v[2277] != 0.0)) {
            s.store_mul(2248, 2154, 2248);
        }

        s.v[2278] = if (s.v[2262] > (s.v[2139] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (!(s.v[2277] != 0.0))) && (s.v[2278] != 0.0)) {
            s.store_exp_ad(2248, A::sub(s.ad_value(2262), s.ad_value(2139)));
        }

        if ((((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (!(s.v[2277] != 0.0))) && (s.v[2278] != 0.0)) {
            s.store_div(2249, 2154, 2248);
        }

        if ((((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (!(s.v[2277] != 0.0))) && (!(s.v[2278] != 0.0))) {
            s.store_div_from_scalar_ad(2248, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) && (!(s.v[2277] != 0.0))) && (!(s.v[2278] != 0.0))) {
            s.store_div_from_scalar_ad(2249, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2262), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2262), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2262), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_div_from_scalar_ad(2238, 1.0, A::offset(A::square(s.ad_value(2262)), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad_lhs(2250, A::square(s.ad_value(2262)), 2238);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_scale_ad(2251, A::mul(A::mul(s.ad_value(2262), s.ad_value(2238)), s.ad_value(2238)), 4.0);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_mul_ad_lhs(2252, A::mul(A::sub(A::scale(s.ad_value(2238), 8.0), A::scale(s.ad_value(2250), 12.0)), s.ad_value(2238)), 2238);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub(2238, 2133, 2262);
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add_ad(2253, A::scale(s.ad_value(2238), 2.0), A::mul(s.ad_value(2115), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2249)), s.ad_value(2248)), A::mul(s.ad_value(2154), A::offset(s.ad_value(2251), 1.0)))));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_ad(2254, A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::sub(A::add(A::offset(A::add(s.ad_value(2249), s.ad_value(2262)), (-1.0)), s.ad_value(2248)), A::mul(s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250))))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::sub(A::add(s.ad_value(2249), s.ad_value(2248)), A::mul(s.ad_value(2154), s.ad_value(2252)))));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_sub_ad(2238, A::square(s.ad_value(2253)), A::scale(A::mul(s.ad_value(2254), s.ad_value(2238)), 2.0));
        }

        if ((!(s.v[2273] != 0.0)) && (!(s.v[2274] != 0.0))) {
            s.store_add_ad_rhs(2153, 2262, A::scale(A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0));
        }

        s.v[2156] = 0.0;

        s.v[2157] = 0.0;

        s.v[2158] = 0.0;

        s.v[2159] = 0.0;

        s.v[2160] = 0.0;

        s.v[2161] = 0.0;

        s.v[2162] = 0.0;

        s.v[2163] = 1.0;

        s.v[2164] = 1.0;

        s.store_sub(2165, 2133, 2153);

        s.v[2166] = 0.0;

        s.store_mul(2167, 2129, 2165);

        s.v[2168] = 1.0;

        s.v[2169] = 1.0;

        s.v[2173] = 1.0;

        s.v[2174] = 1.0;

        s.v[2176] = 1.0;

        s.v[2279] = if (s.v[2133] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2279] != 0.0) {
            s.store_div_from_scalar_ad(2027, 1.0, A::offset(A::square(s.ad_value(2153)), 2.0));
        }

        if (s.v[2279] != 0.0) {
            s.store_mul_ad_lhs(2155, A::square(s.ad_value(2153)), 2027);
        }

        if (s.v[2279] != 0.0) {
            s.store_scale_ad(2156, A::mul(A::mul(s.ad_value(2153), s.ad_value(2027)), s.ad_value(2027)), 4.0);
        }

        if (s.v[2279] != 0.0) {
            s.store_mul_ad_lhs(2157, A::mul(A::sub(A::scale(s.ad_value(2027), 8.0), A::scale(s.ad_value(2155), 12.0)), s.ad_value(2027)), 2027);
        }

        if (s.v[2279] != 0.0) {
            s.store_scalar(2158, 0.0);
        }

        s.v[2280] = if (s.v[2153] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2279] != 0.0) && (s.v[2280] != 0.0)) {
            s.store_exp(2158, 2153);
        }

        if ((s.v[2279] != 0.0) && (s.v[2280] != 0.0)) {
            s.store_div_from_scalar(2159, 1.0, 2158);
        }

        if ((s.v[2279] != 0.0) && (s.v[2280] != 0.0)) {
            s.store_mul(2158, 2154, 2158);
        }

        s.v[2281] = if (s.v[2153] > (s.v[2139] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2279] != 0.0) && (!(s.v[2280] != 0.0))) && (s.v[2281] != 0.0)) {
            s.store_exp_ad(2158, A::sub(s.ad_value(2153), s.ad_value(2139)));
        }

        if (((s.v[2279] != 0.0) && (!(s.v[2280] != 0.0))) && (s.v[2281] != 0.0)) {
            s.store_div(2159, 2154, 2158);
        }

        if (((s.v[2279] != 0.0) && (!(s.v[2280] != 0.0))) && (!(s.v[2281] != 0.0))) {
            s.store_div_from_scalar_ad(2158, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2279] != 0.0) && (!(s.v[2280] != 0.0))) && (!(s.v[2281] != 0.0))) {
            s.store_div_from_scalar_ad(2159, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2153), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2153), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2153), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2279] != 0.0) {
            s.store_sub_ad_rhs(2160, 2158, A::mul(s.ad_value(2154), A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155))));
        }

        s.v[2282] = if (s.v[2153] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2279] != 0.0) && (s.v[2282] != 0.0)) {
            s.store_scale_ad(2161, A::mul(A::square(s.ad_value(2153)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2153), A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2279] != 0.0) && (s.v[2282] != 0.0)) {
            s.store_scale_ad(2160, A::mul(A::mul(A::mul(A::mul(s.ad_value(2154), s.ad_value(2153)), s.ad_value(2153)), s.ad_value(2153)), A::offset(A::scale(s.ad_value(2153), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((s.v[2279] != 0.0) && (s.v[2282] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2153), A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2279] != 0.0) && (s.v[2282] != 0.0)) {
            s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);
        }

        if ((s.v[2279] != 0.0) && (s.v[2282] != 0.0)) {
            s.store_offset_ad(2163, A::scale(A::div(A::mul(s.ad_value(2114), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), A::scale(A::square(s.ad_value(2153)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475), 1.0);
        }

        if ((s.v[2279] != 0.0) && (!(s.v[2282] != 0.0))) {
            s.store_add_ad_lhs(2161, A::offset(s.ad_value(2153), (-1.0)), 2159);
        }

        if ((s.v[2279] != 0.0) && (!(s.v[2282] != 0.0))) {
            s.store_sqrt(2162, 2161);
        }

        if ((s.v[2279] != 0.0) && (!(s.v[2282] != 0.0))) {
            s.store_offset_ad(2163, A::scale(A::div(A::mul(s.ad_value(2114), A::sub_from_scalar(1.0, s.ad_value(2159))), s.ad_value(2162)), 0.5), 1.0);
        }

        if (s.v[2279] != 0.0) {
            s.store_div_ad(2164, A::offset(A::mul(A::scale(s.ad_value(708), 0.2), s.ad_value(2113)), 1.0), A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0));
        }

        s.v[2283] = if (s.v[2160] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul_ad_rhs(2165, 2114, A::sqrt(A::add(s.ad_value(2161), s.ad_value(2160))));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_div_ad(2166, A::mul(A::mul(s.ad_value(2115), s.ad_value(2160)), s.ad_value(2129)), A::add(s.ad_value(2165), A::mul(s.ad_value(2114), s.ad_value(2162))));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul_ad_lhs(2167, A::mul(s.ad_value(2162), s.ad_value(2114)), 2129);
        }

        s.v[2284] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (s.v[2284] != 0.0)) {
            s.store_div_from_scalar_ad(2168, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2113))));
        }

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (!(s.v[2284] != 0.0))) {
            s.store_offset_ad(2168, A::mul(s.ad_value(217), s.ad_value(2113)), 1.0);
        }

        s.v[2285] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (s.v[2285] != 0.0)) {
            s.store_sub_from_scalar_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2166)));
        }

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (!(s.v[2285] != 0.0))) {
            s.store_div_from_scalar_ad(2169, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2166)), 1.0));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul_ad_lhs(2170, A::mul(A::mul(s.ad_value(757), s.ad_value(2168)), s.ad_value(2169)), 2166);
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul_ad_rhs(2171, 774, A::add(s.ad_value(2167), A::mul(s.ad_value(775), s.ad_value(2166))));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_ln_ad(2028, A::div(s.ad_value(2161), A::offset(A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14)));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_add_ad(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul_ad_lhs(2173, A::add(A::offset(s.ad_value(2172), 1.0), s.ad_value(2170)), 2164);
        }

        s.v[2286] = if (s.v[221] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (s.v[2286] != 0.0)) {
            s.store_div_from_scalar_ad(2174, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(2113))));
        }

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (!(s.v[2286] != 0.0))) {
            s.store_offset_ad(2174, A::mul(s.ad_value(221), s.ad_value(2113)), 1.0);
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_mul(2029, 2166, 2174);
        }

        if ((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) {
            s.store_div_ad_rhs(2175, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2287] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (s.v[2287] != 0.0)) {
            s.store_div_from_scalar_ad(2176, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2175))));
        }

        if (((s.v[2279] != 0.0) && (s.v[2283] != 0.0)) && (!(s.v[2287] != 0.0))) {
            s.store_offset_ad(2176, A::mul(s.ad_value(222), s.ad_value(2175)), 1.0);
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

        s.v[2178] = 0.0;

        s.store_scale(2177, 2129, 4.60517018598809);

        s.copy_ad(2194, 2177);

        s.copy_ad(2195, 826);

        s.store_mul(2196, 826, 2130);

        s.copy_ad(2200, 2153);

        s.v[2201] = 0.0;

        s.v[2204] = 0.0;

        s.copy_ad(2206, 2159);

        s.copy_ad(2207, 2161);

        s.copy_ad(2209, 2160);

        s.copy_ad(2210, 2167);

        s.copy_ad(2211, 2153);

        s.copy_ad(2212, 2159);

        s.copy_ad(2214, 2160);

        s.copy_ad(2215, 2161);

        s.store_sub(2216, 2133, 2153);

        s.v[2217] = 1.0;

        s.v[2219] = 1.0;

        s.v[2218] = 0.0;

        s.copy_ad(2228, 2166);

        s.store_mul(2232, 2216, 2129);

        s.v[2229] = 0.0;

        s.copy_ad(2230, 2167);

        s.v[2235] = 0.0;

        s.v[2234] = 1.0;

        s.copy_ad(2237, 2109);

        s.copy_ad(2236, 2232);

        s.v[2288] = if (s.v[2133] > 0.0) { 1.0 } else { 0.0 };

        s.v[2289] = if (s.v[2160] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_mul(2237, 2109, 2176);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_div(2178, 2237, 2173);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_add_ad_rhs(2179, 2165, A::scale(s.ad_value(2115), 0.5));
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_div_ad_lhs(2027, A::div(A::mul(s.ad_value(2115), s.ad_value(2158)), s.ad_value(2179)), 2179);
        }

        s.v[2290] = if (s.v[2027] > 0.0001) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2290] != 0.0)) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.v[2291] = if (s.v[2028] < 1e-10) { 1.0 } else { 0.0 };

        if ((((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2290] != 0.0)) && (s.v[2291] != 0.0)) {
            s.store_scalar(2029, 1.0);
        }

        if ((((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2290] != 0.0)) && (!(s.v[2291] != 0.0))) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (!(s.v[2290] != 0.0))) {
            s.store_scale(2029, 2027, 0.5);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_mul(2180, 2029, 2179);
        }

        s.v[2292] = if ((s.v[706] > 0.0) && (s.v[707] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_lhs(2181, A::scale(s.ad_value(2129), 0.475), 2180);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_sub_ad_rhs(2027, 2166, A::mul(s.ad_value(2163), s.ad_value(2181)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_scale_ad(2182, A::add(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1e-12))), 0.5);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_add_ad(2183, A::sub(A::mul(s.ad_value(2129), s.ad_value(2165)), s.ad_value(2166)), A::mul(A::offset(s.ad_value(2163), (-1.0)), s.ad_value(2181)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_offset_ad(2184, A::div(A::mul(A::scale(s.ad_value(2115), 0.5), s.ad_value(2129)), s.ad_value(2183)), 1.0);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_add_ad_rhs(2027, 2183, A::mul(s.ad_value(775), s.ad_value(2182)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_ad(2185, &A::pow(A::mul(A::mul(s.ad_value(774), s.ad_value(2027)), s.ad_value(704)), s.ad_value(705)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_lhs(2028, A::div(A::mul(s.ad_value(705), A::offset(A::mul(s.ad_value(2184), A::sub_from_scalar(1.0, s.ad_value(775))), (-1.0))), s.ad_value(2027)), 2185);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_div(2027, 2182, 2183);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_rhs(2186, 706, A::pow(A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707))));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_lhs(2029, A::div(A::mul(s.ad_value(707), A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(2027), 1.0)))), s.ad_value(2183)), 2186);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_lhs(2187, A::mul(A::mul(s.ad_value(757), s.ad_value(2168)), s.ad_value(2169)), 2182);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_offset_ad(2027, A::div(A::sub(s.ad_value(2028), A::mul(A::mul(A::mul(s.ad_value(757), s.ad_value(2168)), s.ad_value(2169)), s.ad_value(2184))), s.ad_value(2029)), 1.0);
        }

        s.v[2293] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) && (s.v[2293] != 0.0)) {
            s.store_scale_ad(2028, A::ln(A::offset(A::exp(A::scale(s.ad_value(2027), 2.0)), 1.0)), 0.5);
        }

        if ((((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.copy_ad(2028, 2027);
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_div_ad(2188, A::mul(A::mul(A::neg(s.ad_value(2181)), s.ad_value(2029)), s.ad_value(2028)), A::add(A::add(A::offset(s.ad_value(2185), 1.0), s.ad_value(2186)), s.ad_value(2187)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2292] != 0.0)) {
            s.store_mul_ad_rhs(2189, 2180, A::offset(A::div(s.ad_value(2188), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2188)), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (!(s.v[2292] != 0.0))) {
            s.copy_ad(2189, 2180);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_scale_ad(2190, A::mul(A::mul(s.ad_value(2129), s.ad_value(2178)), s.ad_value(2189)), 0.7071067811865475);
        }

        s.v[2294] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) && (s.v[2294] != 0.0)) {
            s.store_div_ad_rhs(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_div_from_scalar_ad(2191, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2190), 4.0), 1.0)), 1.0));
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_mul(2027, 2191, 2190);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_mul_ad(2192, A::mul(s.ad_value(2189), s.ad_value(2191)), A::offset(A::div(A::mul(A::scale(s.ad_value(2027), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(2027), s.ad_value(2191)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(2027), 4.0), s.ad_value(2027)), s.ad_value(2191)), 1.0)), 1.0));
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_scale(2193, 2192, 0.99);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_div_ad_lhs(2027, A::mul(A::mul(s.ad_value(2193), A::sub(s.ad_value(2193), A::scale(s.ad_value(2179), 2.0))), s.ad_value(2131)), 2160);
        }

        if ((s.v[2288] != 0.0) && (s.v[2289] != 0.0)) {
            s.store_mul_ad_rhs(2194, 2129, A::sub(s.ad_value(2193), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2288] != 0.0) && (!(s.v[2289] != 0.0))) {
            s.copy_ad(2194, 2177);
        }

        if (s.v[2288] != 0.0) {
            s.store_offset(2027, 2110, 1.0);
        }

        if (s.v[2288] != 0.0) {
            s.store_div_ad_lhs(2028, A::mul(A::sqrt(s.ad_value(2027)), s.ad_value(826)), 2194);
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
        }

        if (s.v[2288] != 0.0) {
            s.store_scale(2027, 2028, 2.0);
        }

        if (s.v[2288] != 0.0) {
            s.store_div_ad(2195, A::mul(s.ad_value(2194), s.ad_value(2027)), A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2196, 2195, 2130);
        }

        if (s.v[2288] != 0.0) {
            s.store_add(2197, 2139, 2196);
        }

        s.v[2295] = if (s.v[2196] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2295] != 0.0)) {
            s.store_exp_ad(2198, A::neg(s.ad_value(2196)));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2295] != 0.0))) {
            s.store_div_from_scalar_ad(2198, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2196), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2196), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2196), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2199, 2154, 2198);
        }

        s.v[2296] = if (((s.v[2133]) as f64).abs() <= s.v[2151]) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2296] != 0.0)) {
            s.store_scale_ad(2239, A::square(s.ad_value(2152)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((s.v[2288] != 0.0) && (s.v[2296] != 0.0)) {
            s.store_mul_ad(2200, A::mul(s.ad_value(2133), s.ad_value(2152)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2133), A::sub_from_scalar(1.0, s.ad_value(2199))), s.ad_value(2114)), s.ad_value(2239)), 1.0));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_offset(2260, 2197, 3.0);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub_ad(2243, A::scale(A::sub(A::add(s.ad_value(2259), s.ad_value(2260)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2259), s.ad_value(2260)), A::sub(s.ad_value(2259), s.ad_value(2260))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2260), A::sqrt(A::offset(A::square(s.ad_value(2260)), 5.0))), 0.5));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub(2238, 2133, 2243);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_exp_ad(2239, A::neg(s.ad_value(2243)));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_div_from_scalar_ad(2240, 1.0, A::offset(A::square(s.ad_value(2243)), 2.0));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_mul_ad_lhs(2250, A::square(s.ad_value(2243)), 2240);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_scale_ad(2251, A::mul(A::mul(s.ad_value(2243), s.ad_value(2240)), s.ad_value(2240)), 4.0);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_mul_ad_lhs(2252, A::mul(A::sub(A::scale(s.ad_value(2240), 8.0), A::scale(s.ad_value(2250), 12.0)), s.ad_value(2240)), 2240);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            let assign44430_ad_e57404: A = {
                if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::sub(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), A::mul(s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250))))))
                }
            };
            s.store_ad(2244, &assign44430_ad_e57404);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub_from_scalar_ad(2261, 1.0, A::scale(A::mul(s.ad_value(2115), A::sub(s.ad_value(2239), A::mul(s.ad_value(2199), s.ad_value(2252)))), 0.5));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add_ad(2245, A::scale(s.ad_value(2238), 2.0), A::mul(s.ad_value(2115), A::sub(A::sub_from_scalar(1.0, s.ad_value(2239)), A::mul(s.ad_value(2199), A::offset(s.ad_value(2251), 1.0)))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add_ad(2246, A::sub(s.ad_value(2197), s.ad_value(2243)), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add(824, 2244, 2245);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2246), A::sub(A::scale(A::square(s.ad_value(2245)), 0.5), A::mul(s.ad_value(2244), s.ad_value(2261)))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            let assign44490_ad_e57521: A = A::add(s.ad_value(2243), A::div(A::mul(A::mul(s.ad_value(2244), s.ad_value(824)), s.ad_value(2246)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246)), s.ad_value(2246)), s.ad_value(2245)), A::sub(A::scale(A::square(s.ad_value(2245)), 0.3333333333333333), A::mul(s.ad_value(2244), s.ad_value(2261)))))));
            s.store_ad(2262, &assign44490_ad_e57521);
        }

        s.v[2297] = if (s.v[2262] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (s.v[2297] != 0.0)) {
            s.store_exp(2248, 2262);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (s.v[2297] != 0.0)) {
            s.store_div_from_scalar(2249, 1.0, 2248);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (s.v[2297] != 0.0)) {
            s.store_mul(2248, 2199, 2248);
        }

        s.v[2298] = if (s.v[2262] > (s.v[2197] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (!(s.v[2297] != 0.0))) && (s.v[2298] != 0.0)) {
            s.store_exp_ad(2248, A::sub(s.ad_value(2262), s.ad_value(2197)));
        }

        if ((((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (!(s.v[2297] != 0.0))) && (s.v[2298] != 0.0)) {
            s.store_div(2249, 2199, 2248);
        }

        if ((((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (!(s.v[2297] != 0.0))) && (!(s.v[2298] != 0.0))) {
            s.store_div_from_scalar_ad(2248, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) && (!(s.v[2297] != 0.0))) && (!(s.v[2298] != 0.0))) {
            s.store_div_from_scalar_ad(2249, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2262), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2262), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2262), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_div_from_scalar_ad(2238, 1.0, A::offset(A::square(s.ad_value(2262)), 2.0));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_mul_ad_lhs(2250, A::square(s.ad_value(2262)), 2238);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_scale_ad(2251, A::mul(A::mul(s.ad_value(2262), s.ad_value(2238)), s.ad_value(2238)), 4.0);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_mul_ad_lhs(2252, A::mul(A::sub(A::scale(s.ad_value(2238), 8.0), A::scale(s.ad_value(2250), 12.0)), s.ad_value(2238)), 2238);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub(2238, 2133, 2262);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add_ad(2253, A::scale(s.ad_value(2238), 2.0), A::mul(s.ad_value(2115), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2249)), s.ad_value(2248)), A::mul(s.ad_value(2199), A::offset(s.ad_value(2251), 1.0)))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub_ad(2254, A::square(s.ad_value(2238)), A::mul(s.ad_value(2115), A::sub(A::add(A::offset(A::add(s.ad_value(2249), s.ad_value(2262)), (-1.0)), s.ad_value(2248)), A::mul(s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250))))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::sub(A::add(s.ad_value(2249), s.ad_value(2248)), A::mul(s.ad_value(2199), s.ad_value(2252)))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_sub_ad(2238, A::square(s.ad_value(2253)), A::scale(A::mul(s.ad_value(2254), s.ad_value(2238)), 2.0));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2296] != 0.0))) {
            s.store_add_ad_rhs(2200, 2262, A::scale(A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0));
        }

        if (s.v[2288] != 0.0) {
            s.store_sub(2201, 2200, 2153);
        }

        s.v[2299] = if (s.v[2201] < 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_add_ad(2202, A::scale(A::sub(s.ad_value(2133), s.ad_value(2153)), 2.0), A::mul(s.ad_value(2115), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2159)), A::mul(s.ad_value(2158), s.ad_value(2198))), A::mul(s.ad_value(2199), A::offset(s.ad_value(2156), 1.0)))));
        }

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_mul_ad_lhs(2203, A::mul(s.ad_value(2115), A::sub_from_scalar(1.0, s.ad_value(2198))), 2160);
        }

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2115), A::sub(A::add(s.ad_value(2159), A::mul(s.ad_value(2158), s.ad_value(2198))), A::mul(s.ad_value(2199), s.ad_value(2157)))));
        }

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_sub_ad(2027, A::square(s.ad_value(2202)), A::scale(A::mul(s.ad_value(2027), s.ad_value(2203)), 2.0));
        }

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_scale_ad(2201, A::div(s.ad_value(2203), A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027)))), 2.0);
        }

        if ((s.v[2288] != 0.0) && (s.v[2299] != 0.0)) {
            s.store_add(2200, 2153, 2201);
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2204, 2201, 2129);
        }

        if (s.v[2288] != 0.0) {
            s.store_div_ad(2205, A::square(s.ad_value(2200)), A::offset(A::square(s.ad_value(2200)), 2.0));
        }

        s.v[2300] = if (s.v[2200] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) {
            s.store_exp_ad(2206, A::neg(s.ad_value(2200)));
        }

        s.v[2301] = if (s.v[2200] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (s.v[2301] != 0.0)) {
            s.store_scale_ad(2207, A::mul(A::square(s.ad_value(2200)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2200), A::sub_from_scalar(1.0, A::scale(s.ad_value(2200), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (s.v[2301] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2200), A::sub_from_scalar(1.0, A::scale(s.ad_value(2200), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (s.v[2301] != 0.0)) {
            s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (s.v[2301] != 0.0)) {
            s.store_mul_ad(2209, A::mul(A::mul(A::mul(A::scale(s.ad_value(2199), 0.16666666666666666), s.ad_value(2200)), s.ad_value(2200)), s.ad_value(2200)), A::offset(A::scale(s.ad_value(2200), 1.75), 1.0));
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (!(s.v[2301] != 0.0))) {
            s.store_add_ad_lhs(2207, A::offset(s.ad_value(2200), (-1.0)), 2206);
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (!(s.v[2301] != 0.0))) {
            s.store_sqrt(2208, 2207);
        }

        if (((s.v[2288] != 0.0) && (s.v[2300] != 0.0)) && (!(s.v[2301] != 0.0))) {
            s.store_mul_ad_rhs(2209, 2199, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2206)), s.ad_value(2200)), (-1.0)), s.ad_value(2205)));
        }

        s.v[2302] = if (s.v[2200] > (s.v[2197] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (s.v[2302] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2200), s.ad_value(2197)));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (s.v[2302] != 0.0)) {
            s.store_div(2206, 2199, 2027);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (s.v[2302] != 0.0)) {
            s.store_sub_ad_rhs(2209, 2027, A::mul(s.ad_value(2199), A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205))));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (!(s.v[2302] != 0.0))) {
            s.store_div_from_scalar_ad(2206, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2200), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2200), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2200), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (!(s.v[2302] != 0.0))) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) && (!(s.v[2302] != 0.0))) {
            s.store_sub_ad_rhs(2209, 2027, A::mul(s.ad_value(2199), A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) {
            s.store_add_ad_lhs(2207, A::offset(s.ad_value(2200), (-1.0)), 2206);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2300] != 0.0))) {
            s.store_sqrt(2208, 2207);
        }

        if (s.v[2288] != 0.0) {
            s.store_mul_ad_lhs(2210, A::mul(s.ad_value(2208), s.ad_value(2114)), 2129);
        }

        if (s.v[2288] != 0.0) {
            s.store_scaled_add(2211, 2153, 2200, 0.5);
        }

        if (s.v[2288] != 0.0) {
            s.store_scalar(2212, 0.0);
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2027, 2206, 2159);
        }

        s.v[2303] = if (s.v[2027] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2303] != 0.0)) {
            s.store_sqrt(2212, 2027);
        }

        if (s.v[2288] != 0.0) {
            s.store_scaled_add(2213, 2160, 2209, 0.5);
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad_rhs(2214, 2213, A::scale(A::mul(A::square(s.ad_value(2201)), A::sub(s.ad_value(2212), A::scale(s.ad_value(2131), 2.0))), 0.125));
        }

        s.v[2304] = if (s.v[2211] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) {
            s.store_scale_ad(2215, A::mul(A::square(s.ad_value(2211)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2211), A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) {
            s.store_mul_ad_rhs(2216, 2114, A::sqrt(A::add(s.ad_value(2214), s.ad_value(2215))));
        }

        s.v[2305] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) && (s.v[2305] != 0.0)) {
            s.store_div_from_scalar_ad(2217, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0)));
        }

        if ((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2211), A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) {
            s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);
        }

        if ((s.v[2288] != 0.0) && (s.v[2304] != 0.0)) {
            s.store_add_ad_rhs(2219, 2217, A::scale(A::div(A::mul(s.ad_value(2114), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), A::scale(A::square(s.ad_value(2211)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) {
            s.store_add_ad_lhs(2215, A::offset(s.ad_value(2211), (-1.0)), 2212);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) {
            s.store_mul_ad_rhs(2216, 2114, A::sqrt(A::add(s.ad_value(2214), s.ad_value(2215))));
        }

        s.v[2306] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_add_ad(2220, A::sub_from_scalar(1.0, s.ad_value(2212)), A::scale(A::mul(s.ad_value(2216), s.ad_value(2131)), 2.0));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_div_from_scalar_ad(2217, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0)));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_div_ad_rhs(2027, 2217, A::offset(s.ad_value(2217), 1.0));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_mul_ad_rhs(2221, 730, A::mul(A::mul(A::square(s.ad_value(2027)), s.ad_value(2115)), s.ad_value(2214)));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_add_ad(2222, A::scale(A::sub(s.ad_value(2216), s.ad_value(2221)), 2.0), A::mul(s.ad_value(2115), A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214))));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_mul_ad_rhs(2223, 2221, A::sub(s.ad_value(2221), A::scale(s.ad_value(2216), 2.0)));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_sub_from_scalar_ad(2224, 1.0, A::scale(A::mul(s.ad_value(2115), A::add(s.ad_value(2212), s.ad_value(2214))), 0.5));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_div_ad(2225, A::mul(s.ad_value(2223), s.ad_value(2222)), A::sub(A::square(s.ad_value(2222)), A::mul(s.ad_value(2224), s.ad_value(2223))));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_add(2211, 2211, 2225);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_exp(2226, 2225);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_div(2212, 2212, 2226);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_mul(2214, 2214, 2226);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_add_ad_lhs(2215, A::offset(s.ad_value(2211), (-1.0)), 2212);
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_mul_ad_rhs(2216, 2114, A::sqrt(A::add(s.ad_value(2214), s.ad_value(2215))));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::scale(A::mul(A::mul(s.ad_value(2216), s.ad_value(2217)), s.ad_value(2131)), 2.0));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_div_ad(2201, A::mul(A::mul(s.ad_value(2201), s.ad_value(2226)), A::add(s.ad_value(2220), s.ad_value(2213))), A::add(s.ad_value(2227), A::mul(s.ad_value(2226), s.ad_value(2213))));
        }

        if (((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) && (s.v[2306] != 0.0)) {
            s.store_mul(2204, 2201, 2129);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) {
            s.store_sqrt(2218, 2215);
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2304] != 0.0))) {
            s.store_add_ad_rhs(2219, 2217, A::scale(A::div(A::mul(s.ad_value(2114), A::sub_from_scalar(1.0, s.ad_value(2212))), s.ad_value(2218)), 0.5));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul_ad_rhs(2228, 2129, A::div(A::mul(s.ad_value(2115), s.ad_value(2214)), A::add(s.ad_value(2216), A::mul(s.ad_value(2114), s.ad_value(2218)))));
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad_rhs(2229, 2228, A::mul(s.ad_value(2129), s.ad_value(2219)));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul_ad_lhs(2230, A::mul(s.ad_value(2218), s.ad_value(2114)), 2129);
        }

        s.v[2307] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2288] != 0.0) && (s.v[2307] != 0.0)) {
            s.store_sub_from_scalar_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2228)));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2307] != 0.0))) {
            s.store_div_from_scalar_ad(2169, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2228)), 1.0));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul_ad_lhs(2170, A::mul(A::mul(s.ad_value(757), s.ad_value(2168)), s.ad_value(2169)), 2228);
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad_rhs(2231, 2230, A::mul(s.ad_value(775), s.ad_value(2228)));
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad_rhs(2232, 2230, A::mul(s.ad_value(776), s.ad_value(2228)));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2233, 774, 2231);
        }

        if (s.v[2288] != 0.0) {
            s.store_ln_ad(2028, A::div(s.ad_value(2215), A::offset(A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14)));
        }

        if (s.v[2288] != 0.0) {
            s.store_add_ad(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul_ad_lhs(2234, A::add(A::offset(s.ad_value(2172), 1.0), s.ad_value(2170)), 2164);
        }

        if (s.v[2288] != 0.0) {
            s.store_ln_ad(2235, A::div(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0)));
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2029, 2228, 2174);
        }

        if (s.v[2288] != 0.0) {
            s.store_div_ad_rhs(2175, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2308] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2288] != 0.0) && (s.v[2308] != 0.0)) {
            s.store_div_from_scalar_ad(2176, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2175))));
        }

        if ((s.v[2288] != 0.0) && (!(s.v[2308] != 0.0))) {
            s.store_offset_ad(2176, A::mul(s.ad_value(222), s.ad_value(2175)), 1.0);
        }

        if (s.v[2288] != 0.0) {
            s.store_mul(2237, 2109, 2176);
        }

        if (s.v[2288] != 0.0) {
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

        s.v[1873] = 1.0;

        s.v[1874] = 1.0;

        s.v[1876] = 1.0;

        s.v[1877] = 1.0;

        s.v[838] = 0.0;

        s.v[2309] = if (s.v[1829] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2309] != 0.0) {
            s.store_ln_ad(2037, A::offset(A::mul(s.ad_value(830), s.ad_value(779)), 1.0));
        }

        if (s.v[2309] != 0.0) {
            s.store_div_ad_lhs(2027, A::mul(s.ad_value(1824), s.ad_value(1864)), 1866);
        }

        if (s.v[2309] != 0.0) {
            s.store_add_ad(2036, A::mul(A::div(A::mul(A::add(s.ad_value(225), A::div(s.ad_value(226), s.ad_value(1866))), s.ad_value(1865)), s.ad_value(1866)), s.ad_value(1871)), A::mul(A::mul(A::mul(A::mul(s.ad_value(227), s.ad_value(1867)), s.ad_value(2027)), s.ad_value(2027)), s.ad_value(2037)));
        }

        if (s.v[2309] != 0.0) {
            s.store_div_from_scalar_ad(1873, 1.0, A::add(A::offset(s.ad_value(2036), 1.0), A::square(s.ad_value(2036))));
        }

        if (s.v[2309] != 0.0) {
            s.store_mul(1874, 1869, 1873);
        }

        if (s.v[2309] != 0.0) {
            s.store_div(1875, 1870, 1874);
        }

        if (s.v[2309] != 0.0) {
            s.store_mul_ad_lhs(2038, A::mul(A::square(s.ad_value(1875)), s.ad_value(1860)), 1860);
        }

        s.v[2310] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2309] != 0.0) && (s.v[2310] != 0.0)) {
            s.store_div_ad_rhs(2038, 2038, A::offset(A::mul(s.ad_value(1875), s.ad_value(1860)), 1.0));
        }

        if (s.v[2309] != 0.0) {
            s.store_scale_ad(2039, A::mul(s.ad_value(1874), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2038), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2309] != 0.0) {
            s.store_div_from_scalar(1876, 1.0, 2039);
        }

        if (s.v[2309] != 0.0) {
            s.store_mul(2027, 1874, 1876);
        }

        if (s.v[2309] != 0.0) {
            s.store_mul_ad_rhs(2040, 1864, A::offset(A::scale(A::mul(A::mul(s.ad_value(2038), s.ad_value(2027)), s.ad_value(2027)), 0.5), 1.0));
        }

        if (s.v[2309] != 0.0) {
            s.store_div_ad_lhs(1877, A::mul(s.ad_value(2027), s.ad_value(1866)), 2040);
        }

        if (s.v[2309] != 0.0) {
            s.store_mul_ad_lhs(838, A::mul(A::mul(s.ad_value(716), s.ad_value(1866)), s.ad_value(1860)), 1876);
        }

        s.v[2042] = 0.0;

        s.v[2043] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.v[2311] = if (((((p.p40 != 0.0) && ((s.v[237] > 0.0) || (s.v[238] > 0.0))) || ((p.p42 != 0.0) && ((s.v[247] > 0.0) || (s.v[248] > 0.0)))) || (s.v[262] > 0.0)) || (s.v[263] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2311] != 0.0) {
            s.store_scale_ad(2041, A::add(s.ad_value(1817), A::sqrt(A::add(A::square(s.ad_value(1817)), s.ad_value(789)))), 0.5);
        }

        if (s.v[2311] != 0.0) {
            s.store_add_ad_lhs(2042, A::add(A::sub(A::neg(s.ad_value(2041)), A::scale(s.ad_value(784), 0.5)), A::mul(s.ad_value(782), A::sqrt(A::add(A::add(s.ad_value(2041), A::scale(s.ad_value(784), 0.25)), s.ad_value(790))))), 791);
        }

        if (s.v[2311] != 0.0) {
            s.store_scale_ad(2041, A::add(s.ad_value(1818), A::sqrt(A::add(A::square(s.ad_value(1818)), s.ad_value(792)))), 0.5);
        }

        if (s.v[2311] != 0.0) {
            s.store_add_ad_lhs(2043, A::add(A::sub(A::neg(s.ad_value(2041)), A::scale(s.ad_value(785), 0.5)), A::mul(s.ad_value(783), A::sqrt(A::add(A::add(s.ad_value(2041), A::scale(s.ad_value(785), 0.25)), s.ad_value(793))))), 794);
        }

        if (s.v[2311] != 0.0) {
            s.store_scaled_add(1878, 1817, 2042, (-s.v[354]));
        }

        if (s.v[2311] != 0.0) {
            s.store_scaled_add(1879, 1818, 2043, (-s.v[354]));
        }

        s.v[2312] = if (p.p40 != 0.0) { 1.0 } else { 0.0 };

        s.v[2313] = if (s.v[237] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_mul_ad_lhs(2044, A::sqrt(A::offset(A::square(s.ad_value(1878)), 1e-6)), 795);
        }

        s.v[2314] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) && (s.v[2314] != 0.0)) {
            s.store_scale_ad(2044, A::sub(A::add(s.ad_value(2044), s.ad_value(801)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(801)), A::sub(s.ad_value(2044), s.ad_value(801))), 1e-6))), 0.5);
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_mul_ad_rhs(2027, 798, A::offset(A::mul(s.ad_value(2044), A::add(s.ad_value(242), A::mul(s.ad_value(243), s.ad_value(2044)))), (-1.5)));
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_offset(2046, 2042, 3.0);
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_sub_from_scalar(2047, (-3.0), 235);
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_scale(2048, 834, 30.0);
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_scalar(818, (4.0 - 0.9));
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_add(819, 2046, 2048);
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul(A::mul(s.ad_value(818), s.ad_value(2046)), s.ad_value(2048))))));
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_scalar(818, (4.0 - 0.3));
        }

        if ((s.v[2312] != 0.0) && (s.v[2313] != 0.0)) {
            s.store_add(819, 2047, 2027);
        }

        s.v[2317] = if (s.v[238] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_mul_ad_lhs(2044, A::sqrt(A::offset(A::square(s.ad_value(1879)), 1e-6)), 795);
        }

        s.v[2318] = if (s.v[245] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) && (s.v[2318] != 0.0)) {
            s.store_scale_ad(2044, A::sub(A::add(s.ad_value(2044), s.ad_value(802)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(802)), A::sub(s.ad_value(2044), s.ad_value(802))), 1e-6))), 0.5);
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_mul_ad_rhs(2027, 799, A::offset(A::mul(s.ad_value(2044), A::add(s.ad_value(244), A::mul(s.ad_value(245), s.ad_value(2044)))), (-1.5)));
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_offset(2046, 2043, 3.0);
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_sub_from_scalar(2047, (-3.0), 235);
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_scale(2048, 837, 30.0);
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_scalar(818, (4.0 - 0.9));
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_add(819, 2046, 2048);
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul(A::mul(s.ad_value(818), s.ad_value(2046)), s.ad_value(2048))))));
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_scalar(818, (4.0 - 0.3));
        }

        if ((s.v[2312] != 0.0) && (s.v[2317] != 0.0)) {
            s.store_add(819, 2047, 2027);
        }

        s.v[2321] = if (s.v[236] > 0.0) { 1.0 } else { 0.0 };

        s.v[2322] = if (s.v[1829] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2322] != 0.0)) {
            s.store_offset(2027, 777, 1.0);
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2322] != 0.0)) {
            s.store_div_ad_lhs(2028, A::mul(A::sqrt(s.ad_value(2027)), s.ad_value(826)), 1855);
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2322] != 0.0)) {
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2322] != 0.0)) {
            s.store_scale(2027, 2028, 2.0);
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2322] != 0.0)) {
            s.store_div_ad(1858, A::mul(A::mul(s.ad_value(1855), s.ad_value(1825)), s.ad_value(2027)), A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))));
        }

        s.v[2323] = if ((s.v[1859] - s.v[1858]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2323] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(1859), s.ad_value(1858)));
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2323] != 0.0))) {
            let assign46740_ad_e59878: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2027, &assign46740_ad_e59878);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_add_ad_rhs(2050, 2030, A::mul(s.ad_value(1824), A::sub(A::scale(s.ad_value(1859), 0.5), A::ln(A::scale(A::offset(s.ad_value(2027), 1.0), 0.5)))));
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_mul(2051, 235, 1824);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_add(2052, 1872, 2051);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_scale_ad(2053, A::sub(s.ad_value(2052), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(2052)), A::neg(s.ad_value(2052))), 0.01))), 0.5);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_mul_ad_lhs(2044, A::sqrt(A::offset(A::square(s.ad_value(1872)), 1e-6)), 795);
        }

        s.v[2324] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2324] != 0.0)) {
            s.store_scale_ad(2044, A::sub(A::add(s.ad_value(2044), s.ad_value(800)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(800)), A::sub(s.ad_value(2044), s.ad_value(800))), 1e-6))), 0.5);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_add_ad_rhs(2054, 1862, A::mul(A::sub(A::sub(s.ad_value(2053), s.ad_value(742)), s.ad_value(2050)), s.ad_value(1825)));
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_mul_ad_lhs(2054, A::neg(A::sub(A::add(s.ad_value(825), s.ad_value(2030)), s.ad_value(2050))), 1825);
        }

        s.v[2327] = if (((s.v[2054]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (s.v[2327] != 0.0)) {
            s.store_exp(2027, 2054);
        }

        s.v[2328] = if (s.v[2054] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2327] != 0.0))) && (s.v[2328] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2054)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2054)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2054)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2327] != 0.0))) && (!(s.v[2328] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(s.ad_value(2054), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2054), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2054), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) {
            s.store_mul_ad_rhs(2027, 797, A::offset(A::mul(s.ad_value(2044), A::add(s.ad_value(240), A::mul(s.ad_value(241), s.ad_value(2044)))), (-1.5)));
        }

        s.v[2331] = if ((s.v[1829] <= 0.0) || ((s.v[240] == 0.0) && (s.v[241] == 0.0))) { 1.0 } else { 0.0 };

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) {
            s.store_add_ad_rhs(2027, 240, A::mul(A::scale(s.ad_value(241), 2.0), s.ad_value(2044)));
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) {
            s.store_div_ad_rhs(2058, 246, A::mul(s.ad_value(2027), s.ad_value(797)));
        }

        if (((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) {
            s.store_scaled_div(2059, 1860, 2058, 0.5);
        }

        s.v[2332] = if (s.v[2059] < 0.001) { 1.0 } else { 0.0 };

        s.v[2333] = if (((s.v[2059]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) && (s.v[2333] != 0.0)) {
            s.store_exp(2067, 2059);
        }

        s.v[2334] = if (s.v[2059] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) && (!(s.v[2333] != 0.0))) && (s.v[2334] != 0.0)) {
            s.store_div_from_scalar_ad(2067, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2059)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2059)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2059)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) && (!(s.v[2333] != 0.0))) && (!(s.v[2334] != 0.0))) {
            s.store_scale_ad(2067, A::offset(A::mul(A::offset(s.ad_value(2059), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2059), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2059), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) {
            s.store_div_from_scalar(2068, 1.0, 2067);
        }

        if ((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) {
            s.store_sub(2027, 2067, 2068);
        }

        if ((((s.v[2312] != 0.0) && (s.v[2321] != 0.0)) && (!(s.v[2331] != 0.0))) && (!(s.v[2332] != 0.0))) {
            s.store_add(2029, 2067, 2068);
        }

        s.v[2335] = if (p.p42 != 0.0) { 1.0 } else { 0.0 };

        s.v[2336] = if ((s.v[248] > 0.0) && (s.v[1879] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2335] != 0.0) && (s.v[2336] != 0.0)) {
            s.store_sqrt_ad(2071, A::offset(A::add(A::square(s.ad_value(1879)), A::mul(A::square(s.ad_value(254)), A::square(s.ad_value(836)))), 1e-6));
        }

        if ((s.v[2335] != 0.0) && (s.v[2336] != 0.0)) {
            s.store_div_ad_lhs(2027, A::neg(s.ad_value(807)), 2071);
        }

        s.v[2337] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2335] != 0.0) && (s.v[2336] != 0.0)) && (s.v[2337] != 0.0)) {
            s.store_exp(2029, 2027);
        }

        if (((s.v[2335] != 0.0) && (s.v[2336] != 0.0)) && (!(s.v[2337] != 0.0))) {
            s.store_div_from_scalar_ad(2029, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2338] = if ((s.v[247] > 0.0) && (s.v[1878] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2335] != 0.0) && (s.v[2338] != 0.0)) {
            s.store_sqrt_ad(2072, A::offset(A::add(A::square(s.ad_value(1878)), A::mul(A::square(s.ad_value(253)), A::square(s.ad_value(835)))), 1e-6));
        }

        if ((s.v[2335] != 0.0) && (s.v[2338] != 0.0)) {
            s.store_div_ad_lhs(2027, A::neg(s.ad_value(806)), 2072);
        }

        s.v[2339] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2335] != 0.0) && (s.v[2338] != 0.0)) && (s.v[2339] != 0.0)) {
            s.store_exp(2029, 2027);
        }

        if (((s.v[2335] != 0.0) && (s.v[2338] != 0.0)) && (!(s.v[2339] != 0.0))) {
            s.store_div_from_scalar_ad(2029, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2076] = s.v[715];

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 1e-40;

        s.v[1884] = 1.0;

        s.v[846] = 0.0;

        s.v[2340] = if ((p.p46 != 0.0) && (s.v[287] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2340] != 0.0) {
            s.store_add_ad_lhs(2027, A::scale(A::sub(A::add(s.ad_value(828), s.ad_value(827)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(764)))), 0.5), 762);
        }

        if (s.v[2340] != 0.0) {
            s.store_add_ad_lhs(2073, A::sub(s.ad_value(827), A::scale(A::sub(s.ad_value(2027), A::sqrt(A::add(A::mul(s.ad_value(2027), s.ad_value(2027)), s.ad_value(763)))), 0.5)), 766);
        }

        if (s.v[2340] != 0.0) {
            s.store_add_ad_rhs(2074, 2073, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul_ad(2075, A::mul(s.ad_value(289), A::offset(A::mul(s.ad_value(291), s.ad_value(830)), 1.0)), A::offset(A::mul(s.ad_value(290), s.ad_value(2074)), 1.0));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul_ad_rhs(2076, 723, A::offset(s.ad_value(2075), 1.0));
        }

        if (s.v[2340] != 0.0) {
            s.store_div_from_scalar(2077, 1.0, 2076);
        }

        if (s.v[2340] != 0.0) {
            s.store_div_ad(2078, A::scale(s.ad_value(830), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(293), s.ad_value(830)), 1.0)), 1.0));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul_ad(2079, A::mul(s.ad_value(292), s.ad_value(2078)), A::offset(A::mul(s.ad_value(294), s.ad_value(2074)), 1.0));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul_ad_rhs(1880, 2077, A::sub(A::add(s.ad_value(829), s.ad_value(2079)), s.ad_value(713)));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul(2080, 2077, 760);
        }

        if (s.v[2340] != 0.0) {
            s.store_scale_ad(2081, A::ln(A::add(A::div(s.ad_value(2080), s.ad_value(761)), A::sqrt(s.ad_value(2080)))), 2.0);
        }

        if (s.v[2340] != 0.0) {
            s.store_mul(2082, 2077, 2073);
        }

        if (s.v[2340] != 0.0) {
            s.store_add(2087, 2080, 2082);
        }

        if (s.v[2340] != 0.0) {
            s.store_add_ad_rhs(2088, 2087, A::mul(s.ad_value(761), A::sqrt(s.ad_value(2087))));
        }

        if (s.v[2340] != 0.0) {
            s.store_add(2089, 2088, 2081);
        }

        if (s.v[2340] != 0.0) {
            s.store_offset_ad(2090, A::div(s.ad_value(761), A::scale(A::sqrt(s.ad_value(2087)), 2.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2340] != 0.0) {
            s.store_div_from_scalar(2091, 1.0, 2090);
        }

        if (s.v[2340] != 0.0) {
            s.store_sub(2092, 1880, 2089);
        }

        s.v[2341] = if (s.v[2092] > (-12.0)) { 1.0 } else { 0.0 };

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_offset_ad(2093, A::add(s.ad_value(2092), s.ad_value(725)), (-1.0));
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_scale_ad(2094, A::add(s.ad_value(2093), A::sqrt(A::offset(A::square(s.ad_value(2093)), 10.0))), 0.5);
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_add_ad_lhs(2095, A::sub(s.ad_value(2092), A::mul(s.ad_value(2090), A::ln(s.ad_value(2094)))), 725);
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_scale_ad(2096, A::add(s.ad_value(2095), A::sqrt(A::offset(A::square(s.ad_value(2095)), 2.0))), 0.5);
        }

        s.v[2342] = if ((s.v[2092] - s.v[2096]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) && (s.v[2342] != 0.0)) {
            s.store_exp_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)));
        }

        if (((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) && (!(s.v[2342] != 0.0))) {
            s.store_scale_ad(2097, A::offset(A::mul(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_mul(2098, 724, 2097);
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_ad(2099, &A::pow(s.ad_value(2098), s.ad_value(2091)));
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_add_ad(2100, A::square(s.ad_value(2090)), A::mul(A::sub(A::scale(A::add(s.ad_value(2096), s.ad_value(2090)), 2.0), s.ad_value(2099)), s.ad_value(2099)));
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_mul_ad_rhs(2101, 2090, A::offset(A::div(A::sub(A::sqrt(s.ad_value(2100)), s.ad_value(2090)), s.ad_value(2099)), (-1.0)));
        }

        if ((s.v[2340] != 0.0) && (s.v[2341] != 0.0)) {
            s.store_sub(2083, 2096, 2101);
        }

        s.v[2343] = if ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2340] != 0.0) && (!(s.v[2341] != 0.0))) && (s.v[2343] != 0.0)) {
            s.store_exp_ad(2083, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2341] != 0.0))) && (!(s.v[2343] != 0.0))) {
            let assign47890_ad_e61386: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(2083, 1e-100, A::offset(assign47890_ad_e61386, 1.0));
        }

        if (s.v[2340] != 0.0) {
            s.store_mul_ad_rhs(2084, 2077, A::add(s.ad_value(1857), s.ad_value(2073)));
        }

        s.v[2344] = if ((s.v[2083] < 0.001) && (s.v[1857] < 1e-6)) { 1.0 } else { 0.0 };

        s.v[2345] = if (((-s.v[2084]) + s.v[2082]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2340] != 0.0) && (s.v[2344] != 0.0)) && (s.v[2345] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2082), s.ad_value(2084)));
        }

        if (((s.v[2340] != 0.0) && (s.v[2344] != 0.0)) && (!(s.v[2345] != 0.0))) {
            let assign47940_ad_e61465: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2027, &assign47940_ad_e61465);
        }

        if ((s.v[2340] != 0.0) && (s.v[2344] != 0.0)) {
            s.store_mul_ad_rhs(1881, 2083, A::offset(s.ad_value(2027), (-1.0)));
        }

        if ((s.v[2340] != 0.0) && (s.v[2344] != 0.0)) {
            s.store_add(2085, 1881, 2083);
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_add(2087, 2080, 2084);
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_add_ad_rhs(2088, 2087, A::mul(s.ad_value(761), A::sqrt(s.ad_value(2087))));
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_add(2089, 2088, 2081);
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_offset_ad(2090, A::div(s.ad_value(761), A::scale(A::sqrt(s.ad_value(2087)), 2.0)), 1.0);
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_div_from_scalar(2091, 1.0, 2090);
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_sub(2092, 1880, 2089);
        }

        s.v[2346] = if (s.v[2092] > (-12.0)) { 1.0 } else { 0.0 };

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_offset_ad(2093, A::add(s.ad_value(2092), s.ad_value(725)), (-1.0));
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_scale_ad(2094, A::add(s.ad_value(2093), A::sqrt(A::offset(A::square(s.ad_value(2093)), 10.0))), 0.5);
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_add_ad_lhs(2095, A::sub(s.ad_value(2092), A::mul(s.ad_value(2090), A::ln(s.ad_value(2094)))), 725);
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_scale_ad(2096, A::add(s.ad_value(2095), A::sqrt(A::offset(A::square(s.ad_value(2095)), 2.0))), 0.5);
        }

        s.v[2347] = if ((s.v[2092] - s.v[2096]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) && (s.v[2347] != 0.0)) {
            s.store_exp_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)));
        }

        if ((((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) && (!(s.v[2347] != 0.0))) {
            s.store_scale_ad(2097, A::offset(A::mul(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_mul(2098, 724, 2097);
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_ad(2099, &A::pow(s.ad_value(2098), s.ad_value(2091)));
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_add_ad(2100, A::square(s.ad_value(2090)), A::mul(A::sub(A::scale(A::add(s.ad_value(2096), s.ad_value(2090)), 2.0), s.ad_value(2099)), s.ad_value(2099)));
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_mul_ad_rhs(2101, 2090, A::offset(A::div(A::sub(A::sqrt(s.ad_value(2100)), s.ad_value(2090)), s.ad_value(2099)), (-1.0)));
        }

        if (((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (s.v[2346] != 0.0)) {
            s.store_sub(2085, 2096, 2101);
        }

        s.v[2348] = if ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (!(s.v[2346] != 0.0))) && (s.v[2348] != 0.0)) {
            s.store_exp_ad(2085, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if ((((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) && (!(s.v[2346] != 0.0))) && (!(s.v[2348] != 0.0))) {
            let assign48180_ad_e61818: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(2085, 1e-100, A::offset(assign48180_ad_e61818, 1.0));
        }

        if ((s.v[2340] != 0.0) && (!(s.v[2344] != 0.0))) {
            s.store_sub(1881, 2085, 2083);
        }

        if (s.v[2340] != 0.0) {
            s.store_scaled_add(1882, 2085, 2083, 0.5);
        }

        if (s.v[2340] != 0.0) {
            s.store_ad(1883, &{
                if ((s.v[1880] - s.v[1882]) > 1e-40) {
                    A::sub(s.ad_value(1880), s.ad_value(1882))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.v[2340] != 0.0) {
            s.store_sub_from_scalar_ad(1884, 1.0, A::div(A::scale(s.ad_value(761), 0.5), A::sqrt(A::add(s.ad_value(1883), A::scale(s.ad_value(724), 0.25)))));
        }

        if (s.v[2340] != 0.0) {
            s.store_div_ad_lhs(846, A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(717)), s.ad_value(2076)), s.ad_value(2076)), A::offset(A::mul(s.ad_value(1884), s.ad_value(1882)), 1.0)), s.ad_value(1881)), 1869);
        }

        s.v[1885] = 0.0;

        s.v[847] = 0.0;

        s.v[2349] = if ((s.v[1829] > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2349] != 0.0) {
            s.store_sub_ad_rhs(2086, 826, A::mul(s.ad_value(232), s.ad_value(1860)));
        }

        s.v[2350] = if (s.v[2086] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) {
            s.store_mul_ad_rhs(2029, 712, A::div(A::offset(A::mul(s.ad_value(233), A::sub(A::sqrt(A::add(s.ad_value(728), s.ad_value(2030))), s.ad_value(736))), 1.0), A::offset(s.ad_value(2086), 1e-30)));
        }

        s.v[2351] = if ((((-s.v[2029])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (s.v[2351] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2029)));
        }

        s.v[2352] = if ((-s.v[2029]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (!(s.v[2351] != 0.0))) && (s.v[2352] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2029))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2029))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2029))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (!(s.v[2351] != 0.0))) && (!(s.v[2352] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2029)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2029)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2029)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) {
            s.store_mul_ad_rhs(1885, 229, A::mul(s.ad_value(2086), s.ad_value(2027)));
        }

        if ((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) {
            s.store_mul_ad_rhs(847, 1885, A::add(s.ad_value(838), s.ad_value(846)));
        }

        s.v[2353] = if (s.v[847] > (0.5 * s.v[234])) { 1.0 } else { 0.0 };

        if (((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (s.v[2353] != 0.0)) {
            s.store_offset_ad(2027, A::div(A::scale(s.ad_value(847), 2.0), s.ad_value(234)), (-1.0));
        }

        if (((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (s.v[2353] != 0.0)) {
            s.store_mul_ad(847, A::scale(s.ad_value(234), 0.5), A::offset(A::div(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1.0))), 1.0));
        }

        s.v[2547] = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        s.v[2548] = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2388, 728);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2389, 738);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2390, 729);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2391, 1820);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2392, 1821);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2396, 0.0);
        }

        s.v[2549] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.store_add_ad_lhs(2391, A::scale(A::sub(A::add(s.ad_value(828), s.ad_value(827)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(749)))), 0.5), 747);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.store_add_ad_lhs(1886, A::sub(s.ad_value(827), A::scale(A::sub(s.ad_value(2391), A::sqrt(A::add(A::mul(s.ad_value(2391), s.ad_value(2391)), s.ad_value(748)))), 0.5)), 750);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2392, 1886);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2388, 745);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2389, 748);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2390, 746);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub_ad_lhs(2395, A::sub(s.ad_value(829), s.ad_value(2396)), 700);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_add_ad_rhs(2397, 2392, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2409, 1.0);
        }

        s.v[2550] = if (s.v[190] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2400, 2388, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2401, 2397, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2402, 2395, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_offset_ad(2028, A::div(A::scale(s.ad_value(2390), 0.5), A::sqrt(s.ad_value(2400))), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add_ad_rhs(2029, 2400, A::mul(s.ad_value(2390), A::sqrt(s.ad_value(2400))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad(2403, A::add(A::div(A::sub(s.ad_value(2402), s.ad_value(2029)), s.ad_value(2028)), A::scale(s.ad_value(2400), 0.5)), A::mul(A::offset(s.ad_value(191), 1.0), s.ad_value(2401)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_offset_scaled(2404, 2400, 0.5, 2.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add(2405, 2400, 2401);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad(2028, A::sub(A::sub(s.ad_value(2402), s.ad_value(2405)), A::mul(s.ad_value(2390), A::sqrt(s.ad_value(2405)))), A::scale(A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add_ad_lhs(2406, A::scale(s.ad_value(2028), 2.0), 2404);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2028, A::add(A::add(s.ad_value(2403), s.ad_value(2406)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2403), s.ad_value(2406)), A::sub(s.ad_value(2403), s.ad_value(2406))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad_lhs(2029, A::scale(A::sub(s.ad_value(2402), s.ad_value(2401)), 2.0), 2404);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2407, A::sub(A::add(s.ad_value(2028), s.ad_value(2029)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2028, A::sub(A::add(s.ad_value(2407), s.ad_value(2404)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2407), s.ad_value(2404)), A::sub(s.ad_value(2407), s.ad_value(2404))), 5.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2408, A::add(A::sub(s.ad_value(2028), s.ad_value(2404)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), A::neg(s.ad_value(2404))), A::sub(s.ad_value(2028), A::neg(s.ad_value(2404)))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_mul_ad_rhs(2029, 702, A::offset(A::div(s.ad_value(2408), s.ad_value(2404)), 1.0));
        }

        s.v[2551] = if (s.v[2029] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) && (s.v[2551] != 0.0)) {
            s.store_exp(2409, 2029);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) && (!(s.v[2551] != 0.0))) {
            s.store_div_from_scalar_ad(2409, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_offset_ad(2410, A::mul(s.ad_value(701), s.ad_value(2409)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scale(2411, 2410, s.v[715]);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2412, A::mul(s.ad_value(199), A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0)), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad_rhs(2413, 2411, A::offset(s.ad_value(2412), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2414, 1.0, 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad_rhs(2398, 2390, A::sqrt(A::scale(s.ad_value(2414), s.v[715])));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_square(2399, 2398);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2415, 1.0, 2399);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2416, 2392, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2417, 2395, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_ad(2418, A::scale(s.ad_value(830), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2419, A::mul(s.ad_value(196), s.ad_value(2418)), A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2420, 2388, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sqrt_ad(2028, A::add(A::square(s.ad_value(2391)), s.ad_value(2389)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sqrt_ad(2029, A::add(A::mul(A::sub(s.ad_value(2391), s.ad_value(2419)), A::sub(s.ad_value(2391), s.ad_value(2419))), s.ad_value(2389)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2421, A::scale(s.ad_value(2414), 0.5), A::sub(A::add(s.ad_value(2419), s.ad_value(2028)), s.ad_value(2029)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_add(2422, 2420, 2416);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub(2423, 2422, 2421);
        }

        s.v[2552] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2553] = if (((s.v[2423]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (s.v[2553] != 0.0)) {
            s.store_offset_ad(2424, A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2423), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2423), 0.3125))))), 1.0);
        }

        s.v[2554] = if (s.v[2423] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) && (s.v[2554] != 0.0)) {
            s.store_exp_ad(2438, A::neg(s.ad_value(2423)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) && (!(s.v[2554] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2423), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) {
            s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) {
            s.store_offset_ad(2424, A::div(A::mul(A::mul(s.ad_value(2027), s.ad_value(2398)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2438), A::sub_from_scalar(1.0, s.ad_value(2423))))), A::scale(A::sqrt(A::mul(s.ad_value(2423), A::sub_from_scalar(1.0, s.ad_value(2438)))), 2.0)), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2552] != 0.0))) {
            s.store_offset_ad(2424, A::div(A::scale(s.ad_value(2398), 0.5), A::sqrt(s.ad_value(2423))), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub_ad(2425, A::add(s.ad_value(2423), A::mul(s.ad_value(2398), A::sqrt(s.ad_value(2423)))), A::mul(s.ad_value(2424), A::ln(A::offset(s.ad_value(2424), (-1.0)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_ad_lhs(2426, A::sub(s.ad_value(2417), s.ad_value(2425)), 2424);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2432, A::scale(s.ad_value(2399), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2431, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2433, 1.0);
        }

        s.v[2555] = if (s.v[2426] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_offset_ad(2427, A::mul(s.ad_value(2424), s.ad_value(2426)), (-1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2027, A::add(s.ad_value(2427), A::sqrt(A::offset(A::square(s.ad_value(2427)), 10.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_rhs(2428, 2426, A::ln(s.ad_value(2027)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2429, A::add(s.ad_value(2428), A::sqrt(A::offset(A::square(s.ad_value(2428)), 2.0))), 0.5);
        }

        s.v[2556] = if ((s.v[2426] - s.v[2429]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (s.v[2556] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (!(s.v[2556] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_div(2430, 2027, 2424);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_lhs(2027, A::scale(A::offset(s.ad_value(2429), 1.0), 2.0), 2430);
        }

        s.v[2557] = if (s.v[2430] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (s.v[2557] != 0.0)) {
            s.store_mul_ad_rhs(2431, 2424, A::offset(A::sub(s.ad_value(2429), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2430), s.ad_value(2027)), 1.0)), (-1.0)), s.ad_value(2430))), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (!(s.v[2557] != 0.0))) {
            s.store_mul_ad(2431, A::mul(A::scale(s.ad_value(2424), 0.5), s.ad_value(2430)), A::offset(A::mul(A::scale(s.ad_value(2027), 0.25), s.ad_value(2027)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2027, A::add(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0)), A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_mul_ad(2432, A::scale(s.ad_value(2399), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027)), 1.0)), (-1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_div_ad_rhs(2433, 2432, A::add(s.ad_value(2432), s.ad_value(2431)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_rhs(2423, 2422, A::mul(s.ad_value(2433), s.ad_value(2421)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scale(2435, 2434, 1e-5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2436, 1.0, 2434);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2543, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2437, 0.0);
        }

        s.v[2558] = if (s.v[2423] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2558] != 0.0)) {
            s.store_exp_ad(2438, A::neg(s.ad_value(2423)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2558] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2423), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2559] = if (((s.v[2417]) as f64).abs() <= s.v[2435]) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2559] != 0.0)) {
            s.store_scale_ad(2523, A::square(s.ad_value(2436)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2559] != 0.0)) {
            s.store_mul_ad(2437, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2417), A::sub_from_scalar(1.0, s.ad_value(2438))), s.ad_value(2398)), s.ad_value(2523)), 1.0));
        }

        s.v[2560] = if (s.v[2417] < (-s.v[2435])) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_neg(2525, 2417);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scaled_mul(2526, 2525, 2436, 1.25);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scale_ad(2527, A::sub(A::offset(s.ad_value(2526), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2526), (-6.0)), A::offset(s.ad_value(2526), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub(2522, 2525, 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(2528, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::offset(s.ad_value(2527), 1.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad_lhs(2529, A::scale(s.ad_value(2522), 2.0), 2399);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad_lhs(2530, A::ln(A::mul(s.ad_value(2528), s.ad_value(2415))), 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add(824, 2528, 2529);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), s.ad_value(2528))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad_rhs(2531, 2527, A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), s.ad_value(2528))))));
        }

        s.v[2561] = if (s.v[2531] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) && (s.v[2561] != 0.0)) {
            s.store_exp(2532, 2531);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) && (!(s.v[2561] != 0.0))) {
            s.store_scale_ad(2532, A::offset(A::mul(A::offset(s.ad_value(2531), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2531), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2531), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2531)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2531)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2531), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub(2522, 2525, 2531);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul(2523, 2438, 2533);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::add(A::sub(A::offset(s.ad_value(2532), (-1.0)), s.ad_value(2523)), A::mul(s.ad_value(2438), A::sub_from_scalar(1.0, s.ad_value(2535))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::add(A::add(A::offset(A::sub(s.ad_value(2532), s.ad_value(2531)), (-1.0)), s.ad_value(2523)), A::mul(s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2532), s.ad_value(2523)), A::mul(s.ad_value(2438), s.ad_value(2536)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2437, A::neg(s.ad_value(2531)), A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2539, 1.0, A::offset(A::scale(s.ad_value(2398), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2540, A::offset(A::mul(A::scale(s.ad_value(2434), 1.25), s.ad_value(2539)), (-1.0)), 2539);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad(2541, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));
        }

        s.v[2562] = if ((-s.v[2541]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2562] != 0.0)) {
            s.store_exp_ad(2522, A::neg(s.ad_value(2541)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2562] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar(2542, 1.0, 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2543, A::add(s.ad_value(2417), A::scale(s.ad_value(2399), 0.5)), A::mul(s.ad_value(2398), A::sqrt(A::sub(A::add(s.ad_value(2417), A::scale(s.ad_value(2399), 0.25)), s.ad_value(2542)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_offset(2544, 2423, 3.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2527, A::scale(A::sub(A::add(s.ad_value(2543), s.ad_value(2544)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2544), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub(2522, 2417, 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_exp_ad(2523, A::neg(s.ad_value(2527)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2524, 1.0, A::offset(A::square(s.ad_value(2527)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2527)), 2524);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2527), s.ad_value(2524)), s.ad_value(2524)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2524), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2524)), 2524);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            let assign49800_ad_e64198: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534))))))
                }
            };
            s.store_ad(2528, &assign49800_ad_e64198);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::scale(A::mul(s.ad_value(2399), A::sub(s.ad_value(2523), A::mul(s.ad_value(2438), s.ad_value(2536)))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2529, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::sub_from_scalar(1.0, s.ad_value(2523)), A::mul(s.ad_value(2438), A::offset(s.ad_value(2535), 1.0)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2530, A::sub(s.ad_value(2423), s.ad_value(2527)), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add(824, 2528, 2529);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), A::mul(s.ad_value(2528), s.ad_value(2545)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            let assign49860_ad_e64345: A = A::add(s.ad_value(2527), A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), A::mul(s.ad_value(2528), s.ad_value(2545)))))));
            s.store_ad(2546, &assign49860_ad_e64345);
        }

        s.v[2563] = if (s.v[2546] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_exp(2532, 2546);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_mul(2532, 2438, 2532);
        }

        s.v[2564] = if (s.v[2546] > (s.v[2423] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (s.v[2564] != 0.0)) {
            s.store_exp_ad(2532, A::sub(s.ad_value(2546), s.ad_value(2423)));
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (s.v[2564] != 0.0)) {
            s.store_div(2533, 2438, 2532);
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (!(s.v[2564] != 0.0))) {
            s.store_div_from_scalar_ad(2532, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (!(s.v[2564] != 0.0))) {
            s.store_div_from_scalar_ad(2533, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2546), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2546)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2546)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2546), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub(2522, 2417, 2546);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2533)), s.ad_value(2532)), A::mul(s.ad_value(2438), A::offset(s.ad_value(2535), 1.0)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::add(A::offset(A::add(s.ad_value(2533), s.ad_value(2546)), (-1.0)), s.ad_value(2532)), A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2533), s.ad_value(2532)), A::mul(s.ad_value(2438), s.ad_value(2536)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad_rhs(2437, 2546, A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2440, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2441, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2442, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2443, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2444, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2445, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2446, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2447, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2448, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub(2449, 2417, 2437);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2450, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2451, 2413, 2449);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2452, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2453, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2457, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2458, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2460, 1.0);
        }

        s.v[2565] = if (s.v[2417] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1.0, A::offset(A::square(s.ad_value(2437)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_mul_ad_lhs(2439, A::square(s.ad_value(2437)), 2027);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_scale_ad(2440, A::mul(A::mul(s.ad_value(2437), s.ad_value(2027)), s.ad_value(2027)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_mul_ad_lhs(2441, A::mul(A::sub(A::scale(s.ad_value(2027), 8.0), A::scale(s.ad_value(2439), 12.0)), s.ad_value(2027)), 2027);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_scalar(2442, 0.0);
        }

        s.v[2566] = if (s.v[2437] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_exp(2442, 2437);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_div_from_scalar(2443, 1.0, 2442);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_mul(2442, 2438, 2442);
        }

        s.v[2567] = if (s.v[2437] > (s.v[2423] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (s.v[2567] != 0.0)) {
            s.store_exp_ad(2442, A::sub(s.ad_value(2437), s.ad_value(2423)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (s.v[2567] != 0.0)) {
            s.store_div(2443, 2438, 2442);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (!(s.v[2567] != 0.0))) {
            s.store_div_from_scalar_ad(2442, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (!(s.v[2567] != 0.0))) {
            s.store_div_from_scalar_ad(2443, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2437), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_sub_ad_rhs(2444, 2442, A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439))));
        }

        s.v[2568] = if (s.v[2437] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scale_ad(2445, A::mul(A::square(s.ad_value(2437)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2437), A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scale_ad(2444, A::mul(A::mul(A::mul(A::mul(s.ad_value(2438), s.ad_value(2437)), s.ad_value(2437)), s.ad_value(2437)), A::offset(A::scale(s.ad_value(2437), 1.75), 1.0)), 0.16666666666666666);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2437), A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_offset_ad(2447, A::scale(A::div(A::mul(s.ad_value(2398), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), A::scale(A::square(s.ad_value(2437)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_add_ad_lhs(2445, A::offset(s.ad_value(2437), (-1.0)), 2443);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_sqrt(2446, 2445);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_offset_ad(2447, A::scale(A::div(A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, s.ad_value(2443))), s.ad_value(2446)), 0.5), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_div_ad(2448, A::offset(A::mul(A::scale(s.ad_value(708), 0.2), s.ad_value(2397)), 1.0), A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0));
        }

        s.v[2569] = if (s.v[2444] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_rhs(2449, 2398, A::sqrt(A::add(s.ad_value(2445), s.ad_value(2444))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_div_ad(2450, A::mul(A::mul(s.ad_value(2399), s.ad_value(2444)), s.ad_value(2413)), A::add(s.ad_value(2449), A::mul(s.ad_value(2398), s.ad_value(2446))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2451, A::mul(s.ad_value(2446), s.ad_value(2398)), 2413);
        }

        s.v[2570] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2570] != 0.0)) {
            s.store_div_from_scalar_ad(2452, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2397))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2570] != 0.0))) {
            s.store_offset_ad(2452, A::mul(s.ad_value(217), s.ad_value(2397)), 1.0);
        }

        s.v[2571] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2571] != 0.0)) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2571] != 0.0))) {
            s.store_div_from_scalar_ad(2453, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2450)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2454, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2450);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_rhs(2455, 774, A::add(s.ad_value(2451), A::mul(s.ad_value(775), s.ad_value(2450))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_ln_ad(2028, A::div(s.ad_value(2445), A::offset(A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_add_ad(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2457, A::add(A::offset(s.ad_value(2456), 1.0), s.ad_value(2454)), 2448);
        }

        s.v[2572] = if (s.v[221] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2572] != 0.0)) {
            s.store_div_from_scalar_ad(2458, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(2397))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_offset_ad(2458, A::mul(s.ad_value(221), s.ad_value(2397)), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul(2029, 2450, 2458);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2573] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2573] != 0.0)) {
            s.store_div_from_scalar_ad(2460, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2459))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2573] != 0.0))) {
            s.store_offset_ad(2460, A::mul(s.ad_value(222), s.ad_value(2459)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2395, 1822);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2397, 1823);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2413, 1824);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2414, 1825);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2398, 1826);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2399, 1827);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2415, 1828);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2417, 1829);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2422, 1830);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2423, 1831);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2434, 1832);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2435, 1833);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2436, 1834);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2543, 1835);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2438, 1836);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2437, 1837);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2440, 1838);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2441, 1839);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2442, 1840);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2443, 1841);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2445, 1842);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2444, 1843);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2446, 1844);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2447, 1845);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2448, 1846);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2449, 1847);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2450, 1848);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2451, 1849);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2452, 1850);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2453, 1851);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2457, 1852);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2458, 1853);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2460, 1854);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2393, 720);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2394, 777);
        }

        s.v[2574] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2547] != 0.0) && (s.v[2574] != 0.0)) {
            s.copy_ad(2393, 721);
        }

        if ((s.v[2547] != 0.0) && (s.v[2574] != 0.0)) {
            s.copy_ad(2394, 778);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2462, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scale(2461, 2413, 4.60517018598809);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2478, 2461);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2479, 826);
        }

        if (s.v[2547] != 0.0) {
            s.store_mul(2480, 826, 2414);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2484, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2485, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2488, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2490, 2443);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2491, 2445);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2493, 2444);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2494, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2495, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2496, 2443);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2498, 2444);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2499, 2445);
        }

        if (s.v[2547] != 0.0) {
            s.store_sub(2500, 2417, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2501, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2503, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2502, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2512, 2450);
        }

        if (s.v[2547] != 0.0) {
            s.store_mul(2516, 2500, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2513, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2514, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2519, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2518, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2521, 2393);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2520, 2516);
        }

        s.v[2575] = if (s.v[2417] > 0.0) { 1.0 } else { 0.0 };

        s.v[2576] = if (s.v[2444] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2521, 2393, 2460);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div(2462, 2521, 2457);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_add_ad_rhs(2463, 2449, A::scale(s.ad_value(2399), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_ad_lhs(2027, A::div(A::mul(s.ad_value(2399), s.ad_value(2442)), s.ad_value(2463)), 2463);
        }

        s.v[2577] = if (s.v[2027] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.v[2578] = if (s.v[2028] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) && (s.v[2578] != 0.0)) {
            s.store_scalar(2029, 1.0);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) && (!(s.v[2578] != 0.0))) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) {
            s.store_scale(2029, 2027, 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2464, 2029, 2463);
        }

        s.v[2579] = if ((s.v[706] > 0.0) && (s.v[707] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2465, A::scale(s.ad_value(2413), 0.475), 2464);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_sub_ad_rhs(2027, 2450, A::mul(s.ad_value(2447), s.ad_value(2465)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_scale_ad(2466, A::add(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1e-12))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad(2467, A::sub(A::mul(s.ad_value(2413), s.ad_value(2449)), s.ad_value(2450)), A::mul(A::offset(s.ad_value(2447), (-1.0)), s.ad_value(2465)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_offset_ad(2468, A::div(A::mul(A::scale(s.ad_value(2399), 0.5), s.ad_value(2413)), s.ad_value(2467)), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad_rhs(2027, 2467, A::mul(s.ad_value(775), s.ad_value(2466)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_ad(2469, &A::pow(A::mul(A::mul(s.ad_value(774), s.ad_value(2027)), s.ad_value(704)), s.ad_value(705)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2028, A::div(A::mul(s.ad_value(705), A::offset(A::mul(s.ad_value(2468), A::sub_from_scalar(1.0, s.ad_value(775))), (-1.0))), s.ad_value(2027)), 2469);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_div(2027, 2466, 2467);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_rhs(2470, 706, A::pow(A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2029, A::div(A::mul(s.ad_value(707), A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(2027), 1.0)))), s.ad_value(2467)), 2470);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2471, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2466);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_offset_ad(2027, A::div(A::sub(s.ad_value(2028), A::mul(A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), s.ad_value(2468))), s.ad_value(2029)), 1.0);
        }

        s.v[2580] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) && (s.v[2580] != 0.0)) {
            s.store_scale_ad(2028, A::ln(A::offset(A::exp(A::scale(s.ad_value(2027), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) && (!(s.v[2580] != 0.0))) {
            s.copy_ad(2028, 2027);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_div_ad(2472, A::mul(A::mul(A::neg(s.ad_value(2465)), s.ad_value(2029)), s.ad_value(2028)), A::add(A::add(A::offset(s.ad_value(2469), 1.0), s.ad_value(2470)), s.ad_value(2471)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_rhs(2473, 2464, A::offset(A::div(s.ad_value(2472), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2472)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2579] != 0.0))) {
            s.copy_ad(2473, 2464);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_scale_ad(2474, A::mul(A::mul(s.ad_value(2413), s.ad_value(2462)), s.ad_value(2473)), 0.7071067811865475);
        }

        s.v[2581] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2581] != 0.0)) {
            s.store_div_ad_rhs(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_from_scalar_ad(2475, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2474), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2027, 2475, 2474);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul_ad(2476, A::mul(s.ad_value(2473), s.ad_value(2475)), A::offset(A::div(A::mul(A::scale(s.ad_value(2027), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(2027), s.ad_value(2475)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(2027), 4.0), s.ad_value(2027)), s.ad_value(2475)), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_scale(2477, 2476, 0.99);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_ad_lhs(2027, A::mul(A::mul(s.ad_value(2477), A::sub(s.ad_value(2477), A::scale(s.ad_value(2463), 2.0))), s.ad_value(2415)), 2444);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul_ad_rhs(2478, 2413, A::sub(s.ad_value(2477), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.copy_ad(2478, 2461);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_offset(2027, 2394, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad_lhs(2028, A::mul(A::sqrt(s.ad_value(2027)), s.ad_value(826)), 2478);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scale(2027, 2028, 2.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad(2479, A::mul(s.ad_value(2478), s.ad_value(2027)), A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2480, 2479, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add(2481, 2423, 2480);
        }

        s.v[2582] = if (s.v[2480] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2582] != 0.0)) {
            s.store_exp_ad(2482, A::neg(s.ad_value(2480)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2582] != 0.0))) {
            s.store_div_from_scalar_ad(2482, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2480), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2480), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2480), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2483, 2438, 2482);
        }

        s.v[2583] = if (((s.v[2417]) as f64).abs() <= s.v[2435]) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_scale_ad(2523, A::square(s.ad_value(2436)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_mul_ad(2484, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2417), A::sub_from_scalar(1.0, s.ad_value(2483))), s.ad_value(2398)), s.ad_value(2523)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_offset(2544, 2481, 3.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2527, A::scale(A::sub(A::add(s.ad_value(2543), s.ad_value(2544)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2544), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0))), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub(2522, 2417, 2527);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_exp_ad(2523, A::neg(s.ad_value(2527)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_div_from_scalar_ad(2524, 1.0, A::offset(A::square(s.ad_value(2527)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2527)), 2524);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2527), s.ad_value(2524)), s.ad_value(2524)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2524), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2524)), 2524);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            let assign52130_ad_e66997: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534))))))
                }
            };
            s.store_ad(2528, &assign52130_ad_e66997);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::scale(A::mul(s.ad_value(2399), A::sub(s.ad_value(2523), A::mul(s.ad_value(2483), s.ad_value(2536)))), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2529, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::sub_from_scalar(1.0, s.ad_value(2523)), A::mul(s.ad_value(2483), A::offset(s.ad_value(2535), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2530, A::sub(s.ad_value(2481), s.ad_value(2527)), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add(824, 2528, 2529);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), A::mul(s.ad_value(2528), s.ad_value(2545)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            let assign52190_ad_e67126: A = A::add(s.ad_value(2527), A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), A::mul(s.ad_value(2528), s.ad_value(2545)))))));
            s.store_ad(2546, &assign52190_ad_e67126);
        }

        s.v[2584] = if (s.v[2546] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_exp(2532, 2546);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_mul(2532, 2483, 2532);
        }

        s.v[2585] = if (s.v[2546] > (s.v[2481] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2585] != 0.0)) {
            s.store_exp_ad(2532, A::sub(s.ad_value(2546), s.ad_value(2481)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2585] != 0.0)) {
            s.store_div(2533, 2483, 2532);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2585] != 0.0))) {
            s.store_div_from_scalar_ad(2532, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2585] != 0.0))) {
            s.store_div_from_scalar_ad(2533, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2546), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2546)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2546)), 2522);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2546), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub(2522, 2417, 2546);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2533)), s.ad_value(2532)), A::mul(s.ad_value(2483), A::offset(s.ad_value(2535), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::add(A::offset(A::add(s.ad_value(2533), s.ad_value(2546)), (-1.0)), s.ad_value(2532)), A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534))))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2533), s.ad_value(2532)), A::mul(s.ad_value(2483), s.ad_value(2536)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad_rhs(2484, 2546, A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_sub(2485, 2484, 2437);
        }

        s.v[2586] = if (s.v[2485] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_add_ad(2486, A::scale(A::sub(s.ad_value(2417), s.ad_value(2437)), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2443)), A::mul(s.ad_value(2442), s.ad_value(2482))), A::mul(s.ad_value(2483), A::offset(s.ad_value(2440), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_mul_ad_lhs(2487, A::mul(s.ad_value(2399), A::sub_from_scalar(1.0, s.ad_value(2482))), 2444);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2443), A::mul(s.ad_value(2442), s.ad_value(2482))), A::mul(s.ad_value(2483), s.ad_value(2441)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sub_ad(2027, A::square(s.ad_value(2486)), A::scale(A::mul(s.ad_value(2027), s.ad_value(2487)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_scale_ad(2485, A::div(s.ad_value(2487), A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027)))), 2.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_add(2484, 2437, 2485);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2488, 2485, 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad(2489, A::square(s.ad_value(2484)), A::offset(A::square(s.ad_value(2484)), 2.0));
        }

        s.v[2587] = if (s.v[2484] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) {
            s.store_exp_ad(2490, A::neg(s.ad_value(2484)));
        }

        s.v[2588] = if (s.v[2484] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_scale_ad(2491, A::mul(A::square(s.ad_value(2484)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2484), A::sub_from_scalar(1.0, A::scale(s.ad_value(2484), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2484), A::sub_from_scalar(1.0, A::scale(s.ad_value(2484), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_mul_ad(2493, A::mul(A::mul(A::mul(A::scale(s.ad_value(2483), 0.16666666666666666), s.ad_value(2484)), s.ad_value(2484)), s.ad_value(2484)), A::offset(A::scale(s.ad_value(2484), 1.75), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_sqrt(2492, 2491);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_mul_ad_rhs(2493, 2483, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2490)), s.ad_value(2484)), (-1.0)), s.ad_value(2489)));
        }

        s.v[2589] = if (s.v[2484] > (s.v[2481] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2484), s.ad_value(2481)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_div(2490, 2483, 2027);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub_ad_rhs(2493, 2027, A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2490, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2484), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2484), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2484), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_sub_ad_rhs(2493, 2027, A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) {
            s.store_sqrt(2492, 2491);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2494, A::mul(s.ad_value(2492), s.ad_value(2398)), 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scaled_add(2495, 2437, 2484, 0.5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scalar(2496, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2027, 2490, 2443);
        }

        s.v[2590] = if (s.v[2027] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_sqrt(2496, 2027);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scaled_add(2497, 2444, 2493, 0.5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2498, 2497, A::scale(A::mul(A::square(s.ad_value(2485)), A::sub(s.ad_value(2496), A::scale(s.ad_value(2415), 2.0))), 0.125));
        }

        s.v[2591] = if (s.v[2495] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_scale_ad(2499, A::mul(A::square(s.ad_value(2495)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2495), A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        s.v[2592] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) && (s.v[2592] != 0.0)) {
            s.store_div_from_scalar_ad(2501, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2495), A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_add_ad_rhs(2503, 2501, A::scale(A::div(A::mul(s.ad_value(2398), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), A::scale(A::square(s.ad_value(2495)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        s.v[2593] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2504, A::sub_from_scalar(1.0, s.ad_value(2496)), A::scale(A::mul(s.ad_value(2500), s.ad_value(2415)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_from_scalar_ad(2501, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad_rhs(2027, 2501, A::offset(s.ad_value(2501), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2505, 730, A::mul(A::mul(A::square(s.ad_value(2027)), s.ad_value(2399)), s.ad_value(2498)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2506, A::scale(A::sub(s.ad_value(2500), s.ad_value(2505)), 2.0), A::mul(s.ad_value(2399), A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2507, 2505, A::sub(s.ad_value(2505), A::scale(s.ad_value(2500), 2.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_sub_from_scalar_ad(2508, 1.0, A::scale(A::mul(s.ad_value(2399), A::add(s.ad_value(2496), s.ad_value(2498))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad(2509, A::mul(s.ad_value(2507), s.ad_value(2506)), A::sub(A::square(s.ad_value(2506)), A::mul(s.ad_value(2508), s.ad_value(2507))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add(2495, 2495, 2509);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_exp(2510, 2509);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div(2496, 2496, 2510);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul(2498, 2498, 2510);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::scale(A::mul(A::mul(s.ad_value(2500), s.ad_value(2501)), s.ad_value(2415)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad(2485, A::mul(A::mul(s.ad_value(2485), s.ad_value(2510)), A::add(s.ad_value(2504), s.ad_value(2497))), A::add(s.ad_value(2511), A::mul(s.ad_value(2510), s.ad_value(2497))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul(2488, 2485, 2413);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_sqrt(2502, 2499);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_add_ad_rhs(2503, 2501, A::scale(A::div(A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, s.ad_value(2496))), s.ad_value(2502)), 0.5));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_rhs(2512, 2413, A::div(A::mul(s.ad_value(2399), s.ad_value(2498)), A::add(s.ad_value(2500), A::mul(s.ad_value(2398), s.ad_value(2502)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2513, 2512, A::mul(s.ad_value(2413), s.ad_value(2503)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2514, A::mul(s.ad_value(2502), s.ad_value(2398)), 2413);
        }

        s.v[2594] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2594] != 0.0)) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2594] != 0.0))) {
            s.store_div_from_scalar_ad(2453, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2512)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2454, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2512);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2515, 2514, A::mul(s.ad_value(775), s.ad_value(2512)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2516, 2514, A::mul(s.ad_value(776), s.ad_value(2512)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2517, 774, 2515);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_ln_ad(2028, A::div(s.ad_value(2499), A::offset(A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14)));
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2518, A::add(A::offset(s.ad_value(2456), 1.0), s.ad_value(2454)), 2448);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_ln_ad(2519, A::div(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2029, 2512, 2458);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2595] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2595] != 0.0)) {
            s.store_div_from_scalar_ad(2460, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2459))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2595] != 0.0))) {
            s.store_offset_ad(2460, A::mul(s.ad_value(222), s.ad_value(2459)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2521, 2393, 2460);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2520, 2500, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1887, 2395);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1888, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1889, 2398);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1890, 2417);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1891, 2422);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1892, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1893, 2488);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1894, 2494);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1895, 2501);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1896, 2503);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1897, 2512);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1898, 2513);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1899, 2516);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1900, 2518);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1901, 2519);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1902, 2521);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1903, 2520);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1932, 2414);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1933, 2435);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1934, 2495);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1935, 2500);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(745, 728);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1887, 1822);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1888, 1824);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1889, 1826);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1890, 1829);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1891, 1830);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1892, 1849);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1893, 1860);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1894, 1861);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1895, 1863);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1896, 1864);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1897, 1865);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1898, 1866);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1899, 1868);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1900, 1869);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1901, 1871);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1902, 1870);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1903, 1872);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1932, 1825);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1933, 1833);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1934, 1862);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1935, 1931);
        }

        s.copy_ad(1904, 255);

        s.v[2596] = if (s.v[773] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2596] != 0.0) {
            s.store_div_ad_rhs(1904, 255, A::offset(A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1905] = 1.0;

        s.v[1906] = 1.0;

        s.v[1907] = 0.0;

        s.v[1908] = 1.0;

        s.v[1909] = 1.0;

        s.copy_ad(2359, 1903);

        s.v[2362] = 0.0;

        s.v[2361] = 0.0;

        s.copy_ad(2363, 2359);

        s.v[2597] = if (s.v[1890] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2597] != 0.0) {
            s.store_mul_ad_lhs(2354, A::div(A::mul(A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), s.ad_value(1897)), s.ad_value(1898)), 1901);
        }

        s.v[2598] = if (s.v[2354] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2598] != 0.0)) {
            s.store_div_from_scalar_ad(1905, 1.0, A::add(A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354))));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2598] != 0.0))) {
            s.store_sub_from_scalar(1905, 1.0, 2354);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul(1906, 1900, 1905);
        }

        if (s.v[2597] != 0.0) {
            s.store_div(1907, 1902, 1906);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul_ad_lhs(2355, A::mul(A::square(s.ad_value(1907)), s.ad_value(1893)), 1893);
        }

        s.v[2599] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2599] != 0.0)) {
            s.store_div_ad_rhs(2355, 2355, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2597] != 0.0) {
            s.store_scale_ad(1908, A::mul(s.ad_value(1906), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2355), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_div(2027, 1906, 1908);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul_ad_rhs(2356, 1896, A::offset(A::scale(A::mul(A::mul(s.ad_value(2355), s.ad_value(2027)), s.ad_value(2027)), 0.5), 1.0));
        }

        if (s.v[2597] != 0.0) {
            s.store_div_ad_lhs(1909, A::mul(s.ad_value(2027), s.ad_value(1898)), 2356);
        }

        if (s.v[2597] != 0.0) {
            s.store_scaled_div(2357, 1893, 1909, 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_square(2358, 2357);
        }

        if (s.v[2597] != 0.0) {
            s.store_add_ad_rhs(2359, 1903, A::scale(A::mul(A::mul(s.ad_value(1895), s.ad_value(1893)), A::add(A::offset(A::scale(A::mul(s.ad_value(2357), s.ad_value(1905)), 0.3333333333333333), (-1.0)), s.ad_value(1905))), 0.5));
        }

        if (s.v[2597] != 0.0) {
            s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);
        }

        s.v[2600] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2600] != 0.0)) {
            s.store_scalar(2360, 0.0);
        }

        if ((s.v[2597] != 0.0) && (s.v[2600] != 0.0)) {
            s.store_mul_ad(2361, A::mul(A::scale(s.ad_value(1905), 0.5), s.ad_value(1905)), A::sub(s.ad_value(1897), A::mul(A::scale(s.ad_value(2027), 3.0), A::sub_from_scalar(2.0, s.ad_value(2357)))));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2600] != 0.0))) {
            s.store_mul_ad(2360, A::sub_from_scalar(1.0, s.ad_value(1905)), A::sub(s.ad_value(1897), A::scale(A::mul(s.ad_value(1896), s.ad_value(1893)), 0.5)));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2600] != 0.0))) {
            s.store_scale_ad(2361, A::add(A::mul(A::square(s.ad_value(1905)), A::sub(s.ad_value(1897), A::mul(s.ad_value(2027), A::sub(A::sub_from_scalar(1.0, s.ad_value(2357)), A::scale(s.ad_value(2358), 0.2))))), A::mul(s.ad_value(2360), A::offset(s.ad_value(1905), 1.0))), 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_add_ad_lhs(2362, A::mul(s.ad_value(1905), A::add(s.ad_value(1897), A::mul(s.ad_value(2027), s.ad_value(2357)))), 2360);
        }

        if (s.v[2597] != 0.0) {
            s.store_sub(2363, 2359, 2362);
        }

        s.store_mul(851, 2359, 1904);

        s.store_mul_ad_lhs(853, A::neg(s.ad_value(2361)), 1904);

        s.store_mul_ad_lhs(852, A::neg(s.ad_value(2363)), 1904);

        s.v[2379] = 0.0;

        s.v[2380] = 0.0;

        s.v[2378] = 0.0;

        s.v[2601] = if ((s.v[268] > 0.0) || (s.v[269] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2601] != 0.0) {
            s.store_scalar(2368, 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.copy_ad(2367, 1887);
        }

        s.v[2602] = if (s.v[272] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_add_ad_lhs(2364, A::sub(s.ad_value(1887), s.ad_value(270)), 808);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_scale_ad(2027, A::add(A::add(s.ad_value(2364), s.ad_value(808)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2364), s.ad_value(808)), A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)))), 0.5);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2027, A::sub(A::sub(A::scale(s.ad_value(2027), 2.0), s.ad_value(808)), s.ad_value(2364)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_div(2029, 808, 2027);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_mul(2365, 2364, 2029);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_sqrt_ad(2366, A::sub_from_scalar(1.0, A::mul(s.ad_value(2365), s.ad_value(272))));
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_sub_ad_lhs(2367, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), s.ad_value(2364)), 2365);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_offset_ad(2368, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add(s.ad_value(2028), A::mul(s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027))))), s.ad_value(2029)), s.ad_value(2028)), 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.store_scalar(2370, 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.store_scalar(2371, 0.0);
        }

        s.v[2603] = if (s.v[271] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_add_ad(2027, A::scale(s.ad_value(745), 0.5), A::mul(s.ad_value(1888), A::offset(A::scale(s.ad_value(1889), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_div(2369, 1887, 2027);
        }

        s.v[2604] = if (((s.v[2369]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (s.v[2604] != 0.0)) {
            s.store_div_from_scalar_ad(2370, 1.0, A::offset(A::exp(A::neg(s.ad_value(2369))), 1.0));
        }

        s.v[2605] = if (s.v[2369] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (!(s.v[2604] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_div_from_scalar_ad(2370, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2369), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2369), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2369), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2606] = if (s.v[2369] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (s.v[2606] != 0.0)) {
            s.store_ln_ad(2028, A::offset(A::exp(s.ad_value(2369)), 1.0));
        }

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (!(s.v[2606] != 0.0))) {
            s.copy_ad(2028, 2369);
        }

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_mul(2371, 2027, 2028);
        }

        if (s.v[2601] != 0.0) {
            s.store_add_ad_lhs(2372, A::mul(s.ad_value(271), A::sub(s.ad_value(2370), s.ad_value(2368))), 2368);
        }

        if (s.v[2601] != 0.0) {
            s.store_add_ad_lhs(2373, A::mul(s.ad_value(271), A::sub(s.ad_value(2371), s.ad_value(2367))), 2367);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad(2374, A::sub(A::sub(s.ad_value(1887), A::mul(s.ad_value(1888), s.ad_value(1891))), s.ad_value(1903)), A::scale(s.ad_value(1893), 0.5));
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2375, A::sub(s.ad_value(1887), s.ad_value(2374)), 1892);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2376, A::add(s.ad_value(1893), s.ad_value(2374)), 826);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2377, A::sub(s.ad_value(1887), s.ad_value(2376)), 1894);
        }

        s.v[2607] = if (s.v[831] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2378, 2372, A::add(A::mul(s.ad_value(269), s.ad_value(2376)), A::mul(s.ad_value(268), s.ad_value(2374))));
        }

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2379, 268, A::sub(s.ad_value(2375), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2380, 269, A::sub(s.ad_value(2377), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2378, 2372, A::add(A::mul(s.ad_value(268), s.ad_value(2376)), A::mul(s.ad_value(269), s.ad_value(2374))));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2379, 269, A::sub(s.ad_value(2375), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2380, 268, A::sub(s.ad_value(2377), s.ad_value(2373)));
        }

        if (s.v[2601] != 0.0) {
            s.store_add(851, 851, 2378);
        }

        if (s.v[2601] != 0.0) {
            s.store_add(853, 853, 2380);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(852, A::sub(A::sub(s.ad_value(852), s.ad_value(2378)), s.ad_value(2380)), 2379);
        }

        s.store_mul(1910, 262, 1878);

        s.store_mul(1911, 263, 1879);

        s.v[2383] = 0.0;

        s.v[2381] = 0.0;

        s.v[2608] = if ((s.v[262] > 0.0) && (s.v[264] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2608] != 0.0) {
            s.store_mul_ad_rhs(2027, 266, A::add(A::scale(s.ad_value(1819), 0.5), s.ad_value(787)));
        }

        s.v[2609] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2610] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2610] != 0.0)) {
            s.store_exp(2381, 2027);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2610] != 0.0))) {
            s.store_div_from_scalar_ad(2381, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2611] = if (s.v[2381] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2611] != 0.0)) {
            s.store_ln_ad(2382, A::offset(s.ad_value(2381), 1.0));
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2611] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2382, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0))));
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2611] != 0.0))) {
            s.copy_ad(2382, 2381);
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2611] != 0.0))) {
            s.store_div_ad(2028, A::scale(s.ad_value(2382), 2.0), A::offset(s.ad_value(2382), 2.0));
        }

        if ((s.v[2608] != 0.0) && (!(s.v[2609] != 0.0))) {
            s.copy_ad(2382, 2027);
        }

        if ((s.v[2608] != 0.0) && (!(s.v[2609] != 0.0))) {
            s.store_mul_ad_rhs(2028, 2382, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0))));
        }

        if (s.v[2608] != 0.0) {
            s.store_mul_ad_lhs(2383, A::scale(A::mul(A::div(A::scale(s.ad_value(264), (-2.0)), s.ad_value(266)), s.ad_value(262)), s.v[354]), 2028);
        }

        s.v[2386] = 0.0;

        s.v[2384] = 0.0;

        s.v[2612] = if ((s.v[263] > 0.0) && (s.v[265] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2612] != 0.0) {
            s.store_mul_ad_rhs(2027, 266, A::add(A::scale(s.ad_value(1819), 0.5), s.ad_value(788)));
        }

        s.v[2613] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2614] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2614] != 0.0)) {
            s.store_exp(2384, 2027);
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2614] != 0.0))) {
            s.store_div_from_scalar_ad(2384, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2615] = if (s.v[2384] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2615] != 0.0)) {
            s.store_ln_ad(2385, A::offset(s.ad_value(2384), 1.0));
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2615] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2385, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0))));
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2615] != 0.0))) {
            s.copy_ad(2385, 2384);
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2615] != 0.0))) {
            s.store_div_ad(2028, A::scale(s.ad_value(2385), 2.0), A::offset(s.ad_value(2385), 2.0));
        }

        if ((s.v[2612] != 0.0) && (!(s.v[2613] != 0.0))) {
            s.copy_ad(2385, 2027);
        }

        if ((s.v[2612] != 0.0) && (!(s.v[2613] != 0.0))) {
            s.store_mul_ad_rhs(2028, 2385, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0))));
        }

        if (s.v[2612] != 0.0) {
            s.store_mul_ad_lhs(2386, A::scale(A::mul(A::div(A::scale(s.ad_value(265), (-2.0)), s.ad_value(266)), s.ad_value(263)), s.v[354]), 2028);
        }

        s.store_add(2387, 2383, 2386);

        s.store_add_ad_lhs(856, A::mul(s.ad_value(267), s.ad_value(829)), 2387);

        s.store_mul(854, 274, 834);

        s.store_mul(855, 275, 837);

        s.v[1938] = 0.0;

        s.v[1939] = 0.0;

        s.v[1940] = 0.0;

        s.v[1941] = 0.0;

        s.v[2616] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        s.v[2617] = if (s.v[1890] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.store_scalar(1936, 0.5);
        }

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.store_scalar(1937, 1.0);
        }

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.copy_ad(1938, 1889);
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_scale_ad(1936, A::offset(A::scale(A::div(s.ad_value(1893), s.ad_value(1909)), 0.25), 1.0), 0.5);
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_div_ad_rhs(1937, 1935, A::sub(s.ad_value(1890), s.ad_value(1934)));
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_div(1938, 1889, 1937);
        }

        if (s.v[2616] != 0.0) {
            s.store_square(1939, 1938);
        }

        if (s.v[2616] != 0.0) {
            s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);
        }

        if (s.v[2616] != 0.0) {
            s.store_scale(1941, 1940, 1e-5);
        }

        s.v[2618] = 0.0;

        s.v[2621] = 0.0;

        s.v[2622] = 0.0;

        s.v[2623] = 0.0;

        s.v[2624] = 0.0;

        s.v[2625] = 0.0;

        s.v[2626] = 0.0;

        s.v[2627] = 0.0;

        s.v[2628] = 0.0;

        s.v[2629] = 0.0;

        s.v[2630] = 0.0;

        s.v[2631] = 0.0;

        s.v[2632] = 0.0;

        s.v[2633] = 0.0;

        s.v[2634] = 0.0;

        s.v[2635] = 0.0;

        s.v[2636] = 0.0;

        s.v[2639] = 0.0;

        s.v[2643] = 0.0;

        s.v[2646] = 0.0;

        s.v[2647] = 0.0;

        s.v[2648] = 0.0;

        s.v[2649] = 0.0;

        s.v[2650] = 0.0;

        s.v[2651] = 0.0;

        s.v[2654] = 0.0;

        s.v[2655] = 0.0;

        s.v[2656] = 0.0;

        s.v[2657] = 0.0;

        s.v[2661] = 0.0;

        s.v[2663] = 0.0;

        s.v[2664] = 0.0;

        s.v[857] = 0.0;

        s.v[1918] = 0.0;

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[858] = 0.0;

        s.v[1921] = 0.0;

        s.v[1922] = 0.0;

        s.v[1923] = 0.0;

        s.v[2665] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2666] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2669, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2670, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_rhs(2623, 832, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale_ad(2670, A::div(A::mul(s.ad_value(832), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2671] = if (s.v[651] > 0.5) { 1.0 } else { 0.0 };

        s.v[2672] = if (s.v[408] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) && (s.v[2672] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) && (!(s.v[2672] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) {
            s.store_add_ad(1918, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[417]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[420]));
        }

        s.v[2673] = if (s.v[652] > 0.5) { 1.0 } else { 0.0 };

        s.v[2674] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) && (s.v[2674] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) && (!(s.v[2674] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) {
            s.store_add_ad(1919, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[418]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[421]));
        }

        s.v[2675] = if (s.v[653] > 0.5) { 1.0 } else { 0.0 };

        s.v[2676] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) && (s.v[2676] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) && (!(s.v[2676] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) {
            s.store_add_ad(1920, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[419]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[422]));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2669, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2670, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_rhs(2623, 833, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale_ad(2670, A::div(A::mul(s.ad_value(833), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2677] = if (s.v[678] > 0.5) { 1.0 } else { 0.0 };

        s.v[2678] = if (s.v[575] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) && (s.v[2678] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) && (!(s.v[2678] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))), s.ad_value(575)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) {
            s.store_add_ad(1921, A::mul(s.ad_value(584), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2679] = if (s.v[679] > 0.5) { 1.0 } else { 0.0 };

        s.v[2680] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) && (s.v[2680] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) && (!(s.v[2680] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) {
            s.store_add_ad(1922, A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2681] = if (s.v[680] > 0.5) { 1.0 } else { 0.0 };

        s.v[2682] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) && (s.v[2682] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) && (!(s.v[2682] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) {
            s.store_add_ad(1923, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2683] = if (p.p872 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_scale_ad(642, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873))), p.p872);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_offset(640, 642, p.p862);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_div_from_scalar(450, 1.0, 640);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2683] != 0.0))) {
            s.store_scalar(640, p.p862);
        }

        s.v[2684] = if (p.p874 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2684] != 0.0)) {
            s.store_scale_ad(644, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875))), p.p874);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2684] != 0.0)) {
            s.store_mul_ad_rhs(443, 443, A::offset(s.ad_value(644), 1.0));
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2634, 0.0);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2631, 0.0);
        }

        s.v[2685] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_add_ad_rhs(2623, 832, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2628, A::div(A::mul(s.ad_value(832), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2686] = if (s.v[832] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[2687] = if (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (s.v[2687] != 0.0)) {
            s.store_exp_ad(2629, A::scale(s.ad_value(832), (s.v[371] * (-0.5))));
        }

        s.v[2688] = if (((-0.5) * (s.v[832] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (!(s.v[2687] != 0.0))) && (s.v[2688] != 0.0)) {
            let assign56700_ad_e71361: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2629, &assign56700_ad_e71361);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (!(s.v[2687] != 0.0))) && (!(s.v[2688] != 0.0))) {
            s.store_scale_ad(2629, A::offset(A::mul(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) {
            s.store_div_from_scalar(2630, 1.0, 2629);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) {
            s.store_square(2627, 2630);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_mul_ad_lhs(2627, A::offset(A::scale(A::sub(s.ad_value(832), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_sqrt(2630, 2627);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.v[2689] = if (s.v[832] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2689] != 0.0)) {
            s.store_scale_ad(2631, A::ln(A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2629), 1.0), A::offset(s.ad_value(2629), 3.0))))), (s.v[370] * 2.0));
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2689] != 0.0))) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2630), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2630), 1.0), A::offset(A::scale(s.ad_value(2630), 3.0), 1.0))))), (s.v[370] * 2.0)), 832);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sub(2632, 656, 2631);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2633, A::sub(A::add(s.ad_value(832), s.ad_value(2632)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(2632)), A::sub(s.ad_value(832), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2634, A::sub(A::add(s.ad_value(832), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(659)), A::sub(s.ad_value(832), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2635, A::sub(s.ad_value(832), A::sqrt(A::offset(A::mul(s.ad_value(832), s.ad_value(832)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2690] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2690] != 0.0)) {
            s.store_scalar(1918, 0.0);
        }

        s.v[2691] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[393], 2633);
        }

        s.v[2693] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (s.v[2693] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[429]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (!(s.v[2693] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[429]), p.p831);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_scale(2643, 2636, s.v[423]);
        }

        s.v[2694] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[408]), s.ad_value(2639)), s.v[438]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[435]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[435])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2697] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2697] != 0.0))) {
            let assign57250_ad_e72311: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign57250_ad_e72311);
        }

        s.v[2698] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2699] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2699] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2699] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2700] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        s.v[2701] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (s.v[2701] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2701] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]), p.p831);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[426]), s.ad_value(2636)), s.v[411]);
        }

        s.v[2702] = if (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (s.v[2702] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(441)), s.ad_value(2661)));
        }

        s.v[2703] = if (((-s.v[441]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2702] != 0.0))) && (s.v[2703] != 0.0)) {
            let assign57440_ad_e72651: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign57440_ad_e72651);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2702] != 0.0))) && (!(s.v[2703] != 0.0))) {
            let assign57450_ad_e72702: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign57450_ad_e72702);
        }

        s.v[2704] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2705] = if (s.v[2635] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[2706] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) && (s.v[2706] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[448]), A::scale(s.ad_value(2635), s.v[448])), A::scale(s.ad_value(2635), s.v[448])), A::scale(s.ad_value(2635), s.v[448]));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) && (!(s.v[2706] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[448])), p.p863);
        }

        s.v[2707] = if (s.v[408] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2707] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2707] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_scale_ad(1918, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[417]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[420])), p.p30);
        }

        s.v[2708] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2708] != 0.0)) {
            s.store_scalar(1919, 0.0);
        }

        s.v[2709] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[394], 2633);
        }

        s.v[2711] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[430]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (!(s.v[2711] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[430]), p.p832);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_scale(2643, 2636, s.v[424]);
        }

        s.v[2712] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[409]), s.ad_value(2639)), s.v[439]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[436]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[436])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2715] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (s.v[2715] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2715] != 0.0))) {
            let assign58000_ad_e73577: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign58000_ad_e73577);
        }

        s.v[2716] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2717] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2716] != 0.0))) && (s.v[2717] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2716] != 0.0))) && (!(s.v[2717] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2718] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        s.v[2719] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (s.v[2719] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2719] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]), p.p832);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[427]), s.ad_value(2636)), s.v[412]);
        }

        s.v[2720] = if (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (s.v[2720] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(442)), s.ad_value(2661)));
        }

        s.v[2721] = if (((-s.v[442]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2720] != 0.0))) && (s.v[2721] != 0.0)) {
            let assign58190_ad_e73917: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58190_ad_e73917);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2720] != 0.0))) && (!(s.v[2721] != 0.0))) {
            let assign58200_ad_e73968: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign58200_ad_e73968);
        }

        s.v[2722] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2723] = if (s.v[2635] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[2724] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (s.v[2723] != 0.0)) && (s.v[2724] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[449]), A::scale(s.ad_value(2635), s.v[449])), A::scale(s.ad_value(2635), s.v[449])), A::scale(s.ad_value(2635), s.v[449]));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (s.v[2723] != 0.0)) && (!(s.v[2724] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[449])), p.p864);
        }

        s.v[2725] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2725] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2725] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) {
            s.store_scale_ad(1919, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[418]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[421])), p.p30);
        }

        s.v[2726] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2726] != 0.0)) {
            s.store_scalar(1920, 0.0);
        }

        s.v[2727] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[395], 2633);
        }

        s.v[2729] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (s.v[2729] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[431]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (!(s.v[2729] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[431]), p.p833);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_scale(2643, 2636, s.v[425]);
        }

        s.v[2730] = if (p.p847 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[410]), s.ad_value(2639)), s.v[440]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[437]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[437])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2733] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (s.v[2733] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2733] != 0.0))) {
            let assign58750_ad_e74843: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign58750_ad_e74843);
        }

        s.v[2734] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2735] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2734] != 0.0))) && (s.v[2735] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2734] != 0.0))) && (!(s.v[2735] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2736] = if (p.p853 == 0.0) { 1.0 } else { 0.0 };

        s.v[2737] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (s.v[2737] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2737] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]), p.p833);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[428]), s.ad_value(2636)), s.v[413]);
        }

        s.v[2738] = if (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (s.v[2738] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(443)), s.ad_value(2661)));
        }

        s.v[2739] = if (((-s.v[443]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2738] != 0.0))) && (s.v[2739] != 0.0)) {
            let assign58940_ad_e75183: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58940_ad_e75183);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2738] != 0.0))) && (!(s.v[2739] != 0.0))) {
            let assign58950_ad_e75234: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign58950_ad_e75234);
        }

        s.v[2740] = if (s.v[640] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2741] = if (s.v[2635] > ((-s.v[444]) * s.v[640])) { 1.0 } else { 0.0 };

        s.v[2742] = if (p.p865 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (s.v[2741] != 0.0)) && (s.v[2742] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(450)), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (s.v[2741] != 0.0)) && (!(s.v[2742] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);
        }

        s.v[2743] = if (s.v[473] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            let assign59070_ad_e75459: A = {
                if (s.v[832] < p.p870) {
                    {
                        if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                            A::constant(p.p870)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(832), (-p.p870)), 1.0 / (p.p871))), 1.0)), p.p871), p.p870)
                        }
                    }
                } else {
                    {
                        if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                            s.ad_value(832)
                        } else {
                            A::add(s.ad_value(832), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p870, s.ad_value(832)), 1.0 / (p.p871))), 1.0)), p.p871))
                        }
                    }
                }
            };
            s.store_ad(2663, &assign59070_ad_e75459);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2744] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (s.v[2744] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (!(s.v[2744] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(1920, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(2663), s.ad_value(2664)), s.v[422])), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub_ad_lhs(2663, A::offset(s.ad_value(832), p.p870), 2663);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2745] = if (s.v[467] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (s.v[2745] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (!(s.v[2745] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))), s.ad_value(467)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(472, A::add(A::mul(s.ad_value(470), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(471), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(1920, 1920, 472);
        }

        s.v[2746] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) && (s.v[2746] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) && (!(s.v[2746] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) {
            s.store_scale_ad(1920, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[422])), p.p30);
        }

        s.v[2747] = if (s.v[636] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_mul_ad_rhs(643, 636, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637))));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_add(641, 542, 643);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_div_from_scalar(616, 1.0, 641);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2747] != 0.0))) {
            s.copy_ad(641, 542);
        }

        s.v[2748] = if (s.v[638] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2748] != 0.0)) {
            s.store_mul_ad_rhs(645, 638, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639))));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2748] != 0.0)) {
            s.store_mul_ad_rhs(610, 610, A::offset(s.ad_value(645), 1.0));
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2634, 0.0);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2631, 0.0);
        }

        s.v[2749] = if !(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_add_ad_rhs(2623, 833, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2628, A::div(A::mul(s.ad_value(833), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2750] = if (s.v[833] < s.v[681]) { 1.0 } else { 0.0 };

        s.v[2751] = if (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (s.v[2751] != 0.0)) {
            s.store_exp_ad(2629, A::scale(s.ad_value(833), (s.v[371] * (-0.5))));
        }

        s.v[2752] = if (((-0.5) * (s.v[833] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (!(s.v[2751] != 0.0))) && (s.v[2752] != 0.0)) {
            let assign59600_ad_e76293: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2629, &assign59600_ad_e76293);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (!(s.v[2751] != 0.0))) && (!(s.v[2752] != 0.0))) {
            s.store_scale_ad(2629, A::offset(A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) {
            s.store_div_from_scalar(2630, 1.0, 2629);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) {
            s.store_square(2627, 2630);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_mul_ad_lhs(2627, A::offset(A::scale(A::sub(s.ad_value(833), s.ad_value(681)), s.v[371]), 1.0), 682);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_sqrt(2630, 2627);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.v[2753] = if (s.v[833] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2753] != 0.0)) {
            s.store_scale_ad(2631, A::ln(A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2629), 1.0), A::offset(s.ad_value(2629), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2753] != 0.0))) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2630), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2630), 1.0), A::offset(A::scale(s.ad_value(2630), 3.0), 1.0))))), (s.v[370] * 2.0)), 833);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sub(2632, 683, 2631);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2633, A::sub(A::add(s.ad_value(833), s.ad_value(2632)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(2632)), A::sub(s.ad_value(833), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2634, A::sub(A::add(s.ad_value(833), s.ad_value(686)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(686)), A::sub(s.ad_value(833), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2635, A::sub(s.ad_value(833), A::sqrt(A::offset(A::mul(s.ad_value(833), s.ad_value(833)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2754] = if (s.v[673] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2754] != 0.0)) {
            s.store_scalar(1921, 0.0);
        }

        s.v[2755] = if ((s.v[522] == 0.0) && (s.v[525] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_sub(2639, 569, 2633);
        }

        s.v[2757] = if (s.v[511] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (s.v[2757] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(596)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (!(s.v[2757] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(596)), s.ad_value(511)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_mul(2643, 590, 2636);
        }

        s.v[2758] = if (s.v[525] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_rhs(2646, 605, A::div(A::mul(s.ad_value(2643), s.ad_value(575)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(602), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(602), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(602), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2761] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (s.v[2761] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2761] != 0.0))) {
            let assign60150_ad_e77243: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign60150_ad_e77243);
        }

        s.v[2762] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2763] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2762] != 0.0))) && (s.v[2763] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2762] != 0.0))) && (!(s.v[2763] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2764] = if (s.v[531] == 0.0) { 1.0 } else { 0.0 };

        s.v[2765] = if (s.v[511] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (s.v[2765] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2765] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)), s.ad_value(511)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) {
            s.store_mul_ad_rhs(2661, 578, A::div(A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(593)), s.ad_value(2636)));
        }

        s.v[2766] = if (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (s.v[2766] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(608)), s.ad_value(2661)));
        }

        s.v[2767] = if (((-s.v[608]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2766] != 0.0))) && (s.v[2767] != 0.0)) {
            let assign60340_ad_e77583: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign60340_ad_e77583);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2766] != 0.0))) && (!(s.v[2767] != 0.0))) {
            let assign60350_ad_e77634: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign60350_ad_e77634);
        }

        s.v[2768] = if (s.v[540] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2769] = if (s.v[2635] > ((-s.v[444]) * s.v[540])) { 1.0 } else { 0.0 };

        s.v[2770] = if (s.v[543] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (s.v[2769] != 0.0)) && (s.v[2770] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(614)), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (s.v[2769] != 0.0)) && (!(s.v[2770] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(543)));
        }

        s.v[2771] = if (s.v[575] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2771] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2771] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))), s.ad_value(575)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) {
            s.store_scale_ad(1921, A::add(A::mul(s.ad_value(584), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        s.v[2772] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2772] != 0.0)) {
            s.store_scalar(1922, 0.0);
        }

        s.v[2773] = if ((s.v[523] == 0.0) && (s.v[526] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_sub(2639, 570, 2633);
        }

        s.v[2775] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (s.v[2775] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(597)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (!(s.v[2775] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_mul(2643, 591, 2636);
        }

        s.v[2776] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_rhs(2646, 606, A::div(A::mul(s.ad_value(2643), s.ad_value(576)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(603), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(603), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(603), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2779] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (s.v[2779] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2779] != 0.0))) {
            let assign60900_ad_e78509: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign60900_ad_e78509);
        }

        s.v[2780] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2781] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2780] != 0.0))) && (s.v[2781] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2780] != 0.0))) && (!(s.v[2781] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2782] = if (s.v[532] == 0.0) { 1.0 } else { 0.0 };

        s.v[2783] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (s.v[2783] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2783] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) {
            s.store_mul_ad_rhs(2661, 579, A::div(A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(594)), s.ad_value(2636)));
        }

        s.v[2784] = if (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (s.v[2784] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(609)), s.ad_value(2661)));
        }

        s.v[2785] = if (((-s.v[609]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2784] != 0.0))) && (s.v[2785] != 0.0)) {
            let assign61090_ad_e78849: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61090_ad_e78849);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2784] != 0.0))) && (!(s.v[2785] != 0.0))) {
            let assign61100_ad_e78900: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign61100_ad_e78900);
        }

        s.v[2786] = if (s.v[541] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2787] = if (s.v[2635] > ((-s.v[444]) * s.v[541])) { 1.0 } else { 0.0 };

        s.v[2788] = if (s.v[544] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (s.v[2787] != 0.0)) && (s.v[2788] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(615)), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (s.v[2787] != 0.0)) && (!(s.v[2788] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(544)));
        }

        s.v[2789] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2789] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2789] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) {
            s.store_scale_ad(1922, A::add(A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        s.v[2790] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2790] != 0.0)) {
            s.store_scalar(1923, 0.0);
        }

        s.v[2791] = if ((s.v[524] == 0.0) && (s.v[527] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_sub(2639, 571, 2633);
        }

        s.v[2793] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (s.v[2793] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(598)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (!(s.v[2793] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_mul(2643, 592, 2636);
        }

        s.v[2794] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_rhs(2646, 607, A::div(A::mul(s.ad_value(2643), s.ad_value(577)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(604), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(604), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(604), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2797] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (s.v[2797] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2797] != 0.0))) {
            let assign61650_ad_e79775: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign61650_ad_e79775);
        }

        s.v[2798] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        s.v[2799] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2798] != 0.0))) && (s.v[2799] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2798] != 0.0))) && (!(s.v[2799] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2800] = if (s.v[533] == 0.0) { 1.0 } else { 0.0 };

        s.v[2801] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (s.v[2801] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2801] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) {
            s.store_mul_ad_rhs(2661, 580, A::div(A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(595)), s.ad_value(2636)));
        }

        s.v[2802] = if (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (s.v[2802] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(610)), s.ad_value(2661)));
        }

        s.v[2803] = if (((-s.v[610]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2802] != 0.0))) && (s.v[2803] != 0.0)) {
            let assign61840_ad_e80115: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61840_ad_e80115);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2802] != 0.0))) && (!(s.v[2803] != 0.0))) {
            let assign61850_ad_e80166: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign61850_ad_e80166);
        }

        s.v[2804] = if (s.v[641] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2805] = if (s.v[2635] > ((-s.v[444]) * s.v[641])) { 1.0 } else { 0.0 };

        s.v[2806] = if (s.v[545] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (s.v[2805] != 0.0)) && (s.v[2806] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(616)), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (s.v[2805] != 0.0)) && (!(s.v[2806] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(545)));
        }

        s.v[2807] = if (s.v[635] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            let assign61970_ad_e80391: A = {
                if (s.v[833] < s.v[550]) {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                            s.ad_value(550)
                        } else {
                            A::add(s.ad_value(550), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(833), s.ad_value(550)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                } else {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                            s.ad_value(833)
                        } else {
                            A::add(s.ad_value(833), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(550), s.ad_value(833)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                }
            };
            s.store_ad(2663, &assign61970_ad_e80391);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2808] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (s.v[2808] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (!(s.v[2808] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))), s.ad_value(577)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(1923, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub_ad_lhs(2663, A::add(s.ad_value(833), s.ad_value(550)), 2663);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2809] = if (s.v[630] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (s.v[2809] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (!(s.v[2809] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))), s.ad_value(630)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(472, A::add(A::mul(s.ad_value(633), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(634), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(1923, 1923, 472);
        }

        s.v[2810] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) && (s.v[2810] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) && (!(s.v[2810] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))), s.ad_value(577)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) {
            s.store_scale_ad(1923, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        s.v[1942] = 0.0;

        s.v[1943] = 0.0;

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1946] = 0.0;

        s.v[1947] = 0.0;

        s.v[1948] = 0.0;

        s.v[1949] = 0.0;

        s.v[1950] = 0.0;

        s.v[1951] = 0.0;

        s.v[1952] = 0.0;

        s.v[1953] = 0.0;

        s.v[1954] = 0.0;

        s.v[1955] = 0.0;

        s.v[1956] = 0.0;

        s.v[1957] = 0.0;

        s.v[1958] = 0.0;

        s.v[1959] = 0.0;

        s.v[2811] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[2811] != 0.0) {
            s.store_scalar(1988, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1992, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1986, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1987, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1969, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1970, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1971, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1972, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1973, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1974, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1975, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1976, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1977, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1960, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1961, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1962, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1963, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1964, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1965, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1966, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1967, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1968, 0.0);
        }

        s.v[2812] = if (s.v[1890] > 0.0) { 1.0 } else { 0.0 };

        s.v[2813] = if (s.v[1] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2814] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (s.v[2814] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2815] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (s.v[2815] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2816] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (!(s.v[2815] != 0.0))) && (s.v[2816] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (!(s.v[2815] != 0.0))) && (!(s.v[2816] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2817] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (s.v[2817] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        s.v[2818] = if (s.v[1] == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2819] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2819] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2820] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (s.v[2820] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2821] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (!(s.v[2820] != 0.0))) && (s.v[2821] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (!(s.v[2820] != 0.0))) && (!(s.v[2821] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2822] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (s.v[2822] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2823] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2823] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2824] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (s.v[2824] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2825] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (!(s.v[2824] != 0.0))) && (s.v[2825] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (!(s.v[2824] != 0.0))) && (!(s.v[2825] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2826] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (s.v[2826] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        s.v[2827] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(1942, 1943);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(1943, 2027);
        }

        s.v[2828] = if (s.v[1] == 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.25, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2829] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2829] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2830] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (s.v[2830] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2831] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (!(s.v[2830] != 0.0))) && (s.v[2831] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (!(s.v[2830] != 0.0))) && (!(s.v[2831] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2832] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (s.v[2832] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2833] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2833] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2834] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (s.v[2834] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2835] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (!(s.v[2834] != 0.0))) && (s.v[2835] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (!(s.v[2834] != 0.0))) && (!(s.v[2835] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2836] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (s.v[2836] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.75, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2837] = if (((s.v[1962]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2837] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.v[2838] = if ((((-s.v[1962])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (s.v[2838] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1962)));
        }

        s.v[2839] = if ((-s.v[1962]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (!(s.v[2838] != 0.0))) && (s.v[2839] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (!(s.v[2838] != 0.0))) && (!(s.v[2839] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0))));
        }

        s.v[2840] = if (s.v[1962] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (s.v[2840] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        s.v[2841] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(1942, 1944);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(1944, 2027);
        }

        s.v[2842] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.16666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2843] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2843] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2844] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (s.v[2844] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2845] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (!(s.v[2844] != 0.0))) && (s.v[2845] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (!(s.v[2844] != 0.0))) && (!(s.v[2845] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2846] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (s.v[2846] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2847] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2847] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2848] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (s.v[2848] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2849] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (!(s.v[2848] != 0.0))) && (s.v[2849] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (!(s.v[2848] != 0.0))) && (!(s.v[2849] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2850] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (s.v[2850] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2851] = if (((s.v[1962]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2851] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.v[2852] = if ((((-s.v[1962])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (s.v[2852] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1962)));
        }

        s.v[2853] = if ((-s.v[1962]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (!(s.v[2852] != 0.0))) && (s.v[2853] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (!(s.v[2852] != 0.0))) && (!(s.v[2853] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0))));
        }

        s.v[2854] = if (s.v[1962] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (s.v[2854] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1963, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2855] = if (((s.v[1963]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2855] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1963), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1963), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1963), 0.16666666666666666)))));
        }

        s.v[2856] = if ((((-s.v[1963])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (s.v[2856] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1963)));
        }

        s.v[2857] = if ((-s.v[1963]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (!(s.v[2856] != 0.0))) && (s.v[2857] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (!(s.v[2856] != 0.0))) && (!(s.v[2857] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0))));
        }

        s.v[2858] = if (s.v[1963] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (s.v[2858] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1945, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1963))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1964, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.8333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2859] = if (((s.v[1964]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2859] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1964), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1964), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1964), 0.16666666666666666)))));
        }

        s.v[2860] = if ((((-s.v[1964])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (s.v[2860] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1964)));
        }

        s.v[2861] = if ((-s.v[1964]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (!(s.v[2860] != 0.0))) && (s.v[2861] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (!(s.v[2860] != 0.0))) && (!(s.v[2861] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0))));
        }

        s.v[2862] = if (s.v[1964] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (s.v[2862] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1946, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1964))), 1996);
        }

        s.v[2863] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1942, 1946);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1946, 2027);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(2027, 1943);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1943, 1945);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1945, 2027);
        }

        s.v[2864] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.1, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2865] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2865] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2866] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (s.v[2866] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2867] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (!(s.v[2866] != 0.0))) && (s.v[2867] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (!(s.v[2866] != 0.0))) && (!(s.v[2867] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2868] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (s.v[2868] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.2, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2869] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2869] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2870] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (s.v[2870] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2871] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (!(s.v[2870] != 0.0))) && (s.v[2871] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (!(s.v[2870] != 0.0))) && (!(s.v[2871] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2872] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

    }
}
