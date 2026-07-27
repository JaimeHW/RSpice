#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_205(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3361])) {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3363] = (s.v[108] < 1e-12);s.store_scalar(3363, if s.b[3363] { 1.0 } else { 0.0 });
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3361])) && s.b[3363]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3361])) {s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_div(591, 598, 785);s.store_mul(592, 593, 591);}
        s.b[3364] = (s.v[799] >= 0.0);s.store_scalar(3364, if s.b[3364] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3364]) {s.store_div(335, 592, 3353);}
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3364])) {s.store_div_scaled_inputs_indices(335, 592, -1.0, 3353, 1.0);}
        s.b[3365] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3365, if s.b[3365] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3365]) {s.store_scalar(337, 1.0);}
        s.b[3366] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3366, if s.b[3366] { 1.0 } else { 0.0 });
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3365])) && s.b[3366]) {s.copy_ad(337, 335);}
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3365])) && (!s.b[3366])) {s.store_pow_offset_rhs(337, 335, 956, (-1.0));}
        if (s.b[3334] && (!s.b[3354])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3367] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3367, if s.b[3367] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3367]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3368] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3368, if s.b[3368] { 1.0 } else { 0.0 });
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3367])) && s.b[3368]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3367])) && (!s.b[3368])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }
        if (((s.b[3334] && (!s.b[3354])) && (!s.b[3367])) && (!s.b[3368])) {s.store_mul(339, 338, 340);}
        if (s.b[3334] && (!s.b[3354])) {s.store_mul(3352, 593, 339);s.store_offset(338, 335, 1.0);s.store_div_from_scalar(339, 1.0, 338);s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p[423]), 1.0), 1.0);s.store_offset(781, 338, (-0.001));s.store_scalar(782, 0.0);}
        if (s.b[3334] && (!s.b[3354])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3334] && (!s.b[3354])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(339, 781, 782, 0.5);s.store_mul(717, 408, 339);s.store_scale(718, 698, (6.241449993689894e18 * p[430]));s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));}
        if (s.b[3334] && (!s.b[3354])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3334] && (!s.b[3354])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(597, 717, 718);}
        s.b[3369] = ((p[441] > 0.0) && (p[440] > 1.0));s.store_scalar(3369, if s.b[3369] { 1.0 } else { 0.0 });s.b[3370] = ((s.v[597] > ((s.v[408] * p[440]) - (s.v[408] * p[441]))) && ((s.v[408] * p[441]) >= 0.0));s.store_scalar(3370, if s.b[3370] { 1.0 } else { 0.0 });
        if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) {s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p[440]), 408, p[441]);s.store_square(722, 781);s.store_scaled_mul(723, 408, 408, (p[441] * p[441]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_206(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && (s.v[719] < p[442])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3371] = ((((p[442] == 1.0) || (p[442] == 2.0)) || (p[442] == 4.0)) || (p[442] == 8.0));s.store_scalar(3371, if s.b[3371] { 1.0 } else { 0.0 });s.b[3372] = (p[442] == 1.0);s.store_scalar(3372, if s.b[3372] { 1.0 } else { 0.0 });
        if (((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) && s.b[3372]) {s.store_scalar(720, 1.0);}
        s.b[3373] = (p[442] == 2.0);s.store_scalar(3373, if s.b[3373] { 1.0 } else { 0.0 });
        if ((((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) && (!s.b[3372])) && s.b[3373]) {s.store_scalar(720, 2.0);}
        s.b[3374] = (p[442] == 4.0);s.store_scalar(3374, if s.b[3374] { 1.0 } else { 0.0 });
        if (((((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) && (!s.b[3372])) && (!s.b[3373])) && s.b[3374]) {s.store_scalar(720, 3.0);}
        s.b[3375] = (p[442] == 8.0);s.store_scalar(3375, if s.b[3375] { 1.0 } else { 0.0 });
        if ((((((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) && (!s.b[3372])) && (!s.b[3373])) && (!s.b[3374])) && s.b[3375]) {s.store_scalar(720, 4.0);}
        if ((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && s.b[3371]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) && (!s.b[3371])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p[442])));
            }
        }
        if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 408, p[441], 0.0, 726);s.store_div_scaled_product3_indices(334, 408, 725, 726, p[441], 770, 1.0);s.store_add_scaled_inputs3_indices(336, 408, p[440], 408, (-p[441]), 780, 1.0);}
        if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && s.b[3370]) {
        }
        if (((s.b[3334] && (!s.b[3354])) && s.b[3369]) && (!s.b[3370])) {s.copy_ad(336, 597);s.store_scalar(334, 1.0);}
        if ((s.b[3334] && (!s.b[3354])) && s.b[3369]) {s.copy_ad(597, 336);}
        if (s.b[3334] && (!s.b[3354])) {s.store_neg(334, 697);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[3376] = (s.v[334] < 0.0);s.store_scalar(3376, if s.b[3376] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3376]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(599, 650, 334);s.store_offset_sub(336, 3350, 3351, p[137]);s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3377] = (s.v[336] < 0.0);s.store_scalar(3377, if s.b[3377] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3377]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(600, 651, 336);s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));}
        if (s.b[3334] && (!s.b[3354])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3334] && (!s.b[3354])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));s.store_scalar(601, (p[419] + 1e-25));s.store_mul_scale_offset_mixed_ia(596, 649, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))), -1.0, 1.0);s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);s.store_scaled_add(596, 596, 782, 0.5);}
        s.b[3378] = (s.v[596] < 0.0);s.store_scalar(3378, if s.b[3378] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_207(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3334] && (!s.b[3354])) && s.b[3378]) {s.store_scalar(596, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p[422]);s.store_mul_product3_indices(739, 597, 335, 596, 3352, 1.0);}
        s.b[3379] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3379, if s.b[3379] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3379]) {s.store_sub_from_scalar(781, 1e-25, 739);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3380] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3380, if s.b[3380] { 1.0 } else { 0.0 });s.b[3381] = (2.0 == 1.0);s.store_scalar(3381, if s.b[3381] { 1.0 } else { 0.0 });
        if ((((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) && s.b[3381]) {s.store_scalar(720, 1.0);}
        s.b[3382] = (2.0 == 2.0);s.store_scalar(3382, if s.b[3382] { 1.0 } else { 0.0 });
        if (((((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) && (!s.b[3381])) && s.b[3382]) {s.store_scalar(720, 2.0);}
        s.b[3383] = (2.0 == 4.0);s.store_scalar(3383, if s.b[3383] { 1.0 } else { 0.0 });
        if ((((((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) && (!s.b[3381])) && (!s.b[3382])) && s.b[3383]) {s.store_scalar(720, 3.0);}
        s.b[3384] = (2.0 == 8.0);s.store_scalar(3384, if s.b[3384] { 1.0 } else { 0.0 });
        if (((((((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) && (!s.b[3381])) && (!s.b[3382])) && (!s.b[3383])) && s.b[3384]) {s.store_scalar(720, 4.0);}
        if (((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3334] && (!s.b[3354])) && s.b[3379]) && s.b[3380]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3334] && (!s.b[3354])) && s.b[3379]) && (!s.b[3380])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3334] && (!s.b[3354])) && s.b[3379]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(739, 1e-25, 780);}
        if ((s.b[3334] && (!s.b[3354])) && s.b[3379]) {
        }
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3379])) {
        }
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3379])) {s.store_scalar(334, 1.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_div_from_scalar(4, 1.0, 739);s.store_div(4, 4, 164);}
        s.b[3385] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));s.store_scalar(3385, if s.b[3385] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3385]) {s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));s.store_square(722, 781);s.store_scalar(723, (1000.0 * 1000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3386] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3386, if s.b[3386] { 1.0 } else { 0.0 });s.b[3387] = (2.0 == 1.0);s.store_scalar(3387, if s.b[3387] { 1.0 } else { 0.0 });
        if ((((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) && s.b[3387]) {s.store_scalar(720, 1.0);}
        s.b[3388] = (2.0 == 2.0);s.store_scalar(3388, if s.b[3388] { 1.0 } else { 0.0 });
        if (((((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) && (!s.b[3387])) && s.b[3388]) {s.store_scalar(720, 2.0);}
        s.b[3389] = (2.0 == 4.0);s.store_scalar(3389, if s.b[3389] { 1.0 } else { 0.0 });
        if ((((((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) && (!s.b[3387])) && (!s.b[3388])) && s.b[3389]) {s.store_scalar(720, 3.0);}
        s.b[3390] = (2.0 == 8.0);s.store_scalar(3390, if s.b[3390] { 1.0 } else { 0.0 });
        if (((((((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) && (!s.b[3387])) && (!s.b[3388])) && (!s.b[3389])) && s.b[3390]) {s.store_scalar(720, 4.0);}
        if (((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3334] && (!s.b[3354])) && s.b[3385]) && s.b[3386]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_208(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3334] && (!s.b[3354])) && s.b[3385]) && (!s.b[3386])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3334] && (!s.b[3354])) && s.b[3385]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1000.0);s.store_div_scaled_product_indices(334, 725, 726, 1000.0, 770, 1.0);s.store_offset(4, 780, (1000000.0 - 1000.0));}
        if ((s.b[3334] && (!s.b[3354])) && s.b[3385]) {
        }
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3385])) {
        }
        if ((s.b[3334] && (!s.b[3354])) && (!s.b[3385])) {s.store_scalar(334, 1.0);}
        s.b[3391] = ((p[54] == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(3391, if s.b[3391] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3391]) {s.store_sub_from_scalar(385, p[334], 384);s.store_div_scaled_inputs_indices(4, 4, s.v[165], 385, 1.0);}
        if (s.b[3334] && (!s.b[3354])) {s.store_add(4, 4, 644);}
        s.b[3393] = (s.v[4] < p[444]);s.store_scalar(3393, if s.b[3393] { 1.0 } else { 0.0 });
        if ((s.b[3334] && (!s.b[3354])) && s.b[3393]) {s.store_scalar(4, p[444]);}
        if (s.b[3334] && (!s.b[3354])) {s.store_scale(715, 4, 1.0 / (s.v[365]));}
        s.b[3394] = (s.v[4] < p[444]);s.store_scalar(3394, if s.b[3394] { 1.0 } else { 0.0 });
        if ((!s.b[3334]) && s.b[3394]) {s.store_scalar(4, p[444]);}
        s.b[3395] = (s.v[5] < p[444]);s.store_scalar(3395, if s.b[3395] { 1.0 } else { 0.0 });
        if ((!s.b[3334]) && s.b[3395]) {s.store_scalar(5, p[444]);}
        s.b[3396] = (s.v[370] > 0.0);s.store_scalar(3396, if s.b[3396] { 1.0 } else { 0.0 });
        if ((!s.b[3334]) && s.b[3396]) {s.store_scale(715, 4, 1.0 / (s.v[365]));s.store_scale(716, 5, 1.0 / (s.v[365]));}
        if ((!s.b[3334]) && (!s.b[3396])) {s.store_scale(715, 5, 1.0 / (s.v[365]));s.store_scale(716, 4, 1.0 / (s.v[365]));}
        s.copy_ad(4, 715);s.copy_ad(5, 716);s.b[3397] = (s.v[949] > 0.0);s.store_scalar(3397, if s.b[3397] { 1.0 } else { 0.0 });
        if s.b[3397] {s.copy_ad(134, 0);s.copy_ad(19, 701);s.copy_ad(18, 700);s.copy_ad(741, 702);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.copy_ad(280, 709);s.copy_ad(281, 710);s.copy_ad(400, 699);}
        if (s.b[3397] && (s.v[81] != 0.0)) {s.copy_ad(247, 708);}
        if (!s.b[3397]) {s.store_neg(134, 0);s.copy_ad(19, 702);s.copy_ad(18, 700);s.copy_ad(741, 701);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(400, 0.0);}
        if ((!s.b[3397]) && (s.v[81] != 0.0)) {s.store_sub_from_scalar(247, 1.0, 708);}
        s.store_add(18, 18, 811);s.store_add(19, 19, 810);s.store_add(741, 741, 812);s.store_add_scaled_inputs3_indices(20, 18, (-1.0), 19, (-1.0), 741, (-1.0));s.copy_ad(299, 703);s.copy_ad(301, 704);s.copy_ad(742, 706);s.copy_ad(743, 705);s.store_add_scaled_inputs3_indices(744, 705, (-1.0), 706, (-1.0), 707, (-1.0));s.b[3398] = (p[53] > 0.0);s.store_scalar(3398, if s.b[3398] { 1.0 } else { 0.0 });s.b[3399] = (s.v[766] > 0.0001);s.store_scalar(3399, if s.b[3399] { 1.0 } else { 0.0 });
        if (s.b[3398] && s.b[3399]) {s.store_div_from_scalar(740, 1.0, 766);}
        if (s.b[3398] && (!s.b[3399])) {s.store_scalar(740, (1.0 / 0.0001));}
        s.b[3400] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);s.store_scalar(3400, if s.b[3400] { 1.0 } else { 0.0 });s.b[3401] = (s.v[529] == 1.0);s.store_scalar(3401, if s.b[3401] { 1.0 } else { 0.0 });
        if ((s.b[3398] && s.b[3400]) && s.b[3401]) {s.copy_ad(745, 733);}
        if ((s.b[3398] && s.b[3400]) && (!s.b[3401])) {s.store_add_scaled_product_right_sub(745, 729, 1.0, 683, 733, 729, 1.0);}
        if (s.b[3398] && (!s.b[3400])) {s.copy_ad(745, 729);}
        if s.b[3398] {s.store_mul(746, 134, 745);}
        s.b[3402] = (p[53] == 1.0);s.store_scalar(3402, if s.b[3402] { 1.0 } else { 0.0 });
        if (s.b[3398] && s.b[3402]) {s.store_scale(335, 740, p[433]);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 746, (-1.0), 740, (-p[337]));s.store_scaled_mul(782, 335, 740, (4.0 * p[337]));}
        if (s.b[3398] && s.b[3402]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3398] && s.b[3402]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 335, 1.0, 781, (-0.5), 782, (-0.5));s.copy_ad(746, 336);}
        if (!s.b[3398]) {s.store_scalar(740, 0.0);s.store_scalar(746, 0.0);}
        if (s.v[81] != 0.0) {s.store_mul(751, 747, 247);s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);s.store_mul_scale_offset_indices(752, 747, 247, -1.0, 1.0);}
        if (s.v[81] == 0.0) {s.store_scalar(751, 0.0);s.store_scalar(753, 0.0);s.store_scalar(752, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_209(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(0, 949, 134, p[87]);s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(6), None));s.store_scale(22, 22, p[87]);s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(8), None));s.store_scale(23, 23, p[87]);s.b[3405] = (s.v[949] == 1.0);s.store_scalar(3405, if s.b[3405] { 1.0 } else { 0.0 });
        if s.b[3405] {s.copy_ad(757, 23);}
        if (!s.b[3405]) {s.copy_ad(757, 22);}
        s.b[3407] = (p[48] > 0.0);s.store_scalar(3407, if s.b[3407] { 1.0 } else { 0.0 });s.b[3411] = (p[53] > 0.0);s.store_scalar(3411, if s.b[3411] { 1.0 } else { 0.0 });
        if (!s.b[3411]) {s.store_scalar(767, 0.0);}
        if (p[28] != 0.0) {s.store_scalar(800, 1.0);s.store_scalar(801, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq0_e1025,) = {
    if s.b[999] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1025;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e1032, eq1_e1032_d_n0, eq1_e1032_d_n1, eq1_e1032_d_n2, eq1_e1032_d_n3, eq1_e1032_d_n4, eq1_e1032_d_n5, eq1_e1032_d_n6, eq1_e1032_d_n7, eq1_e1032_d_n8, eq1_e1032_d_n9, eq1_e1032_d_n10, eq1_e1032_d_n11, eq1_e1032_d_n12, eq1_e1032_d_n13, eq1_e1032_d_n14, eq1_e1032_d_n15, eq1_e1032_d_n16, eq1_e1032_d_n17, eq1_e1032_d_n18, eq1_e1032_d_b0, eq1_e1032_d_b1, eq1_e1032_d_b2, eq1_e1032_d_b3, eq1_e1032_d_b4, eq1_e1032_d_b5, eq1_e1032_d_b6, eq1_e1032_d_b7, eq1_e1032_d_b8, eq1_e1032_d_b9, eq1_e1032_d_b10, eq1_e1032_d_b11, eq1_e1032_d_b12,) = {
    if s.b[3307] {
        let eq1_e1029: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[924]);let eq1_e1030: f64 = (s.v[926] + eq1_e1029);let eq1_e1030_d_n0: f64 = (s.dn[926][0] + (s.dn[924][0] * ddt_scale));let eq1_e1030_d_n1: f64 = (s.dn[926][1] + (s.dn[924][1] * ddt_scale));let eq1_e1030_d_n2: f64 = (s.dn[926][2] + (s.dn[924][2] * ddt_scale));let eq1_e1030_d_n3: f64 = (s.dn[926][3] + (s.dn[924][3] * ddt_scale));let eq1_e1030_d_n4: f64 = (s.dn[926][4] + (s.dn[924][4] * ddt_scale));let eq1_e1030_d_n5: f64 = (s.dn[926][5] + (s.dn[924][5] * ddt_scale));let eq1_e1030_d_n6: f64 = (s.dn[926][6] + (s.dn[924][6] * ddt_scale));let eq1_e1030_d_n7: f64 = (s.dn[926][7] + (s.dn[924][7] * ddt_scale));let eq1_e1030_d_n8: f64 = (s.dn[926][8] + (s.dn[924][8] * ddt_scale));let eq1_e1030_d_n9: f64 = (s.dn[926][9] + (s.dn[924][9] * ddt_scale));let eq1_e1030_d_n10: f64 = (s.dn[926][10] + (s.dn[924][10] * ddt_scale));let eq1_e1030_d_n11: f64 = (s.dn[926][11] + (s.dn[924][11] * ddt_scale));let eq1_e1030_d_n12: f64 = (s.dn[926][12] + (s.dn[924][12] * ddt_scale));let eq1_e1030_d_n13: f64 = (s.dn[926][13] + (s.dn[924][13] * ddt_scale));let eq1_e1030_d_n14: f64 = (s.dn[926][14] + (s.dn[924][14] * ddt_scale));let eq1_e1030_d_n15: f64 = (s.dn[926][15] + (s.dn[924][15] * ddt_scale));let eq1_e1030_d_n16: f64 = (s.dn[926][16] + (s.dn[924][16] * ddt_scale));let eq1_e1030_d_n17: f64 = (s.dn[926][17] + (s.dn[924][17] * ddt_scale));let eq1_e1030_d_n18: f64 = (s.dn[926][18] + (s.dn[924][18] * ddt_scale));let eq1_e1030_d_b0: f64 = (s.db[926][0] + (s.db[924][0] * ddt_scale));let eq1_e1030_d_b1: f64 = (s.db[926][1] + (s.db[924][1] * ddt_scale));let eq1_e1030_d_b2: f64 = (s.db[926][2] + (s.db[924][2] * ddt_scale));let eq1_e1030_d_b3: f64 = (s.db[926][3] + (s.db[924][3] * ddt_scale));let eq1_e1030_d_b4: f64 = (s.db[926][4] + (s.db[924][4] * ddt_scale));let eq1_e1030_d_b5: f64 = (s.db[926][5] + (s.db[924][5] * ddt_scale));let eq1_e1030_d_b6: f64 = (s.db[926][6] + (s.db[924][6] * ddt_scale));let eq1_e1030_d_b7: f64 = (s.db[926][7] + (s.db[924][7] * ddt_scale));let eq1_e1030_d_b8: f64 = (s.db[926][8] + (s.db[924][8] * ddt_scale));let eq1_e1030_d_b9: f64 = (s.db[926][9] + (s.db[924][9] * ddt_scale));let eq1_e1030_d_b10: f64 = (s.db[926][10] + (s.db[924][10] * ddt_scale));let eq1_e1030_d_b11: f64 = (s.db[926][11] + (s.db[924][11] * ddt_scale));let eq1_e1030_d_b12: f64 = (s.db[926][12] + (s.db[924][12] * ddt_scale));
        (eq1_e1030, eq1_e1030_d_n0, eq1_e1030_d_n1, eq1_e1030_d_n2, eq1_e1030_d_n3, eq1_e1030_d_n4, eq1_e1030_d_n5, eq1_e1030_d_n6, eq1_e1030_d_n7, eq1_e1030_d_n8, eq1_e1030_d_n9, eq1_e1030_d_n10, eq1_e1030_d_n11, eq1_e1030_d_n12, eq1_e1030_d_n13, eq1_e1030_d_n14, eq1_e1030_d_n15, eq1_e1030_d_n16, eq1_e1030_d_n17, eq1_e1030_d_n18, eq1_e1030_d_b0, eq1_e1030_d_b1, eq1_e1030_d_b2, eq1_e1030_d_b3, eq1_e1030_d_b4, eq1_e1030_d_b5, eq1_e1030_d_b6, eq1_e1030_d_b7, eq1_e1030_d_b8, eq1_e1030_d_b9, eq1_e1030_d_b10, eq1_e1030_d_b11, eq1_e1030_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1032;let eq1_node_derivatives: [f64; 19] = [eq1_e1032_d_n0, eq1_e1032_d_n1, eq1_e1032_d_n2, eq1_e1032_d_n3, eq1_e1032_d_n4, eq1_e1032_d_n5, eq1_e1032_d_n6, eq1_e1032_d_n7, eq1_e1032_d_n8, eq1_e1032_d_n9, eq1_e1032_d_n10, eq1_e1032_d_n11, eq1_e1032_d_n12, eq1_e1032_d_n13, eq1_e1032_d_n14, eq1_e1032_d_n15, eq1_e1032_d_n16, eq1_e1032_d_n17, eq1_e1032_d_n18];let eq1_branch_derivatives: [f64; 13] = [eq1_e1032_d_b0, eq1_e1032_d_b1, eq1_e1032_d_b2, eq1_e1032_d_b3, eq1_e1032_d_b4, eq1_e1032_d_b5, eq1_e1032_d_b6, eq1_e1032_d_b7, eq1_e1032_d_b8, eq1_e1032_d_b9, eq1_e1032_d_b10, eq1_e1032_d_b11, eq1_e1032_d_b12];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1039, eq2_e1039_d_n0, eq2_e1039_d_n1, eq2_e1039_d_n2, eq2_e1039_d_n3, eq2_e1039_d_n4, eq2_e1039_d_n5, eq2_e1039_d_n6, eq2_e1039_d_n7, eq2_e1039_d_n8, eq2_e1039_d_n9, eq2_e1039_d_n10, eq2_e1039_d_n11, eq2_e1039_d_n12, eq2_e1039_d_n13, eq2_e1039_d_n14, eq2_e1039_d_n15, eq2_e1039_d_n16, eq2_e1039_d_n17, eq2_e1039_d_n18, eq2_e1039_d_b0, eq2_e1039_d_b1, eq2_e1039_d_b2, eq2_e1039_d_b3, eq2_e1039_d_b4, eq2_e1039_d_b5, eq2_e1039_d_b6, eq2_e1039_d_b7, eq2_e1039_d_b8, eq2_e1039_d_b9, eq2_e1039_d_b10, eq2_e1039_d_b11, eq2_e1039_d_b12,) = {
    if s.b[3307] {
        let eq2_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[925]);let eq2_e1037: f64 = (s.v[927] + eq2_e1036);let eq2_e1037_d_n0: f64 = (s.dn[927][0] + (s.dn[925][0] * ddt_scale));let eq2_e1037_d_n1: f64 = (s.dn[927][1] + (s.dn[925][1] * ddt_scale));let eq2_e1037_d_n2: f64 = (s.dn[927][2] + (s.dn[925][2] * ddt_scale));let eq2_e1037_d_n3: f64 = (s.dn[927][3] + (s.dn[925][3] * ddt_scale));let eq2_e1037_d_n4: f64 = (s.dn[927][4] + (s.dn[925][4] * ddt_scale));let eq2_e1037_d_n5: f64 = (s.dn[927][5] + (s.dn[925][5] * ddt_scale));let eq2_e1037_d_n6: f64 = (s.dn[927][6] + (s.dn[925][6] * ddt_scale));let eq2_e1037_d_n7: f64 = (s.dn[927][7] + (s.dn[925][7] * ddt_scale));let eq2_e1037_d_n8: f64 = (s.dn[927][8] + (s.dn[925][8] * ddt_scale));let eq2_e1037_d_n9: f64 = (s.dn[927][9] + (s.dn[925][9] * ddt_scale));let eq2_e1037_d_n10: f64 = (s.dn[927][10] + (s.dn[925][10] * ddt_scale));let eq2_e1037_d_n11: f64 = (s.dn[927][11] + (s.dn[925][11] * ddt_scale));let eq2_e1037_d_n12: f64 = (s.dn[927][12] + (s.dn[925][12] * ddt_scale));let eq2_e1037_d_n13: f64 = (s.dn[927][13] + (s.dn[925][13] * ddt_scale));let eq2_e1037_d_n14: f64 = (s.dn[927][14] + (s.dn[925][14] * ddt_scale));let eq2_e1037_d_n15: f64 = (s.dn[927][15] + (s.dn[925][15] * ddt_scale));let eq2_e1037_d_n16: f64 = (s.dn[927][16] + (s.dn[925][16] * ddt_scale));let eq2_e1037_d_n17: f64 = (s.dn[927][17] + (s.dn[925][17] * ddt_scale));let eq2_e1037_d_n18: f64 = (s.dn[927][18] + (s.dn[925][18] * ddt_scale));let eq2_e1037_d_b0: f64 = (s.db[927][0] + (s.db[925][0] * ddt_scale));let eq2_e1037_d_b1: f64 = (s.db[927][1] + (s.db[925][1] * ddt_scale));let eq2_e1037_d_b2: f64 = (s.db[927][2] + (s.db[925][2] * ddt_scale));let eq2_e1037_d_b3: f64 = (s.db[927][3] + (s.db[925][3] * ddt_scale));let eq2_e1037_d_b4: f64 = (s.db[927][4] + (s.db[925][4] * ddt_scale));let eq2_e1037_d_b5: f64 = (s.db[927][5] + (s.db[925][5] * ddt_scale));let eq2_e1037_d_b6: f64 = (s.db[927][6] + (s.db[925][6] * ddt_scale));let eq2_e1037_d_b7: f64 = (s.db[927][7] + (s.db[925][7] * ddt_scale));let eq2_e1037_d_b8: f64 = (s.db[927][8] + (s.db[925][8] * ddt_scale));let eq2_e1037_d_b9: f64 = (s.db[927][9] + (s.db[925][9] * ddt_scale));let eq2_e1037_d_b10: f64 = (s.db[927][10] + (s.db[925][10] * ddt_scale));let eq2_e1037_d_b11: f64 = (s.db[927][11] + (s.db[925][11] * ddt_scale));let eq2_e1037_d_b12: f64 = (s.db[927][12] + (s.db[925][12] * ddt_scale));
        (eq2_e1037, eq2_e1037_d_n0, eq2_e1037_d_n1, eq2_e1037_d_n2, eq2_e1037_d_n3, eq2_e1037_d_n4, eq2_e1037_d_n5, eq2_e1037_d_n6, eq2_e1037_d_n7, eq2_e1037_d_n8, eq2_e1037_d_n9, eq2_e1037_d_n10, eq2_e1037_d_n11, eq2_e1037_d_n12, eq2_e1037_d_n13, eq2_e1037_d_n14, eq2_e1037_d_n15, eq2_e1037_d_n16, eq2_e1037_d_n17, eq2_e1037_d_n18, eq2_e1037_d_b0, eq2_e1037_d_b1, eq2_e1037_d_b2, eq2_e1037_d_b3, eq2_e1037_d_b4, eq2_e1037_d_b5, eq2_e1037_d_b6, eq2_e1037_d_b7, eq2_e1037_d_b8, eq2_e1037_d_b9, eq2_e1037_d_b10, eq2_e1037_d_b11, eq2_e1037_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1039;let eq2_node_derivatives: [f64; 19] = [eq2_e1039_d_n0, eq2_e1039_d_n1, eq2_e1039_d_n2, eq2_e1039_d_n3, eq2_e1039_d_n4, eq2_e1039_d_n5, eq2_e1039_d_n6, eq2_e1039_d_n7, eq2_e1039_d_n8, eq2_e1039_d_n9, eq2_e1039_d_n10, eq2_e1039_d_n11, eq2_e1039_d_n12, eq2_e1039_d_n13, eq2_e1039_d_n14, eq2_e1039_d_n15, eq2_e1039_d_n16, eq2_e1039_d_n17, eq2_e1039_d_n18];let eq2_branch_derivatives: [f64; 13] = [eq2_e1039_d_b0, eq2_e1039_d_b1, eq2_e1039_d_b2, eq2_e1039_d_b3, eq2_e1039_d_b4, eq2_e1039_d_b5, eq2_e1039_d_b6, eq2_e1039_d_b7, eq2_e1039_d_b8, eq2_e1039_d_b9, eq2_e1039_d_b10, eq2_e1039_d_b11, eq2_e1039_d_b12];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1044,) = {
    if (!s.b[3307]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1044;
        stamper.stamp_potential_const_local(
            1,
            eq3_value,
        );
        let (eq4_e1049,) = {
    if (!s.b[3307]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1049;
        stamper.stamp_potential_const_local(
            2,
            eq4_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq5_e1056, eq5_e1056_d_n0, eq5_e1056_d_n1, eq5_e1056_d_n2, eq5_e1056_d_n3, eq5_e1056_d_n4, eq5_e1056_d_n5, eq5_e1056_d_n6, eq5_e1056_d_n7, eq5_e1056_d_n8, eq5_e1056_d_n9, eq5_e1056_d_n10, eq5_e1056_d_n11, eq5_e1056_d_n12, eq5_e1056_d_n13, eq5_e1056_d_n14, eq5_e1056_d_n15, eq5_e1056_d_n16, eq5_e1056_d_n17, eq5_e1056_d_n18, eq5_e1056_d_b0, eq5_e1056_d_b1, eq5_e1056_d_b2, eq5_e1056_d_b3, eq5_e1056_d_b4, eq5_e1056_d_b5, eq5_e1056_d_b6, eq5_e1056_d_b7, eq5_e1056_d_b8, eq5_e1056_d_b9, eq5_e1056_d_b10, eq5_e1056_d_b11, eq5_e1056_d_b12,) = {
    if s.b[3308] {
        let eq5_e1053: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[931]);let eq5_e1054: f64 = (s.v[932] + eq5_e1053);let eq5_e1054_d_n0: f64 = (s.dn[932][0] + (s.dn[931][0] * ddt_scale));let eq5_e1054_d_n1: f64 = (s.dn[932][1] + (s.dn[931][1] * ddt_scale));let eq5_e1054_d_n2: f64 = (s.dn[932][2] + (s.dn[931][2] * ddt_scale));let eq5_e1054_d_n3: f64 = (s.dn[932][3] + (s.dn[931][3] * ddt_scale));let eq5_e1054_d_n4: f64 = (s.dn[932][4] + (s.dn[931][4] * ddt_scale));let eq5_e1054_d_n5: f64 = (s.dn[932][5] + (s.dn[931][5] * ddt_scale));let eq5_e1054_d_n6: f64 = (s.dn[932][6] + (s.dn[931][6] * ddt_scale));let eq5_e1054_d_n7: f64 = (s.dn[932][7] + (s.dn[931][7] * ddt_scale));let eq5_e1054_d_n8: f64 = (s.dn[932][8] + (s.dn[931][8] * ddt_scale));let eq5_e1054_d_n9: f64 = (s.dn[932][9] + (s.dn[931][9] * ddt_scale));let eq5_e1054_d_n10: f64 = (s.dn[932][10] + (s.dn[931][10] * ddt_scale));let eq5_e1054_d_n11: f64 = (s.dn[932][11] + (s.dn[931][11] * ddt_scale));let eq5_e1054_d_n12: f64 = (s.dn[932][12] + (s.dn[931][12] * ddt_scale));let eq5_e1054_d_n13: f64 = (s.dn[932][13] + (s.dn[931][13] * ddt_scale));let eq5_e1054_d_n14: f64 = (s.dn[932][14] + (s.dn[931][14] * ddt_scale));let eq5_e1054_d_n15: f64 = (s.dn[932][15] + (s.dn[931][15] * ddt_scale));let eq5_e1054_d_n16: f64 = (s.dn[932][16] + (s.dn[931][16] * ddt_scale));let eq5_e1054_d_n17: f64 = (s.dn[932][17] + (s.dn[931][17] * ddt_scale));let eq5_e1054_d_n18: f64 = (s.dn[932][18] + (s.dn[931][18] * ddt_scale));let eq5_e1054_d_b0: f64 = (s.db[932][0] + (s.db[931][0] * ddt_scale));let eq5_e1054_d_b1: f64 = (s.db[932][1] + (s.db[931][1] * ddt_scale));let eq5_e1054_d_b2: f64 = (s.db[932][2] + (s.db[931][2] * ddt_scale));let eq5_e1054_d_b3: f64 = (s.db[932][3] + (s.db[931][3] * ddt_scale));let eq5_e1054_d_b4: f64 = (s.db[932][4] + (s.db[931][4] * ddt_scale));let eq5_e1054_d_b5: f64 = (s.db[932][5] + (s.db[931][5] * ddt_scale));let eq5_e1054_d_b6: f64 = (s.db[932][6] + (s.db[931][6] * ddt_scale));let eq5_e1054_d_b7: f64 = (s.db[932][7] + (s.db[931][7] * ddt_scale));let eq5_e1054_d_b8: f64 = (s.db[932][8] + (s.db[931][8] * ddt_scale));let eq5_e1054_d_b9: f64 = (s.db[932][9] + (s.db[931][9] * ddt_scale));let eq5_e1054_d_b10: f64 = (s.db[932][10] + (s.db[931][10] * ddt_scale));let eq5_e1054_d_b11: f64 = (s.db[932][11] + (s.db[931][11] * ddt_scale));let eq5_e1054_d_b12: f64 = (s.db[932][12] + (s.db[931][12] * ddt_scale));
        (eq5_e1054, eq5_e1054_d_n0, eq5_e1054_d_n1, eq5_e1054_d_n2, eq5_e1054_d_n3, eq5_e1054_d_n4, eq5_e1054_d_n5, eq5_e1054_d_n6, eq5_e1054_d_n7, eq5_e1054_d_n8, eq5_e1054_d_n9, eq5_e1054_d_n10, eq5_e1054_d_n11, eq5_e1054_d_n12, eq5_e1054_d_n13, eq5_e1054_d_n14, eq5_e1054_d_n15, eq5_e1054_d_n16, eq5_e1054_d_n17, eq5_e1054_d_n18, eq5_e1054_d_b0, eq5_e1054_d_b1, eq5_e1054_d_b2, eq5_e1054_d_b3, eq5_e1054_d_b4, eq5_e1054_d_b5, eq5_e1054_d_b6, eq5_e1054_d_b7, eq5_e1054_d_b8, eq5_e1054_d_b9, eq5_e1054_d_b10, eq5_e1054_d_b11, eq5_e1054_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1056;let eq5_node_derivatives: [f64; 19] = [eq5_e1056_d_n0, eq5_e1056_d_n1, eq5_e1056_d_n2, eq5_e1056_d_n3, eq5_e1056_d_n4, eq5_e1056_d_n5, eq5_e1056_d_n6, eq5_e1056_d_n7, eq5_e1056_d_n8, eq5_e1056_d_n9, eq5_e1056_d_n10, eq5_e1056_d_n11, eq5_e1056_d_n12, eq5_e1056_d_n13, eq5_e1056_d_n14, eq5_e1056_d_n15, eq5_e1056_d_n16, eq5_e1056_d_n17, eq5_e1056_d_n18];let eq5_branch_derivatives: [f64; 13] = [eq5_e1056_d_b0, eq5_e1056_d_b1, eq5_e1056_d_b2, eq5_e1056_d_b3, eq5_e1056_d_b4, eq5_e1056_d_b5, eq5_e1056_d_b6, eq5_e1056_d_b7, eq5_e1056_d_b8, eq5_e1056_d_b9, eq5_e1056_d_b10, eq5_e1056_d_b11, eq5_e1056_d_b12];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1061,) = {
    if (!s.b[3308]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1061;
        stamper.stamp_potential_const_local(
            3,
            eq6_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq7_e1065: f64 = (s.v[134] + s.v[400]);let eq7_e1065_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);let eq7_e1065_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);let eq7_e1065_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);let eq7_e1065_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);let eq7_e1065_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);let eq7_e1065_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);let eq7_e1065_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);let eq7_e1065_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);let eq7_e1065_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);let eq7_e1065_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);let eq7_e1065_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);let eq7_e1065_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);let eq7_e1065_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);let eq7_e1065_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);let eq7_e1065_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);let eq7_e1065_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);let eq7_e1065_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);let eq7_e1065_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);let eq7_e1065_d_n18: f64 = (s.dn[134][18] + s.dn[400][18]);let eq7_e1065_d_b0: f64 = (s.db[134][0] + s.db[400][0]);let eq7_e1065_d_b1: f64 = (s.db[134][1] + s.db[400][1]);let eq7_e1065_d_b2: f64 = (s.db[134][2] + s.db[400][2]);let eq7_e1065_d_b3: f64 = (s.db[134][3] + s.db[400][3]);let eq7_e1065_d_b4: f64 = (s.db[134][4] + s.db[400][4]);let eq7_e1065_d_b5: f64 = (s.db[134][5] + s.db[400][5]);let eq7_e1065_d_b6: f64 = (s.db[134][6] + s.db[400][6]);let eq7_e1065_d_b7: f64 = (s.db[134][7] + s.db[400][7]);let eq7_e1065_d_b8: f64 = (s.db[134][8] + s.db[400][8]);let eq7_e1065_d_b9: f64 = (s.db[134][9] + s.db[400][9]);let eq7_e1065_d_b10: f64 = (s.db[134][10] + s.db[400][10]);let eq7_e1065_d_b11: f64 = (s.db[134][11] + s.db[400][11]);let eq7_e1065_d_b12: f64 = (s.db[134][12] + s.db[400][12]);let eq7_e1067: f64 = (eq7_e1065 - s.v[738]);let eq7_e1067_d_n0: f64 = (eq7_e1065_d_n0 - s.dn[738][0]);let eq7_e1067_d_n1: f64 = (eq7_e1065_d_n1 - s.dn[738][1]);let eq7_e1067_d_n2: f64 = (eq7_e1065_d_n2 - s.dn[738][2]);let eq7_e1067_d_n3: f64 = (eq7_e1065_d_n3 - s.dn[738][3]);let eq7_e1067_d_n4: f64 = (eq7_e1065_d_n4 - s.dn[738][4]);let eq7_e1067_d_n5: f64 = (eq7_e1065_d_n5 - s.dn[738][5]);let eq7_e1067_d_n6: f64 = (eq7_e1065_d_n6 - s.dn[738][6]);let eq7_e1067_d_n7: f64 = (eq7_e1065_d_n7 - s.dn[738][7]);let eq7_e1067_d_n8: f64 = (eq7_e1065_d_n8 - s.dn[738][8]);let eq7_e1067_d_n9: f64 = (eq7_e1065_d_n9 - s.dn[738][9]);let eq7_e1067_d_n10: f64 = (eq7_e1065_d_n10 - s.dn[738][10]);let eq7_e1067_d_n11: f64 = (eq7_e1065_d_n11 - s.dn[738][11]);let eq7_e1067_d_n12: f64 = (eq7_e1065_d_n12 - s.dn[738][12]);let eq7_e1067_d_n13: f64 = (eq7_e1065_d_n13 - s.dn[738][13]);let eq7_e1067_d_n14: f64 = (eq7_e1065_d_n14 - s.dn[738][14]);let eq7_e1067_d_n15: f64 = (eq7_e1065_d_n15 - s.dn[738][15]);let eq7_e1067_d_n16: f64 = (eq7_e1065_d_n16 - s.dn[738][16]);let eq7_e1067_d_n17: f64 = (eq7_e1065_d_n17 - s.dn[738][17]);let eq7_e1067_d_n18: f64 = (eq7_e1065_d_n18 - s.dn[738][18]);let eq7_e1067_d_b0: f64 = (eq7_e1065_d_b0 - s.db[738][0]);let eq7_e1067_d_b1: f64 = (eq7_e1065_d_b1 - s.db[738][1]);let eq7_e1067_d_b2: f64 = (eq7_e1065_d_b2 - s.db[738][2]);let eq7_e1067_d_b3: f64 = (eq7_e1065_d_b3 - s.db[738][3]);let eq7_e1067_d_b4: f64 = (eq7_e1065_d_b4 - s.db[738][4]);let eq7_e1067_d_b5: f64 = (eq7_e1065_d_b5 - s.db[738][5]);let eq7_e1067_d_b6: f64 = (eq7_e1065_d_b6 - s.db[738][6]);let eq7_e1067_d_b7: f64 = (eq7_e1065_d_b7 - s.db[738][7]);let eq7_e1067_d_b8: f64 = (eq7_e1065_d_b8 - s.db[738][8]);let eq7_e1067_d_b9: f64 = (eq7_e1065_d_b9 - s.db[738][9]);let eq7_e1067_d_b10: f64 = (eq7_e1065_d_b10 - s.db[738][10]);let eq7_e1067_d_b11: f64 = (eq7_e1065_d_b11 - s.db[738][11]);let eq7_e1067_d_b12: f64 = (eq7_e1065_d_b12 - s.db[738][12]);let eq7_e1068: f64 = (p[87] * eq7_e1067);let eq7_e1068_d_n0: f64 = (p[87] * eq7_e1067_d_n0);let eq7_e1068_d_n1: f64 = (p[87] * eq7_e1067_d_n1);let eq7_e1068_d_n2: f64 = (p[87] * eq7_e1067_d_n2);let eq7_e1068_d_n3: f64 = (p[87] * eq7_e1067_d_n3);
        let eq7_e1068_d_n4: f64 = (p[87] * eq7_e1067_d_n4);let eq7_e1068_d_n5: f64 = (p[87] * eq7_e1067_d_n5);let eq7_e1068_d_n6: f64 = (p[87] * eq7_e1067_d_n6);let eq7_e1068_d_n7: f64 = (p[87] * eq7_e1067_d_n7);let eq7_e1068_d_n8: f64 = (p[87] * eq7_e1067_d_n8);let eq7_e1068_d_n9: f64 = (p[87] * eq7_e1067_d_n9);let eq7_e1068_d_n10: f64 = (p[87] * eq7_e1067_d_n10);let eq7_e1068_d_n11: f64 = (p[87] * eq7_e1067_d_n11);let eq7_e1068_d_n12: f64 = (p[87] * eq7_e1067_d_n12);let eq7_e1068_d_n13: f64 = (p[87] * eq7_e1067_d_n13);let eq7_e1068_d_n14: f64 = (p[87] * eq7_e1067_d_n14);let eq7_e1068_d_n15: f64 = (p[87] * eq7_e1067_d_n15);let eq7_e1068_d_n16: f64 = (p[87] * eq7_e1067_d_n16);let eq7_e1068_d_n17: f64 = (p[87] * eq7_e1067_d_n17);let eq7_e1068_d_n18: f64 = (p[87] * eq7_e1067_d_n18);let eq7_e1068_d_b0: f64 = (p[87] * eq7_e1067_d_b0);let eq7_e1068_d_b1: f64 = (p[87] * eq7_e1067_d_b1);let eq7_e1068_d_b2: f64 = (p[87] * eq7_e1067_d_b2);let eq7_e1068_d_b3: f64 = (p[87] * eq7_e1067_d_b3);let eq7_e1068_d_b4: f64 = (p[87] * eq7_e1067_d_b4);let eq7_e1068_d_b5: f64 = (p[87] * eq7_e1067_d_b5);let eq7_e1068_d_b6: f64 = (p[87] * eq7_e1067_d_b6);let eq7_e1068_d_b7: f64 = (p[87] * eq7_e1067_d_b7);let eq7_e1068_d_b8: f64 = (p[87] * eq7_e1067_d_b8);let eq7_e1068_d_b9: f64 = (p[87] * eq7_e1067_d_b9);let eq7_e1068_d_b10: f64 = (p[87] * eq7_e1067_d_b10);let eq7_e1068_d_b11: f64 = (p[87] * eq7_e1067_d_b11);let eq7_e1068_d_b12: f64 = (p[87] * eq7_e1067_d_b12);let eq7_value: f64 = eq7_e1068;let eq7_node_derivatives: [f64; 19] = [eq7_e1068_d_n0, eq7_e1068_d_n1, eq7_e1068_d_n2, eq7_e1068_d_n3, eq7_e1068_d_n4, eq7_e1068_d_n5, eq7_e1068_d_n6, eq7_e1068_d_n7, eq7_e1068_d_n8, eq7_e1068_d_n9, eq7_e1068_d_n10, eq7_e1068_d_n11, eq7_e1068_d_n12, eq7_e1068_d_n13, eq7_e1068_d_n14, eq7_e1068_d_n15, eq7_e1068_d_n16, eq7_e1068_d_n17, eq7_e1068_d_n18];let eq7_branch_derivatives: [f64; 13] = [eq7_e1068_d_b0, eq7_e1068_d_b1, eq7_e1068_d_b2, eq7_e1068_d_b3, eq7_e1068_d_b4, eq7_e1068_d_b5, eq7_e1068_d_b6, eq7_e1068_d_b7, eq7_e1068_d_b8, eq7_e1068_d_b9, eq7_e1068_d_b10, eq7_e1068_d_b11, eq7_e1068_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq8_e1072: f64 = (s.v[424] - s.v[425]);let eq8_e1072_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);let eq8_e1072_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);let eq8_e1072_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);let eq8_e1072_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);let eq8_e1072_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);let eq8_e1072_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);let eq8_e1072_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);let eq8_e1072_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);let eq8_e1072_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);let eq8_e1072_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);let eq8_e1072_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);let eq8_e1072_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);let eq8_e1072_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);let eq8_e1072_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);let eq8_e1072_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);let eq8_e1072_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);let eq8_e1072_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);let eq8_e1072_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);let eq8_e1072_d_n18: f64 = (s.dn[424][18] - s.dn[425][18]);let eq8_e1072_d_b0: f64 = (s.db[424][0] - s.db[425][0]);let eq8_e1072_d_b1: f64 = (s.db[424][1] - s.db[425][1]);let eq8_e1072_d_b2: f64 = (s.db[424][2] - s.db[425][2]);let eq8_e1072_d_b3: f64 = (s.db[424][3] - s.db[425][3]);let eq8_e1072_d_b4: f64 = (s.db[424][4] - s.db[425][4]);let eq8_e1072_d_b5: f64 = (s.db[424][5] - s.db[425][5]);let eq8_e1072_d_b6: f64 = (s.db[424][6] - s.db[425][6]);let eq8_e1072_d_b7: f64 = (s.db[424][7] - s.db[425][7]);let eq8_e1072_d_b8: f64 = (s.db[424][8] - s.db[425][8]);let eq8_e1072_d_b9: f64 = (s.db[424][9] - s.db[425][9]);let eq8_e1072_d_b10: f64 = (s.db[424][10] - s.db[425][10]);let eq8_e1072_d_b11: f64 = (s.db[424][11] - s.db[425][11]);let eq8_e1072_d_b12: f64 = (s.db[424][12] - s.db[425][12]);let eq8_e1073: f64 = (p[87] * eq8_e1072);let eq8_e1073_d_n0: f64 = (p[87] * eq8_e1072_d_n0);let eq8_e1073_d_n1: f64 = (p[87] * eq8_e1072_d_n1);let eq8_e1073_d_n2: f64 = (p[87] * eq8_e1072_d_n2);let eq8_e1073_d_n3: f64 = (p[87] * eq8_e1072_d_n3);let eq8_e1073_d_n4: f64 = (p[87] * eq8_e1072_d_n4);let eq8_e1073_d_n5: f64 = (p[87] * eq8_e1072_d_n5);let eq8_e1073_d_n6: f64 = (p[87] * eq8_e1072_d_n6);let eq8_e1073_d_n7: f64 = (p[87] * eq8_e1072_d_n7);let eq8_e1073_d_n8: f64 = (p[87] * eq8_e1072_d_n8);let eq8_e1073_d_n9: f64 = (p[87] * eq8_e1072_d_n9);let eq8_e1073_d_n10: f64 = (p[87] * eq8_e1072_d_n10);let eq8_e1073_d_n11: f64 = (p[87] * eq8_e1072_d_n11);let eq8_e1073_d_n12: f64 = (p[87] * eq8_e1072_d_n12);let eq8_e1073_d_n13: f64 = (p[87] * eq8_e1072_d_n13);let eq8_e1073_d_n14: f64 = (p[87] * eq8_e1072_d_n14);let eq8_e1073_d_n15: f64 = (p[87] * eq8_e1072_d_n15);let eq8_e1073_d_n16: f64 = (p[87] * eq8_e1072_d_n16);let eq8_e1073_d_n17: f64 = (p[87] * eq8_e1072_d_n17);let eq8_e1073_d_n18: f64 = (p[87] * eq8_e1072_d_n18);let eq8_e1073_d_b0: f64 = (p[87] * eq8_e1072_d_b0);let eq8_e1073_d_b1: f64 = (p[87] * eq8_e1072_d_b1);let eq8_e1073_d_b2: f64 = (p[87] * eq8_e1072_d_b2);let eq8_e1073_d_b3: f64 = (p[87] * eq8_e1072_d_b3);let eq8_e1073_d_b4: f64 = (p[87] * eq8_e1072_d_b4);let eq8_e1073_d_b5: f64 = (p[87] * eq8_e1072_d_b5);let eq8_e1073_d_b6: f64 = (p[87] * eq8_e1072_d_b6);let eq8_e1073_d_b7: f64 = (p[87] * eq8_e1072_d_b7);let eq8_e1073_d_b8: f64 = (p[87] * eq8_e1072_d_b8);let eq8_e1073_d_b9: f64 = (p[87] * eq8_e1072_d_b9);let eq8_e1073_d_b10: f64 = (p[87] * eq8_e1072_d_b10);let eq8_e1073_d_b11: f64 = (p[87] * eq8_e1072_d_b11);let eq8_e1073_d_b12: f64 = (p[87] * eq8_e1072_d_b12);let eq8_value: f64 = eq8_e1073;let eq8_node_derivatives: [f64; 19] = [eq8_e1073_d_n0, eq8_e1073_d_n1, eq8_e1073_d_n2, eq8_e1073_d_n3, eq8_e1073_d_n4, eq8_e1073_d_n5, eq8_e1073_d_n6, eq8_e1073_d_n7, eq8_e1073_d_n8, eq8_e1073_d_n9, eq8_e1073_d_n10, eq8_e1073_d_n11, eq8_e1073_d_n12, eq8_e1073_d_n13, eq8_e1073_d_n14, eq8_e1073_d_n15, eq8_e1073_d_n16, eq8_e1073_d_n17, eq8_e1073_d_n18];
        let eq8_branch_derivatives: [f64; 13] = [eq8_e1073_d_b0, eq8_e1073_d_b1, eq8_e1073_d_b2, eq8_e1073_d_b3, eq8_e1073_d_b4, eq8_e1073_d_b5, eq8_e1073_d_b6, eq8_e1073_d_b7, eq8_e1073_d_b8, eq8_e1073_d_b9, eq8_e1073_d_b10, eq8_e1073_d_b11, eq8_e1073_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq9_e1077: f64 = (s.v[203] + s.v[280]);let eq9_e1077_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);let eq9_e1077_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);let eq9_e1077_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);let eq9_e1077_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);let eq9_e1077_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);let eq9_e1077_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);let eq9_e1077_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);let eq9_e1077_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);let eq9_e1077_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);let eq9_e1077_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);let eq9_e1077_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);let eq9_e1077_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);let eq9_e1077_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);let eq9_e1077_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);let eq9_e1077_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);let eq9_e1077_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);let eq9_e1077_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);let eq9_e1077_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);let eq9_e1077_d_n18: f64 = (s.dn[203][18] + s.dn[280][18]);let eq9_e1077_d_b0: f64 = (s.db[203][0] + s.db[280][0]);let eq9_e1077_d_b1: f64 = (s.db[203][1] + s.db[280][1]);let eq9_e1077_d_b2: f64 = (s.db[203][2] + s.db[280][2]);let eq9_e1077_d_b3: f64 = (s.db[203][3] + s.db[280][3]);let eq9_e1077_d_b4: f64 = (s.db[203][4] + s.db[280][4]);let eq9_e1077_d_b5: f64 = (s.db[203][5] + s.db[280][5]);let eq9_e1077_d_b6: f64 = (s.db[203][6] + s.db[280][6]);let eq9_e1077_d_b7: f64 = (s.db[203][7] + s.db[280][7]);let eq9_e1077_d_b8: f64 = (s.db[203][8] + s.db[280][8]);let eq9_e1077_d_b9: f64 = (s.db[203][9] + s.db[280][9]);let eq9_e1077_d_b10: f64 = (s.db[203][10] + s.db[280][10]);let eq9_e1077_d_b11: f64 = (s.db[203][11] + s.db[280][11]);let eq9_e1077_d_b12: f64 = (s.db[203][12] + s.db[280][12]);let eq9_e1079: f64 = (eq9_e1077 + s.v[431]);let eq9_e1079_d_n0: f64 = (eq9_e1077_d_n0 + s.dn[431][0]);let eq9_e1079_d_n1: f64 = (eq9_e1077_d_n1 + s.dn[431][1]);let eq9_e1079_d_n2: f64 = (eq9_e1077_d_n2 + s.dn[431][2]);let eq9_e1079_d_n3: f64 = (eq9_e1077_d_n3 + s.dn[431][3]);let eq9_e1079_d_n4: f64 = (eq9_e1077_d_n4 + s.dn[431][4]);let eq9_e1079_d_n5: f64 = (eq9_e1077_d_n5 + s.dn[431][5]);let eq9_e1079_d_n6: f64 = (eq9_e1077_d_n6 + s.dn[431][6]);let eq9_e1079_d_n7: f64 = (eq9_e1077_d_n7 + s.dn[431][7]);let eq9_e1079_d_n8: f64 = (eq9_e1077_d_n8 + s.dn[431][8]);let eq9_e1079_d_n9: f64 = (eq9_e1077_d_n9 + s.dn[431][9]);let eq9_e1079_d_n10: f64 = (eq9_e1077_d_n10 + s.dn[431][10]);let eq9_e1079_d_n11: f64 = (eq9_e1077_d_n11 + s.dn[431][11]);let eq9_e1079_d_n12: f64 = (eq9_e1077_d_n12 + s.dn[431][12]);let eq9_e1079_d_n13: f64 = (eq9_e1077_d_n13 + s.dn[431][13]);let eq9_e1079_d_n14: f64 = (eq9_e1077_d_n14 + s.dn[431][14]);let eq9_e1079_d_n15: f64 = (eq9_e1077_d_n15 + s.dn[431][15]);let eq9_e1079_d_n16: f64 = (eq9_e1077_d_n16 + s.dn[431][16]);let eq9_e1079_d_n17: f64 = (eq9_e1077_d_n17 + s.dn[431][17]);let eq9_e1079_d_n18: f64 = (eq9_e1077_d_n18 + s.dn[431][18]);let eq9_e1079_d_b0: f64 = (eq9_e1077_d_b0 + s.db[431][0]);let eq9_e1079_d_b1: f64 = (eq9_e1077_d_b1 + s.db[431][1]);let eq9_e1079_d_b2: f64 = (eq9_e1077_d_b2 + s.db[431][2]);let eq9_e1079_d_b3: f64 = (eq9_e1077_d_b3 + s.db[431][3]);let eq9_e1079_d_b4: f64 = (eq9_e1077_d_b4 + s.db[431][4]);let eq9_e1079_d_b5: f64 = (eq9_e1077_d_b5 + s.db[431][5]);let eq9_e1079_d_b6: f64 = (eq9_e1077_d_b6 + s.db[431][6]);let eq9_e1079_d_b7: f64 = (eq9_e1077_d_b7 + s.db[431][7]);let eq9_e1079_d_b8: f64 = (eq9_e1077_d_b8 + s.db[431][8]);let eq9_e1079_d_b9: f64 = (eq9_e1077_d_b9 + s.db[431][9]);let eq9_e1079_d_b10: f64 = (eq9_e1077_d_b10 + s.db[431][10]);let eq9_e1079_d_b11: f64 = (eq9_e1077_d_b11 + s.db[431][11]);let eq9_e1079_d_b12: f64 = (eq9_e1077_d_b12 + s.db[431][12]);let eq9_e1080: f64 = (p[87] * eq9_e1079);let eq9_e1080_d_n0: f64 = (p[87] * eq9_e1079_d_n0);let eq9_e1080_d_n1: f64 = (p[87] * eq9_e1079_d_n1);let eq9_e1080_d_n2: f64 = (p[87] * eq9_e1079_d_n2);let eq9_e1080_d_n3: f64 = (p[87] * eq9_e1079_d_n3);
        let eq9_e1080_d_n4: f64 = (p[87] * eq9_e1079_d_n4);let eq9_e1080_d_n5: f64 = (p[87] * eq9_e1079_d_n5);let eq9_e1080_d_n6: f64 = (p[87] * eq9_e1079_d_n6);let eq9_e1080_d_n7: f64 = (p[87] * eq9_e1079_d_n7);let eq9_e1080_d_n8: f64 = (p[87] * eq9_e1079_d_n8);let eq9_e1080_d_n9: f64 = (p[87] * eq9_e1079_d_n9);let eq9_e1080_d_n10: f64 = (p[87] * eq9_e1079_d_n10);let eq9_e1080_d_n11: f64 = (p[87] * eq9_e1079_d_n11);let eq9_e1080_d_n12: f64 = (p[87] * eq9_e1079_d_n12);let eq9_e1080_d_n13: f64 = (p[87] * eq9_e1079_d_n13);let eq9_e1080_d_n14: f64 = (p[87] * eq9_e1079_d_n14);let eq9_e1080_d_n15: f64 = (p[87] * eq9_e1079_d_n15);let eq9_e1080_d_n16: f64 = (p[87] * eq9_e1079_d_n16);let eq9_e1080_d_n17: f64 = (p[87] * eq9_e1079_d_n17);let eq9_e1080_d_n18: f64 = (p[87] * eq9_e1079_d_n18);let eq9_e1080_d_b0: f64 = (p[87] * eq9_e1079_d_b0);let eq9_e1080_d_b1: f64 = (p[87] * eq9_e1079_d_b1);let eq9_e1080_d_b2: f64 = (p[87] * eq9_e1079_d_b2);let eq9_e1080_d_b3: f64 = (p[87] * eq9_e1079_d_b3);let eq9_e1080_d_b4: f64 = (p[87] * eq9_e1079_d_b4);let eq9_e1080_d_b5: f64 = (p[87] * eq9_e1079_d_b5);let eq9_e1080_d_b6: f64 = (p[87] * eq9_e1079_d_b6);let eq9_e1080_d_b7: f64 = (p[87] * eq9_e1079_d_b7);let eq9_e1080_d_b8: f64 = (p[87] * eq9_e1079_d_b8);let eq9_e1080_d_b9: f64 = (p[87] * eq9_e1079_d_b9);let eq9_e1080_d_b10: f64 = (p[87] * eq9_e1079_d_b10);let eq9_e1080_d_b11: f64 = (p[87] * eq9_e1079_d_b11);let eq9_e1080_d_b12: f64 = (p[87] * eq9_e1079_d_b12);let eq9_value: f64 = eq9_e1080;let eq9_node_derivatives: [f64; 19] = [eq9_e1080_d_n0, eq9_e1080_d_n1, eq9_e1080_d_n2, eq9_e1080_d_n3, eq9_e1080_d_n4, eq9_e1080_d_n5, eq9_e1080_d_n6, eq9_e1080_d_n7, eq9_e1080_d_n8, eq9_e1080_d_n9, eq9_e1080_d_n10, eq9_e1080_d_n11, eq9_e1080_d_n12, eq9_e1080_d_n13, eq9_e1080_d_n14, eq9_e1080_d_n15, eq9_e1080_d_n16, eq9_e1080_d_n17, eq9_e1080_d_n18];let eq9_branch_derivatives: [f64; 13] = [eq9_e1080_d_b0, eq9_e1080_d_b1, eq9_e1080_d_b2, eq9_e1080_d_b3, eq9_e1080_d_b4, eq9_e1080_d_b5, eq9_e1080_d_b6, eq9_e1080_d_b7, eq9_e1080_d_b8, eq9_e1080_d_b9, eq9_e1080_d_b10, eq9_e1080_d_b11, eq9_e1080_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq10_e1084: f64 = (s.v[204] + s.v[736]);let eq10_e1084_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);let eq10_e1084_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);let eq10_e1084_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);let eq10_e1084_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);let eq10_e1084_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);let eq10_e1084_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);let eq10_e1084_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);let eq10_e1084_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);let eq10_e1084_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);let eq10_e1084_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);let eq10_e1084_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);let eq10_e1084_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);let eq10_e1084_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);let eq10_e1084_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);let eq10_e1084_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);let eq10_e1084_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);let eq10_e1084_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);let eq10_e1084_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);let eq10_e1084_d_n18: f64 = (s.dn[204][18] + s.dn[736][18]);let eq10_e1084_d_b0: f64 = (s.db[204][0] + s.db[736][0]);let eq10_e1084_d_b1: f64 = (s.db[204][1] + s.db[736][1]);let eq10_e1084_d_b2: f64 = (s.db[204][2] + s.db[736][2]);let eq10_e1084_d_b3: f64 = (s.db[204][3] + s.db[736][3]);let eq10_e1084_d_b4: f64 = (s.db[204][4] + s.db[736][4]);let eq10_e1084_d_b5: f64 = (s.db[204][5] + s.db[736][5]);let eq10_e1084_d_b6: f64 = (s.db[204][6] + s.db[736][6]);let eq10_e1084_d_b7: f64 = (s.db[204][7] + s.db[736][7]);let eq10_e1084_d_b8: f64 = (s.db[204][8] + s.db[736][8]);let eq10_e1084_d_b9: f64 = (s.db[204][9] + s.db[736][9]);let eq10_e1084_d_b10: f64 = (s.db[204][10] + s.db[736][10]);let eq10_e1084_d_b11: f64 = (s.db[204][11] + s.db[736][11]);let eq10_e1084_d_b12: f64 = (s.db[204][12] + s.db[736][12]);let eq10_e1086: f64 = (eq10_e1084 + s.v[432]);let eq10_e1086_d_n0: f64 = (eq10_e1084_d_n0 + s.dn[432][0]);let eq10_e1086_d_n1: f64 = (eq10_e1084_d_n1 + s.dn[432][1]);let eq10_e1086_d_n2: f64 = (eq10_e1084_d_n2 + s.dn[432][2]);let eq10_e1086_d_n3: f64 = (eq10_e1084_d_n3 + s.dn[432][3]);let eq10_e1086_d_n4: f64 = (eq10_e1084_d_n4 + s.dn[432][4]);let eq10_e1086_d_n5: f64 = (eq10_e1084_d_n5 + s.dn[432][5]);let eq10_e1086_d_n6: f64 = (eq10_e1084_d_n6 + s.dn[432][6]);let eq10_e1086_d_n7: f64 = (eq10_e1084_d_n7 + s.dn[432][7]);let eq10_e1086_d_n8: f64 = (eq10_e1084_d_n8 + s.dn[432][8]);let eq10_e1086_d_n9: f64 = (eq10_e1084_d_n9 + s.dn[432][9]);let eq10_e1086_d_n10: f64 = (eq10_e1084_d_n10 + s.dn[432][10]);let eq10_e1086_d_n11: f64 = (eq10_e1084_d_n11 + s.dn[432][11]);let eq10_e1086_d_n12: f64 = (eq10_e1084_d_n12 + s.dn[432][12]);let eq10_e1086_d_n13: f64 = (eq10_e1084_d_n13 + s.dn[432][13]);let eq10_e1086_d_n14: f64 = (eq10_e1084_d_n14 + s.dn[432][14]);let eq10_e1086_d_n15: f64 = (eq10_e1084_d_n15 + s.dn[432][15]);let eq10_e1086_d_n16: f64 = (eq10_e1084_d_n16 + s.dn[432][16]);let eq10_e1086_d_n17: f64 = (eq10_e1084_d_n17 + s.dn[432][17]);let eq10_e1086_d_n18: f64 = (eq10_e1084_d_n18 + s.dn[432][18]);let eq10_e1086_d_b0: f64 = (eq10_e1084_d_b0 + s.db[432][0]);let eq10_e1086_d_b1: f64 = (eq10_e1084_d_b1 + s.db[432][1]);let eq10_e1086_d_b2: f64 = (eq10_e1084_d_b2 + s.db[432][2]);let eq10_e1086_d_b3: f64 = (eq10_e1084_d_b3 + s.db[432][3]);let eq10_e1086_d_b4: f64 = (eq10_e1084_d_b4 + s.db[432][4]);let eq10_e1086_d_b5: f64 = (eq10_e1084_d_b5 + s.db[432][5]);let eq10_e1086_d_b6: f64 = (eq10_e1084_d_b6 + s.db[432][6]);let eq10_e1086_d_b7: f64 = (eq10_e1084_d_b7 + s.db[432][7]);let eq10_e1086_d_b8: f64 = (eq10_e1084_d_b8 + s.db[432][8]);let eq10_e1086_d_b9: f64 = (eq10_e1084_d_b9 + s.db[432][9]);let eq10_e1086_d_b10: f64 = (eq10_e1084_d_b10 + s.db[432][10]);let eq10_e1086_d_b11: f64 = (eq10_e1084_d_b11 + s.db[432][11]);let eq10_e1086_d_b12: f64 = (eq10_e1084_d_b12 + s.db[432][12]);let eq10_e1087: f64 = (p[87] * eq10_e1086);let eq10_e1087_d_n0: f64 = (p[87] * eq10_e1086_d_n0);let eq10_e1087_d_n1: f64 = (p[87] * eq10_e1086_d_n1);
        let eq10_e1087_d_n2: f64 = (p[87] * eq10_e1086_d_n2);let eq10_e1087_d_n3: f64 = (p[87] * eq10_e1086_d_n3);let eq10_e1087_d_n4: f64 = (p[87] * eq10_e1086_d_n4);let eq10_e1087_d_n5: f64 = (p[87] * eq10_e1086_d_n5);let eq10_e1087_d_n6: f64 = (p[87] * eq10_e1086_d_n6);let eq10_e1087_d_n7: f64 = (p[87] * eq10_e1086_d_n7);let eq10_e1087_d_n8: f64 = (p[87] * eq10_e1086_d_n8);let eq10_e1087_d_n9: f64 = (p[87] * eq10_e1086_d_n9);let eq10_e1087_d_n10: f64 = (p[87] * eq10_e1086_d_n10);let eq10_e1087_d_n11: f64 = (p[87] * eq10_e1086_d_n11);let eq10_e1087_d_n12: f64 = (p[87] * eq10_e1086_d_n12);let eq10_e1087_d_n13: f64 = (p[87] * eq10_e1086_d_n13);let eq10_e1087_d_n14: f64 = (p[87] * eq10_e1086_d_n14);let eq10_e1087_d_n15: f64 = (p[87] * eq10_e1086_d_n15);let eq10_e1087_d_n16: f64 = (p[87] * eq10_e1086_d_n16);let eq10_e1087_d_n17: f64 = (p[87] * eq10_e1086_d_n17);let eq10_e1087_d_n18: f64 = (p[87] * eq10_e1086_d_n18);let eq10_e1087_d_b0: f64 = (p[87] * eq10_e1086_d_b0);let eq10_e1087_d_b1: f64 = (p[87] * eq10_e1086_d_b1);let eq10_e1087_d_b2: f64 = (p[87] * eq10_e1086_d_b2);let eq10_e1087_d_b3: f64 = (p[87] * eq10_e1086_d_b3);let eq10_e1087_d_b4: f64 = (p[87] * eq10_e1086_d_b4);let eq10_e1087_d_b5: f64 = (p[87] * eq10_e1086_d_b5);let eq10_e1087_d_b6: f64 = (p[87] * eq10_e1086_d_b6);let eq10_e1087_d_b7: f64 = (p[87] * eq10_e1086_d_b7);let eq10_e1087_d_b8: f64 = (p[87] * eq10_e1086_d_b8);let eq10_e1087_d_b9: f64 = (p[87] * eq10_e1086_d_b9);let eq10_e1087_d_b10: f64 = (p[87] * eq10_e1086_d_b10);let eq10_e1087_d_b11: f64 = (p[87] * eq10_e1086_d_b11);let eq10_e1087_d_b12: f64 = (p[87] * eq10_e1086_d_b12);let eq10_value: f64 = eq10_e1087;let eq10_node_derivatives: [f64; 19] = [eq10_e1087_d_n0, eq10_e1087_d_n1, eq10_e1087_d_n2, eq10_e1087_d_n3, eq10_e1087_d_n4, eq10_e1087_d_n5, eq10_e1087_d_n6, eq10_e1087_d_n7, eq10_e1087_d_n8, eq10_e1087_d_n9, eq10_e1087_d_n10, eq10_e1087_d_n11, eq10_e1087_d_n12, eq10_e1087_d_n13, eq10_e1087_d_n14, eq10_e1087_d_n15, eq10_e1087_d_n16, eq10_e1087_d_n17, eq10_e1087_d_n18];let eq10_branch_derivatives: [f64; 13] = [eq10_e1087_d_b0, eq10_e1087_d_b1, eq10_e1087_d_b2, eq10_e1087_d_b3, eq10_e1087_d_b4, eq10_e1087_d_b5, eq10_e1087_d_b6, eq10_e1087_d_b7, eq10_e1087_d_b8, eq10_e1087_d_b9, eq10_e1087_d_b10, eq10_e1087_d_b11, eq10_e1087_d_b12];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq11_e1090: f64 = (p[87] * s.v[281]);let eq11_value: f64 = eq11_e1090;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq11_value),
            &s.dn[281],
            &s.db[281],
            (multiplicity) * (p[87]),
        );let eq12_e1093: f64 = (p[87] * s.v[737]);let eq12_value: f64 = eq12_e1093;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq12_value),
            &s.dn[737],
            &s.db[737],
            (multiplicity) * (p[87]),
        );let eq13_e1096: f64 = (p[87] * s.v[862]);let eq13_value: f64 = eq13_e1096;
        stamper.stamp_current_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq13_value),
            &s.dn[862],
            &s.db[862],
            (multiplicity) * (p[87]),
        );let eq14_e1099: f64 = (p[87] * s.v[861]);let eq14_value: f64 = eq14_e1099;
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq14_value),
            &s.dn[861],
            &s.db[861],
            (multiplicity) * (p[87]),
        );let eq15_e1102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[66]);let eq15_e1103: f64 = (p[87] * eq15_e1102);let eq15_e1103_d_n0: f64 = (p[87] * (s.dn[66][0] * ddt_scale));let eq15_e1103_d_n1: f64 = (p[87] * (s.dn[66][1] * ddt_scale));let eq15_e1103_d_n2: f64 = (p[87] * (s.dn[66][2] * ddt_scale));let eq15_e1103_d_n3: f64 = (p[87] * (s.dn[66][3] * ddt_scale));let eq15_e1103_d_n4: f64 = (p[87] * (s.dn[66][4] * ddt_scale));let eq15_e1103_d_n5: f64 = (p[87] * (s.dn[66][5] * ddt_scale));let eq15_e1103_d_n6: f64 = (p[87] * (s.dn[66][6] * ddt_scale));let eq15_e1103_d_n7: f64 = (p[87] * (s.dn[66][7] * ddt_scale));let eq15_e1103_d_n8: f64 = (p[87] * (s.dn[66][8] * ddt_scale));let eq15_e1103_d_n9: f64 = (p[87] * (s.dn[66][9] * ddt_scale));let eq15_e1103_d_n10: f64 = (p[87] * (s.dn[66][10] * ddt_scale));let eq15_e1103_d_n11: f64 = (p[87] * (s.dn[66][11] * ddt_scale));let eq15_e1103_d_n12: f64 = (p[87] * (s.dn[66][12] * ddt_scale));let eq15_e1103_d_n13: f64 = (p[87] * (s.dn[66][13] * ddt_scale));let eq15_e1103_d_n14: f64 = (p[87] * (s.dn[66][14] * ddt_scale));let eq15_e1103_d_n15: f64 = (p[87] * (s.dn[66][15] * ddt_scale));let eq15_e1103_d_n16: f64 = (p[87] * (s.dn[66][16] * ddt_scale));let eq15_e1103_d_n17: f64 = (p[87] * (s.dn[66][17] * ddt_scale));let eq15_e1103_d_n18: f64 = (p[87] * (s.dn[66][18] * ddt_scale));let eq15_e1103_d_b0: f64 = (p[87] * (s.db[66][0] * ddt_scale));let eq15_e1103_d_b1: f64 = (p[87] * (s.db[66][1] * ddt_scale));let eq15_e1103_d_b2: f64 = (p[87] * (s.db[66][2] * ddt_scale));let eq15_e1103_d_b3: f64 = (p[87] * (s.db[66][3] * ddt_scale));let eq15_e1103_d_b4: f64 = (p[87] * (s.db[66][4] * ddt_scale));let eq15_e1103_d_b5: f64 = (p[87] * (s.db[66][5] * ddt_scale));let eq15_e1103_d_b6: f64 = (p[87] * (s.db[66][6] * ddt_scale));let eq15_e1103_d_b7: f64 = (p[87] * (s.db[66][7] * ddt_scale));let eq15_e1103_d_b8: f64 = (p[87] * (s.db[66][8] * ddt_scale));let eq15_e1103_d_b9: f64 = (p[87] * (s.db[66][9] * ddt_scale));let eq15_e1103_d_b10: f64 = (p[87] * (s.db[66][10] * ddt_scale));let eq15_e1103_d_b11: f64 = (p[87] * (s.db[66][11] * ddt_scale));let eq15_e1103_d_b12: f64 = (p[87] * (s.db[66][12] * ddt_scale));let eq15_value: f64 = eq15_e1103;let eq15_node_derivatives: [f64; 19] = [eq15_e1103_d_n0, eq15_e1103_d_n1, eq15_e1103_d_n2, eq15_e1103_d_n3, eq15_e1103_d_n4, eq15_e1103_d_n5, eq15_e1103_d_n6, eq15_e1103_d_n7, eq15_e1103_d_n8, eq15_e1103_d_n9, eq15_e1103_d_n10, eq15_e1103_d_n11, eq15_e1103_d_n12, eq15_e1103_d_n13, eq15_e1103_d_n14, eq15_e1103_d_n15, eq15_e1103_d_n16, eq15_e1103_d_n17, eq15_e1103_d_n18];let eq15_branch_derivatives: [f64; 13] = [eq15_e1103_d_b0, eq15_e1103_d_b1, eq15_e1103_d_b2, eq15_e1103_d_b3, eq15_e1103_d_b4, eq15_e1103_d_b5, eq15_e1103_d_b6, eq15_e1103_d_b7, eq15_e1103_d_b8, eq15_e1103_d_b9, eq15_e1103_d_b10, eq15_e1103_d_b11, eq15_e1103_d_b12];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );let eq16_e1106: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[65]);let eq16_e1107: f64 = (p[87] * eq16_e1106);let eq16_e1107_d_n0: f64 = (p[87] * (s.dn[65][0] * ddt_scale));let eq16_e1107_d_n1: f64 = (p[87] * (s.dn[65][1] * ddt_scale));let eq16_e1107_d_n2: f64 = (p[87] * (s.dn[65][2] * ddt_scale));let eq16_e1107_d_n3: f64 = (p[87] * (s.dn[65][3] * ddt_scale));let eq16_e1107_d_n4: f64 = (p[87] * (s.dn[65][4] * ddt_scale));let eq16_e1107_d_n5: f64 = (p[87] * (s.dn[65][5] * ddt_scale));let eq16_e1107_d_n6: f64 = (p[87] * (s.dn[65][6] * ddt_scale));let eq16_e1107_d_n7: f64 = (p[87] * (s.dn[65][7] * ddt_scale));let eq16_e1107_d_n8: f64 = (p[87] * (s.dn[65][8] * ddt_scale));let eq16_e1107_d_n9: f64 = (p[87] * (s.dn[65][9] * ddt_scale));let eq16_e1107_d_n10: f64 = (p[87] * (s.dn[65][10] * ddt_scale));let eq16_e1107_d_n11: f64 = (p[87] * (s.dn[65][11] * ddt_scale));let eq16_e1107_d_n12: f64 = (p[87] * (s.dn[65][12] * ddt_scale));let eq16_e1107_d_n13: f64 = (p[87] * (s.dn[65][13] * ddt_scale));let eq16_e1107_d_n14: f64 = (p[87] * (s.dn[65][14] * ddt_scale));let eq16_e1107_d_n15: f64 = (p[87] * (s.dn[65][15] * ddt_scale));let eq16_e1107_d_n16: f64 = (p[87] * (s.dn[65][16] * ddt_scale));let eq16_e1107_d_n17: f64 = (p[87] * (s.dn[65][17] * ddt_scale));let eq16_e1107_d_n18: f64 = (p[87] * (s.dn[65][18] * ddt_scale));let eq16_e1107_d_b0: f64 = (p[87] * (s.db[65][0] * ddt_scale));let eq16_e1107_d_b1: f64 = (p[87] * (s.db[65][1] * ddt_scale));let eq16_e1107_d_b2: f64 = (p[87] * (s.db[65][2] * ddt_scale));let eq16_e1107_d_b3: f64 = (p[87] * (s.db[65][3] * ddt_scale));let eq16_e1107_d_b4: f64 = (p[87] * (s.db[65][4] * ddt_scale));let eq16_e1107_d_b5: f64 = (p[87] * (s.db[65][5] * ddt_scale));let eq16_e1107_d_b6: f64 = (p[87] * (s.db[65][6] * ddt_scale));let eq16_e1107_d_b7: f64 = (p[87] * (s.db[65][7] * ddt_scale));let eq16_e1107_d_b8: f64 = (p[87] * (s.db[65][8] * ddt_scale));let eq16_e1107_d_b9: f64 = (p[87] * (s.db[65][9] * ddt_scale));let eq16_e1107_d_b10: f64 = (p[87] * (s.db[65][10] * ddt_scale));let eq16_e1107_d_b11: f64 = (p[87] * (s.db[65][11] * ddt_scale));let eq16_e1107_d_b12: f64 = (p[87] * (s.db[65][12] * ddt_scale));let eq16_value: f64 = eq16_e1107;let eq16_node_derivatives: [f64; 19] = [eq16_e1107_d_n0, eq16_e1107_d_n1, eq16_e1107_d_n2, eq16_e1107_d_n3, eq16_e1107_d_n4, eq16_e1107_d_n5, eq16_e1107_d_n6, eq16_e1107_d_n7, eq16_e1107_d_n8, eq16_e1107_d_n9, eq16_e1107_d_n10, eq16_e1107_d_n11, eq16_e1107_d_n12, eq16_e1107_d_n13, eq16_e1107_d_n14, eq16_e1107_d_n15, eq16_e1107_d_n16, eq16_e1107_d_n17, eq16_e1107_d_n18];let eq16_branch_derivatives: [f64; 13] = [eq16_e1107_d_b0, eq16_e1107_d_b1, eq16_e1107_d_b2, eq16_e1107_d_b3, eq16_e1107_d_b4, eq16_e1107_d_b5, eq16_e1107_d_b6, eq16_e1107_d_b7, eq16_e1107_d_b8, eq16_e1107_d_b9, eq16_e1107_d_b10, eq16_e1107_d_b11, eq16_e1107_d_b12];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq17_e1113, eq17_e1113_d_n0, eq17_e1113_d_n1, eq17_e1113_d_n2, eq17_e1113_d_n3, eq17_e1113_d_n4, eq17_e1113_d_n5, eq17_e1113_d_n6, eq17_e1113_d_n7, eq17_e1113_d_n8, eq17_e1113_d_n9, eq17_e1113_d_n10, eq17_e1113_d_n11, eq17_e1113_d_n12, eq17_e1113_d_n13, eq17_e1113_d_n14, eq17_e1113_d_n15, eq17_e1113_d_n16, eq17_e1113_d_n17, eq17_e1113_d_n18, eq17_e1113_d_b0, eq17_e1113_d_b1, eq17_e1113_d_b2, eq17_e1113_d_b3, eq17_e1113_d_b4, eq17_e1113_d_b5, eq17_e1113_d_b6, eq17_e1113_d_b7, eq17_e1113_d_b8, eq17_e1113_d_b9, eq17_e1113_d_b10, eq17_e1113_d_b11, eq17_e1113_d_b12,) = {
    if s.b[3407] {
        let eq17_e1111: f64 = (p[87] * s.v[870]);
        (eq17_e1111, (p[87] * s.dn[870][0]), (p[87] * s.dn[870][1]), (p[87] * s.dn[870][2]), (p[87] * s.dn[870][3]), (p[87] * s.dn[870][4]), (p[87] * s.dn[870][5]), (p[87] * s.dn[870][6]), (p[87] * s.dn[870][7]), (p[87] * s.dn[870][8]), (p[87] * s.dn[870][9]), (p[87] * s.dn[870][10]), (p[87] * s.dn[870][11]), (p[87] * s.dn[870][12]), (p[87] * s.dn[870][13]), (p[87] * s.dn[870][14]), (p[87] * s.dn[870][15]), (p[87] * s.dn[870][16]), (p[87] * s.dn[870][17]), (p[87] * s.dn[870][18]), (p[87] * s.db[870][0]), (p[87] * s.db[870][1]), (p[87] * s.db[870][2]), (p[87] * s.db[870][3]), (p[87] * s.db[870][4]), (p[87] * s.db[870][5]), (p[87] * s.db[870][6]), (p[87] * s.db[870][7]), (p[87] * s.db[870][8]), (p[87] * s.db[870][9]), (p[87] * s.db[870][10]), (p[87] * s.db[870][11]), (p[87] * s.db[870][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1113;let eq17_node_derivatives: [f64; 19] = [eq17_e1113_d_n0, eq17_e1113_d_n1, eq17_e1113_d_n2, eq17_e1113_d_n3, eq17_e1113_d_n4, eq17_e1113_d_n5, eq17_e1113_d_n6, eq17_e1113_d_n7, eq17_e1113_d_n8, eq17_e1113_d_n9, eq17_e1113_d_n10, eq17_e1113_d_n11, eq17_e1113_d_n12, eq17_e1113_d_n13, eq17_e1113_d_n14, eq17_e1113_d_n15, eq17_e1113_d_n16, eq17_e1113_d_n17, eq17_e1113_d_n18];let eq17_branch_derivatives: [f64; 13] = [eq17_e1113_d_b0, eq17_e1113_d_b1, eq17_e1113_d_b2, eq17_e1113_d_b3, eq17_e1113_d_b4, eq17_e1113_d_b5, eq17_e1113_d_b6, eq17_e1113_d_b7, eq17_e1113_d_b8, eq17_e1113_d_b9, eq17_e1113_d_b10, eq17_e1113_d_b11, eq17_e1113_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1119, eq18_e1119_d_n0, eq18_e1119_d_n1, eq18_e1119_d_n2, eq18_e1119_d_n3, eq18_e1119_d_n4, eq18_e1119_d_n5, eq18_e1119_d_n6, eq18_e1119_d_n7, eq18_e1119_d_n8, eq18_e1119_d_n9, eq18_e1119_d_n10, eq18_e1119_d_n11, eq18_e1119_d_n12, eq18_e1119_d_n13, eq18_e1119_d_n14, eq18_e1119_d_n15, eq18_e1119_d_n16, eq18_e1119_d_n17, eq18_e1119_d_n18, eq18_e1119_d_b0, eq18_e1119_d_b1, eq18_e1119_d_b2, eq18_e1119_d_b3, eq18_e1119_d_b4, eq18_e1119_d_b5, eq18_e1119_d_b6, eq18_e1119_d_b7, eq18_e1119_d_b8, eq18_e1119_d_b9, eq18_e1119_d_b10, eq18_e1119_d_b11, eq18_e1119_d_b12,) = {
    if s.b[3407] {
        let eq18_e1117: f64 = (p[87] * s.v[869]);
        (eq18_e1117, (p[87] * s.dn[869][0]), (p[87] * s.dn[869][1]), (p[87] * s.dn[869][2]), (p[87] * s.dn[869][3]), (p[87] * s.dn[869][4]), (p[87] * s.dn[869][5]), (p[87] * s.dn[869][6]), (p[87] * s.dn[869][7]), (p[87] * s.dn[869][8]), (p[87] * s.dn[869][9]), (p[87] * s.dn[869][10]), (p[87] * s.dn[869][11]), (p[87] * s.dn[869][12]), (p[87] * s.dn[869][13]), (p[87] * s.dn[869][14]), (p[87] * s.dn[869][15]), (p[87] * s.dn[869][16]), (p[87] * s.dn[869][17]), (p[87] * s.dn[869][18]), (p[87] * s.db[869][0]), (p[87] * s.db[869][1]), (p[87] * s.db[869][2]), (p[87] * s.db[869][3]), (p[87] * s.db[869][4]), (p[87] * s.db[869][5]), (p[87] * s.db[869][6]), (p[87] * s.db[869][7]), (p[87] * s.db[869][8]), (p[87] * s.db[869][9]), (p[87] * s.db[869][10]), (p[87] * s.db[869][11]), (p[87] * s.db[869][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1119;let eq18_node_derivatives: [f64; 19] = [eq18_e1119_d_n0, eq18_e1119_d_n1, eq18_e1119_d_n2, eq18_e1119_d_n3, eq18_e1119_d_n4, eq18_e1119_d_n5, eq18_e1119_d_n6, eq18_e1119_d_n7, eq18_e1119_d_n8, eq18_e1119_d_n9, eq18_e1119_d_n10, eq18_e1119_d_n11, eq18_e1119_d_n12, eq18_e1119_d_n13, eq18_e1119_d_n14, eq18_e1119_d_n15, eq18_e1119_d_n16, eq18_e1119_d_n17, eq18_e1119_d_n18];let eq18_branch_derivatives: [f64; 13] = [eq18_e1119_d_b0, eq18_e1119_d_b1, eq18_e1119_d_b2, eq18_e1119_d_b3, eq18_e1119_d_b4, eq18_e1119_d_b5, eq18_e1119_d_b6, eq18_e1119_d_b7, eq18_e1119_d_b8, eq18_e1119_d_b9, eq18_e1119_d_b10, eq18_e1119_d_b11, eq18_e1119_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1126, eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18, eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12,) = {
    if s.b[3407] {
        let eq19_e1123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[68]);let eq19_e1124: f64 = (p[87] * eq19_e1123);let eq19_e1124_d_n0: f64 = (p[87] * (s.dn[68][0] * ddt_scale));let eq19_e1124_d_n1: f64 = (p[87] * (s.dn[68][1] * ddt_scale));let eq19_e1124_d_n2: f64 = (p[87] * (s.dn[68][2] * ddt_scale));let eq19_e1124_d_n3: f64 = (p[87] * (s.dn[68][3] * ddt_scale));let eq19_e1124_d_n4: f64 = (p[87] * (s.dn[68][4] * ddt_scale));let eq19_e1124_d_n5: f64 = (p[87] * (s.dn[68][5] * ddt_scale));let eq19_e1124_d_n6: f64 = (p[87] * (s.dn[68][6] * ddt_scale));let eq19_e1124_d_n7: f64 = (p[87] * (s.dn[68][7] * ddt_scale));let eq19_e1124_d_n8: f64 = (p[87] * (s.dn[68][8] * ddt_scale));let eq19_e1124_d_n9: f64 = (p[87] * (s.dn[68][9] * ddt_scale));let eq19_e1124_d_n10: f64 = (p[87] * (s.dn[68][10] * ddt_scale));let eq19_e1124_d_n11: f64 = (p[87] * (s.dn[68][11] * ddt_scale));let eq19_e1124_d_n12: f64 = (p[87] * (s.dn[68][12] * ddt_scale));let eq19_e1124_d_n13: f64 = (p[87] * (s.dn[68][13] * ddt_scale));let eq19_e1124_d_n14: f64 = (p[87] * (s.dn[68][14] * ddt_scale));let eq19_e1124_d_n15: f64 = (p[87] * (s.dn[68][15] * ddt_scale));let eq19_e1124_d_n16: f64 = (p[87] * (s.dn[68][16] * ddt_scale));let eq19_e1124_d_n17: f64 = (p[87] * (s.dn[68][17] * ddt_scale));let eq19_e1124_d_n18: f64 = (p[87] * (s.dn[68][18] * ddt_scale));let eq19_e1124_d_b0: f64 = (p[87] * (s.db[68][0] * ddt_scale));let eq19_e1124_d_b1: f64 = (p[87] * (s.db[68][1] * ddt_scale));let eq19_e1124_d_b2: f64 = (p[87] * (s.db[68][2] * ddt_scale));let eq19_e1124_d_b3: f64 = (p[87] * (s.db[68][3] * ddt_scale));let eq19_e1124_d_b4: f64 = (p[87] * (s.db[68][4] * ddt_scale));let eq19_e1124_d_b5: f64 = (p[87] * (s.db[68][5] * ddt_scale));let eq19_e1124_d_b6: f64 = (p[87] * (s.db[68][6] * ddt_scale));let eq19_e1124_d_b7: f64 = (p[87] * (s.db[68][7] * ddt_scale));let eq19_e1124_d_b8: f64 = (p[87] * (s.db[68][8] * ddt_scale));let eq19_e1124_d_b9: f64 = (p[87] * (s.db[68][9] * ddt_scale));let eq19_e1124_d_b10: f64 = (p[87] * (s.db[68][10] * ddt_scale));let eq19_e1124_d_b11: f64 = (p[87] * (s.db[68][11] * ddt_scale));let eq19_e1124_d_b12: f64 = (p[87] * (s.db[68][12] * ddt_scale));
        (eq19_e1124, eq19_e1124_d_n0, eq19_e1124_d_n1, eq19_e1124_d_n2, eq19_e1124_d_n3, eq19_e1124_d_n4, eq19_e1124_d_n5, eq19_e1124_d_n6, eq19_e1124_d_n7, eq19_e1124_d_n8, eq19_e1124_d_n9, eq19_e1124_d_n10, eq19_e1124_d_n11, eq19_e1124_d_n12, eq19_e1124_d_n13, eq19_e1124_d_n14, eq19_e1124_d_n15, eq19_e1124_d_n16, eq19_e1124_d_n17, eq19_e1124_d_n18, eq19_e1124_d_b0, eq19_e1124_d_b1, eq19_e1124_d_b2, eq19_e1124_d_b3, eq19_e1124_d_b4, eq19_e1124_d_b5, eq19_e1124_d_b6, eq19_e1124_d_b7, eq19_e1124_d_b8, eq19_e1124_d_b9, eq19_e1124_d_b10, eq19_e1124_d_b11, eq19_e1124_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1126;let eq19_node_derivatives: [f64; 19] = [eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18];let eq19_branch_derivatives: [f64; 13] = [eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq20_e1133, eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18, eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12,) = {
    if s.b[3407] {
        let eq20_e1130: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[67]);let eq20_e1131: f64 = (p[87] * eq20_e1130);let eq20_e1131_d_n0: f64 = (p[87] * (s.dn[67][0] * ddt_scale));let eq20_e1131_d_n1: f64 = (p[87] * (s.dn[67][1] * ddt_scale));let eq20_e1131_d_n2: f64 = (p[87] * (s.dn[67][2] * ddt_scale));let eq20_e1131_d_n3: f64 = (p[87] * (s.dn[67][3] * ddt_scale));let eq20_e1131_d_n4: f64 = (p[87] * (s.dn[67][4] * ddt_scale));let eq20_e1131_d_n5: f64 = (p[87] * (s.dn[67][5] * ddt_scale));let eq20_e1131_d_n6: f64 = (p[87] * (s.dn[67][6] * ddt_scale));let eq20_e1131_d_n7: f64 = (p[87] * (s.dn[67][7] * ddt_scale));let eq20_e1131_d_n8: f64 = (p[87] * (s.dn[67][8] * ddt_scale));let eq20_e1131_d_n9: f64 = (p[87] * (s.dn[67][9] * ddt_scale));let eq20_e1131_d_n10: f64 = (p[87] * (s.dn[67][10] * ddt_scale));let eq20_e1131_d_n11: f64 = (p[87] * (s.dn[67][11] * ddt_scale));let eq20_e1131_d_n12: f64 = (p[87] * (s.dn[67][12] * ddt_scale));let eq20_e1131_d_n13: f64 = (p[87] * (s.dn[67][13] * ddt_scale));let eq20_e1131_d_n14: f64 = (p[87] * (s.dn[67][14] * ddt_scale));let eq20_e1131_d_n15: f64 = (p[87] * (s.dn[67][15] * ddt_scale));let eq20_e1131_d_n16: f64 = (p[87] * (s.dn[67][16] * ddt_scale));let eq20_e1131_d_n17: f64 = (p[87] * (s.dn[67][17] * ddt_scale));let eq20_e1131_d_n18: f64 = (p[87] * (s.dn[67][18] * ddt_scale));let eq20_e1131_d_b0: f64 = (p[87] * (s.db[67][0] * ddt_scale));let eq20_e1131_d_b1: f64 = (p[87] * (s.db[67][1] * ddt_scale));let eq20_e1131_d_b2: f64 = (p[87] * (s.db[67][2] * ddt_scale));let eq20_e1131_d_b3: f64 = (p[87] * (s.db[67][3] * ddt_scale));let eq20_e1131_d_b4: f64 = (p[87] * (s.db[67][4] * ddt_scale));let eq20_e1131_d_b5: f64 = (p[87] * (s.db[67][5] * ddt_scale));let eq20_e1131_d_b6: f64 = (p[87] * (s.db[67][6] * ddt_scale));let eq20_e1131_d_b7: f64 = (p[87] * (s.db[67][7] * ddt_scale));let eq20_e1131_d_b8: f64 = (p[87] * (s.db[67][8] * ddt_scale));let eq20_e1131_d_b9: f64 = (p[87] * (s.db[67][9] * ddt_scale));let eq20_e1131_d_b10: f64 = (p[87] * (s.db[67][10] * ddt_scale));let eq20_e1131_d_b11: f64 = (p[87] * (s.db[67][11] * ddt_scale));let eq20_e1131_d_b12: f64 = (p[87] * (s.db[67][12] * ddt_scale));
        (eq20_e1131, eq20_e1131_d_n0, eq20_e1131_d_n1, eq20_e1131_d_n2, eq20_e1131_d_n3, eq20_e1131_d_n4, eq20_e1131_d_n5, eq20_e1131_d_n6, eq20_e1131_d_n7, eq20_e1131_d_n8, eq20_e1131_d_n9, eq20_e1131_d_n10, eq20_e1131_d_n11, eq20_e1131_d_n12, eq20_e1131_d_n13, eq20_e1131_d_n14, eq20_e1131_d_n15, eq20_e1131_d_n16, eq20_e1131_d_n17, eq20_e1131_d_n18, eq20_e1131_d_b0, eq20_e1131_d_b1, eq20_e1131_d_b2, eq20_e1131_d_b3, eq20_e1131_d_b4, eq20_e1131_d_b5, eq20_e1131_d_b6, eq20_e1131_d_b7, eq20_e1131_d_b8, eq20_e1131_d_b9, eq20_e1131_d_b10, eq20_e1131_d_b11, eq20_e1131_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1133;let eq20_node_derivatives: [f64; 19] = [eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18];let eq20_branch_derivatives: [f64; 13] = [eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1139, eq21_e1139_d_n0, eq21_e1139_d_n1, eq21_e1139_d_n2, eq21_e1139_d_n3, eq21_e1139_d_n4, eq21_e1139_d_n5, eq21_e1139_d_n6, eq21_e1139_d_n7, eq21_e1139_d_n8, eq21_e1139_d_n9, eq21_e1139_d_n10, eq21_e1139_d_n11, eq21_e1139_d_n12, eq21_e1139_d_n13, eq21_e1139_d_n14, eq21_e1139_d_n15, eq21_e1139_d_n16, eq21_e1139_d_n17, eq21_e1139_d_n18, eq21_e1139_d_b0, eq21_e1139_d_b1, eq21_e1139_d_b2, eq21_e1139_d_b3, eq21_e1139_d_b4, eq21_e1139_d_b5, eq21_e1139_d_b6, eq21_e1139_d_b7, eq21_e1139_d_b8, eq21_e1139_d_b9, eq21_e1139_d_b10, eq21_e1139_d_b11, eq21_e1139_d_b12,) = {
    if s.b[3408] {
        let eq21_e1137: f64 = (p[87] * s.v[200]);
        (eq21_e1137, (p[87] * s.dn[200][0]), (p[87] * s.dn[200][1]), (p[87] * s.dn[200][2]), (p[87] * s.dn[200][3]), (p[87] * s.dn[200][4]), (p[87] * s.dn[200][5]), (p[87] * s.dn[200][6]), (p[87] * s.dn[200][7]), (p[87] * s.dn[200][8]), (p[87] * s.dn[200][9]), (p[87] * s.dn[200][10]), (p[87] * s.dn[200][11]), (p[87] * s.dn[200][12]), (p[87] * s.dn[200][13]), (p[87] * s.dn[200][14]), (p[87] * s.dn[200][15]), (p[87] * s.dn[200][16]), (p[87] * s.dn[200][17]), (p[87] * s.dn[200][18]), (p[87] * s.db[200][0]), (p[87] * s.db[200][1]), (p[87] * s.db[200][2]), (p[87] * s.db[200][3]), (p[87] * s.db[200][4]), (p[87] * s.db[200][5]), (p[87] * s.db[200][6]), (p[87] * s.db[200][7]), (p[87] * s.db[200][8]), (p[87] * s.db[200][9]), (p[87] * s.db[200][10]), (p[87] * s.db[200][11]), (p[87] * s.db[200][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1139;let eq21_node_derivatives: [f64; 19] = [eq21_e1139_d_n0, eq21_e1139_d_n1, eq21_e1139_d_n2, eq21_e1139_d_n3, eq21_e1139_d_n4, eq21_e1139_d_n5, eq21_e1139_d_n6, eq21_e1139_d_n7, eq21_e1139_d_n8, eq21_e1139_d_n9, eq21_e1139_d_n10, eq21_e1139_d_n11, eq21_e1139_d_n12, eq21_e1139_d_n13, eq21_e1139_d_n14, eq21_e1139_d_n15, eq21_e1139_d_n16, eq21_e1139_d_n17, eq21_e1139_d_n18];let eq21_branch_derivatives: [f64; 13] = [eq21_e1139_d_b0, eq21_e1139_d_b1, eq21_e1139_d_b2, eq21_e1139_d_b3, eq21_e1139_d_b4, eq21_e1139_d_b5, eq21_e1139_d_b6, eq21_e1139_d_b7, eq21_e1139_d_b8, eq21_e1139_d_b9, eq21_e1139_d_b10, eq21_e1139_d_b11, eq21_e1139_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1145, eq22_e1145_d_n0, eq22_e1145_d_n1, eq22_e1145_d_n2, eq22_e1145_d_n3, eq22_e1145_d_n4, eq22_e1145_d_n5, eq22_e1145_d_n6, eq22_e1145_d_n7, eq22_e1145_d_n8, eq22_e1145_d_n9, eq22_e1145_d_n10, eq22_e1145_d_n11, eq22_e1145_d_n12, eq22_e1145_d_n13, eq22_e1145_d_n14, eq22_e1145_d_n15, eq22_e1145_d_n16, eq22_e1145_d_n17, eq22_e1145_d_n18, eq22_e1145_d_b0, eq22_e1145_d_b1, eq22_e1145_d_b2, eq22_e1145_d_b3, eq22_e1145_d_b4, eq22_e1145_d_b5, eq22_e1145_d_b6, eq22_e1145_d_b7, eq22_e1145_d_b8, eq22_e1145_d_b9, eq22_e1145_d_b10, eq22_e1145_d_b11, eq22_e1145_d_b12,) = {
    if s.b[3408] {
        let eq22_e1143: f64 = (p[87] * s.v[201]);
        (eq22_e1143, (p[87] * s.dn[201][0]), (p[87] * s.dn[201][1]), (p[87] * s.dn[201][2]), (p[87] * s.dn[201][3]), (p[87] * s.dn[201][4]), (p[87] * s.dn[201][5]), (p[87] * s.dn[201][6]), (p[87] * s.dn[201][7]), (p[87] * s.dn[201][8]), (p[87] * s.dn[201][9]), (p[87] * s.dn[201][10]), (p[87] * s.dn[201][11]), (p[87] * s.dn[201][12]), (p[87] * s.dn[201][13]), (p[87] * s.dn[201][14]), (p[87] * s.dn[201][15]), (p[87] * s.dn[201][16]), (p[87] * s.dn[201][17]), (p[87] * s.dn[201][18]), (p[87] * s.db[201][0]), (p[87] * s.db[201][1]), (p[87] * s.db[201][2]), (p[87] * s.db[201][3]), (p[87] * s.db[201][4]), (p[87] * s.db[201][5]), (p[87] * s.db[201][6]), (p[87] * s.db[201][7]), (p[87] * s.db[201][8]), (p[87] * s.db[201][9]), (p[87] * s.db[201][10]), (p[87] * s.db[201][11]), (p[87] * s.db[201][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1145;let eq22_node_derivatives: [f64; 19] = [eq22_e1145_d_n0, eq22_e1145_d_n1, eq22_e1145_d_n2, eq22_e1145_d_n3, eq22_e1145_d_n4, eq22_e1145_d_n5, eq22_e1145_d_n6, eq22_e1145_d_n7, eq22_e1145_d_n8, eq22_e1145_d_n9, eq22_e1145_d_n10, eq22_e1145_d_n11, eq22_e1145_d_n12, eq22_e1145_d_n13, eq22_e1145_d_n14, eq22_e1145_d_n15, eq22_e1145_d_n16, eq22_e1145_d_n17, eq22_e1145_d_n18];let eq22_branch_derivatives: [f64; 13] = [eq22_e1145_d_b0, eq22_e1145_d_b1, eq22_e1145_d_b2, eq22_e1145_d_b3, eq22_e1145_d_b4, eq22_e1145_d_b5, eq22_e1145_d_b6, eq22_e1145_d_b7, eq22_e1145_d_b8, eq22_e1145_d_b9, eq22_e1145_d_b10, eq22_e1145_d_b11, eq22_e1145_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1151, eq23_e1151_d_n0, eq23_e1151_d_n1, eq23_e1151_d_n2, eq23_e1151_d_n3, eq23_e1151_d_n4, eq23_e1151_d_n5, eq23_e1151_d_n6, eq23_e1151_d_n7, eq23_e1151_d_n8, eq23_e1151_d_n9, eq23_e1151_d_n10, eq23_e1151_d_n11, eq23_e1151_d_n12, eq23_e1151_d_n13, eq23_e1151_d_n14, eq23_e1151_d_n15, eq23_e1151_d_n16, eq23_e1151_d_n17, eq23_e1151_d_n18, eq23_e1151_d_b0, eq23_e1151_d_b1, eq23_e1151_d_b2, eq23_e1151_d_b3, eq23_e1151_d_b4, eq23_e1151_d_b5, eq23_e1151_d_b6, eq23_e1151_d_b7, eq23_e1151_d_b8, eq23_e1151_d_b9, eq23_e1151_d_b10, eq23_e1151_d_b11, eq23_e1151_d_b12,) = {
    if s.b[3408] {
        let eq23_e1149: f64 = (p[87] * s.v[202]);
        (eq23_e1149, (p[87] * s.dn[202][0]), (p[87] * s.dn[202][1]), (p[87] * s.dn[202][2]), (p[87] * s.dn[202][3]), (p[87] * s.dn[202][4]), (p[87] * s.dn[202][5]), (p[87] * s.dn[202][6]), (p[87] * s.dn[202][7]), (p[87] * s.dn[202][8]), (p[87] * s.dn[202][9]), (p[87] * s.dn[202][10]), (p[87] * s.dn[202][11]), (p[87] * s.dn[202][12]), (p[87] * s.dn[202][13]), (p[87] * s.dn[202][14]), (p[87] * s.dn[202][15]), (p[87] * s.dn[202][16]), (p[87] * s.dn[202][17]), (p[87] * s.dn[202][18]), (p[87] * s.db[202][0]), (p[87] * s.db[202][1]), (p[87] * s.db[202][2]), (p[87] * s.db[202][3]), (p[87] * s.db[202][4]), (p[87] * s.db[202][5]), (p[87] * s.db[202][6]), (p[87] * s.db[202][7]), (p[87] * s.db[202][8]), (p[87] * s.db[202][9]), (p[87] * s.db[202][10]), (p[87] * s.db[202][11]), (p[87] * s.db[202][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1151;let eq23_node_derivatives: [f64; 19] = [eq23_e1151_d_n0, eq23_e1151_d_n1, eq23_e1151_d_n2, eq23_e1151_d_n3, eq23_e1151_d_n4, eq23_e1151_d_n5, eq23_e1151_d_n6, eq23_e1151_d_n7, eq23_e1151_d_n8, eq23_e1151_d_n9, eq23_e1151_d_n10, eq23_e1151_d_n11, eq23_e1151_d_n12, eq23_e1151_d_n13, eq23_e1151_d_n14, eq23_e1151_d_n15, eq23_e1151_d_n16, eq23_e1151_d_n17, eq23_e1151_d_n18];let eq23_branch_derivatives: [f64; 13] = [eq23_e1151_d_b0, eq23_e1151_d_b1, eq23_e1151_d_b2, eq23_e1151_d_b3, eq23_e1151_d_b4, eq23_e1151_d_b5, eq23_e1151_d_b6, eq23_e1151_d_b7, eq23_e1151_d_b8, eq23_e1151_d_b9, eq23_e1151_d_b10, eq23_e1151_d_b11, eq23_e1151_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq24_e1157, eq24_e1157_d_n0, eq24_e1157_d_n1, eq24_e1157_d_n2, eq24_e1157_d_n3, eq24_e1157_d_n4, eq24_e1157_d_n5, eq24_e1157_d_n6, eq24_e1157_d_n7, eq24_e1157_d_n8, eq24_e1157_d_n9, eq24_e1157_d_n10, eq24_e1157_d_n11, eq24_e1157_d_n12, eq24_e1157_d_n13, eq24_e1157_d_n14, eq24_e1157_d_n15, eq24_e1157_d_n16, eq24_e1157_d_n17, eq24_e1157_d_n18, eq24_e1157_d_b0, eq24_e1157_d_b1, eq24_e1157_d_b2, eq24_e1157_d_b3, eq24_e1157_d_b4, eq24_e1157_d_b5, eq24_e1157_d_b6, eq24_e1157_d_b7, eq24_e1157_d_b8, eq24_e1157_d_b9, eq24_e1157_d_b10, eq24_e1157_d_b11, eq24_e1157_d_b12,) = {
    if (s.v[75] != 0.0) {
        let eq24_e1155: f64 = ((nv0 - nv6) / s.v[4]);let eq24_e1155_d_n0: f64 = ((s.v[4] - ((nv0 - nv6) * s.dn[4][0])) / (s.v[4] * s.v[4]));let eq24_e1155_d_n1: f64 = (-(((nv0 - nv6) * s.dn[4][1]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n2: f64 = (-(((nv0 - nv6) * s.dn[4][2]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n3: f64 = (-(((nv0 - nv6) * s.dn[4][3]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n4: f64 = (-(((nv0 - nv6) * s.dn[4][4]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n5: f64 = (-(((nv0 - nv6) * s.dn[4][5]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n6: f64 = (((-s.v[4]) - ((nv0 - nv6) * s.dn[4][6])) / (s.v[4] * s.v[4]));let eq24_e1155_d_n7: f64 = (-(((nv0 - nv6) * s.dn[4][7]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n8: f64 = (-(((nv0 - nv6) * s.dn[4][8]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n9: f64 = (-(((nv0 - nv6) * s.dn[4][9]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n10: f64 = (-(((nv0 - nv6) * s.dn[4][10]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n11: f64 = (-(((nv0 - nv6) * s.dn[4][11]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n12: f64 = (-(((nv0 - nv6) * s.dn[4][12]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n13: f64 = (-(((nv0 - nv6) * s.dn[4][13]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n14: f64 = (-(((nv0 - nv6) * s.dn[4][14]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n15: f64 = (-(((nv0 - nv6) * s.dn[4][15]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n16: f64 = (-(((nv0 - nv6) * s.dn[4][16]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n17: f64 = (-(((nv0 - nv6) * s.dn[4][17]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n18: f64 = (-(((nv0 - nv6) * s.dn[4][18]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b0: f64 = (-(((nv0 - nv6) * s.db[4][0]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b1: f64 = (-(((nv0 - nv6) * s.db[4][1]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b2: f64 = (-(((nv0 - nv6) * s.db[4][2]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b3: f64 = (-(((nv0 - nv6) * s.db[4][3]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b4: f64 = (-(((nv0 - nv6) * s.db[4][4]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b5: f64 = (-(((nv0 - nv6) * s.db[4][5]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b6: f64 = (-(((nv0 - nv6) * s.db[4][6]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b7: f64 = (-(((nv0 - nv6) * s.db[4][7]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b8: f64 = (-(((nv0 - nv6) * s.db[4][8]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b9: f64 = (-(((nv0 - nv6) * s.db[4][9]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b10: f64 = (-(((nv0 - nv6) * s.db[4][10]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b11: f64 = (-(((nv0 - nv6) * s.db[4][11]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b12: f64 = (-(((nv0 - nv6) * s.db[4][12]) / (s.v[4] * s.v[4])));
        (eq24_e1155, eq24_e1155_d_n0, eq24_e1155_d_n1, eq24_e1155_d_n2, eq24_e1155_d_n3, eq24_e1155_d_n4, eq24_e1155_d_n5, eq24_e1155_d_n6, eq24_e1155_d_n7, eq24_e1155_d_n8, eq24_e1155_d_n9, eq24_e1155_d_n10, eq24_e1155_d_n11, eq24_e1155_d_n12, eq24_e1155_d_n13, eq24_e1155_d_n14, eq24_e1155_d_n15, eq24_e1155_d_n16, eq24_e1155_d_n17, eq24_e1155_d_n18, eq24_e1155_d_b0, eq24_e1155_d_b1, eq24_e1155_d_b2, eq24_e1155_d_b3, eq24_e1155_d_b4, eq24_e1155_d_b5, eq24_e1155_d_b6, eq24_e1155_d_b7, eq24_e1155_d_b8, eq24_e1155_d_b9, eq24_e1155_d_b10, eq24_e1155_d_b11, eq24_e1155_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1157;let eq24_node_derivatives: [f64; 19] = [eq24_e1157_d_n0, eq24_e1157_d_n1, eq24_e1157_d_n2, eq24_e1157_d_n3, eq24_e1157_d_n4, eq24_e1157_d_n5, eq24_e1157_d_n6, eq24_e1157_d_n7, eq24_e1157_d_n8, eq24_e1157_d_n9, eq24_e1157_d_n10, eq24_e1157_d_n11, eq24_e1157_d_n12, eq24_e1157_d_n13, eq24_e1157_d_n14, eq24_e1157_d_n15, eq24_e1157_d_n16, eq24_e1157_d_n17, eq24_e1157_d_n18];let eq24_branch_derivatives: [f64; 13] = [eq24_e1157_d_b0, eq24_e1157_d_b1, eq24_e1157_d_b2, eq24_e1157_d_b3, eq24_e1157_d_b4, eq24_e1157_d_b5, eq24_e1157_d_b6, eq24_e1157_d_b7, eq24_e1157_d_b8, eq24_e1157_d_b9, eq24_e1157_d_b10, eq24_e1157_d_b11, eq24_e1157_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1162,) = {
    if (s.v[75] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1162;
        stamper.stamp_potential_const_local(
            4,
            eq25_value,
        );
        let (eq26_e1168, eq26_e1168_d_n0, eq26_e1168_d_n1, eq26_e1168_d_n2, eq26_e1168_d_n3, eq26_e1168_d_n4, eq26_e1168_d_n5, eq26_e1168_d_n6, eq26_e1168_d_n7, eq26_e1168_d_n8, eq26_e1168_d_n9, eq26_e1168_d_n10, eq26_e1168_d_n11, eq26_e1168_d_n12, eq26_e1168_d_n13, eq26_e1168_d_n14, eq26_e1168_d_n15, eq26_e1168_d_n16, eq26_e1168_d_n17, eq26_e1168_d_n18, eq26_e1168_d_b0, eq26_e1168_d_b1, eq26_e1168_d_b2, eq26_e1168_d_b3, eq26_e1168_d_b4, eq26_e1168_d_b5, eq26_e1168_d_b6, eq26_e1168_d_b7, eq26_e1168_d_b8, eq26_e1168_d_b9, eq26_e1168_d_b10, eq26_e1168_d_b11, eq26_e1168_d_b12,) = {
    if (s.v[76] != 0.0) {
        let eq26_e1166: f64 = ((nv8 - nv2) / s.v[5]);let eq26_e1166_d_n0: f64 = (-(((nv8 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n1: f64 = (-(((nv8 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n2: f64 = (((-s.v[5]) - ((nv8 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));let eq26_e1166_d_n3: f64 = (-(((nv8 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n4: f64 = (-(((nv8 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n5: f64 = (-(((nv8 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n6: f64 = (-(((nv8 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n7: f64 = (-(((nv8 - nv2) * s.dn[5][7]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n8: f64 = ((s.v[5] - ((nv8 - nv2) * s.dn[5][8])) / (s.v[5] * s.v[5]));let eq26_e1166_d_n9: f64 = (-(((nv8 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n10: f64 = (-(((nv8 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n11: f64 = (-(((nv8 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n12: f64 = (-(((nv8 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n13: f64 = (-(((nv8 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n14: f64 = (-(((nv8 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n15: f64 = (-(((nv8 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n16: f64 = (-(((nv8 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n17: f64 = (-(((nv8 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n18: f64 = (-(((nv8 - nv2) * s.dn[5][18]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b0: f64 = (-(((nv8 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b1: f64 = (-(((nv8 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b2: f64 = (-(((nv8 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b3: f64 = (-(((nv8 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b4: f64 = (-(((nv8 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b5: f64 = (-(((nv8 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b6: f64 = (-(((nv8 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b7: f64 = (-(((nv8 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b8: f64 = (-(((nv8 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b9: f64 = (-(((nv8 - nv2) * s.db[5][9]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b10: f64 = (-(((nv8 - nv2) * s.db[5][10]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b11: f64 = (-(((nv8 - nv2) * s.db[5][11]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b12: f64 = (-(((nv8 - nv2) * s.db[5][12]) / (s.v[5] * s.v[5])));
        (eq26_e1166, eq26_e1166_d_n0, eq26_e1166_d_n1, eq26_e1166_d_n2, eq26_e1166_d_n3, eq26_e1166_d_n4, eq26_e1166_d_n5, eq26_e1166_d_n6, eq26_e1166_d_n7, eq26_e1166_d_n8, eq26_e1166_d_n9, eq26_e1166_d_n10, eq26_e1166_d_n11, eq26_e1166_d_n12, eq26_e1166_d_n13, eq26_e1166_d_n14, eq26_e1166_d_n15, eq26_e1166_d_n16, eq26_e1166_d_n17, eq26_e1166_d_n18, eq26_e1166_d_b0, eq26_e1166_d_b1, eq26_e1166_d_b2, eq26_e1166_d_b3, eq26_e1166_d_b4, eq26_e1166_d_b5, eq26_e1166_d_b6, eq26_e1166_d_b7, eq26_e1166_d_b8, eq26_e1166_d_b9, eq26_e1166_d_b10, eq26_e1166_d_b11, eq26_e1166_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1168;let eq26_node_derivatives: [f64; 19] = [eq26_e1168_d_n0, eq26_e1168_d_n1, eq26_e1168_d_n2, eq26_e1168_d_n3, eq26_e1168_d_n4, eq26_e1168_d_n5, eq26_e1168_d_n6, eq26_e1168_d_n7, eq26_e1168_d_n8, eq26_e1168_d_n9, eq26_e1168_d_n10, eq26_e1168_d_n11, eq26_e1168_d_n12, eq26_e1168_d_n13, eq26_e1168_d_n14, eq26_e1168_d_n15, eq26_e1168_d_n16, eq26_e1168_d_n17, eq26_e1168_d_n18];let eq26_branch_derivatives: [f64; 13] = [eq26_e1168_d_b0, eq26_e1168_d_b1, eq26_e1168_d_b2, eq26_e1168_d_b3, eq26_e1168_d_b4, eq26_e1168_d_b5, eq26_e1168_d_b6, eq26_e1168_d_b7, eq26_e1168_d_b8, eq26_e1168_d_b9, eq26_e1168_d_b10, eq26_e1168_d_b11, eq26_e1168_d_b12];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1173,) = {
    if (s.v[76] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e1173;
        stamper.stamp_potential_const_local(
            5,
            eq27_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq28_e1177: f64 = (s.v[18] + s.v[753]);let eq28_e1177_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq28_e1177_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq28_e1177_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq28_e1177_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq28_e1177_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq28_e1177_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq28_e1177_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq28_e1177_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq28_e1177_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq28_e1177_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq28_e1177_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq28_e1177_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq28_e1177_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq28_e1177_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq28_e1177_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq28_e1177_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq28_e1177_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq28_e1177_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq28_e1177_d_n18: f64 = (s.dn[18][18] + s.dn[753][18]);let eq28_e1177_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq28_e1177_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq28_e1177_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq28_e1177_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq28_e1177_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq28_e1177_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq28_e1177_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq28_e1177_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq28_e1177_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq28_e1177_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq28_e1177_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq28_e1177_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq28_e1177_d_b12: f64 = (s.db[18][12] + s.db[753][12]);let eq28_e1178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1177);let eq28_e1179: f64 = (p[87] * eq28_e1178);let eq28_e1179_d_n0: f64 = (p[87] * (eq28_e1177_d_n0 * ddt_scale));let eq28_e1179_d_n1: f64 = (p[87] * (eq28_e1177_d_n1 * ddt_scale));let eq28_e1179_d_n2: f64 = (p[87] * (eq28_e1177_d_n2 * ddt_scale));let eq28_e1179_d_n3: f64 = (p[87] * (eq28_e1177_d_n3 * ddt_scale));let eq28_e1179_d_n4: f64 = (p[87] * (eq28_e1177_d_n4 * ddt_scale));let eq28_e1179_d_n5: f64 = (p[87] * (eq28_e1177_d_n5 * ddt_scale));let eq28_e1179_d_n6: f64 = (p[87] * (eq28_e1177_d_n6 * ddt_scale));let eq28_e1179_d_n7: f64 = (p[87] * (eq28_e1177_d_n7 * ddt_scale));let eq28_e1179_d_n8: f64 = (p[87] * (eq28_e1177_d_n8 * ddt_scale));let eq28_e1179_d_n9: f64 = (p[87] * (eq28_e1177_d_n9 * ddt_scale));let eq28_e1179_d_n10: f64 = (p[87] * (eq28_e1177_d_n10 * ddt_scale));let eq28_e1179_d_n11: f64 = (p[87] * (eq28_e1177_d_n11 * ddt_scale));let eq28_e1179_d_n12: f64 = (p[87] * (eq28_e1177_d_n12 * ddt_scale));let eq28_e1179_d_n13: f64 = (p[87] * (eq28_e1177_d_n13 * ddt_scale));let eq28_e1179_d_n14: f64 = (p[87] * (eq28_e1177_d_n14 * ddt_scale));let eq28_e1179_d_n15: f64 = (p[87] * (eq28_e1177_d_n15 * ddt_scale));let eq28_e1179_d_n16: f64 = (p[87] * (eq28_e1177_d_n16 * ddt_scale));let eq28_e1179_d_n17: f64 = (p[87] * (eq28_e1177_d_n17 * ddt_scale));let eq28_e1179_d_n18: f64 = (p[87] * (eq28_e1177_d_n18 * ddt_scale));let eq28_e1179_d_b0: f64 = (p[87] * (eq28_e1177_d_b0 * ddt_scale));let eq28_e1179_d_b1: f64 = (p[87] * (eq28_e1177_d_b1 * ddt_scale));let eq28_e1179_d_b2: f64 = (p[87] * (eq28_e1177_d_b2 * ddt_scale));let eq28_e1179_d_b3: f64 = (p[87] * (eq28_e1177_d_b3 * ddt_scale));let eq28_e1179_d_b4: f64 = (p[87] * (eq28_e1177_d_b4 * ddt_scale));let eq28_e1179_d_b5: f64 = (p[87] * (eq28_e1177_d_b5 * ddt_scale));let eq28_e1179_d_b6: f64 = (p[87] * (eq28_e1177_d_b6 * ddt_scale));let eq28_e1179_d_b7: f64 = (p[87] * (eq28_e1177_d_b7 * ddt_scale));let eq28_e1179_d_b8: f64 = (p[87] * (eq28_e1177_d_b8 * ddt_scale));
        let eq28_e1179_d_b9: f64 = (p[87] * (eq28_e1177_d_b9 * ddt_scale));let eq28_e1179_d_b10: f64 = (p[87] * (eq28_e1177_d_b10 * ddt_scale));let eq28_e1179_d_b11: f64 = (p[87] * (eq28_e1177_d_b11 * ddt_scale));let eq28_e1179_d_b12: f64 = (p[87] * (eq28_e1177_d_b12 * ddt_scale));let eq28_value: f64 = eq28_e1179;let eq28_node_derivatives: [f64; 19] = [eq28_e1179_d_n0, eq28_e1179_d_n1, eq28_e1179_d_n2, eq28_e1179_d_n3, eq28_e1179_d_n4, eq28_e1179_d_n5, eq28_e1179_d_n6, eq28_e1179_d_n7, eq28_e1179_d_n8, eq28_e1179_d_n9, eq28_e1179_d_n10, eq28_e1179_d_n11, eq28_e1179_d_n12, eq28_e1179_d_n13, eq28_e1179_d_n14, eq28_e1179_d_n15, eq28_e1179_d_n16, eq28_e1179_d_n17, eq28_e1179_d_n18];let eq28_branch_derivatives: [f64; 13] = [eq28_e1179_d_b0, eq28_e1179_d_b1, eq28_e1179_d_b2, eq28_e1179_d_b3, eq28_e1179_d_b4, eq28_e1179_d_b5, eq28_e1179_d_b6, eq28_e1179_d_b7, eq28_e1179_d_b8, eq28_e1179_d_b9, eq28_e1179_d_b10, eq28_e1179_d_b11, eq28_e1179_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }
}
