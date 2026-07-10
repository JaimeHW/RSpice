#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {s.store_mul_scale_offset_indices(329, 474, 735, 1.0, s.v[94]);s.store_square(329, 329);s.store_offset_scaled(332, 328, (-1.6), 0.6);s.store_scalar(331, 0.5);s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));}
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(330, 329, 331, 226);s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));}
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));}
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {s.store_sqrt(329, 329);s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);s.store_scaled_sub(380, 330, 329, 0.5);s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));}
        s.b[795] = (s.v[380] < s.v[382]);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && s.b[795]) {s.copy_ad(351, 380);}
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {s.store_offset_sub(44, 381, 380, (-0.0008));s.store_scale(45, 381, (4.0 * 0.0008));}
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[733] && (!s.b[782])) {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t78: usize = 0;
        while {
            let t77: f64 = if ((s.b[733] && (!s.b[782])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t77 != 0.0
        } {
            t78 += 1;assert!(t78 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[733] && (!s.b[782])) {s.copy_ad(328, 474);s.store_mul(329, 225, 351);s.store_exp_neg_input(330, 329);}
            s.b[796] = (s.v[351] > 1e-9);s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[796]) {s.store_exp_mul(327, 225, 351);s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));}
            s.b[797] = (s.v[351] < (-1e-9));s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && s.b[797]) {s.store_mul_sqrt_mixed_ia(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));s.store_mul_scale_offset_mixed_ai(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 330, -1.0, 1.0);}
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && (!s.b[797])) {s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);}
            if (s.b[733] && (!s.b[782])) {s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[798] = (s.v[333] < 0.0);s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[798]) {s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);}
            if (s.b[733] && (!s.b[782])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if (s.b[733] && (!s.b[782])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[733] && (!s.b[782])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(334, 334, 332, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);s.store_sub_mixed_ia(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0));s.copy_ad(334, 167);}
            s.b[799] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[799]) {s.store_scalar(167, s.v[57]);}
            if (s.b[733] && (!s.b[782])) {s.copy_ad(351, 333);s.copy_ad(357, 331);s.store_primal_offset(167, 167, 1.0);}
        }
        if (s.b[733] && (!s.b[782])) {s.store_add(351, 475, 351);s.store_add_scaled_product_mixed_iia(350, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
        s.b[800] = ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2)));s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[733] && s.b[800]) {s.store_scalar(446, s.v[136]);s.store_add_scaled_inputs4_indices(445, 174, 1.0, 446, (-1.0), 185, 1.0, 320, -1.0);s.store_scalar(143, p.p137);s.copy_ad(207, 445);s.store_sqrt_div_scaled_inputs(208, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10), 225, 1.0);s.store_div_scaled_product_by_product_indices(209, 230, 230, 1.0, 544, 544, 1.0);s.store_div_scaled_product_by_product_indices(210, 208, 208, 1.0, 323, 323, 1.0);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_scalar_by_product_indices(223, 1.0, 209, 210, 1.0);s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));s.store_add_scaled_inputs3_indices(217, 216, 1.0, 215, (-1.0), 143, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(218, 216, 1.0, 217, (-0.5), A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0), (-0.5));s.store_exp_mul(224, 225, 218);s.store_add_scaled_product_mixed_aii(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);s.store_offset_mul(220, 225, 218, (-1.0));}
        s.b[801] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_div_scaled_inputs_indices(214, 105, 2.0, 225, 1.0);s.store_scalar(250, (300.0 * 0.0001));s.store_scalar(316, 0.0);s.store_neg_ad(328, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(173))), (-1.0)));s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));s.store_mul_ad_product_lhs_mixed_ai(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 328, 329);s.copy_ad(394, 222);s.copy_ad(395, 218);s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        s.b[802] = (s.v[336] < (10.0 * 2.220446049250313e-16));s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[802]) {s.store_scalar(336, (10.0 * 2.220446049250313e-16));}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.copy_ad(163, 376);s.store_sub(166, 376, 395);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[803] = (s.v[166] < 0.0);s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[803]) {s.store_scalar(166, 0.0);}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_scale(332, 166, (1.0 + 0.3));s.store_offset_sub(333, 332, 173, (-0.03));s.store_sqrt_add_scaled_square_input(334, 333, 1.0, 332, (4.0 * 0.03));s.store_add_scaled_inputs3_indices(165, 332, 1.0, 333, (-0.5), 334, (-0.5));}
        s.b[804] = (s.v[165] > s.v[166]);s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[804]) {s.copy_ad(165, 166);}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.copy_ad(449, 165);s.store_scalar(822, (s.v[88] * 100.0));s.store_primal_scale(823, 107, 100.0);s.store_scalar(824, (s.v[97] * 100.0));}
        s.b[825] = (p.p36 == 0.0);s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[825]) {s.store_scalar(447, 0.0);}
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_scalar(448, 4.12);s.store_primal_scaled_mul(805, 823, 824, (p.p142 * 1.6021918e-19));s.store_div(806, 805, 302);s.store_div_scaled_inputs_mixed_ai(807, A::offset(A::add_scaled_inputs4(s.ad_value(514), p.p145, s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(237), 1.0), p.p144), -1.0, 822, 1.0);s.store_scalar(562, 0.0);}
        let mut t2: usize = 0;
        while {
            let t0: f64 = (100.0 - 1.0);let t1: f64 = if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (s.v[562] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.copy_ad(808, 562);s.store_scalar(809, 100.0);s.store_primal_div(810, 808, 809);s.store_add_scaled_inputs3_mixed_iia(811, 159, 1.0, 175, 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(810), 1.0), -1.0);s.store_sub_from_scalar_div_indices(812, 1.0, 811, 448);s.store_add_div_rhs_indices(815, 807, 811, 822);s.store_square(813, 815);s.store_sqrt_square_offset(44, 812, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(812, 812, 0.5, 44, 0.5, (1e-10 * 0.001));}
            s.b[826] = (s.v[812] < 0.0);s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[826]) {s.store_scalar(812, 0.0);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_offset_scaled_ad(814, A::mul(A::sqrt(s.ad_value(812)), s.ad_value(812)), (-p.p143), p.p143);s.store_div_scaled_inputs_indices(816, 814, -1.0, 815, 1.0);}
            s.b[827] = (s.v[816] < (-34.0));s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[827]) {s.store_scalar(818, 0.0);}
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[827])) {s.store_exp(818, 816);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.copy_ad(819, 806);s.store_mul3_affine_lhs(820, 819, 814, (0.25 * 7.38905609893065), 0.0, 814);}
            s.b[828] = (((2.0 * s.v[815]) + s.v[814]) < 0.0);s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[828]) {s.copy_ad(450, 820);}
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) {s.copy_ad(817, 805);s.store_mul3_lhs(821, 817, 813, 818);}
            s.b[829] = ((s.v[821] < s.v[820]) || (s.v[815] < 0.0));s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && s.b[829]) {s.copy_ad(450, 820);}
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && (!s.b[829])) {s.copy_ad(450, 821);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_add(447, 447, 450);}
            s.b[830] = (s.v[450] < 1e-9);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[830]) {s.store_scalar(562, 100.0);s.store_scalar(167, s.v[57]);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_primal_offset(562, 562, 1.0);}
        }
        s.b[843] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[843]) {s.store_scalar(263, 0.0);}
        s.b[844] = (p.p44 <= 0.0);s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {s.copy_ad(831, 445);s.store_square(838, 323);s.copy_ad(839, 545);s.store_div(833, 839, 838);s.store_div_from_scalar(840, 2.0, 839);s.store_mul(834, 840, 838);s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(837, 834, 835, 1.0);s.store_sqrt_square_offset(44, 837, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(836, 837, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[845] = (s.v[836] < 0.0);s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[845]) {s.store_scalar(836, 0.0);}
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {s.store_offset(836, 836, 1e-50);s.store_sqrt(836, 836);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {s.store_add_scaled_product_mixed_aii(841, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(836)), 1.0, 831, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(842, 173, p.p122, 395, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(841)), -1.0);s.store_sqrt_square_offset(44, 842, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(842, 842, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[846] = (s.v[842] < 0.0);s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[846]) {s.store_scalar(842, 0.0);}
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {s.store_mul(831, 134, 445);s.store_div_square_rhs(833, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(834, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(836, 834, 835, 1.0);s.store_scaled_offset(838, 834, 1.0, 2.0);}
        s.b[847] = ((s.v[836] < (1e-50 + s.v[838])) && (s.v[838] >= 0.0));s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_sub_offset_lhs(44, 838, 1e-50, 836);s.store_square(49, 44);s.store_square(50, 838);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t3,) = {
    if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3);
        let (t4,) = {
    if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4);
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[848] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });s.b[849] = (4.0 == 1.0);s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && s.b[849]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5);s.b[850] = (4.0 == 2.0);s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
        let (t6,) = {
    if ((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && s.b[850]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6);s.b[851] = (4.0 == 4.0);s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && s.b[851]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t7);s.b[852] = (4.0 == 8.0);s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
        let (t8,) = {
    if ((((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && (!s.b[851])) && s.b[852]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t8);
        let (t9,) = {
    if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t9);let mut td: usize = 0;
        while {
            let tc: f64 = if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {s.store_sqrt(53, 53);}
            let (tb,) = {
    if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
        let ta: f64 = (s.v[54] + 1.0);
        (ta,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, tb);
        }
        if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && (!s.b[848])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 838, 53);s.store_sub_offset_lhs(836, 838, 1e-50, 43);}
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && (!s.b[847])) {
        }
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {
            if (s.v[836] <= 0.0) {
                s.store_scalar(836, 0.0);
            } else {
                s.store_sqrt(836, 836);
            }
        }
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {s.store_add_mul_sub_from_scalar_rhs_indices(841, 831, 833, 1.0, 836);s.store_div_from_scalar_offset_input(832, s.v[100], 131, s.v[100]);s.store_add_scaled_product_mixed_aii(842, A::scale_offset(s.ad_value(173), p.p122, s.v[176]), 1.0, 832, 841, (-1.0));s.store_sqrt_square_offset(44, 842, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(842, 842, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[853] = (s.v[842] < 0.0);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[853]) {s.store_scalar(842, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) {s.store_offset(842, 842, 1e-50);s.store_ad_value(832, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(842), 1.0));s.store_mul_product3_indices(263, 832, 132, 842, 394, 1.0);}
        s.b[861] = (p.p26 == 1.0);s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) {s.store_mul_ad_affine_product_rhs(854, 736, s.ad_value(107), A::exp_scaled_input(s.ad_value(225), (-p.p141)), 1.6021918e-19, 0.0);s.store_offset_scaled(855, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(856, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 854, 855, 1.0);s.store_mul_add_lhs(567, 263, 447, 856);s.store_mul_scaled_ln_offset_rhs(857, 227, p.p140, 567, 1.0);s.store_sqrt_mul_scaled_lhs(858, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_sqrt_ad(859, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(395), s.ad_value(857)))), (-1.0)), 1.0, s.ad_value(225), A::sub(s.ad_value(395), s.ad_value(857)), 1.0));s.store_sqrt_ad(860, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(395))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(395), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(393, 858, 859, -1.0, 860, -1.0);}
        if ((((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) && (p.p37 != 0.0)) {s.store_div_from_scalar_offset_input(398, p.p138, 263, p.p139);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[861])) {s.store_scalar(393, 0.0);}
        if ((s.b[733] && s.b[800]) && (!s.b[801])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if (s.b[733] && (!s.b[800])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if s.b[733] {s.copy_ad(343, 349);s.copy_ad(344, 350);s.copy_ad(345, 351);}
        let (te,) = {
    if s.b[733] {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, te);
        if s.b[733] {s.store_scalar(611, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (s.b[733] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;assert!(t2c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[733] {s.store_sub(863, 351, 475);s.store_mul(862, 225, 863);s.store_exp_neg_input(327, 862);}
            s.b[897] = (s.v[863] < (-1e-9));s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[897]) {s.store_mul_sqrt_mixed_ia(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)));s.store_div_scaled_offset_numerator_indices(869, 327, (-s.v[122]), s.v[122], 357, 1.0);}
            s.b[898] = (s.v[863] > 1e-9);s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[897])) && s.b[898]) {s.store_exp(864, 862);s.store_mul_scaled_sqrt_ad_rhs(357, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(864), s.ad_value(862)), (-1.0), 1.0));s.store_div_mixed_ai(869, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(864), 1.0), s.v[122]), 357);}
            if ((s.b[733] && (!s.b[897])) && (!s.b[898])) {s.store_mul_scale_offset_indices(357, 862, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(869, 225, 474, -1.0, 0.0);}
            if s.b[733] {s.copy_ad(361, 369);s.store_mul(862, 225, 349);s.store_exp_mul(867, 225, 349);s.store_scalar(865, 1.0);s.store_sqrt_ad(866, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(867), 1.0, s.ad_value(862), 1.0, s.ad_value(865), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(896, 225, 379, A::offset(s.ad_value(867), 1.0), 2.0, 866, 2.0);s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 866, -1.0);s.store_mul_scale_offset_indices(868, 896, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(863, 350, 1.0, 349, (-1.0), 738, 1.0);s.store_mul(862, 225, 863);}
            s.b[899] = ((-s.v[862]) >= 500.0);s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[899]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(862)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if (s.b[733] && (!s.b[899])) {s.store_neg(44, 862);s.store_scalar(327, 1.0);}
            let mut t1c: usize = 0;
            while {
                let t1b: f64 = if ((s.b[733] && (!s.b[899])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t1b != 0.0
            } {
                t1c += 1;assert!(t1c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (s.b[733] && (!s.b[899])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if (s.b[733] && (!s.b[899])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if s.b[733] {s.store_exp_neg_input(327, 862);s.store_sqrt_offset_ad(864, A::add(s.ad_value(327), s.ad_value(862)), (-1.0));}
            s.b[900] = (s.v[863] < (-1e-9));s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[900]) {s.store_mul(363, 238, 864);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 864, 738, 2.0);s.store_neg(365, 364);}
            s.b[901] = (s.v[863] > 1e-9);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[900])) && s.b[901]) {s.store_mul_scale_offset_indices(363, 864, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 864, 738, 2.0);s.store_neg(365, 364);}
            if ((s.b[733] && (!s.b[900])) && (!s.b[901])) {s.store_scaled_mul(363, 238, 862, (-0.7071067811865476));s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));s.store_neg(365, 364);}
            s.b[902] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[902]) {s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t1d,) = {
    if (s.b[733] && s.b[902]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t1d);
            let (t1e,) = {
    if (s.b[733] && s.b[902]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1e);
            if (s.b[733] && s.b[902]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[903] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });s.b[904] = (2.0 == 1.0);s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
            let (t1f,) = {
    if (((s.b[733] && s.b[902]) && s.b[903]) && s.b[904]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1f);s.b[905] = (2.0 == 2.0);s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
            let (t20,) = {
    if ((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && s.b[905]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t20);s.b[906] = (2.0 == 4.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
            let (t21,) = {
    if (((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && s.b[906]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t21);s.b[907] = (2.0 == 8.0);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
            let (t22,) = {
    if ((((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && (!s.b[906])) && s.b[907]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t22);
            let (t23,) = {
    if ((s.b[733] && s.b[902]) && s.b[903]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t23);let mut t27: usize = 0;
            while {
                let t26: f64 = if (((s.b[733] && s.b[902]) && s.b[903]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t26 != 0.0
            } {
                t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && s.b[902]) && s.b[903]) {s.store_sqrt(53, 53);}
                let (t25,) = {
    if ((s.b[733] && s.b[902]) && s.b[903]) {
        let t24: f64 = (s.v[54] + 1.0);
        (t24,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t25);
            }
            if ((s.b[733] && s.b[902]) && (!s.b[903])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[733] && s.b[902]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(895, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(363, A::neg(s.ad_value(406)), -1.0, 895, 1.0);}
            if (s.b[733] && s.b[902]) {
            }
            if (s.b[733] && (!s.b[902])) {
            }
            if (s.b[733] && (!s.b[902])) {s.store_scalar(327, 1.0);}
            if s.b[733] {s.store_mul(364, 364, 327);s.store_mul(365, 365, 327);}
            s.b[908] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[908]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 363);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t28,) = {
    if (s.b[733] && s.b[908]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t28);
            let (t29,) = {
    if (s.b[733] && s.b[908]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t29);
            if (s.b[733] && s.b[908]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[909] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });s.b[910] = (2.0 == 1.0);s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
            let (t2a,) = {
    if (((s.b[733] && s.b[908]) && s.b[909]) && s.b[910]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t2a);s.b[911] = (2.0 == 2.0);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
            let (tf,) = {
    if ((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && s.b[911]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, tf);s.b[912] = (2.0 == 4.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
            let (t10,) = {
    if (((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && s.b[912]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t10);s.b[913] = (2.0 == 8.0);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
            let (t11,) = {
    if ((((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t11);
            let (t12,) = {
    if ((s.b[733] && s.b[908]) && s.b[909]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t12);let mut t16: usize = 0;
            while {
                let t15: f64 = if (((s.b[733] && s.b[908]) && s.b[909]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t15 != 0.0
            } {
                t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && s.b[908]) && s.b[909]) {s.store_sqrt(53, 53);}
                let (t14,) = {
    if ((s.b[733] && s.b[908]) && s.b[909]) {
        let t13: f64 = (s.v[54] + 1.0);
        (t13,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t14);
            }
            if ((s.b[733] && s.b[908]) && (!s.b[909])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[733] && s.b[908]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(895, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(363, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 895);}
            if (s.b[733] && s.b[908]) {
            }
            if (s.b[733] && (!s.b[908])) {
            }
            if (s.b[733] && (!s.b[908])) {s.store_scalar(327, 1.0);}
            if s.b[733] {s.store_mul(365, 365, 327);s.store_mul(364, 364, 327);s.store_add(356, 361, 363);}
            s.b[914] = (s.v[430] == 1.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[914]) {s.copy_ad(611, 167);s.store_scalar(167, s.v[57]);}
            if (s.b[733] && (!s.b[914])) {s.store_add_scaled_inputs_product_mixed_iiia(873, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(874, 1.0, 324, A::add(s.ad_value(868), s.ad_value(365)), 1.0);s.store_mul_scale_offset_indices(875, 364, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(876, 869, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(863, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);s.store_mul(865, 735, 869);s.store_sub(877, 350, 863);s.store_scalar(878, (-1.0));s.store_scalar(879, 1.0);s.store_neg(880, 865);s.store_add_scaled_inputs3_indices(881, 351, 1.0, 350, (-1.0), 357, (-s.v[94]));s.store_scalar(882, (-1.0));s.store_sub_from_scalar_scaled_input(883, 1.0, 869, s.v[94]);s.store_add_scaled_inputs4(884, A::mul3(s.ad_value(874), s.ad_value(879), s.ad_value(883)), 1.0, A::mul3(s.ad_value(874), s.ad_value(880), s.ad_value(882)), (-1.0), A::mul3(s.ad_value(875), s.ad_value(878), s.ad_value(883)), -1.0, A::mul3(s.ad_value(876), s.ad_value(878), s.ad_value(882)), 1.0);s.store_div_from_scalar_offset_input(885, 1.0, 884, 1e-50);s.store_add_scaled_products_indices(886, 879, 883, 1.0, 880, 882, (-1.0));s.store_add_scaled_products_indices(887, 876, 882, 1.0, 875, 883, (-1.0));s.store_add_scaled_products_indices(888, 875, 880, 1.0, 876, 879, (-1.0));s.store_mul_scale_offset_indices(889, 883, 878, -1.0, 0.0);s.store_mul(890, 874, 883);s.store_add_scaled_products_indices(891, 876, 878, 1.0, 874, 880, (-1.0));s.store_primal_mul(892, 878, 882);s.store_mul_scale_offset_indices(893, 882, 874, -1.0, 0.0);s.store_add_scaled_products_indices(894, 874, 879, 1.0, 875, 878, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(870, 885, 886, 873, -1.0, 887, 877, -1.0, 888, 881, -1.0);s.store_mul_add_scaled_products3_indices_rhs(871, 885, 889, 873, -1.0, 890, 877, -1.0, 891, 881, -1.0);s.store_mul_add_scaled_products3_indices_rhs(872, 885, 892, 873, -1.0, 893, 877, -1.0, 894, 881, -1.0);s.store_abs(863, 870);}
            s.b[915] = (s.v[863] < ((s.v[871]) as f64).abs());s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[915]) {s.store_abs(863, 871);}
            s.b[916] = (s.v[863] < ((s.v[872]) as f64).abs());s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[916]) {s.store_abs(863, 872);}
            if (s.b[733] && (!s.b[914])) {s.store_scalar(407, 1.0);}
            s.b[917] = (s.v[167] > 80.0);s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[917]) {s.store_scalar(407, 125.0);}
            s.b[918] = (s.v[167] > 40.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[914])) && (!s.b[917])) && s.b[918]) {s.store_scalar(407, 125.0);}
            s.b[919] = (s.v[167] > 20.0);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
            if ((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && s.b[919]) {s.store_scalar(407, 25.0);}
            s.b[920] = (s.v[167] > 10.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && (!s.b[919])) && s.b[920]) {s.store_scalar(407, 5.0);}
            s.b[921] = (s.v[863] > (0.1 / s.v[407]));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[921]) {s.store_mul_mixed_ia(870, 870, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));s.store_mul_mixed_ia(871, 871, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));s.store_mul_mixed_ia(872, 872, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));}
            if (s.b[733] && (!s.b[914])) {s.store_add(349, 349, 870);s.store_add(350, 350, 871);s.store_add(351, 351, 872);}
            let (t19,) = {
    if (s.b[733] && (!s.b[914])) {
        let t17: f64 = (5e-12 * s.v[407]);let t18: f64 = t17;
        (t18,)
    } else {
        (s.v[408],)
    }
};
            s.store_scalar(408, t19);s.b[922] = (s.v[863] < s.v[408]);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
            let (t1a,) = {
    if ((s.b[733] && (!s.b[914])) && s.b[922]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t1a);
            if s.b[733] {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if s.b[733] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }
        s.b[923] = (s.v[430] == 0.0);s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[923]) {s.copy_ad(349, 343);s.copy_ad(350, 344);s.copy_ad(351, 345);}
        if s.b[733] {s.copy_ad(161, 349);s.store_neg(244, 355);}
        s.b[924] = (s.v[244] <= 1e-50);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[924]) {s.store_scalar(244, 1e-50);}
        if s.b[733] {s.store_mul(192, 244, 324);}
        s.b[925] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[925]) {s.store_scale(327, 108, (-s.v[98]));s.copy_ad(362, 369);s.copy_ad(366, 363);s.store_add(359, 362, 366);s.store_scaled_add(437, 359, 356, (-0.5));s.store_mul(196, 327, 437);s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_scalar(197, 0.0);s.store_scaled_mul(392, 357, 108, s.v[98]);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);}
        let (t2d,) = {
    if (s.b[733] && s.b[925]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t2d);
        if (s.b[733] && s.b[925]) {s.copy_ad(352, 349);s.copy_ad(353, 350);s.copy_ad(354, 351);s.copy_ad(360, 357);s.copy_ad(162, 161);s.copy_ad(314, 162);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(453, 157);s.store_scalar(932, 1e-50);s.store_div_square_rhs(927, 545, 323);s.store_offset_mul_ad(929, A::div_from_scalar(2.0, s.ad_value(927)), A::sub(s.ad_value(159), s.ad_value(932)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(927), 1.0);}
        s.b[933] = ((s.v[929] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_sub(44, 332, 929);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2e,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[933]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2e);
        let (t2f,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[933]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2f);
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[934] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });s.b[935] = (4.0 == 1.0);s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        let (t30,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && s.b[935]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t30);s.b[936] = (4.0 == 2.0);s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });
        let (t31,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && s.b[936]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t31);s.b[937] = (4.0 == 4.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        let (t32,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && s.b[937]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t32);s.b[938] = (4.0 == 8.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });
        let (t33,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && (!s.b[937])) && s.b[938]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t33);
        let (t34,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t34);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        let mut t38: usize = 0;
        while {
            let t37: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t37 != 0.0
        } {
            t38 += 1;assert!(t38 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {s.store_sqrt(53, 53);}
            let (t36,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
        let t35: f64 = (s.v[54] + 1.0);
        (t35,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t36);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[933]) && (!s.b[934])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);s.store_sub(929, 332, 43);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[933])) {
        }
        if (s.b[733] && (!s.b[925])) {s.store_sqrt(928, 929);s.store_add_mul_sub_from_scalar_rhs_indices(932, 159, 927, 1.0, 928);s.store_sqrt_square_offset(44, 932, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(932, 932, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[939] = (s.v[932] < 0.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[939]) {s.store_scalar(932, 0.0);}
        if (s.b[733] && (!s.b[925])) {s.store_div(926, 157, 932);s.store_pow_offset_rhs(927, 926, 138, (-1.0));s.store_mul(931, 927, 926);s.store_offset(928, 931, 1.0);s.store_pow_ad(929, s.ad_value(928), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(930, 929, 928);s.store_div(452, 157, 930);s.copy_ad(157, 452);}
        s.b[940] = (s.v[157] < 0.0);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[940]) {s.copy_ad(162, 161);s.store_sub(164, 162, 161);s.copy_ad(352, 162);s.copy_ad(353, 350);s.copy_ad(354, 351);}
        let (t39,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[940]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t39);s.b[941] = (s.v[144] >= 1.0);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && s.b[941]) {s.store_scalar(352, s.v[622]);s.store_scalar(353, s.v[623]);s.store_scalar(354, s.v[624]);}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[942] = (s.v[165] < 0.0);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[942]) {s.store_scalar(165, 0.0);}
        s.b[943] = (s.v[165] > s.v[157]);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[942])) && s.b[943]) {s.copy_ad(165, 157);}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.copy_ad(164, 165);s.store_add(162, 349, 164);s.copy_ad(352, 162);s.copy_ad(388, 390);s.store_scaled_square(944, 474, (s.v[95] * s.v[95]));}
        s.b[950] = (s.v[352] < s.v[385]);s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_neg(945, 475);s.store_add_scaled_inputs3_mixed_aai(946, A::square(A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(945)), (-4.0), 944, (-4.0));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_sqrt(946, 946);s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);s.store_scaled_sub(948, 947, 946, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));}
        s.b[951] = (s.v[948] < s.v[382]);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && s.b[951]) {s.copy_ad(354, 948);}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {s.store_offset_sub(44, 949, 948, (-0.0008));s.store_scale(45, 949, (4.0 * 0.0008));}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 949, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {s.store_neg_ad(945, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));s.store_add_scaled_inputs3_mixed_aai(946, A::square(A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(945)), (-4.0), 944, (-4.0));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {s.store_sqrt(946, 946);s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);s.store_scaled_sub(948, 947, 946, 0.5);s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));}
        s.b[952] = (s.v[948] < s.v[382]);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && s.b[952]) {s.copy_ad(354, 948);}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {s.store_offset_sub(44, 949, 948, (-0.0008));s.store_scale(45, 949, (4.0 * 0.0008));}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 949, 1.0, 44, (-0.5), 45, (-0.5));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_div_scaled_inputs_indices(953, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[961] = (s.v[953] > 0.0);s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[961]) {s.store_sqrt_div_scaled_inputs(401, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[961])) {s.store_scalar(401, 0.0);}
        s.b[962] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));s.store_scalar(962, if s.b[962] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_scalar(168, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        let mut t3b: usize = 0;
        while {
            let t3a: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            t3a != 0.0
        } {
            t3b += 1;assert!(t3b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.copy_ad(954, 474);s.store_mul(955, 225, 354);s.store_exp_neg_input(956, 955);}
            s.b[963] = (s.v[354] > 1e-9);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[963]) {s.store_exp_mul(953, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(957, 954, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(958, s.v[122], 957, A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));}
            s.b[964] = (s.v[354] < (-1e-9));s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && s.b[964]) {s.store_mul_sqrt_mixed_ia(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));s.store_mul_scale_offset_mixed_ai(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 956, -1.0, 1.0);}
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && (!s.b[964])) {s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(958, 225, s.v[122], -1.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_sqrt_add_scaled_square_product(45, 957, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 957, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[965] = (s.v[959] < 0.0);s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[965]) {s.store_scalar(959, 0.0);s.store_scalar(960, 0.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 959, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(960, 960, 958, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);s.store_sub_mixed_ia(959, 354, A::div_scaled_inputs4(s.ad_value(957), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));}
            s.b[966] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[966]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.copy_ad(354, 959);s.copy_ad(360, 957);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_scalar(168, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        let mut t3d: usize = 0;
        while {
            let t3c: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            t3c != 0.0
        } {
            t3d += 1;assert!(t3d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.copy_ad(954, 474);s.store_mul(955, 225, 354);s.store_exp_neg_input(956, 955);}
            s.b[967] = (s.v[354] > 1e-9);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[967]) {s.store_exp_mul(953, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(957, 954, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(958, s.v[122], 957, A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));}
            s.b[968] = (s.v[354] < (-1e-9));s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && s.b[968]) {s.store_mul_sqrt_mixed_ia(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));s.store_mul_scale_offset_mixed_ai(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 956, -1.0, 1.0);}
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && (!s.b[968])) {s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(958, 225, s.v[122], -1.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_sqrt_add_scaled_square_product(45, 957, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 957, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[969] = (s.v[959] < 0.0);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[969]) {s.store_scalar(959, 0.0);s.store_scalar(960, 0.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 959, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(960, 960, 958, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);s.store_sub_mixed_ia(959, 354, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(957), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(957), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(958), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0));}
            s.b[970] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[970]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.copy_ad(354, 959);s.copy_ad(360, 957);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        s.b[971] = (s.v[353] < 0.0);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[971]) {s.store_scalar(353, 0.0);}
        s.b[1007] = (s.v[349] < 0.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1007]) {s.copy_ad(352, 349);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        s.b[1008] = (s.v[353] < 0.01);s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1008]) {s.store_add_scaled_product_mixed_iia(353, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(346, 352);s.copy_ad(347, 353);s.copy_ad(348, 354);}
        let (t3e,) = {
    if (s.b[733] && (!s.b[925])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t3e);
        if (s.b[733] && (!s.b[925])) {s.store_scalar(611, 0.0);s.store_scalar(168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        let mut t5c: usize = 0;
        while {
            let t5b: f64 = if ((s.b[733] && (!s.b[925])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            t5b != 0.0
        } {
            t5c += 1;assert!(t5c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[733] && (!s.b[925])) {s.store_sub(973, 354, 475);s.store_mul(972, 225, 973);s.store_exp_neg_input(327, 972);}
            s.b[1009] = (s.v[973] < (-1e-9));s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1009]) {s.store_mul_sqrt_mixed_ia(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)));s.store_div_scaled_offset_numerator_indices(979, 327, (-s.v[122]), s.v[122], 360, 1.0);}
            s.b[1010] = (s.v[973] > 1e-9);s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && s.b[1010]) {s.store_exp(974, 972);s.store_mul_scaled_sqrt_ad_rhs(360, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(974), s.ad_value(972)), (-1.0), 1.0));s.store_div_mixed_ai(979, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(974), 1.0), s.v[122]), 360);}
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && (!s.b[1010])) {s.store_mul_scale_offset_indices(360, 972, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(979, 225, 474, -1.0, 0.0);}
            if (s.b[733] && (!s.b[925])) {s.copy_ad(362, 369);s.store_exp_ad(977, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));s.store_scalar(975, 1.0);s.store_sqrt_ad(976, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(977), 1.0, s.ad_value(972), 1.0, s.ad_value(975), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(1006, 225, 379, A::offset(s.ad_value(977), 1.0), 2.0, 976, 2.0);s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 976, -1.0);s.store_mul_scale_offset_indices(978, 1006, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(973, 353, 1.0, 352, (-1.0), 738, 1.0);s.store_mul(972, 225, 973);}
            s.b[1011] = ((-s.v[972]) >= 500.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1011]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(972)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_neg(44, 972);s.store_scalar(327, 1.0);}
            let mut t4b: usize = 0;
            while {
                let t4a: f64 = if (((s.b[733] && (!s.b[925])) && (!s.b[1011])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t4a != 0.0
            } {
                t4b += 1;assert!(t4b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if (s.b[733] && (!s.b[925])) {s.store_sqrt_offset_ad(974, A::add(s.ad_value(327), s.ad_value(972)), (-1.0));}
            s.b[1012] = (s.v[973] < (-1e-9));s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1012]) {s.store_mul(366, 238, 974);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 974, 738, 2.0);s.store_neg(368, 367);}
            s.b[1013] = (s.v[973] > 1e-9);s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && s.b[1013]) {s.store_mul_scale_offset_indices(366, 974, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 974, 738, 2.0);s.store_neg(368, 367);}
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && (!s.b[1013])) {s.store_scaled_mul(366, 238, 972, (-0.7071067811865476));s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));s.store_neg(368, 367);}
            s.b[1014] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t4c,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t4c);
            let (t4d,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t4d);
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1015] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });s.b[1016] = (2.0 == 1.0);s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
            let (t4e,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && s.b[1016]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t4e);s.b[1017] = (2.0 == 2.0);s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
            let (t4f,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && s.b[1017]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t4f);s.b[1018] = (2.0 == 4.0);s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
            let (t50,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && s.b[1018]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t50);s.b[1019] = (2.0 == 8.0);s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
            let (t51,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && (!s.b[1018])) && s.b[1019]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t51);
            let (t52,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t52);let mut t56: usize = 0;
            while {
                let t55: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t55 != 0.0
            } {
                t56 += 1;assert!(t56 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {s.store_sqrt(53, 53);}
                let (t54,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
        let t53: f64 = (s.v[54] + 1.0);
        (t53,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t54);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1014]) && (!s.b[1015])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(1005, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(366, A::neg(s.ad_value(406)), -1.0, 1005, 1.0);}
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {s.store_scalar(327, 1.0);}
            if (s.b[733] && (!s.b[925])) {s.store_mul(367, 367, 327);s.store_mul(368, 368, 327);}
            s.b[1020] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 366);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t57,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t57);
            let (t58,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t58);
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1021] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });s.b[1022] = (2.0 == 1.0);s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
            let (t59,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && s.b[1022]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t59);s.b[1023] = (2.0 == 2.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
            let (t5a,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && s.b[1023]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t5a);s.b[1024] = (2.0 == 4.0);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
            let (t3f,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && s.b[1024]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t3f);s.b[1025] = (2.0 == 8.0);s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
            let (t40,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && (!s.b[1024])) && s.b[1025]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t40);
            let (t41,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t41);let mut t45: usize = 0;
            while {
                let t44: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t44 != 0.0
            } {
                t45 += 1;assert!(t45 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {s.store_sqrt(53, 53);}
                let (t43,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
        let t42: f64 = (s.v[54] + 1.0);
        (t42,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t43);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1020]) && (!s.b[1021])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(1005, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(366, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 1005);}
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {s.store_scalar(327, 1.0);}
            if (s.b[733] && (!s.b[925])) {s.store_mul(368, 368, 327);s.store_mul(367, 367, 327);s.store_add(359, 362, 366);}
            s.b[1026] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1026]) {s.copy_ad(611, 168);s.store_scalar(168, s.v[58]);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_add_scaled_inputs_product_mixed_iiia(983, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(984, 1.0, 324, A::add(s.ad_value(978), s.ad_value(368)), 1.0);s.store_mul_scale_offset_indices(985, 367, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(986, 979, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(973, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);s.store_mul(975, 735, 979);s.store_sub(987, 353, 973);s.store_scalar(988, (-1.0));s.store_scalar(989, 1.0);s.store_neg(990, 975);s.store_add_scaled_inputs3_indices(991, 354, 1.0, 353, (-1.0), 360, (-s.v[94]));s.store_scalar(992, (-1.0));s.store_sub_from_scalar_scaled_input(993, 1.0, 979, s.v[94]);s.store_add_scaled_inputs4(994, A::mul3(s.ad_value(984), s.ad_value(989), s.ad_value(993)), 1.0, A::mul3(s.ad_value(984), s.ad_value(990), s.ad_value(992)), (-1.0), A::mul3(s.ad_value(985), s.ad_value(988), s.ad_value(993)), -1.0, A::mul3(s.ad_value(986), s.ad_value(988), s.ad_value(992)), 1.0);s.store_div_from_scalar_offset_input(995, 1.0, 994, 1e-50);s.store_add_scaled_products_indices(996, 989, 993, 1.0, 990, 992, (-1.0));s.store_add_scaled_products_indices(997, 986, 992, 1.0, 985, 993, (-1.0));s.store_add_scaled_products_indices(998, 985, 990, 1.0, 986, 989, (-1.0));s.store_mul_scale_offset_indices(999, 993, 988, -1.0, 0.0);s.store_mul(1000, 984, 993);s.store_add_scaled_products_indices(1001, 986, 988, 1.0, 984, 990, (-1.0));s.store_primal_mul(1002, 988, 992);s.store_mul_scale_offset_indices(1003, 992, 984, -1.0, 0.0);s.store_add_scaled_products_indices(1004, 984, 989, 1.0, 985, 988, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(980, 995, 996, 983, -1.0, 997, 987, -1.0, 998, 991, -1.0);s.store_mul_add_scaled_products3_indices_rhs(981, 995, 999, 983, -1.0, 1000, 987, -1.0, 1001, 991, -1.0);s.store_mul_add_scaled_products3_indices_rhs(982, 995, 1002, 983, -1.0, 1003, 987, -1.0, 1004, 991, -1.0);s.store_abs(973, 980);}
            s.b[1027] = (s.v[973] < ((s.v[981]) as f64).abs());s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1027]) {s.store_abs(973, 981);}
            s.b[1028] = (s.v[973] < ((s.v[982]) as f64).abs());s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1028]) {s.store_abs(973, 982);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_scalar(407, 1.0);}
            s.b[1029] = (s.v[168] > 80.0);s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1029]) {s.store_scalar(407, 125.0);}
            s.b[1030] = (s.v[168] > 40.0);s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
            if ((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && s.b[1030]) {s.store_scalar(407, 125.0);}
            s.b[1031] = (s.v[168] > 20.0);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && s.b[1031]) {s.store_scalar(407, 25.0);}
            s.b[1032] = (s.v[168] > 10.0);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && (!s.b[1031])) && s.b[1032]) {s.store_scalar(407, 5.0);}
            s.b[1033] = (s.v[973] > (0.1 / s.v[407]));s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1033]) {s.store_mul_mixed_ia(980, 980, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));s.store_mul_mixed_ia(981, 981, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));s.store_mul_mixed_ia(982, 982, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_add(352, 352, 980);s.store_add(353, 353, 981);s.store_add(354, 354, 982);}
            let (t48,) = {
    if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {
        let t46: f64 = (5e-12 * s.v[407]);let t47: f64 = t46;
        (t47,)
    } else {
        (s.v[408],)
    }
};
            s.store_scalar(408, t48);s.b[1034] = (s.v[973] < s.v[408]);s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
            let (t49,) = {
    if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1034]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t49);
            if (s.b[733] && (!s.b[925])) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if (s.b[733] && (!s.b[925])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }
        s.b[1035] = (s.v[430] == 0.0);s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1035]) {s.copy_ad(352, 346);s.copy_ad(353, 347);s.copy_ad(354, 348);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(162, 352);s.copy_ad(157, 453);}
        s.b[1036] = (s.v[349] < 0.0);s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        let (t5d,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1036]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t5d);
        if (s.b[733] && (!s.b[925])) {s.copy_ad(374, 349);s.copy_ad(375, 352);s.store_sub(164, 375, 374);s.copy_ad(373, 351);s.store_scale(400, 401, 9662367879.197212);s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);}
        s.b[1037] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1037]) {s.store_scalar(246, 0.0);}
        if (s.b[733] && (!s.b[925])) {s.store_scaled_add(437, 359, 356, (-0.5));s.store_sub(411, 352, 349);s.store_offset(411, 411, 5e-12);s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);}
        s.b[1038] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t5e,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t5e);
        let (t5f,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5f);
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1039] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });s.b[1040] = (2.0 == 1.0);s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        let (t60,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && s.b[1040]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t60);s.b[1041] = (2.0 == 2.0);s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        let (t61,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && s.b[1041]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t61);s.b[1042] = (2.0 == 4.0);s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        let (t62,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && s.b[1042]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t62);s.b[1043] = (2.0 == 8.0);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        let (t63,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && (!s.b[1042])) && s.b[1043]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t63);
        let (t64,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t64);let mut t68: usize = 0;
        while {
            let t67: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t67 != 0.0
        } {
            t68 += 1;assert!(t68 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {s.store_sqrt(53, 53);}
            let (t66,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
        let t65: f64 = (s.v[54] + 1.0);
        (t65,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t66);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && (!s.b[1039])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[1038])) {s.store_neg(328, 409);}
        if (s.b[733] && (!s.b[925])) {s.store_neg(409, 328);}
        s.b[1044] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[925])) && s.b[1044]) {s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));}
        if (s.b[733] && (!s.b[925])) {s.store_sub(414, 355, 358);}
        s.b[1045] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t69,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t69);
        let (t6a,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6a);
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1046] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });s.b[1047] = (2.0 == 1.0);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        let (t6b,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && s.b[1047]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6b);s.b[1048] = (2.0 == 2.0);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        let (t6c,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6c);s.b[1049] = (2.0 == 4.0);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        let (t6d,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && s.b[1049]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6d);s.b[1050] = (2.0 == 8.0);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        let (t6e,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && (!s.b[1049])) && s.b[1050]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6e);
        let (t6f,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6f);let mut t73: usize = 0;
        while {
            let t72: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t72 != 0.0
        } {
            t73 += 1;assert!(t73 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {s.store_sqrt(53, 53);}
            let (t71,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
        let t70: f64 = (s.v[54] + 1.0);
        (t70,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t71);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && (!s.b[1046])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[1045])) {
        }
        if (s.b[733] && (!s.b[925])) {s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ai(328, A::square(s.ad_value(411)), 411, 411);s.store_mul(415, 412, 411);s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);}
        s.b[1051] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {s.store_sub_from_scalar(44, 1e-5, 413);s.store_square(49, 44);s.store_scalar(50, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t74,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t74);
        let (t75,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t75);
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });s.b[1053] = (2.0 == 1.0);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        let (t76,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && s.b[1053]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t76);s.b[1054] = (2.0 == 2.0);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
    }
}
