#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) && s.b[1790]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), 1190, 617);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) && (!s.b[1790])) {s.store_pow_abs_mul_base_indices(1191, 1190, 617, 546);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && (!s.b[1789])) {s.store_add_scaled_product_mixed_iai(1217, 614, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), 620, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1774])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(480, 674, 1218, 1.0, 675, 1219, 1.0, 676, 1220, 1.0);s.store_primal_add_scaled_products3_indices(695, 674, 564, 1.0, 675, 565, 1.0, 676, 566, 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 695, A::exp_scaled_input(s.ad_value(489), (s.v[372] * s.v[696])), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(485, 480, 1.0, 695, A::exp_scaled_input(s.ad_value(490), (s.v[372] * s.v[696])), (-1.0), (-1.0));}
        s.b[1791] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });s.b[1792] = ((s.v[479] > 0.0) && (s.v[480] > 0.0));s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });s.b[1793] = ((((((s.v[484] / s.v[479]) > 0.001) || ((s.v[485] / s.v[480]) > 0.001)) && (s.v[484] > 0.0)) && (s.v[485] > 0.0)) && (s.v[485] > s.v[484]));s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1792]) && s.b[1793]) {s.store_div(491, 484, 485);s.store_div_scaled_inputs(698, A::ln(s.ad_value(491)), s.v[371], A::sub(s.ad_value(489), s.ad_value(490)), 1.0);s.store_div_scaled_value_offset_denominator(697, s.ad_value(484), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(489), s.v[372], s.ad_value(698))), (-1.0), 1.0);}
        if ((s.b[1155] && s.b[1172]) && s.b[1791]) {s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(486), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, 697, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[372], s.ad_value(698))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(487), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, 697, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[372], s.ad_value(698))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(483, A::add_scaled_offset_product_rhs(s.ad_value(478), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(488), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, 697, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[372], s.ad_value(698))), (-1.0), (-1.0));}
        s.b[1794] = (((s.v[476] < 0.0) && (s.v[477] < 0.0)) && (s.v[478] < 0.0));s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });s.b[1795] = (((((((s.v[481] / s.v[476]) > 0.001) || ((s.v[482] / s.v[477]) > 0.001)) || ((s.v[483] / s.v[478]) > 0.001)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0)) && (s.v[483] < 0.0));s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) {s.store_div(491, 481, 482);s.store_div_scaled_inputs(492, A::ln(s.ad_value(491)), (-s.v[371]), A::sub(s.ad_value(486), s.ad_value(487)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(494, 487, 487, 1.0, 486, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) {s.store_scaled_mul_ad(495, A::offset(s.ad_value(491), (-1.0)), A::offset(A::pow(s.ad_value(491), s.ad_value(494)), (-1.0)), s.v[371]);s.store_primal_div_add_scaled_inputs_rhs_indices(494, 486, 486, 1.0, 487, -1.0);s.store_sub_mixed_ai(496, A::add_scaled_products(A::pow(s.ad_value(491), s.ad_value(494)), A::sub(s.ad_value(487), s.ad_value(486)), 1.0, s.ad_value(491), s.ad_value(486), 1.0), 487);s.store_div(493, 495, 496);s.store_add(700, 492, 493);}
        s.b[1796] = (((((s.v[488] * s.v[372]) * s.v[700])) as f64).abs() < 1e-6);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {
        (1.0,)
    } else {
        (s.v[694],)
    }
};
        s.store_scalar(694, t0);
        if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(699, 483, A::div_from_scalar(1.0, s.ad_value(488)), 1.0, 700, (0.5 * s.v[372]));s.store_div_scaled_product_indices(700, 483, 700, ((-0.5) * s.v[372]), 488, 1.0);}
        let (t1,) = {
    if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) {
        (0.0,)
    } else {
        (s.v[694],)
    }
};
        s.store_scalar(694, t1);
        if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) {s.store_div_scaled_value_offset_denominator(699, s.ad_value(483), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), (-s.v[372]), s.ad_value(700))), (-1.0), 1.0);}
        let (t8,) = {
    if (s.b[1155] && s.b[1172]) {
        let t2: f64 = (s.v[674] * s.v[582]);let t3: f64 = (s.v[675] * s.v[583]);let t4: f64 = (t2 + t3);let t5: f64 = (s.v[676] * s.v[584]);let t6: f64 = (t4 + t5);let t7: f64 = (s.v[554] * t6);
        (t7,)
    } else {
        (s.v[502],)
    }
};
        s.store_scalar(502, t8);s.b[1797] = ((s.v[674] * s.v[582]) <= s.v[502]);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1797]) {
        (0.0,)
    } else {
        (s.v[679],)
    }
};
        s.store_scalar(679, t9);s.b[1798] = ((s.v[675] * s.v[583]) <= s.v[502]);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1798]) {
        (0.0,)
    } else {
        (s.v[680],)
    }
};
        s.store_scalar(680, ta);s.b[1799] = ((s.v[676] * s.v[584]) <= s.v[502]);s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1799]) {
        (0.0,)
    } else {
        (s.v[681],)
    }
};
        s.store_scalar(681, tb);s.b[1800] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1800]) {s.store_primal_ln_ad(688, A::div_scalar_offset_denominator((0.5 * p.p839), s.ad_value(695), 1e-21, 1.0));s.store_ln_ad(690, A::div_scalar_offset_denominator((0.5 * p.p839), s.ad_value(697), 1e-21, 1.0));s.store_ln_ad(692, A::div_scalar_offset_denominator((0.5 * p.p839), A::abs(s.ad_value(699)), 1e-21, 1.0));}
        if (s.b[1155] && s.b[1172]) {s.store_primal_min_with_scalar(688, 688, 230.25850929940458);s.store_primal_exp(689, 688);s.store_min_with_scalar(690, 690, 230.25850929940458);s.store_exp(691, 690);s.store_min_with_scalar(692, 692, 230.25850929940458);s.store_exp(693, 692);}
        s.store_scalar(1929, 0.0);s.store_scalar(1930, 0.0);s.store_scalar(1931, 0.0);s.store_offset_voltage(357, ctx, nodes, Some(4), None, s.v[352]);s.store_square(358, 357);s.store_offset(359, 357, (-s.v[351]));s.store_div_from_scalar(360, s.v[351], 357);s.store_ln(361, 360);s.store_scale(1916, 357, (1.3806505e-23 * 6.241449993689894e18));s.store_div_from_scalar(362, 1.0, 1916);s.store_sub_scaled_inputs_mixed_ai(363, A::sub_from_scalar(1.179, A::scale(s.ad_value(357), 9.025e-5)), 1.0, 358, 3.05e-7);s.store_mul_ad_affine_product_lhs(364, A::scale_offset(s.ad_value(357), 0.00045, 1.045), A::sub_scaled_inputs(A::scale_offset(s.ad_value(357), 0.0014, 0.523), 1.0, s.ad_value(358), 1.48e-6), 1.1111111111111112e-5, 0.0, 358);
        if (!(s.v[364] > 0.001)) {s.store_scalar(364, 0.001);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(1919, 357, (4.0 * 1.3806505e-23));s.store_add_scaled_inputs_product_mixed_iiia(717, 363, 1.0, 185, 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(181), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);
        if (!(s.v[717] > 0.05)) {s.store_scalar(717, 0.05);}
        s.store_div_mixed_ai(718, A::sqrt(A::mul_scaled_lhs(s.ad_value(181), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.b[2004] = (s.v[186] > 0.0);s.store_scalar(2004, if s.b[2004] { 1.0 } else { 0.0 });
        if s.b[2004] {s.store_primal_div_from_scalar(721, 80000000.0, 759);}
        if s.b[2004] {
            if (s.v[186] > s.v[721]) {
                s.copy_ad(720, 186);
            } else {
                s.copy_ad(720, 721);
            }
        }
        if s.b[2004] {
            if (5e24 > s.v[720]) {
                s.store_scalar(720, 5e24);
            } else {
            }
        }
        if s.b[2004] {s.store_div_scaled_product3_indices(719, 758, 758, 1916, 2.0, 720, (1.6021918e-19 * s.v[756]));}
        s.store_scaled_mul(722, 1916, 1916, 100.0);s.b[2005] = (p.p51 > 0.0);s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });
        if s.b[2005] {s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(718), s.ad_value(718)), s.ad_value(717));s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);s.store_add(717, 717, 724);s.store_mul_scale_offset_mixed_ia(718, 718, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0, 1.0);}
        s.store_sqrt(725, 717);s.store_scale(726, 717, 0.95);s.store_scaled_mul(727, 717, 717, 0.0025);s.copy_ad(728, 727);s.store_scaled_sqrt(729, 728, 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(730, 726, 0.5, 729, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(726), s.ad_value(729))), s.ad_value(727)), (-0.5));s.store_scaled_add(731, 717, 363, 0.5);s.store_sub_mixed_ai(732, A::sqrt(A::add(s.ad_value(183), s.ad_value(717))), 725);s.store_add_scaled_inputs3_sqrt_first_mixed_aii(733, A::add_scaled_inputs3(s.ad_value(183), 1.0, s.ad_value(184), 1.0, s.ad_value(717), 1.0), 1.0, 725, (-1.0), 732, -1.0);s.store_add_scaled_product_mixed_aia(734, A::add_scaled_inputs3(s.ad_value(363), 1.0, s.ad_value(185), 1.0, s.ad_value(254), 1.0), 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(761), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);
        if (!(s.v[734] > 0.05)) {s.store_scalar(734, 0.05);}
        s.store_div_mixed_ai(735, A::sqrt(A::mul_scaled_lhs(s.ad_value(761), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);s.b[2006] = (p.p51 > 0.0);s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });
        if s.b[2006] {s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(735), s.ad_value(735)), s.ad_value(734));s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);s.store_add(734, 734, 724);s.store_mul_scale_offset_mixed_ia(735, 735, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0, 1.0);}
        s.store_scale(736, 734, 0.95);s.store_scaled_mul(737, 734, 734, 0.0025);s.copy_ad(738, 737);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scaled_sqrt(729, 738, 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(739, 736, 0.5, 729, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(736), s.ad_value(729))), s.ad_value(737)), (-0.5));s.store_offset_add_ad(701, s.ad_value(175), A::mul3(s.ad_value(176), s.ad_value(359), A::offset(A::mul(s.ad_value(177), s.ad_value(359)), 1.0)), s.v[17]);s.store_exp_mul(740, 178, 361);s.store_mul(702, 187, 740);s.store_div(703, 188, 360);s.store_exp_mul(741, 201, 361);s.store_mul(704, 200, 741);s.store_scaled_mul(1917, 704, 758, s.v[16]);s.store_mul_exp_mixed_ia(706, 204, A::mul(s.ad_value(205), s.ad_value(361)));s.store_exp_mul(742, 203, 361);s.store_mul(705, 202, 742);s.store_mul_exp_mixed_ia(708, 208, A::mul(s.ad_value(209), s.ad_value(361)));s.store_exp_mul(743, 207, 361);s.store_mul(707, 206, 743);s.store_exp_mul(744, 211, 361);s.store_mul(709, 210, 744);s.store_exp_mul(745, 214, 361);s.store_mul(710, 213, 745);s.store_scaled_mul(746, 1917, 710, 2.0);s.store_exp_mul(747, 218, 361);s.store_mul(1921, 217, 747);s.store_mul(1922, 256, 747);s.store_mul_exp_mixed_ia(713, 228, A::mul_scaled_lhs(s.ad_value(229), -1.0, s.ad_value(361)));s.store_scaled_mul(1920, 274, 357, (4.0 * 1.3806505e-23));s.b[2007] = ((p.p46 != 0.0) && (s.v[285] > 0.0));s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });
        if s.b[2007] {s.store_offset_add_scaled_product_indices(714, 280, 1.0, 281, 359, 1.0, s.v[19]);s.store_exp_mul(748, 286, 361);s.store_mul(715, 285, 748);s.store_scaled_mul(1918, 715, 758, s.v[18]);s.store_mul_scale_offset_mixed_ia(1924, 1916, A::mul(s.ad_value(284), s.ad_value(360)), 1.0, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(749, 363, 1.0, 282, 1.0, 1924, A::ln_scaled_input(A::mul(s.ad_value(283), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);}
        if s.b[2007] {
            if (s.v[749] > 0.05) {
            } else {
                s.store_scalar(749, 0.05);
            }
        }
        if s.b[2007] {s.store_div_mixed_ai(750, A::sqrt(A::mul_scaled_lhs(s.ad_value(283), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);s.store_square(1925, 750);s.store_ln(1926, 1925);s.store_scale(751, 749, 0.95);s.store_scaled_mul(752, 749, 749, 0.0025);s.copy_ad(753, 752);s.store_scaled_sqrt(754, 753, 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(755, 751, 0.5, 754, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(751), s.ad_value(754))), s.ad_value(752)), (-0.5));}
        if (!s.b[2007]) {s.store_scalar(714, 0.0);s.store_scalar(748, 1.0);s.store_scalar(715, 0.0);s.store_scalar(1918, 0.0);s.copy_ad(1924, 1916);s.store_scalar(749, 0.0);s.store_scalar(750, 1.0);s.store_scalar(1925, 1.0);s.store_scalar(1926, 0.0);s.store_scalar(751, 0.0);s.store_scalar(752, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[2007]) {s.store_scalar(753, 0.0);s.store_scalar(754, 0.0);s.store_scalar(755, 0.0);}
        s.b[2008] = (s.v[0] == 1.0);s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });
        if s.b[2008] {s.store_voltage(814, ctx, nodes, Some(6), Some(7));s.store_voltage(815, ctx, nodes, Some(8), Some(7));s.store_voltage(816, ctx, nodes, Some(7), Some(9));s.store_scaled_voltage(821, ctx, nodes, Some(7), Some(11), -1.0);s.store_scaled_voltage(822, ctx, nodes, Some(8), Some(12), -1.0);}
        if (!s.b[2008]) {s.store_scaled_voltage(814, ctx, nodes, Some(6), Some(7), -1.0);s.store_scaled_voltage(815, ctx, nodes, Some(8), Some(7), -1.0);s.store_scaled_voltage(816, ctx, nodes, Some(7), Some(9), -1.0);s.store_voltage(821, ctx, nodes, Some(7), Some(11));s.store_voltage(822, ctx, nodes, Some(8), Some(12));}
        s.store_add(818, 814, 816);s.copy_ad(823, 814);s.copy_ad(824, 816);s.store_add(825, 815, 816);s.store_sub(826, 814, 815);s.store_scale(1801, 823, (-s.v[356]));s.store_scale(1802, 826, (-s.v[356]));s.store_scaled_sub(1803, 818, 701, (-s.v[356]));s.store_scalar(820, 1.0);s.b[2009] = (s.v[815] < 0.0);s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });
        if s.b[2009] {s.store_scalar(820, (-1.0));s.store_sub(814, 814, 815);s.store_add(816, 816, 815);s.store_neg(815, 815);}
        s.store_add(817, 815, 816);s.store_div_scaled_product_offset_denominator_mixed_iia(819, 815, 815, 1.0, A::sqrt_square_offset(s.ad_value(815), 0.01), 0.1, 1.0);s.store_add_scaled_inputs4_mixed_iiai(2013, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(728))), (-0.5), 726, 1.0);s.copy_ad(1804, 2013);s.store_add_scaled_inputs4_mixed_iiai(1932, 816, 1.0, 2013, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2013)), s.ad_value(727))), (-(-0.5)), 730, 1.0);s.copy_ad(1805, 1932);s.store_scalar(1933, 0.0);s.b[2169] = ((p.p45 != 0.0) && (s.v[182] != 1.0));s.store_scalar(2169, if s.b[2169] { 1.0 } else { 0.0 });
        if s.b[2169] {s.store_add_scaled_inputs3_indices(1934, 1932, 1.0, 815, 0.5, 819, (-0.5));s.store_sub_mixed_ai(1935, A::sqrt(A::add(s.ad_value(1934), s.ad_value(717))), 725);s.store_offset_div_scaled_inputs2_indices(1929, 1935, 2.0, 732, (-2.0), 733, 1.0, (-1.0));s.store_add_scaled_product_mixed_iaa(1936, 1935, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(182), s.ad_value(733), 0.25), A::add(s.ad_value(1929), A::sqrt_square_offset(s.ad_value(1929), 0.4804530139182)), (-1.0));s.store_add_scaled_square_product_indices(1937, 1936, 1.0, 725, 1936, 2.0);s.store_add_scaled_inputs3_indices(1932, 1937, 1.0, 815, (-0.5), 819, (-(-0.5)));s.store_sub(1933, 1805, 1932);}
        s.copy_ad(2010, 717);s.copy_ad(2011, 727);s.copy_ad(2012, 718);s.copy_ad(2014, 1932);s.copy_ad(2018, 1933);s.copy_ad(2015, 1921);s.copy_ad(2016, 766);s.store_add_scaled_inputs3_indices(2017, 818, 1.0, 2018, (-1.0), 701, -1.0);s.store_add_scaled_inputs3_indices(2019, 2014, 1.0, 815, 0.5, 819, (-0.5));s.store_scalar(2031, 1.0);s.b[2170] = (s.v[188] > 0.0);s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });
        if s.b[2170] {s.store_mul(2022, 2010, 362);s.store_mul(2023, 2019, 362);s.store_mul(2024, 2017, 362);s.store_offset_div_scaled_inputs_sqrt_rhs(1930, 2012, 0.5, 2022, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(1931, 2022, 1.0, 2012, A::sqrt(s.ad_value(2022)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2170] {s.store_add_scaled_inputs_product_mixed_aiai(2025, A::div_scaled_inputs2(s.ad_value(2024), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2022, 0.5, A::offset(s.ad_value(189), 1.0), 2023, (-1.0));s.store_offset_scaled(2026, 2022, 0.5, 2.0);s.store_add(2027, 2022, 2023);s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2024), 1.0, s.ad_value(2027), (-1.0), s.ad_value(2012), A::sqrt(s.ad_value(2027)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2022), s.ad_value(2012)), A::sqrt(s.ad_value(2022)))), 2.0);s.store_add_scaled_inputs(2028, 1930, 2.0, 2026, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2025, 0.5, 2028, 0.5, 2025, 2028, 20.0, 0.5);s.store_add_scaled_inputs3_indices(1931, 2024, 2.0, 2023, (-2.0), 2026, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2029, 1930, 0.5, 1931, 0.5, 1930, 1931, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2029, 0.5, 2026, 0.5, 2029, 2026, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2030, 1930, 0.5, 2026, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(1931, 703, A::div(s.ad_value(2030), s.ad_value(2026)), 1.0, 1.0);}
        s.b[2171] = (s.v[1931] > (-230.25850929940458));s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });
        if (s.b[2170] && s.b[2171]) {s.store_exp(2031, 1931);}
        if (s.b[2170] && (!s.b[2171])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2031, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_offset_mul(2032, 702, 2031, 1.0);s.store_mul(2033, 1916, 2032);s.store_mul_ad_product_rhs(2034, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2019)), 1.0));s.store_mul_scale_offset_indices(2035, 2033, 2034, 1.0, 1.0);s.store_div_from_scalar(2036, 1.0, 2035);s.store_mul_sqrt_mixed_ia(2020, 2012, A::mul(s.ad_value(1916), s.ad_value(2036)));s.store_square(2021, 2020);s.store_div_from_scalar(2037, 1.0, 2021);s.store_mul(2038, 2014, 2036);s.store_mul(2039, 2017, 2036);s.store_div_scaled_value_offset_denominator(2040, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(195), s.ad_value(819), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2041, 194, 2040, A::offset(A::mul(s.ad_value(196), s.ad_value(2019)), 1.0));s.store_mul(2042, 2010, 2036);s.store_sqrt_square_add(1930, 2013, 2011);s.store_sqrt_add_ad(1931, A::square(A::sub(s.ad_value(2013), s.ad_value(2041))), s.ad_value(2011));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2043, 2036, 2041, 0.5, 1930, 0.5, 1931, ((-1.0) * (0.5)), 0.0);s.store_add(2044, 2042, 2038);s.store_sub(2045, 2044, 2043);s.b[2172] = (p.p45 > 0.0);s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });s.b[2173] = (((s.v[2045]) as f64).abs() < 1e-5);s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
    ) {
        if (s.b[2172] && s.b[2173]) {s.store_offset_ad(2046, A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.3125), 0.5)), 1.0);}
        s.b[2174] = (s.v[2045] < 460.51701859880916);s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });
        if ((s.b[2172] && (!s.b[2173])) && s.b[2174]) {s.store_exp_neg_input(2060, 2045);}
        if ((s.b[2172] && (!s.b[2173])) && (!s.b[2174])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2172] && (!s.b[2173])) {s.store_scalar(1929, (if (s.v[2045] > 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[2172] && (!s.b[2173])) {s.store_offset_ad(2046, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2020), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2045))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2045), 1.0, s.ad_value(2060))), 2.0), 1.0);}
        if (!s.b[2172]) {s.store_offset_div_scaled_inputs_sqrt_rhs(2046, 2020, 0.5, 2045, 1.0, 1.0);}
        s.store_add_scaled_value_products_mixed_iiaia(2047, 2045, 1.0, 2020, A::sqrt(s.ad_value(2045)), 1.0, 2046, A::ln(A::offset(s.ad_value(2046), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2048, 2039, 1.0, 2047, (-1.0), 2046, 1.0);s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2021)), 1.0)), (-1.0));s.store_scalar(2053, 0.0);s.store_scalar(2055, 1.0);s.b[2175] = (s.v[2048] > (-30.0));s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });
        if s.b[2175] {s.store_offset_mul(2049, 2046, 2048, (-1.0));s.store_scaled_add_mixed_ia(1929, 2049, A::sqrt_square_offset(s.ad_value(2049), 10.0), 0.5);s.store_sub_mixed_ia(2050, 2048, A::ln(s.ad_value(1929)));s.store_scaled_add_mixed_ia(2051, 2050, A::sqrt_square_offset(s.ad_value(2050), 2.0), 0.5);}
        s.b[2176] = ((s.v[2048] - s.v[2051]) < 230.25850929940458);s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });
        if (s.b[2175] && s.b[2176]) {s.store_exp_sub(1929, 2048, 2051);}
        if (s.b[2175] && (!s.b[2176])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if s.b[2175] {s.store_div(2052, 1929, 2046);s.store_sub_mixed_ai(1929, A::scaled_offset(s.ad_value(2051), 1.0, 2.0), 2052);}
        s.b[2177] = (s.v[2052] > 1e-6);s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });
        if (s.b[2175] && s.b[2177]) {s.store_mul_scale_offset_mixed_ia(2053, 2046, A::sub(s.ad_value(2051), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2052), s.ad_value(1929), 1.0), 1.0, (-1.0), s.ad_value(2052), 1.0)), 1.0, 1.0);}
        if (s.b[2175] && (!s.b[2177])) {s.store_mul_ad_affine_product_rhs(2053, 2046, s.ad_value(2052), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);}
        if s.b[2175] {s.store_add_scaled_inputs3_offset_mixed_iia(1929, 2039, 0.5, 2053, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
    ) {
        if s.b[2175] {s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2021)), s.ad_value(1929), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2055, 2054, 2054, 1.0, 2053, 1.0);s.store_add_scaled_product_indices(2045, 2044, 1.0, 2055, 2043, (-1.0));}
        s.store_offset_scaled(2056, 2020, 0.7071067811865475, 1.0);let tc: f64 = (1e-5 * s.v[2056]);s.store_scalar(2057, tc);s.store_div_from_scalar(2058, 1.0, 2056);s.store_scalar(2165, 0.0);s.store_scalar(2059, 0.0);s.b[2178] = (s.v[2045] < 460.51701859880916);s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });
        if s.b[2178] {s.store_exp_neg_input(2060, 2045);}
        if (!s.b[2178]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2179] = (((s.v[2039]) as f64).abs() <= s.v[2057]);s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });
        if s.b[2179] {s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2059, 2039, 2058, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2060)), s.ad_value(2020), s.ad_value(2145)), 1.0));}
        s.b[2180] = (s.v[2039] < (-s.v[2057]));s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });
        if ((!s.b[2179]) && s.b[2180]) {s.store_neg(2147, 2039);s.store_scaled_mul(2148, 2147, 2058, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2149, 2148, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2144, 2147, 2149);s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::offset(s.ad_value(2149), 1.0), 1.0);s.store_sub_scaled_inputs(2151, 2144, 2.0, 2021, 1.0);s.store_sub_ln_mul_lhs(2152, 2150, 2037, 2149);s.store_add(813, 2150, 2151);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.5, s.ad_value(2150), 1.0), 1.0);s.store_add_mixed_ia(2153, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.3333333333333333, s.ad_value(2150), 1.0))), 1.0));}
        s.b[2181] = (s.v[2153] < 230.25850929940458);s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });
        if (((!s.b[2179]) && s.b[2180]) && s.b[2181]) {s.store_exp(2154, 2153);}
        if (((!s.b[2179]) && s.b[2180]) && (!s.b[2181])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2154, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((!s.b[2179]) && s.b[2180]) {s.store_div_from_scalar(2155, 1.0, 2154);s.store_div_from_scalar_offset_square(2144, 1.0, 2153, 2.0);s.store_mul_square_lhs(2156, 2153, 2144);s.store_mul3_affine_lhs(2157, 2153, 2144, 4.0, 0.0, 2144);s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);s.store_sub(2144, 2147, 2153);s.store_mul(2145, 2060, 2155);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
    ) {
        if ((!s.b[2179]) && s.b[2180]) {s.store_add_scaled_product_mixed_iia(2159, 2144, 2.0, 2021, A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2145), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2157)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2153), (-1.0), s.ad_value(2145), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::sub(A::offset(s.ad_value(2153), (-1.0)), s.ad_value(2156)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2154), 1.0, s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2059, 2153, -1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);}
        if ((!s.b[2179]) && (!s.b[2180])) {s.store_div_from_scalar_offset_scaled_input(2161, 1.0, 2020, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2162, 2161, A::mul_scaled_lhs(s.ad_value(2056), 1.25, s.ad_value(2161)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2163, 2039, 2058, A::offset(A::mul(s.ad_value(2162), s.ad_value(2039)), 1.0));}
        s.b[2182] = ((-s.v[2163]) > (-230.25850929940458));s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });
        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2182]) {s.store_exp_neg_input(2144, 2163);}
        if (((!s.b[2179]) && (!s.b[2180])) && (!s.b[2182])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2144, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2163)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[2179]) && (!s.b[2180])) {s.store_sub_from_scalar(2164, 1.0, 2144);s.store_add_scaled_inputs_product_mixed_iiia(2165, 2039, 1.0, 2021, 0.5, 2020, A::sqrt(A::add_scaled_inputs3(s.ad_value(2039), 1.0, s.ad_value(2021), 0.25, s.ad_value(2164), -1.0)), (-1.0));s.store_offset(2166, 2045, 3.0);s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2165), s.ad_value(2166)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt_square_offset(s.ad_value(2166), 5.0), 0.5));s.store_sub(2144, 2039, 2149);s.store_exp_neg_input(2145, 2149);s.store_div_from_scalar_offset_square(2146, 1.0, 2149, 2.0);s.store_mul_square_lhs(2156, 2149, 2146);s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), 2146, 2146);}
        if ((!s.b[2179]) && (!s.b[2180])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2060] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
    ) {
        if ((!s.b[2179]) && (!s.b[2180])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2167, 1.0, 2021, A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2152, 2045, 1.0, 2149, (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);s.store_add(813, 2150, 2151);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);s.store_add_mixed_ia(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));}
        s.b[2183] = (s.v[2168] < 230.25850929940458);s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });
        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2183]) {s.store_exp(2154, 2168);s.store_div_from_scalar(2155, 1.0, 2154);s.store_mul(2154, 2060, 2154);}
        s.b[2184] = (s.v[2168] > (s.v[2045] - 230.25850929940458));s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });
        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && s.b[2184]) {s.store_exp_sub(2154, 2168, 2045);s.store_div(2155, 2060, 2154);}
        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && (!s.b[2184])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((!s.b[2179]) && (!s.b[2180])) {s.store_div_from_scalar_offset_square(2144, 1.0, 2168, 2.0);s.store_mul_square_lhs(2156, 2168, 2144);s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);s.store_sub(2144, 2039, 2168);s.store_add_scaled_product_mixed_iia(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));s.store_add_scaled_inputs_mixed_ia(2059, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
    ) {
        s.store_scalar(2062, 0.0);s.store_scalar(2063, 0.0);s.store_scalar(2064, 0.0);s.store_scalar(2065, 0.0);s.store_scalar(2066, 0.0);s.store_scalar(2067, 0.0);s.store_scalar(2068, 0.0);s.store_scalar(2069, 1.0);s.store_scalar(2070, 1.0);s.store_sub(2071, 2039, 2059);s.store_scalar(2072, 0.0);s.store_mul(2073, 2035, 2071);s.store_scalar(2074, 1.0);s.store_scalar(2075, 1.0);s.store_scalar(2079, 1.0);s.store_scalar(2080, 1.0);s.store_scalar(2082, 1.0);s.b[2185] = (s.v[2039] > 0.0);s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });
        if s.b[2185] {s.store_div_from_scalar_offset_square(1929, 1.0, 2059, 2.0);s.store_mul_square_lhs(2061, 2059, 1929);s.store_mul3_affine_lhs(2062, 2059, 1929, 4.0, 0.0, 1929);s.store_mul_ad_product_lhs_mixed_ai(2063, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2061), 12.0), 1929, 1929);s.store_scalar(2064, 0.0);}
        s.b[2186] = (s.v[2059] < 230.25850929940458);s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });
        if (s.b[2185] && s.b[2186]) {s.store_exp(2064, 2059);s.store_div_from_scalar(2065, 1.0, 2064);s.store_mul(2064, 2060, 2064);}
        s.b[2187] = (s.v[2059] > (s.v[2045] - 230.25850929940458));s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });
        if ((s.b[2185] && (!s.b[2186])) && s.b[2187]) {s.store_exp_sub(2064, 2059, 2045);s.store_div(2065, 2060, 2064);}
        if ((s.b[2185] && (!s.b[2186])) && (!s.b[2187])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2064, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2065, 1e-100, 2059, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2185] {s.store_add_scaled_product_mixed_iia(2066, 2064, 1.0, 2060, A::add(A::offset(s.ad_value(2059), 1.0), s.ad_value(2061)), (-1.0));}
        s.b[2188] = (s.v[2059] < 1e-5);s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });
        if (s.b[2185] && s.b[2188]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2067, 2059, 1.0, 2059, 1.0, 2059, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2066, A::mul3(s.ad_value(2060), s.ad_value(2059), s.ad_value(2059)), 2059, A::scale_offset(s.ad_value(2059), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2059), 1.0, A::scale(s.ad_value(2059), 0.25), 0.3333333333333333));s.store_scaled_mul(2068, 2059, 1929, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2069, 2020, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.5)), 1.0, A::square(s.ad_value(2059)), 0.16666666666666666), 0.7071067811865475, 1929, 1.0, 1.0);}
        if (s.b[2185] && (!s.b[2188])) {s.store_add_offset_lhs(2067, 2059, (-1.0), 2065);s.store_sqrt(2068, 2067);s.store_offset_scaled_ad(2069, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2065)), s.ad_value(2068)), 0.5, 1.0);}
        if s.b[2185] {s.store_div_scaled_offset_numerator(2070, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2019)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2019)), 1.0), 1.0);}
        s.b[2189] = (s.v[2066] > 1e-100);s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });
        if (s.b[2185] && s.b[2189]) {s.store_mul_sqrt_mixed_ia(2071, 2020, A::add(s.ad_value(2067), s.ad_value(2066)));s.store_div_scaled_product3_mixed_iiia(2072, 2021, 2066, 2035, 1.0, A::add_scaled_product(s.ad_value(2071), 1.0, s.ad_value(2020), s.ad_value(2068), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
    ) {
        if (s.b[2185] && s.b[2189]) {s.store_mul3_lhs(2073, 2068, 2020, 2035);}
        s.b[2190] = (s.v[215] < 0.0);s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });
        if ((s.b[2185] && s.b[2189]) && s.b[2190]) {s.store_div_from_scalar_sub_from_scalar_ad(2074, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2019)));}
        if ((s.b[2185] && s.b[2189]) && (!s.b[2190])) {s.store_offset_mul(2074, 215, 2019, 1.0);}
        s.b[2191] = (s.v[216] < 0.0);s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });
        if ((s.b[2185] && s.b[2189]) && s.b[2191]) {s.store_sub_from_scalar_scaled_mul(2075, 1.0, 216, 2072, 1.0);}
        if ((s.b[2185] && s.b[2189]) && (!s.b[2191])) {s.store_div_from_scalar_offset_product(2075, 1.0, 216, 2072, 1.0);}
        if (s.b[2185] && s.b[2189]) {s.store_mul_product3_indices(2076, 2072, 746, 2074, 2075, 1.0);s.store_mul_add_scaled_product_rhs_indices(2077, 763, 2073, 1.0, 764, 2072, 1.0);s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2067), 1.0, A::add(s.ad_value(2067), s.ad_value(2066)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2077), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);s.store_mul_add_mixed_iai(2079, 2070, A::offset(s.ad_value(2078), 1.0), 2076);}
        s.b[2192] = (s.v[219] < 0.0);s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });
        if ((s.b[2185] && s.b[2189]) && s.b[2192]) {s.store_div_from_scalar_sub_from_scalar_ad(2080, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2019)));}
        if ((s.b[2185] && s.b[2189]) && (!s.b[2192])) {s.store_offset_mul(2080, 219, 2019, 1.0);}
        if (s.b[2185] && s.b[2189]) {s.store_mul(1931, 2072, 2080);s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);}
        s.b[2193] = (s.v[220] < 0.0);s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });
        if ((s.b[2185] && s.b[2189]) && s.b[2193]) {s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));}
        if ((s.b[2185] && s.b[2189]) && (!s.b[2193])) {s.store_offset_mul(2082, 220, 2081, 1.0);}
        s.copy_ad(1806, 2017);s.copy_ad(1807, 2019);s.copy_ad(1808, 2035);s.copy_ad(1809, 2036);s.copy_ad(1810, 2020);s.copy_ad(1811, 2021);s.copy_ad(1812, 2037);s.copy_ad(1813, 2039);s.copy_ad(1814, 2044);s.copy_ad(1815, 2045);s.copy_ad(1816, 2056);s.store_scalar(1817, s.v[2057]);s.copy_ad(1818, 2058);s.copy_ad(1819, 2165);s.copy_ad(1820, 2060);s.copy_ad(1821, 2059);s.copy_ad(1822, 2062);s.copy_ad(1823, 2063);s.copy_ad(1824, 2064);s.copy_ad(1825, 2065);s.copy_ad(1826, 2067);s.copy_ad(1827, 2066);s.copy_ad(1828, 2068);s.copy_ad(1829, 2069);s.copy_ad(1830, 2070);s.copy_ad(1831, 2071);s.copy_ad(1832, 2072);s.copy_ad(1833, 2073);s.copy_ad(1834, 2074);s.copy_ad(1835, 2075);s.copy_ad(1836, 2079);s.copy_ad(1837, 2080);s.copy_ad(1838, 2082);s.store_scalar(2084, 0.0);s.store_scale(2083, 2035, 4.60517018598809);s.copy_ad(2100, 2083);s.copy_ad(2101, 815);s.store_mul(2102, 815, 2036);s.copy_ad(2106, 2059);s.store_scalar(2107, 0.0);s.store_scalar(2110, 0.0);s.copy_ad(2112, 2065);s.copy_ad(2113, 2067);s.copy_ad(2115, 2066);s.copy_ad(2116, 2073);s.copy_ad(2117, 2059);s.copy_ad(2118, 2065);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
    ) {
        s.copy_ad(2120, 2066);s.copy_ad(2121, 2067);s.store_sub(2122, 2039, 2059);s.store_scalar(2123, 1.0);s.store_scalar(2125, 1.0);s.store_scalar(2124, 0.0);s.copy_ad(2134, 2072);s.store_mul(2138, 2122, 2035);s.store_scalar(2135, 0.0);s.copy_ad(2136, 2073);s.store_scalar(2141, 0.0);s.store_scalar(2140, 1.0);s.copy_ad(2143, 2015);s.copy_ad(2142, 2138);s.b[2194] = (s.v[2039] > 0.0);s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });s.b[2195] = (s.v[2066] > 1e-100);s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2195]) {s.store_mul(2143, 2015, 2082);s.store_div(2084, 2143, 2079);s.store_add_scaled_inputs(2085, 2071, 1.0, 2021, 0.5);s.store_div_scaled_product_by_product_indices(1929, 2021, 2064, 1.0, 2085, 2085, 1.0);}
        s.b[2196] = (s.v[1929] > 0.0001);s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });
        if ((s.b[2194] && s.b[2195]) && s.b[2196]) {s.store_sub_from_scalar(1930, 1.0, 1929);}
        s.b[2197] = (s.v[1930] < 1e-10);s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });
        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && s.b[2197]) {s.store_scalar(1931, 1.0);}
        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && (!s.b[2197])) {s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));}
        if ((s.b[2194] && s.b[2195]) && (!s.b[2196])) {s.store_scale(1931, 1929, 0.5);}
        if (s.b[2194] && s.b[2195]) {s.store_mul(2086, 1931, 2085);}
        s.b[2198] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });
        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {s.store_scaled_mul(2087, 2035, 2086, 0.475);s.store_add_scaled_product_indices(1929, 2072, 1.0, 2069, 2087, (-1.0));s.store_scaled_add_mixed_ia(2088, 1929, A::sqrt_square_offset(s.ad_value(1929), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2089, 2072, (-1.0), 2035, 2071, 1.0, A::offset(s.ad_value(2069), (-1.0)), 2087, 1.0);s.store_offset_div_scaled_product_indices(2090, 2021, 2035, 0.5, 2089, 1.0, 1.0);s.store_add_scaled_product_indices(1929, 2089, 1.0, 764, 2088, 1.0);s.store_pow_ad(2091, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));s.store_mul_mixed_ai(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2090), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2091);s.store_div(1929, 2088, 2089);s.store_mul_pow_mixed_iaa(2092, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));s.store_mul_div_scaled_product_mixed_iiai(1931, 2092, 708, A::add(A::offset(s.ad_value(2090), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, 2089, 1.0);s.store_mul_product3_indices(2093, 2088, 746, 2074, 2075, 1.0);s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), s.ad_value(2090), (-1.0), s.ad_value(1931), 1.0), 1.0);}
        s.b[2199] = (s.v[1929] < 230.25850929940458);s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });
        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && s.b[2199]) {s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);}
        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && (!s.b[2199])) {s.copy_ad(1930, 1929);}
        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {s.store_div_scaled_product3_mixed_iiia(2094, 2087, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2091), 1.0, s.ad_value(2092), 1.0, s.ad_value(2093), 1.0, 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
    ) {
        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {s.store_mul_scale_offset_mixed_ia(2095, 2086, A::div_scaled_value_offset_denominator(s.ad_value(2094), 1.0, A::sqrt_square_offset(s.ad_value(2094), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if ((s.b[2194] && s.b[2195]) && (!s.b[2198])) {s.copy_ad(2095, 2086);}
        if (s.b[2194] && s.b[2195]) {s.store_mul3_affine_lhs(2096, 2035, 2084, 0.7071067811865475, 0.0, 2095);}
        s.b[2200] = (s.v[0] == (-1.0));s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });
        if ((s.b[2194] && s.b[2195]) && s.b[2200]) {s.store_div_mixed_ia(2096, 2096, A::sqrt(A::offset(s.ad_value(2096), 1.0)));}
        if (s.b[2194] && s.b[2195]) {s.store_div_from_scalar_offset_ad(2097, 2.0, A::sqrt(A::scale_offset(s.ad_value(2096), 4.0, 1.0)), 1.0);s.store_mul(1929, 2097, 2096);s.store_mul_ad_product_rhs_mixed_ia(2098, 2095, 2097, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2097)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2097), 4.0), 1.0)), 1.0));s.store_scale(2099, 2098, 0.99);s.store_div_scaled_product3_mixed_iaii(1929, 2099, A::sub_scaled_inputs(s.ad_value(2099), 1.0, s.ad_value(2085), 2.0), 2037, 1.0, 2066, 1.0);}
        if (s.b[2194] && s.b[2195]) {
            s.store_mul_sub_mixed_iia(2100, 2035, 2099, A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if (s.b[2194] && (!s.b[2195])) {s.copy_ad(2100, 2083);}
        if s.b[2194] {s.store_offset(1929, 2016, 1.0);s.store_div_scaled_product_mixed_aii(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2100, 1.0);s.store_add_mixed_ai(1931, A::square(s.ad_value(1930)), 1929);s.store_scale(1929, 1930, 2.0);s.store_div_scaled_product_add_scaled_denominator(2101, 2100, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);s.store_mul(2102, 2101, 2036);s.store_add(2103, 2045, 2102);}
        s.b[2201] = (s.v[2102] < 460.51701859880916);s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2201]) {s.store_exp_neg_input(2104, 2102);}
        if (s.b[2194] && (!s.b[2201])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2104, 1e-200, 2102, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2194] {s.store_mul(2105, 2060, 2104);}
        s.b[2202] = (((s.v[2039]) as f64).abs() <= s.v[2057]);s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2202]) {s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2106, 2039, 2058, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2105)), s.ad_value(2020), s.ad_value(2145)), 1.0));}
        if (s.b[2194] && (!s.b[2202])) {s.store_offset(2166, 2103, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
    ) {
        if (s.b[2194] && (!s.b[2202])) {s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2165), s.ad_value(2166)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt_square_offset(s.ad_value(2166), 5.0), 0.5));s.store_sub(2144, 2039, 2149);s.store_exp_neg_input(2145, 2149);s.store_div_from_scalar_offset_square(2146, 1.0, 2149, 2.0);s.store_mul_square_lhs(2156, 2149, 2146);s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), 2146, 2146);}
        if (s.b[2194] && (!s.b[2202])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2105] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }
        if (s.b[2194] && (!s.b[2202])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2167, 1.0, 2021, A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2152, 2103, 1.0, 2149, (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);s.store_add(813, 2150, 2151);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);s.store_add_mixed_ia(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));}
        s.b[2203] = (s.v[2168] < 230.25850929940458);s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });
        if ((s.b[2194] && (!s.b[2202])) && s.b[2203]) {s.store_exp(2154, 2168);s.store_div_from_scalar(2155, 1.0, 2154);s.store_mul(2154, 2105, 2154);}
        s.b[2204] = (s.v[2168] > (s.v[2103] - 230.25850929940458));s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });
        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {s.store_exp_sub(2154, 2168, 2103);s.store_div(2155, 2105, 2154);}
        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && (!s.b[2204])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2194] && (!s.b[2202])) {s.store_div_from_scalar_offset_square(2144, 1.0, 2168, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
    ) {
        if (s.b[2194] && (!s.b[2202])) {s.store_mul_square_lhs(2156, 2168, 2144);s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);s.store_sub(2144, 2039, 2168);s.store_add_scaled_product_mixed_iia(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));s.store_add_scaled_inputs_mixed_ia(2106, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);}
        if s.b[2194] {s.store_sub(2107, 2106, 2059);}
        s.b[2205] = (s.v[2107] < 1e-10);s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2205]) {s.store_add_scaled_inputs_product_mixed_iiia(2108, 2039, 2.0, 2059, (-2.0), 2021, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0), 1.0, s.ad_value(2105), s.ad_value(2062), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2109, A::mul_sub_from_scalar_rhs(s.ad_value(2021), 1.0, s.ad_value(2104)), 2066);s.store_sub_from_scalar_scaled_mul_mixed_ia(1929, 2.0, 2021, A::add_scaled_value_products(s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0, s.ad_value(2105), s.ad_value(2063), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(1929, 2108, 1.0, 1929, 2109, (-2.0));s.store_scaled_div_mixed_ia(2107, 2109, A::add(s.ad_value(2108), A::sqrt(s.ad_value(1929))), 2.0);s.store_add(2106, 2059, 2107);}
        if s.b[2194] {s.store_mul(2110, 2107, 2035);s.store_div_scaled_product_offset_denominator_mixed_iia(2111, 2106, 2106, 1.0, A::square(s.ad_value(2106)), 2.0, 1.0);}
        s.b[2206] = (s.v[2106] < 230.25850929940458);s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2206]) {s.store_exp_neg_input(2112, 2106);}
        s.b[2207] = (s.v[2106] < 1e-5);s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });
        if ((s.b[2194] && s.b[2206]) && s.b[2207]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2113, 2106, 1.0, 2106, 1.0, 2106, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2106), 1.0, A::scale(s.ad_value(2106), 0.25), 0.3333333333333333));s.store_scaled_mul(2114, 2106, 1929, 0.7071067811865475);s.store_mul3_ad_middle(2115, A::mul3_scaled_output(s.ad_value(2105), s.ad_value(2106), s.ad_value(2106), 0.16666666666666666), 2106, A::scale_offset(s.ad_value(2106), 1.75, 1.0));}
        if ((s.b[2194] && s.b[2206]) && (!s.b[2207])) {s.store_add_offset_lhs(2113, 2106, (-1.0), 2112);s.store_sqrt(2114, 2113);}
    }
}
