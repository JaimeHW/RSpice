#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[540]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(370, 0.0);
        }

        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        s.b[618] = (s.v[192] < s.v[262]);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[621] = (s.v[62] < p.p85);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[621])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[622] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[622]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[623] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && s.b[623]) {
            let assign14070_ad_e18870: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign14070_ad_e18870, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {
            let assign14080_ad_e18946: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign14080_ad_e18946, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[624] = (s.v[64] < p.p85);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[627] = (s.v[63] < p.p85);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[627])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[630] = (s.v[62] < p.p85);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[630])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[631] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[631]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[632] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && s.b[632]) {
            let assign15080_ad_e20780: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign15080_ad_e20780, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && (!s.b[632])) {
            let assign15090_ad_e20857: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign15090_ad_e20857, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[633] = (s.v[64] < p.p85);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[633])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[636] = (s.v[63] < p.p85);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[636])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[617]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(370, 0.0);
        }

        s.b[694] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        s.b[695] = (s.v[193] < s.v[262]);
        s.v[695] = if s.b[695] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[698] = (s.v[62] < p.p85);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[698])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[699] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[699] = if s.b[699] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[699]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[700] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[700] = if s.b[700] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && s.b[700]) {
            let assign18960_ad_e26828: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign18960_ad_e26828, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {
            let assign18970_ad_e26904: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign18970_ad_e26904, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[701] = (s.v[64] < p.p85);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[704] = (s.v[63] < p.p85);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[707] = (s.v[62] < p.p85);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[707])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[708] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[708]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[709] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[709] = if s.b[709] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && s.b[709]) {
            let assign19970_ad_e28738: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign19970_ad_e28738, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {
            let assign19980_ad_e28815: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign19980_ad_e28815, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[710] = (s.v[64] < p.p85);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[713] = (s.v[63] < p.p85);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[694]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[694])) {
            s.store_scalar(370, 0.0);
        }

        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        s.b[772] = (s.v[194] < s.v[262]);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[775] = (s.v[62] < p.p85);
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[775])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[776] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[776]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[777] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && s.b[777]) {
            let assign23850_ad_e34786: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign23850_ad_e34786, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && (!s.b[777])) {
            let assign23860_ad_e34862: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign23860_ad_e34862, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[778] = (s.v[64] < p.p85);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[781] = (s.v[63] < p.p85);
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[784] = (s.v[62] < p.p85);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[784])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[785] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[785]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[786] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && s.b[786]) {
            let assign24860_ad_e36696: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign24860_ad_e36696, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && (!s.b[786])) {
            let assign24870_ad_e36773: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign24870_ad_e36773, 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[787] = (s.v[64] < p.p85);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[787])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[790] = (s.v[63] < p.p85);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[790])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[771]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[771])) {
            s.store_scalar(370, 0.0);
        }

        s.store_voltage(277, ctx, nodes, Some(0), Some(2));

        s.b[858] = (s.v[45] == 1.0);
        s.v[858] = if s.b[858] { 1.0 } else { 0.0 };

        s.b[866] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[866] = if s.b[866] { 1.0 } else { 0.0 };

        s.b[867] = (s.v[277] < s.v[262]);
        s.v[867] = if s.b[867] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[870] = (s.v[62] < p.p85);
        s.v[870] = if s.b[870] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[870])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[871] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[871] = if s.b[871] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[871]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[872] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[872] = if s.b[872] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && s.b[872]) {
            let assign29660_ad_e43817: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign29660_ad_e43817, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && (!s.b[872])) {
            let assign29670_ad_e43894: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign29670_ad_e43894, 1.0), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[873] = (s.v[64] < p.p85);
        s.v[873] = if s.b[873] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[873])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[876] = (s.v[63] < p.p85);
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[876])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[879] = (s.v[62] < p.p85);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[879])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[880] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[880]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[881] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && s.b[881]) {
            let assign30670_ad_e45819: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign30670_ad_e45819, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && (!s.b[881])) {
            let assign30680_ad_e45897: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_mul_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign30680_ad_e45897, 1.0), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[882] = (s.v[64] < p.p85);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[882])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[885] = (s.v[63] < p.p85);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[885])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if ((!s.b[858]) && s.b[866]) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((!s.b[858]) && (!s.b[866])) {
            s.store_scalar(370, 0.0);
        }

        s.b[945] = (p.p84 > 0.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        s.b[946] = (s.v[313] < p.p85);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[946]) {
            s.store_offset_sub_scaled_inputs(349, s.ad_value(277), p.p86, s.ad_value(348), p.p86, s.v[313]);
            s.store_sub_from_scalar_scaled_input(350, s.v[313], 348, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(349), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(351, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(352, 314, 0.5, 315, 0.5, s.v[313]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[313]);
        }

        if (s.b[945] && (!s.b[946])) {
            s.store_scalar(352, s.v[313]);
            s.store_scalar(350, s.v[313]);
        }

        if s.b[945] {
            s.copy_ad(353, 370);
        }

        s.b[947] = ((s.v[277] - (s.v[348] - s.v[347])) > 0.0);
        s.v[947] = if s.b[947] { 1.0 } else { 0.0 };

        s.b[948] = ((((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[948] = if s.b[948] { 1.0 } else { 0.0 };

        if ((s.b[945] && s.b[947]) && s.b[948]) {
            s.store_exp_scaled_input_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), 1.0, A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), (-1.0), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), 1.0), s.v[85]);
        }

        s.b[949] = ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && s.b[949]) {
            let assign34480_ad_e51899: A = A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-0.3333333333333333), (((-230.25850929940458)) * (0.3333333333333333)));
            let assign34480_ad_e51902: A = A::mul_sub_from_scalar_lhs_scaled_output((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(assign34480_ad_e51899, 1.0), 0.5);
            let assign34480_ad_e51904: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(assign34480_ad_e51902, 1.0));
            s.store_div_from_scalar_offset_ad(354, 1e-100, assign34480_ad_e51904, 1.0);
        }

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {
            let assign34490_ad_e51994: A = A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)));
            let assign34490_ad_e51996: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), assign34490_ad_e51994, 0.5);
            let assign34490_ad_e51998: A = A::mul_offset_lhs(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign34490_ad_e51996, 1.0));
            s.store_scaled_offset_ad(354, assign34490_ad_e51998, 1.0, 1e100);
        }

        if (s.b[945] && (!s.b[947])) {
            s.store_scalar(354, 1.0);
        }

        s.b[950] = ((p.p91 == 0.0) || (s.v[277] < s.v[347]));
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[950]) {
            s.store_scale(357, 353, p.p90);
        }

        if (s.b[945] && (!s.b[950])) {
            s.store_mul_scaled_exp_ad_rhs(357, 353, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(347)), A::sub(s.ad_value(277), s.ad_value(347)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));
        }

        if s.b[945] {
            if (s.v[357] > p.p79) {
                s.store_scalar(357, p.p79);
            } else {
            }
        }

        if s.b[945] {
            s.store_mul(355, 319, 357);
            s.store_scaled_sub(331, 355, 319, (1.6021918e-19 * s.v[256]));
        }

        s.b[951] = (p.p92 > 0.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[951]) {
            s.store_scale(334, 331, (1e-23 / s.v[333]));
            s.store_voltage(336, ctx, nodes, Some(3), None);
            s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));
        }

        if (s.b[945] && (!s.b[951])) {
            s.copy_ad(334, 331);
        }

        s.b[952] = ((p.p91 == 0.0) || (s.v[277] < s.v[348]));
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[952]) {
            s.store_scale(358, 354, p.p90);
        }

        if (s.b[945] && (!s.b[952])) {
            s.store_mul_scaled_exp_ad_rhs(358, 354, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(348)), A::sub(s.ad_value(277), s.ad_value(348)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));
        }

        if s.b[945] {
            if (s.v[358] > p.p79) {
                s.store_scalar(358, p.p79);
            } else {
            }
        }

        if s.b[945] {
            s.store_mul(356, 319, 358);
            s.store_scaled_sub(332, 356, 319, (1.6021918e-19 * s.v[256]));
        }

        s.b[953] = (p.p92 > 0.0);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[953]) {
            s.store_scale(335, 332, (1e-23 / s.v[333]));
            s.store_voltage(337, ctx, nodes, Some(4), None);
            s.store_scaled_sub(339, 337, 335, 1.0 / (p.p92));
        }

        if (s.b[945] && (!s.b[953])) {
            s.copy_ad(335, 332);
        }

        if s.b[945] {
            s.store_sub_from_scalar(325, s.v[368], 277);
            s.store_sqrt_square_offset(315, 325, ((4.0 * s.v[369]) * s.v[369]));
            s.store_scaled_add(325, 325, 315, 0.5);
        }

        s.b[954] = (s.v[325] < 0.0);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[954]) {
            s.store_scalar(325, 0.0);
        }

        if s.b[945] {
            s.store_sqrt_scaled_input(326, 325, ((2.0 * s.v[0]) * 1.0 / ((1.6021918e-19 * s.v[307]))));
            s.store_offset_sub_from_scalar_ad(314, p.p94, s.ad_value(326), (-1e-7));
            s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        }

        if s.b[945] {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if s.b[945] {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(326, 314, (-0.5), 315, (-0.5), p.p94);
        }

        s.b[955] = (p.p95 > 0.0);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[955]) {
            s.store_mul_div_from_scalar_rhs(342, 326, 1.0, 343);
            s.store_voltage(344, ctx, nodes, Some(5), None);
            s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));
        }

        if (s.b[945] && (!s.b[955])) {
            s.copy_ad(342, 326);
        }

        s.b[958] = ((p.p84 > 0.0) && (p.p92 > 0.0));
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        s.b[959] = ((p.p84 > 0.0) && (p.p95 > 0.0));
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq6_e135,) = {
    if (s.v[957] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e135;
        stamper.stamp_potential_const_local(
            0,
            eq6_value,
        );
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3,) = {
    if s.b[958] {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[336]);
        let eq7_e140_d_n0: f64 = (s.dn[336][0] * ddt_scale);
        let eq7_e140_d_n1: f64 = (s.dn[336][1] * ddt_scale);
        let eq7_e140_d_n2: f64 = (s.dn[336][2] * ddt_scale);
        let eq7_e140_d_n3: f64 = (s.dn[336][3] * ddt_scale);
        let eq7_e140_d_n4: f64 = (s.dn[336][4] * ddt_scale);
        let eq7_e140_d_n5: f64 = (s.dn[336][5] * ddt_scale);
        let eq7_e140_d_b0: f64 = (s.db[336][0] * ddt_scale);
        let eq7_e140_d_b1: f64 = (s.db[336][1] * ddt_scale);
        let eq7_e140_d_b2: f64 = (s.db[336][2] * ddt_scale);
        let eq7_e140_d_b3: f64 = (s.db[336][3] * ddt_scale);
        let eq7_e141: f64 = (s.v[338] + eq7_e140);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + eq7_e140_d_n0);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + eq7_e140_d_n1);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + eq7_e140_d_n2);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + eq7_e140_d_n3);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + eq7_e140_d_n4);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + eq7_e140_d_n5);
        let eq7_e141_d_b0: f64 = (s.db[338][0] + eq7_e140_d_b0);
        let eq7_e141_d_b1: f64 = (s.db[338][1] + eq7_e140_d_b1);
        let eq7_e141_d_b2: f64 = (s.db[338][2] + eq7_e140_d_b2);
        let eq7_e141_d_b3: f64 = (s.db[338][3] + eq7_e140_d_b3);
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        let eq7_node_derivatives: [f64; 6] = [eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5];
        let eq7_branch_derivatives: [f64; 4] = [eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3,) = {
    if s.b[958] {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[337]);
        let eq8_e149_d_n0: f64 = (s.dn[337][0] * ddt_scale);
        let eq8_e149_d_n1: f64 = (s.dn[337][1] * ddt_scale);
        let eq8_e149_d_n2: f64 = (s.dn[337][2] * ddt_scale);
        let eq8_e149_d_n3: f64 = (s.dn[337][3] * ddt_scale);
        let eq8_e149_d_n4: f64 = (s.dn[337][4] * ddt_scale);
        let eq8_e149_d_n5: f64 = (s.dn[337][5] * ddt_scale);
        let eq8_e149_d_b0: f64 = (s.db[337][0] * ddt_scale);
        let eq8_e149_d_b1: f64 = (s.db[337][1] * ddt_scale);
        let eq8_e149_d_b2: f64 = (s.db[337][2] * ddt_scale);
        let eq8_e149_d_b3: f64 = (s.db[337][3] * ddt_scale);
        let eq8_e150: f64 = (s.v[339] + eq8_e149);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + eq8_e149_d_n0);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + eq8_e149_d_n1);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + eq8_e149_d_n2);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + eq8_e149_d_n3);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + eq8_e149_d_n4);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + eq8_e149_d_n5);
        let eq8_e150_d_b0: f64 = (s.db[339][0] + eq8_e149_d_b0);
        let eq8_e150_d_b1: f64 = (s.db[339][1] + eq8_e149_d_b1);
        let eq8_e150_d_b2: f64 = (s.db[339][2] + eq8_e149_d_b2);
        let eq8_e150_d_b3: f64 = (s.db[339][3] + eq8_e149_d_b3);
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        let eq8_node_derivatives: [f64; 6] = [eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5];
        let eq8_branch_derivatives: [f64; 4] = [eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e158,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let (eq10_e163,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e163;
        stamper.stamp_potential_const_local(
            2,
            eq10_value,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3,) = {
    if s.b[959] {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[344]);
        let eq11_e168_d_n0: f64 = (s.dn[344][0] * ddt_scale);
        let eq11_e168_d_n1: f64 = (s.dn[344][1] * ddt_scale);
        let eq11_e168_d_n2: f64 = (s.dn[344][2] * ddt_scale);
        let eq11_e168_d_n3: f64 = (s.dn[344][3] * ddt_scale);
        let eq11_e168_d_n4: f64 = (s.dn[344][4] * ddt_scale);
        let eq11_e168_d_n5: f64 = (s.dn[344][5] * ddt_scale);
        let eq11_e168_d_b0: f64 = (s.db[344][0] * ddt_scale);
        let eq11_e168_d_b1: f64 = (s.db[344][1] * ddt_scale);
        let eq11_e168_d_b2: f64 = (s.db[344][2] * ddt_scale);
        let eq11_e168_d_b3: f64 = (s.db[344][3] * ddt_scale);
        let eq11_e169: f64 = (s.v[345] + eq11_e168);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + eq11_e168_d_n0);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + eq11_e168_d_n5);
        let eq11_e169_d_b0: f64 = (s.db[345][0] + eq11_e168_d_b0);
        let eq11_e169_d_b1: f64 = (s.db[345][1] + eq11_e168_d_b1);
        let eq11_e169_d_b2: f64 = (s.db[345][2] + eq11_e168_d_b2);
        let eq11_e169_d_b3: f64 = (s.db[345][3] + eq11_e168_d_b3);
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        let eq11_node_derivatives: [f64; 6] = [eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e177,) = {
    if (!s.b[959]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e177;
        stamper.stamp_potential_const_local(
            3,
            eq12_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5, eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3,) = {
    if s.b[958] {
        let eq7_e140_q: f64 = s.v[336];
        let eq7_e141: f64 = (s.v[338] + s.v[336]);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + s.dn[336][0]);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + s.dn[336][1]);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + s.dn[336][2]);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + s.dn[336][3]);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + s.dn[336][4]);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + s.dn[336][5]);
        let eq7_e141_d_b0: f64 = (s.db[338][0] + s.db[336][0]);
        let eq7_e141_d_b1: f64 = (s.db[338][1] + s.db[336][1]);
        let eq7_e141_d_b2: f64 = (s.db[338][2] + s.db[336][2]);
        let eq7_e141_d_b3: f64 = (s.db[338][3] + s.db[336][3]);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n0: f64 = (1e-12 * s.dn[336][0]);
        let eq7_e142_q_d_n1: f64 = (1e-12 * s.dn[336][1]);
        let eq7_e142_q_d_n2: f64 = (1e-12 * s.dn[336][2]);
        let eq7_e142_q_d_n3: f64 = (1e-12 * s.dn[336][3]);
        let eq7_e142_q_d_n4: f64 = (1e-12 * s.dn[336][4]);
        let eq7_e142_q_d_n5: f64 = (1e-12 * s.dn[336][5]);
        let eq7_e142_q_d_b0: f64 = (1e-12 * s.db[336][0]);
        let eq7_e142_q_d_b1: f64 = (1e-12 * s.db[336][1]);
        let eq7_e142_q_d_b2: f64 = (1e-12 * s.db[336][2]);
        let eq7_e142_q_d_b3: f64 = (1e-12 * s.db[336][3]);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3, eq7_e142_q, eq7_e142_q_d_n0, eq7_e142_q_d_n1, eq7_e142_q_d_n2, eq7_e142_q_d_n3, eq7_e142_q_d_n4, eq7_e142_q_d_n5, eq7_e142_q_d_b0, eq7_e142_q_d_b1, eq7_e142_q_d_b2, eq7_e142_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];
        let eq7_reactive_branch_derivatives: [f64; 4] = [eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5, eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3,) = {
    if s.b[958] {
        let eq8_e149_q: f64 = s.v[337];
        let eq8_e150: f64 = (s.v[339] + s.v[337]);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + s.dn[337][0]);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + s.dn[337][1]);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + s.dn[337][2]);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + s.dn[337][3]);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + s.dn[337][4]);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + s.dn[337][5]);
        let eq8_e150_d_b0: f64 = (s.db[339][0] + s.db[337][0]);
        let eq8_e150_d_b1: f64 = (s.db[339][1] + s.db[337][1]);
        let eq8_e150_d_b2: f64 = (s.db[339][2] + s.db[337][2]);
        let eq8_e150_d_b3: f64 = (s.db[339][3] + s.db[337][3]);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n0: f64 = (1e-12 * s.dn[337][0]);
        let eq8_e151_q_d_n1: f64 = (1e-12 * s.dn[337][1]);
        let eq8_e151_q_d_n2: f64 = (1e-12 * s.dn[337][2]);
        let eq8_e151_q_d_n3: f64 = (1e-12 * s.dn[337][3]);
        let eq8_e151_q_d_n4: f64 = (1e-12 * s.dn[337][4]);
        let eq8_e151_q_d_n5: f64 = (1e-12 * s.dn[337][5]);
        let eq8_e151_q_d_b0: f64 = (1e-12 * s.db[337][0]);
        let eq8_e151_q_d_b1: f64 = (1e-12 * s.db[337][1]);
        let eq8_e151_q_d_b2: f64 = (1e-12 * s.db[337][2]);
        let eq8_e151_q_d_b3: f64 = (1e-12 * s.db[337][3]);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3, eq8_e151_q, eq8_e151_q_d_n0, eq8_e151_q_d_n1, eq8_e151_q_d_n2, eq8_e151_q_d_n3, eq8_e151_q_d_n4, eq8_e151_q_d_n5, eq8_e151_q_d_b0, eq8_e151_q_d_b1, eq8_e151_q_d_b2, eq8_e151_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];
        let eq8_reactive_branch_derivatives: [f64; 4] = [eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5, eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3,) = {
    if s.b[959] {
        let eq11_e168_q: f64 = s.v[344];
        let eq11_e169: f64 = (s.v[345] + s.v[344]);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + s.dn[344][0]);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + s.dn[344][1]);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + s.dn[344][2]);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + s.dn[344][3]);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + s.dn[344][4]);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + s.dn[344][5]);
        let eq11_e169_d_b0: f64 = (s.db[345][0] + s.db[344][0]);
        let eq11_e169_d_b1: f64 = (s.db[345][1] + s.db[344][1]);
        let eq11_e169_d_b2: f64 = (s.db[345][2] + s.db[344][2]);
        let eq11_e169_d_b3: f64 = (s.db[345][3] + s.db[344][3]);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n0: f64 = (1e-13 * s.dn[344][0]);
        let eq11_e170_q_d_n1: f64 = (1e-13 * s.dn[344][1]);
        let eq11_e170_q_d_n2: f64 = (1e-13 * s.dn[344][2]);
        let eq11_e170_q_d_n3: f64 = (1e-13 * s.dn[344][3]);
        let eq11_e170_q_d_n4: f64 = (1e-13 * s.dn[344][4]);
        let eq11_e170_q_d_n5: f64 = (1e-13 * s.dn[344][5]);
        let eq11_e170_q_d_b0: f64 = (1e-13 * s.db[344][0]);
        let eq11_e170_q_d_b1: f64 = (1e-13 * s.db[344][1]);
        let eq11_e170_q_d_b2: f64 = (1e-13 * s.db[344][2]);
        let eq11_e170_q_d_b3: f64 = (1e-13 * s.db[344][3]);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3, eq11_e170_q, eq11_e170_q_d_n0, eq11_e170_q_d_n1, eq11_e170_q_d_n2, eq11_e170_q_d_n3, eq11_e170_q_d_n4, eq11_e170_q_d_n5, eq11_e170_q_d_b0, eq11_e170_q_d_b1, eq11_e170_q_d_b2, eq11_e170_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];
        let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
