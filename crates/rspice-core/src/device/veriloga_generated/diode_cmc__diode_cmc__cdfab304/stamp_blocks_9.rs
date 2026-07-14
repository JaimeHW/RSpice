#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[787])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[788] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[788]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[789] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[788])) && s.b[789]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[788])) && (!s.b[789])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && (!s.b[772])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[790] = (s.v[63] < p.p85);s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[790])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[791] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[791]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[792] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[791])) && s.b[792]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[791])) && (!s.b[792])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && (!s.b[772])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(372, 283, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);}
        if (s.b[418] && s.b[771]) {s.store_offset(370, 370, (-1.0));s.store_offset(371, 371, (-1.0));s.store_offset(372, 372, (-1.0));s.store_primal_div_from_scalar(429, 1.0, 430);}
        s.b[793] = (s.v[194] > 0.0);s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });
        if ((s.b[418] && s.b[771]) && s.b[793]) {s.store_primal_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[418] && s.b[771]) && (!s.b[793])) {s.store_primal_sub_mixed_ai(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 194);}
        if (s.b[418] && s.b[771]) {s.store_primal_sub(432, 264, 431);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(433, 194, 0.5, 432, 0.5, 194, 432, ((4.0 * s.v[84]) * s.v[84]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(434, 194, 0.5, 267, 0.5, A::add_scaled_square_product(A::sub(s.ad_value(194), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0), (-0.5));s.store_primal_scaled_sub_mixed_ia(435, 194, A::sqrt_square_offset(s.ad_value(194), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        if (s.b[418] && (!s.b[771])) {s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(431, 0.0);s.store_scalar(428, 0.0);s.store_scalar(430, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);}
        s.b[794] = (s.v[256] == 0.0);s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[794]) {s.store_scalar(268, 0.0);s.store_scalar(291, 0.0);s.store_scalar(269, 0.0);}
        s.b[795] = (s.v[122] == 0.5);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[794])) && s.b[795]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));}
        if ((s.b[418] && (!s.b[794])) && (!s.b[795])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);}
        if (s.b[418] && (!s.b[794])) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(194), s.ad_value(428)), 1.0);s.store_mul(437, 101, 370);}
        s.b[796] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[794])) && s.b[796]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {s.store_primal_sub(439, 107, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[797] = (s.v[9] == 0.5);s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && s.b[797]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && (!s.b[797])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));}
        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {s.store_primal_add(442, 440, 441);}
        s.b[798] = (s.v[9] == 0.5);s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && s.b[798]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[143]);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && (!s.b[798])) {s.store_primal_powf_scaled_input(436, 439, s.v[143], s.v[9]);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {s.store_primal_scale(443, 436, s.v[137]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 98, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[20]);}
        s.b[799] = (s.v[23] == 0.0);s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[794])) && s.b[799]) {s.store_scalar(445, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[122] * s.v[152]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[800] = (((-s.v[9]) * s.v[125]) == (-1.0));s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[800]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[800])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));}
        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[149]), 447, 450, s.v[149], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[801] = (s.v[457] > 0.0);s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[801]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[801])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[802] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[802]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[802])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[803] = (s.v[457] > 0.0);s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[803]) {s.copy_ad(458, 421);}
        s.b[804] = (s.v[456] > (-230.25850929940458));s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) && s.b[804]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) && (!s.b[804])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[149] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);}
        s.b[805] = (s.v[29] == 0.0);s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[794])) && s.b[805]) {s.store_scalar(460, 0.0);}
        s.b[806] = (s.v[9] == 0.5);s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && s.b[806]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[806])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[143]), ((s.v[6]) * (s.v[143])), s.v[9]);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[805])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), 436, 1.0);}
        s.b[807] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && s.b[807]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));}
        s.b[808] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[807])) && s.b[808]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 155, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[807])) && (!s.b[808])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 155, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[794])) && (!s.b[805])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[29], 0.0);}
        s.b[809] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(809, if s.b[809] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[794])) && s.b[809]) {s.store_scalar(462, 1.0);}
        s.b[810] = (s.v[435] > ((-s.v[158]) * s.v[38]));s.store_scalar(810, if s.b[810] { 1.0 } else { 0.0 });s.b[811] = (s.v[41] == 4.0);s.store_scalar(811, if s.b[811] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) && s.b[811]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));}
        if ((((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) && (!s.b[811])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);}
        if (((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[794])) && (!s.b[809])) && (!s.b[810])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);}
        if (s.b[418] && (!s.b[794])) {s.store_mul_add_scaled_inputs4_indices_rhs(268, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(291, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[812] = (s.v[257] == 0.0);s.store_scalar(812, if s.b[812] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[812]) {s.store_scalar(270, 0.0);s.store_scalar(292, 0.0);s.store_scalar(271, 0.0);}
        s.b[813] = (s.v[123] == 0.5);s.store_scalar(813, if s.b[813] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[812])) && s.b[813]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));}
        if ((s.b[418] && (!s.b[812])) && (!s.b[813])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);}
        if (s.b[418] && (!s.b[812])) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(194), s.ad_value(428)), 1.0);s.store_mul(437, 102, 371);}
        s.b[814] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));s.store_scalar(814, if s.b[814] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[812])) && s.b[814]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {s.store_primal_sub(439, 108, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[815] = (s.v[10] == 0.5);s.store_scalar(815, if s.b[815] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && s.b[815]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && (!s.b[815])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));}
        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {s.store_primal_add(442, 440, 441);}
        s.b[816] = (s.v[10] == 0.5);s.store_scalar(816, if s.b[816] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && s.b[816]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[144]);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && (!s.b[816])) {s.store_primal_powf_scaled_input(436, 439, s.v[144], s.v[10]);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {s.store_primal_scale(443, 436, s.v[138]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 99, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[21]);}
        s.b[817] = (s.v[24] == 0.0);s.store_scalar(817, if s.b[817] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[812])) && s.b[817]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[123] * s.v[153]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {s.store_primal_mul(451, 449, 450);}
        s.b[818] = (((-s.v[10]) * s.v[126]) == (-1.0));s.store_scalar(818, if s.b[818] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[818]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[818])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));}
        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[150]), 447, 450, s.v[150], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[819] = (s.v[457] > 0.0);s.store_scalar(819, if s.b[819] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[819]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[819])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[820] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(820, if s.b[820] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[820]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[820])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[821] = (s.v[457] > 0.0);s.store_scalar(821, if s.b[821] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[821]) {s.copy_ad(458, 421);}
        s.b[822] = (s.v[456] > (-230.25850929940458));s.store_scalar(822, if s.b[822] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) && s.b[822]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) && (!s.b[822])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[150] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);}
        s.b[823] = (s.v[30] == 0.0);s.store_scalar(823, if s.b[823] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[812])) && s.b[823]) {s.store_scalar(460, 0.0);}
        s.b[824] = (s.v[10] == 0.5);s.store_scalar(824, if s.b[824] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && s.b[824]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[824])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[144]), ((s.v[7]) * (s.v[144])), s.v[10]);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[823])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), 436, 1.0);}
        s.b[825] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && s.b[825]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));}
        s.b[826] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[825])) && s.b[826]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 156, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[825])) && (!s.b[826])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 156, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[812])) && (!s.b[823])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[30], 0.0);}
        s.b[827] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[812])) && s.b[827]) {s.store_scalar(462, 1.0);}
        s.b[828] = (s.v[435] > ((-s.v[158]) * s.v[39]));s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });s.b[829] = (s.v[42] == 4.0);s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) && s.b[829]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) && (!s.b[829])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);}
        if (((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[812])) && (!s.b[827])) && (!s.b[828])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if (s.b[418] && (!s.b[812])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[830] = (s.v[258] == 0.0);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[830]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
        s.b[831] = (s.v[124] == 0.5);s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[830])) && s.b[831]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));}
        if ((s.b[418] && (!s.b[830])) && (!s.b[831])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);}
        if (s.b[418] && (!s.b[830])) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(194), s.ad_value(428)), 1.0);s.store_mul(437, 103, 372);}
        s.b[832] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));s.store_scalar(832, if s.b[832] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[830])) && s.b[832]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {s.store_primal_sub(439, 109, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[833] = (s.v[11] == 0.5);s.store_scalar(833, if s.b[833] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && s.b[833]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && (!s.b[833])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));}
        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {s.store_primal_add(442, 440, 441);}
        s.b[834] = (s.v[11] == 0.5);s.store_scalar(834, if s.b[834] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && s.b[834]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[145]);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && (!s.b[834])) {s.store_primal_powf_scaled_input(436, 439, s.v[145], s.v[11]);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {s.store_primal_scale(443, 436, s.v[139]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 100, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[22]);}
        s.b[835] = (s.v[25] == 0.0);s.store_scalar(835, if s.b[835] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[830])) && s.b[835]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[124] * s.v[154]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[836] = (((-s.v[11]) * s.v[127]) == (-1.0));s.store_scalar(836, if s.b[836] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[836]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[836])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));}
        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[151]), 447, 450, s.v[151], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[837] = (s.v[457] > 0.0);s.store_scalar(837, if s.b[837] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[837]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[837])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[838] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(838, if s.b[838] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[838]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[838])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[839] = (s.v[457] > 0.0);s.store_scalar(839, if s.b[839] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[839]) {s.copy_ad(458, 421);}
        s.b[840] = (s.v[456] > (-230.25850929940458));s.store_scalar(840, if s.b[840] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) && s.b[840]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) && (!s.b[840])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[151] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);}
        s.b[841] = (s.v[31] == 0.0);s.store_scalar(841, if s.b[841] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[830])) && s.b[841]) {s.store_scalar(460, 0.0);}
        s.b[842] = (s.v[11] == 0.5);s.store_scalar(842, if s.b[842] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && s.b[842]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[842])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[145]), ((s.v[8]) * (s.v[145])), s.v[11]);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[841])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), 436, 1.0);}
        s.b[843] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && s.b[843]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));}
        s.b[844] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[843])) && s.b[844]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 157, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[843])) && (!s.b[844])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 157, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[830])) && (!s.b[841])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[31], 0.0);}
        s.b[845] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[830])) && s.b[845]) {s.store_scalar(462, 1.0);}
        s.b[846] = (s.v[435] > ((-s.v[158]) * s.v[40]));s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });s.b[847] = (s.v[43] == 4.0);s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) && s.b[847]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));}
        if ((((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) && (!s.b[847])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);}
        if (((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[830])) && (!s.b[845])) && (!s.b[846])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if (s.b[418] && (!s.b[830])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        if s.b[418] {s.store_add_scaled_inputs3_indices(184, 268, s.v[256], 270, s.v[257], 272, s.v[258]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
    ) {
        if s.b[418] {s.copy_ad(300, 289);s.store_add_scaled_offset_product_rhs_mixed_iia(188, 183, 1.0, 300, A::exp(A::mul_scaled_lhs(s.ad_value(193), s.v[85], s.ad_value(301))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(189, 184, 1.0, 300, A::exp(A::mul_scaled_lhs(s.ad_value(194), s.v[85], s.ad_value(301))), (-1.0), (-1.0));}
        s.b[848] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });s.b[849] = ((s.v[183] > 0.0) && (s.v[184] > 0.0));s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });s.b[850] = (((((s.v[188] / s.v[183]) > 0.001) || ((s.v[189] / s.v[184]) > 0.001)) && (s.v[188] > 0.0)) && (s.v[189] > 0.0));s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[848]) && s.b[849]) && s.b[850]) {s.store_div(195, 188, 189);s.store_div_scaled_inputs(303, A::ln(s.ad_value(195)), s.v[84], A::sub(s.ad_value(193), s.ad_value(194)), 1.0);s.store_div_scaled_value_offset_denominator(302, s.ad_value(188), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(193), s.v[85], s.ad_value(303))), (-1.0), 1.0);}
        if (s.b[418] && s.b[848]) {s.store_add_scaled_offset_product_rhs_mixed_aia(185, A::add_scaled_offset_product_rhs(s.ad_value(180), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(190), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, 302, A::exp(A::mul_scaled_lhs(s.ad_value(190), s.v[85], s.ad_value(303))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(186, A::add_scaled_offset_product_rhs(s.ad_value(181), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(191), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, 302, A::exp(A::mul_scaled_lhs(s.ad_value(191), s.v[85], s.ad_value(303))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(187, A::add_scaled_offset_product_rhs(s.ad_value(182), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(192), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, 302, A::exp(A::mul_scaled_lhs(s.ad_value(192), s.v[85], s.ad_value(303))), (-1.0), (-1.0));}
        s.b[851] = (((s.v[180] < 0.0) && (s.v[181] < 0.0)) && (s.v[182] < 0.0));s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });s.b[852] = (((((((s.v[185] / s.v[180]) > 0.001) || ((s.v[186] / s.v[181]) > 0.001)) || ((s.v[187] / s.v[182]) > 0.001)) && (s.v[185] < 0.0)) && (s.v[186] < 0.0)) && (s.v[187] < 0.0));s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) {s.store_div(195, 185, 186);s.store_div_scaled_inputs(196, A::ln(s.ad_value(195)), (-s.v[84]), A::sub(s.ad_value(190), s.ad_value(191)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(198, 191, 191, 1.0, 190, -1.0);s.store_scaled_mul_ad(199, A::offset(s.ad_value(195), (-1.0)), A::offset(A::pow(s.ad_value(195), s.ad_value(198)), (-1.0)), s.v[84]);s.store_primal_div_add_scaled_inputs_rhs_indices(198, 190, 190, 1.0, 191, -1.0);s.store_sub_mixed_ai(200, A::add_scaled_products(A::pow(s.ad_value(195), s.ad_value(198)), A::sub(s.ad_value(191), s.ad_value(190)), 1.0, s.ad_value(195), s.ad_value(190), 1.0), 191);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) {s.store_div(197, 199, 200);s.store_add(305, 196, 197);}
        s.b[853] = (((((s.v[192] * s.v[85]) * s.v[305])) as f64).abs() < 1e-6);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) && s.b[853]) {s.store_scalar(306, 1.0);s.store_mul_add_scaled_inputs_rhs_mixed_ai(304, 187, A::div_from_scalar(1.0, s.ad_value(192)), 1.0, 305, (0.5 * s.v[85]));s.store_div_scaled_product_indices(305, 187, 305, ((-0.5) * s.v[85]), 192, 1.0);}
        if ((((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) && (!s.b[853])) {s.store_scalar(306, 0.0);s.store_div_scaled_value_offset_denominator(304, s.ad_value(187), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(192), (-s.v[85]), s.ad_value(305))), (-1.0), 1.0);}
        if s.b[418] {s.store_primal_add_scaled_inputs3_indices(208, 128, (s.v[256] * s.v[47]), 129, (s.v[257] * s.v[47]), 130, (s.v[258] * s.v[47]));}
        s.b[854] = ((s.v[256] * s.v[128]) <= s.v[208]);s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[854]) {s.store_scalar(259, 0.0);}
        s.b[855] = ((s.v[257] * s.v[129]) <= s.v[208]);s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[855]) {s.store_scalar(260, 0.0);}
        s.b[856] = ((s.v[258] * s.v[130]) <= s.v[208]);s.store_scalar(856, if s.b[856] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[856]) {s.store_scalar(261, 0.0);}
        s.b[857] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(857, if s.b[857] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[857]) {s.store_primal_ln_ad(294, A::div_scalar_offset_denominator((0.5 * s.v[2]), s.ad_value(300), 1e-21, 1.0));s.store_ln_ad(296, A::div_scalar_offset_denominator((0.5 * s.v[2]), s.ad_value(302), 1e-21, 1.0));s.store_ln_ad(298, A::div_scalar_offset_denominator((0.5 * s.v[2]), A::abs(s.ad_value(304)), 1e-21, 1.0));}
        if s.b[418] {s.store_primal_min_with_scalar(294, 294, 230.25850929940458);s.store_primal_exp(295, 294);s.store_min_with_scalar(296, 296, 230.25850929940458);s.store_exp(297, 296);s.store_min_with_scalar(298, 298, 230.25850929940458);s.store_exp(299, 298);}
        s.store_voltage(277, ctx, nodes, Some(0), Some(2));s.b[858] = (s.v[45] == 1.0);s.store_scalar(858, if s.b[858] { 1.0 } else { 0.0 });
        if s.b[858] {s.store_scaled_mul(201, 277, 301, s.v[85]);}
        if s.b[858] {
            if (s.v[201] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(202, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0);
            } else {
                if (s.v[201] > s.v[294]) {
                    s.store_mul_scale_offset_mixed_ia(202, 295, A::sub(s.ad_value(201), s.ad_value(294)), 1.0, 1.0);
                } else {
                    s.store_exp(202, 201);
                }
            }
        }
        if s.b[858] {s.store_mul_scale_offset_indices(209, 300, 202, 1.0, (-1.0));s.store_scaled_mul(201, 277, 303, s.v[85]);}
        if s.b[858] {
            if (s.v[201] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(202, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0);
            } else {
                if (s.v[201] > s.v[296]) {
                    s.store_mul_scale_offset_mixed_ia(202, 297, A::sub(s.ad_value(201), s.ad_value(296)), 1.0, 1.0);
                } else {
                    s.store_exp(202, 201);
                }
            }
        }
        if s.b[858] {s.store_mul_scale_offset_indices(210, 302, 202, 1.0, (-1.0));s.store_scalar(211, 0.0);}
        s.b[859] = (s.v[306] > 0.0);s.store_scalar(859, if s.b[859] { 1.0 } else { 0.0 });
        if (s.b[858] && s.b[859]) {s.store_mul_add_scaled_product_rhs_indices(211, 277, 304, 1.0, 277, 305, 1.0);}
        if (s.b[858] && (!s.b[859])) {s.store_scaled_mul(201, 277, 305, (-s.v[85]));}
        if (s.b[858] && (!s.b[859])) {
            if (s.v[201] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(202, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0);
            } else {
                if (s.v[201] > s.v[298]) {
                    s.store_mul_scale_offset_mixed_ia(202, 299, A::sub(s.ad_value(201), s.ad_value(298)), 1.0, 1.0);
                } else {
                    s.store_exp(202, 201);
                }
            }
        }
        if (s.b[858] && (!s.b[859])) {s.store_mul_scaled_offset_rhs(211, 304, -1.0, 202, (-1.0));}
        if s.b[858] {s.store_add_scaled_inputs3_indices(274, 209, 1.0, 210, 1.0, 211, 1.0);s.store_add(290, 210, 211);s.store_scalar(268, 0.0);s.store_scalar(270, 0.0);s.store_scalar(272, 0.0);s.store_scalar(291, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
    ) {
        if s.b[858] {s.store_scalar(292, 0.0);s.store_scalar(293, 0.0);s.store_primal_scaled_mul(215, 265, 265, 4.0);s.store_primal_div(216, 265, 266);s.store_add_scaled_product_indices(217, 277, 1.0, 265, 216, 1.0);s.store_add(218, 266, 217);s.store_sub(219, 266, 217);s.store_sqrt_square_add(220, 219, 215);s.store_div_scaled_product_add_scaled_denominator_indices(204, 277, 266, 2.0, 218, 1.0, 220, 1.0, 1.0);}
        s.b[860] = (s.v[259] > 0.5);s.store_scalar(860, if s.b[860] { 1.0 } else { 0.0 });s.b[861] = (s.v[122] == 0.5);s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
        if ((s.b[858] && s.b[860]) && s.b[861]) {s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(119)));}
        if ((s.b[858] && s.b[860]) && (!s.b[861])) {s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))), s.v[122]);}
        if (s.b[858] && s.b[860]) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(203)), 1.0, 134, A::sub(s.ad_value(277), s.ad_value(204)), 1.0);}
        if (s.b[858] && (!s.b[860])) {s.store_scalar(269, 0.0);}
        s.b[862] = (s.v[260] > 0.5);s.store_scalar(862, if s.b[862] { 1.0 } else { 0.0 });s.b[863] = (s.v[123] == 0.5);s.store_scalar(863, if s.b[863] { 1.0 } else { 0.0 });
        if ((s.b[858] && s.b[862]) && s.b[863]) {s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(120)));}
        if ((s.b[858] && s.b[862]) && (!s.b[863])) {s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))), s.v[123]);}
        if (s.b[858] && s.b[862]) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(203)), 1.0, 135, A::sub(s.ad_value(277), s.ad_value(204)), 1.0);}
        if (s.b[858] && (!s.b[862])) {s.store_scalar(271, 0.0);}
        s.b[864] = (s.v[261] > 0.5);s.store_scalar(864, if s.b[864] { 1.0 } else { 0.0 });s.b[865] = (s.v[124] == 0.5);s.store_scalar(865, if s.b[865] { 1.0 } else { 0.0 });
        if ((s.b[858] && s.b[864]) && s.b[865]) {s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(121)));}
        if ((s.b[858] && s.b[864]) && (!s.b[865])) {s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))), s.v[124]);}
        if (s.b[858] && s.b[864]) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(203)), 1.0, 136, A::sub(s.ad_value(277), s.ad_value(204)), 1.0);}
        if (s.b[858] && (!s.b[864])) {s.store_scalar(273, 0.0);}
        s.b[866] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(866, if s.b[866] { 1.0 } else { 0.0 });
        if ((!s.b[858]) && s.b[866]) {s.store_primal_scaled_mul(215, 265, 265, 4.0);s.store_primal_div(216, 265, 266);s.store_add_scaled_product_indices(217, 277, 1.0, 265, 216, 1.0);s.store_add(218, 266, 217);s.store_sub(219, 266, 217);s.store_sqrt_square_add(220, 219, 215);s.store_div_scaled_product_add_scaled_denominator_indices(221, 277, 266, 2.0, 218, 1.0, 220, 1.0, 1.0);}
        s.b[867] = (s.v[277] < s.v[262]);s.store_scalar(867, if s.b[867] { 1.0 } else { 0.0 });s.b[868] = ((((0.5 * (s.v[277] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(868, if s.b[868] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[868]) {s.store_exp_scaled_input(223, 277, (s.v[85] * 0.5));}
        s.b[869] = ((0.5 * (s.v[277] * s.v[85])) < (-230.25850929940458));s.store_scalar(869, if s.b[869] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[868])) && s.b[869]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(223, 1e-100, (-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[868])) && (!s.b[869])) {s.store_scaled_offset_ad(223, A::mul_offset_rhs(A::scale_offset(s.ad_value(277), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(277), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(277), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && s.b[867]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[870] = (s.v[62] < p.p85);s.store_scalar(870, if s.b[870] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[870])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[871] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(871, if s.b[871] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[871]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[872] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(872, if s.b[872] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && s.b[872]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && (!s.b[872])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && s.b[867]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[873] = (s.v[64] < p.p85);s.store_scalar(873, if s.b[873] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[873])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[874] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[874]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[875] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[874])) && s.b[875]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[874])) && (!s.b[875])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && s.b[867]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[876] = (s.v[63] < p.p85);s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[876])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[877] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(877, if s.b[877] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[877]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[878] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[877])) && s.b[878]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[877])) && (!s.b[878])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {s.store_sqrt_ad(223, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(277), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[879] = (s.v[62] < p.p85);s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[879])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[880] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[880]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[881] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && s.b[881]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && (!s.b[881])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[882] = (s.v[64] < p.p85);s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[882])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[883] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[883]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[884] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[883])) && s.b[884]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[883])) && (!s.b[884])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[885] = (s.v[63] < p.p85);s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
    }
}
