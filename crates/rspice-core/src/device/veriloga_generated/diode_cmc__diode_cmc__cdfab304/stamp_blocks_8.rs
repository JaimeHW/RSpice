#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[699]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[700] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(700, if s.b[700] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && s.b[700]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && s.b[695]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[701] = (s.v[64] < p.p85);s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {s.store_offset_sub_scaled_inputs_indices(360, 193, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[702] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(702, if s.b[702] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[702]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[703] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(703, if s.b[703] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && s.b[703]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && (!s.b[703])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && s.b[695]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[704] = (s.v[63] < p.p85);s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {s.store_offset_sub_scaled_inputs_indices(360, 193, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[705] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[705]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[706] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && s.b[706]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && (!s.b[706])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {s.store_primal_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(193), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[707] = (s.v[62] < p.p85);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[707])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[708] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[708]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[709] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(709, if s.b[709] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && s.b[709]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[710] = (s.v[64] < p.p85);s.store_scalar(710, if s.b[710] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[711] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(711, if s.b[711] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[711]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[712] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && s.b[712]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && (!s.b[712])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[713] = (s.v[63] < p.p85);s.store_scalar(713, if s.b[713] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[714] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(714, if s.b[714] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[714]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[715] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(715, if s.b[715] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && s.b[715]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && (!s.b[715])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(372, 283, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);}
        if (s.b[418] && s.b[694]) {s.store_offset(370, 370, (-1.0));s.store_offset(371, 371, (-1.0));s.store_offset(372, 372, (-1.0));s.store_primal_div_from_scalar(429, 1.0, 430);}
        s.b[716] = (s.v[193] > 0.0);s.store_scalar(716, if s.b[716] { 1.0 } else { 0.0 });
        if ((s.b[418] && s.b[694]) && s.b[716]) {s.store_primal_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));}
        if ((s.b[418] && s.b[694]) && (!s.b[716])) {s.store_primal_sub_mixed_ai(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 193);}
        if (s.b[418] && s.b[694]) {s.store_primal_sub(432, 264, 431);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(433, 193, 0.5, 432, 0.5, 193, 432, ((4.0 * s.v[84]) * s.v[84]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(434, 193, 0.5, 267, 0.5, A::add_scaled_square_product(A::sub(s.ad_value(193), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0), (-0.5));s.store_primal_scaled_sub_mixed_ia(435, 193, A::sqrt_square_offset(s.ad_value(193), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        if (s.b[418] && (!s.b[694])) {s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(431, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[418] && (!s.b[694])) {s.store_scalar(428, 0.0);s.store_scalar(430, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);}
        s.b[717] = (s.v[256] == 0.0);s.store_scalar(717, if s.b[717] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[717]) {s.store_scalar(268, 0.0);s.store_scalar(291, 0.0);s.store_scalar(269, 0.0);}
        s.b[718] = (s.v[122] == 0.5);s.store_scalar(718, if s.b[718] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[717])) && s.b[718]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));}
        if ((s.b[418] && (!s.b[717])) && (!s.b[718])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);}
        if (s.b[418] && (!s.b[717])) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);s.store_mul(437, 101, 370);}
        s.b[719] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));s.store_scalar(719, if s.b[719] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[717])) && s.b[719]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {s.store_primal_sub(439, 107, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[720] = (s.v[9] == 0.5);s.store_scalar(720, if s.b[720] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && s.b[720]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && (!s.b[720])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));}
        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {s.store_primal_add(442, 440, 441);}
        s.b[721] = (s.v[9] == 0.5);s.store_scalar(721, if s.b[721] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && s.b[721]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[143]);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && (!s.b[721])) {s.store_primal_powf_scaled_input(436, 439, s.v[143], s.v[9]);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {s.store_primal_scale(443, 436, s.v[137]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 98, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[20]);}
        s.b[722] = (s.v[23] == 0.0);s.store_scalar(722, if s.b[722] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[717])) && s.b[722]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[122] * s.v[152]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[723] = (((-s.v[9]) * s.v[125]) == (-1.0));s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[723]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[723])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));}
        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[149]), 447, 450, s.v[149], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[724] = (s.v[457] > 0.0);s.store_scalar(724, if s.b[724] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[724]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[724])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[725] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(725, if s.b[725] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[725]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[725])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[726] = (s.v[457] > 0.0);s.store_scalar(726, if s.b[726] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[726]) {s.copy_ad(458, 421);}
        s.b[727] = (s.v[456] > (-230.25850929940458));s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) && s.b[727]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) && (!s.b[727])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[149] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);}
        s.b[728] = (s.v[29] == 0.0);s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[717])) && s.b[728]) {s.store_scalar(460, 0.0);}
        s.b[729] = (s.v[9] == 0.5);s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && s.b[729]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[729])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[143]), ((s.v[6]) * (s.v[143])), s.v[9]);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[728])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), 436, 1.0);}
        s.b[730] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && s.b[730]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));}
        s.b[731] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[730])) && s.b[731]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 155, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[730])) && (!s.b[731])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 155, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[717])) && (!s.b[728])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[29], 0.0);}
        s.b[732] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[717])) && s.b[732]) {s.store_scalar(462, 1.0);}
        s.b[733] = (s.v[435] > ((-s.v[158]) * s.v[38]));s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });s.b[734] = (s.v[41] == 4.0);s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) && s.b[734]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));}
        if ((((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) && (!s.b[734])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);}
        if (((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[717])) && (!s.b[732])) && (!s.b[733])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);}
        if (s.b[418] && (!s.b[717])) {s.store_mul_add_scaled_inputs4_indices_rhs(268, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(291, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[735] = (s.v[257] == 0.0);s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[735]) {s.store_scalar(270, 0.0);s.store_scalar(292, 0.0);s.store_scalar(271, 0.0);}
        s.b[736] = (s.v[123] == 0.5);s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[418] && (!s.b[735])) && s.b[736]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));}
        if ((s.b[418] && (!s.b[735])) && (!s.b[736])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);}
        if (s.b[418] && (!s.b[735])) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);s.store_mul(437, 102, 371);}
        s.b[737] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[735])) && s.b[737]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {s.store_primal_sub(439, 108, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[738] = (s.v[10] == 0.5);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && s.b[738]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && (!s.b[738])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));}
        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {s.store_primal_add(442, 440, 441);}
        s.b[739] = (s.v[10] == 0.5);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && s.b[739]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[144]);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && (!s.b[739])) {s.store_primal_powf_scaled_input(436, 439, s.v[144], s.v[10]);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {s.store_primal_scale(443, 436, s.v[138]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 99, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[21]);}
        s.b[740] = (s.v[24] == 0.0);s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[735])) && s.b[740]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[123] * s.v[153]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[741] = (((-s.v[10]) * s.v[126]) == (-1.0));s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[741]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[741])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));}
        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[150]), 447, 450, s.v[150], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[742] = (s.v[457] > 0.0);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[742]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[742])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[743] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[743]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[743])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[744] = (s.v[457] > 0.0);s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[744]) {s.copy_ad(458, 421);}
        s.b[745] = (s.v[456] > (-230.25850929940458));s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) && s.b[745]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) && (!s.b[745])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[150] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);}
        s.b[746] = (s.v[30] == 0.0);s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[735])) && s.b[746]) {s.store_scalar(460, 0.0);}
        s.b[747] = (s.v[10] == 0.5);s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && s.b[747]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[747])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[144]), ((s.v[7]) * (s.v[144])), s.v[10]);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[746])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), 436, 1.0);}
        s.b[748] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && s.b[748]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));}
        s.b[749] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[748])) && s.b[749]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 156, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[748])) && (!s.b[749])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 156, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[735])) && (!s.b[746])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[30], 0.0);}
        s.b[750] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[735])) && s.b[750]) {s.store_scalar(462, 1.0);}
        s.b[751] = (s.v[435] > ((-s.v[158]) * s.v[39]));s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });s.b[752] = (s.v[42] == 4.0);s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) && s.b[752]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));}
        if ((((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) && (!s.b[752])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);}
        if (((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[735])) && (!s.b[750])) && (!s.b[751])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if (s.b[418] && (!s.b[735])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[753] = (s.v[258] == 0.0);s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[753]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
        s.b[754] = (s.v[124] == 0.5);s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[753])) && s.b[754]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));}
        if ((s.b[418] && (!s.b[753])) && (!s.b[754])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[418] && (!s.b[753])) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);s.store_mul(437, 103, 372);}
        s.b[755] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[753])) && s.b[755]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {s.store_primal_sub(439, 109, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[756] = (s.v[11] == 0.5);s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && s.b[756]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && (!s.b[756])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));}
        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {s.store_primal_add(442, 440, 441);}
        s.b[757] = (s.v[11] == 0.5);s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && s.b[757]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[145]);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && (!s.b[757])) {s.store_primal_powf_scaled_input(436, 439, s.v[145], s.v[11]);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {s.store_primal_scale(443, 436, s.v[139]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 100, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[22]);}
        s.b[758] = (s.v[25] == 0.0);s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[753])) && s.b[758]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[124] * s.v[154]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[759] = (((-s.v[11]) * s.v[127]) == (-1.0));s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[759]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[759])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));}
        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[151]), 447, 450, s.v[151], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[760] = (s.v[457] > 0.0);s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[760]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[760])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[761] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[761]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[761])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[762] = (s.v[457] > 0.0);s.store_scalar(762, if s.b[762] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[762]) {s.copy_ad(458, 421);}
        s.b[763] = (s.v[456] > (-230.25850929940458));s.store_scalar(763, if s.b[763] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) && s.b[763]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) && (!s.b[763])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[151] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);}
        s.b[764] = (s.v[31] == 0.0);s.store_scalar(764, if s.b[764] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[753])) && s.b[764]) {s.store_scalar(460, 0.0);}
        s.b[765] = (s.v[11] == 0.5);s.store_scalar(765, if s.b[765] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && s.b[765]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[765])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[145]), ((s.v[8]) * (s.v[145])), s.v[11]);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[764])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), 436, 1.0);}
        s.b[766] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(766, if s.b[766] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && s.b[766]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));}
        s.b[767] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));s.store_scalar(767, if s.b[767] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[766])) && s.b[767]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 157, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[766])) && (!s.b[767])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 157, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[753])) && (!s.b[764])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[31], 0.0);}
        s.b[768] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[753])) && s.b[768]) {s.store_scalar(462, 1.0);}
        s.b[769] = (s.v[435] > ((-s.v[158]) * s.v[40]));s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });s.b[770] = (s.v[43] == 4.0);s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) && s.b[770]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));}
        if ((((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) && (!s.b[770])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);}
        if (((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[753])) && (!s.b[768])) && (!s.b[769])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if (s.b[418] && (!s.b[753])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        if s.b[418] {s.store_add_scaled_inputs3_indices(183, 268, s.v[256], 270, s.v[257], 272, s.v[258]);}
        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[771]) {s.store_primal_scaled_mul(422, 265, 265, 4.0);s.store_primal_div(423, 265, 266);s.store_primal_add_scaled_product_indices(424, 194, 1.0, 265, 423, 1.0);s.store_primal_add(425, 266, 424);s.store_primal_sub(426, 266, 424);s.store_primal_sqrt_square_add(427, 426, 422);s.store_primal_div_scaled_product_add_scaled_denominator_indices(428, 194, 266, 2.0, 425, 1.0, 427, 1.0, 1.0);}
        s.b[772] = (s.v[194] < s.v[262]);s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });s.b[773] = ((((0.5 * (s.v[194] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[773]) {s.store_primal_exp_scaled_input(430, 194, (s.v[85] * 0.5));}
        s.b[774] = ((0.5 * (s.v[194] * s.v[85])) < (-230.25850929940458));s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[773])) && s.b[774]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(430, 1e-100, (-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[773])) && (!s.b[774])) {s.store_primal_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(194), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(194), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(194), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && s.b[772]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && s.b[771]) && s.b[772]) {s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[775] = (s.v[62] < p.p85);s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[775])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[776] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[776]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[777] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && s.b[777]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && (!s.b[777])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && s.b[772]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[778] = (s.v[64] < p.p85);s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_sqrt_square_add(315, 314, 315);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[779] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[779]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[780] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[779])) && s.b[780]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[779])) && (!s.b[780])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && s.b[772]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[781] = (s.v[63] < p.p85);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[782] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[782]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[783] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[782])) && s.b[783]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[782])) && (!s.b[783])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && (!s.b[772])) {s.store_primal_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(194), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[784] = (s.v[62] < p.p85);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[784])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[785] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[785]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[786] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && s.b[786]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && (!s.b[786])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[771]) && (!s.b[772])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[787] = (s.v[64] < p.p85);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
    }
}
